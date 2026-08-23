#![doc = "DPLL subsystem.\n"]
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
pub const PROTONAME: &str = "dpll";
pub const PROTONAME_CSTR: &CStr = c"dpll";
#[doc = "working modes a dpll can support, differentiates if and how dpll selects\none of its inputs to syntonize with it, valid values for DPLL_A_MODE\nattribute\n"]
#[doc = "Enum - defines an integer enumeration, with values for each entry incrementing by 1, (e.g. 0, 1, 2, 3)"]
#[derive(Debug, Clone, Copy)]
pub enum Mode {
    #[doc = "input can be only selected by sending a request to dpll\n"]
    Manual = 1,
    #[doc = "highest prio input pin auto selected by dpll\n"]
    Automatic = 2,
}
impl Mode {
    pub fn from_value(value: u64) -> Option<Self> {
        Some(match value {
            1 => Self::Manual,
            2 => Self::Automatic,
            _ => return None,
        })
    }
}
#[doc = "provides information of dpll device lock status, valid values for\nDPLL_A_LOCK_STATUS attribute\n"]
#[doc = "Enum - defines an integer enumeration, with values for each entry incrementing by 1, (e.g. 0, 1, 2, 3)"]
#[derive(Debug, Clone, Copy)]
pub enum LockStatus {
    #[doc = "dpll was not yet locked to any valid input (or forced by setting\nDPLL_A_MODE to DPLL_MODE_DETACHED)\n"]
    Unlocked = 1,
    #[doc = "dpll is locked to a valid signal, but no holdover available\n"]
    Locked = 2,
    #[doc = "dpll is locked and holdover acquired\n"]
    LockedHoAcq = 3,
    #[doc = "dpll is in holdover state - lost a valid lock or was forced by\ndisconnecting all the pins (latter possible only when dpll lock-state\nwas already DPLL_LOCK_STATUS_LOCKED_HO_ACQ, if dpll lock-state was not\nDPLL_LOCK_STATUS_LOCKED_HO_ACQ, the dpll\\'s lock-state shall remain\nDPLL_LOCK_STATUS_UNLOCKED)\n"]
    Holdover = 4,
}
impl LockStatus {
    pub fn from_value(value: u64) -> Option<Self> {
        Some(match value {
            1 => Self::Unlocked,
            2 => Self::Locked,
            3 => Self::LockedHoAcq,
            4 => Self::Holdover,
            _ => return None,
        })
    }
}
#[doc = "if previous status change was done due to a failure, this provides\ninformation of dpll device lock status error. Valid values for\nDPLL_A_LOCK_STATUS_ERROR attribute\n"]
#[doc = "Enum - defines an integer enumeration, with values for each entry incrementing by 1, (e.g. 0, 1, 2, 3)"]
#[derive(Debug, Clone, Copy)]
pub enum LockStatusError {
    #[doc = "dpll device lock status was changed without any error\n"]
    None = 1,
    #[doc = "dpll device lock status was changed due to undefined error. Driver fills\nthis value up in case it is not able to obtain suitable exact error\ntype.\n"]
    Undefined = 2,
    #[doc = "dpll device lock status was changed because of associated media got\ndown. This may happen for example if dpll device was previously locked\non an input pin of type PIN_TYPE_SYNCE_ETH_PORT.\n"]
    MediaDown = 3,
    #[doc = "the FFO (Fractional Frequency Offset) between the RX and TX symbol rate\non the media got too high. This may happen for example if dpll device\nwas previously locked on an input pin of type PIN_TYPE_SYNCE_ETH_PORT.\n"]
    FractionalFrequencyOffsetTooHigh = 4,
}
impl LockStatusError {
    pub fn from_value(value: u64) -> Option<Self> {
        Some(match value {
            1 => Self::None,
            2 => Self::Undefined,
            3 => Self::MediaDown,
            4 => Self::FractionalFrequencyOffsetTooHigh,
            _ => return None,
        })
    }
}
#[doc = "level of quality of a clock device. This mainly applies when the dpll\nlock-status is DPLL_LOCK_STATUS_HOLDOVER. The current list is defined\naccording to the table 11-7 contained in ITU-T G.8264/Y.1364 document.\nOne may extend this list freely by other ITU-T defined clock qualities,\nor different ones defined by another standardization body (for those,\nplease use different prefix).\n"]
#[doc = "Enum - defines an integer enumeration, with values for each entry incrementing by 1, (e.g. 0, 1, 2, 3)"]
#[derive(Debug, Clone, Copy)]
pub enum ClockQualityLevel {
    ItuOpt1Prc = 1,
    ItuOpt1SsuA = 2,
    ItuOpt1SsuB = 3,
    ItuOpt1Eec1 = 4,
    ItuOpt1Prtc = 5,
    ItuOpt1Eprtc = 6,
    ItuOpt1Eeec = 7,
    ItuOpt1Eprc = 8,
}
impl ClockQualityLevel {
    pub fn from_value(value: u64) -> Option<Self> {
        Some(match value {
            1 => Self::ItuOpt1Prc,
            2 => Self::ItuOpt1SsuA,
            3 => Self::ItuOpt1SsuB,
            4 => Self::ItuOpt1Eec1,
            5 => Self::ItuOpt1Prtc,
            6 => Self::ItuOpt1Eprtc,
            7 => Self::ItuOpt1Eeec,
            8 => Self::ItuOpt1Eprc,
            _ => return None,
        })
    }
}
#[doc = "temperature divider allowing userspace to calculate the temperature as\nfloat with three digit decimal precision. Value of (DPLL_A_TEMP /\nDPLL_TEMP_DIVIDER) is integer part of temperature value. Value of\n(DPLL_A_TEMP % DPLL_TEMP_DIVIDER) is fractional part of temperature\nvalue.\n"]
pub const TEMP_DIVIDER: u64 = 1000u64;
#[doc = "type of dpll, valid values for DPLL_A_TYPE attribute\n"]
#[doc = "Enum - defines an integer enumeration, with values for each entry incrementing by 1, (e.g. 0, 1, 2, 3)"]
#[derive(Debug, Clone, Copy)]
pub enum Type {
    #[doc = "dpll produces Pulse-Per-Second signal\n"]
    Pps = 1,
    #[doc = "dpll drives the Ethernet Equipment Clock\n"]
    Eec = 2,
    #[doc = "generic dpll type for devices outside PPS/EEC classes\n"]
    Generic = 3,
}
impl Type {
    pub fn from_value(value: u64) -> Option<Self> {
        Some(match value {
            1 => Self::Pps,
            2 => Self::Eec,
            3 => Self::Generic,
            _ => return None,
        })
    }
}
#[doc = "defines possible types of a pin, valid values for DPLL_A_PIN_TYPE\nattribute\n"]
#[doc = "Enum - defines an integer enumeration, with values for each entry incrementing by 1, (e.g. 0, 1, 2, 3)"]
#[derive(Debug, Clone, Copy)]
pub enum PinType {
    #[doc = "aggregates another layer of selectable pins\n"]
    Mux = 1,
    #[doc = "external input\n"]
    Ext = 2,
    #[doc = "ethernet port PHY\\'s recovered clock\n"]
    SynceEthPort = 3,
    #[doc = "device internal oscillator\n"]
    IntOscillator = 4,
    #[doc = "GNSS recovered clock\n"]
    Gnss = 5,
}
impl PinType {
    pub fn from_value(value: u64) -> Option<Self> {
        Some(match value {
            1 => Self::Mux,
            2 => Self::Ext,
            3 => Self::SynceEthPort,
            4 => Self::IntOscillator,
            5 => Self::Gnss,
            _ => return None,
        })
    }
}
#[doc = "defines possible direction of a pin, valid values for\nDPLL_A_PIN_DIRECTION attribute\n"]
#[doc = "Enum - defines an integer enumeration, with values for each entry incrementing by 1, (e.g. 0, 1, 2, 3)"]
#[derive(Debug, Clone, Copy)]
pub enum PinDirection {
    #[doc = "pin used as a input of a signal\n"]
    Input = 1,
    #[doc = "pin used to output the signal\n"]
    Output = 2,
}
impl PinDirection {
    pub fn from_value(value: u64) -> Option<Self> {
        Some(match value {
            1 => Self::Input,
            2 => Self::Output,
            _ => return None,
        })
    }
}
pub const PIN_FREQUENCY_1_HZ: u64 = 1u64;
pub const PIN_FREQUENCY_10_KHZ: u64 = 10000u64;
pub const PIN_FREQUENCY_77_5_KHZ: u64 = 77500u64;
pub const PIN_FREQUENCY_10_MHZ: u64 = 10000000u64;
#[doc = "defines possible states of a pin, valid values for DPLL_A_PIN_STATE\nattribute\n"]
#[doc = "Enum - defines an integer enumeration, with values for each entry incrementing by 1, (e.g. 0, 1, 2, 3)"]
#[derive(Debug, Clone, Copy)]
pub enum PinState {
    #[doc = "pin connected, active input of phase locked loop\n"]
    Connected = 1,
    #[doc = "pin disconnected, not considered as a valid input\n"]
    Disconnected = 2,
    #[doc = "pin enabled for automatic input selection\n"]
    Selectable = 3,
}
impl PinState {
    pub fn from_value(value: u64) -> Option<Self> {
        Some(match value {
            1 => Self::Connected,
            2 => Self::Disconnected,
            3 => Self::Selectable,
            _ => return None,
        })
    }
}
#[doc = "defines possible operational states of a pin with respect to its parent\nDPLL device, valid values for DPLL_A_PIN_OPERSTATE attribute\n"]
#[doc = "Enum - defines an integer enumeration, with values for each entry incrementing by 1, (e.g. 0, 1, 2, 3)"]
#[derive(Debug, Clone, Copy)]
pub enum PinOperstate {
    #[doc = "pin is qualified and actively used by the DPLL\n"]
    Active = 1,
    #[doc = "pin is qualified but not actively used by the DPLL\n"]
    Standby = 2,
    #[doc = "pin does not have a valid signal\n"]
    NoSignal = 3,
    #[doc = "pin signal failed qualification (e.g. frequency or phase monitor)\n"]
    QualFailed = 4,
}
impl PinOperstate {
    pub fn from_value(value: u64) -> Option<Self> {
        Some(match value {
            1 => Self::Active,
            2 => Self::Standby,
            3 => Self::NoSignal,
            4 => Self::QualFailed,
            _ => return None,
        })
    }
}
#[doc = "defines possible capabilities of a pin, valid flags on\nDPLL_A_PIN_CAPABILITIES attribute\n"]
#[doc = "Flags - defines an integer enumeration, with values for each entry occupying a bit, starting from bit 0, (e.g. 1, 2, 4, 8)"]
#[derive(Debug, Clone, Copy)]
pub enum PinCapabilities {
    #[doc = "pin direction can be changed\n"]
    DirectionCanChange = 1 << 0,
    #[doc = "pin priority can be changed\n"]
    PriorityCanChange = 1 << 1,
    #[doc = "pin state can be changed\n"]
    StateCanChange = 1 << 2,
}
impl PinCapabilities {
    pub fn from_value(value: u64) -> Option<Self> {
        Some(match value {
            n if n == 1 << 0 => Self::DirectionCanChange,
            n if n == 1 << 1 => Self::PriorityCanChange,
            n if n == 1 << 2 => Self::StateCanChange,
            _ => return None,
        })
    }
}
#[doc = "phase offset divider allows userspace to calculate a value of measured\nsignal phase difference between a pin and dpll device as a fractional\nvalue with three digit decimal precision. Value of (DPLL_A_PHASE_OFFSET\n/ DPLL_PHASE_OFFSET_DIVIDER) is an integer part of a measured phase\noffset value. Value of (DPLL_A_PHASE_OFFSET % DPLL_PHASE_OFFSET_DIVIDER)\nis a fractional part of a measured phase offset value.\n"]
pub const PHASE_OFFSET_DIVIDER: u64 = 1000u64;
#[doc = "pin measured frequency divider allows userspace to calculate a value of\nmeasured input frequency as a fractional value with three digit decimal\nprecision (millihertz). Value of (DPLL_A_PIN_MEASURED_FREQUENCY /\nDPLL_PIN_MEASURED_FREQUENCY_DIVIDER) is an integer part of a measured\nfrequency value. Value of (DPLL_A_PIN_MEASURED_FREQUENCY %\nDPLL_PIN_MEASURED_FREQUENCY_DIVIDER) is a fractional part of a measured\nfrequency value.\n"]
pub const PIN_MEASURED_FREQUENCY_DIVIDER: u64 = 1000u64;
#[doc = "Allow control (enable/disable) and status checking over features.\n"]
#[doc = "Enum - defines an integer enumeration, with values for each entry incrementing by 1, (e.g. 0, 1, 2, 3)"]
#[derive(Debug, Clone, Copy)]
pub enum FeatureState {
    #[doc = "feature shall be disabled\n"]
    Disable = 0,
    #[doc = "feature shall be enabled\n"]
    Enable = 1,
}
impl FeatureState {
    pub fn from_value(value: u64) -> Option<Self> {
        Some(match value {
            0 => Self::Disable,
            1 => Self::Enable,
            _ => return None,
        })
    }
}
#[derive(Clone)]
pub enum Dpll<'a> {
    Id(u32),
    ModuleName(&'a CStr),
    Pad(&'a [u8]),
    ClockId(u64),
    #[doc = "Associated type: [`Mode`] (enum)"]
    Mode(u32),
    #[doc = "Associated type: [`Mode`] (enum)\nAttribute may repeat multiple times (treat it as array)"]
    ModeSupported(u32),
    #[doc = "Associated type: [`LockStatus`] (enum)"]
    LockStatus(u32),
    Temp(i32),
    #[doc = "Associated type: [`Type`] (enum)"]
    Type(u32),
    #[doc = "Associated type: [`LockStatusError`] (enum)"]
    LockStatusError(u32),
    #[doc = "Level of quality of a clock device. This mainly applies when the dpll\nlock-status is DPLL_LOCK_STATUS_HOLDOVER. This could be put to message\nmultiple times to indicate possible parallel quality levels (e.g. one\nspecified by ITU option 1 and another one specified by option 2).\n\nAssociated type: [`ClockQualityLevel`] (enum)\nAttribute may repeat multiple times (treat it as array)"]
    ClockQualityLevel(u32),
    #[doc = "Receive or request state of phase offset monitor feature. If enabled,\ndpll device shall monitor and notify all currently available inputs for\nchanges of their phase offset against the dpll device.\n\nAssociated type: [`FeatureState`] (enum)"]
    PhaseOffsetMonitor(u32),
    #[doc = "Averaging factor applied to calculation of reported phase offset.\n"]
    PhaseOffsetAvgFactor(u32),
    #[doc = "Current or desired state of the frequency monitor feature. If enabled,\ndpll device shall measure all currently available inputs for their\nactual input frequency.\n\nAssociated type: [`FeatureState`] (enum)"]
    FrequencyMonitor(u32),
}
impl<'a> IterableDpll<'a> {
    pub fn get_id(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Dpll::Id(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Dpll",
            "Id",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_module_name(&self) -> Result<&'a CStr, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Dpll::ModuleName(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Dpll",
            "ModuleName",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_pad(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Dpll::Pad(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Dpll",
            "Pad",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_clock_id(&self) -> Result<u64, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Dpll::ClockId(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Dpll",
            "ClockId",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Associated type: [`Mode`] (enum)"]
    pub fn get_mode(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Dpll::Mode(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Dpll",
            "Mode",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Associated type: [`Mode`] (enum)\nAttribute may repeat multiple times (treat it as array)"]
    pub fn get_mode_supported(&self) -> MultiAttrIterable<Self, Dpll<'a>, u32> {
        MultiAttrIterable::new(self.clone(), |variant| {
            if let Dpll::ModeSupported(val) = variant {
                Some(val)
            } else {
                None
            }
        })
    }
    #[doc = "Associated type: [`LockStatus`] (enum)"]
    pub fn get_lock_status(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Dpll::LockStatus(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Dpll",
            "LockStatus",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_temp(&self) -> Result<i32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Dpll::Temp(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Dpll",
            "Temp",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Associated type: [`Type`] (enum)"]
    pub fn get_type(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Dpll::Type(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Dpll",
            "Type",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Associated type: [`LockStatusError`] (enum)"]
    pub fn get_lock_status_error(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Dpll::LockStatusError(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Dpll",
            "LockStatusError",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Level of quality of a clock device. This mainly applies when the dpll\nlock-status is DPLL_LOCK_STATUS_HOLDOVER. This could be put to message\nmultiple times to indicate possible parallel quality levels (e.g. one\nspecified by ITU option 1 and another one specified by option 2).\n\nAssociated type: [`ClockQualityLevel`] (enum)\nAttribute may repeat multiple times (treat it as array)"]
    pub fn get_clock_quality_level(&self) -> MultiAttrIterable<Self, Dpll<'a>, u32> {
        MultiAttrIterable::new(self.clone(), |variant| {
            if let Dpll::ClockQualityLevel(val) = variant {
                Some(val)
            } else {
                None
            }
        })
    }
    #[doc = "Receive or request state of phase offset monitor feature. If enabled,\ndpll device shall monitor and notify all currently available inputs for\nchanges of their phase offset against the dpll device.\n\nAssociated type: [`FeatureState`] (enum)"]
    pub fn get_phase_offset_monitor(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Dpll::PhaseOffsetMonitor(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Dpll",
            "PhaseOffsetMonitor",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Averaging factor applied to calculation of reported phase offset.\n"]
    pub fn get_phase_offset_avg_factor(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Dpll::PhaseOffsetAvgFactor(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Dpll",
            "PhaseOffsetAvgFactor",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Current or desired state of the frequency monitor feature. If enabled,\ndpll device shall measure all currently available inputs for their\nactual input frequency.\n\nAssociated type: [`FeatureState`] (enum)"]
    pub fn get_frequency_monitor(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Dpll::FrequencyMonitor(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Dpll",
            "FrequencyMonitor",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl Dpll<'_> {
    pub fn new<'a>(buf: &'a [u8]) -> IterableDpll<'a> {
        IterableDpll::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        let res = match r#type {
            1u16 => "Id",
            2u16 => "ModuleName",
            3u16 => "Pad",
            4u16 => "ClockId",
            5u16 => "Mode",
            6u16 => "ModeSupported",
            7u16 => "LockStatus",
            8u16 => "Temp",
            9u16 => "Type",
            10u16 => "LockStatusError",
            11u16 => "ClockQualityLevel",
            12u16 => "PhaseOffsetMonitor",
            13u16 => "PhaseOffsetAvgFactor",
            14u16 => "FrequencyMonitor",
            _ => return None,
        };
        Some(res)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableDpll<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableDpll<'a> {
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
impl<'a> Iterator for IterableDpll<'a> {
    type Item = Result<Dpll<'a>, ErrorContext>;
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
                1u16 => Dpll::Id({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                2u16 => Dpll::ModuleName({
                    let res = CStr::from_bytes_with_nul(next).ok();
                    let Some(val) = res else { break };
                    val
                }),
                3u16 => Dpll::Pad({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                4u16 => Dpll::ClockId({
                    let res = parse_u64(next);
                    let Some(val) = res else { break };
                    val
                }),
                5u16 => Dpll::Mode({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                6u16 => Dpll::ModeSupported({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                7u16 => Dpll::LockStatus({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                8u16 => Dpll::Temp({
                    let res = parse_i32(next);
                    let Some(val) = res else { break };
                    val
                }),
                9u16 => Dpll::Type({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                10u16 => Dpll::LockStatusError({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                11u16 => Dpll::ClockQualityLevel({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                12u16 => Dpll::PhaseOffsetMonitor({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                13u16 => Dpll::PhaseOffsetAvgFactor({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                14u16 => Dpll::FrequencyMonitor({
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
            "Dpll",
            r#type.and_then(|t| Dpll::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl<'a> std::fmt::Debug for IterableDpll<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("Dpll");
        let mut iter = IterableDpll::with_loc(&[], self.orig_loc);
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
                Dpll::Id(val) => fmt.field("Id", &val),
                Dpll::ModuleName(val) => fmt.field("ModuleName", &val),
                Dpll::Pad(val) => fmt.field("Pad", &val),
                Dpll::ClockId(val) => fmt.field("ClockId", &val),
                Dpll::Mode(val) => fmt.field("Mode", &FormatEnum(val.into(), Mode::from_value)),
                Dpll::ModeSupported(val) => {
                    fmt.field("ModeSupported", &FormatEnum(val.into(), Mode::from_value))
                }
                Dpll::LockStatus(val) => fmt.field(
                    "LockStatus",
                    &FormatEnum(val.into(), LockStatus::from_value),
                ),
                Dpll::Temp(val) => fmt.field("Temp", &val),
                Dpll::Type(val) => fmt.field("Type", &FormatEnum(val.into(), Type::from_value)),
                Dpll::LockStatusError(val) => fmt.field(
                    "LockStatusError",
                    &FormatEnum(val.into(), LockStatusError::from_value),
                ),
                Dpll::ClockQualityLevel(val) => fmt.field(
                    "ClockQualityLevel",
                    &FormatEnum(val.into(), ClockQualityLevel::from_value),
                ),
                Dpll::PhaseOffsetMonitor(val) => fmt.field(
                    "PhaseOffsetMonitor",
                    &FormatEnum(val.into(), FeatureState::from_value),
                ),
                Dpll::PhaseOffsetAvgFactor(val) => fmt.field("PhaseOffsetAvgFactor", &val),
                Dpll::FrequencyMonitor(val) => fmt.field(
                    "FrequencyMonitor",
                    &FormatEnum(val.into(), FeatureState::from_value),
                ),
            };
        }
        fmt.finish()
    }
}
impl IterableDpll<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("Dpll", offset));
            return (stack, missing_type.and_then(|t| Dpll::attr_from_type(t)));
        }
        if cur > offset || cur + self.buf.len() < offset {
            return (stack, None);
        }
        let mut attrs = self.clone();
        let mut last_off = cur + attrs.pos;
        while let Some(attr) = attrs.next() {
            let Ok(attr) = attr else { break };
            match attr {
                Dpll::Id(val) => {
                    if last_off == offset {
                        stack.push(("Id", last_off));
                        break;
                    }
                }
                Dpll::ModuleName(val) => {
                    if last_off == offset {
                        stack.push(("ModuleName", last_off));
                        break;
                    }
                }
                Dpll::Pad(val) => {
                    if last_off == offset {
                        stack.push(("Pad", last_off));
                        break;
                    }
                }
                Dpll::ClockId(val) => {
                    if last_off == offset {
                        stack.push(("ClockId", last_off));
                        break;
                    }
                }
                Dpll::Mode(val) => {
                    if last_off == offset {
                        stack.push(("Mode", last_off));
                        break;
                    }
                }
                Dpll::ModeSupported(val) => {
                    if last_off == offset {
                        stack.push(("ModeSupported", last_off));
                        break;
                    }
                }
                Dpll::LockStatus(val) => {
                    if last_off == offset {
                        stack.push(("LockStatus", last_off));
                        break;
                    }
                }
                Dpll::Temp(val) => {
                    if last_off == offset {
                        stack.push(("Temp", last_off));
                        break;
                    }
                }
                Dpll::Type(val) => {
                    if last_off == offset {
                        stack.push(("Type", last_off));
                        break;
                    }
                }
                Dpll::LockStatusError(val) => {
                    if last_off == offset {
                        stack.push(("LockStatusError", last_off));
                        break;
                    }
                }
                Dpll::ClockQualityLevel(val) => {
                    if last_off == offset {
                        stack.push(("ClockQualityLevel", last_off));
                        break;
                    }
                }
                Dpll::PhaseOffsetMonitor(val) => {
                    if last_off == offset {
                        stack.push(("PhaseOffsetMonitor", last_off));
                        break;
                    }
                }
                Dpll::PhaseOffsetAvgFactor(val) => {
                    if last_off == offset {
                        stack.push(("PhaseOffsetAvgFactor", last_off));
                        break;
                    }
                }
                Dpll::FrequencyMonitor(val) => {
                    if last_off == offset {
                        stack.push(("FrequencyMonitor", last_off));
                        break;
                    }
                }
                _ => {}
            };
            last_off = cur + attrs.pos;
        }
        if !stack.is_empty() {
            stack.push(("Dpll", cur));
        }
        (stack, None)
    }
}
#[derive(Clone)]
pub enum Pin<'a> {
    Id(u32),
    ParentId(u32),
    ModuleName(&'a CStr),
    Pad(&'a [u8]),
    ClockId(u64),
    BoardLabel(&'a CStr),
    PanelLabel(&'a CStr),
    PackageLabel(&'a CStr),
    #[doc = "Associated type: [`PinType`] (enum)"]
    Type(u32),
    #[doc = "Associated type: [`PinDirection`] (enum)"]
    Direction(u32),
    Frequency(u64),
    #[doc = "Attribute may repeat multiple times (treat it as array)"]
    FrequencySupported(IterableFrequencyRange<'a>),
    FrequencyMin(u64),
    FrequencyMax(u64),
    Prio(u32),
    #[doc = "Associated type: [`PinState`] (enum)"]
    State(u32),
    #[doc = "Associated type: [`PinCapabilities`] (enum)"]
    Capabilities(u32),
    #[doc = "Attribute may repeat multiple times (treat it as array)"]
    ParentDevice(IterablePinParentDevice<'a>),
    #[doc = "Attribute may repeat multiple times (treat it as array)"]
    ParentPin(IterablePinParentPin<'a>),
    PhaseAdjustMin(i32),
    PhaseAdjustMax(i32),
    PhaseAdjust(i32),
    PhaseOffset(i64),
    #[doc = "The FFO (Fractional Frequency Offset) of the pin. At top level this\nrepresents the RX vs TX symbol rate offset on the media associated with\nthe pin. Inside the pin-parent-device nest it represents the frequency\noffset between the pin and its parent DPLL device. Value is in PPM\n(parts per million). This is a lower-precision version of\nfractional-frequency-offset-ppt.\n"]
    FractionalFrequencyOffset(i32),
    #[doc = "Frequency of Embedded SYNC signal. If provided, the pin is configured\nwith a SYNC signal embedded into its base clock frequency.\n"]
    EsyncFrequency(u64),
    #[doc = "If provided a pin is capable of embedding a SYNC signal (within given\nrange) into its base frequency signal.\n\nAttribute may repeat multiple times (treat it as array)"]
    EsyncFrequencySupported(IterableFrequencyRange<'a>),
    #[doc = "A ratio of high to low state of a SYNC signal pulse embedded into base\nclock frequency. Value is in percents.\n"]
    EsyncPulse(u32),
    #[doc = "Capable pin provides list of pins that can be bound to create a\nreference-sync pin pair.\n\nAttribute may repeat multiple times (treat it as array)"]
    ReferenceSync(IterableReferenceSync<'a>),
    #[doc = "Granularity of phase adjustment, in picoseconds. The value of phase\nadjustment must be a multiple of this granularity.\n"]
    PhaseAdjustGran(u32),
    #[doc = "The FFO (Fractional Frequency Offset) of the pin. At top level this\nrepresents the RX vs TX symbol rate offset on the media associated with\nthe pin. Inside the pin-parent-device nest it represents the frequency\noffset between the pin and its parent DPLL device. Value is in PPT\n(parts per trillion, 10\\^-12). This is a higher-precision version of\nfractional-frequency-offset.\n"]
    FractionalFrequencyOffsetPpt(i32),
    #[doc = "The measured frequency of the input pin in millihertz (mHz). Value of\n(DPLL_A_PIN_MEASURED_FREQUENCY / DPLL_PIN_MEASURED_FREQUENCY_DIVIDER) is\nan integer part (Hz) of a measured frequency value. Value of\n(DPLL_A_PIN_MEASURED_FREQUENCY % DPLL_PIN_MEASURED_FREQUENCY_DIVIDER) is\na fractional part of a measured frequency value.\n"]
    MeasuredFrequency(u64),
    #[doc = "Operational state of the pin with respect to its parent DPLL device.\nUnlike state (which reflects the administrative intent), operstate\nreflects the actual hardware status.\n\nAssociated type: [`PinOperstate`] (enum)"]
    Operstate(u32),
}
impl<'a> IterablePin<'a> {
    pub fn get_id(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Pin::Id(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Pin",
            "Id",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_parent_id(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Pin::ParentId(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Pin",
            "ParentId",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_module_name(&self) -> Result<&'a CStr, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Pin::ModuleName(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Pin",
            "ModuleName",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_pad(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Pin::Pad(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Pin",
            "Pad",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_clock_id(&self) -> Result<u64, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Pin::ClockId(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Pin",
            "ClockId",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_board_label(&self) -> Result<&'a CStr, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Pin::BoardLabel(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Pin",
            "BoardLabel",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_panel_label(&self) -> Result<&'a CStr, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Pin::PanelLabel(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Pin",
            "PanelLabel",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_package_label(&self) -> Result<&'a CStr, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Pin::PackageLabel(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Pin",
            "PackageLabel",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Associated type: [`PinType`] (enum)"]
    pub fn get_type(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Pin::Type(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Pin",
            "Type",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Associated type: [`PinDirection`] (enum)"]
    pub fn get_direction(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Pin::Direction(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Pin",
            "Direction",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_frequency(&self) -> Result<u64, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Pin::Frequency(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Pin",
            "Frequency",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Attribute may repeat multiple times (treat it as array)"]
    pub fn get_frequency_supported(
        &self,
    ) -> MultiAttrIterable<Self, Pin<'a>, IterableFrequencyRange<'a>> {
        MultiAttrIterable::new(self.clone(), |variant| {
            if let Pin::FrequencySupported(val) = variant {
                Some(val)
            } else {
                None
            }
        })
    }
    pub fn get_frequency_min(&self) -> Result<u64, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Pin::FrequencyMin(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Pin",
            "FrequencyMin",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_frequency_max(&self) -> Result<u64, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Pin::FrequencyMax(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Pin",
            "FrequencyMax",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_prio(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Pin::Prio(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Pin",
            "Prio",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Associated type: [`PinState`] (enum)"]
    pub fn get_state(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Pin::State(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Pin",
            "State",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Associated type: [`PinCapabilities`] (enum)"]
    pub fn get_capabilities(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Pin::Capabilities(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Pin",
            "Capabilities",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Attribute may repeat multiple times (treat it as array)"]
    pub fn get_parent_device(
        &self,
    ) -> MultiAttrIterable<Self, Pin<'a>, IterablePinParentDevice<'a>> {
        MultiAttrIterable::new(self.clone(), |variant| {
            if let Pin::ParentDevice(val) = variant {
                Some(val)
            } else {
                None
            }
        })
    }
    #[doc = "Attribute may repeat multiple times (treat it as array)"]
    pub fn get_parent_pin(&self) -> MultiAttrIterable<Self, Pin<'a>, IterablePinParentPin<'a>> {
        MultiAttrIterable::new(self.clone(), |variant| {
            if let Pin::ParentPin(val) = variant {
                Some(val)
            } else {
                None
            }
        })
    }
    pub fn get_phase_adjust_min(&self) -> Result<i32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Pin::PhaseAdjustMin(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Pin",
            "PhaseAdjustMin",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_phase_adjust_max(&self) -> Result<i32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Pin::PhaseAdjustMax(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Pin",
            "PhaseAdjustMax",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_phase_adjust(&self) -> Result<i32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Pin::PhaseAdjust(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Pin",
            "PhaseAdjust",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_phase_offset(&self) -> Result<i64, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Pin::PhaseOffset(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Pin",
            "PhaseOffset",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "The FFO (Fractional Frequency Offset) of the pin. At top level this\nrepresents the RX vs TX symbol rate offset on the media associated with\nthe pin. Inside the pin-parent-device nest it represents the frequency\noffset between the pin and its parent DPLL device. Value is in PPM\n(parts per million). This is a lower-precision version of\nfractional-frequency-offset-ppt.\n"]
    pub fn get_fractional_frequency_offset(&self) -> Result<i32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Pin::FractionalFrequencyOffset(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Pin",
            "FractionalFrequencyOffset",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Frequency of Embedded SYNC signal. If provided, the pin is configured\nwith a SYNC signal embedded into its base clock frequency.\n"]
    pub fn get_esync_frequency(&self) -> Result<u64, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Pin::EsyncFrequency(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Pin",
            "EsyncFrequency",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "If provided a pin is capable of embedding a SYNC signal (within given\nrange) into its base frequency signal.\n\nAttribute may repeat multiple times (treat it as array)"]
    pub fn get_esync_frequency_supported(
        &self,
    ) -> MultiAttrIterable<Self, Pin<'a>, IterableFrequencyRange<'a>> {
        MultiAttrIterable::new(self.clone(), |variant| {
            if let Pin::EsyncFrequencySupported(val) = variant {
                Some(val)
            } else {
                None
            }
        })
    }
    #[doc = "A ratio of high to low state of a SYNC signal pulse embedded into base\nclock frequency. Value is in percents.\n"]
    pub fn get_esync_pulse(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Pin::EsyncPulse(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Pin",
            "EsyncPulse",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Capable pin provides list of pins that can be bound to create a\nreference-sync pin pair.\n\nAttribute may repeat multiple times (treat it as array)"]
    pub fn get_reference_sync(
        &self,
    ) -> MultiAttrIterable<Self, Pin<'a>, IterableReferenceSync<'a>> {
        MultiAttrIterable::new(self.clone(), |variant| {
            if let Pin::ReferenceSync(val) = variant {
                Some(val)
            } else {
                None
            }
        })
    }
    #[doc = "Granularity of phase adjustment, in picoseconds. The value of phase\nadjustment must be a multiple of this granularity.\n"]
    pub fn get_phase_adjust_gran(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Pin::PhaseAdjustGran(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Pin",
            "PhaseAdjustGran",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "The FFO (Fractional Frequency Offset) of the pin. At top level this\nrepresents the RX vs TX symbol rate offset on the media associated with\nthe pin. Inside the pin-parent-device nest it represents the frequency\noffset between the pin and its parent DPLL device. Value is in PPT\n(parts per trillion, 10\\^-12). This is a higher-precision version of\nfractional-frequency-offset.\n"]
    pub fn get_fractional_frequency_offset_ppt(&self) -> Result<i32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Pin::FractionalFrequencyOffsetPpt(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Pin",
            "FractionalFrequencyOffsetPpt",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "The measured frequency of the input pin in millihertz (mHz). Value of\n(DPLL_A_PIN_MEASURED_FREQUENCY / DPLL_PIN_MEASURED_FREQUENCY_DIVIDER) is\nan integer part (Hz) of a measured frequency value. Value of\n(DPLL_A_PIN_MEASURED_FREQUENCY % DPLL_PIN_MEASURED_FREQUENCY_DIVIDER) is\na fractional part of a measured frequency value.\n"]
    pub fn get_measured_frequency(&self) -> Result<u64, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Pin::MeasuredFrequency(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Pin",
            "MeasuredFrequency",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Operational state of the pin with respect to its parent DPLL device.\nUnlike state (which reflects the administrative intent), operstate\nreflects the actual hardware status.\n\nAssociated type: [`PinOperstate`] (enum)"]
    pub fn get_operstate(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Pin::Operstate(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Pin",
            "Operstate",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl Pin<'_> {
    pub fn new<'a>(buf: &'a [u8]) -> IterablePin<'a> {
        IterablePin::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        let res = match r#type {
            1u16 => "Id",
            2u16 => "ParentId",
            3u16 => "ModuleName",
            4u16 => "Pad",
            5u16 => "ClockId",
            6u16 => "BoardLabel",
            7u16 => "PanelLabel",
            8u16 => "PackageLabel",
            9u16 => "Type",
            10u16 => "Direction",
            11u16 => "Frequency",
            12u16 => "FrequencySupported",
            13u16 => "FrequencyMin",
            14u16 => "FrequencyMax",
            15u16 => "Prio",
            16u16 => "State",
            17u16 => "Capabilities",
            18u16 => "ParentDevice",
            19u16 => "ParentPin",
            20u16 => "PhaseAdjustMin",
            21u16 => "PhaseAdjustMax",
            22u16 => "PhaseAdjust",
            23u16 => "PhaseOffset",
            24u16 => "FractionalFrequencyOffset",
            25u16 => "EsyncFrequency",
            26u16 => "EsyncFrequencySupported",
            27u16 => "EsyncPulse",
            28u16 => "ReferenceSync",
            29u16 => "PhaseAdjustGran",
            30u16 => "FractionalFrequencyOffsetPpt",
            31u16 => "MeasuredFrequency",
            32u16 => "Operstate",
            _ => return None,
        };
        Some(res)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterablePin<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterablePin<'a> {
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
impl<'a> Iterator for IterablePin<'a> {
    type Item = Result<Pin<'a>, ErrorContext>;
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
                1u16 => Pin::Id({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                2u16 => Pin::ParentId({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                3u16 => Pin::ModuleName({
                    let res = CStr::from_bytes_with_nul(next).ok();
                    let Some(val) = res else { break };
                    val
                }),
                4u16 => Pin::Pad({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                5u16 => Pin::ClockId({
                    let res = parse_u64(next);
                    let Some(val) = res else { break };
                    val
                }),
                6u16 => Pin::BoardLabel({
                    let res = CStr::from_bytes_with_nul(next).ok();
                    let Some(val) = res else { break };
                    val
                }),
                7u16 => Pin::PanelLabel({
                    let res = CStr::from_bytes_with_nul(next).ok();
                    let Some(val) = res else { break };
                    val
                }),
                8u16 => Pin::PackageLabel({
                    let res = CStr::from_bytes_with_nul(next).ok();
                    let Some(val) = res else { break };
                    val
                }),
                9u16 => Pin::Type({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                10u16 => Pin::Direction({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                11u16 => Pin::Frequency({
                    let res = parse_u64(next);
                    let Some(val) = res else { break };
                    val
                }),
                12u16 => Pin::FrequencySupported({
                    let res = Some(IterableFrequencyRange::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                13u16 => Pin::FrequencyMin({
                    let res = parse_u64(next);
                    let Some(val) = res else { break };
                    val
                }),
                14u16 => Pin::FrequencyMax({
                    let res = parse_u64(next);
                    let Some(val) = res else { break };
                    val
                }),
                15u16 => Pin::Prio({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                16u16 => Pin::State({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                17u16 => Pin::Capabilities({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                18u16 => Pin::ParentDevice({
                    let res = Some(IterablePinParentDevice::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                19u16 => Pin::ParentPin({
                    let res = Some(IterablePinParentPin::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                20u16 => Pin::PhaseAdjustMin({
                    let res = parse_i32(next);
                    let Some(val) = res else { break };
                    val
                }),
                21u16 => Pin::PhaseAdjustMax({
                    let res = parse_i32(next);
                    let Some(val) = res else { break };
                    val
                }),
                22u16 => Pin::PhaseAdjust({
                    let res = parse_i32(next);
                    let Some(val) = res else { break };
                    val
                }),
                23u16 => Pin::PhaseOffset({
                    let res = parse_i64(next);
                    let Some(val) = res else { break };
                    val
                }),
                24u16 => Pin::FractionalFrequencyOffset({
                    let res = parse_i32(next);
                    let Some(val) = res else { break };
                    val
                }),
                25u16 => Pin::EsyncFrequency({
                    let res = parse_u64(next);
                    let Some(val) = res else { break };
                    val
                }),
                26u16 => Pin::EsyncFrequencySupported({
                    let res = Some(IterableFrequencyRange::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                27u16 => Pin::EsyncPulse({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                28u16 => Pin::ReferenceSync({
                    let res = Some(IterableReferenceSync::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                29u16 => Pin::PhaseAdjustGran({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                30u16 => Pin::FractionalFrequencyOffsetPpt({
                    let res = parse_i32(next);
                    let Some(val) = res else { break };
                    val
                }),
                31u16 => Pin::MeasuredFrequency({
                    let res = parse_u64(next);
                    let Some(val) = res else { break };
                    val
                }),
                32u16 => Pin::Operstate({
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
            "Pin",
            r#type.and_then(|t| Pin::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl<'a> std::fmt::Debug for IterablePin<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("Pin");
        let mut iter = IterablePin::with_loc(&[], self.orig_loc);
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
                Pin::Id(val) => fmt.field("Id", &val),
                Pin::ParentId(val) => fmt.field("ParentId", &val),
                Pin::ModuleName(val) => fmt.field("ModuleName", &val),
                Pin::Pad(val) => fmt.field("Pad", &val),
                Pin::ClockId(val) => fmt.field("ClockId", &val),
                Pin::BoardLabel(val) => fmt.field("BoardLabel", &val),
                Pin::PanelLabel(val) => fmt.field("PanelLabel", &val),
                Pin::PackageLabel(val) => fmt.field("PackageLabel", &val),
                Pin::Type(val) => fmt.field("Type", &FormatEnum(val.into(), PinType::from_value)),
                Pin::Direction(val) => fmt.field(
                    "Direction",
                    &FormatEnum(val.into(), PinDirection::from_value),
                ),
                Pin::Frequency(val) => fmt.field("Frequency", &val),
                Pin::FrequencySupported(val) => fmt.field("FrequencySupported", &val),
                Pin::FrequencyMin(val) => fmt.field("FrequencyMin", &val),
                Pin::FrequencyMax(val) => fmt.field("FrequencyMax", &val),
                Pin::Prio(val) => fmt.field("Prio", &val),
                Pin::State(val) => {
                    fmt.field("State", &FormatEnum(val.into(), PinState::from_value))
                }
                Pin::Capabilities(val) => fmt.field(
                    "Capabilities",
                    &FormatFlags(val.into(), PinCapabilities::from_value),
                ),
                Pin::ParentDevice(val) => fmt.field("ParentDevice", &val),
                Pin::ParentPin(val) => fmt.field("ParentPin", &val),
                Pin::PhaseAdjustMin(val) => fmt.field("PhaseAdjustMin", &val),
                Pin::PhaseAdjustMax(val) => fmt.field("PhaseAdjustMax", &val),
                Pin::PhaseAdjust(val) => fmt.field("PhaseAdjust", &val),
                Pin::PhaseOffset(val) => fmt.field("PhaseOffset", &val),
                Pin::FractionalFrequencyOffset(val) => fmt.field("FractionalFrequencyOffset", &val),
                Pin::EsyncFrequency(val) => fmt.field("EsyncFrequency", &val),
                Pin::EsyncFrequencySupported(val) => fmt.field("EsyncFrequencySupported", &val),
                Pin::EsyncPulse(val) => fmt.field("EsyncPulse", &val),
                Pin::ReferenceSync(val) => fmt.field("ReferenceSync", &val),
                Pin::PhaseAdjustGran(val) => fmt.field("PhaseAdjustGran", &val),
                Pin::FractionalFrequencyOffsetPpt(val) => {
                    fmt.field("FractionalFrequencyOffsetPpt", &val)
                }
                Pin::MeasuredFrequency(val) => fmt.field("MeasuredFrequency", &val),
                Pin::Operstate(val) => fmt.field(
                    "Operstate",
                    &FormatEnum(val.into(), PinOperstate::from_value),
                ),
            };
        }
        fmt.finish()
    }
}
impl IterablePin<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("Pin", offset));
            return (stack, missing_type.and_then(|t| Pin::attr_from_type(t)));
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
                Pin::Id(val) => {
                    if last_off == offset {
                        stack.push(("Id", last_off));
                        break;
                    }
                }
                Pin::ParentId(val) => {
                    if last_off == offset {
                        stack.push(("ParentId", last_off));
                        break;
                    }
                }
                Pin::ModuleName(val) => {
                    if last_off == offset {
                        stack.push(("ModuleName", last_off));
                        break;
                    }
                }
                Pin::Pad(val) => {
                    if last_off == offset {
                        stack.push(("Pad", last_off));
                        break;
                    }
                }
                Pin::ClockId(val) => {
                    if last_off == offset {
                        stack.push(("ClockId", last_off));
                        break;
                    }
                }
                Pin::BoardLabel(val) => {
                    if last_off == offset {
                        stack.push(("BoardLabel", last_off));
                        break;
                    }
                }
                Pin::PanelLabel(val) => {
                    if last_off == offset {
                        stack.push(("PanelLabel", last_off));
                        break;
                    }
                }
                Pin::PackageLabel(val) => {
                    if last_off == offset {
                        stack.push(("PackageLabel", last_off));
                        break;
                    }
                }
                Pin::Type(val) => {
                    if last_off == offset {
                        stack.push(("Type", last_off));
                        break;
                    }
                }
                Pin::Direction(val) => {
                    if last_off == offset {
                        stack.push(("Direction", last_off));
                        break;
                    }
                }
                Pin::Frequency(val) => {
                    if last_off == offset {
                        stack.push(("Frequency", last_off));
                        break;
                    }
                }
                Pin::FrequencySupported(val) => {
                    (stack, missing) = val.lookup_attr(offset, missing_type);
                    if !stack.is_empty() {
                        break;
                    }
                }
                Pin::FrequencyMin(val) => {
                    if last_off == offset {
                        stack.push(("FrequencyMin", last_off));
                        break;
                    }
                }
                Pin::FrequencyMax(val) => {
                    if last_off == offset {
                        stack.push(("FrequencyMax", last_off));
                        break;
                    }
                }
                Pin::Prio(val) => {
                    if last_off == offset {
                        stack.push(("Prio", last_off));
                        break;
                    }
                }
                Pin::State(val) => {
                    if last_off == offset {
                        stack.push(("State", last_off));
                        break;
                    }
                }
                Pin::Capabilities(val) => {
                    if last_off == offset {
                        stack.push(("Capabilities", last_off));
                        break;
                    }
                }
                Pin::ParentDevice(val) => {
                    (stack, missing) = val.lookup_attr(offset, missing_type);
                    if !stack.is_empty() {
                        break;
                    }
                }
                Pin::ParentPin(val) => {
                    (stack, missing) = val.lookup_attr(offset, missing_type);
                    if !stack.is_empty() {
                        break;
                    }
                }
                Pin::PhaseAdjustMin(val) => {
                    if last_off == offset {
                        stack.push(("PhaseAdjustMin", last_off));
                        break;
                    }
                }
                Pin::PhaseAdjustMax(val) => {
                    if last_off == offset {
                        stack.push(("PhaseAdjustMax", last_off));
                        break;
                    }
                }
                Pin::PhaseAdjust(val) => {
                    if last_off == offset {
                        stack.push(("PhaseAdjust", last_off));
                        break;
                    }
                }
                Pin::PhaseOffset(val) => {
                    if last_off == offset {
                        stack.push(("PhaseOffset", last_off));
                        break;
                    }
                }
                Pin::FractionalFrequencyOffset(val) => {
                    if last_off == offset {
                        stack.push(("FractionalFrequencyOffset", last_off));
                        break;
                    }
                }
                Pin::EsyncFrequency(val) => {
                    if last_off == offset {
                        stack.push(("EsyncFrequency", last_off));
                        break;
                    }
                }
                Pin::EsyncFrequencySupported(val) => {
                    (stack, missing) = val.lookup_attr(offset, missing_type);
                    if !stack.is_empty() {
                        break;
                    }
                }
                Pin::EsyncPulse(val) => {
                    if last_off == offset {
                        stack.push(("EsyncPulse", last_off));
                        break;
                    }
                }
                Pin::ReferenceSync(val) => {
                    (stack, missing) = val.lookup_attr(offset, missing_type);
                    if !stack.is_empty() {
                        break;
                    }
                }
                Pin::PhaseAdjustGran(val) => {
                    if last_off == offset {
                        stack.push(("PhaseAdjustGran", last_off));
                        break;
                    }
                }
                Pin::FractionalFrequencyOffsetPpt(val) => {
                    if last_off == offset {
                        stack.push(("FractionalFrequencyOffsetPpt", last_off));
                        break;
                    }
                }
                Pin::MeasuredFrequency(val) => {
                    if last_off == offset {
                        stack.push(("MeasuredFrequency", last_off));
                        break;
                    }
                }
                Pin::Operstate(val) => {
                    if last_off == offset {
                        stack.push(("Operstate", last_off));
                        break;
                    }
                }
                _ => {}
            };
            last_off = cur + attrs.pos;
        }
        if !stack.is_empty() {
            stack.push(("Pin", cur));
        }
        (stack, missing)
    }
}
#[derive(Clone)]
pub enum PinParentDevice {
    ParentId(u32),
    #[doc = "Associated type: [`PinDirection`] (enum)"]
    Direction(u32),
    Prio(u32),
    #[doc = "Associated type: [`PinState`] (enum)"]
    State(u32),
    PhaseOffset(i64),
    #[doc = "The FFO (Fractional Frequency Offset) of the pin. At top level this\nrepresents the RX vs TX symbol rate offset on the media associated with\nthe pin. Inside the pin-parent-device nest it represents the frequency\noffset between the pin and its parent DPLL device. Value is in PPM\n(parts per million). This is a lower-precision version of\nfractional-frequency-offset-ppt.\n"]
    FractionalFrequencyOffset(i32),
    #[doc = "The FFO (Fractional Frequency Offset) of the pin. At top level this\nrepresents the RX vs TX symbol rate offset on the media associated with\nthe pin. Inside the pin-parent-device nest it represents the frequency\noffset between the pin and its parent DPLL device. Value is in PPT\n(parts per trillion, 10\\^-12). This is a higher-precision version of\nfractional-frequency-offset.\n"]
    FractionalFrequencyOffsetPpt(i32),
    #[doc = "Operational state of the pin with respect to its parent DPLL device.\nUnlike state (which reflects the administrative intent), operstate\nreflects the actual hardware status.\n\nAssociated type: [`PinOperstate`] (enum)"]
    Operstate(u32),
}
impl<'a> IterablePinParentDevice<'a> {
    pub fn get_parent_id(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(PinParentDevice::ParentId(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "PinParentDevice",
            "ParentId",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Associated type: [`PinDirection`] (enum)"]
    pub fn get_direction(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(PinParentDevice::Direction(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "PinParentDevice",
            "Direction",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_prio(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(PinParentDevice::Prio(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "PinParentDevice",
            "Prio",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Associated type: [`PinState`] (enum)"]
    pub fn get_state(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(PinParentDevice::State(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "PinParentDevice",
            "State",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_phase_offset(&self) -> Result<i64, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(PinParentDevice::PhaseOffset(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "PinParentDevice",
            "PhaseOffset",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "The FFO (Fractional Frequency Offset) of the pin. At top level this\nrepresents the RX vs TX symbol rate offset on the media associated with\nthe pin. Inside the pin-parent-device nest it represents the frequency\noffset between the pin and its parent DPLL device. Value is in PPM\n(parts per million). This is a lower-precision version of\nfractional-frequency-offset-ppt.\n"]
    pub fn get_fractional_frequency_offset(&self) -> Result<i32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(PinParentDevice::FractionalFrequencyOffset(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "PinParentDevice",
            "FractionalFrequencyOffset",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "The FFO (Fractional Frequency Offset) of the pin. At top level this\nrepresents the RX vs TX symbol rate offset on the media associated with\nthe pin. Inside the pin-parent-device nest it represents the frequency\noffset between the pin and its parent DPLL device. Value is in PPT\n(parts per trillion, 10\\^-12). This is a higher-precision version of\nfractional-frequency-offset.\n"]
    pub fn get_fractional_frequency_offset_ppt(&self) -> Result<i32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(PinParentDevice::FractionalFrequencyOffsetPpt(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "PinParentDevice",
            "FractionalFrequencyOffsetPpt",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Operational state of the pin with respect to its parent DPLL device.\nUnlike state (which reflects the administrative intent), operstate\nreflects the actual hardware status.\n\nAssociated type: [`PinOperstate`] (enum)"]
    pub fn get_operstate(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(PinParentDevice::Operstate(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "PinParentDevice",
            "Operstate",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl PinParentDevice {
    pub fn new<'a>(buf: &'a [u8]) -> IterablePinParentDevice<'a> {
        IterablePinParentDevice::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        Pin::attr_from_type(r#type)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterablePinParentDevice<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterablePinParentDevice<'a> {
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
impl<'a> Iterator for IterablePinParentDevice<'a> {
    type Item = Result<PinParentDevice, ErrorContext>;
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
                2u16 => PinParentDevice::ParentId({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                10u16 => PinParentDevice::Direction({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                15u16 => PinParentDevice::Prio({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                16u16 => PinParentDevice::State({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                23u16 => PinParentDevice::PhaseOffset({
                    let res = parse_i64(next);
                    let Some(val) = res else { break };
                    val
                }),
                24u16 => PinParentDevice::FractionalFrequencyOffset({
                    let res = parse_i32(next);
                    let Some(val) = res else { break };
                    val
                }),
                30u16 => PinParentDevice::FractionalFrequencyOffsetPpt({
                    let res = parse_i32(next);
                    let Some(val) = res else { break };
                    val
                }),
                32u16 => PinParentDevice::Operstate({
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
            "PinParentDevice",
            r#type.and_then(|t| PinParentDevice::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl std::fmt::Debug for IterablePinParentDevice<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("PinParentDevice");
        let mut iter = IterablePinParentDevice::with_loc(&[], self.orig_loc);
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
                PinParentDevice::ParentId(val) => fmt.field("ParentId", &val),
                PinParentDevice::Direction(val) => fmt.field(
                    "Direction",
                    &FormatEnum(val.into(), PinDirection::from_value),
                ),
                PinParentDevice::Prio(val) => fmt.field("Prio", &val),
                PinParentDevice::State(val) => {
                    fmt.field("State", &FormatEnum(val.into(), PinState::from_value))
                }
                PinParentDevice::PhaseOffset(val) => fmt.field("PhaseOffset", &val),
                PinParentDevice::FractionalFrequencyOffset(val) => {
                    fmt.field("FractionalFrequencyOffset", &val)
                }
                PinParentDevice::FractionalFrequencyOffsetPpt(val) => {
                    fmt.field("FractionalFrequencyOffsetPpt", &val)
                }
                PinParentDevice::Operstate(val) => fmt.field(
                    "Operstate",
                    &FormatEnum(val.into(), PinOperstate::from_value),
                ),
            };
        }
        fmt.finish()
    }
}
impl IterablePinParentDevice<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("PinParentDevice", offset));
            return (
                stack,
                missing_type.and_then(|t| PinParentDevice::attr_from_type(t)),
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
                PinParentDevice::ParentId(val) => {
                    if last_off == offset {
                        stack.push(("ParentId", last_off));
                        break;
                    }
                }
                PinParentDevice::Direction(val) => {
                    if last_off == offset {
                        stack.push(("Direction", last_off));
                        break;
                    }
                }
                PinParentDevice::Prio(val) => {
                    if last_off == offset {
                        stack.push(("Prio", last_off));
                        break;
                    }
                }
                PinParentDevice::State(val) => {
                    if last_off == offset {
                        stack.push(("State", last_off));
                        break;
                    }
                }
                PinParentDevice::PhaseOffset(val) => {
                    if last_off == offset {
                        stack.push(("PhaseOffset", last_off));
                        break;
                    }
                }
                PinParentDevice::FractionalFrequencyOffset(val) => {
                    if last_off == offset {
                        stack.push(("FractionalFrequencyOffset", last_off));
                        break;
                    }
                }
                PinParentDevice::FractionalFrequencyOffsetPpt(val) => {
                    if last_off == offset {
                        stack.push(("FractionalFrequencyOffsetPpt", last_off));
                        break;
                    }
                }
                PinParentDevice::Operstate(val) => {
                    if last_off == offset {
                        stack.push(("Operstate", last_off));
                        break;
                    }
                }
                _ => {}
            };
            last_off = cur + attrs.pos;
        }
        if !stack.is_empty() {
            stack.push(("PinParentDevice", cur));
        }
        (stack, None)
    }
}
#[derive(Clone)]
pub enum PinParentPin {
    ParentId(u32),
    #[doc = "Associated type: [`PinState`] (enum)"]
    State(u32),
}
impl<'a> IterablePinParentPin<'a> {
    pub fn get_parent_id(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(PinParentPin::ParentId(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "PinParentPin",
            "ParentId",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Associated type: [`PinState`] (enum)"]
    pub fn get_state(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(PinParentPin::State(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "PinParentPin",
            "State",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl PinParentPin {
    pub fn new<'a>(buf: &'a [u8]) -> IterablePinParentPin<'a> {
        IterablePinParentPin::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        Pin::attr_from_type(r#type)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterablePinParentPin<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterablePinParentPin<'a> {
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
impl<'a> Iterator for IterablePinParentPin<'a> {
    type Item = Result<PinParentPin, ErrorContext>;
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
                2u16 => PinParentPin::ParentId({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                16u16 => PinParentPin::State({
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
            "PinParentPin",
            r#type.and_then(|t| PinParentPin::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl std::fmt::Debug for IterablePinParentPin<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("PinParentPin");
        let mut iter = IterablePinParentPin::with_loc(&[], self.orig_loc);
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
                PinParentPin::ParentId(val) => fmt.field("ParentId", &val),
                PinParentPin::State(val) => {
                    fmt.field("State", &FormatEnum(val.into(), PinState::from_value))
                }
            };
        }
        fmt.finish()
    }
}
impl IterablePinParentPin<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("PinParentPin", offset));
            return (
                stack,
                missing_type.and_then(|t| PinParentPin::attr_from_type(t)),
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
                PinParentPin::ParentId(val) => {
                    if last_off == offset {
                        stack.push(("ParentId", last_off));
                        break;
                    }
                }
                PinParentPin::State(val) => {
                    if last_off == offset {
                        stack.push(("State", last_off));
                        break;
                    }
                }
                _ => {}
            };
            last_off = cur + attrs.pos;
        }
        if !stack.is_empty() {
            stack.push(("PinParentPin", cur));
        }
        (stack, None)
    }
}
#[derive(Clone)]
pub enum FrequencyRange {
    FrequencyMin(u64),
    FrequencyMax(u64),
}
impl<'a> IterableFrequencyRange<'a> {
    pub fn get_frequency_min(&self) -> Result<u64, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(FrequencyRange::FrequencyMin(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "FrequencyRange",
            "FrequencyMin",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_frequency_max(&self) -> Result<u64, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(FrequencyRange::FrequencyMax(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "FrequencyRange",
            "FrequencyMax",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl FrequencyRange {
    pub fn new<'a>(buf: &'a [u8]) -> IterableFrequencyRange<'a> {
        IterableFrequencyRange::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        Pin::attr_from_type(r#type)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableFrequencyRange<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableFrequencyRange<'a> {
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
impl<'a> Iterator for IterableFrequencyRange<'a> {
    type Item = Result<FrequencyRange, ErrorContext>;
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
                13u16 => FrequencyRange::FrequencyMin({
                    let res = parse_u64(next);
                    let Some(val) = res else { break };
                    val
                }),
                14u16 => FrequencyRange::FrequencyMax({
                    let res = parse_u64(next);
                    let Some(val) = res else { break };
                    val
                }),
                n if cfg!(any(test, feature = "deny-unknown-attrs")) => break,
                n => continue,
            };
            return Some(Ok(res));
        }
        Some(Err(ErrorContext::new(
            "FrequencyRange",
            r#type.and_then(|t| FrequencyRange::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl std::fmt::Debug for IterableFrequencyRange<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("FrequencyRange");
        let mut iter = IterableFrequencyRange::with_loc(&[], self.orig_loc);
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
                FrequencyRange::FrequencyMin(val) => fmt.field("FrequencyMin", &val),
                FrequencyRange::FrequencyMax(val) => fmt.field("FrequencyMax", &val),
            };
        }
        fmt.finish()
    }
}
impl IterableFrequencyRange<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("FrequencyRange", offset));
            return (
                stack,
                missing_type.and_then(|t| FrequencyRange::attr_from_type(t)),
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
                FrequencyRange::FrequencyMin(val) => {
                    if last_off == offset {
                        stack.push(("FrequencyMin", last_off));
                        break;
                    }
                }
                FrequencyRange::FrequencyMax(val) => {
                    if last_off == offset {
                        stack.push(("FrequencyMax", last_off));
                        break;
                    }
                }
                _ => {}
            };
            last_off = cur + attrs.pos;
        }
        if !stack.is_empty() {
            stack.push(("FrequencyRange", cur));
        }
        (stack, None)
    }
}
#[derive(Clone)]
pub enum ReferenceSync {
    Id(u32),
    #[doc = "Associated type: [`PinState`] (enum)"]
    State(u32),
}
impl<'a> IterableReferenceSync<'a> {
    pub fn get_id(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(ReferenceSync::Id(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "ReferenceSync",
            "Id",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Associated type: [`PinState`] (enum)"]
    pub fn get_state(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(ReferenceSync::State(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "ReferenceSync",
            "State",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl ReferenceSync {
    pub fn new<'a>(buf: &'a [u8]) -> IterableReferenceSync<'a> {
        IterableReferenceSync::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        Pin::attr_from_type(r#type)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableReferenceSync<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableReferenceSync<'a> {
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
impl<'a> Iterator for IterableReferenceSync<'a> {
    type Item = Result<ReferenceSync, ErrorContext>;
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
                1u16 => ReferenceSync::Id({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                16u16 => ReferenceSync::State({
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
            "ReferenceSync",
            r#type.and_then(|t| ReferenceSync::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl std::fmt::Debug for IterableReferenceSync<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("ReferenceSync");
        let mut iter = IterableReferenceSync::with_loc(&[], self.orig_loc);
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
                ReferenceSync::Id(val) => fmt.field("Id", &val),
                ReferenceSync::State(val) => {
                    fmt.field("State", &FormatEnum(val.into(), PinState::from_value))
                }
            };
        }
        fmt.finish()
    }
}
impl IterableReferenceSync<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("ReferenceSync", offset));
            return (
                stack,
                missing_type.and_then(|t| ReferenceSync::attr_from_type(t)),
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
                ReferenceSync::Id(val) => {
                    if last_off == offset {
                        stack.push(("Id", last_off));
                        break;
                    }
                }
                ReferenceSync::State(val) => {
                    if last_off == offset {
                        stack.push(("State", last_off));
                        break;
                    }
                }
                _ => {}
            };
            last_off = cur + attrs.pos;
        }
        if !stack.is_empty() {
            stack.push(("ReferenceSync", cur));
        }
        (stack, None)
    }
}
pub struct PushDpll<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushDpll<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushDpll<Prev> {
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
    pub fn push_id(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 1u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_module_name(mut self, value: &CStr) -> Self {
        push_header(
            self.as_vec_mut(),
            2u16,
            value.to_bytes_with_nul().len() as u16,
        );
        self.as_vec_mut().extend(value.to_bytes_with_nul());
        self
    }
    pub fn push_module_name_bytes(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 2u16, (value.len() + 1) as u16);
        self.as_vec_mut().extend(value);
        self.as_vec_mut().push(0);
        self
    }
    pub fn push_pad(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 3u16, value.len() as u16);
        self.as_vec_mut().extend(value);
        self
    }
    pub fn push_clock_id(mut self, value: u64) -> Self {
        push_header(self.as_vec_mut(), 4u16, 8 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Associated type: [`Mode`] (enum)"]
    pub fn push_mode(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 5u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Associated type: [`Mode`] (enum)\nAttribute may repeat multiple times (treat it as array)"]
    pub fn push_mode_supported(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 6u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Associated type: [`LockStatus`] (enum)"]
    pub fn push_lock_status(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 7u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_temp(mut self, value: i32) -> Self {
        push_header(self.as_vec_mut(), 8u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Associated type: [`Type`] (enum)"]
    pub fn push_type(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 9u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Associated type: [`LockStatusError`] (enum)"]
    pub fn push_lock_status_error(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 10u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Level of quality of a clock device. This mainly applies when the dpll\nlock-status is DPLL_LOCK_STATUS_HOLDOVER. This could be put to message\nmultiple times to indicate possible parallel quality levels (e.g. one\nspecified by ITU option 1 and another one specified by option 2).\n\nAssociated type: [`ClockQualityLevel`] (enum)\nAttribute may repeat multiple times (treat it as array)"]
    pub fn push_clock_quality_level(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 11u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Receive or request state of phase offset monitor feature. If enabled,\ndpll device shall monitor and notify all currently available inputs for\nchanges of their phase offset against the dpll device.\n\nAssociated type: [`FeatureState`] (enum)"]
    pub fn push_phase_offset_monitor(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 12u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Averaging factor applied to calculation of reported phase offset.\n"]
    pub fn push_phase_offset_avg_factor(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 13u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Current or desired state of the frequency monitor feature. If enabled,\ndpll device shall measure all currently available inputs for their\nactual input frequency.\n\nAssociated type: [`FeatureState`] (enum)"]
    pub fn push_frequency_monitor(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 14u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
}
impl<Prev: Pusher> Drop for PushDpll<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushPin<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushPin<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushPin<Prev> {
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
    pub fn push_id(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 1u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_parent_id(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 2u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_module_name(mut self, value: &CStr) -> Self {
        push_header(
            self.as_vec_mut(),
            3u16,
            value.to_bytes_with_nul().len() as u16,
        );
        self.as_vec_mut().extend(value.to_bytes_with_nul());
        self
    }
    pub fn push_module_name_bytes(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 3u16, (value.len() + 1) as u16);
        self.as_vec_mut().extend(value);
        self.as_vec_mut().push(0);
        self
    }
    pub fn push_pad(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 4u16, value.len() as u16);
        self.as_vec_mut().extend(value);
        self
    }
    pub fn push_clock_id(mut self, value: u64) -> Self {
        push_header(self.as_vec_mut(), 5u16, 8 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_board_label(mut self, value: &CStr) -> Self {
        push_header(
            self.as_vec_mut(),
            6u16,
            value.to_bytes_with_nul().len() as u16,
        );
        self.as_vec_mut().extend(value.to_bytes_with_nul());
        self
    }
    pub fn push_board_label_bytes(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 6u16, (value.len() + 1) as u16);
        self.as_vec_mut().extend(value);
        self.as_vec_mut().push(0);
        self
    }
    pub fn push_panel_label(mut self, value: &CStr) -> Self {
        push_header(
            self.as_vec_mut(),
            7u16,
            value.to_bytes_with_nul().len() as u16,
        );
        self.as_vec_mut().extend(value.to_bytes_with_nul());
        self
    }
    pub fn push_panel_label_bytes(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 7u16, (value.len() + 1) as u16);
        self.as_vec_mut().extend(value);
        self.as_vec_mut().push(0);
        self
    }
    pub fn push_package_label(mut self, value: &CStr) -> Self {
        push_header(
            self.as_vec_mut(),
            8u16,
            value.to_bytes_with_nul().len() as u16,
        );
        self.as_vec_mut().extend(value.to_bytes_with_nul());
        self
    }
    pub fn push_package_label_bytes(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 8u16, (value.len() + 1) as u16);
        self.as_vec_mut().extend(value);
        self.as_vec_mut().push(0);
        self
    }
    #[doc = "Associated type: [`PinType`] (enum)"]
    pub fn push_type(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 9u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Associated type: [`PinDirection`] (enum)"]
    pub fn push_direction(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 10u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_frequency(mut self, value: u64) -> Self {
        push_header(self.as_vec_mut(), 11u16, 8 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Attribute may repeat multiple times (treat it as array)"]
    pub fn nested_frequency_supported(mut self) -> PushFrequencyRange<Self> {
        let header_offset = push_nested_header(self.as_vec_mut(), 12u16);
        PushFrequencyRange {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
    pub fn push_frequency_min(mut self, value: u64) -> Self {
        push_header(self.as_vec_mut(), 13u16, 8 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_frequency_max(mut self, value: u64) -> Self {
        push_header(self.as_vec_mut(), 14u16, 8 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_prio(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 15u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Associated type: [`PinState`] (enum)"]
    pub fn push_state(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 16u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Associated type: [`PinCapabilities`] (enum)"]
    pub fn push_capabilities(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 17u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Attribute may repeat multiple times (treat it as array)"]
    pub fn nested_parent_device(mut self) -> PushPinParentDevice<Self> {
        let header_offset = push_nested_header(self.as_vec_mut(), 18u16);
        PushPinParentDevice {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
    #[doc = "Attribute may repeat multiple times (treat it as array)"]
    pub fn nested_parent_pin(mut self) -> PushPinParentPin<Self> {
        let header_offset = push_nested_header(self.as_vec_mut(), 19u16);
        PushPinParentPin {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
    pub fn push_phase_adjust_min(mut self, value: i32) -> Self {
        push_header(self.as_vec_mut(), 20u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_phase_adjust_max(mut self, value: i32) -> Self {
        push_header(self.as_vec_mut(), 21u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_phase_adjust(mut self, value: i32) -> Self {
        push_header(self.as_vec_mut(), 22u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_phase_offset(mut self, value: i64) -> Self {
        push_header(self.as_vec_mut(), 23u16, 8 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "The FFO (Fractional Frequency Offset) of the pin. At top level this\nrepresents the RX vs TX symbol rate offset on the media associated with\nthe pin. Inside the pin-parent-device nest it represents the frequency\noffset between the pin and its parent DPLL device. Value is in PPM\n(parts per million). This is a lower-precision version of\nfractional-frequency-offset-ppt.\n"]
    pub fn push_fractional_frequency_offset(mut self, value: i32) -> Self {
        push_header(self.as_vec_mut(), 24u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Frequency of Embedded SYNC signal. If provided, the pin is configured\nwith a SYNC signal embedded into its base clock frequency.\n"]
    pub fn push_esync_frequency(mut self, value: u64) -> Self {
        push_header(self.as_vec_mut(), 25u16, 8 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "If provided a pin is capable of embedding a SYNC signal (within given\nrange) into its base frequency signal.\n\nAttribute may repeat multiple times (treat it as array)"]
    pub fn nested_esync_frequency_supported(mut self) -> PushFrequencyRange<Self> {
        let header_offset = push_nested_header(self.as_vec_mut(), 26u16);
        PushFrequencyRange {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
    #[doc = "A ratio of high to low state of a SYNC signal pulse embedded into base\nclock frequency. Value is in percents.\n"]
    pub fn push_esync_pulse(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 27u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Capable pin provides list of pins that can be bound to create a\nreference-sync pin pair.\n\nAttribute may repeat multiple times (treat it as array)"]
    pub fn nested_reference_sync(mut self) -> PushReferenceSync<Self> {
        let header_offset = push_nested_header(self.as_vec_mut(), 28u16);
        PushReferenceSync {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
    #[doc = "Granularity of phase adjustment, in picoseconds. The value of phase\nadjustment must be a multiple of this granularity.\n"]
    pub fn push_phase_adjust_gran(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 29u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "The FFO (Fractional Frequency Offset) of the pin. At top level this\nrepresents the RX vs TX symbol rate offset on the media associated with\nthe pin. Inside the pin-parent-device nest it represents the frequency\noffset between the pin and its parent DPLL device. Value is in PPT\n(parts per trillion, 10\\^-12). This is a higher-precision version of\nfractional-frequency-offset.\n"]
    pub fn push_fractional_frequency_offset_ppt(mut self, value: i32) -> Self {
        push_header(self.as_vec_mut(), 30u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "The measured frequency of the input pin in millihertz (mHz). Value of\n(DPLL_A_PIN_MEASURED_FREQUENCY / DPLL_PIN_MEASURED_FREQUENCY_DIVIDER) is\nan integer part (Hz) of a measured frequency value. Value of\n(DPLL_A_PIN_MEASURED_FREQUENCY % DPLL_PIN_MEASURED_FREQUENCY_DIVIDER) is\na fractional part of a measured frequency value.\n"]
    pub fn push_measured_frequency(mut self, value: u64) -> Self {
        push_header(self.as_vec_mut(), 31u16, 8 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Operational state of the pin with respect to its parent DPLL device.\nUnlike state (which reflects the administrative intent), operstate\nreflects the actual hardware status.\n\nAssociated type: [`PinOperstate`] (enum)"]
    pub fn push_operstate(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 32u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
}
impl<Prev: Pusher> Drop for PushPin<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushPinParentDevice<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushPinParentDevice<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushPinParentDevice<Prev> {
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
    pub fn push_parent_id(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 2u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Associated type: [`PinDirection`] (enum)"]
    pub fn push_direction(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 10u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_prio(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 15u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Associated type: [`PinState`] (enum)"]
    pub fn push_state(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 16u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_phase_offset(mut self, value: i64) -> Self {
        push_header(self.as_vec_mut(), 23u16, 8 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "The FFO (Fractional Frequency Offset) of the pin. At top level this\nrepresents the RX vs TX symbol rate offset on the media associated with\nthe pin. Inside the pin-parent-device nest it represents the frequency\noffset between the pin and its parent DPLL device. Value is in PPM\n(parts per million). This is a lower-precision version of\nfractional-frequency-offset-ppt.\n"]
    pub fn push_fractional_frequency_offset(mut self, value: i32) -> Self {
        push_header(self.as_vec_mut(), 24u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "The FFO (Fractional Frequency Offset) of the pin. At top level this\nrepresents the RX vs TX symbol rate offset on the media associated with\nthe pin. Inside the pin-parent-device nest it represents the frequency\noffset between the pin and its parent DPLL device. Value is in PPT\n(parts per trillion, 10\\^-12). This is a higher-precision version of\nfractional-frequency-offset.\n"]
    pub fn push_fractional_frequency_offset_ppt(mut self, value: i32) -> Self {
        push_header(self.as_vec_mut(), 30u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Operational state of the pin with respect to its parent DPLL device.\nUnlike state (which reflects the administrative intent), operstate\nreflects the actual hardware status.\n\nAssociated type: [`PinOperstate`] (enum)"]
    pub fn push_operstate(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 32u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
}
impl<Prev: Pusher> Drop for PushPinParentDevice<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushPinParentPin<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushPinParentPin<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushPinParentPin<Prev> {
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
    pub fn push_parent_id(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 2u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Associated type: [`PinState`] (enum)"]
    pub fn push_state(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 16u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
}
impl<Prev: Pusher> Drop for PushPinParentPin<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushFrequencyRange<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushFrequencyRange<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushFrequencyRange<Prev> {
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
    pub fn push_frequency_min(mut self, value: u64) -> Self {
        push_header(self.as_vec_mut(), 13u16, 8 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_frequency_max(mut self, value: u64) -> Self {
        push_header(self.as_vec_mut(), 14u16, 8 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
}
impl<Prev: Pusher> Drop for PushFrequencyRange<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushReferenceSync<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushReferenceSync<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushReferenceSync<Prev> {
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
    pub fn push_id(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 1u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Associated type: [`PinState`] (enum)"]
    pub fn push_state(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 16u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
}
impl<Prev: Pusher> Drop for PushReferenceSync<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
#[doc = "Notify attributes:\n- [`.get_id()`](IterableDpll::get_id)\n- [`.get_module_name()`](IterableDpll::get_module_name)\n- [`.get_mode()`](IterableDpll::get_mode)\n- [`.get_mode_supported()`](IterableDpll::get_mode_supported)\n- [`.get_lock_status()`](IterableDpll::get_lock_status)\n- [`.get_lock_status_error()`](IterableDpll::get_lock_status_error)\n- [`.get_temp()`](IterableDpll::get_temp)\n- [`.get_clock_id()`](IterableDpll::get_clock_id)\n- [`.get_type()`](IterableDpll::get_type)\n- [`.get_phase_offset_monitor()`](IterableDpll::get_phase_offset_monitor)\n- [`.get_phase_offset_avg_factor()`](IterableDpll::get_phase_offset_avg_factor)\n- [`.get_frequency_monitor()`](IterableDpll::get_frequency_monitor)\n"]
#[derive(Debug)]
pub struct OpDeviceCreateNotif;
impl OpDeviceCreateNotif {
    pub const CMD: u8 = 4u8;
    pub fn decode_notif<'a>(buf: &'a [u8]) -> IterableDpll<'a> {
        let (_header, attrs) = buf.split_at(buf.len().min(BuiltinNfgenmsg::len()));
        IterableDpll::with_loc(attrs, buf.as_ptr() as usize)
    }
}
#[doc = "Notify attributes:\n- [`.get_id()`](IterableDpll::get_id)\n- [`.get_module_name()`](IterableDpll::get_module_name)\n- [`.get_mode()`](IterableDpll::get_mode)\n- [`.get_mode_supported()`](IterableDpll::get_mode_supported)\n- [`.get_lock_status()`](IterableDpll::get_lock_status)\n- [`.get_lock_status_error()`](IterableDpll::get_lock_status_error)\n- [`.get_temp()`](IterableDpll::get_temp)\n- [`.get_clock_id()`](IterableDpll::get_clock_id)\n- [`.get_type()`](IterableDpll::get_type)\n- [`.get_phase_offset_monitor()`](IterableDpll::get_phase_offset_monitor)\n- [`.get_phase_offset_avg_factor()`](IterableDpll::get_phase_offset_avg_factor)\n- [`.get_frequency_monitor()`](IterableDpll::get_frequency_monitor)\n"]
#[derive(Debug)]
pub struct OpDeviceDeleteNotif;
impl OpDeviceDeleteNotif {
    pub const CMD: u8 = 5u8;
    pub fn decode_notif<'a>(buf: &'a [u8]) -> IterableDpll<'a> {
        let (_header, attrs) = buf.split_at(buf.len().min(BuiltinNfgenmsg::len()));
        IterableDpll::with_loc(attrs, buf.as_ptr() as usize)
    }
}
#[doc = "Notify attributes:\n- [`.get_id()`](IterableDpll::get_id)\n- [`.get_module_name()`](IterableDpll::get_module_name)\n- [`.get_mode()`](IterableDpll::get_mode)\n- [`.get_mode_supported()`](IterableDpll::get_mode_supported)\n- [`.get_lock_status()`](IterableDpll::get_lock_status)\n- [`.get_lock_status_error()`](IterableDpll::get_lock_status_error)\n- [`.get_temp()`](IterableDpll::get_temp)\n- [`.get_clock_id()`](IterableDpll::get_clock_id)\n- [`.get_type()`](IterableDpll::get_type)\n- [`.get_phase_offset_monitor()`](IterableDpll::get_phase_offset_monitor)\n- [`.get_phase_offset_avg_factor()`](IterableDpll::get_phase_offset_avg_factor)\n- [`.get_frequency_monitor()`](IterableDpll::get_frequency_monitor)\n"]
#[derive(Debug)]
pub struct OpDeviceChangeNotif;
impl OpDeviceChangeNotif {
    pub const CMD: u8 = 6u8;
    pub fn decode_notif<'a>(buf: &'a [u8]) -> IterableDpll<'a> {
        let (_header, attrs) = buf.split_at(buf.len().min(BuiltinNfgenmsg::len()));
        IterableDpll::with_loc(attrs, buf.as_ptr() as usize)
    }
}
#[doc = "Notify attributes:\n- [`.get_id()`](IterablePin::get_id)\n- [`.get_module_name()`](IterablePin::get_module_name)\n- [`.get_clock_id()`](IterablePin::get_clock_id)\n- [`.get_board_label()`](IterablePin::get_board_label)\n- [`.get_panel_label()`](IterablePin::get_panel_label)\n- [`.get_package_label()`](IterablePin::get_package_label)\n- [`.get_type()`](IterablePin::get_type)\n- [`.get_frequency()`](IterablePin::get_frequency)\n- [`.get_frequency_supported()`](IterablePin::get_frequency_supported)\n- [`.get_capabilities()`](IterablePin::get_capabilities)\n- [`.get_parent_device()`](IterablePin::get_parent_device)\n- [`.get_parent_pin()`](IterablePin::get_parent_pin)\n- [`.get_phase_adjust_gran()`](IterablePin::get_phase_adjust_gran)\n- [`.get_phase_adjust_min()`](IterablePin::get_phase_adjust_min)\n- [`.get_phase_adjust_max()`](IterablePin::get_phase_adjust_max)\n- [`.get_phase_adjust()`](IterablePin::get_phase_adjust)\n- [`.get_fractional_frequency_offset()`](IterablePin::get_fractional_frequency_offset)\n- [`.get_fractional_frequency_offset_ppt()`](IterablePin::get_fractional_frequency_offset_ppt)\n- [`.get_esync_frequency()`](IterablePin::get_esync_frequency)\n- [`.get_esync_frequency_supported()`](IterablePin::get_esync_frequency_supported)\n- [`.get_esync_pulse()`](IterablePin::get_esync_pulse)\n- [`.get_reference_sync()`](IterablePin::get_reference_sync)\n- [`.get_measured_frequency()`](IterablePin::get_measured_frequency)\n"]
#[derive(Debug)]
pub struct OpPinCreateNotif;
impl OpPinCreateNotif {
    pub const CMD: u8 = 10u8;
    pub fn decode_notif<'a>(buf: &'a [u8]) -> IterablePin<'a> {
        let (_header, attrs) = buf.split_at(buf.len().min(BuiltinNfgenmsg::len()));
        IterablePin::with_loc(attrs, buf.as_ptr() as usize)
    }
}
#[doc = "Notify attributes:\n- [`.get_id()`](IterablePin::get_id)\n- [`.get_module_name()`](IterablePin::get_module_name)\n- [`.get_clock_id()`](IterablePin::get_clock_id)\n- [`.get_board_label()`](IterablePin::get_board_label)\n- [`.get_panel_label()`](IterablePin::get_panel_label)\n- [`.get_package_label()`](IterablePin::get_package_label)\n- [`.get_type()`](IterablePin::get_type)\n- [`.get_frequency()`](IterablePin::get_frequency)\n- [`.get_frequency_supported()`](IterablePin::get_frequency_supported)\n- [`.get_capabilities()`](IterablePin::get_capabilities)\n- [`.get_parent_device()`](IterablePin::get_parent_device)\n- [`.get_parent_pin()`](IterablePin::get_parent_pin)\n- [`.get_phase_adjust_gran()`](IterablePin::get_phase_adjust_gran)\n- [`.get_phase_adjust_min()`](IterablePin::get_phase_adjust_min)\n- [`.get_phase_adjust_max()`](IterablePin::get_phase_adjust_max)\n- [`.get_phase_adjust()`](IterablePin::get_phase_adjust)\n- [`.get_fractional_frequency_offset()`](IterablePin::get_fractional_frequency_offset)\n- [`.get_fractional_frequency_offset_ppt()`](IterablePin::get_fractional_frequency_offset_ppt)\n- [`.get_esync_frequency()`](IterablePin::get_esync_frequency)\n- [`.get_esync_frequency_supported()`](IterablePin::get_esync_frequency_supported)\n- [`.get_esync_pulse()`](IterablePin::get_esync_pulse)\n- [`.get_reference_sync()`](IterablePin::get_reference_sync)\n- [`.get_measured_frequency()`](IterablePin::get_measured_frequency)\n"]
#[derive(Debug)]
pub struct OpPinDeleteNotif;
impl OpPinDeleteNotif {
    pub const CMD: u8 = 11u8;
    pub fn decode_notif<'a>(buf: &'a [u8]) -> IterablePin<'a> {
        let (_header, attrs) = buf.split_at(buf.len().min(BuiltinNfgenmsg::len()));
        IterablePin::with_loc(attrs, buf.as_ptr() as usize)
    }
}
#[doc = "Notify attributes:\n- [`.get_id()`](IterablePin::get_id)\n- [`.get_module_name()`](IterablePin::get_module_name)\n- [`.get_clock_id()`](IterablePin::get_clock_id)\n- [`.get_board_label()`](IterablePin::get_board_label)\n- [`.get_panel_label()`](IterablePin::get_panel_label)\n- [`.get_package_label()`](IterablePin::get_package_label)\n- [`.get_type()`](IterablePin::get_type)\n- [`.get_frequency()`](IterablePin::get_frequency)\n- [`.get_frequency_supported()`](IterablePin::get_frequency_supported)\n- [`.get_capabilities()`](IterablePin::get_capabilities)\n- [`.get_parent_device()`](IterablePin::get_parent_device)\n- [`.get_parent_pin()`](IterablePin::get_parent_pin)\n- [`.get_phase_adjust_gran()`](IterablePin::get_phase_adjust_gran)\n- [`.get_phase_adjust_min()`](IterablePin::get_phase_adjust_min)\n- [`.get_phase_adjust_max()`](IterablePin::get_phase_adjust_max)\n- [`.get_phase_adjust()`](IterablePin::get_phase_adjust)\n- [`.get_fractional_frequency_offset()`](IterablePin::get_fractional_frequency_offset)\n- [`.get_fractional_frequency_offset_ppt()`](IterablePin::get_fractional_frequency_offset_ppt)\n- [`.get_esync_frequency()`](IterablePin::get_esync_frequency)\n- [`.get_esync_frequency_supported()`](IterablePin::get_esync_frequency_supported)\n- [`.get_esync_pulse()`](IterablePin::get_esync_pulse)\n- [`.get_reference_sync()`](IterablePin::get_reference_sync)\n- [`.get_measured_frequency()`](IterablePin::get_measured_frequency)\n"]
#[derive(Debug)]
pub struct OpPinChangeNotif;
impl OpPinChangeNotif {
    pub const CMD: u8 = 12u8;
    pub fn decode_notif<'a>(buf: &'a [u8]) -> IterablePin<'a> {
        let (_header, attrs) = buf.split_at(buf.len().min(BuiltinNfgenmsg::len()));
        IterablePin::with_loc(attrs, buf.as_ptr() as usize)
    }
}
pub struct NotifGroup;
impl NotifGroup {
    #[doc = "Notifications:\n- [`OpDeviceCreateNotif`]\n- [`OpDeviceDeleteNotif`]\n- [`OpDeviceChangeNotif`]\n- [`OpPinCreateNotif`]\n- [`OpPinDeleteNotif`]\n- [`OpPinChangeNotif`]\n"]
    pub const MONITOR: &str = "monitor";
    #[doc = "Notifications:\n- [`OpDeviceCreateNotif`]\n- [`OpDeviceDeleteNotif`]\n- [`OpDeviceChangeNotif`]\n- [`OpPinCreateNotif`]\n- [`OpPinDeleteNotif`]\n- [`OpPinChangeNotif`]\n"]
    pub const MONITOR_CSTR: &CStr = c"monitor";
}
#[doc = "Get id of dpll device that matches given attributes\n\nFlags: admin-perm\n\nRequest attributes:\n- [.push_module_name()](PushDpll::push_module_name)\n- [.push_clock_id()](PushDpll::push_clock_id)\n- [.push_type()](PushDpll::push_type)\n\nReply attributes:\n- [.get_id()](IterableDpll::get_id)\n\n"]
#[derive(Debug)]
pub struct OpDeviceIdGetDo<'r> {
    request: Request<'r>,
}
impl<'r> OpDeviceIdGetDo<'r> {
    pub fn new(mut request: Request<'r>) -> Self {
        Self::write_header(request.buf_mut());
        Self { request: request }
    }
    pub fn encode_request<'buf>(buf: &'buf mut Vec<u8>) -> PushDpll<&'buf mut Vec<u8>> {
        Self::write_header(buf);
        PushDpll::new(buf)
    }
    pub fn encode(&mut self) -> PushDpll<&mut Vec<u8>> {
        PushDpll::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushDpll<RequestBuf<'r>> {
        PushDpll::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> IterableDpll<'a> {
        let (_header, attrs) = buf.split_at(buf.len().min(BuiltinNfgenmsg::len()));
        IterableDpll::with_loc(attrs, buf.as_ptr() as usize)
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
impl NetlinkRequest for OpDeviceIdGetDo<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Generic("dpll".as_bytes())
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = IterableDpll<'buf>;
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
#[doc = "Get list of DPLL devices (dump) or attributes of a single dpll device\n\nFlags: admin-perm\n\nReply attributes:\n- [.get_id()](IterableDpll::get_id)\n- [.get_module_name()](IterableDpll::get_module_name)\n- [.get_clock_id()](IterableDpll::get_clock_id)\n- [.get_mode()](IterableDpll::get_mode)\n- [.get_mode_supported()](IterableDpll::get_mode_supported)\n- [.get_lock_status()](IterableDpll::get_lock_status)\n- [.get_temp()](IterableDpll::get_temp)\n- [.get_type()](IterableDpll::get_type)\n- [.get_lock_status_error()](IterableDpll::get_lock_status_error)\n- [.get_phase_offset_monitor()](IterableDpll::get_phase_offset_monitor)\n- [.get_phase_offset_avg_factor()](IterableDpll::get_phase_offset_avg_factor)\n- [.get_frequency_monitor()](IterableDpll::get_frequency_monitor)\n\n"]
#[derive(Debug)]
pub struct OpDeviceGetDump<'r> {
    request: Request<'r>,
}
impl<'r> OpDeviceGetDump<'r> {
    pub fn new(mut request: Request<'r>) -> Self {
        Self::write_header(request.buf_mut());
        Self {
            request: request.set_dump(),
        }
    }
    pub fn encode_request<'buf>(buf: &'buf mut Vec<u8>) -> PushDpll<&'buf mut Vec<u8>> {
        Self::write_header(buf);
        PushDpll::new(buf)
    }
    pub fn encode(&mut self) -> PushDpll<&mut Vec<u8>> {
        PushDpll::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushDpll<RequestBuf<'r>> {
        PushDpll::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> IterableDpll<'a> {
        let (_header, attrs) = buf.split_at(buf.len().min(BuiltinNfgenmsg::len()));
        IterableDpll::with_loc(attrs, buf.as_ptr() as usize)
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
impl NetlinkRequest for OpDeviceGetDump<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Generic("dpll".as_bytes())
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = IterableDpll<'buf>;
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
#[doc = "Get list of DPLL devices (dump) or attributes of a single dpll device\n\nFlags: admin-perm\n\nRequest attributes:\n- [.push_id()](PushDpll::push_id)\n\nReply attributes:\n- [.get_id()](IterableDpll::get_id)\n- [.get_module_name()](IterableDpll::get_module_name)\n- [.get_clock_id()](IterableDpll::get_clock_id)\n- [.get_mode()](IterableDpll::get_mode)\n- [.get_mode_supported()](IterableDpll::get_mode_supported)\n- [.get_lock_status()](IterableDpll::get_lock_status)\n- [.get_temp()](IterableDpll::get_temp)\n- [.get_type()](IterableDpll::get_type)\n- [.get_lock_status_error()](IterableDpll::get_lock_status_error)\n- [.get_phase_offset_monitor()](IterableDpll::get_phase_offset_monitor)\n- [.get_phase_offset_avg_factor()](IterableDpll::get_phase_offset_avg_factor)\n- [.get_frequency_monitor()](IterableDpll::get_frequency_monitor)\n\n"]
#[derive(Debug)]
pub struct OpDeviceGetDo<'r> {
    request: Request<'r>,
}
impl<'r> OpDeviceGetDo<'r> {
    pub fn new(mut request: Request<'r>) -> Self {
        Self::write_header(request.buf_mut());
        Self { request: request }
    }
    pub fn encode_request<'buf>(buf: &'buf mut Vec<u8>) -> PushDpll<&'buf mut Vec<u8>> {
        Self::write_header(buf);
        PushDpll::new(buf)
    }
    pub fn encode(&mut self) -> PushDpll<&mut Vec<u8>> {
        PushDpll::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushDpll<RequestBuf<'r>> {
        PushDpll::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> IterableDpll<'a> {
        let (_header, attrs) = buf.split_at(buf.len().min(BuiltinNfgenmsg::len()));
        IterableDpll::with_loc(attrs, buf.as_ptr() as usize)
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
impl NetlinkRequest for OpDeviceGetDo<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Generic("dpll".as_bytes())
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = IterableDpll<'buf>;
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
#[doc = "Set attributes for a DPLL device\n\nFlags: admin-perm\n\nRequest attributes:\n- [.push_id()](PushDpll::push_id)\n- [.push_mode()](PushDpll::push_mode)\n- [.push_phase_offset_monitor()](PushDpll::push_phase_offset_monitor)\n- [.push_phase_offset_avg_factor()](PushDpll::push_phase_offset_avg_factor)\n- [.push_frequency_monitor()](PushDpll::push_frequency_monitor)\n\n"]
#[derive(Debug)]
pub struct OpDeviceSetDo<'r> {
    request: Request<'r>,
}
impl<'r> OpDeviceSetDo<'r> {
    pub fn new(mut request: Request<'r>) -> Self {
        Self::write_header(request.buf_mut());
        Self { request: request }
    }
    pub fn encode_request<'buf>(buf: &'buf mut Vec<u8>) -> PushDpll<&'buf mut Vec<u8>> {
        Self::write_header(buf);
        PushDpll::new(buf)
    }
    pub fn encode(&mut self) -> PushDpll<&mut Vec<u8>> {
        PushDpll::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushDpll<RequestBuf<'r>> {
        PushDpll::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> IterableDpll<'a> {
        let (_header, attrs) = buf.split_at(buf.len().min(BuiltinNfgenmsg::len()));
        IterableDpll::with_loc(attrs, buf.as_ptr() as usize)
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
impl NetlinkRequest for OpDeviceSetDo<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Generic("dpll".as_bytes())
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = IterableDpll<'buf>;
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
#[doc = "Get id of a pin that matches given attributes\n\nFlags: admin-perm\n\nRequest attributes:\n- [.push_module_name()](PushPin::push_module_name)\n- [.push_clock_id()](PushPin::push_clock_id)\n- [.push_board_label()](PushPin::push_board_label)\n- [.push_panel_label()](PushPin::push_panel_label)\n- [.push_package_label()](PushPin::push_package_label)\n- [.push_type()](PushPin::push_type)\n\nReply attributes:\n- [.get_id()](IterablePin::get_id)\n\n"]
#[derive(Debug)]
pub struct OpPinIdGetDo<'r> {
    request: Request<'r>,
}
impl<'r> OpPinIdGetDo<'r> {
    pub fn new(mut request: Request<'r>) -> Self {
        Self::write_header(request.buf_mut());
        Self { request: request }
    }
    pub fn encode_request<'buf>(buf: &'buf mut Vec<u8>) -> PushPin<&'buf mut Vec<u8>> {
        Self::write_header(buf);
        PushPin::new(buf)
    }
    pub fn encode(&mut self) -> PushPin<&mut Vec<u8>> {
        PushPin::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushPin<RequestBuf<'r>> {
        PushPin::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> IterablePin<'a> {
        let (_header, attrs) = buf.split_at(buf.len().min(BuiltinNfgenmsg::len()));
        IterablePin::with_loc(attrs, buf.as_ptr() as usize)
    }
    fn write_header<Prev: Pusher>(prev: &mut Prev) {
        let mut header = BuiltinNfgenmsg::new();
        header.cmd = 7u8;
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
impl NetlinkRequest for OpPinIdGetDo<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Generic("dpll".as_bytes())
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = IterablePin<'buf>;
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
#[doc = "Get list of pins and its attributes.\n\n- dump request without any attributes given - list all the pins in the\n  system\n- dump request with target dpll - list all the pins registered with a\n  given dpll device\n- do request with target dpll and target pin - single pin attributes\n\nFlags: admin-perm\n\nRequest attributes:\n- [.push_id()](PushPin::push_id)\n\nReply attributes:\n- [.get_id()](IterablePin::get_id)\n- [.get_module_name()](IterablePin::get_module_name)\n- [.get_clock_id()](IterablePin::get_clock_id)\n- [.get_board_label()](IterablePin::get_board_label)\n- [.get_panel_label()](IterablePin::get_panel_label)\n- [.get_package_label()](IterablePin::get_package_label)\n- [.get_type()](IterablePin::get_type)\n- [.get_frequency()](IterablePin::get_frequency)\n- [.get_frequency_supported()](IterablePin::get_frequency_supported)\n- [.get_capabilities()](IterablePin::get_capabilities)\n- [.get_parent_device()](IterablePin::get_parent_device)\n- [.get_parent_pin()](IterablePin::get_parent_pin)\n- [.get_phase_adjust_min()](IterablePin::get_phase_adjust_min)\n- [.get_phase_adjust_max()](IterablePin::get_phase_adjust_max)\n- [.get_phase_adjust()](IterablePin::get_phase_adjust)\n- [.get_fractional_frequency_offset()](IterablePin::get_fractional_frequency_offset)\n- [.get_esync_frequency()](IterablePin::get_esync_frequency)\n- [.get_esync_frequency_supported()](IterablePin::get_esync_frequency_supported)\n- [.get_esync_pulse()](IterablePin::get_esync_pulse)\n- [.get_reference_sync()](IterablePin::get_reference_sync)\n- [.get_phase_adjust_gran()](IterablePin::get_phase_adjust_gran)\n- [.get_fractional_frequency_offset_ppt()](IterablePin::get_fractional_frequency_offset_ppt)\n- [.get_measured_frequency()](IterablePin::get_measured_frequency)\n\n"]
#[derive(Debug)]
pub struct OpPinGetDump<'r> {
    request: Request<'r>,
}
impl<'r> OpPinGetDump<'r> {
    pub fn new(mut request: Request<'r>) -> Self {
        Self::write_header(request.buf_mut());
        Self {
            request: request.set_dump(),
        }
    }
    pub fn encode_request<'buf>(buf: &'buf mut Vec<u8>) -> PushPin<&'buf mut Vec<u8>> {
        Self::write_header(buf);
        PushPin::new(buf)
    }
    pub fn encode(&mut self) -> PushPin<&mut Vec<u8>> {
        PushPin::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushPin<RequestBuf<'r>> {
        PushPin::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> IterablePin<'a> {
        let (_header, attrs) = buf.split_at(buf.len().min(BuiltinNfgenmsg::len()));
        IterablePin::with_loc(attrs, buf.as_ptr() as usize)
    }
    fn write_header<Prev: Pusher>(prev: &mut Prev) {
        let mut header = BuiltinNfgenmsg::new();
        header.cmd = 8u8;
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
impl NetlinkRequest for OpPinGetDump<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Generic("dpll".as_bytes())
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = IterablePin<'buf>;
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
#[doc = "Get list of pins and its attributes.\n\n- dump request without any attributes given - list all the pins in the\n  system\n- dump request with target dpll - list all the pins registered with a\n  given dpll device\n- do request with target dpll and target pin - single pin attributes\n\nFlags: admin-perm\n\nRequest attributes:\n- [.push_id()](PushPin::push_id)\n\nReply attributes:\n- [.get_id()](IterablePin::get_id)\n- [.get_module_name()](IterablePin::get_module_name)\n- [.get_clock_id()](IterablePin::get_clock_id)\n- [.get_board_label()](IterablePin::get_board_label)\n- [.get_panel_label()](IterablePin::get_panel_label)\n- [.get_package_label()](IterablePin::get_package_label)\n- [.get_type()](IterablePin::get_type)\n- [.get_frequency()](IterablePin::get_frequency)\n- [.get_frequency_supported()](IterablePin::get_frequency_supported)\n- [.get_capabilities()](IterablePin::get_capabilities)\n- [.get_parent_device()](IterablePin::get_parent_device)\n- [.get_parent_pin()](IterablePin::get_parent_pin)\n- [.get_phase_adjust_min()](IterablePin::get_phase_adjust_min)\n- [.get_phase_adjust_max()](IterablePin::get_phase_adjust_max)\n- [.get_phase_adjust()](IterablePin::get_phase_adjust)\n- [.get_fractional_frequency_offset()](IterablePin::get_fractional_frequency_offset)\n- [.get_esync_frequency()](IterablePin::get_esync_frequency)\n- [.get_esync_frequency_supported()](IterablePin::get_esync_frequency_supported)\n- [.get_esync_pulse()](IterablePin::get_esync_pulse)\n- [.get_reference_sync()](IterablePin::get_reference_sync)\n- [.get_phase_adjust_gran()](IterablePin::get_phase_adjust_gran)\n- [.get_fractional_frequency_offset_ppt()](IterablePin::get_fractional_frequency_offset_ppt)\n- [.get_measured_frequency()](IterablePin::get_measured_frequency)\n\n"]
#[derive(Debug)]
pub struct OpPinGetDo<'r> {
    request: Request<'r>,
}
impl<'r> OpPinGetDo<'r> {
    pub fn new(mut request: Request<'r>) -> Self {
        Self::write_header(request.buf_mut());
        Self { request: request }
    }
    pub fn encode_request<'buf>(buf: &'buf mut Vec<u8>) -> PushPin<&'buf mut Vec<u8>> {
        Self::write_header(buf);
        PushPin::new(buf)
    }
    pub fn encode(&mut self) -> PushPin<&mut Vec<u8>> {
        PushPin::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushPin<RequestBuf<'r>> {
        PushPin::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> IterablePin<'a> {
        let (_header, attrs) = buf.split_at(buf.len().min(BuiltinNfgenmsg::len()));
        IterablePin::with_loc(attrs, buf.as_ptr() as usize)
    }
    fn write_header<Prev: Pusher>(prev: &mut Prev) {
        let mut header = BuiltinNfgenmsg::new();
        header.cmd = 8u8;
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
impl NetlinkRequest for OpPinGetDo<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Generic("dpll".as_bytes())
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = IterablePin<'buf>;
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
#[doc = "Set attributes of a target pin\n\nFlags: admin-perm\n\nRequest attributes:\n- [.push_id()](PushPin::push_id)\n- [.push_direction()](PushPin::push_direction)\n- [.push_frequency()](PushPin::push_frequency)\n- [.push_prio()](PushPin::push_prio)\n- [.push_state()](PushPin::push_state)\n- [.nested_parent_device()](PushPin::nested_parent_device)\n- [.nested_parent_pin()](PushPin::nested_parent_pin)\n- [.push_phase_adjust()](PushPin::push_phase_adjust)\n- [.push_esync_frequency()](PushPin::push_esync_frequency)\n- [.nested_reference_sync()](PushPin::nested_reference_sync)\n\n"]
#[derive(Debug)]
pub struct OpPinSetDo<'r> {
    request: Request<'r>,
}
impl<'r> OpPinSetDo<'r> {
    pub fn new(mut request: Request<'r>) -> Self {
        Self::write_header(request.buf_mut());
        Self { request: request }
    }
    pub fn encode_request<'buf>(buf: &'buf mut Vec<u8>) -> PushPin<&'buf mut Vec<u8>> {
        Self::write_header(buf);
        PushPin::new(buf)
    }
    pub fn encode(&mut self) -> PushPin<&mut Vec<u8>> {
        PushPin::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushPin<RequestBuf<'r>> {
        PushPin::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> IterablePin<'a> {
        let (_header, attrs) = buf.split_at(buf.len().min(BuiltinNfgenmsg::len()));
        IterablePin::with_loc(attrs, buf.as_ptr() as usize)
    }
    fn write_header<Prev: Pusher>(prev: &mut Prev) {
        let mut header = BuiltinNfgenmsg::new();
        header.cmd = 9u8;
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
impl NetlinkRequest for OpPinSetDo<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Generic("dpll".as_bytes())
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = IterablePin<'buf>;
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
    #[doc = "Get id of dpll device that matches given attributes\n\nFlags: admin-perm\n\nRequest attributes:\n- [.push_module_name()](PushDpll::push_module_name)\n- [.push_clock_id()](PushDpll::push_clock_id)\n- [.push_type()](PushDpll::push_type)\n\nReply attributes:\n- [.get_id()](IterableDpll::get_id)\n\n"]
    pub fn op_device_id_get_do(self) -> OpDeviceIdGetDo<'buf> {
        let mut res = OpDeviceIdGetDo::new(self);
        res.request.do_writeback(
            res.protocol(),
            "op-device-id-get-do",
            OpDeviceIdGetDo::lookup,
        );
        res
    }
    #[doc = "Get list of DPLL devices (dump) or attributes of a single dpll device\n\nFlags: admin-perm\n\nReply attributes:\n- [.get_id()](IterableDpll::get_id)\n- [.get_module_name()](IterableDpll::get_module_name)\n- [.get_clock_id()](IterableDpll::get_clock_id)\n- [.get_mode()](IterableDpll::get_mode)\n- [.get_mode_supported()](IterableDpll::get_mode_supported)\n- [.get_lock_status()](IterableDpll::get_lock_status)\n- [.get_temp()](IterableDpll::get_temp)\n- [.get_type()](IterableDpll::get_type)\n- [.get_lock_status_error()](IterableDpll::get_lock_status_error)\n- [.get_phase_offset_monitor()](IterableDpll::get_phase_offset_monitor)\n- [.get_phase_offset_avg_factor()](IterableDpll::get_phase_offset_avg_factor)\n- [.get_frequency_monitor()](IterableDpll::get_frequency_monitor)\n\n"]
    pub fn op_device_get_dump(self) -> OpDeviceGetDump<'buf> {
        let mut res = OpDeviceGetDump::new(self);
        res.request.do_writeback(
            res.protocol(),
            "op-device-get-dump",
            OpDeviceGetDump::lookup,
        );
        res
    }
    #[doc = "Get list of DPLL devices (dump) or attributes of a single dpll device\n\nFlags: admin-perm\n\nRequest attributes:\n- [.push_id()](PushDpll::push_id)\n\nReply attributes:\n- [.get_id()](IterableDpll::get_id)\n- [.get_module_name()](IterableDpll::get_module_name)\n- [.get_clock_id()](IterableDpll::get_clock_id)\n- [.get_mode()](IterableDpll::get_mode)\n- [.get_mode_supported()](IterableDpll::get_mode_supported)\n- [.get_lock_status()](IterableDpll::get_lock_status)\n- [.get_temp()](IterableDpll::get_temp)\n- [.get_type()](IterableDpll::get_type)\n- [.get_lock_status_error()](IterableDpll::get_lock_status_error)\n- [.get_phase_offset_monitor()](IterableDpll::get_phase_offset_monitor)\n- [.get_phase_offset_avg_factor()](IterableDpll::get_phase_offset_avg_factor)\n- [.get_frequency_monitor()](IterableDpll::get_frequency_monitor)\n\n"]
    pub fn op_device_get_do(self) -> OpDeviceGetDo<'buf> {
        let mut res = OpDeviceGetDo::new(self);
        res.request
            .do_writeback(res.protocol(), "op-device-get-do", OpDeviceGetDo::lookup);
        res
    }
    #[doc = "Set attributes for a DPLL device\n\nFlags: admin-perm\n\nRequest attributes:\n- [.push_id()](PushDpll::push_id)\n- [.push_mode()](PushDpll::push_mode)\n- [.push_phase_offset_monitor()](PushDpll::push_phase_offset_monitor)\n- [.push_phase_offset_avg_factor()](PushDpll::push_phase_offset_avg_factor)\n- [.push_frequency_monitor()](PushDpll::push_frequency_monitor)\n\n"]
    pub fn op_device_set_do(self) -> OpDeviceSetDo<'buf> {
        let mut res = OpDeviceSetDo::new(self);
        res.request
            .do_writeback(res.protocol(), "op-device-set-do", OpDeviceSetDo::lookup);
        res
    }
    #[doc = "Get id of a pin that matches given attributes\n\nFlags: admin-perm\n\nRequest attributes:\n- [.push_module_name()](PushPin::push_module_name)\n- [.push_clock_id()](PushPin::push_clock_id)\n- [.push_board_label()](PushPin::push_board_label)\n- [.push_panel_label()](PushPin::push_panel_label)\n- [.push_package_label()](PushPin::push_package_label)\n- [.push_type()](PushPin::push_type)\n\nReply attributes:\n- [.get_id()](IterablePin::get_id)\n\n"]
    pub fn op_pin_id_get_do(self) -> OpPinIdGetDo<'buf> {
        let mut res = OpPinIdGetDo::new(self);
        res.request
            .do_writeback(res.protocol(), "op-pin-id-get-do", OpPinIdGetDo::lookup);
        res
    }
    #[doc = "Get list of pins and its attributes.\n\n- dump request without any attributes given - list all the pins in the\n  system\n- dump request with target dpll - list all the pins registered with a\n  given dpll device\n- do request with target dpll and target pin - single pin attributes\n\nFlags: admin-perm\n\nRequest attributes:\n- [.push_id()](PushPin::push_id)\n\nReply attributes:\n- [.get_id()](IterablePin::get_id)\n- [.get_module_name()](IterablePin::get_module_name)\n- [.get_clock_id()](IterablePin::get_clock_id)\n- [.get_board_label()](IterablePin::get_board_label)\n- [.get_panel_label()](IterablePin::get_panel_label)\n- [.get_package_label()](IterablePin::get_package_label)\n- [.get_type()](IterablePin::get_type)\n- [.get_frequency()](IterablePin::get_frequency)\n- [.get_frequency_supported()](IterablePin::get_frequency_supported)\n- [.get_capabilities()](IterablePin::get_capabilities)\n- [.get_parent_device()](IterablePin::get_parent_device)\n- [.get_parent_pin()](IterablePin::get_parent_pin)\n- [.get_phase_adjust_min()](IterablePin::get_phase_adjust_min)\n- [.get_phase_adjust_max()](IterablePin::get_phase_adjust_max)\n- [.get_phase_adjust()](IterablePin::get_phase_adjust)\n- [.get_fractional_frequency_offset()](IterablePin::get_fractional_frequency_offset)\n- [.get_esync_frequency()](IterablePin::get_esync_frequency)\n- [.get_esync_frequency_supported()](IterablePin::get_esync_frequency_supported)\n- [.get_esync_pulse()](IterablePin::get_esync_pulse)\n- [.get_reference_sync()](IterablePin::get_reference_sync)\n- [.get_phase_adjust_gran()](IterablePin::get_phase_adjust_gran)\n- [.get_fractional_frequency_offset_ppt()](IterablePin::get_fractional_frequency_offset_ppt)\n- [.get_measured_frequency()](IterablePin::get_measured_frequency)\n\n"]
    pub fn op_pin_get_dump(self) -> OpPinGetDump<'buf> {
        let mut res = OpPinGetDump::new(self);
        res.request
            .do_writeback(res.protocol(), "op-pin-get-dump", OpPinGetDump::lookup);
        res
    }
    #[doc = "Get list of pins and its attributes.\n\n- dump request without any attributes given - list all the pins in the\n  system\n- dump request with target dpll - list all the pins registered with a\n  given dpll device\n- do request with target dpll and target pin - single pin attributes\n\nFlags: admin-perm\n\nRequest attributes:\n- [.push_id()](PushPin::push_id)\n\nReply attributes:\n- [.get_id()](IterablePin::get_id)\n- [.get_module_name()](IterablePin::get_module_name)\n- [.get_clock_id()](IterablePin::get_clock_id)\n- [.get_board_label()](IterablePin::get_board_label)\n- [.get_panel_label()](IterablePin::get_panel_label)\n- [.get_package_label()](IterablePin::get_package_label)\n- [.get_type()](IterablePin::get_type)\n- [.get_frequency()](IterablePin::get_frequency)\n- [.get_frequency_supported()](IterablePin::get_frequency_supported)\n- [.get_capabilities()](IterablePin::get_capabilities)\n- [.get_parent_device()](IterablePin::get_parent_device)\n- [.get_parent_pin()](IterablePin::get_parent_pin)\n- [.get_phase_adjust_min()](IterablePin::get_phase_adjust_min)\n- [.get_phase_adjust_max()](IterablePin::get_phase_adjust_max)\n- [.get_phase_adjust()](IterablePin::get_phase_adjust)\n- [.get_fractional_frequency_offset()](IterablePin::get_fractional_frequency_offset)\n- [.get_esync_frequency()](IterablePin::get_esync_frequency)\n- [.get_esync_frequency_supported()](IterablePin::get_esync_frequency_supported)\n- [.get_esync_pulse()](IterablePin::get_esync_pulse)\n- [.get_reference_sync()](IterablePin::get_reference_sync)\n- [.get_phase_adjust_gran()](IterablePin::get_phase_adjust_gran)\n- [.get_fractional_frequency_offset_ppt()](IterablePin::get_fractional_frequency_offset_ppt)\n- [.get_measured_frequency()](IterablePin::get_measured_frequency)\n\n"]
    pub fn op_pin_get_do(self) -> OpPinGetDo<'buf> {
        let mut res = OpPinGetDo::new(self);
        res.request
            .do_writeback(res.protocol(), "op-pin-get-do", OpPinGetDo::lookup);
        res
    }
    #[doc = "Set attributes of a target pin\n\nFlags: admin-perm\n\nRequest attributes:\n- [.push_id()](PushPin::push_id)\n- [.push_direction()](PushPin::push_direction)\n- [.push_frequency()](PushPin::push_frequency)\n- [.push_prio()](PushPin::push_prio)\n- [.push_state()](PushPin::push_state)\n- [.nested_parent_device()](PushPin::nested_parent_device)\n- [.nested_parent_pin()](PushPin::nested_parent_pin)\n- [.push_phase_adjust()](PushPin::push_phase_adjust)\n- [.push_esync_frequency()](PushPin::push_esync_frequency)\n- [.nested_reference_sync()](PushPin::nested_reference_sync)\n\n"]
    pub fn op_pin_set_do(self) -> OpPinSetDo<'buf> {
        let mut res = OpPinSetDo::new(self);
        res.request
            .do_writeback(res.protocol(), "op-pin-set-do", OpPinSetDo::lookup);
        res
    }
}
#[cfg(test)]
mod generated_tests {
    use super::*;
    #[test]
    fn tests() {
        let _ = IterableDpll::get_clock_id;
        let _ = IterableDpll::get_frequency_monitor;
        let _ = IterableDpll::get_id;
        let _ = IterableDpll::get_lock_status;
        let _ = IterableDpll::get_lock_status_error;
        let _ = IterableDpll::get_mode;
        let _ = IterableDpll::get_mode_supported;
        let _ = IterableDpll::get_module_name;
        let _ = IterableDpll::get_phase_offset_avg_factor;
        let _ = IterableDpll::get_phase_offset_monitor;
        let _ = IterableDpll::get_temp;
        let _ = IterableDpll::get_type;
        let _ = IterablePin::get_board_label;
        let _ = IterablePin::get_capabilities;
        let _ = IterablePin::get_clock_id;
        let _ = IterablePin::get_esync_frequency;
        let _ = IterablePin::get_esync_frequency_supported;
        let _ = IterablePin::get_esync_pulse;
        let _ = IterablePin::get_fractional_frequency_offset;
        let _ = IterablePin::get_fractional_frequency_offset_ppt;
        let _ = IterablePin::get_frequency;
        let _ = IterablePin::get_frequency_supported;
        let _ = IterablePin::get_id;
        let _ = IterablePin::get_measured_frequency;
        let _ = IterablePin::get_module_name;
        let _ = IterablePin::get_package_label;
        let _ = IterablePin::get_panel_label;
        let _ = IterablePin::get_parent_device;
        let _ = IterablePin::get_parent_pin;
        let _ = IterablePin::get_phase_adjust;
        let _ = IterablePin::get_phase_adjust_gran;
        let _ = IterablePin::get_phase_adjust_max;
        let _ = IterablePin::get_phase_adjust_min;
        let _ = IterablePin::get_reference_sync;
        let _ = IterablePin::get_type;
        let _ = OpDeviceChangeNotif;
        let _ = OpDeviceCreateNotif;
        let _ = OpDeviceDeleteNotif;
        let _ = OpPinChangeNotif;
        let _ = OpPinCreateNotif;
        let _ = OpPinDeleteNotif;
        let _ = PushDpll::<&mut Vec<u8>>::push_clock_id;
        let _ = PushDpll::<&mut Vec<u8>>::push_frequency_monitor;
        let _ = PushDpll::<&mut Vec<u8>>::push_id;
        let _ = PushDpll::<&mut Vec<u8>>::push_mode;
        let _ = PushDpll::<&mut Vec<u8>>::push_module_name;
        let _ = PushDpll::<&mut Vec<u8>>::push_phase_offset_avg_factor;
        let _ = PushDpll::<&mut Vec<u8>>::push_phase_offset_monitor;
        let _ = PushDpll::<&mut Vec<u8>>::push_type;
        let _ = PushPin::<&mut Vec<u8>>::nested_parent_device;
        let _ = PushPin::<&mut Vec<u8>>::nested_parent_pin;
        let _ = PushPin::<&mut Vec<u8>>::nested_reference_sync;
        let _ = PushPin::<&mut Vec<u8>>::push_board_label;
        let _ = PushPin::<&mut Vec<u8>>::push_clock_id;
        let _ = PushPin::<&mut Vec<u8>>::push_direction;
        let _ = PushPin::<&mut Vec<u8>>::push_esync_frequency;
        let _ = PushPin::<&mut Vec<u8>>::push_frequency;
        let _ = PushPin::<&mut Vec<u8>>::push_id;
        let _ = PushPin::<&mut Vec<u8>>::push_module_name;
        let _ = PushPin::<&mut Vec<u8>>::push_package_label;
        let _ = PushPin::<&mut Vec<u8>>::push_panel_label;
        let _ = PushPin::<&mut Vec<u8>>::push_phase_adjust;
        let _ = PushPin::<&mut Vec<u8>>::push_prio;
        let _ = PushPin::<&mut Vec<u8>>::push_state;
        let _ = PushPin::<&mut Vec<u8>>::push_type;
    }
}
