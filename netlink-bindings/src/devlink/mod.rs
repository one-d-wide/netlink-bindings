#![doc = "Partial family for Devlink.\n"]
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
pub const PROTONAME: &str = "devlink";
pub const PROTONAME_CSTR: &CStr = c"devlink";
#[doc = "Enum - defines an integer enumeration, with values for each entry incrementing by 1, (e.g. 0, 1, 2, 3)"]
#[derive(Debug, Clone, Copy)]
pub enum SbPoolType {
    Ingress = 0,
    Egress = 1,
}
impl SbPoolType {
    pub fn from_value(value: u64) -> Option<Self> {
        Some(match value {
            0 => Self::Ingress,
            1 => Self::Egress,
            _ => return None,
        })
    }
}
#[doc = "Enum - defines an integer enumeration, with values for each entry incrementing by 1, (e.g. 0, 1, 2, 3)"]
#[derive(Debug, Clone, Copy)]
pub enum PortType {
    Notset = 0,
    Auto = 1,
    Eth = 2,
    Ib = 3,
}
impl PortType {
    pub fn from_value(value: u64) -> Option<Self> {
        Some(match value {
            0 => Self::Notset,
            1 => Self::Auto,
            2 => Self::Eth,
            3 => Self::Ib,
            _ => return None,
        })
    }
}
#[doc = "Enum - defines an integer enumeration, with values for each entry incrementing by 1, (e.g. 0, 1, 2, 3)"]
#[derive(Debug, Clone, Copy)]
pub enum PortFlavour {
    Physical = 0,
    Cpu = 1,
    Dsa = 2,
    PciPf = 3,
    PciVf = 4,
    Virtual = 5,
    Unused = 6,
    PciSf = 7,
}
impl PortFlavour {
    pub fn from_value(value: u64) -> Option<Self> {
        Some(match value {
            0 => Self::Physical,
            1 => Self::Cpu,
            2 => Self::Dsa,
            3 => Self::PciPf,
            4 => Self::PciVf,
            5 => Self::Virtual,
            6 => Self::Unused,
            7 => Self::PciSf,
            _ => return None,
        })
    }
}
#[doc = "Enum - defines an integer enumeration, with values for each entry incrementing by 1, (e.g. 0, 1, 2, 3)"]
#[derive(Debug, Clone, Copy)]
pub enum PortFnState {
    Inactive = 0,
    Active = 1,
}
impl PortFnState {
    pub fn from_value(value: u64) -> Option<Self> {
        Some(match value {
            0 => Self::Inactive,
            1 => Self::Active,
            _ => return None,
        })
    }
}
#[doc = "Enum - defines an integer enumeration, with values for each entry incrementing by 1, (e.g. 0, 1, 2, 3)"]
#[derive(Debug, Clone, Copy)]
pub enum PortFnOpstate {
    Detached = 0,
    Attached = 1,
}
impl PortFnOpstate {
    pub fn from_value(value: u64) -> Option<Self> {
        Some(match value {
            0 => Self::Detached,
            1 => Self::Attached,
            _ => return None,
        })
    }
}
#[doc = "Enum - defines an integer enumeration, with values for each entry incrementing by 1, (e.g. 0, 1, 2, 3)"]
#[derive(Debug, Clone, Copy)]
pub enum PortFnAttrCap {
    RoceBit = 0,
    MigratableBit = 1,
    IpsecCryptoBit = 2,
    IpsecPacketBit = 3,
}
impl PortFnAttrCap {
    pub fn from_value(value: u64) -> Option<Self> {
        Some(match value {
            0 => Self::RoceBit,
            1 => Self::MigratableBit,
            2 => Self::IpsecCryptoBit,
            3 => Self::IpsecPacketBit,
            _ => return None,
        })
    }
}
#[doc = "Enum - defines an integer enumeration, with values for each entry incrementing by 1, (e.g. 0, 1, 2, 3)"]
#[derive(Debug, Clone, Copy)]
pub enum RateType {
    Leaf = 0,
    Node = 1,
}
impl RateType {
    pub fn from_value(value: u64) -> Option<Self> {
        Some(match value {
            0 => Self::Leaf,
            1 => Self::Node,
            _ => return None,
        })
    }
}
#[doc = "Enum - defines an integer enumeration, with values for each entry incrementing by 1, (e.g. 0, 1, 2, 3)"]
#[derive(Debug, Clone, Copy)]
pub enum SbThresholdType {
    Static = 0,
    Dynamic = 1,
}
impl SbThresholdType {
    pub fn from_value(value: u64) -> Option<Self> {
        Some(match value {
            0 => Self::Static,
            1 => Self::Dynamic,
            _ => return None,
        })
    }
}
#[doc = "Enum - defines an integer enumeration, with values for each entry incrementing by 1, (e.g. 0, 1, 2, 3)"]
#[derive(Debug, Clone, Copy)]
pub enum EswitchMode {
    Legacy = 0,
    Switchdev = 1,
    SwitchdevInactive = 2,
}
impl EswitchMode {
    pub fn from_value(value: u64) -> Option<Self> {
        Some(match value {
            0 => Self::Legacy,
            1 => Self::Switchdev,
            2 => Self::SwitchdevInactive,
            _ => return None,
        })
    }
}
#[doc = "Enum - defines an integer enumeration, with values for each entry incrementing by 1, (e.g. 0, 1, 2, 3)"]
#[derive(Debug, Clone, Copy)]
pub enum EswitchInlineMode {
    None = 0,
    Link = 1,
    Network = 2,
    Transport = 3,
}
impl EswitchInlineMode {
    pub fn from_value(value: u64) -> Option<Self> {
        Some(match value {
            0 => Self::None,
            1 => Self::Link,
            2 => Self::Network,
            3 => Self::Transport,
            _ => return None,
        })
    }
}
#[doc = "Enum - defines an integer enumeration, with values for each entry incrementing by 1, (e.g. 0, 1, 2, 3)"]
#[derive(Debug, Clone, Copy)]
pub enum EswitchEncapMode {
    None = 0,
    Basic = 1,
}
impl EswitchEncapMode {
    pub fn from_value(value: u64) -> Option<Self> {
        Some(match value {
            0 => Self::None,
            1 => Self::Basic,
            _ => return None,
        })
    }
}
#[doc = "Enum - defines an integer enumeration, with values for each entry incrementing by 1, (e.g. 0, 1, 2, 3)"]
#[derive(Debug, Clone, Copy)]
pub enum DpipeHeaderId {
    Ethernet = 0,
    Ipv4 = 1,
    Ipv6 = 2,
}
impl DpipeHeaderId {
    pub fn from_value(value: u64) -> Option<Self> {
        Some(match value {
            0 => Self::Ethernet,
            1 => Self::Ipv4,
            2 => Self::Ipv6,
            _ => return None,
        })
    }
}
#[doc = "Enum - defines an integer enumeration, with values for each entry incrementing by 1, (e.g. 0, 1, 2, 3)"]
#[derive(Debug, Clone, Copy)]
pub enum DpipeMatchType {
    FieldExact = 0,
}
impl DpipeMatchType {
    pub fn from_value(value: u64) -> Option<Self> {
        Some(match value {
            0 => Self::FieldExact,
            _ => return None,
        })
    }
}
#[doc = "Enum - defines an integer enumeration, with values for each entry incrementing by 1, (e.g. 0, 1, 2, 3)"]
#[derive(Debug, Clone, Copy)]
pub enum DpipeActionType {
    FieldModify = 0,
}
impl DpipeActionType {
    pub fn from_value(value: u64) -> Option<Self> {
        Some(match value {
            0 => Self::FieldModify,
            _ => return None,
        })
    }
}
#[doc = "Enum - defines an integer enumeration, with values for each entry incrementing by 1, (e.g. 0, 1, 2, 3)"]
#[derive(Debug, Clone, Copy)]
pub enum DpipeFieldMappingType {
    None = 0,
    Ifindex = 1,
}
impl DpipeFieldMappingType {
    pub fn from_value(value: u64) -> Option<Self> {
        Some(match value {
            0 => Self::None,
            1 => Self::Ifindex,
            _ => return None,
        })
    }
}
#[doc = "Enum - defines an integer enumeration, with values for each entry incrementing by 1, (e.g. 0, 1, 2, 3)"]
#[derive(Debug, Clone, Copy)]
pub enum ResourceUnit {
    Entry = 0,
}
impl ResourceUnit {
    pub fn from_value(value: u64) -> Option<Self> {
        Some(match value {
            0 => Self::Entry,
            _ => return None,
        })
    }
}
#[doc = "Enum - defines an integer enumeration, with values for each entry incrementing by 1, (e.g. 0, 1, 2, 3)"]
#[derive(Debug, Clone, Copy)]
pub enum ResourceScope {
    Dev = 0,
    Port = 1,
}
impl ResourceScope {
    pub fn from_value(value: u64) -> Option<Self> {
        Some(match value {
            0 => Self::Dev,
            1 => Self::Port,
            _ => return None,
        })
    }
}
#[doc = "Enum - defines an integer enumeration, with values for each entry incrementing by 1, (e.g. 0, 1, 2, 3)"]
#[derive(Debug, Clone, Copy)]
pub enum ReloadAction {
    DriverReinit = 1,
    FwActivate = 2,
}
impl ReloadAction {
    pub fn from_value(value: u64) -> Option<Self> {
        Some(match value {
            1 => Self::DriverReinit,
            2 => Self::FwActivate,
            _ => return None,
        })
    }
}
#[doc = "Enum - defines an integer enumeration, with values for each entry incrementing by 1, (e.g. 0, 1, 2, 3)"]
#[derive(Debug, Clone, Copy)]
pub enum ParamCmode {
    Runtime = 0,
    Driverinit = 1,
    Permanent = 2,
}
impl ParamCmode {
    pub fn from_value(value: u64) -> Option<Self> {
        Some(match value {
            0 => Self::Runtime,
            1 => Self::Driverinit,
            2 => Self::Permanent,
            _ => return None,
        })
    }
}
#[doc = "Enum - defines an integer enumeration, with values for each entry incrementing by 1, (e.g. 0, 1, 2, 3)"]
#[derive(Debug, Clone, Copy)]
pub enum FlashOverwrite {
    SettingsBit = 0,
    IdentifiersBit = 1,
}
impl FlashOverwrite {
    pub fn from_value(value: u64) -> Option<Self> {
        Some(match value {
            0 => Self::SettingsBit,
            1 => Self::IdentifiersBit,
            _ => return None,
        })
    }
}
#[doc = "Enum - defines an integer enumeration, with values for each entry incrementing by 1, (e.g. 0, 1, 2, 3)"]
#[derive(Debug, Clone, Copy)]
pub enum TrapAction {
    Drop = 0,
    Trap = 1,
    Mirror = 2,
}
impl TrapAction {
    pub fn from_value(value: u64) -> Option<Self> {
        Some(match value {
            0 => Self::Drop,
            1 => Self::Trap,
            2 => Self::Mirror,
            _ => return None,
        })
    }
}
#[doc = "Enum - defines an integer enumeration, with values for each entry incrementing by 1, (e.g. 0, 1, 2, 3)"]
#[derive(Debug, Clone, Copy)]
pub enum TrapType {
    Drop = 0,
    Exception = 1,
    Control = 2,
}
impl TrapType {
    pub fn from_value(value: u64) -> Option<Self> {
        Some(match value {
            0 => Self::Drop,
            1 => Self::Exception,
            2 => Self::Control,
            _ => return None,
        })
    }
}
#[doc = "Enum - defines an integer enumeration, with values for each entry incrementing by 1, (e.g. 0, 1, 2, 3)"]
#[derive(Debug, Clone, Copy)]
pub enum VarAttrType {
    U8 = 1,
    U16 = 2,
    U32 = 3,
    U64 = 4,
    String = 5,
    Flag = 6,
    NulString = 10,
    Binary = 11,
    U64Array = 129,
}
impl VarAttrType {
    pub fn from_value(value: u64) -> Option<Self> {
        Some(match value {
            1 => Self::U8,
            2 => Self::U16,
            3 => Self::U32,
            4 => Self::U64,
            5 => Self::String,
            6 => Self::Flag,
            10 => Self::NulString,
            11 => Self::Binary,
            129 => Self::U64Array,
            _ => return None,
        })
    }
}
pub const RATE_TC_INDEX_MAX: u64 = 7u64;
#[derive(Clone)]
pub enum Devlink<'a> {
    BusName(&'a CStr),
    DevName(&'a CStr),
    PortIndex(u32),
    #[doc = "Associated type: [`PortType`] (enum)"]
    PortType(u16),
    PortDesiredType(u16),
    PortNetdevIfindex(u32),
    PortNetdevName(&'a CStr),
    PortIbdevName(&'a CStr),
    PortSplitCount(u32),
    PortSplitGroup(u32),
    SbIndex(u32),
    SbSize(u32),
    SbIngressPoolCount(u16),
    SbEgressPoolCount(u16),
    SbIngressTcCount(u16),
    SbEgressTcCount(u16),
    SbPoolIndex(u16),
    #[doc = "Associated type: [`SbPoolType`] (enum)"]
    SbPoolType(u8),
    SbPoolSize(u32),
    #[doc = "Associated type: [`SbThresholdType`] (enum)"]
    SbPoolThresholdType(u8),
    SbThreshold(u32),
    SbTcIndex(u16),
    SbOccCur(u32),
    SbOccMax(u32),
    #[doc = "Associated type: [`EswitchMode`] (enum)"]
    EswitchMode(u16),
    #[doc = "Associated type: [`EswitchInlineMode`] (enum)"]
    EswitchInlineMode(u8),
    DpipeTables(IterableDlDpipeTables<'a>),
    #[doc = "Attribute may repeat multiple times (treat it as array)"]
    DpipeTable(IterableDlDpipeTable<'a>),
    DpipeTableName(&'a CStr),
    DpipeTableSize(u64),
    DpipeTableMatches(IterableDlDpipeTableMatches<'a>),
    DpipeTableActions(IterableDlDpipeTableActions<'a>),
    DpipeTableCountersEnabled(u8),
    DpipeEntries(IterableDlDpipeEntries<'a>),
    #[doc = "Attribute may repeat multiple times (treat it as array)"]
    DpipeEntry(IterableDlDpipeEntry<'a>),
    DpipeEntryIndex(u64),
    DpipeEntryMatchValues(IterableDlDpipeEntryMatchValues<'a>),
    DpipeEntryActionValues(IterableDlDpipeEntryActionValues<'a>),
    DpipeEntryCounter(u64),
    #[doc = "Attribute may repeat multiple times (treat it as array)"]
    DpipeMatch(IterableDlDpipeMatch<'a>),
    #[doc = "Attribute may repeat multiple times (treat it as array)"]
    DpipeMatchValue(IterableDlDpipeMatchValue<'a>),
    #[doc = "Associated type: [`DpipeMatchType`] (enum)"]
    DpipeMatchType(u32),
    #[doc = "Attribute may repeat multiple times (treat it as array)"]
    DpipeAction(IterableDlDpipeAction<'a>),
    #[doc = "Attribute may repeat multiple times (treat it as array)"]
    DpipeActionValue(IterableDlDpipeActionValue<'a>),
    #[doc = "Associated type: [`DpipeActionType`] (enum)"]
    DpipeActionType(u32),
    DpipeValue(&'a [u8]),
    DpipeValueMask(&'a [u8]),
    DpipeValueMapping(u32),
    DpipeHeaders(IterableDlDpipeHeaders<'a>),
    #[doc = "Attribute may repeat multiple times (treat it as array)"]
    DpipeHeader(IterableDlDpipeHeader<'a>),
    DpipeHeaderName(&'a CStr),
    #[doc = "Associated type: [`DpipeHeaderId`] (enum)"]
    DpipeHeaderId(u32),
    DpipeHeaderFields(IterableDlDpipeHeaderFields<'a>),
    DpipeHeaderGlobal(u8),
    DpipeHeaderIndex(u32),
    #[doc = "Attribute may repeat multiple times (treat it as array)"]
    DpipeField(IterableDlDpipeField<'a>),
    DpipeFieldName(&'a CStr),
    DpipeFieldId(u32),
    DpipeFieldBitwidth(u32),
    #[doc = "Associated type: [`DpipeFieldMappingType`] (enum)"]
    DpipeFieldMappingType(u32),
    Pad(&'a [u8]),
    #[doc = "Associated type: [`EswitchEncapMode`] (enum)"]
    EswitchEncapMode(u8),
    ResourceList(IterableDlResourceList<'a>),
    #[doc = "Attribute may repeat multiple times (treat it as array)"]
    Resource(IterableDlResource<'a>),
    ResourceName(&'a CStr),
    ResourceId(u64),
    ResourceSize(u64),
    ResourceSizeNew(u64),
    ResourceSizeValid(u8),
    ResourceSizeMin(u64),
    ResourceSizeMax(u64),
    ResourceSizeGran(u64),
    #[doc = "Associated type: [`ResourceUnit`] (enum)"]
    ResourceUnit(u8),
    ResourceOcc(u64),
    DpipeTableResourceId(u64),
    DpipeTableResourceUnits(u64),
    #[doc = "Associated type: [`PortFlavour`] (enum)"]
    PortFlavour(u16),
    PortNumber(u32),
    PortSplitSubportNumber(u32),
    Param(IterableDlParam<'a>),
    ParamName(&'a CStr),
    ParamGeneric(()),
    #[doc = "Associated type: [`VarAttrType`] (enum)"]
    ParamType(u8),
    #[doc = "Associated type: [`ParamCmode`] (enum)"]
    ParamValueCmode(u8),
    RegionName(&'a CStr),
    RegionSize(u64),
    RegionSnapshots(IterableDlRegionSnapshots<'a>),
    RegionSnapshot(IterableDlRegionSnapshot<'a>),
    RegionSnapshotId(u32),
    RegionChunks(IterableDlRegionChunks<'a>),
    RegionChunk(IterableDlRegionChunk<'a>),
    RegionChunkData(&'a [u8]),
    RegionChunkAddr(u64),
    RegionChunkLen(u64),
    InfoDriverName(&'a CStr),
    InfoSerialNumber(&'a CStr),
    #[doc = "Attribute may repeat multiple times (treat it as array)"]
    InfoVersionFixed(IterableDlInfoVersion<'a>),
    #[doc = "Attribute may repeat multiple times (treat it as array)"]
    InfoVersionRunning(IterableDlInfoVersion<'a>),
    #[doc = "Attribute may repeat multiple times (treat it as array)"]
    InfoVersionStored(IterableDlInfoVersion<'a>),
    InfoVersionName(&'a CStr),
    InfoVersionValue(&'a CStr),
    SbPoolCellSize(u32),
    Fmsg(IterableDlFmsg<'a>),
    FmsgObjNestStart(()),
    FmsgPairNestStart(()),
    FmsgArrNestStart(()),
    FmsgNestEnd(()),
    FmsgObjName(&'a CStr),
    #[doc = "Associated type: [`VarAttrType`] (enum)"]
    FmsgObjValueType(u8),
    HealthReporter(IterableDlHealthReporter<'a>),
    HealthReporterName(&'a CStr),
    HealthReporterState(u8),
    HealthReporterErrCount(u64),
    HealthReporterRecoverCount(u64),
    HealthReporterDumpTs(u64),
    HealthReporterGracefulPeriod(u64),
    HealthReporterAutoRecover(u8),
    FlashUpdateFileName(&'a CStr),
    FlashUpdateComponent(&'a CStr),
    FlashUpdateStatusMsg(&'a CStr),
    FlashUpdateStatusDone(u64),
    FlashUpdateStatusTotal(u64),
    PortPciPfNumber(u16),
    PortPciVfNumber(u16),
    Stats(IterableDlAttrStats<'a>),
    TrapName(&'a CStr),
    #[doc = "Associated type: [`TrapAction`] (enum)"]
    TrapAction(u8),
    #[doc = "Associated type: [`TrapType`] (enum)"]
    TrapType(u8),
    TrapGeneric(()),
    TrapMetadata(IterableDlTrapMetadata<'a>),
    TrapGroupName(&'a CStr),
    ReloadFailed(u8),
    HealthReporterDumpTsNs(u64),
    NetnsFd(u32),
    NetnsPid(u32),
    NetnsId(u32),
    HealthReporterAutoDump(u8),
    TrapPolicerId(u32),
    TrapPolicerRate(u64),
    TrapPolicerBurst(u64),
    PortFunction(IterableDlPortFunction<'a>),
    InfoBoardSerialNumber(&'a CStr),
    PortLanes(u32),
    PortSplittable(u8),
    PortExternal(u8),
    PortControllerNumber(u32),
    FlashUpdateStatusTimeout(u64),
    #[doc = "Associated type: [`FlashOverwrite`] (1 bit per enumeration)"]
    FlashUpdateOverwriteMask(BuiltinBitfield32),
    #[doc = "Associated type: [`ReloadAction`] (enum)"]
    ReloadAction(u8),
    #[doc = "Associated type: [`ReloadAction`] (1 bit per enumeration)"]
    ReloadActionsPerformed(BuiltinBitfield32),
    #[doc = "Associated type: [`ReloadAction`] (1 bit per enumeration)"]
    ReloadLimits(BuiltinBitfield32),
    DevStats(IterableDlDevStats<'a>),
    ReloadStats(IterableDlReloadStats<'a>),
    #[doc = "Attribute may repeat multiple times (treat it as array)"]
    ReloadStatsEntry(IterableDlReloadStatsEntry<'a>),
    ReloadStatsLimit(u8),
    ReloadStatsValue(u32),
    RemoteReloadStats(IterableDlReloadStats<'a>),
    #[doc = "Attribute may repeat multiple times (treat it as array)"]
    ReloadActionInfo(IterableDlReloadActInfo<'a>),
    #[doc = "Attribute may repeat multiple times (treat it as array)"]
    ReloadActionStats(IterableDlReloadActStats<'a>),
    PortPciSfNumber(u32),
    #[doc = "Associated type: [`RateType`] (enum)"]
    RateType(u16),
    RateTxShare(u64),
    RateTxMax(u64),
    RateNodeName(&'a CStr),
    RateParentNodeName(&'a CStr),
    RegionMaxSnapshots(u32),
    LinecardIndex(u32),
    LinecardState(u8),
    LinecardType(&'a CStr),
    LinecardSupportedTypes(IterableDlLinecardSupportedTypes<'a>),
    Selftests(IterableDlSelftestId<'a>),
    RateTxPriority(u32),
    RateTxWeight(u32),
    RegionDirect(()),
    #[doc = "Attribute may repeat multiple times (treat it as array)"]
    RateTcBws(IterableDlRateTcBws<'a>),
    #[doc = "Time (in msec) for recoveries before starting the grace period.\n"]
    HealthReporterBurstPeriod(u64),
    #[doc = "Request restoring parameter to its default value.\n"]
    ParamResetDefault(()),
    #[doc = "Unique devlink instance index.\n"]
    Index(u32),
    #[doc = "Bitmask selecting which resource classes to include in a resource-dump\nresponse. Bit 0 (dev) selects device-level resources; bit 1 (port)\nselects port-level resources. When absent all classes are returned.\n\nAssociated type: [`ResourceScope`] (1 bit per enumeration)"]
    ResourceScopeMask(u32),
}
impl<'a> IterableDevlink<'a> {
    pub fn get_bus_name(&self) -> Result<&'a CStr, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Devlink::BusName(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Devlink",
            "BusName",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_dev_name(&self) -> Result<&'a CStr, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Devlink::DevName(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Devlink",
            "DevName",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_port_index(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Devlink::PortIndex(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Devlink",
            "PortIndex",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Associated type: [`PortType`] (enum)"]
    pub fn get_port_type(&self) -> Result<u16, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Devlink::PortType(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Devlink",
            "PortType",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_port_desired_type(&self) -> Result<u16, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Devlink::PortDesiredType(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Devlink",
            "PortDesiredType",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_port_netdev_ifindex(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Devlink::PortNetdevIfindex(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Devlink",
            "PortNetdevIfindex",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_port_netdev_name(&self) -> Result<&'a CStr, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Devlink::PortNetdevName(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Devlink",
            "PortNetdevName",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_port_ibdev_name(&self) -> Result<&'a CStr, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Devlink::PortIbdevName(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Devlink",
            "PortIbdevName",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_port_split_count(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Devlink::PortSplitCount(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Devlink",
            "PortSplitCount",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_port_split_group(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Devlink::PortSplitGroup(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Devlink",
            "PortSplitGroup",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_sb_index(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Devlink::SbIndex(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Devlink",
            "SbIndex",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_sb_size(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Devlink::SbSize(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Devlink",
            "SbSize",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_sb_ingress_pool_count(&self) -> Result<u16, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Devlink::SbIngressPoolCount(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Devlink",
            "SbIngressPoolCount",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_sb_egress_pool_count(&self) -> Result<u16, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Devlink::SbEgressPoolCount(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Devlink",
            "SbEgressPoolCount",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_sb_ingress_tc_count(&self) -> Result<u16, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Devlink::SbIngressTcCount(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Devlink",
            "SbIngressTcCount",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_sb_egress_tc_count(&self) -> Result<u16, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Devlink::SbEgressTcCount(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Devlink",
            "SbEgressTcCount",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_sb_pool_index(&self) -> Result<u16, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Devlink::SbPoolIndex(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Devlink",
            "SbPoolIndex",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Associated type: [`SbPoolType`] (enum)"]
    pub fn get_sb_pool_type(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Devlink::SbPoolType(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Devlink",
            "SbPoolType",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_sb_pool_size(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Devlink::SbPoolSize(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Devlink",
            "SbPoolSize",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Associated type: [`SbThresholdType`] (enum)"]
    pub fn get_sb_pool_threshold_type(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Devlink::SbPoolThresholdType(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Devlink",
            "SbPoolThresholdType",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_sb_threshold(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Devlink::SbThreshold(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Devlink",
            "SbThreshold",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_sb_tc_index(&self) -> Result<u16, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Devlink::SbTcIndex(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Devlink",
            "SbTcIndex",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_sb_occ_cur(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Devlink::SbOccCur(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Devlink",
            "SbOccCur",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_sb_occ_max(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Devlink::SbOccMax(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Devlink",
            "SbOccMax",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Associated type: [`EswitchMode`] (enum)"]
    pub fn get_eswitch_mode(&self) -> Result<u16, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Devlink::EswitchMode(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Devlink",
            "EswitchMode",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Associated type: [`EswitchInlineMode`] (enum)"]
    pub fn get_eswitch_inline_mode(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Devlink::EswitchInlineMode(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Devlink",
            "EswitchInlineMode",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_dpipe_tables(&self) -> Result<IterableDlDpipeTables<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Devlink::DpipeTables(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Devlink",
            "DpipeTables",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Attribute may repeat multiple times (treat it as array)"]
    pub fn get_dpipe_table(
        &self,
    ) -> MultiAttrIterable<Self, Devlink<'a>, IterableDlDpipeTable<'a>> {
        MultiAttrIterable::new(self.clone(), |variant| {
            if let Devlink::DpipeTable(val) = variant {
                Some(val)
            } else {
                None
            }
        })
    }
    pub fn get_dpipe_table_name(&self) -> Result<&'a CStr, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Devlink::DpipeTableName(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Devlink",
            "DpipeTableName",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_dpipe_table_size(&self) -> Result<u64, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Devlink::DpipeTableSize(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Devlink",
            "DpipeTableSize",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_dpipe_table_matches(&self) -> Result<IterableDlDpipeTableMatches<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Devlink::DpipeTableMatches(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Devlink",
            "DpipeTableMatches",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_dpipe_table_actions(&self) -> Result<IterableDlDpipeTableActions<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Devlink::DpipeTableActions(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Devlink",
            "DpipeTableActions",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_dpipe_table_counters_enabled(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Devlink::DpipeTableCountersEnabled(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Devlink",
            "DpipeTableCountersEnabled",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_dpipe_entries(&self) -> Result<IterableDlDpipeEntries<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Devlink::DpipeEntries(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Devlink",
            "DpipeEntries",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Attribute may repeat multiple times (treat it as array)"]
    pub fn get_dpipe_entry(
        &self,
    ) -> MultiAttrIterable<Self, Devlink<'a>, IterableDlDpipeEntry<'a>> {
        MultiAttrIterable::new(self.clone(), |variant| {
            if let Devlink::DpipeEntry(val) = variant {
                Some(val)
            } else {
                None
            }
        })
    }
    pub fn get_dpipe_entry_index(&self) -> Result<u64, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Devlink::DpipeEntryIndex(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Devlink",
            "DpipeEntryIndex",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_dpipe_entry_match_values(
        &self,
    ) -> Result<IterableDlDpipeEntryMatchValues<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Devlink::DpipeEntryMatchValues(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Devlink",
            "DpipeEntryMatchValues",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_dpipe_entry_action_values(
        &self,
    ) -> Result<IterableDlDpipeEntryActionValues<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Devlink::DpipeEntryActionValues(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Devlink",
            "DpipeEntryActionValues",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_dpipe_entry_counter(&self) -> Result<u64, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Devlink::DpipeEntryCounter(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Devlink",
            "DpipeEntryCounter",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Attribute may repeat multiple times (treat it as array)"]
    pub fn get_dpipe_match(
        &self,
    ) -> MultiAttrIterable<Self, Devlink<'a>, IterableDlDpipeMatch<'a>> {
        MultiAttrIterable::new(self.clone(), |variant| {
            if let Devlink::DpipeMatch(val) = variant {
                Some(val)
            } else {
                None
            }
        })
    }
    #[doc = "Attribute may repeat multiple times (treat it as array)"]
    pub fn get_dpipe_match_value(
        &self,
    ) -> MultiAttrIterable<Self, Devlink<'a>, IterableDlDpipeMatchValue<'a>> {
        MultiAttrIterable::new(self.clone(), |variant| {
            if let Devlink::DpipeMatchValue(val) = variant {
                Some(val)
            } else {
                None
            }
        })
    }
    #[doc = "Associated type: [`DpipeMatchType`] (enum)"]
    pub fn get_dpipe_match_type(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Devlink::DpipeMatchType(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Devlink",
            "DpipeMatchType",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Attribute may repeat multiple times (treat it as array)"]
    pub fn get_dpipe_action(
        &self,
    ) -> MultiAttrIterable<Self, Devlink<'a>, IterableDlDpipeAction<'a>> {
        MultiAttrIterable::new(self.clone(), |variant| {
            if let Devlink::DpipeAction(val) = variant {
                Some(val)
            } else {
                None
            }
        })
    }
    #[doc = "Attribute may repeat multiple times (treat it as array)"]
    pub fn get_dpipe_action_value(
        &self,
    ) -> MultiAttrIterable<Self, Devlink<'a>, IterableDlDpipeActionValue<'a>> {
        MultiAttrIterable::new(self.clone(), |variant| {
            if let Devlink::DpipeActionValue(val) = variant {
                Some(val)
            } else {
                None
            }
        })
    }
    #[doc = "Associated type: [`DpipeActionType`] (enum)"]
    pub fn get_dpipe_action_type(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Devlink::DpipeActionType(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Devlink",
            "DpipeActionType",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_dpipe_value(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Devlink::DpipeValue(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Devlink",
            "DpipeValue",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_dpipe_value_mask(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Devlink::DpipeValueMask(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Devlink",
            "DpipeValueMask",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_dpipe_value_mapping(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Devlink::DpipeValueMapping(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Devlink",
            "DpipeValueMapping",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_dpipe_headers(&self) -> Result<IterableDlDpipeHeaders<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Devlink::DpipeHeaders(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Devlink",
            "DpipeHeaders",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Attribute may repeat multiple times (treat it as array)"]
    pub fn get_dpipe_header(
        &self,
    ) -> MultiAttrIterable<Self, Devlink<'a>, IterableDlDpipeHeader<'a>> {
        MultiAttrIterable::new(self.clone(), |variant| {
            if let Devlink::DpipeHeader(val) = variant {
                Some(val)
            } else {
                None
            }
        })
    }
    pub fn get_dpipe_header_name(&self) -> Result<&'a CStr, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Devlink::DpipeHeaderName(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Devlink",
            "DpipeHeaderName",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Associated type: [`DpipeHeaderId`] (enum)"]
    pub fn get_dpipe_header_id(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Devlink::DpipeHeaderId(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Devlink",
            "DpipeHeaderId",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_dpipe_header_fields(&self) -> Result<IterableDlDpipeHeaderFields<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Devlink::DpipeHeaderFields(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Devlink",
            "DpipeHeaderFields",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_dpipe_header_global(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Devlink::DpipeHeaderGlobal(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Devlink",
            "DpipeHeaderGlobal",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_dpipe_header_index(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Devlink::DpipeHeaderIndex(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Devlink",
            "DpipeHeaderIndex",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Attribute may repeat multiple times (treat it as array)"]
    pub fn get_dpipe_field(
        &self,
    ) -> MultiAttrIterable<Self, Devlink<'a>, IterableDlDpipeField<'a>> {
        MultiAttrIterable::new(self.clone(), |variant| {
            if let Devlink::DpipeField(val) = variant {
                Some(val)
            } else {
                None
            }
        })
    }
    pub fn get_dpipe_field_name(&self) -> Result<&'a CStr, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Devlink::DpipeFieldName(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Devlink",
            "DpipeFieldName",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_dpipe_field_id(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Devlink::DpipeFieldId(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Devlink",
            "DpipeFieldId",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_dpipe_field_bitwidth(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Devlink::DpipeFieldBitwidth(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Devlink",
            "DpipeFieldBitwidth",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Associated type: [`DpipeFieldMappingType`] (enum)"]
    pub fn get_dpipe_field_mapping_type(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Devlink::DpipeFieldMappingType(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Devlink",
            "DpipeFieldMappingType",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_pad(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Devlink::Pad(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Devlink",
            "Pad",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Associated type: [`EswitchEncapMode`] (enum)"]
    pub fn get_eswitch_encap_mode(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Devlink::EswitchEncapMode(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Devlink",
            "EswitchEncapMode",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_resource_list(&self) -> Result<IterableDlResourceList<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Devlink::ResourceList(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Devlink",
            "ResourceList",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Attribute may repeat multiple times (treat it as array)"]
    pub fn get_resource(&self) -> MultiAttrIterable<Self, Devlink<'a>, IterableDlResource<'a>> {
        MultiAttrIterable::new(self.clone(), |variant| {
            if let Devlink::Resource(val) = variant {
                Some(val)
            } else {
                None
            }
        })
    }
    pub fn get_resource_name(&self) -> Result<&'a CStr, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Devlink::ResourceName(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Devlink",
            "ResourceName",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_resource_id(&self) -> Result<u64, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Devlink::ResourceId(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Devlink",
            "ResourceId",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_resource_size(&self) -> Result<u64, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Devlink::ResourceSize(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Devlink",
            "ResourceSize",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_resource_size_new(&self) -> Result<u64, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Devlink::ResourceSizeNew(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Devlink",
            "ResourceSizeNew",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_resource_size_valid(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Devlink::ResourceSizeValid(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Devlink",
            "ResourceSizeValid",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_resource_size_min(&self) -> Result<u64, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Devlink::ResourceSizeMin(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Devlink",
            "ResourceSizeMin",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_resource_size_max(&self) -> Result<u64, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Devlink::ResourceSizeMax(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Devlink",
            "ResourceSizeMax",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_resource_size_gran(&self) -> Result<u64, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Devlink::ResourceSizeGran(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Devlink",
            "ResourceSizeGran",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Associated type: [`ResourceUnit`] (enum)"]
    pub fn get_resource_unit(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Devlink::ResourceUnit(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Devlink",
            "ResourceUnit",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_resource_occ(&self) -> Result<u64, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Devlink::ResourceOcc(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Devlink",
            "ResourceOcc",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_dpipe_table_resource_id(&self) -> Result<u64, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Devlink::DpipeTableResourceId(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Devlink",
            "DpipeTableResourceId",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_dpipe_table_resource_units(&self) -> Result<u64, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Devlink::DpipeTableResourceUnits(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Devlink",
            "DpipeTableResourceUnits",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Associated type: [`PortFlavour`] (enum)"]
    pub fn get_port_flavour(&self) -> Result<u16, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Devlink::PortFlavour(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Devlink",
            "PortFlavour",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_port_number(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Devlink::PortNumber(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Devlink",
            "PortNumber",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_port_split_subport_number(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Devlink::PortSplitSubportNumber(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Devlink",
            "PortSplitSubportNumber",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_param(&self) -> Result<IterableDlParam<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Devlink::Param(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Devlink",
            "Param",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_param_name(&self) -> Result<&'a CStr, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Devlink::ParamName(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Devlink",
            "ParamName",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_param_generic(&self) -> Result<(), ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Devlink::ParamGeneric(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Devlink",
            "ParamGeneric",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Associated type: [`VarAttrType`] (enum)"]
    pub fn get_param_type(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Devlink::ParamType(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Devlink",
            "ParamType",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Associated type: [`ParamCmode`] (enum)"]
    pub fn get_param_value_cmode(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Devlink::ParamValueCmode(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Devlink",
            "ParamValueCmode",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_region_name(&self) -> Result<&'a CStr, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Devlink::RegionName(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Devlink",
            "RegionName",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_region_size(&self) -> Result<u64, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Devlink::RegionSize(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Devlink",
            "RegionSize",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_region_snapshots(&self) -> Result<IterableDlRegionSnapshots<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Devlink::RegionSnapshots(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Devlink",
            "RegionSnapshots",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_region_snapshot(&self) -> Result<IterableDlRegionSnapshot<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Devlink::RegionSnapshot(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Devlink",
            "RegionSnapshot",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_region_snapshot_id(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Devlink::RegionSnapshotId(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Devlink",
            "RegionSnapshotId",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_region_chunks(&self) -> Result<IterableDlRegionChunks<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Devlink::RegionChunks(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Devlink",
            "RegionChunks",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_region_chunk(&self) -> Result<IterableDlRegionChunk<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Devlink::RegionChunk(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Devlink",
            "RegionChunk",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_region_chunk_data(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Devlink::RegionChunkData(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Devlink",
            "RegionChunkData",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_region_chunk_addr(&self) -> Result<u64, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Devlink::RegionChunkAddr(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Devlink",
            "RegionChunkAddr",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_region_chunk_len(&self) -> Result<u64, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Devlink::RegionChunkLen(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Devlink",
            "RegionChunkLen",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_info_driver_name(&self) -> Result<&'a CStr, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Devlink::InfoDriverName(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Devlink",
            "InfoDriverName",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_info_serial_number(&self) -> Result<&'a CStr, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Devlink::InfoSerialNumber(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Devlink",
            "InfoSerialNumber",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Attribute may repeat multiple times (treat it as array)"]
    pub fn get_info_version_fixed(
        &self,
    ) -> MultiAttrIterable<Self, Devlink<'a>, IterableDlInfoVersion<'a>> {
        MultiAttrIterable::new(self.clone(), |variant| {
            if let Devlink::InfoVersionFixed(val) = variant {
                Some(val)
            } else {
                None
            }
        })
    }
    #[doc = "Attribute may repeat multiple times (treat it as array)"]
    pub fn get_info_version_running(
        &self,
    ) -> MultiAttrIterable<Self, Devlink<'a>, IterableDlInfoVersion<'a>> {
        MultiAttrIterable::new(self.clone(), |variant| {
            if let Devlink::InfoVersionRunning(val) = variant {
                Some(val)
            } else {
                None
            }
        })
    }
    #[doc = "Attribute may repeat multiple times (treat it as array)"]
    pub fn get_info_version_stored(
        &self,
    ) -> MultiAttrIterable<Self, Devlink<'a>, IterableDlInfoVersion<'a>> {
        MultiAttrIterable::new(self.clone(), |variant| {
            if let Devlink::InfoVersionStored(val) = variant {
                Some(val)
            } else {
                None
            }
        })
    }
    pub fn get_info_version_name(&self) -> Result<&'a CStr, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Devlink::InfoVersionName(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Devlink",
            "InfoVersionName",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_info_version_value(&self) -> Result<&'a CStr, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Devlink::InfoVersionValue(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Devlink",
            "InfoVersionValue",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_sb_pool_cell_size(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Devlink::SbPoolCellSize(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Devlink",
            "SbPoolCellSize",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_fmsg(&self) -> Result<IterableDlFmsg<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Devlink::Fmsg(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Devlink",
            "Fmsg",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_fmsg_obj_nest_start(&self) -> Result<(), ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Devlink::FmsgObjNestStart(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Devlink",
            "FmsgObjNestStart",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_fmsg_pair_nest_start(&self) -> Result<(), ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Devlink::FmsgPairNestStart(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Devlink",
            "FmsgPairNestStart",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_fmsg_arr_nest_start(&self) -> Result<(), ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Devlink::FmsgArrNestStart(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Devlink",
            "FmsgArrNestStart",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_fmsg_nest_end(&self) -> Result<(), ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Devlink::FmsgNestEnd(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Devlink",
            "FmsgNestEnd",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_fmsg_obj_name(&self) -> Result<&'a CStr, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Devlink::FmsgObjName(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Devlink",
            "FmsgObjName",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Associated type: [`VarAttrType`] (enum)"]
    pub fn get_fmsg_obj_value_type(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Devlink::FmsgObjValueType(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Devlink",
            "FmsgObjValueType",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_health_reporter(&self) -> Result<IterableDlHealthReporter<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Devlink::HealthReporter(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Devlink",
            "HealthReporter",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_health_reporter_name(&self) -> Result<&'a CStr, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Devlink::HealthReporterName(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Devlink",
            "HealthReporterName",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_health_reporter_state(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Devlink::HealthReporterState(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Devlink",
            "HealthReporterState",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_health_reporter_err_count(&self) -> Result<u64, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Devlink::HealthReporterErrCount(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Devlink",
            "HealthReporterErrCount",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_health_reporter_recover_count(&self) -> Result<u64, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Devlink::HealthReporterRecoverCount(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Devlink",
            "HealthReporterRecoverCount",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_health_reporter_dump_ts(&self) -> Result<u64, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Devlink::HealthReporterDumpTs(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Devlink",
            "HealthReporterDumpTs",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_health_reporter_graceful_period(&self) -> Result<u64, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Devlink::HealthReporterGracefulPeriod(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Devlink",
            "HealthReporterGracefulPeriod",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_health_reporter_auto_recover(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Devlink::HealthReporterAutoRecover(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Devlink",
            "HealthReporterAutoRecover",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_flash_update_file_name(&self) -> Result<&'a CStr, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Devlink::FlashUpdateFileName(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Devlink",
            "FlashUpdateFileName",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_flash_update_component(&self) -> Result<&'a CStr, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Devlink::FlashUpdateComponent(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Devlink",
            "FlashUpdateComponent",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_flash_update_status_msg(&self) -> Result<&'a CStr, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Devlink::FlashUpdateStatusMsg(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Devlink",
            "FlashUpdateStatusMsg",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_flash_update_status_done(&self) -> Result<u64, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Devlink::FlashUpdateStatusDone(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Devlink",
            "FlashUpdateStatusDone",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_flash_update_status_total(&self) -> Result<u64, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Devlink::FlashUpdateStatusTotal(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Devlink",
            "FlashUpdateStatusTotal",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_port_pci_pf_number(&self) -> Result<u16, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Devlink::PortPciPfNumber(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Devlink",
            "PortPciPfNumber",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_port_pci_vf_number(&self) -> Result<u16, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Devlink::PortPciVfNumber(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Devlink",
            "PortPciVfNumber",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_stats(&self) -> Result<IterableDlAttrStats<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Devlink::Stats(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Devlink",
            "Stats",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_trap_name(&self) -> Result<&'a CStr, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Devlink::TrapName(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Devlink",
            "TrapName",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Associated type: [`TrapAction`] (enum)"]
    pub fn get_trap_action(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Devlink::TrapAction(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Devlink",
            "TrapAction",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Associated type: [`TrapType`] (enum)"]
    pub fn get_trap_type(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Devlink::TrapType(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Devlink",
            "TrapType",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_trap_generic(&self) -> Result<(), ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Devlink::TrapGeneric(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Devlink",
            "TrapGeneric",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_trap_metadata(&self) -> Result<IterableDlTrapMetadata<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Devlink::TrapMetadata(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Devlink",
            "TrapMetadata",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_trap_group_name(&self) -> Result<&'a CStr, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Devlink::TrapGroupName(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Devlink",
            "TrapGroupName",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_reload_failed(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Devlink::ReloadFailed(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Devlink",
            "ReloadFailed",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_health_reporter_dump_ts_ns(&self) -> Result<u64, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Devlink::HealthReporterDumpTsNs(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Devlink",
            "HealthReporterDumpTsNs",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_netns_fd(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Devlink::NetnsFd(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Devlink",
            "NetnsFd",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_netns_pid(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Devlink::NetnsPid(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Devlink",
            "NetnsPid",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_netns_id(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Devlink::NetnsId(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Devlink",
            "NetnsId",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_health_reporter_auto_dump(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Devlink::HealthReporterAutoDump(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Devlink",
            "HealthReporterAutoDump",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_trap_policer_id(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Devlink::TrapPolicerId(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Devlink",
            "TrapPolicerId",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_trap_policer_rate(&self) -> Result<u64, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Devlink::TrapPolicerRate(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Devlink",
            "TrapPolicerRate",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_trap_policer_burst(&self) -> Result<u64, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Devlink::TrapPolicerBurst(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Devlink",
            "TrapPolicerBurst",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_port_function(&self) -> Result<IterableDlPortFunction<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Devlink::PortFunction(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Devlink",
            "PortFunction",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_info_board_serial_number(&self) -> Result<&'a CStr, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Devlink::InfoBoardSerialNumber(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Devlink",
            "InfoBoardSerialNumber",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_port_lanes(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Devlink::PortLanes(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Devlink",
            "PortLanes",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_port_splittable(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Devlink::PortSplittable(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Devlink",
            "PortSplittable",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_port_external(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Devlink::PortExternal(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Devlink",
            "PortExternal",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_port_controller_number(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Devlink::PortControllerNumber(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Devlink",
            "PortControllerNumber",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_flash_update_status_timeout(&self) -> Result<u64, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Devlink::FlashUpdateStatusTimeout(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Devlink",
            "FlashUpdateStatusTimeout",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Associated type: [`FlashOverwrite`] (1 bit per enumeration)"]
    pub fn get_flash_update_overwrite_mask(&self) -> Result<BuiltinBitfield32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Devlink::FlashUpdateOverwriteMask(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Devlink",
            "FlashUpdateOverwriteMask",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Associated type: [`ReloadAction`] (enum)"]
    pub fn get_reload_action(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Devlink::ReloadAction(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Devlink",
            "ReloadAction",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Associated type: [`ReloadAction`] (1 bit per enumeration)"]
    pub fn get_reload_actions_performed(&self) -> Result<BuiltinBitfield32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Devlink::ReloadActionsPerformed(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Devlink",
            "ReloadActionsPerformed",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Associated type: [`ReloadAction`] (1 bit per enumeration)"]
    pub fn get_reload_limits(&self) -> Result<BuiltinBitfield32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Devlink::ReloadLimits(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Devlink",
            "ReloadLimits",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_dev_stats(&self) -> Result<IterableDlDevStats<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Devlink::DevStats(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Devlink",
            "DevStats",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_reload_stats(&self) -> Result<IterableDlReloadStats<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Devlink::ReloadStats(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Devlink",
            "ReloadStats",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Attribute may repeat multiple times (treat it as array)"]
    pub fn get_reload_stats_entry(
        &self,
    ) -> MultiAttrIterable<Self, Devlink<'a>, IterableDlReloadStatsEntry<'a>> {
        MultiAttrIterable::new(self.clone(), |variant| {
            if let Devlink::ReloadStatsEntry(val) = variant {
                Some(val)
            } else {
                None
            }
        })
    }
    pub fn get_reload_stats_limit(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Devlink::ReloadStatsLimit(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Devlink",
            "ReloadStatsLimit",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_reload_stats_value(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Devlink::ReloadStatsValue(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Devlink",
            "ReloadStatsValue",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_remote_reload_stats(&self) -> Result<IterableDlReloadStats<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Devlink::RemoteReloadStats(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Devlink",
            "RemoteReloadStats",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Attribute may repeat multiple times (treat it as array)"]
    pub fn get_reload_action_info(
        &self,
    ) -> MultiAttrIterable<Self, Devlink<'a>, IterableDlReloadActInfo<'a>> {
        MultiAttrIterable::new(self.clone(), |variant| {
            if let Devlink::ReloadActionInfo(val) = variant {
                Some(val)
            } else {
                None
            }
        })
    }
    #[doc = "Attribute may repeat multiple times (treat it as array)"]
    pub fn get_reload_action_stats(
        &self,
    ) -> MultiAttrIterable<Self, Devlink<'a>, IterableDlReloadActStats<'a>> {
        MultiAttrIterable::new(self.clone(), |variant| {
            if let Devlink::ReloadActionStats(val) = variant {
                Some(val)
            } else {
                None
            }
        })
    }
    pub fn get_port_pci_sf_number(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Devlink::PortPciSfNumber(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Devlink",
            "PortPciSfNumber",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Associated type: [`RateType`] (enum)"]
    pub fn get_rate_type(&self) -> Result<u16, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Devlink::RateType(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Devlink",
            "RateType",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_rate_tx_share(&self) -> Result<u64, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Devlink::RateTxShare(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Devlink",
            "RateTxShare",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_rate_tx_max(&self) -> Result<u64, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Devlink::RateTxMax(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Devlink",
            "RateTxMax",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_rate_node_name(&self) -> Result<&'a CStr, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Devlink::RateNodeName(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Devlink",
            "RateNodeName",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_rate_parent_node_name(&self) -> Result<&'a CStr, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Devlink::RateParentNodeName(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Devlink",
            "RateParentNodeName",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_region_max_snapshots(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Devlink::RegionMaxSnapshots(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Devlink",
            "RegionMaxSnapshots",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_linecard_index(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Devlink::LinecardIndex(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Devlink",
            "LinecardIndex",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_linecard_state(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Devlink::LinecardState(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Devlink",
            "LinecardState",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_linecard_type(&self) -> Result<&'a CStr, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Devlink::LinecardType(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Devlink",
            "LinecardType",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_linecard_supported_types(
        &self,
    ) -> Result<IterableDlLinecardSupportedTypes<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Devlink::LinecardSupportedTypes(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Devlink",
            "LinecardSupportedTypes",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_selftests(&self) -> Result<IterableDlSelftestId<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Devlink::Selftests(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Devlink",
            "Selftests",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_rate_tx_priority(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Devlink::RateTxPriority(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Devlink",
            "RateTxPriority",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_rate_tx_weight(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Devlink::RateTxWeight(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Devlink",
            "RateTxWeight",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_region_direct(&self) -> Result<(), ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Devlink::RegionDirect(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Devlink",
            "RegionDirect",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Attribute may repeat multiple times (treat it as array)"]
    pub fn get_rate_tc_bws(&self) -> MultiAttrIterable<Self, Devlink<'a>, IterableDlRateTcBws<'a>> {
        MultiAttrIterable::new(self.clone(), |variant| {
            if let Devlink::RateTcBws(val) = variant {
                Some(val)
            } else {
                None
            }
        })
    }
    #[doc = "Time (in msec) for recoveries before starting the grace period.\n"]
    pub fn get_health_reporter_burst_period(&self) -> Result<u64, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Devlink::HealthReporterBurstPeriod(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Devlink",
            "HealthReporterBurstPeriod",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Request restoring parameter to its default value.\n"]
    pub fn get_param_reset_default(&self) -> Result<(), ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Devlink::ParamResetDefault(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Devlink",
            "ParamResetDefault",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Unique devlink instance index.\n"]
    pub fn get_index(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Devlink::Index(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Devlink",
            "Index",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Bitmask selecting which resource classes to include in a resource-dump\nresponse. Bit 0 (dev) selects device-level resources; bit 1 (port)\nselects port-level resources. When absent all classes are returned.\n\nAssociated type: [`ResourceScope`] (1 bit per enumeration)"]
    pub fn get_resource_scope_mask(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Devlink::ResourceScopeMask(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Devlink",
            "ResourceScopeMask",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl Devlink<'_> {
    pub fn new<'a>(buf: &'a [u8]) -> IterableDevlink<'a> {
        IterableDevlink::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        let res = match r#type {
            1u16 => "BusName",
            2u16 => "DevName",
            3u16 => "PortIndex",
            4u16 => "PortType",
            5u16 => "PortDesiredType",
            6u16 => "PortNetdevIfindex",
            7u16 => "PortNetdevName",
            8u16 => "PortIbdevName",
            9u16 => "PortSplitCount",
            10u16 => "PortSplitGroup",
            11u16 => "SbIndex",
            12u16 => "SbSize",
            13u16 => "SbIngressPoolCount",
            14u16 => "SbEgressPoolCount",
            15u16 => "SbIngressTcCount",
            16u16 => "SbEgressTcCount",
            17u16 => "SbPoolIndex",
            18u16 => "SbPoolType",
            19u16 => "SbPoolSize",
            20u16 => "SbPoolThresholdType",
            21u16 => "SbThreshold",
            22u16 => "SbTcIndex",
            23u16 => "SbOccCur",
            24u16 => "SbOccMax",
            25u16 => "EswitchMode",
            26u16 => "EswitchInlineMode",
            27u16 => "DpipeTables",
            28u16 => "DpipeTable",
            29u16 => "DpipeTableName",
            30u16 => "DpipeTableSize",
            31u16 => "DpipeTableMatches",
            32u16 => "DpipeTableActions",
            33u16 => "DpipeTableCountersEnabled",
            34u16 => "DpipeEntries",
            35u16 => "DpipeEntry",
            36u16 => "DpipeEntryIndex",
            37u16 => "DpipeEntryMatchValues",
            38u16 => "DpipeEntryActionValues",
            39u16 => "DpipeEntryCounter",
            40u16 => "DpipeMatch",
            41u16 => "DpipeMatchValue",
            42u16 => "DpipeMatchType",
            43u16 => "DpipeAction",
            44u16 => "DpipeActionValue",
            45u16 => "DpipeActionType",
            46u16 => "DpipeValue",
            47u16 => "DpipeValueMask",
            48u16 => "DpipeValueMapping",
            49u16 => "DpipeHeaders",
            50u16 => "DpipeHeader",
            51u16 => "DpipeHeaderName",
            52u16 => "DpipeHeaderId",
            53u16 => "DpipeHeaderFields",
            54u16 => "DpipeHeaderGlobal",
            55u16 => "DpipeHeaderIndex",
            56u16 => "DpipeField",
            57u16 => "DpipeFieldName",
            58u16 => "DpipeFieldId",
            59u16 => "DpipeFieldBitwidth",
            60u16 => "DpipeFieldMappingType",
            61u16 => "Pad",
            62u16 => "EswitchEncapMode",
            63u16 => "ResourceList",
            64u16 => "Resource",
            65u16 => "ResourceName",
            66u16 => "ResourceId",
            67u16 => "ResourceSize",
            68u16 => "ResourceSizeNew",
            69u16 => "ResourceSizeValid",
            70u16 => "ResourceSizeMin",
            71u16 => "ResourceSizeMax",
            72u16 => "ResourceSizeGran",
            73u16 => "ResourceUnit",
            74u16 => "ResourceOcc",
            75u16 => "DpipeTableResourceId",
            76u16 => "DpipeTableResourceUnits",
            77u16 => "PortFlavour",
            78u16 => "PortNumber",
            79u16 => "PortSplitSubportNumber",
            80u16 => "Param",
            81u16 => "ParamName",
            82u16 => "ParamGeneric",
            83u16 => "ParamType",
            87u16 => "ParamValueCmode",
            88u16 => "RegionName",
            89u16 => "RegionSize",
            90u16 => "RegionSnapshots",
            91u16 => "RegionSnapshot",
            92u16 => "RegionSnapshotId",
            93u16 => "RegionChunks",
            94u16 => "RegionChunk",
            95u16 => "RegionChunkData",
            96u16 => "RegionChunkAddr",
            97u16 => "RegionChunkLen",
            98u16 => "InfoDriverName",
            99u16 => "InfoSerialNumber",
            100u16 => "InfoVersionFixed",
            101u16 => "InfoVersionRunning",
            102u16 => "InfoVersionStored",
            103u16 => "InfoVersionName",
            104u16 => "InfoVersionValue",
            105u16 => "SbPoolCellSize",
            106u16 => "Fmsg",
            107u16 => "FmsgObjNestStart",
            108u16 => "FmsgPairNestStart",
            109u16 => "FmsgArrNestStart",
            110u16 => "FmsgNestEnd",
            111u16 => "FmsgObjName",
            112u16 => "FmsgObjValueType",
            114u16 => "HealthReporter",
            115u16 => "HealthReporterName",
            116u16 => "HealthReporterState",
            117u16 => "HealthReporterErrCount",
            118u16 => "HealthReporterRecoverCount",
            119u16 => "HealthReporterDumpTs",
            120u16 => "HealthReporterGracefulPeriod",
            121u16 => "HealthReporterAutoRecover",
            122u16 => "FlashUpdateFileName",
            123u16 => "FlashUpdateComponent",
            124u16 => "FlashUpdateStatusMsg",
            125u16 => "FlashUpdateStatusDone",
            126u16 => "FlashUpdateStatusTotal",
            127u16 => "PortPciPfNumber",
            128u16 => "PortPciVfNumber",
            129u16 => "Stats",
            130u16 => "TrapName",
            131u16 => "TrapAction",
            132u16 => "TrapType",
            133u16 => "TrapGeneric",
            134u16 => "TrapMetadata",
            135u16 => "TrapGroupName",
            136u16 => "ReloadFailed",
            137u16 => "HealthReporterDumpTsNs",
            138u16 => "NetnsFd",
            139u16 => "NetnsPid",
            140u16 => "NetnsId",
            141u16 => "HealthReporterAutoDump",
            142u16 => "TrapPolicerId",
            143u16 => "TrapPolicerRate",
            144u16 => "TrapPolicerBurst",
            145u16 => "PortFunction",
            146u16 => "InfoBoardSerialNumber",
            147u16 => "PortLanes",
            148u16 => "PortSplittable",
            149u16 => "PortExternal",
            150u16 => "PortControllerNumber",
            151u16 => "FlashUpdateStatusTimeout",
            152u16 => "FlashUpdateOverwriteMask",
            153u16 => "ReloadAction",
            154u16 => "ReloadActionsPerformed",
            155u16 => "ReloadLimits",
            156u16 => "DevStats",
            157u16 => "ReloadStats",
            158u16 => "ReloadStatsEntry",
            159u16 => "ReloadStatsLimit",
            160u16 => "ReloadStatsValue",
            161u16 => "RemoteReloadStats",
            162u16 => "ReloadActionInfo",
            163u16 => "ReloadActionStats",
            164u16 => "PortPciSfNumber",
            165u16 => "RateType",
            166u16 => "RateTxShare",
            167u16 => "RateTxMax",
            168u16 => "RateNodeName",
            169u16 => "RateParentNodeName",
            170u16 => "RegionMaxSnapshots",
            171u16 => "LinecardIndex",
            172u16 => "LinecardState",
            173u16 => "LinecardType",
            174u16 => "LinecardSupportedTypes",
            176u16 => "Selftests",
            177u16 => "RateTxPriority",
            178u16 => "RateTxWeight",
            179u16 => "RegionDirect",
            180u16 => "RateTcBws",
            181u16 => "HealthReporterBurstPeriod",
            183u16 => "ParamResetDefault",
            184u16 => "Index",
            185u16 => "ResourceScopeMask",
            _ => return None,
        };
        Some(res)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableDevlink<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableDevlink<'a> {
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
impl<'a> Iterator for IterableDevlink<'a> {
    type Item = Result<Devlink<'a>, ErrorContext>;
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
                1u16 => Devlink::BusName({
                    let res = CStr::from_bytes_with_nul(next).ok();
                    let Some(val) = res else { break };
                    val
                }),
                2u16 => Devlink::DevName({
                    let res = CStr::from_bytes_with_nul(next).ok();
                    let Some(val) = res else { break };
                    val
                }),
                3u16 => Devlink::PortIndex({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                4u16 => Devlink::PortType({
                    let res = parse_u16(next);
                    let Some(val) = res else { break };
                    val
                }),
                5u16 => Devlink::PortDesiredType({
                    let res = parse_u16(next);
                    let Some(val) = res else { break };
                    val
                }),
                6u16 => Devlink::PortNetdevIfindex({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                7u16 => Devlink::PortNetdevName({
                    let res = CStr::from_bytes_with_nul(next).ok();
                    let Some(val) = res else { break };
                    val
                }),
                8u16 => Devlink::PortIbdevName({
                    let res = CStr::from_bytes_with_nul(next).ok();
                    let Some(val) = res else { break };
                    val
                }),
                9u16 => Devlink::PortSplitCount({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                10u16 => Devlink::PortSplitGroup({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                11u16 => Devlink::SbIndex({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                12u16 => Devlink::SbSize({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                13u16 => Devlink::SbIngressPoolCount({
                    let res = parse_u16(next);
                    let Some(val) = res else { break };
                    val
                }),
                14u16 => Devlink::SbEgressPoolCount({
                    let res = parse_u16(next);
                    let Some(val) = res else { break };
                    val
                }),
                15u16 => Devlink::SbIngressTcCount({
                    let res = parse_u16(next);
                    let Some(val) = res else { break };
                    val
                }),
                16u16 => Devlink::SbEgressTcCount({
                    let res = parse_u16(next);
                    let Some(val) = res else { break };
                    val
                }),
                17u16 => Devlink::SbPoolIndex({
                    let res = parse_u16(next);
                    let Some(val) = res else { break };
                    val
                }),
                18u16 => Devlink::SbPoolType({
                    let res = parse_u8(next);
                    let Some(val) = res else { break };
                    val
                }),
                19u16 => Devlink::SbPoolSize({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                20u16 => Devlink::SbPoolThresholdType({
                    let res = parse_u8(next);
                    let Some(val) = res else { break };
                    val
                }),
                21u16 => Devlink::SbThreshold({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                22u16 => Devlink::SbTcIndex({
                    let res = parse_u16(next);
                    let Some(val) = res else { break };
                    val
                }),
                23u16 => Devlink::SbOccCur({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                24u16 => Devlink::SbOccMax({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                25u16 => Devlink::EswitchMode({
                    let res = parse_u16(next);
                    let Some(val) = res else { break };
                    val
                }),
                26u16 => Devlink::EswitchInlineMode({
                    let res = parse_u8(next);
                    let Some(val) = res else { break };
                    val
                }),
                27u16 => Devlink::DpipeTables({
                    let res = Some(IterableDlDpipeTables::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                28u16 => Devlink::DpipeTable({
                    let res = Some(IterableDlDpipeTable::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                29u16 => Devlink::DpipeTableName({
                    let res = CStr::from_bytes_with_nul(next).ok();
                    let Some(val) = res else { break };
                    val
                }),
                30u16 => Devlink::DpipeTableSize({
                    let res = parse_u64(next);
                    let Some(val) = res else { break };
                    val
                }),
                31u16 => Devlink::DpipeTableMatches({
                    let res = Some(IterableDlDpipeTableMatches::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                32u16 => Devlink::DpipeTableActions({
                    let res = Some(IterableDlDpipeTableActions::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                33u16 => Devlink::DpipeTableCountersEnabled({
                    let res = parse_u8(next);
                    let Some(val) = res else { break };
                    val
                }),
                34u16 => Devlink::DpipeEntries({
                    let res = Some(IterableDlDpipeEntries::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                35u16 => Devlink::DpipeEntry({
                    let res = Some(IterableDlDpipeEntry::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                36u16 => Devlink::DpipeEntryIndex({
                    let res = parse_u64(next);
                    let Some(val) = res else { break };
                    val
                }),
                37u16 => Devlink::DpipeEntryMatchValues({
                    let res = Some(IterableDlDpipeEntryMatchValues::with_loc(
                        next,
                        self.orig_loc,
                    ));
                    let Some(val) = res else { break };
                    val
                }),
                38u16 => Devlink::DpipeEntryActionValues({
                    let res = Some(IterableDlDpipeEntryActionValues::with_loc(
                        next,
                        self.orig_loc,
                    ));
                    let Some(val) = res else { break };
                    val
                }),
                39u16 => Devlink::DpipeEntryCounter({
                    let res = parse_u64(next);
                    let Some(val) = res else { break };
                    val
                }),
                40u16 => Devlink::DpipeMatch({
                    let res = Some(IterableDlDpipeMatch::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                41u16 => Devlink::DpipeMatchValue({
                    let res = Some(IterableDlDpipeMatchValue::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                42u16 => Devlink::DpipeMatchType({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                43u16 => Devlink::DpipeAction({
                    let res = Some(IterableDlDpipeAction::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                44u16 => Devlink::DpipeActionValue({
                    let res = Some(IterableDlDpipeActionValue::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                45u16 => Devlink::DpipeActionType({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                46u16 => Devlink::DpipeValue({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                47u16 => Devlink::DpipeValueMask({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                48u16 => Devlink::DpipeValueMapping({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                49u16 => Devlink::DpipeHeaders({
                    let res = Some(IterableDlDpipeHeaders::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                50u16 => Devlink::DpipeHeader({
                    let res = Some(IterableDlDpipeHeader::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                51u16 => Devlink::DpipeHeaderName({
                    let res = CStr::from_bytes_with_nul(next).ok();
                    let Some(val) = res else { break };
                    val
                }),
                52u16 => Devlink::DpipeHeaderId({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                53u16 => Devlink::DpipeHeaderFields({
                    let res = Some(IterableDlDpipeHeaderFields::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                54u16 => Devlink::DpipeHeaderGlobal({
                    let res = parse_u8(next);
                    let Some(val) = res else { break };
                    val
                }),
                55u16 => Devlink::DpipeHeaderIndex({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                56u16 => Devlink::DpipeField({
                    let res = Some(IterableDlDpipeField::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                57u16 => Devlink::DpipeFieldName({
                    let res = CStr::from_bytes_with_nul(next).ok();
                    let Some(val) = res else { break };
                    val
                }),
                58u16 => Devlink::DpipeFieldId({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                59u16 => Devlink::DpipeFieldBitwidth({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                60u16 => Devlink::DpipeFieldMappingType({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                61u16 => Devlink::Pad({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                62u16 => Devlink::EswitchEncapMode({
                    let res = parse_u8(next);
                    let Some(val) = res else { break };
                    val
                }),
                63u16 => Devlink::ResourceList({
                    let res = Some(IterableDlResourceList::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                64u16 => Devlink::Resource({
                    let res = Some(IterableDlResource::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                65u16 => Devlink::ResourceName({
                    let res = CStr::from_bytes_with_nul(next).ok();
                    let Some(val) = res else { break };
                    val
                }),
                66u16 => Devlink::ResourceId({
                    let res = parse_u64(next);
                    let Some(val) = res else { break };
                    val
                }),
                67u16 => Devlink::ResourceSize({
                    let res = parse_u64(next);
                    let Some(val) = res else { break };
                    val
                }),
                68u16 => Devlink::ResourceSizeNew({
                    let res = parse_u64(next);
                    let Some(val) = res else { break };
                    val
                }),
                69u16 => Devlink::ResourceSizeValid({
                    let res = parse_u8(next);
                    let Some(val) = res else { break };
                    val
                }),
                70u16 => Devlink::ResourceSizeMin({
                    let res = parse_u64(next);
                    let Some(val) = res else { break };
                    val
                }),
                71u16 => Devlink::ResourceSizeMax({
                    let res = parse_u64(next);
                    let Some(val) = res else { break };
                    val
                }),
                72u16 => Devlink::ResourceSizeGran({
                    let res = parse_u64(next);
                    let Some(val) = res else { break };
                    val
                }),
                73u16 => Devlink::ResourceUnit({
                    let res = parse_u8(next);
                    let Some(val) = res else { break };
                    val
                }),
                74u16 => Devlink::ResourceOcc({
                    let res = parse_u64(next);
                    let Some(val) = res else { break };
                    val
                }),
                75u16 => Devlink::DpipeTableResourceId({
                    let res = parse_u64(next);
                    let Some(val) = res else { break };
                    val
                }),
                76u16 => Devlink::DpipeTableResourceUnits({
                    let res = parse_u64(next);
                    let Some(val) = res else { break };
                    val
                }),
                77u16 => Devlink::PortFlavour({
                    let res = parse_u16(next);
                    let Some(val) = res else { break };
                    val
                }),
                78u16 => Devlink::PortNumber({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                79u16 => Devlink::PortSplitSubportNumber({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                80u16 => Devlink::Param({
                    let res = Some(IterableDlParam::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                81u16 => Devlink::ParamName({
                    let res = CStr::from_bytes_with_nul(next).ok();
                    let Some(val) = res else { break };
                    val
                }),
                82u16 => Devlink::ParamGeneric(()),
                83u16 => Devlink::ParamType({
                    let res = parse_u8(next);
                    let Some(val) = res else { break };
                    val
                }),
                87u16 => Devlink::ParamValueCmode({
                    let res = parse_u8(next);
                    let Some(val) = res else { break };
                    val
                }),
                88u16 => Devlink::RegionName({
                    let res = CStr::from_bytes_with_nul(next).ok();
                    let Some(val) = res else { break };
                    val
                }),
                89u16 => Devlink::RegionSize({
                    let res = parse_u64(next);
                    let Some(val) = res else { break };
                    val
                }),
                90u16 => Devlink::RegionSnapshots({
                    let res = Some(IterableDlRegionSnapshots::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                91u16 => Devlink::RegionSnapshot({
                    let res = Some(IterableDlRegionSnapshot::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                92u16 => Devlink::RegionSnapshotId({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                93u16 => Devlink::RegionChunks({
                    let res = Some(IterableDlRegionChunks::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                94u16 => Devlink::RegionChunk({
                    let res = Some(IterableDlRegionChunk::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                95u16 => Devlink::RegionChunkData({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                96u16 => Devlink::RegionChunkAddr({
                    let res = parse_u64(next);
                    let Some(val) = res else { break };
                    val
                }),
                97u16 => Devlink::RegionChunkLen({
                    let res = parse_u64(next);
                    let Some(val) = res else { break };
                    val
                }),
                98u16 => Devlink::InfoDriverName({
                    let res = CStr::from_bytes_with_nul(next).ok();
                    let Some(val) = res else { break };
                    val
                }),
                99u16 => Devlink::InfoSerialNumber({
                    let res = CStr::from_bytes_with_nul(next).ok();
                    let Some(val) = res else { break };
                    val
                }),
                100u16 => Devlink::InfoVersionFixed({
                    let res = Some(IterableDlInfoVersion::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                101u16 => Devlink::InfoVersionRunning({
                    let res = Some(IterableDlInfoVersion::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                102u16 => Devlink::InfoVersionStored({
                    let res = Some(IterableDlInfoVersion::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                103u16 => Devlink::InfoVersionName({
                    let res = CStr::from_bytes_with_nul(next).ok();
                    let Some(val) = res else { break };
                    val
                }),
                104u16 => Devlink::InfoVersionValue({
                    let res = CStr::from_bytes_with_nul(next).ok();
                    let Some(val) = res else { break };
                    val
                }),
                105u16 => Devlink::SbPoolCellSize({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                106u16 => Devlink::Fmsg({
                    let res = Some(IterableDlFmsg::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                107u16 => Devlink::FmsgObjNestStart(()),
                108u16 => Devlink::FmsgPairNestStart(()),
                109u16 => Devlink::FmsgArrNestStart(()),
                110u16 => Devlink::FmsgNestEnd(()),
                111u16 => Devlink::FmsgObjName({
                    let res = CStr::from_bytes_with_nul(next).ok();
                    let Some(val) = res else { break };
                    val
                }),
                112u16 => Devlink::FmsgObjValueType({
                    let res = parse_u8(next);
                    let Some(val) = res else { break };
                    val
                }),
                114u16 => Devlink::HealthReporter({
                    let res = Some(IterableDlHealthReporter::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                115u16 => Devlink::HealthReporterName({
                    let res = CStr::from_bytes_with_nul(next).ok();
                    let Some(val) = res else { break };
                    val
                }),
                116u16 => Devlink::HealthReporterState({
                    let res = parse_u8(next);
                    let Some(val) = res else { break };
                    val
                }),
                117u16 => Devlink::HealthReporterErrCount({
                    let res = parse_u64(next);
                    let Some(val) = res else { break };
                    val
                }),
                118u16 => Devlink::HealthReporterRecoverCount({
                    let res = parse_u64(next);
                    let Some(val) = res else { break };
                    val
                }),
                119u16 => Devlink::HealthReporterDumpTs({
                    let res = parse_u64(next);
                    let Some(val) = res else { break };
                    val
                }),
                120u16 => Devlink::HealthReporterGracefulPeriod({
                    let res = parse_u64(next);
                    let Some(val) = res else { break };
                    val
                }),
                121u16 => Devlink::HealthReporterAutoRecover({
                    let res = parse_u8(next);
                    let Some(val) = res else { break };
                    val
                }),
                122u16 => Devlink::FlashUpdateFileName({
                    let res = CStr::from_bytes_with_nul(next).ok();
                    let Some(val) = res else { break };
                    val
                }),
                123u16 => Devlink::FlashUpdateComponent({
                    let res = CStr::from_bytes_with_nul(next).ok();
                    let Some(val) = res else { break };
                    val
                }),
                124u16 => Devlink::FlashUpdateStatusMsg({
                    let res = CStr::from_bytes_with_nul(next).ok();
                    let Some(val) = res else { break };
                    val
                }),
                125u16 => Devlink::FlashUpdateStatusDone({
                    let res = parse_u64(next);
                    let Some(val) = res else { break };
                    val
                }),
                126u16 => Devlink::FlashUpdateStatusTotal({
                    let res = parse_u64(next);
                    let Some(val) = res else { break };
                    val
                }),
                127u16 => Devlink::PortPciPfNumber({
                    let res = parse_u16(next);
                    let Some(val) = res else { break };
                    val
                }),
                128u16 => Devlink::PortPciVfNumber({
                    let res = parse_u16(next);
                    let Some(val) = res else { break };
                    val
                }),
                129u16 => Devlink::Stats({
                    let res = Some(IterableDlAttrStats::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                130u16 => Devlink::TrapName({
                    let res = CStr::from_bytes_with_nul(next).ok();
                    let Some(val) = res else { break };
                    val
                }),
                131u16 => Devlink::TrapAction({
                    let res = parse_u8(next);
                    let Some(val) = res else { break };
                    val
                }),
                132u16 => Devlink::TrapType({
                    let res = parse_u8(next);
                    let Some(val) = res else { break };
                    val
                }),
                133u16 => Devlink::TrapGeneric(()),
                134u16 => Devlink::TrapMetadata({
                    let res = Some(IterableDlTrapMetadata::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                135u16 => Devlink::TrapGroupName({
                    let res = CStr::from_bytes_with_nul(next).ok();
                    let Some(val) = res else { break };
                    val
                }),
                136u16 => Devlink::ReloadFailed({
                    let res = parse_u8(next);
                    let Some(val) = res else { break };
                    val
                }),
                137u16 => Devlink::HealthReporterDumpTsNs({
                    let res = parse_u64(next);
                    let Some(val) = res else { break };
                    val
                }),
                138u16 => Devlink::NetnsFd({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                139u16 => Devlink::NetnsPid({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                140u16 => Devlink::NetnsId({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                141u16 => Devlink::HealthReporterAutoDump({
                    let res = parse_u8(next);
                    let Some(val) = res else { break };
                    val
                }),
                142u16 => Devlink::TrapPolicerId({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                143u16 => Devlink::TrapPolicerRate({
                    let res = parse_u64(next);
                    let Some(val) = res else { break };
                    val
                }),
                144u16 => Devlink::TrapPolicerBurst({
                    let res = parse_u64(next);
                    let Some(val) = res else { break };
                    val
                }),
                145u16 => Devlink::PortFunction({
                    let res = Some(IterableDlPortFunction::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                146u16 => Devlink::InfoBoardSerialNumber({
                    let res = CStr::from_bytes_with_nul(next).ok();
                    let Some(val) = res else { break };
                    val
                }),
                147u16 => Devlink::PortLanes({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                148u16 => Devlink::PortSplittable({
                    let res = parse_u8(next);
                    let Some(val) = res else { break };
                    val
                }),
                149u16 => Devlink::PortExternal({
                    let res = parse_u8(next);
                    let Some(val) = res else { break };
                    val
                }),
                150u16 => Devlink::PortControllerNumber({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                151u16 => Devlink::FlashUpdateStatusTimeout({
                    let res = parse_u64(next);
                    let Some(val) = res else { break };
                    val
                }),
                152u16 => Devlink::FlashUpdateOverwriteMask({
                    let res = BuiltinBitfield32::new_from_slice(next);
                    let Some(val) = res else { break };
                    val
                }),
                153u16 => Devlink::ReloadAction({
                    let res = parse_u8(next);
                    let Some(val) = res else { break };
                    val
                }),
                154u16 => Devlink::ReloadActionsPerformed({
                    let res = BuiltinBitfield32::new_from_slice(next);
                    let Some(val) = res else { break };
                    val
                }),
                155u16 => Devlink::ReloadLimits({
                    let res = BuiltinBitfield32::new_from_slice(next);
                    let Some(val) = res else { break };
                    val
                }),
                156u16 => Devlink::DevStats({
                    let res = Some(IterableDlDevStats::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                157u16 => Devlink::ReloadStats({
                    let res = Some(IterableDlReloadStats::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                158u16 => Devlink::ReloadStatsEntry({
                    let res = Some(IterableDlReloadStatsEntry::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                159u16 => Devlink::ReloadStatsLimit({
                    let res = parse_u8(next);
                    let Some(val) = res else { break };
                    val
                }),
                160u16 => Devlink::ReloadStatsValue({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                161u16 => Devlink::RemoteReloadStats({
                    let res = Some(IterableDlReloadStats::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                162u16 => Devlink::ReloadActionInfo({
                    let res = Some(IterableDlReloadActInfo::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                163u16 => Devlink::ReloadActionStats({
                    let res = Some(IterableDlReloadActStats::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                164u16 => Devlink::PortPciSfNumber({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                165u16 => Devlink::RateType({
                    let res = parse_u16(next);
                    let Some(val) = res else { break };
                    val
                }),
                166u16 => Devlink::RateTxShare({
                    let res = parse_u64(next);
                    let Some(val) = res else { break };
                    val
                }),
                167u16 => Devlink::RateTxMax({
                    let res = parse_u64(next);
                    let Some(val) = res else { break };
                    val
                }),
                168u16 => Devlink::RateNodeName({
                    let res = CStr::from_bytes_with_nul(next).ok();
                    let Some(val) = res else { break };
                    val
                }),
                169u16 => Devlink::RateParentNodeName({
                    let res = CStr::from_bytes_with_nul(next).ok();
                    let Some(val) = res else { break };
                    val
                }),
                170u16 => Devlink::RegionMaxSnapshots({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                171u16 => Devlink::LinecardIndex({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                172u16 => Devlink::LinecardState({
                    let res = parse_u8(next);
                    let Some(val) = res else { break };
                    val
                }),
                173u16 => Devlink::LinecardType({
                    let res = CStr::from_bytes_with_nul(next).ok();
                    let Some(val) = res else { break };
                    val
                }),
                174u16 => Devlink::LinecardSupportedTypes({
                    let res = Some(IterableDlLinecardSupportedTypes::with_loc(
                        next,
                        self.orig_loc,
                    ));
                    let Some(val) = res else { break };
                    val
                }),
                176u16 => Devlink::Selftests({
                    let res = Some(IterableDlSelftestId::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                177u16 => Devlink::RateTxPriority({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                178u16 => Devlink::RateTxWeight({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                179u16 => Devlink::RegionDirect(()),
                180u16 => Devlink::RateTcBws({
                    let res = Some(IterableDlRateTcBws::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                181u16 => Devlink::HealthReporterBurstPeriod({
                    let res = parse_u64(next);
                    let Some(val) = res else { break };
                    val
                }),
                183u16 => Devlink::ParamResetDefault(()),
                184u16 => Devlink::Index({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                185u16 => Devlink::ResourceScopeMask({
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
            "Devlink",
            r#type.and_then(|t| Devlink::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl<'a> std::fmt::Debug for IterableDevlink<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("Devlink");
        let mut iter = IterableDevlink::with_loc(&[], self.orig_loc);
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
                Devlink::BusName(val) => fmt.field("BusName", &val),
                Devlink::DevName(val) => fmt.field("DevName", &val),
                Devlink::PortIndex(val) => fmt.field("PortIndex", &val),
                Devlink::PortType(val) => {
                    fmt.field("PortType", &FormatEnum(val.into(), PortType::from_value))
                }
                Devlink::PortDesiredType(val) => fmt.field("PortDesiredType", &val),
                Devlink::PortNetdevIfindex(val) => fmt.field("PortNetdevIfindex", &val),
                Devlink::PortNetdevName(val) => fmt.field("PortNetdevName", &val),
                Devlink::PortIbdevName(val) => fmt.field("PortIbdevName", &val),
                Devlink::PortSplitCount(val) => fmt.field("PortSplitCount", &val),
                Devlink::PortSplitGroup(val) => fmt.field("PortSplitGroup", &val),
                Devlink::SbIndex(val) => fmt.field("SbIndex", &val),
                Devlink::SbSize(val) => fmt.field("SbSize", &val),
                Devlink::SbIngressPoolCount(val) => fmt.field("SbIngressPoolCount", &val),
                Devlink::SbEgressPoolCount(val) => fmt.field("SbEgressPoolCount", &val),
                Devlink::SbIngressTcCount(val) => fmt.field("SbIngressTcCount", &val),
                Devlink::SbEgressTcCount(val) => fmt.field("SbEgressTcCount", &val),
                Devlink::SbPoolIndex(val) => fmt.field("SbPoolIndex", &val),
                Devlink::SbPoolType(val) => fmt.field(
                    "SbPoolType",
                    &FormatEnum(val.into(), SbPoolType::from_value),
                ),
                Devlink::SbPoolSize(val) => fmt.field("SbPoolSize", &val),
                Devlink::SbPoolThresholdType(val) => fmt.field(
                    "SbPoolThresholdType",
                    &FormatEnum(val.into(), SbThresholdType::from_value),
                ),
                Devlink::SbThreshold(val) => fmt.field("SbThreshold", &val),
                Devlink::SbTcIndex(val) => fmt.field("SbTcIndex", &val),
                Devlink::SbOccCur(val) => fmt.field("SbOccCur", &val),
                Devlink::SbOccMax(val) => fmt.field("SbOccMax", &val),
                Devlink::EswitchMode(val) => fmt.field(
                    "EswitchMode",
                    &FormatEnum(val.into(), EswitchMode::from_value),
                ),
                Devlink::EswitchInlineMode(val) => fmt.field(
                    "EswitchInlineMode",
                    &FormatEnum(val.into(), EswitchInlineMode::from_value),
                ),
                Devlink::DpipeTables(val) => fmt.field("DpipeTables", &val),
                Devlink::DpipeTable(val) => fmt.field("DpipeTable", &val),
                Devlink::DpipeTableName(val) => fmt.field("DpipeTableName", &val),
                Devlink::DpipeTableSize(val) => fmt.field("DpipeTableSize", &val),
                Devlink::DpipeTableMatches(val) => fmt.field("DpipeTableMatches", &val),
                Devlink::DpipeTableActions(val) => fmt.field("DpipeTableActions", &val),
                Devlink::DpipeTableCountersEnabled(val) => {
                    fmt.field("DpipeTableCountersEnabled", &val)
                }
                Devlink::DpipeEntries(val) => fmt.field("DpipeEntries", &val),
                Devlink::DpipeEntry(val) => fmt.field("DpipeEntry", &val),
                Devlink::DpipeEntryIndex(val) => fmt.field("DpipeEntryIndex", &val),
                Devlink::DpipeEntryMatchValues(val) => fmt.field("DpipeEntryMatchValues", &val),
                Devlink::DpipeEntryActionValues(val) => fmt.field("DpipeEntryActionValues", &val),
                Devlink::DpipeEntryCounter(val) => fmt.field("DpipeEntryCounter", &val),
                Devlink::DpipeMatch(val) => fmt.field("DpipeMatch", &val),
                Devlink::DpipeMatchValue(val) => fmt.field("DpipeMatchValue", &val),
                Devlink::DpipeMatchType(val) => fmt.field(
                    "DpipeMatchType",
                    &FormatEnum(val.into(), DpipeMatchType::from_value),
                ),
                Devlink::DpipeAction(val) => fmt.field("DpipeAction", &val),
                Devlink::DpipeActionValue(val) => fmt.field("DpipeActionValue", &val),
                Devlink::DpipeActionType(val) => fmt.field(
                    "DpipeActionType",
                    &FormatEnum(val.into(), DpipeActionType::from_value),
                ),
                Devlink::DpipeValue(val) => fmt.field("DpipeValue", &FormatHexdump(val)),
                Devlink::DpipeValueMask(val) => fmt.field("DpipeValueMask", &FormatHexdump(val)),
                Devlink::DpipeValueMapping(val) => fmt.field("DpipeValueMapping", &val),
                Devlink::DpipeHeaders(val) => fmt.field("DpipeHeaders", &val),
                Devlink::DpipeHeader(val) => fmt.field("DpipeHeader", &val),
                Devlink::DpipeHeaderName(val) => fmt.field("DpipeHeaderName", &val),
                Devlink::DpipeHeaderId(val) => fmt.field(
                    "DpipeHeaderId",
                    &FormatEnum(val.into(), DpipeHeaderId::from_value),
                ),
                Devlink::DpipeHeaderFields(val) => fmt.field("DpipeHeaderFields", &val),
                Devlink::DpipeHeaderGlobal(val) => fmt.field("DpipeHeaderGlobal", &val),
                Devlink::DpipeHeaderIndex(val) => fmt.field("DpipeHeaderIndex", &val),
                Devlink::DpipeField(val) => fmt.field("DpipeField", &val),
                Devlink::DpipeFieldName(val) => fmt.field("DpipeFieldName", &val),
                Devlink::DpipeFieldId(val) => fmt.field("DpipeFieldId", &val),
                Devlink::DpipeFieldBitwidth(val) => fmt.field("DpipeFieldBitwidth", &val),
                Devlink::DpipeFieldMappingType(val) => fmt.field(
                    "DpipeFieldMappingType",
                    &FormatEnum(val.into(), DpipeFieldMappingType::from_value),
                ),
                Devlink::Pad(val) => fmt.field("Pad", &val),
                Devlink::EswitchEncapMode(val) => fmt.field(
                    "EswitchEncapMode",
                    &FormatEnum(val.into(), EswitchEncapMode::from_value),
                ),
                Devlink::ResourceList(val) => fmt.field("ResourceList", &val),
                Devlink::Resource(val) => fmt.field("Resource", &val),
                Devlink::ResourceName(val) => fmt.field("ResourceName", &val),
                Devlink::ResourceId(val) => fmt.field("ResourceId", &val),
                Devlink::ResourceSize(val) => fmt.field("ResourceSize", &val),
                Devlink::ResourceSizeNew(val) => fmt.field("ResourceSizeNew", &val),
                Devlink::ResourceSizeValid(val) => fmt.field("ResourceSizeValid", &val),
                Devlink::ResourceSizeMin(val) => fmt.field("ResourceSizeMin", &val),
                Devlink::ResourceSizeMax(val) => fmt.field("ResourceSizeMax", &val),
                Devlink::ResourceSizeGran(val) => fmt.field("ResourceSizeGran", &val),
                Devlink::ResourceUnit(val) => fmt.field(
                    "ResourceUnit",
                    &FormatEnum(val.into(), ResourceUnit::from_value),
                ),
                Devlink::ResourceOcc(val) => fmt.field("ResourceOcc", &val),
                Devlink::DpipeTableResourceId(val) => fmt.field("DpipeTableResourceId", &val),
                Devlink::DpipeTableResourceUnits(val) => fmt.field("DpipeTableResourceUnits", &val),
                Devlink::PortFlavour(val) => fmt.field(
                    "PortFlavour",
                    &FormatEnum(val.into(), PortFlavour::from_value),
                ),
                Devlink::PortNumber(val) => fmt.field("PortNumber", &val),
                Devlink::PortSplitSubportNumber(val) => fmt.field("PortSplitSubportNumber", &val),
                Devlink::Param(val) => fmt.field("Param", &val),
                Devlink::ParamName(val) => fmt.field("ParamName", &val),
                Devlink::ParamGeneric(val) => fmt.field("ParamGeneric", &val),
                Devlink::ParamType(val) => fmt.field(
                    "ParamType",
                    &FormatEnum(val.into(), VarAttrType::from_value),
                ),
                Devlink::ParamValueCmode(val) => fmt.field(
                    "ParamValueCmode",
                    &FormatEnum(val.into(), ParamCmode::from_value),
                ),
                Devlink::RegionName(val) => fmt.field("RegionName", &val),
                Devlink::RegionSize(val) => fmt.field("RegionSize", &val),
                Devlink::RegionSnapshots(val) => fmt.field("RegionSnapshots", &val),
                Devlink::RegionSnapshot(val) => fmt.field("RegionSnapshot", &val),
                Devlink::RegionSnapshotId(val) => fmt.field("RegionSnapshotId", &val),
                Devlink::RegionChunks(val) => fmt.field("RegionChunks", &val),
                Devlink::RegionChunk(val) => fmt.field("RegionChunk", &val),
                Devlink::RegionChunkData(val) => fmt.field("RegionChunkData", &FormatHexdump(val)),
                Devlink::RegionChunkAddr(val) => fmt.field("RegionChunkAddr", &val),
                Devlink::RegionChunkLen(val) => fmt.field("RegionChunkLen", &val),
                Devlink::InfoDriverName(val) => fmt.field("InfoDriverName", &val),
                Devlink::InfoSerialNumber(val) => fmt.field("InfoSerialNumber", &val),
                Devlink::InfoVersionFixed(val) => fmt.field("InfoVersionFixed", &val),
                Devlink::InfoVersionRunning(val) => fmt.field("InfoVersionRunning", &val),
                Devlink::InfoVersionStored(val) => fmt.field("InfoVersionStored", &val),
                Devlink::InfoVersionName(val) => fmt.field("InfoVersionName", &val),
                Devlink::InfoVersionValue(val) => fmt.field("InfoVersionValue", &val),
                Devlink::SbPoolCellSize(val) => fmt.field("SbPoolCellSize", &val),
                Devlink::Fmsg(val) => fmt.field("Fmsg", &val),
                Devlink::FmsgObjNestStart(val) => fmt.field("FmsgObjNestStart", &val),
                Devlink::FmsgPairNestStart(val) => fmt.field("FmsgPairNestStart", &val),
                Devlink::FmsgArrNestStart(val) => fmt.field("FmsgArrNestStart", &val),
                Devlink::FmsgNestEnd(val) => fmt.field("FmsgNestEnd", &val),
                Devlink::FmsgObjName(val) => fmt.field("FmsgObjName", &val),
                Devlink::FmsgObjValueType(val) => fmt.field(
                    "FmsgObjValueType",
                    &FormatEnum(val.into(), VarAttrType::from_value),
                ),
                Devlink::HealthReporter(val) => fmt.field("HealthReporter", &val),
                Devlink::HealthReporterName(val) => fmt.field("HealthReporterName", &val),
                Devlink::HealthReporterState(val) => fmt.field("HealthReporterState", &val),
                Devlink::HealthReporterErrCount(val) => fmt.field("HealthReporterErrCount", &val),
                Devlink::HealthReporterRecoverCount(val) => {
                    fmt.field("HealthReporterRecoverCount", &val)
                }
                Devlink::HealthReporterDumpTs(val) => fmt.field("HealthReporterDumpTs", &val),
                Devlink::HealthReporterGracefulPeriod(val) => {
                    fmt.field("HealthReporterGracefulPeriod", &val)
                }
                Devlink::HealthReporterAutoRecover(val) => {
                    fmt.field("HealthReporterAutoRecover", &val)
                }
                Devlink::FlashUpdateFileName(val) => fmt.field("FlashUpdateFileName", &val),
                Devlink::FlashUpdateComponent(val) => fmt.field("FlashUpdateComponent", &val),
                Devlink::FlashUpdateStatusMsg(val) => fmt.field("FlashUpdateStatusMsg", &val),
                Devlink::FlashUpdateStatusDone(val) => fmt.field("FlashUpdateStatusDone", &val),
                Devlink::FlashUpdateStatusTotal(val) => fmt.field("FlashUpdateStatusTotal", &val),
                Devlink::PortPciPfNumber(val) => fmt.field("PortPciPfNumber", &val),
                Devlink::PortPciVfNumber(val) => fmt.field("PortPciVfNumber", &val),
                Devlink::Stats(val) => fmt.field("Stats", &val),
                Devlink::TrapName(val) => fmt.field("TrapName", &val),
                Devlink::TrapAction(val) => fmt.field(
                    "TrapAction",
                    &FormatEnum(val.into(), TrapAction::from_value),
                ),
                Devlink::TrapType(val) => {
                    fmt.field("TrapType", &FormatEnum(val.into(), TrapType::from_value))
                }
                Devlink::TrapGeneric(val) => fmt.field("TrapGeneric", &val),
                Devlink::TrapMetadata(val) => fmt.field("TrapMetadata", &val),
                Devlink::TrapGroupName(val) => fmt.field("TrapGroupName", &val),
                Devlink::ReloadFailed(val) => fmt.field("ReloadFailed", &val),
                Devlink::HealthReporterDumpTsNs(val) => fmt.field("HealthReporterDumpTsNs", &val),
                Devlink::NetnsFd(val) => fmt.field("NetnsFd", &val),
                Devlink::NetnsPid(val) => fmt.field("NetnsPid", &val),
                Devlink::NetnsId(val) => fmt.field("NetnsId", &val),
                Devlink::HealthReporterAutoDump(val) => fmt.field("HealthReporterAutoDump", &val),
                Devlink::TrapPolicerId(val) => fmt.field("TrapPolicerId", &val),
                Devlink::TrapPolicerRate(val) => fmt.field("TrapPolicerRate", &val),
                Devlink::TrapPolicerBurst(val) => fmt.field("TrapPolicerBurst", &val),
                Devlink::PortFunction(val) => fmt.field("PortFunction", &val),
                Devlink::InfoBoardSerialNumber(val) => fmt.field("InfoBoardSerialNumber", &val),
                Devlink::PortLanes(val) => fmt.field("PortLanes", &val),
                Devlink::PortSplittable(val) => fmt.field("PortSplittable", &val),
                Devlink::PortExternal(val) => fmt.field("PortExternal", &val),
                Devlink::PortControllerNumber(val) => fmt.field("PortControllerNumber", &val),
                Devlink::FlashUpdateStatusTimeout(val) => {
                    fmt.field("FlashUpdateStatusTimeout", &val)
                }
                Devlink::FlashUpdateOverwriteMask(val) => fmt.field(
                    "FlashUpdateOverwriteMask",
                    &FormatFlags(val.value.into(), |val| {
                        FlashOverwrite::from_value(val.trailing_zeros().into())
                    }),
                ),
                Devlink::ReloadAction(val) => fmt.field(
                    "ReloadAction",
                    &FormatEnum(val.into(), ReloadAction::from_value),
                ),
                Devlink::ReloadActionsPerformed(val) => fmt.field(
                    "ReloadActionsPerformed",
                    &FormatFlags(val.value.into(), |val| {
                        ReloadAction::from_value(val.trailing_zeros().into())
                    }),
                ),
                Devlink::ReloadLimits(val) => fmt.field(
                    "ReloadLimits",
                    &FormatFlags(val.value.into(), |val| {
                        ReloadAction::from_value(val.trailing_zeros().into())
                    }),
                ),
                Devlink::DevStats(val) => fmt.field("DevStats", &val),
                Devlink::ReloadStats(val) => fmt.field("ReloadStats", &val),
                Devlink::ReloadStatsEntry(val) => fmt.field("ReloadStatsEntry", &val),
                Devlink::ReloadStatsLimit(val) => fmt.field("ReloadStatsLimit", &val),
                Devlink::ReloadStatsValue(val) => fmt.field("ReloadStatsValue", &val),
                Devlink::RemoteReloadStats(val) => fmt.field("RemoteReloadStats", &val),
                Devlink::ReloadActionInfo(val) => fmt.field("ReloadActionInfo", &val),
                Devlink::ReloadActionStats(val) => fmt.field("ReloadActionStats", &val),
                Devlink::PortPciSfNumber(val) => fmt.field("PortPciSfNumber", &val),
                Devlink::RateType(val) => {
                    fmt.field("RateType", &FormatEnum(val.into(), RateType::from_value))
                }
                Devlink::RateTxShare(val) => fmt.field("RateTxShare", &val),
                Devlink::RateTxMax(val) => fmt.field("RateTxMax", &val),
                Devlink::RateNodeName(val) => fmt.field("RateNodeName", &val),
                Devlink::RateParentNodeName(val) => fmt.field("RateParentNodeName", &val),
                Devlink::RegionMaxSnapshots(val) => fmt.field("RegionMaxSnapshots", &val),
                Devlink::LinecardIndex(val) => fmt.field("LinecardIndex", &val),
                Devlink::LinecardState(val) => fmt.field("LinecardState", &val),
                Devlink::LinecardType(val) => fmt.field("LinecardType", &val),
                Devlink::LinecardSupportedTypes(val) => fmt.field("LinecardSupportedTypes", &val),
                Devlink::Selftests(val) => fmt.field("Selftests", &val),
                Devlink::RateTxPriority(val) => fmt.field("RateTxPriority", &val),
                Devlink::RateTxWeight(val) => fmt.field("RateTxWeight", &val),
                Devlink::RegionDirect(val) => fmt.field("RegionDirect", &val),
                Devlink::RateTcBws(val) => fmt.field("RateTcBws", &val),
                Devlink::HealthReporterBurstPeriod(val) => {
                    fmt.field("HealthReporterBurstPeriod", &val)
                }
                Devlink::ParamResetDefault(val) => fmt.field("ParamResetDefault", &val),
                Devlink::Index(val) => fmt.field("Index", &val),
                Devlink::ResourceScopeMask(val) => fmt.field(
                    "ResourceScopeMask",
                    &FormatFlags(val.into(), |val| {
                        ResourceScope::from_value(val.trailing_zeros().into())
                    }),
                ),
            };
        }
        fmt.finish()
    }
}
impl IterableDevlink<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("Devlink", offset));
            return (stack, missing_type.and_then(|t| Devlink::attr_from_type(t)));
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
                Devlink::BusName(val) => {
                    if last_off == offset {
                        stack.push(("BusName", last_off));
                        break;
                    }
                }
                Devlink::DevName(val) => {
                    if last_off == offset {
                        stack.push(("DevName", last_off));
                        break;
                    }
                }
                Devlink::PortIndex(val) => {
                    if last_off == offset {
                        stack.push(("PortIndex", last_off));
                        break;
                    }
                }
                Devlink::PortType(val) => {
                    if last_off == offset {
                        stack.push(("PortType", last_off));
                        break;
                    }
                }
                Devlink::PortDesiredType(val) => {
                    if last_off == offset {
                        stack.push(("PortDesiredType", last_off));
                        break;
                    }
                }
                Devlink::PortNetdevIfindex(val) => {
                    if last_off == offset {
                        stack.push(("PortNetdevIfindex", last_off));
                        break;
                    }
                }
                Devlink::PortNetdevName(val) => {
                    if last_off == offset {
                        stack.push(("PortNetdevName", last_off));
                        break;
                    }
                }
                Devlink::PortIbdevName(val) => {
                    if last_off == offset {
                        stack.push(("PortIbdevName", last_off));
                        break;
                    }
                }
                Devlink::PortSplitCount(val) => {
                    if last_off == offset {
                        stack.push(("PortSplitCount", last_off));
                        break;
                    }
                }
                Devlink::PortSplitGroup(val) => {
                    if last_off == offset {
                        stack.push(("PortSplitGroup", last_off));
                        break;
                    }
                }
                Devlink::SbIndex(val) => {
                    if last_off == offset {
                        stack.push(("SbIndex", last_off));
                        break;
                    }
                }
                Devlink::SbSize(val) => {
                    if last_off == offset {
                        stack.push(("SbSize", last_off));
                        break;
                    }
                }
                Devlink::SbIngressPoolCount(val) => {
                    if last_off == offset {
                        stack.push(("SbIngressPoolCount", last_off));
                        break;
                    }
                }
                Devlink::SbEgressPoolCount(val) => {
                    if last_off == offset {
                        stack.push(("SbEgressPoolCount", last_off));
                        break;
                    }
                }
                Devlink::SbIngressTcCount(val) => {
                    if last_off == offset {
                        stack.push(("SbIngressTcCount", last_off));
                        break;
                    }
                }
                Devlink::SbEgressTcCount(val) => {
                    if last_off == offset {
                        stack.push(("SbEgressTcCount", last_off));
                        break;
                    }
                }
                Devlink::SbPoolIndex(val) => {
                    if last_off == offset {
                        stack.push(("SbPoolIndex", last_off));
                        break;
                    }
                }
                Devlink::SbPoolType(val) => {
                    if last_off == offset {
                        stack.push(("SbPoolType", last_off));
                        break;
                    }
                }
                Devlink::SbPoolSize(val) => {
                    if last_off == offset {
                        stack.push(("SbPoolSize", last_off));
                        break;
                    }
                }
                Devlink::SbPoolThresholdType(val) => {
                    if last_off == offset {
                        stack.push(("SbPoolThresholdType", last_off));
                        break;
                    }
                }
                Devlink::SbThreshold(val) => {
                    if last_off == offset {
                        stack.push(("SbThreshold", last_off));
                        break;
                    }
                }
                Devlink::SbTcIndex(val) => {
                    if last_off == offset {
                        stack.push(("SbTcIndex", last_off));
                        break;
                    }
                }
                Devlink::SbOccCur(val) => {
                    if last_off == offset {
                        stack.push(("SbOccCur", last_off));
                        break;
                    }
                }
                Devlink::SbOccMax(val) => {
                    if last_off == offset {
                        stack.push(("SbOccMax", last_off));
                        break;
                    }
                }
                Devlink::EswitchMode(val) => {
                    if last_off == offset {
                        stack.push(("EswitchMode", last_off));
                        break;
                    }
                }
                Devlink::EswitchInlineMode(val) => {
                    if last_off == offset {
                        stack.push(("EswitchInlineMode", last_off));
                        break;
                    }
                }
                Devlink::DpipeTables(val) => {
                    (stack, missing) = val.lookup_attr(offset, missing_type);
                    if !stack.is_empty() {
                        break;
                    }
                }
                Devlink::DpipeTable(val) => {
                    (stack, missing) = val.lookup_attr(offset, missing_type);
                    if !stack.is_empty() {
                        break;
                    }
                }
                Devlink::DpipeTableName(val) => {
                    if last_off == offset {
                        stack.push(("DpipeTableName", last_off));
                        break;
                    }
                }
                Devlink::DpipeTableSize(val) => {
                    if last_off == offset {
                        stack.push(("DpipeTableSize", last_off));
                        break;
                    }
                }
                Devlink::DpipeTableMatches(val) => {
                    (stack, missing) = val.lookup_attr(offset, missing_type);
                    if !stack.is_empty() {
                        break;
                    }
                }
                Devlink::DpipeTableActions(val) => {
                    (stack, missing) = val.lookup_attr(offset, missing_type);
                    if !stack.is_empty() {
                        break;
                    }
                }
                Devlink::DpipeTableCountersEnabled(val) => {
                    if last_off == offset {
                        stack.push(("DpipeTableCountersEnabled", last_off));
                        break;
                    }
                }
                Devlink::DpipeEntries(val) => {
                    (stack, missing) = val.lookup_attr(offset, missing_type);
                    if !stack.is_empty() {
                        break;
                    }
                }
                Devlink::DpipeEntry(val) => {
                    (stack, missing) = val.lookup_attr(offset, missing_type);
                    if !stack.is_empty() {
                        break;
                    }
                }
                Devlink::DpipeEntryIndex(val) => {
                    if last_off == offset {
                        stack.push(("DpipeEntryIndex", last_off));
                        break;
                    }
                }
                Devlink::DpipeEntryMatchValues(val) => {
                    (stack, missing) = val.lookup_attr(offset, missing_type);
                    if !stack.is_empty() {
                        break;
                    }
                }
                Devlink::DpipeEntryActionValues(val) => {
                    (stack, missing) = val.lookup_attr(offset, missing_type);
                    if !stack.is_empty() {
                        break;
                    }
                }
                Devlink::DpipeEntryCounter(val) => {
                    if last_off == offset {
                        stack.push(("DpipeEntryCounter", last_off));
                        break;
                    }
                }
                Devlink::DpipeMatch(val) => {
                    (stack, missing) = val.lookup_attr(offset, missing_type);
                    if !stack.is_empty() {
                        break;
                    }
                }
                Devlink::DpipeMatchValue(val) => {
                    (stack, missing) = val.lookup_attr(offset, missing_type);
                    if !stack.is_empty() {
                        break;
                    }
                }
                Devlink::DpipeMatchType(val) => {
                    if last_off == offset {
                        stack.push(("DpipeMatchType", last_off));
                        break;
                    }
                }
                Devlink::DpipeAction(val) => {
                    (stack, missing) = val.lookup_attr(offset, missing_type);
                    if !stack.is_empty() {
                        break;
                    }
                }
                Devlink::DpipeActionValue(val) => {
                    (stack, missing) = val.lookup_attr(offset, missing_type);
                    if !stack.is_empty() {
                        break;
                    }
                }
                Devlink::DpipeActionType(val) => {
                    if last_off == offset {
                        stack.push(("DpipeActionType", last_off));
                        break;
                    }
                }
                Devlink::DpipeValue(val) => {
                    if last_off == offset {
                        stack.push(("DpipeValue", last_off));
                        break;
                    }
                }
                Devlink::DpipeValueMask(val) => {
                    if last_off == offset {
                        stack.push(("DpipeValueMask", last_off));
                        break;
                    }
                }
                Devlink::DpipeValueMapping(val) => {
                    if last_off == offset {
                        stack.push(("DpipeValueMapping", last_off));
                        break;
                    }
                }
                Devlink::DpipeHeaders(val) => {
                    (stack, missing) = val.lookup_attr(offset, missing_type);
                    if !stack.is_empty() {
                        break;
                    }
                }
                Devlink::DpipeHeader(val) => {
                    (stack, missing) = val.lookup_attr(offset, missing_type);
                    if !stack.is_empty() {
                        break;
                    }
                }
                Devlink::DpipeHeaderName(val) => {
                    if last_off == offset {
                        stack.push(("DpipeHeaderName", last_off));
                        break;
                    }
                }
                Devlink::DpipeHeaderId(val) => {
                    if last_off == offset {
                        stack.push(("DpipeHeaderId", last_off));
                        break;
                    }
                }
                Devlink::DpipeHeaderFields(val) => {
                    (stack, missing) = val.lookup_attr(offset, missing_type);
                    if !stack.is_empty() {
                        break;
                    }
                }
                Devlink::DpipeHeaderGlobal(val) => {
                    if last_off == offset {
                        stack.push(("DpipeHeaderGlobal", last_off));
                        break;
                    }
                }
                Devlink::DpipeHeaderIndex(val) => {
                    if last_off == offset {
                        stack.push(("DpipeHeaderIndex", last_off));
                        break;
                    }
                }
                Devlink::DpipeField(val) => {
                    (stack, missing) = val.lookup_attr(offset, missing_type);
                    if !stack.is_empty() {
                        break;
                    }
                }
                Devlink::DpipeFieldName(val) => {
                    if last_off == offset {
                        stack.push(("DpipeFieldName", last_off));
                        break;
                    }
                }
                Devlink::DpipeFieldId(val) => {
                    if last_off == offset {
                        stack.push(("DpipeFieldId", last_off));
                        break;
                    }
                }
                Devlink::DpipeFieldBitwidth(val) => {
                    if last_off == offset {
                        stack.push(("DpipeFieldBitwidth", last_off));
                        break;
                    }
                }
                Devlink::DpipeFieldMappingType(val) => {
                    if last_off == offset {
                        stack.push(("DpipeFieldMappingType", last_off));
                        break;
                    }
                }
                Devlink::Pad(val) => {
                    if last_off == offset {
                        stack.push(("Pad", last_off));
                        break;
                    }
                }
                Devlink::EswitchEncapMode(val) => {
                    if last_off == offset {
                        stack.push(("EswitchEncapMode", last_off));
                        break;
                    }
                }
                Devlink::ResourceList(val) => {
                    (stack, missing) = val.lookup_attr(offset, missing_type);
                    if !stack.is_empty() {
                        break;
                    }
                }
                Devlink::Resource(val) => {
                    (stack, missing) = val.lookup_attr(offset, missing_type);
                    if !stack.is_empty() {
                        break;
                    }
                }
                Devlink::ResourceName(val) => {
                    if last_off == offset {
                        stack.push(("ResourceName", last_off));
                        break;
                    }
                }
                Devlink::ResourceId(val) => {
                    if last_off == offset {
                        stack.push(("ResourceId", last_off));
                        break;
                    }
                }
                Devlink::ResourceSize(val) => {
                    if last_off == offset {
                        stack.push(("ResourceSize", last_off));
                        break;
                    }
                }
                Devlink::ResourceSizeNew(val) => {
                    if last_off == offset {
                        stack.push(("ResourceSizeNew", last_off));
                        break;
                    }
                }
                Devlink::ResourceSizeValid(val) => {
                    if last_off == offset {
                        stack.push(("ResourceSizeValid", last_off));
                        break;
                    }
                }
                Devlink::ResourceSizeMin(val) => {
                    if last_off == offset {
                        stack.push(("ResourceSizeMin", last_off));
                        break;
                    }
                }
                Devlink::ResourceSizeMax(val) => {
                    if last_off == offset {
                        stack.push(("ResourceSizeMax", last_off));
                        break;
                    }
                }
                Devlink::ResourceSizeGran(val) => {
                    if last_off == offset {
                        stack.push(("ResourceSizeGran", last_off));
                        break;
                    }
                }
                Devlink::ResourceUnit(val) => {
                    if last_off == offset {
                        stack.push(("ResourceUnit", last_off));
                        break;
                    }
                }
                Devlink::ResourceOcc(val) => {
                    if last_off == offset {
                        stack.push(("ResourceOcc", last_off));
                        break;
                    }
                }
                Devlink::DpipeTableResourceId(val) => {
                    if last_off == offset {
                        stack.push(("DpipeTableResourceId", last_off));
                        break;
                    }
                }
                Devlink::DpipeTableResourceUnits(val) => {
                    if last_off == offset {
                        stack.push(("DpipeTableResourceUnits", last_off));
                        break;
                    }
                }
                Devlink::PortFlavour(val) => {
                    if last_off == offset {
                        stack.push(("PortFlavour", last_off));
                        break;
                    }
                }
                Devlink::PortNumber(val) => {
                    if last_off == offset {
                        stack.push(("PortNumber", last_off));
                        break;
                    }
                }
                Devlink::PortSplitSubportNumber(val) => {
                    if last_off == offset {
                        stack.push(("PortSplitSubportNumber", last_off));
                        break;
                    }
                }
                Devlink::Param(val) => {
                    (stack, missing) = val.lookup_attr(offset, missing_type);
                    if !stack.is_empty() {
                        break;
                    }
                }
                Devlink::ParamName(val) => {
                    if last_off == offset {
                        stack.push(("ParamName", last_off));
                        break;
                    }
                }
                Devlink::ParamGeneric(val) => {
                    if last_off == offset {
                        stack.push(("ParamGeneric", last_off));
                        break;
                    }
                }
                Devlink::ParamType(val) => {
                    if last_off == offset {
                        stack.push(("ParamType", last_off));
                        break;
                    }
                }
                Devlink::ParamValueCmode(val) => {
                    if last_off == offset {
                        stack.push(("ParamValueCmode", last_off));
                        break;
                    }
                }
                Devlink::RegionName(val) => {
                    if last_off == offset {
                        stack.push(("RegionName", last_off));
                        break;
                    }
                }
                Devlink::RegionSize(val) => {
                    if last_off == offset {
                        stack.push(("RegionSize", last_off));
                        break;
                    }
                }
                Devlink::RegionSnapshots(val) => {
                    (stack, missing) = val.lookup_attr(offset, missing_type);
                    if !stack.is_empty() {
                        break;
                    }
                }
                Devlink::RegionSnapshot(val) => {
                    (stack, missing) = val.lookup_attr(offset, missing_type);
                    if !stack.is_empty() {
                        break;
                    }
                }
                Devlink::RegionSnapshotId(val) => {
                    if last_off == offset {
                        stack.push(("RegionSnapshotId", last_off));
                        break;
                    }
                }
                Devlink::RegionChunks(val) => {
                    (stack, missing) = val.lookup_attr(offset, missing_type);
                    if !stack.is_empty() {
                        break;
                    }
                }
                Devlink::RegionChunk(val) => {
                    (stack, missing) = val.lookup_attr(offset, missing_type);
                    if !stack.is_empty() {
                        break;
                    }
                }
                Devlink::RegionChunkData(val) => {
                    if last_off == offset {
                        stack.push(("RegionChunkData", last_off));
                        break;
                    }
                }
                Devlink::RegionChunkAddr(val) => {
                    if last_off == offset {
                        stack.push(("RegionChunkAddr", last_off));
                        break;
                    }
                }
                Devlink::RegionChunkLen(val) => {
                    if last_off == offset {
                        stack.push(("RegionChunkLen", last_off));
                        break;
                    }
                }
                Devlink::InfoDriverName(val) => {
                    if last_off == offset {
                        stack.push(("InfoDriverName", last_off));
                        break;
                    }
                }
                Devlink::InfoSerialNumber(val) => {
                    if last_off == offset {
                        stack.push(("InfoSerialNumber", last_off));
                        break;
                    }
                }
                Devlink::InfoVersionFixed(val) => {
                    (stack, missing) = val.lookup_attr(offset, missing_type);
                    if !stack.is_empty() {
                        break;
                    }
                }
                Devlink::InfoVersionRunning(val) => {
                    (stack, missing) = val.lookup_attr(offset, missing_type);
                    if !stack.is_empty() {
                        break;
                    }
                }
                Devlink::InfoVersionStored(val) => {
                    (stack, missing) = val.lookup_attr(offset, missing_type);
                    if !stack.is_empty() {
                        break;
                    }
                }
                Devlink::InfoVersionName(val) => {
                    if last_off == offset {
                        stack.push(("InfoVersionName", last_off));
                        break;
                    }
                }
                Devlink::InfoVersionValue(val) => {
                    if last_off == offset {
                        stack.push(("InfoVersionValue", last_off));
                        break;
                    }
                }
                Devlink::SbPoolCellSize(val) => {
                    if last_off == offset {
                        stack.push(("SbPoolCellSize", last_off));
                        break;
                    }
                }
                Devlink::Fmsg(val) => {
                    (stack, missing) = val.lookup_attr(offset, missing_type);
                    if !stack.is_empty() {
                        break;
                    }
                }
                Devlink::FmsgObjNestStart(val) => {
                    if last_off == offset {
                        stack.push(("FmsgObjNestStart", last_off));
                        break;
                    }
                }
                Devlink::FmsgPairNestStart(val) => {
                    if last_off == offset {
                        stack.push(("FmsgPairNestStart", last_off));
                        break;
                    }
                }
                Devlink::FmsgArrNestStart(val) => {
                    if last_off == offset {
                        stack.push(("FmsgArrNestStart", last_off));
                        break;
                    }
                }
                Devlink::FmsgNestEnd(val) => {
                    if last_off == offset {
                        stack.push(("FmsgNestEnd", last_off));
                        break;
                    }
                }
                Devlink::FmsgObjName(val) => {
                    if last_off == offset {
                        stack.push(("FmsgObjName", last_off));
                        break;
                    }
                }
                Devlink::FmsgObjValueType(val) => {
                    if last_off == offset {
                        stack.push(("FmsgObjValueType", last_off));
                        break;
                    }
                }
                Devlink::HealthReporter(val) => {
                    (stack, missing) = val.lookup_attr(offset, missing_type);
                    if !stack.is_empty() {
                        break;
                    }
                }
                Devlink::HealthReporterName(val) => {
                    if last_off == offset {
                        stack.push(("HealthReporterName", last_off));
                        break;
                    }
                }
                Devlink::HealthReporterState(val) => {
                    if last_off == offset {
                        stack.push(("HealthReporterState", last_off));
                        break;
                    }
                }
                Devlink::HealthReporterErrCount(val) => {
                    if last_off == offset {
                        stack.push(("HealthReporterErrCount", last_off));
                        break;
                    }
                }
                Devlink::HealthReporterRecoverCount(val) => {
                    if last_off == offset {
                        stack.push(("HealthReporterRecoverCount", last_off));
                        break;
                    }
                }
                Devlink::HealthReporterDumpTs(val) => {
                    if last_off == offset {
                        stack.push(("HealthReporterDumpTs", last_off));
                        break;
                    }
                }
                Devlink::HealthReporterGracefulPeriod(val) => {
                    if last_off == offset {
                        stack.push(("HealthReporterGracefulPeriod", last_off));
                        break;
                    }
                }
                Devlink::HealthReporterAutoRecover(val) => {
                    if last_off == offset {
                        stack.push(("HealthReporterAutoRecover", last_off));
                        break;
                    }
                }
                Devlink::FlashUpdateFileName(val) => {
                    if last_off == offset {
                        stack.push(("FlashUpdateFileName", last_off));
                        break;
                    }
                }
                Devlink::FlashUpdateComponent(val) => {
                    if last_off == offset {
                        stack.push(("FlashUpdateComponent", last_off));
                        break;
                    }
                }
                Devlink::FlashUpdateStatusMsg(val) => {
                    if last_off == offset {
                        stack.push(("FlashUpdateStatusMsg", last_off));
                        break;
                    }
                }
                Devlink::FlashUpdateStatusDone(val) => {
                    if last_off == offset {
                        stack.push(("FlashUpdateStatusDone", last_off));
                        break;
                    }
                }
                Devlink::FlashUpdateStatusTotal(val) => {
                    if last_off == offset {
                        stack.push(("FlashUpdateStatusTotal", last_off));
                        break;
                    }
                }
                Devlink::PortPciPfNumber(val) => {
                    if last_off == offset {
                        stack.push(("PortPciPfNumber", last_off));
                        break;
                    }
                }
                Devlink::PortPciVfNumber(val) => {
                    if last_off == offset {
                        stack.push(("PortPciVfNumber", last_off));
                        break;
                    }
                }
                Devlink::Stats(val) => {
                    (stack, missing) = val.lookup_attr(offset, missing_type);
                    if !stack.is_empty() {
                        break;
                    }
                }
                Devlink::TrapName(val) => {
                    if last_off == offset {
                        stack.push(("TrapName", last_off));
                        break;
                    }
                }
                Devlink::TrapAction(val) => {
                    if last_off == offset {
                        stack.push(("TrapAction", last_off));
                        break;
                    }
                }
                Devlink::TrapType(val) => {
                    if last_off == offset {
                        stack.push(("TrapType", last_off));
                        break;
                    }
                }
                Devlink::TrapGeneric(val) => {
                    if last_off == offset {
                        stack.push(("TrapGeneric", last_off));
                        break;
                    }
                }
                Devlink::TrapMetadata(val) => {
                    (stack, missing) = val.lookup_attr(offset, missing_type);
                    if !stack.is_empty() {
                        break;
                    }
                }
                Devlink::TrapGroupName(val) => {
                    if last_off == offset {
                        stack.push(("TrapGroupName", last_off));
                        break;
                    }
                }
                Devlink::ReloadFailed(val) => {
                    if last_off == offset {
                        stack.push(("ReloadFailed", last_off));
                        break;
                    }
                }
                Devlink::HealthReporterDumpTsNs(val) => {
                    if last_off == offset {
                        stack.push(("HealthReporterDumpTsNs", last_off));
                        break;
                    }
                }
                Devlink::NetnsFd(val) => {
                    if last_off == offset {
                        stack.push(("NetnsFd", last_off));
                        break;
                    }
                }
                Devlink::NetnsPid(val) => {
                    if last_off == offset {
                        stack.push(("NetnsPid", last_off));
                        break;
                    }
                }
                Devlink::NetnsId(val) => {
                    if last_off == offset {
                        stack.push(("NetnsId", last_off));
                        break;
                    }
                }
                Devlink::HealthReporterAutoDump(val) => {
                    if last_off == offset {
                        stack.push(("HealthReporterAutoDump", last_off));
                        break;
                    }
                }
                Devlink::TrapPolicerId(val) => {
                    if last_off == offset {
                        stack.push(("TrapPolicerId", last_off));
                        break;
                    }
                }
                Devlink::TrapPolicerRate(val) => {
                    if last_off == offset {
                        stack.push(("TrapPolicerRate", last_off));
                        break;
                    }
                }
                Devlink::TrapPolicerBurst(val) => {
                    if last_off == offset {
                        stack.push(("TrapPolicerBurst", last_off));
                        break;
                    }
                }
                Devlink::PortFunction(val) => {
                    (stack, missing) = val.lookup_attr(offset, missing_type);
                    if !stack.is_empty() {
                        break;
                    }
                }
                Devlink::InfoBoardSerialNumber(val) => {
                    if last_off == offset {
                        stack.push(("InfoBoardSerialNumber", last_off));
                        break;
                    }
                }
                Devlink::PortLanes(val) => {
                    if last_off == offset {
                        stack.push(("PortLanes", last_off));
                        break;
                    }
                }
                Devlink::PortSplittable(val) => {
                    if last_off == offset {
                        stack.push(("PortSplittable", last_off));
                        break;
                    }
                }
                Devlink::PortExternal(val) => {
                    if last_off == offset {
                        stack.push(("PortExternal", last_off));
                        break;
                    }
                }
                Devlink::PortControllerNumber(val) => {
                    if last_off == offset {
                        stack.push(("PortControllerNumber", last_off));
                        break;
                    }
                }
                Devlink::FlashUpdateStatusTimeout(val) => {
                    if last_off == offset {
                        stack.push(("FlashUpdateStatusTimeout", last_off));
                        break;
                    }
                }
                Devlink::FlashUpdateOverwriteMask(val) => {
                    if last_off == offset {
                        stack.push(("FlashUpdateOverwriteMask", last_off));
                        break;
                    }
                }
                Devlink::ReloadAction(val) => {
                    if last_off == offset {
                        stack.push(("ReloadAction", last_off));
                        break;
                    }
                }
                Devlink::ReloadActionsPerformed(val) => {
                    if last_off == offset {
                        stack.push(("ReloadActionsPerformed", last_off));
                        break;
                    }
                }
                Devlink::ReloadLimits(val) => {
                    if last_off == offset {
                        stack.push(("ReloadLimits", last_off));
                        break;
                    }
                }
                Devlink::DevStats(val) => {
                    (stack, missing) = val.lookup_attr(offset, missing_type);
                    if !stack.is_empty() {
                        break;
                    }
                }
                Devlink::ReloadStats(val) => {
                    (stack, missing) = val.lookup_attr(offset, missing_type);
                    if !stack.is_empty() {
                        break;
                    }
                }
                Devlink::ReloadStatsEntry(val) => {
                    (stack, missing) = val.lookup_attr(offset, missing_type);
                    if !stack.is_empty() {
                        break;
                    }
                }
                Devlink::ReloadStatsLimit(val) => {
                    if last_off == offset {
                        stack.push(("ReloadStatsLimit", last_off));
                        break;
                    }
                }
                Devlink::ReloadStatsValue(val) => {
                    if last_off == offset {
                        stack.push(("ReloadStatsValue", last_off));
                        break;
                    }
                }
                Devlink::RemoteReloadStats(val) => {
                    (stack, missing) = val.lookup_attr(offset, missing_type);
                    if !stack.is_empty() {
                        break;
                    }
                }
                Devlink::ReloadActionInfo(val) => {
                    (stack, missing) = val.lookup_attr(offset, missing_type);
                    if !stack.is_empty() {
                        break;
                    }
                }
                Devlink::ReloadActionStats(val) => {
                    (stack, missing) = val.lookup_attr(offset, missing_type);
                    if !stack.is_empty() {
                        break;
                    }
                }
                Devlink::PortPciSfNumber(val) => {
                    if last_off == offset {
                        stack.push(("PortPciSfNumber", last_off));
                        break;
                    }
                }
                Devlink::RateType(val) => {
                    if last_off == offset {
                        stack.push(("RateType", last_off));
                        break;
                    }
                }
                Devlink::RateTxShare(val) => {
                    if last_off == offset {
                        stack.push(("RateTxShare", last_off));
                        break;
                    }
                }
                Devlink::RateTxMax(val) => {
                    if last_off == offset {
                        stack.push(("RateTxMax", last_off));
                        break;
                    }
                }
                Devlink::RateNodeName(val) => {
                    if last_off == offset {
                        stack.push(("RateNodeName", last_off));
                        break;
                    }
                }
                Devlink::RateParentNodeName(val) => {
                    if last_off == offset {
                        stack.push(("RateParentNodeName", last_off));
                        break;
                    }
                }
                Devlink::RegionMaxSnapshots(val) => {
                    if last_off == offset {
                        stack.push(("RegionMaxSnapshots", last_off));
                        break;
                    }
                }
                Devlink::LinecardIndex(val) => {
                    if last_off == offset {
                        stack.push(("LinecardIndex", last_off));
                        break;
                    }
                }
                Devlink::LinecardState(val) => {
                    if last_off == offset {
                        stack.push(("LinecardState", last_off));
                        break;
                    }
                }
                Devlink::LinecardType(val) => {
                    if last_off == offset {
                        stack.push(("LinecardType", last_off));
                        break;
                    }
                }
                Devlink::LinecardSupportedTypes(val) => {
                    (stack, missing) = val.lookup_attr(offset, missing_type);
                    if !stack.is_empty() {
                        break;
                    }
                }
                Devlink::Selftests(val) => {
                    (stack, missing) = val.lookup_attr(offset, missing_type);
                    if !stack.is_empty() {
                        break;
                    }
                }
                Devlink::RateTxPriority(val) => {
                    if last_off == offset {
                        stack.push(("RateTxPriority", last_off));
                        break;
                    }
                }
                Devlink::RateTxWeight(val) => {
                    if last_off == offset {
                        stack.push(("RateTxWeight", last_off));
                        break;
                    }
                }
                Devlink::RegionDirect(val) => {
                    if last_off == offset {
                        stack.push(("RegionDirect", last_off));
                        break;
                    }
                }
                Devlink::RateTcBws(val) => {
                    (stack, missing) = val.lookup_attr(offset, missing_type);
                    if !stack.is_empty() {
                        break;
                    }
                }
                Devlink::HealthReporterBurstPeriod(val) => {
                    if last_off == offset {
                        stack.push(("HealthReporterBurstPeriod", last_off));
                        break;
                    }
                }
                Devlink::ParamResetDefault(val) => {
                    if last_off == offset {
                        stack.push(("ParamResetDefault", last_off));
                        break;
                    }
                }
                Devlink::Index(val) => {
                    if last_off == offset {
                        stack.push(("Index", last_off));
                        break;
                    }
                }
                Devlink::ResourceScopeMask(val) => {
                    if last_off == offset {
                        stack.push(("ResourceScopeMask", last_off));
                        break;
                    }
                }
                _ => {}
            };
            last_off = cur + attrs.pos;
        }
        if !stack.is_empty() {
            stack.push(("Devlink", cur));
        }
        (stack, missing)
    }
}
#[derive(Clone)]
pub enum DlDevStats<'a> {
    ReloadStats(IterableDlReloadStats<'a>),
    RemoteReloadStats(IterableDlReloadStats<'a>),
}
impl<'a> IterableDlDevStats<'a> {
    pub fn get_reload_stats(&self) -> Result<IterableDlReloadStats<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(DlDevStats::ReloadStats(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "DlDevStats",
            "ReloadStats",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_remote_reload_stats(&self) -> Result<IterableDlReloadStats<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(DlDevStats::RemoteReloadStats(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "DlDevStats",
            "RemoteReloadStats",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl DlDevStats<'_> {
    pub fn new<'a>(buf: &'a [u8]) -> IterableDlDevStats<'a> {
        IterableDlDevStats::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        Devlink::attr_from_type(r#type)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableDlDevStats<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableDlDevStats<'a> {
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
impl<'a> Iterator for IterableDlDevStats<'a> {
    type Item = Result<DlDevStats<'a>, ErrorContext>;
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
                157u16 => DlDevStats::ReloadStats({
                    let res = Some(IterableDlReloadStats::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                161u16 => DlDevStats::RemoteReloadStats({
                    let res = Some(IterableDlReloadStats::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                n if cfg!(any(test, feature = "deny-unknown-attrs")) => break,
                n => continue,
            };
            return Some(Ok(res));
        }
        Some(Err(ErrorContext::new(
            "DlDevStats",
            r#type.and_then(|t| DlDevStats::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl<'a> std::fmt::Debug for IterableDlDevStats<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("DlDevStats");
        let mut iter = IterableDlDevStats::with_loc(&[], self.orig_loc);
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
                DlDevStats::ReloadStats(val) => fmt.field("ReloadStats", &val),
                DlDevStats::RemoteReloadStats(val) => fmt.field("RemoteReloadStats", &val),
            };
        }
        fmt.finish()
    }
}
impl IterableDlDevStats<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("DlDevStats", offset));
            return (
                stack,
                missing_type.and_then(|t| DlDevStats::attr_from_type(t)),
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
                DlDevStats::ReloadStats(val) => {
                    (stack, missing) = val.lookup_attr(offset, missing_type);
                    if !stack.is_empty() {
                        break;
                    }
                }
                DlDevStats::RemoteReloadStats(val) => {
                    (stack, missing) = val.lookup_attr(offset, missing_type);
                    if !stack.is_empty() {
                        break;
                    }
                }
                _ => {}
            };
            last_off = cur + attrs.pos;
        }
        if !stack.is_empty() {
            stack.push(("DlDevStats", cur));
        }
        (stack, missing)
    }
}
#[derive(Clone)]
pub enum DlReloadStats<'a> {
    #[doc = "Attribute may repeat multiple times (treat it as array)"]
    ReloadActionInfo(IterableDlReloadActInfo<'a>),
}
impl<'a> IterableDlReloadStats<'a> {
    #[doc = "Attribute may repeat multiple times (treat it as array)"]
    pub fn get_reload_action_info(
        &self,
    ) -> MultiAttrIterable<Self, DlReloadStats<'a>, IterableDlReloadActInfo<'a>> {
        MultiAttrIterable::new(self.clone(), |variant| {
            if let DlReloadStats::ReloadActionInfo(val) = variant {
                Some(val)
            } else {
                None
            }
        })
    }
}
impl DlReloadStats<'_> {
    pub fn new<'a>(buf: &'a [u8]) -> IterableDlReloadStats<'a> {
        IterableDlReloadStats::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        Devlink::attr_from_type(r#type)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableDlReloadStats<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableDlReloadStats<'a> {
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
impl<'a> Iterator for IterableDlReloadStats<'a> {
    type Item = Result<DlReloadStats<'a>, ErrorContext>;
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
                162u16 => DlReloadStats::ReloadActionInfo({
                    let res = Some(IterableDlReloadActInfo::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                n if cfg!(any(test, feature = "deny-unknown-attrs")) => break,
                n => continue,
            };
            return Some(Ok(res));
        }
        Some(Err(ErrorContext::new(
            "DlReloadStats",
            r#type.and_then(|t| DlReloadStats::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl<'a> std::fmt::Debug for IterableDlReloadStats<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("DlReloadStats");
        let mut iter = IterableDlReloadStats::with_loc(&[], self.orig_loc);
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
                DlReloadStats::ReloadActionInfo(val) => fmt.field("ReloadActionInfo", &val),
            };
        }
        fmt.finish()
    }
}
impl IterableDlReloadStats<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("DlReloadStats", offset));
            return (
                stack,
                missing_type.and_then(|t| DlReloadStats::attr_from_type(t)),
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
                DlReloadStats::ReloadActionInfo(val) => {
                    (stack, missing) = val.lookup_attr(offset, missing_type);
                    if !stack.is_empty() {
                        break;
                    }
                }
                _ => {}
            };
            last_off = cur + attrs.pos;
        }
        if !stack.is_empty() {
            stack.push(("DlReloadStats", cur));
        }
        (stack, missing)
    }
}
#[derive(Clone)]
pub enum DlReloadActInfo<'a> {
    #[doc = "Associated type: [`ReloadAction`] (enum)"]
    ReloadAction(u8),
    #[doc = "Attribute may repeat multiple times (treat it as array)"]
    ReloadActionStats(IterableDlReloadActStats<'a>),
}
impl<'a> IterableDlReloadActInfo<'a> {
    #[doc = "Associated type: [`ReloadAction`] (enum)"]
    pub fn get_reload_action(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(DlReloadActInfo::ReloadAction(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "DlReloadActInfo",
            "ReloadAction",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Attribute may repeat multiple times (treat it as array)"]
    pub fn get_reload_action_stats(
        &self,
    ) -> MultiAttrIterable<Self, DlReloadActInfo<'a>, IterableDlReloadActStats<'a>> {
        MultiAttrIterable::new(self.clone(), |variant| {
            if let DlReloadActInfo::ReloadActionStats(val) = variant {
                Some(val)
            } else {
                None
            }
        })
    }
}
impl DlReloadActInfo<'_> {
    pub fn new<'a>(buf: &'a [u8]) -> IterableDlReloadActInfo<'a> {
        IterableDlReloadActInfo::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        Devlink::attr_from_type(r#type)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableDlReloadActInfo<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableDlReloadActInfo<'a> {
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
impl<'a> Iterator for IterableDlReloadActInfo<'a> {
    type Item = Result<DlReloadActInfo<'a>, ErrorContext>;
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
                153u16 => DlReloadActInfo::ReloadAction({
                    let res = parse_u8(next);
                    let Some(val) = res else { break };
                    val
                }),
                163u16 => DlReloadActInfo::ReloadActionStats({
                    let res = Some(IterableDlReloadActStats::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                n if cfg!(any(test, feature = "deny-unknown-attrs")) => break,
                n => continue,
            };
            return Some(Ok(res));
        }
        Some(Err(ErrorContext::new(
            "DlReloadActInfo",
            r#type.and_then(|t| DlReloadActInfo::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl<'a> std::fmt::Debug for IterableDlReloadActInfo<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("DlReloadActInfo");
        let mut iter = IterableDlReloadActInfo::with_loc(&[], self.orig_loc);
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
                DlReloadActInfo::ReloadAction(val) => fmt.field(
                    "ReloadAction",
                    &FormatEnum(val.into(), ReloadAction::from_value),
                ),
                DlReloadActInfo::ReloadActionStats(val) => fmt.field("ReloadActionStats", &val),
            };
        }
        fmt.finish()
    }
}
impl IterableDlReloadActInfo<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("DlReloadActInfo", offset));
            return (
                stack,
                missing_type.and_then(|t| DlReloadActInfo::attr_from_type(t)),
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
                DlReloadActInfo::ReloadAction(val) => {
                    if last_off == offset {
                        stack.push(("ReloadAction", last_off));
                        break;
                    }
                }
                DlReloadActInfo::ReloadActionStats(val) => {
                    (stack, missing) = val.lookup_attr(offset, missing_type);
                    if !stack.is_empty() {
                        break;
                    }
                }
                _ => {}
            };
            last_off = cur + attrs.pos;
        }
        if !stack.is_empty() {
            stack.push(("DlReloadActInfo", cur));
        }
        (stack, missing)
    }
}
#[derive(Clone)]
pub enum DlReloadActStats<'a> {
    #[doc = "Attribute may repeat multiple times (treat it as array)"]
    ReloadStatsEntry(IterableDlReloadStatsEntry<'a>),
}
impl<'a> IterableDlReloadActStats<'a> {
    #[doc = "Attribute may repeat multiple times (treat it as array)"]
    pub fn get_reload_stats_entry(
        &self,
    ) -> MultiAttrIterable<Self, DlReloadActStats<'a>, IterableDlReloadStatsEntry<'a>> {
        MultiAttrIterable::new(self.clone(), |variant| {
            if let DlReloadActStats::ReloadStatsEntry(val) = variant {
                Some(val)
            } else {
                None
            }
        })
    }
}
impl DlReloadActStats<'_> {
    pub fn new<'a>(buf: &'a [u8]) -> IterableDlReloadActStats<'a> {
        IterableDlReloadActStats::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        Devlink::attr_from_type(r#type)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableDlReloadActStats<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableDlReloadActStats<'a> {
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
impl<'a> Iterator for IterableDlReloadActStats<'a> {
    type Item = Result<DlReloadActStats<'a>, ErrorContext>;
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
                158u16 => DlReloadActStats::ReloadStatsEntry({
                    let res = Some(IterableDlReloadStatsEntry::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                n if cfg!(any(test, feature = "deny-unknown-attrs")) => break,
                n => continue,
            };
            return Some(Ok(res));
        }
        Some(Err(ErrorContext::new(
            "DlReloadActStats",
            r#type.and_then(|t| DlReloadActStats::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl<'a> std::fmt::Debug for IterableDlReloadActStats<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("DlReloadActStats");
        let mut iter = IterableDlReloadActStats::with_loc(&[], self.orig_loc);
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
                DlReloadActStats::ReloadStatsEntry(val) => fmt.field("ReloadStatsEntry", &val),
            };
        }
        fmt.finish()
    }
}
impl IterableDlReloadActStats<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("DlReloadActStats", offset));
            return (
                stack,
                missing_type.and_then(|t| DlReloadActStats::attr_from_type(t)),
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
                DlReloadActStats::ReloadStatsEntry(val) => {
                    (stack, missing) = val.lookup_attr(offset, missing_type);
                    if !stack.is_empty() {
                        break;
                    }
                }
                _ => {}
            };
            last_off = cur + attrs.pos;
        }
        if !stack.is_empty() {
            stack.push(("DlReloadActStats", cur));
        }
        (stack, missing)
    }
}
#[derive(Clone)]
pub enum DlReloadStatsEntry {
    ReloadStatsLimit(u8),
    ReloadStatsValue(u32),
}
impl<'a> IterableDlReloadStatsEntry<'a> {
    pub fn get_reload_stats_limit(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(DlReloadStatsEntry::ReloadStatsLimit(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "DlReloadStatsEntry",
            "ReloadStatsLimit",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_reload_stats_value(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(DlReloadStatsEntry::ReloadStatsValue(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "DlReloadStatsEntry",
            "ReloadStatsValue",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl DlReloadStatsEntry {
    pub fn new<'a>(buf: &'a [u8]) -> IterableDlReloadStatsEntry<'a> {
        IterableDlReloadStatsEntry::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        Devlink::attr_from_type(r#type)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableDlReloadStatsEntry<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableDlReloadStatsEntry<'a> {
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
impl<'a> Iterator for IterableDlReloadStatsEntry<'a> {
    type Item = Result<DlReloadStatsEntry, ErrorContext>;
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
                159u16 => DlReloadStatsEntry::ReloadStatsLimit({
                    let res = parse_u8(next);
                    let Some(val) = res else { break };
                    val
                }),
                160u16 => DlReloadStatsEntry::ReloadStatsValue({
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
            "DlReloadStatsEntry",
            r#type.and_then(|t| DlReloadStatsEntry::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl std::fmt::Debug for IterableDlReloadStatsEntry<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("DlReloadStatsEntry");
        let mut iter = IterableDlReloadStatsEntry::with_loc(&[], self.orig_loc);
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
                DlReloadStatsEntry::ReloadStatsLimit(val) => fmt.field("ReloadStatsLimit", &val),
                DlReloadStatsEntry::ReloadStatsValue(val) => fmt.field("ReloadStatsValue", &val),
            };
        }
        fmt.finish()
    }
}
impl IterableDlReloadStatsEntry<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("DlReloadStatsEntry", offset));
            return (
                stack,
                missing_type.and_then(|t| DlReloadStatsEntry::attr_from_type(t)),
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
                DlReloadStatsEntry::ReloadStatsLimit(val) => {
                    if last_off == offset {
                        stack.push(("ReloadStatsLimit", last_off));
                        break;
                    }
                }
                DlReloadStatsEntry::ReloadStatsValue(val) => {
                    if last_off == offset {
                        stack.push(("ReloadStatsValue", last_off));
                        break;
                    }
                }
                _ => {}
            };
            last_off = cur + attrs.pos;
        }
        if !stack.is_empty() {
            stack.push(("DlReloadStatsEntry", cur));
        }
        (stack, None)
    }
}
#[derive(Clone)]
pub enum DlInfoVersion<'a> {
    InfoVersionName(&'a CStr),
    InfoVersionValue(&'a CStr),
}
impl<'a> IterableDlInfoVersion<'a> {
    pub fn get_info_version_name(&self) -> Result<&'a CStr, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(DlInfoVersion::InfoVersionName(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "DlInfoVersion",
            "InfoVersionName",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_info_version_value(&self) -> Result<&'a CStr, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(DlInfoVersion::InfoVersionValue(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "DlInfoVersion",
            "InfoVersionValue",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl DlInfoVersion<'_> {
    pub fn new<'a>(buf: &'a [u8]) -> IterableDlInfoVersion<'a> {
        IterableDlInfoVersion::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        Devlink::attr_from_type(r#type)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableDlInfoVersion<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableDlInfoVersion<'a> {
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
impl<'a> Iterator for IterableDlInfoVersion<'a> {
    type Item = Result<DlInfoVersion<'a>, ErrorContext>;
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
                103u16 => DlInfoVersion::InfoVersionName({
                    let res = CStr::from_bytes_with_nul(next).ok();
                    let Some(val) = res else { break };
                    val
                }),
                104u16 => DlInfoVersion::InfoVersionValue({
                    let res = CStr::from_bytes_with_nul(next).ok();
                    let Some(val) = res else { break };
                    val
                }),
                n if cfg!(any(test, feature = "deny-unknown-attrs")) => break,
                n => continue,
            };
            return Some(Ok(res));
        }
        Some(Err(ErrorContext::new(
            "DlInfoVersion",
            r#type.and_then(|t| DlInfoVersion::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl<'a> std::fmt::Debug for IterableDlInfoVersion<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("DlInfoVersion");
        let mut iter = IterableDlInfoVersion::with_loc(&[], self.orig_loc);
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
                DlInfoVersion::InfoVersionName(val) => fmt.field("InfoVersionName", &val),
                DlInfoVersion::InfoVersionValue(val) => fmt.field("InfoVersionValue", &val),
            };
        }
        fmt.finish()
    }
}
impl IterableDlInfoVersion<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("DlInfoVersion", offset));
            return (
                stack,
                missing_type.and_then(|t| DlInfoVersion::attr_from_type(t)),
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
                DlInfoVersion::InfoVersionName(val) => {
                    if last_off == offset {
                        stack.push(("InfoVersionName", last_off));
                        break;
                    }
                }
                DlInfoVersion::InfoVersionValue(val) => {
                    if last_off == offset {
                        stack.push(("InfoVersionValue", last_off));
                        break;
                    }
                }
                _ => {}
            };
            last_off = cur + attrs.pos;
        }
        if !stack.is_empty() {
            stack.push(("DlInfoVersion", cur));
        }
        (stack, None)
    }
}
#[derive(Clone)]
pub enum DlPortFunction<'a> {
    HwAddr(&'a [u8]),
    #[doc = "Associated type: [`PortFnState`] (enum)"]
    State(u8),
    #[doc = "Associated type: [`PortFnOpstate`] (enum)"]
    Opstate(u8),
    #[doc = "Associated type: [`PortFnAttrCap`] (1 bit per enumeration)"]
    Caps(BuiltinBitfield32),
}
impl<'a> IterableDlPortFunction<'a> {
    pub fn get_hw_addr(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(DlPortFunction::HwAddr(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "DlPortFunction",
            "HwAddr",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Associated type: [`PortFnState`] (enum)"]
    pub fn get_state(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(DlPortFunction::State(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "DlPortFunction",
            "State",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Associated type: [`PortFnOpstate`] (enum)"]
    pub fn get_opstate(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(DlPortFunction::Opstate(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "DlPortFunction",
            "Opstate",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Associated type: [`PortFnAttrCap`] (1 bit per enumeration)"]
    pub fn get_caps(&self) -> Result<BuiltinBitfield32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(DlPortFunction::Caps(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "DlPortFunction",
            "Caps",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl DlPortFunction<'_> {
    pub fn new<'a>(buf: &'a [u8]) -> IterableDlPortFunction<'a> {
        IterableDlPortFunction::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        let res = match r#type {
            1u16 => "HwAddr",
            2u16 => "State",
            3u16 => "Opstate",
            4u16 => "Caps",
            _ => return None,
        };
        Some(res)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableDlPortFunction<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableDlPortFunction<'a> {
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
impl<'a> Iterator for IterableDlPortFunction<'a> {
    type Item = Result<DlPortFunction<'a>, ErrorContext>;
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
                1u16 => DlPortFunction::HwAddr({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                2u16 => DlPortFunction::State({
                    let res = parse_u8(next);
                    let Some(val) = res else { break };
                    val
                }),
                3u16 => DlPortFunction::Opstate({
                    let res = parse_u8(next);
                    let Some(val) = res else { break };
                    val
                }),
                4u16 => DlPortFunction::Caps({
                    let res = BuiltinBitfield32::new_from_slice(next);
                    let Some(val) = res else { break };
                    val
                }),
                n if cfg!(any(test, feature = "deny-unknown-attrs")) => break,
                n => continue,
            };
            return Some(Ok(res));
        }
        Some(Err(ErrorContext::new(
            "DlPortFunction",
            r#type.and_then(|t| DlPortFunction::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl<'a> std::fmt::Debug for IterableDlPortFunction<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("DlPortFunction");
        let mut iter = IterableDlPortFunction::with_loc(&[], self.orig_loc);
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
                DlPortFunction::HwAddr(val) => fmt.field("HwAddr", &FormatHexdump(val)),
                DlPortFunction::State(val) => {
                    fmt.field("State", &FormatEnum(val.into(), PortFnState::from_value))
                }
                DlPortFunction::Opstate(val) => fmt.field(
                    "Opstate",
                    &FormatEnum(val.into(), PortFnOpstate::from_value),
                ),
                DlPortFunction::Caps(val) => fmt.field(
                    "Caps",
                    &FormatFlags(val.value.into(), |val| {
                        PortFnAttrCap::from_value(val.trailing_zeros().into())
                    }),
                ),
            };
        }
        fmt.finish()
    }
}
impl IterableDlPortFunction<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("DlPortFunction", offset));
            return (
                stack,
                missing_type.and_then(|t| DlPortFunction::attr_from_type(t)),
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
                DlPortFunction::HwAddr(val) => {
                    if last_off == offset {
                        stack.push(("HwAddr", last_off));
                        break;
                    }
                }
                DlPortFunction::State(val) => {
                    if last_off == offset {
                        stack.push(("State", last_off));
                        break;
                    }
                }
                DlPortFunction::Opstate(val) => {
                    if last_off == offset {
                        stack.push(("Opstate", last_off));
                        break;
                    }
                }
                DlPortFunction::Caps(val) => {
                    if last_off == offset {
                        stack.push(("Caps", last_off));
                        break;
                    }
                }
                _ => {}
            };
            last_off = cur + attrs.pos;
        }
        if !stack.is_empty() {
            stack.push(("DlPortFunction", cur));
        }
        (stack, None)
    }
}
#[derive(Clone)]
pub enum DlDpipeTables<'a> {
    #[doc = "Attribute may repeat multiple times (treat it as array)"]
    DpipeTable(IterableDlDpipeTable<'a>),
}
impl<'a> IterableDlDpipeTables<'a> {
    #[doc = "Attribute may repeat multiple times (treat it as array)"]
    pub fn get_dpipe_table(
        &self,
    ) -> MultiAttrIterable<Self, DlDpipeTables<'a>, IterableDlDpipeTable<'a>> {
        MultiAttrIterable::new(self.clone(), |variant| {
            if let DlDpipeTables::DpipeTable(val) = variant {
                Some(val)
            } else {
                None
            }
        })
    }
}
impl DlDpipeTables<'_> {
    pub fn new<'a>(buf: &'a [u8]) -> IterableDlDpipeTables<'a> {
        IterableDlDpipeTables::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        Devlink::attr_from_type(r#type)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableDlDpipeTables<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableDlDpipeTables<'a> {
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
impl<'a> Iterator for IterableDlDpipeTables<'a> {
    type Item = Result<DlDpipeTables<'a>, ErrorContext>;
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
                28u16 => DlDpipeTables::DpipeTable({
                    let res = Some(IterableDlDpipeTable::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                n if cfg!(any(test, feature = "deny-unknown-attrs")) => break,
                n => continue,
            };
            return Some(Ok(res));
        }
        Some(Err(ErrorContext::new(
            "DlDpipeTables",
            r#type.and_then(|t| DlDpipeTables::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl<'a> std::fmt::Debug for IterableDlDpipeTables<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("DlDpipeTables");
        let mut iter = IterableDlDpipeTables::with_loc(&[], self.orig_loc);
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
                DlDpipeTables::DpipeTable(val) => fmt.field("DpipeTable", &val),
            };
        }
        fmt.finish()
    }
}
impl IterableDlDpipeTables<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("DlDpipeTables", offset));
            return (
                stack,
                missing_type.and_then(|t| DlDpipeTables::attr_from_type(t)),
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
                DlDpipeTables::DpipeTable(val) => {
                    (stack, missing) = val.lookup_attr(offset, missing_type);
                    if !stack.is_empty() {
                        break;
                    }
                }
                _ => {}
            };
            last_off = cur + attrs.pos;
        }
        if !stack.is_empty() {
            stack.push(("DlDpipeTables", cur));
        }
        (stack, missing)
    }
}
#[derive(Clone)]
pub enum DlDpipeTable<'a> {
    DpipeTableName(&'a CStr),
    DpipeTableSize(u64),
    DpipeTableMatches(IterableDlDpipeTableMatches<'a>),
    DpipeTableActions(IterableDlDpipeTableActions<'a>),
    DpipeTableCountersEnabled(u8),
    DpipeTableResourceId(u64),
    DpipeTableResourceUnits(u64),
}
impl<'a> IterableDlDpipeTable<'a> {
    pub fn get_dpipe_table_name(&self) -> Result<&'a CStr, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(DlDpipeTable::DpipeTableName(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "DlDpipeTable",
            "DpipeTableName",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_dpipe_table_size(&self) -> Result<u64, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(DlDpipeTable::DpipeTableSize(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "DlDpipeTable",
            "DpipeTableSize",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_dpipe_table_matches(&self) -> Result<IterableDlDpipeTableMatches<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(DlDpipeTable::DpipeTableMatches(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "DlDpipeTable",
            "DpipeTableMatches",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_dpipe_table_actions(&self) -> Result<IterableDlDpipeTableActions<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(DlDpipeTable::DpipeTableActions(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "DlDpipeTable",
            "DpipeTableActions",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_dpipe_table_counters_enabled(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(DlDpipeTable::DpipeTableCountersEnabled(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "DlDpipeTable",
            "DpipeTableCountersEnabled",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_dpipe_table_resource_id(&self) -> Result<u64, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(DlDpipeTable::DpipeTableResourceId(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "DlDpipeTable",
            "DpipeTableResourceId",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_dpipe_table_resource_units(&self) -> Result<u64, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(DlDpipeTable::DpipeTableResourceUnits(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "DlDpipeTable",
            "DpipeTableResourceUnits",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl DlDpipeTable<'_> {
    pub fn new<'a>(buf: &'a [u8]) -> IterableDlDpipeTable<'a> {
        IterableDlDpipeTable::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        Devlink::attr_from_type(r#type)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableDlDpipeTable<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableDlDpipeTable<'a> {
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
impl<'a> Iterator for IterableDlDpipeTable<'a> {
    type Item = Result<DlDpipeTable<'a>, ErrorContext>;
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
                29u16 => DlDpipeTable::DpipeTableName({
                    let res = CStr::from_bytes_with_nul(next).ok();
                    let Some(val) = res else { break };
                    val
                }),
                30u16 => DlDpipeTable::DpipeTableSize({
                    let res = parse_u64(next);
                    let Some(val) = res else { break };
                    val
                }),
                31u16 => DlDpipeTable::DpipeTableMatches({
                    let res = Some(IterableDlDpipeTableMatches::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                32u16 => DlDpipeTable::DpipeTableActions({
                    let res = Some(IterableDlDpipeTableActions::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                33u16 => DlDpipeTable::DpipeTableCountersEnabled({
                    let res = parse_u8(next);
                    let Some(val) = res else { break };
                    val
                }),
                75u16 => DlDpipeTable::DpipeTableResourceId({
                    let res = parse_u64(next);
                    let Some(val) = res else { break };
                    val
                }),
                76u16 => DlDpipeTable::DpipeTableResourceUnits({
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
            "DlDpipeTable",
            r#type.and_then(|t| DlDpipeTable::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl<'a> std::fmt::Debug for IterableDlDpipeTable<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("DlDpipeTable");
        let mut iter = IterableDlDpipeTable::with_loc(&[], self.orig_loc);
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
                DlDpipeTable::DpipeTableName(val) => fmt.field("DpipeTableName", &val),
                DlDpipeTable::DpipeTableSize(val) => fmt.field("DpipeTableSize", &val),
                DlDpipeTable::DpipeTableMatches(val) => fmt.field("DpipeTableMatches", &val),
                DlDpipeTable::DpipeTableActions(val) => fmt.field("DpipeTableActions", &val),
                DlDpipeTable::DpipeTableCountersEnabled(val) => {
                    fmt.field("DpipeTableCountersEnabled", &val)
                }
                DlDpipeTable::DpipeTableResourceId(val) => fmt.field("DpipeTableResourceId", &val),
                DlDpipeTable::DpipeTableResourceUnits(val) => {
                    fmt.field("DpipeTableResourceUnits", &val)
                }
            };
        }
        fmt.finish()
    }
}
impl IterableDlDpipeTable<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("DlDpipeTable", offset));
            return (
                stack,
                missing_type.and_then(|t| DlDpipeTable::attr_from_type(t)),
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
                DlDpipeTable::DpipeTableName(val) => {
                    if last_off == offset {
                        stack.push(("DpipeTableName", last_off));
                        break;
                    }
                }
                DlDpipeTable::DpipeTableSize(val) => {
                    if last_off == offset {
                        stack.push(("DpipeTableSize", last_off));
                        break;
                    }
                }
                DlDpipeTable::DpipeTableMatches(val) => {
                    (stack, missing) = val.lookup_attr(offset, missing_type);
                    if !stack.is_empty() {
                        break;
                    }
                }
                DlDpipeTable::DpipeTableActions(val) => {
                    (stack, missing) = val.lookup_attr(offset, missing_type);
                    if !stack.is_empty() {
                        break;
                    }
                }
                DlDpipeTable::DpipeTableCountersEnabled(val) => {
                    if last_off == offset {
                        stack.push(("DpipeTableCountersEnabled", last_off));
                        break;
                    }
                }
                DlDpipeTable::DpipeTableResourceId(val) => {
                    if last_off == offset {
                        stack.push(("DpipeTableResourceId", last_off));
                        break;
                    }
                }
                DlDpipeTable::DpipeTableResourceUnits(val) => {
                    if last_off == offset {
                        stack.push(("DpipeTableResourceUnits", last_off));
                        break;
                    }
                }
                _ => {}
            };
            last_off = cur + attrs.pos;
        }
        if !stack.is_empty() {
            stack.push(("DlDpipeTable", cur));
        }
        (stack, missing)
    }
}
#[derive(Clone)]
pub enum DlDpipeTableMatches<'a> {
    #[doc = "Attribute may repeat multiple times (treat it as array)"]
    DpipeMatch(IterableDlDpipeMatch<'a>),
}
impl<'a> IterableDlDpipeTableMatches<'a> {
    #[doc = "Attribute may repeat multiple times (treat it as array)"]
    pub fn get_dpipe_match(
        &self,
    ) -> MultiAttrIterable<Self, DlDpipeTableMatches<'a>, IterableDlDpipeMatch<'a>> {
        MultiAttrIterable::new(self.clone(), |variant| {
            if let DlDpipeTableMatches::DpipeMatch(val) = variant {
                Some(val)
            } else {
                None
            }
        })
    }
}
impl DlDpipeTableMatches<'_> {
    pub fn new<'a>(buf: &'a [u8]) -> IterableDlDpipeTableMatches<'a> {
        IterableDlDpipeTableMatches::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        Devlink::attr_from_type(r#type)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableDlDpipeTableMatches<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableDlDpipeTableMatches<'a> {
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
impl<'a> Iterator for IterableDlDpipeTableMatches<'a> {
    type Item = Result<DlDpipeTableMatches<'a>, ErrorContext>;
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
                40u16 => DlDpipeTableMatches::DpipeMatch({
                    let res = Some(IterableDlDpipeMatch::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                n if cfg!(any(test, feature = "deny-unknown-attrs")) => break,
                n => continue,
            };
            return Some(Ok(res));
        }
        Some(Err(ErrorContext::new(
            "DlDpipeTableMatches",
            r#type.and_then(|t| DlDpipeTableMatches::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl<'a> std::fmt::Debug for IterableDlDpipeTableMatches<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("DlDpipeTableMatches");
        let mut iter = IterableDlDpipeTableMatches::with_loc(&[], self.orig_loc);
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
                DlDpipeTableMatches::DpipeMatch(val) => fmt.field("DpipeMatch", &val),
            };
        }
        fmt.finish()
    }
}
impl IterableDlDpipeTableMatches<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("DlDpipeTableMatches", offset));
            return (
                stack,
                missing_type.and_then(|t| DlDpipeTableMatches::attr_from_type(t)),
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
                DlDpipeTableMatches::DpipeMatch(val) => {
                    (stack, missing) = val.lookup_attr(offset, missing_type);
                    if !stack.is_empty() {
                        break;
                    }
                }
                _ => {}
            };
            last_off = cur + attrs.pos;
        }
        if !stack.is_empty() {
            stack.push(("DlDpipeTableMatches", cur));
        }
        (stack, missing)
    }
}
#[derive(Clone)]
pub enum DlDpipeTableActions<'a> {
    #[doc = "Attribute may repeat multiple times (treat it as array)"]
    DpipeAction(IterableDlDpipeAction<'a>),
}
impl<'a> IterableDlDpipeTableActions<'a> {
    #[doc = "Attribute may repeat multiple times (treat it as array)"]
    pub fn get_dpipe_action(
        &self,
    ) -> MultiAttrIterable<Self, DlDpipeTableActions<'a>, IterableDlDpipeAction<'a>> {
        MultiAttrIterable::new(self.clone(), |variant| {
            if let DlDpipeTableActions::DpipeAction(val) = variant {
                Some(val)
            } else {
                None
            }
        })
    }
}
impl DlDpipeTableActions<'_> {
    pub fn new<'a>(buf: &'a [u8]) -> IterableDlDpipeTableActions<'a> {
        IterableDlDpipeTableActions::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        Devlink::attr_from_type(r#type)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableDlDpipeTableActions<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableDlDpipeTableActions<'a> {
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
impl<'a> Iterator for IterableDlDpipeTableActions<'a> {
    type Item = Result<DlDpipeTableActions<'a>, ErrorContext>;
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
                43u16 => DlDpipeTableActions::DpipeAction({
                    let res = Some(IterableDlDpipeAction::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                n if cfg!(any(test, feature = "deny-unknown-attrs")) => break,
                n => continue,
            };
            return Some(Ok(res));
        }
        Some(Err(ErrorContext::new(
            "DlDpipeTableActions",
            r#type.and_then(|t| DlDpipeTableActions::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl<'a> std::fmt::Debug for IterableDlDpipeTableActions<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("DlDpipeTableActions");
        let mut iter = IterableDlDpipeTableActions::with_loc(&[], self.orig_loc);
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
                DlDpipeTableActions::DpipeAction(val) => fmt.field("DpipeAction", &val),
            };
        }
        fmt.finish()
    }
}
impl IterableDlDpipeTableActions<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("DlDpipeTableActions", offset));
            return (
                stack,
                missing_type.and_then(|t| DlDpipeTableActions::attr_from_type(t)),
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
                DlDpipeTableActions::DpipeAction(val) => {
                    (stack, missing) = val.lookup_attr(offset, missing_type);
                    if !stack.is_empty() {
                        break;
                    }
                }
                _ => {}
            };
            last_off = cur + attrs.pos;
        }
        if !stack.is_empty() {
            stack.push(("DlDpipeTableActions", cur));
        }
        (stack, missing)
    }
}
#[derive(Clone)]
pub enum DlDpipeEntries<'a> {
    #[doc = "Attribute may repeat multiple times (treat it as array)"]
    DpipeEntry(IterableDlDpipeEntry<'a>),
}
impl<'a> IterableDlDpipeEntries<'a> {
    #[doc = "Attribute may repeat multiple times (treat it as array)"]
    pub fn get_dpipe_entry(
        &self,
    ) -> MultiAttrIterable<Self, DlDpipeEntries<'a>, IterableDlDpipeEntry<'a>> {
        MultiAttrIterable::new(self.clone(), |variant| {
            if let DlDpipeEntries::DpipeEntry(val) = variant {
                Some(val)
            } else {
                None
            }
        })
    }
}
impl DlDpipeEntries<'_> {
    pub fn new<'a>(buf: &'a [u8]) -> IterableDlDpipeEntries<'a> {
        IterableDlDpipeEntries::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        Devlink::attr_from_type(r#type)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableDlDpipeEntries<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableDlDpipeEntries<'a> {
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
impl<'a> Iterator for IterableDlDpipeEntries<'a> {
    type Item = Result<DlDpipeEntries<'a>, ErrorContext>;
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
                35u16 => DlDpipeEntries::DpipeEntry({
                    let res = Some(IterableDlDpipeEntry::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                n if cfg!(any(test, feature = "deny-unknown-attrs")) => break,
                n => continue,
            };
            return Some(Ok(res));
        }
        Some(Err(ErrorContext::new(
            "DlDpipeEntries",
            r#type.and_then(|t| DlDpipeEntries::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl<'a> std::fmt::Debug for IterableDlDpipeEntries<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("DlDpipeEntries");
        let mut iter = IterableDlDpipeEntries::with_loc(&[], self.orig_loc);
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
                DlDpipeEntries::DpipeEntry(val) => fmt.field("DpipeEntry", &val),
            };
        }
        fmt.finish()
    }
}
impl IterableDlDpipeEntries<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("DlDpipeEntries", offset));
            return (
                stack,
                missing_type.and_then(|t| DlDpipeEntries::attr_from_type(t)),
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
                DlDpipeEntries::DpipeEntry(val) => {
                    (stack, missing) = val.lookup_attr(offset, missing_type);
                    if !stack.is_empty() {
                        break;
                    }
                }
                _ => {}
            };
            last_off = cur + attrs.pos;
        }
        if !stack.is_empty() {
            stack.push(("DlDpipeEntries", cur));
        }
        (stack, missing)
    }
}
#[derive(Clone)]
pub enum DlDpipeEntry<'a> {
    DpipeEntryIndex(u64),
    DpipeEntryMatchValues(IterableDlDpipeEntryMatchValues<'a>),
    DpipeEntryActionValues(IterableDlDpipeEntryActionValues<'a>),
    DpipeEntryCounter(u64),
}
impl<'a> IterableDlDpipeEntry<'a> {
    pub fn get_dpipe_entry_index(&self) -> Result<u64, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(DlDpipeEntry::DpipeEntryIndex(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "DlDpipeEntry",
            "DpipeEntryIndex",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_dpipe_entry_match_values(
        &self,
    ) -> Result<IterableDlDpipeEntryMatchValues<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(DlDpipeEntry::DpipeEntryMatchValues(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "DlDpipeEntry",
            "DpipeEntryMatchValues",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_dpipe_entry_action_values(
        &self,
    ) -> Result<IterableDlDpipeEntryActionValues<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(DlDpipeEntry::DpipeEntryActionValues(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "DlDpipeEntry",
            "DpipeEntryActionValues",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_dpipe_entry_counter(&self) -> Result<u64, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(DlDpipeEntry::DpipeEntryCounter(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "DlDpipeEntry",
            "DpipeEntryCounter",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl DlDpipeEntry<'_> {
    pub fn new<'a>(buf: &'a [u8]) -> IterableDlDpipeEntry<'a> {
        IterableDlDpipeEntry::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        Devlink::attr_from_type(r#type)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableDlDpipeEntry<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableDlDpipeEntry<'a> {
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
impl<'a> Iterator for IterableDlDpipeEntry<'a> {
    type Item = Result<DlDpipeEntry<'a>, ErrorContext>;
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
                36u16 => DlDpipeEntry::DpipeEntryIndex({
                    let res = parse_u64(next);
                    let Some(val) = res else { break };
                    val
                }),
                37u16 => DlDpipeEntry::DpipeEntryMatchValues({
                    let res = Some(IterableDlDpipeEntryMatchValues::with_loc(
                        next,
                        self.orig_loc,
                    ));
                    let Some(val) = res else { break };
                    val
                }),
                38u16 => DlDpipeEntry::DpipeEntryActionValues({
                    let res = Some(IterableDlDpipeEntryActionValues::with_loc(
                        next,
                        self.orig_loc,
                    ));
                    let Some(val) = res else { break };
                    val
                }),
                39u16 => DlDpipeEntry::DpipeEntryCounter({
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
            "DlDpipeEntry",
            r#type.and_then(|t| DlDpipeEntry::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl<'a> std::fmt::Debug for IterableDlDpipeEntry<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("DlDpipeEntry");
        let mut iter = IterableDlDpipeEntry::with_loc(&[], self.orig_loc);
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
                DlDpipeEntry::DpipeEntryIndex(val) => fmt.field("DpipeEntryIndex", &val),
                DlDpipeEntry::DpipeEntryMatchValues(val) => {
                    fmt.field("DpipeEntryMatchValues", &val)
                }
                DlDpipeEntry::DpipeEntryActionValues(val) => {
                    fmt.field("DpipeEntryActionValues", &val)
                }
                DlDpipeEntry::DpipeEntryCounter(val) => fmt.field("DpipeEntryCounter", &val),
            };
        }
        fmt.finish()
    }
}
impl IterableDlDpipeEntry<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("DlDpipeEntry", offset));
            return (
                stack,
                missing_type.and_then(|t| DlDpipeEntry::attr_from_type(t)),
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
                DlDpipeEntry::DpipeEntryIndex(val) => {
                    if last_off == offset {
                        stack.push(("DpipeEntryIndex", last_off));
                        break;
                    }
                }
                DlDpipeEntry::DpipeEntryMatchValues(val) => {
                    (stack, missing) = val.lookup_attr(offset, missing_type);
                    if !stack.is_empty() {
                        break;
                    }
                }
                DlDpipeEntry::DpipeEntryActionValues(val) => {
                    (stack, missing) = val.lookup_attr(offset, missing_type);
                    if !stack.is_empty() {
                        break;
                    }
                }
                DlDpipeEntry::DpipeEntryCounter(val) => {
                    if last_off == offset {
                        stack.push(("DpipeEntryCounter", last_off));
                        break;
                    }
                }
                _ => {}
            };
            last_off = cur + attrs.pos;
        }
        if !stack.is_empty() {
            stack.push(("DlDpipeEntry", cur));
        }
        (stack, missing)
    }
}
#[derive(Clone)]
pub enum DlDpipeEntryMatchValues<'a> {
    #[doc = "Attribute may repeat multiple times (treat it as array)"]
    DpipeMatchValue(IterableDlDpipeMatchValue<'a>),
}
impl<'a> IterableDlDpipeEntryMatchValues<'a> {
    #[doc = "Attribute may repeat multiple times (treat it as array)"]
    pub fn get_dpipe_match_value(
        &self,
    ) -> MultiAttrIterable<Self, DlDpipeEntryMatchValues<'a>, IterableDlDpipeMatchValue<'a>> {
        MultiAttrIterable::new(self.clone(), |variant| {
            if let DlDpipeEntryMatchValues::DpipeMatchValue(val) = variant {
                Some(val)
            } else {
                None
            }
        })
    }
}
impl DlDpipeEntryMatchValues<'_> {
    pub fn new<'a>(buf: &'a [u8]) -> IterableDlDpipeEntryMatchValues<'a> {
        IterableDlDpipeEntryMatchValues::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        Devlink::attr_from_type(r#type)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableDlDpipeEntryMatchValues<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableDlDpipeEntryMatchValues<'a> {
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
impl<'a> Iterator for IterableDlDpipeEntryMatchValues<'a> {
    type Item = Result<DlDpipeEntryMatchValues<'a>, ErrorContext>;
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
                41u16 => DlDpipeEntryMatchValues::DpipeMatchValue({
                    let res = Some(IterableDlDpipeMatchValue::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                n if cfg!(any(test, feature = "deny-unknown-attrs")) => break,
                n => continue,
            };
            return Some(Ok(res));
        }
        Some(Err(ErrorContext::new(
            "DlDpipeEntryMatchValues",
            r#type.and_then(|t| DlDpipeEntryMatchValues::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl<'a> std::fmt::Debug for IterableDlDpipeEntryMatchValues<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("DlDpipeEntryMatchValues");
        let mut iter = IterableDlDpipeEntryMatchValues::with_loc(&[], self.orig_loc);
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
                DlDpipeEntryMatchValues::DpipeMatchValue(val) => fmt.field("DpipeMatchValue", &val),
            };
        }
        fmt.finish()
    }
}
impl IterableDlDpipeEntryMatchValues<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("DlDpipeEntryMatchValues", offset));
            return (
                stack,
                missing_type.and_then(|t| DlDpipeEntryMatchValues::attr_from_type(t)),
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
                DlDpipeEntryMatchValues::DpipeMatchValue(val) => {
                    (stack, missing) = val.lookup_attr(offset, missing_type);
                    if !stack.is_empty() {
                        break;
                    }
                }
                _ => {}
            };
            last_off = cur + attrs.pos;
        }
        if !stack.is_empty() {
            stack.push(("DlDpipeEntryMatchValues", cur));
        }
        (stack, missing)
    }
}
#[derive(Clone)]
pub enum DlDpipeEntryActionValues<'a> {
    #[doc = "Attribute may repeat multiple times (treat it as array)"]
    DpipeActionValue(IterableDlDpipeActionValue<'a>),
}
impl<'a> IterableDlDpipeEntryActionValues<'a> {
    #[doc = "Attribute may repeat multiple times (treat it as array)"]
    pub fn get_dpipe_action_value(
        &self,
    ) -> MultiAttrIterable<Self, DlDpipeEntryActionValues<'a>, IterableDlDpipeActionValue<'a>> {
        MultiAttrIterable::new(self.clone(), |variant| {
            if let DlDpipeEntryActionValues::DpipeActionValue(val) = variant {
                Some(val)
            } else {
                None
            }
        })
    }
}
impl DlDpipeEntryActionValues<'_> {
    pub fn new<'a>(buf: &'a [u8]) -> IterableDlDpipeEntryActionValues<'a> {
        IterableDlDpipeEntryActionValues::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        Devlink::attr_from_type(r#type)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableDlDpipeEntryActionValues<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableDlDpipeEntryActionValues<'a> {
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
impl<'a> Iterator for IterableDlDpipeEntryActionValues<'a> {
    type Item = Result<DlDpipeEntryActionValues<'a>, ErrorContext>;
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
                44u16 => DlDpipeEntryActionValues::DpipeActionValue({
                    let res = Some(IterableDlDpipeActionValue::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                n if cfg!(any(test, feature = "deny-unknown-attrs")) => break,
                n => continue,
            };
            return Some(Ok(res));
        }
        Some(Err(ErrorContext::new(
            "DlDpipeEntryActionValues",
            r#type.and_then(|t| DlDpipeEntryActionValues::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl<'a> std::fmt::Debug for IterableDlDpipeEntryActionValues<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("DlDpipeEntryActionValues");
        let mut iter = IterableDlDpipeEntryActionValues::with_loc(&[], self.orig_loc);
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
                DlDpipeEntryActionValues::DpipeActionValue(val) => {
                    fmt.field("DpipeActionValue", &val)
                }
            };
        }
        fmt.finish()
    }
}
impl IterableDlDpipeEntryActionValues<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("DlDpipeEntryActionValues", offset));
            return (
                stack,
                missing_type.and_then(|t| DlDpipeEntryActionValues::attr_from_type(t)),
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
                DlDpipeEntryActionValues::DpipeActionValue(val) => {
                    (stack, missing) = val.lookup_attr(offset, missing_type);
                    if !stack.is_empty() {
                        break;
                    }
                }
                _ => {}
            };
            last_off = cur + attrs.pos;
        }
        if !stack.is_empty() {
            stack.push(("DlDpipeEntryActionValues", cur));
        }
        (stack, missing)
    }
}
#[derive(Clone)]
pub enum DlDpipeMatch {
    #[doc = "Associated type: [`DpipeMatchType`] (enum)"]
    DpipeMatchType(u32),
    #[doc = "Associated type: [`DpipeHeaderId`] (enum)"]
    DpipeHeaderId(u32),
    DpipeHeaderGlobal(u8),
    DpipeHeaderIndex(u32),
    DpipeFieldId(u32),
}
impl<'a> IterableDlDpipeMatch<'a> {
    #[doc = "Associated type: [`DpipeMatchType`] (enum)"]
    pub fn get_dpipe_match_type(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(DlDpipeMatch::DpipeMatchType(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "DlDpipeMatch",
            "DpipeMatchType",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Associated type: [`DpipeHeaderId`] (enum)"]
    pub fn get_dpipe_header_id(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(DlDpipeMatch::DpipeHeaderId(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "DlDpipeMatch",
            "DpipeHeaderId",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_dpipe_header_global(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(DlDpipeMatch::DpipeHeaderGlobal(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "DlDpipeMatch",
            "DpipeHeaderGlobal",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_dpipe_header_index(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(DlDpipeMatch::DpipeHeaderIndex(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "DlDpipeMatch",
            "DpipeHeaderIndex",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_dpipe_field_id(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(DlDpipeMatch::DpipeFieldId(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "DlDpipeMatch",
            "DpipeFieldId",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl DlDpipeMatch {
    pub fn new<'a>(buf: &'a [u8]) -> IterableDlDpipeMatch<'a> {
        IterableDlDpipeMatch::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        Devlink::attr_from_type(r#type)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableDlDpipeMatch<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableDlDpipeMatch<'a> {
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
impl<'a> Iterator for IterableDlDpipeMatch<'a> {
    type Item = Result<DlDpipeMatch, ErrorContext>;
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
                42u16 => DlDpipeMatch::DpipeMatchType({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                52u16 => DlDpipeMatch::DpipeHeaderId({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                54u16 => DlDpipeMatch::DpipeHeaderGlobal({
                    let res = parse_u8(next);
                    let Some(val) = res else { break };
                    val
                }),
                55u16 => DlDpipeMatch::DpipeHeaderIndex({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                58u16 => DlDpipeMatch::DpipeFieldId({
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
            "DlDpipeMatch",
            r#type.and_then(|t| DlDpipeMatch::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl std::fmt::Debug for IterableDlDpipeMatch<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("DlDpipeMatch");
        let mut iter = IterableDlDpipeMatch::with_loc(&[], self.orig_loc);
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
                DlDpipeMatch::DpipeMatchType(val) => fmt.field(
                    "DpipeMatchType",
                    &FormatEnum(val.into(), DpipeMatchType::from_value),
                ),
                DlDpipeMatch::DpipeHeaderId(val) => fmt.field(
                    "DpipeHeaderId",
                    &FormatEnum(val.into(), DpipeHeaderId::from_value),
                ),
                DlDpipeMatch::DpipeHeaderGlobal(val) => fmt.field("DpipeHeaderGlobal", &val),
                DlDpipeMatch::DpipeHeaderIndex(val) => fmt.field("DpipeHeaderIndex", &val),
                DlDpipeMatch::DpipeFieldId(val) => fmt.field("DpipeFieldId", &val),
            };
        }
        fmt.finish()
    }
}
impl IterableDlDpipeMatch<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("DlDpipeMatch", offset));
            return (
                stack,
                missing_type.and_then(|t| DlDpipeMatch::attr_from_type(t)),
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
                DlDpipeMatch::DpipeMatchType(val) => {
                    if last_off == offset {
                        stack.push(("DpipeMatchType", last_off));
                        break;
                    }
                }
                DlDpipeMatch::DpipeHeaderId(val) => {
                    if last_off == offset {
                        stack.push(("DpipeHeaderId", last_off));
                        break;
                    }
                }
                DlDpipeMatch::DpipeHeaderGlobal(val) => {
                    if last_off == offset {
                        stack.push(("DpipeHeaderGlobal", last_off));
                        break;
                    }
                }
                DlDpipeMatch::DpipeHeaderIndex(val) => {
                    if last_off == offset {
                        stack.push(("DpipeHeaderIndex", last_off));
                        break;
                    }
                }
                DlDpipeMatch::DpipeFieldId(val) => {
                    if last_off == offset {
                        stack.push(("DpipeFieldId", last_off));
                        break;
                    }
                }
                _ => {}
            };
            last_off = cur + attrs.pos;
        }
        if !stack.is_empty() {
            stack.push(("DlDpipeMatch", cur));
        }
        (stack, None)
    }
}
#[derive(Clone)]
pub enum DlDpipeMatchValue<'a> {
    #[doc = "Attribute may repeat multiple times (treat it as array)"]
    DpipeMatch(IterableDlDpipeMatch<'a>),
    DpipeValue(&'a [u8]),
    DpipeValueMask(&'a [u8]),
    DpipeValueMapping(u32),
}
impl<'a> IterableDlDpipeMatchValue<'a> {
    #[doc = "Attribute may repeat multiple times (treat it as array)"]
    pub fn get_dpipe_match(
        &self,
    ) -> MultiAttrIterable<Self, DlDpipeMatchValue<'a>, IterableDlDpipeMatch<'a>> {
        MultiAttrIterable::new(self.clone(), |variant| {
            if let DlDpipeMatchValue::DpipeMatch(val) = variant {
                Some(val)
            } else {
                None
            }
        })
    }
    pub fn get_dpipe_value(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(DlDpipeMatchValue::DpipeValue(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "DlDpipeMatchValue",
            "DpipeValue",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_dpipe_value_mask(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(DlDpipeMatchValue::DpipeValueMask(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "DlDpipeMatchValue",
            "DpipeValueMask",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_dpipe_value_mapping(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(DlDpipeMatchValue::DpipeValueMapping(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "DlDpipeMatchValue",
            "DpipeValueMapping",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl DlDpipeMatchValue<'_> {
    pub fn new<'a>(buf: &'a [u8]) -> IterableDlDpipeMatchValue<'a> {
        IterableDlDpipeMatchValue::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        Devlink::attr_from_type(r#type)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableDlDpipeMatchValue<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableDlDpipeMatchValue<'a> {
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
impl<'a> Iterator for IterableDlDpipeMatchValue<'a> {
    type Item = Result<DlDpipeMatchValue<'a>, ErrorContext>;
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
                40u16 => DlDpipeMatchValue::DpipeMatch({
                    let res = Some(IterableDlDpipeMatch::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                46u16 => DlDpipeMatchValue::DpipeValue({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                47u16 => DlDpipeMatchValue::DpipeValueMask({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                48u16 => DlDpipeMatchValue::DpipeValueMapping({
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
            "DlDpipeMatchValue",
            r#type.and_then(|t| DlDpipeMatchValue::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl<'a> std::fmt::Debug for IterableDlDpipeMatchValue<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("DlDpipeMatchValue");
        let mut iter = IterableDlDpipeMatchValue::with_loc(&[], self.orig_loc);
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
                DlDpipeMatchValue::DpipeMatch(val) => fmt.field("DpipeMatch", &val),
                DlDpipeMatchValue::DpipeValue(val) => fmt.field("DpipeValue", &FormatHexdump(val)),
                DlDpipeMatchValue::DpipeValueMask(val) => {
                    fmt.field("DpipeValueMask", &FormatHexdump(val))
                }
                DlDpipeMatchValue::DpipeValueMapping(val) => fmt.field("DpipeValueMapping", &val),
            };
        }
        fmt.finish()
    }
}
impl IterableDlDpipeMatchValue<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("DlDpipeMatchValue", offset));
            return (
                stack,
                missing_type.and_then(|t| DlDpipeMatchValue::attr_from_type(t)),
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
                DlDpipeMatchValue::DpipeMatch(val) => {
                    (stack, missing) = val.lookup_attr(offset, missing_type);
                    if !stack.is_empty() {
                        break;
                    }
                }
                DlDpipeMatchValue::DpipeValue(val) => {
                    if last_off == offset {
                        stack.push(("DpipeValue", last_off));
                        break;
                    }
                }
                DlDpipeMatchValue::DpipeValueMask(val) => {
                    if last_off == offset {
                        stack.push(("DpipeValueMask", last_off));
                        break;
                    }
                }
                DlDpipeMatchValue::DpipeValueMapping(val) => {
                    if last_off == offset {
                        stack.push(("DpipeValueMapping", last_off));
                        break;
                    }
                }
                _ => {}
            };
            last_off = cur + attrs.pos;
        }
        if !stack.is_empty() {
            stack.push(("DlDpipeMatchValue", cur));
        }
        (stack, missing)
    }
}
#[derive(Clone)]
pub enum DlDpipeAction {
    #[doc = "Associated type: [`DpipeActionType`] (enum)"]
    DpipeActionType(u32),
    #[doc = "Associated type: [`DpipeHeaderId`] (enum)"]
    DpipeHeaderId(u32),
    DpipeHeaderGlobal(u8),
    DpipeHeaderIndex(u32),
    DpipeFieldId(u32),
}
impl<'a> IterableDlDpipeAction<'a> {
    #[doc = "Associated type: [`DpipeActionType`] (enum)"]
    pub fn get_dpipe_action_type(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(DlDpipeAction::DpipeActionType(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "DlDpipeAction",
            "DpipeActionType",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Associated type: [`DpipeHeaderId`] (enum)"]
    pub fn get_dpipe_header_id(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(DlDpipeAction::DpipeHeaderId(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "DlDpipeAction",
            "DpipeHeaderId",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_dpipe_header_global(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(DlDpipeAction::DpipeHeaderGlobal(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "DlDpipeAction",
            "DpipeHeaderGlobal",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_dpipe_header_index(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(DlDpipeAction::DpipeHeaderIndex(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "DlDpipeAction",
            "DpipeHeaderIndex",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_dpipe_field_id(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(DlDpipeAction::DpipeFieldId(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "DlDpipeAction",
            "DpipeFieldId",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl DlDpipeAction {
    pub fn new<'a>(buf: &'a [u8]) -> IterableDlDpipeAction<'a> {
        IterableDlDpipeAction::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        Devlink::attr_from_type(r#type)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableDlDpipeAction<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableDlDpipeAction<'a> {
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
impl<'a> Iterator for IterableDlDpipeAction<'a> {
    type Item = Result<DlDpipeAction, ErrorContext>;
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
                45u16 => DlDpipeAction::DpipeActionType({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                52u16 => DlDpipeAction::DpipeHeaderId({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                54u16 => DlDpipeAction::DpipeHeaderGlobal({
                    let res = parse_u8(next);
                    let Some(val) = res else { break };
                    val
                }),
                55u16 => DlDpipeAction::DpipeHeaderIndex({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                58u16 => DlDpipeAction::DpipeFieldId({
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
            "DlDpipeAction",
            r#type.and_then(|t| DlDpipeAction::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl std::fmt::Debug for IterableDlDpipeAction<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("DlDpipeAction");
        let mut iter = IterableDlDpipeAction::with_loc(&[], self.orig_loc);
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
                DlDpipeAction::DpipeActionType(val) => fmt.field(
                    "DpipeActionType",
                    &FormatEnum(val.into(), DpipeActionType::from_value),
                ),
                DlDpipeAction::DpipeHeaderId(val) => fmt.field(
                    "DpipeHeaderId",
                    &FormatEnum(val.into(), DpipeHeaderId::from_value),
                ),
                DlDpipeAction::DpipeHeaderGlobal(val) => fmt.field("DpipeHeaderGlobal", &val),
                DlDpipeAction::DpipeHeaderIndex(val) => fmt.field("DpipeHeaderIndex", &val),
                DlDpipeAction::DpipeFieldId(val) => fmt.field("DpipeFieldId", &val),
            };
        }
        fmt.finish()
    }
}
impl IterableDlDpipeAction<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("DlDpipeAction", offset));
            return (
                stack,
                missing_type.and_then(|t| DlDpipeAction::attr_from_type(t)),
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
                DlDpipeAction::DpipeActionType(val) => {
                    if last_off == offset {
                        stack.push(("DpipeActionType", last_off));
                        break;
                    }
                }
                DlDpipeAction::DpipeHeaderId(val) => {
                    if last_off == offset {
                        stack.push(("DpipeHeaderId", last_off));
                        break;
                    }
                }
                DlDpipeAction::DpipeHeaderGlobal(val) => {
                    if last_off == offset {
                        stack.push(("DpipeHeaderGlobal", last_off));
                        break;
                    }
                }
                DlDpipeAction::DpipeHeaderIndex(val) => {
                    if last_off == offset {
                        stack.push(("DpipeHeaderIndex", last_off));
                        break;
                    }
                }
                DlDpipeAction::DpipeFieldId(val) => {
                    if last_off == offset {
                        stack.push(("DpipeFieldId", last_off));
                        break;
                    }
                }
                _ => {}
            };
            last_off = cur + attrs.pos;
        }
        if !stack.is_empty() {
            stack.push(("DlDpipeAction", cur));
        }
        (stack, None)
    }
}
#[derive(Clone)]
pub enum DlDpipeActionValue<'a> {
    #[doc = "Attribute may repeat multiple times (treat it as array)"]
    DpipeAction(IterableDlDpipeAction<'a>),
    DpipeValue(&'a [u8]),
    DpipeValueMask(&'a [u8]),
    DpipeValueMapping(u32),
}
impl<'a> IterableDlDpipeActionValue<'a> {
    #[doc = "Attribute may repeat multiple times (treat it as array)"]
    pub fn get_dpipe_action(
        &self,
    ) -> MultiAttrIterable<Self, DlDpipeActionValue<'a>, IterableDlDpipeAction<'a>> {
        MultiAttrIterable::new(self.clone(), |variant| {
            if let DlDpipeActionValue::DpipeAction(val) = variant {
                Some(val)
            } else {
                None
            }
        })
    }
    pub fn get_dpipe_value(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(DlDpipeActionValue::DpipeValue(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "DlDpipeActionValue",
            "DpipeValue",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_dpipe_value_mask(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(DlDpipeActionValue::DpipeValueMask(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "DlDpipeActionValue",
            "DpipeValueMask",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_dpipe_value_mapping(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(DlDpipeActionValue::DpipeValueMapping(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "DlDpipeActionValue",
            "DpipeValueMapping",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl DlDpipeActionValue<'_> {
    pub fn new<'a>(buf: &'a [u8]) -> IterableDlDpipeActionValue<'a> {
        IterableDlDpipeActionValue::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        Devlink::attr_from_type(r#type)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableDlDpipeActionValue<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableDlDpipeActionValue<'a> {
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
impl<'a> Iterator for IterableDlDpipeActionValue<'a> {
    type Item = Result<DlDpipeActionValue<'a>, ErrorContext>;
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
                43u16 => DlDpipeActionValue::DpipeAction({
                    let res = Some(IterableDlDpipeAction::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                46u16 => DlDpipeActionValue::DpipeValue({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                47u16 => DlDpipeActionValue::DpipeValueMask({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                48u16 => DlDpipeActionValue::DpipeValueMapping({
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
            "DlDpipeActionValue",
            r#type.and_then(|t| DlDpipeActionValue::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl<'a> std::fmt::Debug for IterableDlDpipeActionValue<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("DlDpipeActionValue");
        let mut iter = IterableDlDpipeActionValue::with_loc(&[], self.orig_loc);
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
                DlDpipeActionValue::DpipeAction(val) => fmt.field("DpipeAction", &val),
                DlDpipeActionValue::DpipeValue(val) => fmt.field("DpipeValue", &FormatHexdump(val)),
                DlDpipeActionValue::DpipeValueMask(val) => {
                    fmt.field("DpipeValueMask", &FormatHexdump(val))
                }
                DlDpipeActionValue::DpipeValueMapping(val) => fmt.field("DpipeValueMapping", &val),
            };
        }
        fmt.finish()
    }
}
impl IterableDlDpipeActionValue<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("DlDpipeActionValue", offset));
            return (
                stack,
                missing_type.and_then(|t| DlDpipeActionValue::attr_from_type(t)),
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
                DlDpipeActionValue::DpipeAction(val) => {
                    (stack, missing) = val.lookup_attr(offset, missing_type);
                    if !stack.is_empty() {
                        break;
                    }
                }
                DlDpipeActionValue::DpipeValue(val) => {
                    if last_off == offset {
                        stack.push(("DpipeValue", last_off));
                        break;
                    }
                }
                DlDpipeActionValue::DpipeValueMask(val) => {
                    if last_off == offset {
                        stack.push(("DpipeValueMask", last_off));
                        break;
                    }
                }
                DlDpipeActionValue::DpipeValueMapping(val) => {
                    if last_off == offset {
                        stack.push(("DpipeValueMapping", last_off));
                        break;
                    }
                }
                _ => {}
            };
            last_off = cur + attrs.pos;
        }
        if !stack.is_empty() {
            stack.push(("DlDpipeActionValue", cur));
        }
        (stack, missing)
    }
}
#[derive(Clone)]
pub enum DlDpipeHeaders<'a> {
    #[doc = "Attribute may repeat multiple times (treat it as array)"]
    DpipeHeader(IterableDlDpipeHeader<'a>),
}
impl<'a> IterableDlDpipeHeaders<'a> {
    #[doc = "Attribute may repeat multiple times (treat it as array)"]
    pub fn get_dpipe_header(
        &self,
    ) -> MultiAttrIterable<Self, DlDpipeHeaders<'a>, IterableDlDpipeHeader<'a>> {
        MultiAttrIterable::new(self.clone(), |variant| {
            if let DlDpipeHeaders::DpipeHeader(val) = variant {
                Some(val)
            } else {
                None
            }
        })
    }
}
impl DlDpipeHeaders<'_> {
    pub fn new<'a>(buf: &'a [u8]) -> IterableDlDpipeHeaders<'a> {
        IterableDlDpipeHeaders::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        Devlink::attr_from_type(r#type)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableDlDpipeHeaders<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableDlDpipeHeaders<'a> {
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
impl<'a> Iterator for IterableDlDpipeHeaders<'a> {
    type Item = Result<DlDpipeHeaders<'a>, ErrorContext>;
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
                50u16 => DlDpipeHeaders::DpipeHeader({
                    let res = Some(IterableDlDpipeHeader::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                n if cfg!(any(test, feature = "deny-unknown-attrs")) => break,
                n => continue,
            };
            return Some(Ok(res));
        }
        Some(Err(ErrorContext::new(
            "DlDpipeHeaders",
            r#type.and_then(|t| DlDpipeHeaders::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl<'a> std::fmt::Debug for IterableDlDpipeHeaders<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("DlDpipeHeaders");
        let mut iter = IterableDlDpipeHeaders::with_loc(&[], self.orig_loc);
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
                DlDpipeHeaders::DpipeHeader(val) => fmt.field("DpipeHeader", &val),
            };
        }
        fmt.finish()
    }
}
impl IterableDlDpipeHeaders<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("DlDpipeHeaders", offset));
            return (
                stack,
                missing_type.and_then(|t| DlDpipeHeaders::attr_from_type(t)),
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
                DlDpipeHeaders::DpipeHeader(val) => {
                    (stack, missing) = val.lookup_attr(offset, missing_type);
                    if !stack.is_empty() {
                        break;
                    }
                }
                _ => {}
            };
            last_off = cur + attrs.pos;
        }
        if !stack.is_empty() {
            stack.push(("DlDpipeHeaders", cur));
        }
        (stack, missing)
    }
}
#[derive(Clone)]
pub enum DlDpipeHeader<'a> {
    DpipeHeaderName(&'a CStr),
    #[doc = "Associated type: [`DpipeHeaderId`] (enum)"]
    DpipeHeaderId(u32),
    DpipeHeaderFields(IterableDlDpipeHeaderFields<'a>),
    DpipeHeaderGlobal(u8),
}
impl<'a> IterableDlDpipeHeader<'a> {
    pub fn get_dpipe_header_name(&self) -> Result<&'a CStr, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(DlDpipeHeader::DpipeHeaderName(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "DlDpipeHeader",
            "DpipeHeaderName",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Associated type: [`DpipeHeaderId`] (enum)"]
    pub fn get_dpipe_header_id(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(DlDpipeHeader::DpipeHeaderId(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "DlDpipeHeader",
            "DpipeHeaderId",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_dpipe_header_fields(&self) -> Result<IterableDlDpipeHeaderFields<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(DlDpipeHeader::DpipeHeaderFields(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "DlDpipeHeader",
            "DpipeHeaderFields",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_dpipe_header_global(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(DlDpipeHeader::DpipeHeaderGlobal(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "DlDpipeHeader",
            "DpipeHeaderGlobal",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl DlDpipeHeader<'_> {
    pub fn new<'a>(buf: &'a [u8]) -> IterableDlDpipeHeader<'a> {
        IterableDlDpipeHeader::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        Devlink::attr_from_type(r#type)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableDlDpipeHeader<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableDlDpipeHeader<'a> {
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
impl<'a> Iterator for IterableDlDpipeHeader<'a> {
    type Item = Result<DlDpipeHeader<'a>, ErrorContext>;
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
                51u16 => DlDpipeHeader::DpipeHeaderName({
                    let res = CStr::from_bytes_with_nul(next).ok();
                    let Some(val) = res else { break };
                    val
                }),
                52u16 => DlDpipeHeader::DpipeHeaderId({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                53u16 => DlDpipeHeader::DpipeHeaderFields({
                    let res = Some(IterableDlDpipeHeaderFields::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                54u16 => DlDpipeHeader::DpipeHeaderGlobal({
                    let res = parse_u8(next);
                    let Some(val) = res else { break };
                    val
                }),
                n if cfg!(any(test, feature = "deny-unknown-attrs")) => break,
                n => continue,
            };
            return Some(Ok(res));
        }
        Some(Err(ErrorContext::new(
            "DlDpipeHeader",
            r#type.and_then(|t| DlDpipeHeader::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl<'a> std::fmt::Debug for IterableDlDpipeHeader<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("DlDpipeHeader");
        let mut iter = IterableDlDpipeHeader::with_loc(&[], self.orig_loc);
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
                DlDpipeHeader::DpipeHeaderName(val) => fmt.field("DpipeHeaderName", &val),
                DlDpipeHeader::DpipeHeaderId(val) => fmt.field(
                    "DpipeHeaderId",
                    &FormatEnum(val.into(), DpipeHeaderId::from_value),
                ),
                DlDpipeHeader::DpipeHeaderFields(val) => fmt.field("DpipeHeaderFields", &val),
                DlDpipeHeader::DpipeHeaderGlobal(val) => fmt.field("DpipeHeaderGlobal", &val),
            };
        }
        fmt.finish()
    }
}
impl IterableDlDpipeHeader<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("DlDpipeHeader", offset));
            return (
                stack,
                missing_type.and_then(|t| DlDpipeHeader::attr_from_type(t)),
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
                DlDpipeHeader::DpipeHeaderName(val) => {
                    if last_off == offset {
                        stack.push(("DpipeHeaderName", last_off));
                        break;
                    }
                }
                DlDpipeHeader::DpipeHeaderId(val) => {
                    if last_off == offset {
                        stack.push(("DpipeHeaderId", last_off));
                        break;
                    }
                }
                DlDpipeHeader::DpipeHeaderFields(val) => {
                    (stack, missing) = val.lookup_attr(offset, missing_type);
                    if !stack.is_empty() {
                        break;
                    }
                }
                DlDpipeHeader::DpipeHeaderGlobal(val) => {
                    if last_off == offset {
                        stack.push(("DpipeHeaderGlobal", last_off));
                        break;
                    }
                }
                _ => {}
            };
            last_off = cur + attrs.pos;
        }
        if !stack.is_empty() {
            stack.push(("DlDpipeHeader", cur));
        }
        (stack, missing)
    }
}
#[derive(Clone)]
pub enum DlDpipeHeaderFields<'a> {
    #[doc = "Attribute may repeat multiple times (treat it as array)"]
    DpipeField(IterableDlDpipeField<'a>),
}
impl<'a> IterableDlDpipeHeaderFields<'a> {
    #[doc = "Attribute may repeat multiple times (treat it as array)"]
    pub fn get_dpipe_field(
        &self,
    ) -> MultiAttrIterable<Self, DlDpipeHeaderFields<'a>, IterableDlDpipeField<'a>> {
        MultiAttrIterable::new(self.clone(), |variant| {
            if let DlDpipeHeaderFields::DpipeField(val) = variant {
                Some(val)
            } else {
                None
            }
        })
    }
}
impl DlDpipeHeaderFields<'_> {
    pub fn new<'a>(buf: &'a [u8]) -> IterableDlDpipeHeaderFields<'a> {
        IterableDlDpipeHeaderFields::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        Devlink::attr_from_type(r#type)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableDlDpipeHeaderFields<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableDlDpipeHeaderFields<'a> {
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
impl<'a> Iterator for IterableDlDpipeHeaderFields<'a> {
    type Item = Result<DlDpipeHeaderFields<'a>, ErrorContext>;
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
                56u16 => DlDpipeHeaderFields::DpipeField({
                    let res = Some(IterableDlDpipeField::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                n if cfg!(any(test, feature = "deny-unknown-attrs")) => break,
                n => continue,
            };
            return Some(Ok(res));
        }
        Some(Err(ErrorContext::new(
            "DlDpipeHeaderFields",
            r#type.and_then(|t| DlDpipeHeaderFields::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl<'a> std::fmt::Debug for IterableDlDpipeHeaderFields<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("DlDpipeHeaderFields");
        let mut iter = IterableDlDpipeHeaderFields::with_loc(&[], self.orig_loc);
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
                DlDpipeHeaderFields::DpipeField(val) => fmt.field("DpipeField", &val),
            };
        }
        fmt.finish()
    }
}
impl IterableDlDpipeHeaderFields<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("DlDpipeHeaderFields", offset));
            return (
                stack,
                missing_type.and_then(|t| DlDpipeHeaderFields::attr_from_type(t)),
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
                DlDpipeHeaderFields::DpipeField(val) => {
                    (stack, missing) = val.lookup_attr(offset, missing_type);
                    if !stack.is_empty() {
                        break;
                    }
                }
                _ => {}
            };
            last_off = cur + attrs.pos;
        }
        if !stack.is_empty() {
            stack.push(("DlDpipeHeaderFields", cur));
        }
        (stack, missing)
    }
}
#[derive(Clone)]
pub enum DlDpipeField<'a> {
    DpipeFieldName(&'a CStr),
    DpipeFieldId(u32),
    DpipeFieldBitwidth(u32),
    #[doc = "Associated type: [`DpipeFieldMappingType`] (enum)"]
    DpipeFieldMappingType(u32),
}
impl<'a> IterableDlDpipeField<'a> {
    pub fn get_dpipe_field_name(&self) -> Result<&'a CStr, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(DlDpipeField::DpipeFieldName(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "DlDpipeField",
            "DpipeFieldName",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_dpipe_field_id(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(DlDpipeField::DpipeFieldId(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "DlDpipeField",
            "DpipeFieldId",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_dpipe_field_bitwidth(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(DlDpipeField::DpipeFieldBitwidth(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "DlDpipeField",
            "DpipeFieldBitwidth",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Associated type: [`DpipeFieldMappingType`] (enum)"]
    pub fn get_dpipe_field_mapping_type(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(DlDpipeField::DpipeFieldMappingType(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "DlDpipeField",
            "DpipeFieldMappingType",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl DlDpipeField<'_> {
    pub fn new<'a>(buf: &'a [u8]) -> IterableDlDpipeField<'a> {
        IterableDlDpipeField::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        Devlink::attr_from_type(r#type)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableDlDpipeField<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableDlDpipeField<'a> {
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
impl<'a> Iterator for IterableDlDpipeField<'a> {
    type Item = Result<DlDpipeField<'a>, ErrorContext>;
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
                57u16 => DlDpipeField::DpipeFieldName({
                    let res = CStr::from_bytes_with_nul(next).ok();
                    let Some(val) = res else { break };
                    val
                }),
                58u16 => DlDpipeField::DpipeFieldId({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                59u16 => DlDpipeField::DpipeFieldBitwidth({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                60u16 => DlDpipeField::DpipeFieldMappingType({
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
            "DlDpipeField",
            r#type.and_then(|t| DlDpipeField::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl<'a> std::fmt::Debug for IterableDlDpipeField<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("DlDpipeField");
        let mut iter = IterableDlDpipeField::with_loc(&[], self.orig_loc);
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
                DlDpipeField::DpipeFieldName(val) => fmt.field("DpipeFieldName", &val),
                DlDpipeField::DpipeFieldId(val) => fmt.field("DpipeFieldId", &val),
                DlDpipeField::DpipeFieldBitwidth(val) => fmt.field("DpipeFieldBitwidth", &val),
                DlDpipeField::DpipeFieldMappingType(val) => fmt.field(
                    "DpipeFieldMappingType",
                    &FormatEnum(val.into(), DpipeFieldMappingType::from_value),
                ),
            };
        }
        fmt.finish()
    }
}
impl IterableDlDpipeField<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("DlDpipeField", offset));
            return (
                stack,
                missing_type.and_then(|t| DlDpipeField::attr_from_type(t)),
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
                DlDpipeField::DpipeFieldName(val) => {
                    if last_off == offset {
                        stack.push(("DpipeFieldName", last_off));
                        break;
                    }
                }
                DlDpipeField::DpipeFieldId(val) => {
                    if last_off == offset {
                        stack.push(("DpipeFieldId", last_off));
                        break;
                    }
                }
                DlDpipeField::DpipeFieldBitwidth(val) => {
                    if last_off == offset {
                        stack.push(("DpipeFieldBitwidth", last_off));
                        break;
                    }
                }
                DlDpipeField::DpipeFieldMappingType(val) => {
                    if last_off == offset {
                        stack.push(("DpipeFieldMappingType", last_off));
                        break;
                    }
                }
                _ => {}
            };
            last_off = cur + attrs.pos;
        }
        if !stack.is_empty() {
            stack.push(("DlDpipeField", cur));
        }
        (stack, None)
    }
}
#[derive(Clone)]
pub enum DlResource<'a> {
    ResourceName(&'a CStr),
    ResourceId(u64),
    ResourceSize(u64),
    ResourceSizeNew(u64),
    ResourceSizeValid(u8),
    ResourceSizeMin(u64),
    ResourceSizeMax(u64),
    ResourceSizeGran(u64),
    #[doc = "Associated type: [`ResourceUnit`] (enum)"]
    ResourceUnit(u8),
    ResourceOcc(u64),
}
impl<'a> IterableDlResource<'a> {
    pub fn get_resource_name(&self) -> Result<&'a CStr, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(DlResource::ResourceName(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "DlResource",
            "ResourceName",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_resource_id(&self) -> Result<u64, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(DlResource::ResourceId(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "DlResource",
            "ResourceId",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_resource_size(&self) -> Result<u64, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(DlResource::ResourceSize(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "DlResource",
            "ResourceSize",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_resource_size_new(&self) -> Result<u64, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(DlResource::ResourceSizeNew(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "DlResource",
            "ResourceSizeNew",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_resource_size_valid(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(DlResource::ResourceSizeValid(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "DlResource",
            "ResourceSizeValid",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_resource_size_min(&self) -> Result<u64, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(DlResource::ResourceSizeMin(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "DlResource",
            "ResourceSizeMin",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_resource_size_max(&self) -> Result<u64, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(DlResource::ResourceSizeMax(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "DlResource",
            "ResourceSizeMax",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_resource_size_gran(&self) -> Result<u64, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(DlResource::ResourceSizeGran(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "DlResource",
            "ResourceSizeGran",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Associated type: [`ResourceUnit`] (enum)"]
    pub fn get_resource_unit(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(DlResource::ResourceUnit(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "DlResource",
            "ResourceUnit",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_resource_occ(&self) -> Result<u64, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(DlResource::ResourceOcc(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "DlResource",
            "ResourceOcc",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl DlResource<'_> {
    pub fn new<'a>(buf: &'a [u8]) -> IterableDlResource<'a> {
        IterableDlResource::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        Devlink::attr_from_type(r#type)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableDlResource<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableDlResource<'a> {
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
impl<'a> Iterator for IterableDlResource<'a> {
    type Item = Result<DlResource<'a>, ErrorContext>;
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
                65u16 => DlResource::ResourceName({
                    let res = CStr::from_bytes_with_nul(next).ok();
                    let Some(val) = res else { break };
                    val
                }),
                66u16 => DlResource::ResourceId({
                    let res = parse_u64(next);
                    let Some(val) = res else { break };
                    val
                }),
                67u16 => DlResource::ResourceSize({
                    let res = parse_u64(next);
                    let Some(val) = res else { break };
                    val
                }),
                68u16 => DlResource::ResourceSizeNew({
                    let res = parse_u64(next);
                    let Some(val) = res else { break };
                    val
                }),
                69u16 => DlResource::ResourceSizeValid({
                    let res = parse_u8(next);
                    let Some(val) = res else { break };
                    val
                }),
                70u16 => DlResource::ResourceSizeMin({
                    let res = parse_u64(next);
                    let Some(val) = res else { break };
                    val
                }),
                71u16 => DlResource::ResourceSizeMax({
                    let res = parse_u64(next);
                    let Some(val) = res else { break };
                    val
                }),
                72u16 => DlResource::ResourceSizeGran({
                    let res = parse_u64(next);
                    let Some(val) = res else { break };
                    val
                }),
                73u16 => DlResource::ResourceUnit({
                    let res = parse_u8(next);
                    let Some(val) = res else { break };
                    val
                }),
                74u16 => DlResource::ResourceOcc({
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
            "DlResource",
            r#type.and_then(|t| DlResource::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl<'a> std::fmt::Debug for IterableDlResource<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("DlResource");
        let mut iter = IterableDlResource::with_loc(&[], self.orig_loc);
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
                DlResource::ResourceName(val) => fmt.field("ResourceName", &val),
                DlResource::ResourceId(val) => fmt.field("ResourceId", &val),
                DlResource::ResourceSize(val) => fmt.field("ResourceSize", &val),
                DlResource::ResourceSizeNew(val) => fmt.field("ResourceSizeNew", &val),
                DlResource::ResourceSizeValid(val) => fmt.field("ResourceSizeValid", &val),
                DlResource::ResourceSizeMin(val) => fmt.field("ResourceSizeMin", &val),
                DlResource::ResourceSizeMax(val) => fmt.field("ResourceSizeMax", &val),
                DlResource::ResourceSizeGran(val) => fmt.field("ResourceSizeGran", &val),
                DlResource::ResourceUnit(val) => fmt.field(
                    "ResourceUnit",
                    &FormatEnum(val.into(), ResourceUnit::from_value),
                ),
                DlResource::ResourceOcc(val) => fmt.field("ResourceOcc", &val),
            };
        }
        fmt.finish()
    }
}
impl IterableDlResource<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("DlResource", offset));
            return (
                stack,
                missing_type.and_then(|t| DlResource::attr_from_type(t)),
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
                DlResource::ResourceName(val) => {
                    if last_off == offset {
                        stack.push(("ResourceName", last_off));
                        break;
                    }
                }
                DlResource::ResourceId(val) => {
                    if last_off == offset {
                        stack.push(("ResourceId", last_off));
                        break;
                    }
                }
                DlResource::ResourceSize(val) => {
                    if last_off == offset {
                        stack.push(("ResourceSize", last_off));
                        break;
                    }
                }
                DlResource::ResourceSizeNew(val) => {
                    if last_off == offset {
                        stack.push(("ResourceSizeNew", last_off));
                        break;
                    }
                }
                DlResource::ResourceSizeValid(val) => {
                    if last_off == offset {
                        stack.push(("ResourceSizeValid", last_off));
                        break;
                    }
                }
                DlResource::ResourceSizeMin(val) => {
                    if last_off == offset {
                        stack.push(("ResourceSizeMin", last_off));
                        break;
                    }
                }
                DlResource::ResourceSizeMax(val) => {
                    if last_off == offset {
                        stack.push(("ResourceSizeMax", last_off));
                        break;
                    }
                }
                DlResource::ResourceSizeGran(val) => {
                    if last_off == offset {
                        stack.push(("ResourceSizeGran", last_off));
                        break;
                    }
                }
                DlResource::ResourceUnit(val) => {
                    if last_off == offset {
                        stack.push(("ResourceUnit", last_off));
                        break;
                    }
                }
                DlResource::ResourceOcc(val) => {
                    if last_off == offset {
                        stack.push(("ResourceOcc", last_off));
                        break;
                    }
                }
                _ => {}
            };
            last_off = cur + attrs.pos;
        }
        if !stack.is_empty() {
            stack.push(("DlResource", cur));
        }
        (stack, None)
    }
}
#[derive(Clone)]
pub enum DlResourceList<'a> {
    #[doc = "Attribute may repeat multiple times (treat it as array)"]
    Resource(IterableDlResource<'a>),
}
impl<'a> IterableDlResourceList<'a> {
    #[doc = "Attribute may repeat multiple times (treat it as array)"]
    pub fn get_resource(
        &self,
    ) -> MultiAttrIterable<Self, DlResourceList<'a>, IterableDlResource<'a>> {
        MultiAttrIterable::new(self.clone(), |variant| {
            if let DlResourceList::Resource(val) = variant {
                Some(val)
            } else {
                None
            }
        })
    }
}
impl DlResourceList<'_> {
    pub fn new<'a>(buf: &'a [u8]) -> IterableDlResourceList<'a> {
        IterableDlResourceList::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        Devlink::attr_from_type(r#type)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableDlResourceList<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableDlResourceList<'a> {
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
impl<'a> Iterator for IterableDlResourceList<'a> {
    type Item = Result<DlResourceList<'a>, ErrorContext>;
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
                64u16 => DlResourceList::Resource({
                    let res = Some(IterableDlResource::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                n if cfg!(any(test, feature = "deny-unknown-attrs")) => break,
                n => continue,
            };
            return Some(Ok(res));
        }
        Some(Err(ErrorContext::new(
            "DlResourceList",
            r#type.and_then(|t| DlResourceList::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl<'a> std::fmt::Debug for IterableDlResourceList<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("DlResourceList");
        let mut iter = IterableDlResourceList::with_loc(&[], self.orig_loc);
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
                DlResourceList::Resource(val) => fmt.field("Resource", &val),
            };
        }
        fmt.finish()
    }
}
impl IterableDlResourceList<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("DlResourceList", offset));
            return (
                stack,
                missing_type.and_then(|t| DlResourceList::attr_from_type(t)),
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
                DlResourceList::Resource(val) => {
                    (stack, missing) = val.lookup_attr(offset, missing_type);
                    if !stack.is_empty() {
                        break;
                    }
                }
                _ => {}
            };
            last_off = cur + attrs.pos;
        }
        if !stack.is_empty() {
            stack.push(("DlResourceList", cur));
        }
        (stack, missing)
    }
}
#[derive(Clone)]
pub enum DlParam<'a> {
    ParamName(&'a CStr),
    ParamGeneric(()),
    #[doc = "Associated type: [`VarAttrType`] (enum)"]
    ParamType(u8),
}
impl<'a> IterableDlParam<'a> {
    pub fn get_param_name(&self) -> Result<&'a CStr, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(DlParam::ParamName(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "DlParam",
            "ParamName",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_param_generic(&self) -> Result<(), ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(DlParam::ParamGeneric(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "DlParam",
            "ParamGeneric",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Associated type: [`VarAttrType`] (enum)"]
    pub fn get_param_type(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(DlParam::ParamType(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "DlParam",
            "ParamType",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl DlParam<'_> {
    pub fn new<'a>(buf: &'a [u8]) -> IterableDlParam<'a> {
        IterableDlParam::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        Devlink::attr_from_type(r#type)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableDlParam<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableDlParam<'a> {
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
impl<'a> Iterator for IterableDlParam<'a> {
    type Item = Result<DlParam<'a>, ErrorContext>;
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
                81u16 => DlParam::ParamName({
                    let res = CStr::from_bytes_with_nul(next).ok();
                    let Some(val) = res else { break };
                    val
                }),
                82u16 => DlParam::ParamGeneric(()),
                83u16 => DlParam::ParamType({
                    let res = parse_u8(next);
                    let Some(val) = res else { break };
                    val
                }),
                n if cfg!(any(test, feature = "deny-unknown-attrs")) => break,
                n => continue,
            };
            return Some(Ok(res));
        }
        Some(Err(ErrorContext::new(
            "DlParam",
            r#type.and_then(|t| DlParam::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl<'a> std::fmt::Debug for IterableDlParam<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("DlParam");
        let mut iter = IterableDlParam::with_loc(&[], self.orig_loc);
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
                DlParam::ParamName(val) => fmt.field("ParamName", &val),
                DlParam::ParamGeneric(val) => fmt.field("ParamGeneric", &val),
                DlParam::ParamType(val) => fmt.field(
                    "ParamType",
                    &FormatEnum(val.into(), VarAttrType::from_value),
                ),
            };
        }
        fmt.finish()
    }
}
impl IterableDlParam<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("DlParam", offset));
            return (stack, missing_type.and_then(|t| DlParam::attr_from_type(t)));
        }
        if cur > offset || cur + self.buf.len() < offset {
            return (stack, None);
        }
        let mut attrs = self.clone();
        let mut last_off = cur + attrs.pos;
        while let Some(attr) = attrs.next() {
            let Ok(attr) = attr else { break };
            match attr {
                DlParam::ParamName(val) => {
                    if last_off == offset {
                        stack.push(("ParamName", last_off));
                        break;
                    }
                }
                DlParam::ParamGeneric(val) => {
                    if last_off == offset {
                        stack.push(("ParamGeneric", last_off));
                        break;
                    }
                }
                DlParam::ParamType(val) => {
                    if last_off == offset {
                        stack.push(("ParamType", last_off));
                        break;
                    }
                }
                _ => {}
            };
            last_off = cur + attrs.pos;
        }
        if !stack.is_empty() {
            stack.push(("DlParam", cur));
        }
        (stack, None)
    }
}
#[derive(Clone)]
pub enum DlRegionSnapshots<'a> {
    RegionSnapshot(IterableDlRegionSnapshot<'a>),
}
impl<'a> IterableDlRegionSnapshots<'a> {
    pub fn get_region_snapshot(&self) -> Result<IterableDlRegionSnapshot<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(DlRegionSnapshots::RegionSnapshot(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "DlRegionSnapshots",
            "RegionSnapshot",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl DlRegionSnapshots<'_> {
    pub fn new<'a>(buf: &'a [u8]) -> IterableDlRegionSnapshots<'a> {
        IterableDlRegionSnapshots::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        Devlink::attr_from_type(r#type)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableDlRegionSnapshots<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableDlRegionSnapshots<'a> {
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
impl<'a> Iterator for IterableDlRegionSnapshots<'a> {
    type Item = Result<DlRegionSnapshots<'a>, ErrorContext>;
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
                91u16 => DlRegionSnapshots::RegionSnapshot({
                    let res = Some(IterableDlRegionSnapshot::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                n if cfg!(any(test, feature = "deny-unknown-attrs")) => break,
                n => continue,
            };
            return Some(Ok(res));
        }
        Some(Err(ErrorContext::new(
            "DlRegionSnapshots",
            r#type.and_then(|t| DlRegionSnapshots::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl<'a> std::fmt::Debug for IterableDlRegionSnapshots<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("DlRegionSnapshots");
        let mut iter = IterableDlRegionSnapshots::with_loc(&[], self.orig_loc);
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
                DlRegionSnapshots::RegionSnapshot(val) => fmt.field("RegionSnapshot", &val),
            };
        }
        fmt.finish()
    }
}
impl IterableDlRegionSnapshots<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("DlRegionSnapshots", offset));
            return (
                stack,
                missing_type.and_then(|t| DlRegionSnapshots::attr_from_type(t)),
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
                DlRegionSnapshots::RegionSnapshot(val) => {
                    (stack, missing) = val.lookup_attr(offset, missing_type);
                    if !stack.is_empty() {
                        break;
                    }
                }
                _ => {}
            };
            last_off = cur + attrs.pos;
        }
        if !stack.is_empty() {
            stack.push(("DlRegionSnapshots", cur));
        }
        (stack, missing)
    }
}
#[derive(Clone)]
pub enum DlRegionSnapshot {
    RegionSnapshotId(u32),
}
impl<'a> IterableDlRegionSnapshot<'a> {
    pub fn get_region_snapshot_id(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(DlRegionSnapshot::RegionSnapshotId(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "DlRegionSnapshot",
            "RegionSnapshotId",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl DlRegionSnapshot {
    pub fn new<'a>(buf: &'a [u8]) -> IterableDlRegionSnapshot<'a> {
        IterableDlRegionSnapshot::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        Devlink::attr_from_type(r#type)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableDlRegionSnapshot<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableDlRegionSnapshot<'a> {
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
impl<'a> Iterator for IterableDlRegionSnapshot<'a> {
    type Item = Result<DlRegionSnapshot, ErrorContext>;
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
                92u16 => DlRegionSnapshot::RegionSnapshotId({
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
            "DlRegionSnapshot",
            r#type.and_then(|t| DlRegionSnapshot::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl std::fmt::Debug for IterableDlRegionSnapshot<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("DlRegionSnapshot");
        let mut iter = IterableDlRegionSnapshot::with_loc(&[], self.orig_loc);
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
                DlRegionSnapshot::RegionSnapshotId(val) => fmt.field("RegionSnapshotId", &val),
            };
        }
        fmt.finish()
    }
}
impl IterableDlRegionSnapshot<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("DlRegionSnapshot", offset));
            return (
                stack,
                missing_type.and_then(|t| DlRegionSnapshot::attr_from_type(t)),
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
                DlRegionSnapshot::RegionSnapshotId(val) => {
                    if last_off == offset {
                        stack.push(("RegionSnapshotId", last_off));
                        break;
                    }
                }
                _ => {}
            };
            last_off = cur + attrs.pos;
        }
        if !stack.is_empty() {
            stack.push(("DlRegionSnapshot", cur));
        }
        (stack, None)
    }
}
#[derive(Clone)]
pub enum DlRegionChunks<'a> {
    RegionChunk(IterableDlRegionChunk<'a>),
}
impl<'a> IterableDlRegionChunks<'a> {
    pub fn get_region_chunk(&self) -> Result<IterableDlRegionChunk<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(DlRegionChunks::RegionChunk(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "DlRegionChunks",
            "RegionChunk",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl DlRegionChunks<'_> {
    pub fn new<'a>(buf: &'a [u8]) -> IterableDlRegionChunks<'a> {
        IterableDlRegionChunks::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        Devlink::attr_from_type(r#type)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableDlRegionChunks<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableDlRegionChunks<'a> {
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
impl<'a> Iterator for IterableDlRegionChunks<'a> {
    type Item = Result<DlRegionChunks<'a>, ErrorContext>;
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
                94u16 => DlRegionChunks::RegionChunk({
                    let res = Some(IterableDlRegionChunk::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                n if cfg!(any(test, feature = "deny-unknown-attrs")) => break,
                n => continue,
            };
            return Some(Ok(res));
        }
        Some(Err(ErrorContext::new(
            "DlRegionChunks",
            r#type.and_then(|t| DlRegionChunks::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl<'a> std::fmt::Debug for IterableDlRegionChunks<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("DlRegionChunks");
        let mut iter = IterableDlRegionChunks::with_loc(&[], self.orig_loc);
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
                DlRegionChunks::RegionChunk(val) => fmt.field("RegionChunk", &val),
            };
        }
        fmt.finish()
    }
}
impl IterableDlRegionChunks<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("DlRegionChunks", offset));
            return (
                stack,
                missing_type.and_then(|t| DlRegionChunks::attr_from_type(t)),
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
                DlRegionChunks::RegionChunk(val) => {
                    (stack, missing) = val.lookup_attr(offset, missing_type);
                    if !stack.is_empty() {
                        break;
                    }
                }
                _ => {}
            };
            last_off = cur + attrs.pos;
        }
        if !stack.is_empty() {
            stack.push(("DlRegionChunks", cur));
        }
        (stack, missing)
    }
}
#[derive(Clone)]
pub enum DlRegionChunk<'a> {
    RegionChunkData(&'a [u8]),
    RegionChunkAddr(u64),
}
impl<'a> IterableDlRegionChunk<'a> {
    pub fn get_region_chunk_data(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(DlRegionChunk::RegionChunkData(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "DlRegionChunk",
            "RegionChunkData",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_region_chunk_addr(&self) -> Result<u64, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(DlRegionChunk::RegionChunkAddr(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "DlRegionChunk",
            "RegionChunkAddr",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl DlRegionChunk<'_> {
    pub fn new<'a>(buf: &'a [u8]) -> IterableDlRegionChunk<'a> {
        IterableDlRegionChunk::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        Devlink::attr_from_type(r#type)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableDlRegionChunk<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableDlRegionChunk<'a> {
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
impl<'a> Iterator for IterableDlRegionChunk<'a> {
    type Item = Result<DlRegionChunk<'a>, ErrorContext>;
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
                95u16 => DlRegionChunk::RegionChunkData({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                96u16 => DlRegionChunk::RegionChunkAddr({
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
            "DlRegionChunk",
            r#type.and_then(|t| DlRegionChunk::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl<'a> std::fmt::Debug for IterableDlRegionChunk<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("DlRegionChunk");
        let mut iter = IterableDlRegionChunk::with_loc(&[], self.orig_loc);
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
                DlRegionChunk::RegionChunkData(val) => {
                    fmt.field("RegionChunkData", &FormatHexdump(val))
                }
                DlRegionChunk::RegionChunkAddr(val) => fmt.field("RegionChunkAddr", &val),
            };
        }
        fmt.finish()
    }
}
impl IterableDlRegionChunk<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("DlRegionChunk", offset));
            return (
                stack,
                missing_type.and_then(|t| DlRegionChunk::attr_from_type(t)),
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
                DlRegionChunk::RegionChunkData(val) => {
                    if last_off == offset {
                        stack.push(("RegionChunkData", last_off));
                        break;
                    }
                }
                DlRegionChunk::RegionChunkAddr(val) => {
                    if last_off == offset {
                        stack.push(("RegionChunkAddr", last_off));
                        break;
                    }
                }
                _ => {}
            };
            last_off = cur + attrs.pos;
        }
        if !stack.is_empty() {
            stack.push(("DlRegionChunk", cur));
        }
        (stack, None)
    }
}
#[derive(Clone)]
pub enum DlFmsg<'a> {
    FmsgObjNestStart(()),
    FmsgPairNestStart(()),
    FmsgArrNestStart(()),
    FmsgNestEnd(()),
    FmsgObjName(&'a CStr),
}
impl<'a> IterableDlFmsg<'a> {
    pub fn get_fmsg_obj_nest_start(&self) -> Result<(), ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(DlFmsg::FmsgObjNestStart(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "DlFmsg",
            "FmsgObjNestStart",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_fmsg_pair_nest_start(&self) -> Result<(), ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(DlFmsg::FmsgPairNestStart(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "DlFmsg",
            "FmsgPairNestStart",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_fmsg_arr_nest_start(&self) -> Result<(), ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(DlFmsg::FmsgArrNestStart(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "DlFmsg",
            "FmsgArrNestStart",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_fmsg_nest_end(&self) -> Result<(), ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(DlFmsg::FmsgNestEnd(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "DlFmsg",
            "FmsgNestEnd",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_fmsg_obj_name(&self) -> Result<&'a CStr, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(DlFmsg::FmsgObjName(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "DlFmsg",
            "FmsgObjName",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl DlFmsg<'_> {
    pub fn new<'a>(buf: &'a [u8]) -> IterableDlFmsg<'a> {
        IterableDlFmsg::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        Devlink::attr_from_type(r#type)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableDlFmsg<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableDlFmsg<'a> {
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
impl<'a> Iterator for IterableDlFmsg<'a> {
    type Item = Result<DlFmsg<'a>, ErrorContext>;
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
                107u16 => DlFmsg::FmsgObjNestStart(()),
                108u16 => DlFmsg::FmsgPairNestStart(()),
                109u16 => DlFmsg::FmsgArrNestStart(()),
                110u16 => DlFmsg::FmsgNestEnd(()),
                111u16 => DlFmsg::FmsgObjName({
                    let res = CStr::from_bytes_with_nul(next).ok();
                    let Some(val) = res else { break };
                    val
                }),
                n if cfg!(any(test, feature = "deny-unknown-attrs")) => break,
                n => continue,
            };
            return Some(Ok(res));
        }
        Some(Err(ErrorContext::new(
            "DlFmsg",
            r#type.and_then(|t| DlFmsg::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl<'a> std::fmt::Debug for IterableDlFmsg<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("DlFmsg");
        let mut iter = IterableDlFmsg::with_loc(&[], self.orig_loc);
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
                DlFmsg::FmsgObjNestStart(val) => fmt.field("FmsgObjNestStart", &val),
                DlFmsg::FmsgPairNestStart(val) => fmt.field("FmsgPairNestStart", &val),
                DlFmsg::FmsgArrNestStart(val) => fmt.field("FmsgArrNestStart", &val),
                DlFmsg::FmsgNestEnd(val) => fmt.field("FmsgNestEnd", &val),
                DlFmsg::FmsgObjName(val) => fmt.field("FmsgObjName", &val),
            };
        }
        fmt.finish()
    }
}
impl IterableDlFmsg<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("DlFmsg", offset));
            return (stack, missing_type.and_then(|t| DlFmsg::attr_from_type(t)));
        }
        if cur > offset || cur + self.buf.len() < offset {
            return (stack, None);
        }
        let mut attrs = self.clone();
        let mut last_off = cur + attrs.pos;
        while let Some(attr) = attrs.next() {
            let Ok(attr) = attr else { break };
            match attr {
                DlFmsg::FmsgObjNestStart(val) => {
                    if last_off == offset {
                        stack.push(("FmsgObjNestStart", last_off));
                        break;
                    }
                }
                DlFmsg::FmsgPairNestStart(val) => {
                    if last_off == offset {
                        stack.push(("FmsgPairNestStart", last_off));
                        break;
                    }
                }
                DlFmsg::FmsgArrNestStart(val) => {
                    if last_off == offset {
                        stack.push(("FmsgArrNestStart", last_off));
                        break;
                    }
                }
                DlFmsg::FmsgNestEnd(val) => {
                    if last_off == offset {
                        stack.push(("FmsgNestEnd", last_off));
                        break;
                    }
                }
                DlFmsg::FmsgObjName(val) => {
                    if last_off == offset {
                        stack.push(("FmsgObjName", last_off));
                        break;
                    }
                }
                _ => {}
            };
            last_off = cur + attrs.pos;
        }
        if !stack.is_empty() {
            stack.push(("DlFmsg", cur));
        }
        (stack, None)
    }
}
#[derive(Clone)]
pub enum DlHealthReporter<'a> {
    HealthReporterName(&'a CStr),
    HealthReporterState(u8),
    HealthReporterErrCount(u64),
    HealthReporterRecoverCount(u64),
    HealthReporterDumpTs(u64),
    HealthReporterGracefulPeriod(u64),
    HealthReporterAutoRecover(u8),
    HealthReporterDumpTsNs(u64),
    HealthReporterAutoDump(u8),
    #[doc = "Time (in msec) for recoveries before starting the grace period.\n"]
    HealthReporterBurstPeriod(u64),
}
impl<'a> IterableDlHealthReporter<'a> {
    pub fn get_health_reporter_name(&self) -> Result<&'a CStr, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(DlHealthReporter::HealthReporterName(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "DlHealthReporter",
            "HealthReporterName",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_health_reporter_state(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(DlHealthReporter::HealthReporterState(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "DlHealthReporter",
            "HealthReporterState",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_health_reporter_err_count(&self) -> Result<u64, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(DlHealthReporter::HealthReporterErrCount(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "DlHealthReporter",
            "HealthReporterErrCount",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_health_reporter_recover_count(&self) -> Result<u64, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(DlHealthReporter::HealthReporterRecoverCount(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "DlHealthReporter",
            "HealthReporterRecoverCount",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_health_reporter_dump_ts(&self) -> Result<u64, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(DlHealthReporter::HealthReporterDumpTs(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "DlHealthReporter",
            "HealthReporterDumpTs",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_health_reporter_graceful_period(&self) -> Result<u64, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(DlHealthReporter::HealthReporterGracefulPeriod(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "DlHealthReporter",
            "HealthReporterGracefulPeriod",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_health_reporter_auto_recover(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(DlHealthReporter::HealthReporterAutoRecover(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "DlHealthReporter",
            "HealthReporterAutoRecover",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_health_reporter_dump_ts_ns(&self) -> Result<u64, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(DlHealthReporter::HealthReporterDumpTsNs(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "DlHealthReporter",
            "HealthReporterDumpTsNs",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_health_reporter_auto_dump(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(DlHealthReporter::HealthReporterAutoDump(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "DlHealthReporter",
            "HealthReporterAutoDump",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Time (in msec) for recoveries before starting the grace period.\n"]
    pub fn get_health_reporter_burst_period(&self) -> Result<u64, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(DlHealthReporter::HealthReporterBurstPeriod(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "DlHealthReporter",
            "HealthReporterBurstPeriod",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl DlHealthReporter<'_> {
    pub fn new<'a>(buf: &'a [u8]) -> IterableDlHealthReporter<'a> {
        IterableDlHealthReporter::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        Devlink::attr_from_type(r#type)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableDlHealthReporter<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableDlHealthReporter<'a> {
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
impl<'a> Iterator for IterableDlHealthReporter<'a> {
    type Item = Result<DlHealthReporter<'a>, ErrorContext>;
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
                115u16 => DlHealthReporter::HealthReporterName({
                    let res = CStr::from_bytes_with_nul(next).ok();
                    let Some(val) = res else { break };
                    val
                }),
                116u16 => DlHealthReporter::HealthReporterState({
                    let res = parse_u8(next);
                    let Some(val) = res else { break };
                    val
                }),
                117u16 => DlHealthReporter::HealthReporterErrCount({
                    let res = parse_u64(next);
                    let Some(val) = res else { break };
                    val
                }),
                118u16 => DlHealthReporter::HealthReporterRecoverCount({
                    let res = parse_u64(next);
                    let Some(val) = res else { break };
                    val
                }),
                119u16 => DlHealthReporter::HealthReporterDumpTs({
                    let res = parse_u64(next);
                    let Some(val) = res else { break };
                    val
                }),
                120u16 => DlHealthReporter::HealthReporterGracefulPeriod({
                    let res = parse_u64(next);
                    let Some(val) = res else { break };
                    val
                }),
                121u16 => DlHealthReporter::HealthReporterAutoRecover({
                    let res = parse_u8(next);
                    let Some(val) = res else { break };
                    val
                }),
                137u16 => DlHealthReporter::HealthReporterDumpTsNs({
                    let res = parse_u64(next);
                    let Some(val) = res else { break };
                    val
                }),
                141u16 => DlHealthReporter::HealthReporterAutoDump({
                    let res = parse_u8(next);
                    let Some(val) = res else { break };
                    val
                }),
                181u16 => DlHealthReporter::HealthReporterBurstPeriod({
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
            "DlHealthReporter",
            r#type.and_then(|t| DlHealthReporter::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl<'a> std::fmt::Debug for IterableDlHealthReporter<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("DlHealthReporter");
        let mut iter = IterableDlHealthReporter::with_loc(&[], self.orig_loc);
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
                DlHealthReporter::HealthReporterName(val) => fmt.field("HealthReporterName", &val),
                DlHealthReporter::HealthReporterState(val) => {
                    fmt.field("HealthReporterState", &val)
                }
                DlHealthReporter::HealthReporterErrCount(val) => {
                    fmt.field("HealthReporterErrCount", &val)
                }
                DlHealthReporter::HealthReporterRecoverCount(val) => {
                    fmt.field("HealthReporterRecoverCount", &val)
                }
                DlHealthReporter::HealthReporterDumpTs(val) => {
                    fmt.field("HealthReporterDumpTs", &val)
                }
                DlHealthReporter::HealthReporterGracefulPeriod(val) => {
                    fmt.field("HealthReporterGracefulPeriod", &val)
                }
                DlHealthReporter::HealthReporterAutoRecover(val) => {
                    fmt.field("HealthReporterAutoRecover", &val)
                }
                DlHealthReporter::HealthReporterDumpTsNs(val) => {
                    fmt.field("HealthReporterDumpTsNs", &val)
                }
                DlHealthReporter::HealthReporterAutoDump(val) => {
                    fmt.field("HealthReporterAutoDump", &val)
                }
                DlHealthReporter::HealthReporterBurstPeriod(val) => {
                    fmt.field("HealthReporterBurstPeriod", &val)
                }
            };
        }
        fmt.finish()
    }
}
impl IterableDlHealthReporter<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("DlHealthReporter", offset));
            return (
                stack,
                missing_type.and_then(|t| DlHealthReporter::attr_from_type(t)),
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
                DlHealthReporter::HealthReporterName(val) => {
                    if last_off == offset {
                        stack.push(("HealthReporterName", last_off));
                        break;
                    }
                }
                DlHealthReporter::HealthReporterState(val) => {
                    if last_off == offset {
                        stack.push(("HealthReporterState", last_off));
                        break;
                    }
                }
                DlHealthReporter::HealthReporterErrCount(val) => {
                    if last_off == offset {
                        stack.push(("HealthReporterErrCount", last_off));
                        break;
                    }
                }
                DlHealthReporter::HealthReporterRecoverCount(val) => {
                    if last_off == offset {
                        stack.push(("HealthReporterRecoverCount", last_off));
                        break;
                    }
                }
                DlHealthReporter::HealthReporterDumpTs(val) => {
                    if last_off == offset {
                        stack.push(("HealthReporterDumpTs", last_off));
                        break;
                    }
                }
                DlHealthReporter::HealthReporterGracefulPeriod(val) => {
                    if last_off == offset {
                        stack.push(("HealthReporterGracefulPeriod", last_off));
                        break;
                    }
                }
                DlHealthReporter::HealthReporterAutoRecover(val) => {
                    if last_off == offset {
                        stack.push(("HealthReporterAutoRecover", last_off));
                        break;
                    }
                }
                DlHealthReporter::HealthReporterDumpTsNs(val) => {
                    if last_off == offset {
                        stack.push(("HealthReporterDumpTsNs", last_off));
                        break;
                    }
                }
                DlHealthReporter::HealthReporterAutoDump(val) => {
                    if last_off == offset {
                        stack.push(("HealthReporterAutoDump", last_off));
                        break;
                    }
                }
                DlHealthReporter::HealthReporterBurstPeriod(val) => {
                    if last_off == offset {
                        stack.push(("HealthReporterBurstPeriod", last_off));
                        break;
                    }
                }
                _ => {}
            };
            last_off = cur + attrs.pos;
        }
        if !stack.is_empty() {
            stack.push(("DlHealthReporter", cur));
        }
        (stack, None)
    }
}
#[derive(Clone)]
pub enum DlAttrStats {
    StatsRxPackets(u64),
    StatsRxBytes(u64),
    StatsRxDropped(u64),
}
impl<'a> IterableDlAttrStats<'a> {
    pub fn get_stats_rx_packets(&self) -> Result<u64, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(DlAttrStats::StatsRxPackets(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "DlAttrStats",
            "StatsRxPackets",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_stats_rx_bytes(&self) -> Result<u64, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(DlAttrStats::StatsRxBytes(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "DlAttrStats",
            "StatsRxBytes",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_stats_rx_dropped(&self) -> Result<u64, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(DlAttrStats::StatsRxDropped(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "DlAttrStats",
            "StatsRxDropped",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl DlAttrStats {
    pub fn new<'a>(buf: &'a [u8]) -> IterableDlAttrStats<'a> {
        IterableDlAttrStats::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        let res = match r#type {
            0u16 => "StatsRxPackets",
            1u16 => "StatsRxBytes",
            2u16 => "StatsRxDropped",
            _ => return None,
        };
        Some(res)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableDlAttrStats<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableDlAttrStats<'a> {
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
impl<'a> Iterator for IterableDlAttrStats<'a> {
    type Item = Result<DlAttrStats, ErrorContext>;
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
                0u16 => DlAttrStats::StatsRxPackets({
                    let res = parse_u64(next);
                    let Some(val) = res else { break };
                    val
                }),
                1u16 => DlAttrStats::StatsRxBytes({
                    let res = parse_u64(next);
                    let Some(val) = res else { break };
                    val
                }),
                2u16 => DlAttrStats::StatsRxDropped({
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
            "DlAttrStats",
            r#type.and_then(|t| DlAttrStats::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl std::fmt::Debug for IterableDlAttrStats<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("DlAttrStats");
        let mut iter = IterableDlAttrStats::with_loc(&[], self.orig_loc);
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
                DlAttrStats::StatsRxPackets(val) => fmt.field("StatsRxPackets", &val),
                DlAttrStats::StatsRxBytes(val) => fmt.field("StatsRxBytes", &val),
                DlAttrStats::StatsRxDropped(val) => fmt.field("StatsRxDropped", &val),
            };
        }
        fmt.finish()
    }
}
impl IterableDlAttrStats<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("DlAttrStats", offset));
            return (
                stack,
                missing_type.and_then(|t| DlAttrStats::attr_from_type(t)),
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
                DlAttrStats::StatsRxPackets(val) => {
                    if last_off == offset {
                        stack.push(("StatsRxPackets", last_off));
                        break;
                    }
                }
                DlAttrStats::StatsRxBytes(val) => {
                    if last_off == offset {
                        stack.push(("StatsRxBytes", last_off));
                        break;
                    }
                }
                DlAttrStats::StatsRxDropped(val) => {
                    if last_off == offset {
                        stack.push(("StatsRxDropped", last_off));
                        break;
                    }
                }
                _ => {}
            };
            last_off = cur + attrs.pos;
        }
        if !stack.is_empty() {
            stack.push(("DlAttrStats", cur));
        }
        (stack, None)
    }
}
#[derive(Clone)]
pub enum DlTrapMetadata {
    TrapMetadataTypeInPort(()),
    TrapMetadataTypeFaCookie(()),
}
impl<'a> IterableDlTrapMetadata<'a> {
    pub fn get_trap_metadata_type_in_port(&self) -> Result<(), ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(DlTrapMetadata::TrapMetadataTypeInPort(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "DlTrapMetadata",
            "TrapMetadataTypeInPort",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_trap_metadata_type_fa_cookie(&self) -> Result<(), ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(DlTrapMetadata::TrapMetadataTypeFaCookie(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "DlTrapMetadata",
            "TrapMetadataTypeFaCookie",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl DlTrapMetadata {
    pub fn new<'a>(buf: &'a [u8]) -> IterableDlTrapMetadata<'a> {
        IterableDlTrapMetadata::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        let res = match r#type {
            0u16 => "TrapMetadataTypeInPort",
            1u16 => "TrapMetadataTypeFaCookie",
            _ => return None,
        };
        Some(res)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableDlTrapMetadata<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableDlTrapMetadata<'a> {
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
impl<'a> Iterator for IterableDlTrapMetadata<'a> {
    type Item = Result<DlTrapMetadata, ErrorContext>;
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
                0u16 => DlTrapMetadata::TrapMetadataTypeInPort(()),
                1u16 => DlTrapMetadata::TrapMetadataTypeFaCookie(()),
                n if cfg!(any(test, feature = "deny-unknown-attrs")) => break,
                n => continue,
            };
            return Some(Ok(res));
        }
        Some(Err(ErrorContext::new(
            "DlTrapMetadata",
            r#type.and_then(|t| DlTrapMetadata::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl std::fmt::Debug for IterableDlTrapMetadata<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("DlTrapMetadata");
        let mut iter = IterableDlTrapMetadata::with_loc(&[], self.orig_loc);
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
                DlTrapMetadata::TrapMetadataTypeInPort(val) => {
                    fmt.field("TrapMetadataTypeInPort", &val)
                }
                DlTrapMetadata::TrapMetadataTypeFaCookie(val) => {
                    fmt.field("TrapMetadataTypeFaCookie", &val)
                }
            };
        }
        fmt.finish()
    }
}
impl IterableDlTrapMetadata<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("DlTrapMetadata", offset));
            return (
                stack,
                missing_type.and_then(|t| DlTrapMetadata::attr_from_type(t)),
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
                DlTrapMetadata::TrapMetadataTypeInPort(val) => {
                    if last_off == offset {
                        stack.push(("TrapMetadataTypeInPort", last_off));
                        break;
                    }
                }
                DlTrapMetadata::TrapMetadataTypeFaCookie(val) => {
                    if last_off == offset {
                        stack.push(("TrapMetadataTypeFaCookie", last_off));
                        break;
                    }
                }
                _ => {}
            };
            last_off = cur + attrs.pos;
        }
        if !stack.is_empty() {
            stack.push(("DlTrapMetadata", cur));
        }
        (stack, None)
    }
}
#[derive(Clone)]
pub enum DlLinecardSupportedTypes<'a> {
    LinecardType(&'a CStr),
}
impl<'a> IterableDlLinecardSupportedTypes<'a> {
    pub fn get_linecard_type(&self) -> Result<&'a CStr, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(DlLinecardSupportedTypes::LinecardType(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "DlLinecardSupportedTypes",
            "LinecardType",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl DlLinecardSupportedTypes<'_> {
    pub fn new<'a>(buf: &'a [u8]) -> IterableDlLinecardSupportedTypes<'a> {
        IterableDlLinecardSupportedTypes::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        Devlink::attr_from_type(r#type)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableDlLinecardSupportedTypes<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableDlLinecardSupportedTypes<'a> {
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
impl<'a> Iterator for IterableDlLinecardSupportedTypes<'a> {
    type Item = Result<DlLinecardSupportedTypes<'a>, ErrorContext>;
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
                173u16 => DlLinecardSupportedTypes::LinecardType({
                    let res = CStr::from_bytes_with_nul(next).ok();
                    let Some(val) = res else { break };
                    val
                }),
                n if cfg!(any(test, feature = "deny-unknown-attrs")) => break,
                n => continue,
            };
            return Some(Ok(res));
        }
        Some(Err(ErrorContext::new(
            "DlLinecardSupportedTypes",
            r#type.and_then(|t| DlLinecardSupportedTypes::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl<'a> std::fmt::Debug for IterableDlLinecardSupportedTypes<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("DlLinecardSupportedTypes");
        let mut iter = IterableDlLinecardSupportedTypes::with_loc(&[], self.orig_loc);
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
                DlLinecardSupportedTypes::LinecardType(val) => fmt.field("LinecardType", &val),
            };
        }
        fmt.finish()
    }
}
impl IterableDlLinecardSupportedTypes<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("DlLinecardSupportedTypes", offset));
            return (
                stack,
                missing_type.and_then(|t| DlLinecardSupportedTypes::attr_from_type(t)),
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
                DlLinecardSupportedTypes::LinecardType(val) => {
                    if last_off == offset {
                        stack.push(("LinecardType", last_off));
                        break;
                    }
                }
                _ => {}
            };
            last_off = cur + attrs.pos;
        }
        if !stack.is_empty() {
            stack.push(("DlLinecardSupportedTypes", cur));
        }
        (stack, None)
    }
}
#[derive(Clone)]
pub enum DlSelftestId {
    Flash(()),
}
impl<'a> IterableDlSelftestId<'a> {
    pub fn get_flash(&self) -> Result<(), ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(DlSelftestId::Flash(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "DlSelftestId",
            "Flash",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl DlSelftestId {
    pub fn new<'a>(buf: &'a [u8]) -> IterableDlSelftestId<'a> {
        IterableDlSelftestId::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        let res = match r#type {
            1u16 => "Flash",
            _ => return None,
        };
        Some(res)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableDlSelftestId<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableDlSelftestId<'a> {
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
impl<'a> Iterator for IterableDlSelftestId<'a> {
    type Item = Result<DlSelftestId, ErrorContext>;
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
                1u16 => DlSelftestId::Flash(()),
                n if cfg!(any(test, feature = "deny-unknown-attrs")) => break,
                n => continue,
            };
            return Some(Ok(res));
        }
        Some(Err(ErrorContext::new(
            "DlSelftestId",
            r#type.and_then(|t| DlSelftestId::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl std::fmt::Debug for IterableDlSelftestId<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("DlSelftestId");
        let mut iter = IterableDlSelftestId::with_loc(&[], self.orig_loc);
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
                DlSelftestId::Flash(val) => fmt.field("Flash", &val),
            };
        }
        fmt.finish()
    }
}
impl IterableDlSelftestId<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("DlSelftestId", offset));
            return (
                stack,
                missing_type.and_then(|t| DlSelftestId::attr_from_type(t)),
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
                DlSelftestId::Flash(val) => {
                    if last_off == offset {
                        stack.push(("Flash", last_off));
                        break;
                    }
                }
                _ => {}
            };
            last_off = cur + attrs.pos;
        }
        if !stack.is_empty() {
            stack.push(("DlSelftestId", cur));
        }
        (stack, None)
    }
}
#[derive(Clone)]
pub enum DlRateTcBws {
    Index(u8),
    #[doc = "Specifies the bandwidth share assigned to the Traffic Class. The\nbandwidth for the traffic class is determined in proportion to the sum\nof the shares of all configured classes.\n"]
    Bw(u32),
}
impl<'a> IterableDlRateTcBws<'a> {
    pub fn get_index(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(DlRateTcBws::Index(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "DlRateTcBws",
            "Index",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Specifies the bandwidth share assigned to the Traffic Class. The\nbandwidth for the traffic class is determined in proportion to the sum\nof the shares of all configured classes.\n"]
    pub fn get_bw(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(DlRateTcBws::Bw(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "DlRateTcBws",
            "Bw",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl DlRateTcBws {
    pub fn new<'a>(buf: &'a [u8]) -> IterableDlRateTcBws<'a> {
        IterableDlRateTcBws::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        let res = match r#type {
            1u16 => "Index",
            2u16 => "Bw",
            _ => return None,
        };
        Some(res)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableDlRateTcBws<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableDlRateTcBws<'a> {
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
impl<'a> Iterator for IterableDlRateTcBws<'a> {
    type Item = Result<DlRateTcBws, ErrorContext>;
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
                1u16 => DlRateTcBws::Index({
                    let res = parse_u8(next);
                    let Some(val) = res else { break };
                    val
                }),
                2u16 => DlRateTcBws::Bw({
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
            "DlRateTcBws",
            r#type.and_then(|t| DlRateTcBws::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl std::fmt::Debug for IterableDlRateTcBws<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("DlRateTcBws");
        let mut iter = IterableDlRateTcBws::with_loc(&[], self.orig_loc);
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
                DlRateTcBws::Index(val) => fmt.field("Index", &val),
                DlRateTcBws::Bw(val) => fmt.field("Bw", &val),
            };
        }
        fmt.finish()
    }
}
impl IterableDlRateTcBws<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("DlRateTcBws", offset));
            return (
                stack,
                missing_type.and_then(|t| DlRateTcBws::attr_from_type(t)),
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
                DlRateTcBws::Index(val) => {
                    if last_off == offset {
                        stack.push(("Index", last_off));
                        break;
                    }
                }
                DlRateTcBws::Bw(val) => {
                    if last_off == offset {
                        stack.push(("Bw", last_off));
                        break;
                    }
                }
                _ => {}
            };
            last_off = cur + attrs.pos;
        }
        if !stack.is_empty() {
            stack.push(("DlRateTcBws", cur));
        }
        (stack, None)
    }
}
pub struct PushDevlink<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushDevlink<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushDevlink<Prev> {
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
    pub fn push_bus_name(mut self, value: &CStr) -> Self {
        push_header(
            self.as_vec_mut(),
            1u16,
            value.to_bytes_with_nul().len() as u16,
        );
        self.as_vec_mut().extend(value.to_bytes_with_nul());
        self
    }
    pub fn push_bus_name_bytes(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 1u16, (value.len() + 1) as u16);
        self.as_vec_mut().extend(value);
        self.as_vec_mut().push(0);
        self
    }
    pub fn push_dev_name(mut self, value: &CStr) -> Self {
        push_header(
            self.as_vec_mut(),
            2u16,
            value.to_bytes_with_nul().len() as u16,
        );
        self.as_vec_mut().extend(value.to_bytes_with_nul());
        self
    }
    pub fn push_dev_name_bytes(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 2u16, (value.len() + 1) as u16);
        self.as_vec_mut().extend(value);
        self.as_vec_mut().push(0);
        self
    }
    pub fn push_port_index(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 3u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Associated type: [`PortType`] (enum)"]
    pub fn push_port_type(mut self, value: u16) -> Self {
        push_header(self.as_vec_mut(), 4u16, 2 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_port_desired_type(mut self, value: u16) -> Self {
        push_header(self.as_vec_mut(), 5u16, 2 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_port_netdev_ifindex(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 6u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_port_netdev_name(mut self, value: &CStr) -> Self {
        push_header(
            self.as_vec_mut(),
            7u16,
            value.to_bytes_with_nul().len() as u16,
        );
        self.as_vec_mut().extend(value.to_bytes_with_nul());
        self
    }
    pub fn push_port_netdev_name_bytes(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 7u16, (value.len() + 1) as u16);
        self.as_vec_mut().extend(value);
        self.as_vec_mut().push(0);
        self
    }
    pub fn push_port_ibdev_name(mut self, value: &CStr) -> Self {
        push_header(
            self.as_vec_mut(),
            8u16,
            value.to_bytes_with_nul().len() as u16,
        );
        self.as_vec_mut().extend(value.to_bytes_with_nul());
        self
    }
    pub fn push_port_ibdev_name_bytes(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 8u16, (value.len() + 1) as u16);
        self.as_vec_mut().extend(value);
        self.as_vec_mut().push(0);
        self
    }
    pub fn push_port_split_count(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 9u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_port_split_group(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 10u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_sb_index(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 11u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_sb_size(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 12u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_sb_ingress_pool_count(mut self, value: u16) -> Self {
        push_header(self.as_vec_mut(), 13u16, 2 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_sb_egress_pool_count(mut self, value: u16) -> Self {
        push_header(self.as_vec_mut(), 14u16, 2 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_sb_ingress_tc_count(mut self, value: u16) -> Self {
        push_header(self.as_vec_mut(), 15u16, 2 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_sb_egress_tc_count(mut self, value: u16) -> Self {
        push_header(self.as_vec_mut(), 16u16, 2 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_sb_pool_index(mut self, value: u16) -> Self {
        push_header(self.as_vec_mut(), 17u16, 2 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Associated type: [`SbPoolType`] (enum)"]
    pub fn push_sb_pool_type(mut self, value: u8) -> Self {
        push_header(self.as_vec_mut(), 18u16, 1 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_sb_pool_size(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 19u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Associated type: [`SbThresholdType`] (enum)"]
    pub fn push_sb_pool_threshold_type(mut self, value: u8) -> Self {
        push_header(self.as_vec_mut(), 20u16, 1 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_sb_threshold(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 21u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_sb_tc_index(mut self, value: u16) -> Self {
        push_header(self.as_vec_mut(), 22u16, 2 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_sb_occ_cur(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 23u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_sb_occ_max(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 24u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Associated type: [`EswitchMode`] (enum)"]
    pub fn push_eswitch_mode(mut self, value: u16) -> Self {
        push_header(self.as_vec_mut(), 25u16, 2 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Associated type: [`EswitchInlineMode`] (enum)"]
    pub fn push_eswitch_inline_mode(mut self, value: u8) -> Self {
        push_header(self.as_vec_mut(), 26u16, 1 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn nested_dpipe_tables(mut self) -> PushDlDpipeTables<Self> {
        let header_offset = push_nested_header(self.as_vec_mut(), 27u16);
        PushDlDpipeTables {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
    #[doc = "Attribute may repeat multiple times (treat it as array)"]
    pub fn nested_dpipe_table(mut self) -> PushDlDpipeTable<Self> {
        let header_offset = push_nested_header(self.as_vec_mut(), 28u16);
        PushDlDpipeTable {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
    pub fn push_dpipe_table_name(mut self, value: &CStr) -> Self {
        push_header(
            self.as_vec_mut(),
            29u16,
            value.to_bytes_with_nul().len() as u16,
        );
        self.as_vec_mut().extend(value.to_bytes_with_nul());
        self
    }
    pub fn push_dpipe_table_name_bytes(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 29u16, (value.len() + 1) as u16);
        self.as_vec_mut().extend(value);
        self.as_vec_mut().push(0);
        self
    }
    pub fn push_dpipe_table_size(mut self, value: u64) -> Self {
        push_header(self.as_vec_mut(), 30u16, 8 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn nested_dpipe_table_matches(mut self) -> PushDlDpipeTableMatches<Self> {
        let header_offset = push_nested_header(self.as_vec_mut(), 31u16);
        PushDlDpipeTableMatches {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
    pub fn nested_dpipe_table_actions(mut self) -> PushDlDpipeTableActions<Self> {
        let header_offset = push_nested_header(self.as_vec_mut(), 32u16);
        PushDlDpipeTableActions {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
    pub fn push_dpipe_table_counters_enabled(mut self, value: u8) -> Self {
        push_header(self.as_vec_mut(), 33u16, 1 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn nested_dpipe_entries(mut self) -> PushDlDpipeEntries<Self> {
        let header_offset = push_nested_header(self.as_vec_mut(), 34u16);
        PushDlDpipeEntries {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
    #[doc = "Attribute may repeat multiple times (treat it as array)"]
    pub fn nested_dpipe_entry(mut self) -> PushDlDpipeEntry<Self> {
        let header_offset = push_nested_header(self.as_vec_mut(), 35u16);
        PushDlDpipeEntry {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
    pub fn push_dpipe_entry_index(mut self, value: u64) -> Self {
        push_header(self.as_vec_mut(), 36u16, 8 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn nested_dpipe_entry_match_values(mut self) -> PushDlDpipeEntryMatchValues<Self> {
        let header_offset = push_nested_header(self.as_vec_mut(), 37u16);
        PushDlDpipeEntryMatchValues {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
    pub fn nested_dpipe_entry_action_values(mut self) -> PushDlDpipeEntryActionValues<Self> {
        let header_offset = push_nested_header(self.as_vec_mut(), 38u16);
        PushDlDpipeEntryActionValues {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
    pub fn push_dpipe_entry_counter(mut self, value: u64) -> Self {
        push_header(self.as_vec_mut(), 39u16, 8 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Attribute may repeat multiple times (treat it as array)"]
    pub fn nested_dpipe_match(mut self) -> PushDlDpipeMatch<Self> {
        let header_offset = push_nested_header(self.as_vec_mut(), 40u16);
        PushDlDpipeMatch {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
    #[doc = "Attribute may repeat multiple times (treat it as array)"]
    pub fn nested_dpipe_match_value(mut self) -> PushDlDpipeMatchValue<Self> {
        let header_offset = push_nested_header(self.as_vec_mut(), 41u16);
        PushDlDpipeMatchValue {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
    #[doc = "Associated type: [`DpipeMatchType`] (enum)"]
    pub fn push_dpipe_match_type(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 42u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Attribute may repeat multiple times (treat it as array)"]
    pub fn nested_dpipe_action(mut self) -> PushDlDpipeAction<Self> {
        let header_offset = push_nested_header(self.as_vec_mut(), 43u16);
        PushDlDpipeAction {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
    #[doc = "Attribute may repeat multiple times (treat it as array)"]
    pub fn nested_dpipe_action_value(mut self) -> PushDlDpipeActionValue<Self> {
        let header_offset = push_nested_header(self.as_vec_mut(), 44u16);
        PushDlDpipeActionValue {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
    #[doc = "Associated type: [`DpipeActionType`] (enum)"]
    pub fn push_dpipe_action_type(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 45u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_dpipe_value(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 46u16, value.len() as u16);
        self.as_vec_mut().extend(value);
        self
    }
    pub fn push_dpipe_value_mask(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 47u16, value.len() as u16);
        self.as_vec_mut().extend(value);
        self
    }
    pub fn push_dpipe_value_mapping(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 48u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn nested_dpipe_headers(mut self) -> PushDlDpipeHeaders<Self> {
        let header_offset = push_nested_header(self.as_vec_mut(), 49u16);
        PushDlDpipeHeaders {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
    #[doc = "Attribute may repeat multiple times (treat it as array)"]
    pub fn nested_dpipe_header(mut self) -> PushDlDpipeHeader<Self> {
        let header_offset = push_nested_header(self.as_vec_mut(), 50u16);
        PushDlDpipeHeader {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
    pub fn push_dpipe_header_name(mut self, value: &CStr) -> Self {
        push_header(
            self.as_vec_mut(),
            51u16,
            value.to_bytes_with_nul().len() as u16,
        );
        self.as_vec_mut().extend(value.to_bytes_with_nul());
        self
    }
    pub fn push_dpipe_header_name_bytes(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 51u16, (value.len() + 1) as u16);
        self.as_vec_mut().extend(value);
        self.as_vec_mut().push(0);
        self
    }
    #[doc = "Associated type: [`DpipeHeaderId`] (enum)"]
    pub fn push_dpipe_header_id(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 52u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn nested_dpipe_header_fields(mut self) -> PushDlDpipeHeaderFields<Self> {
        let header_offset = push_nested_header(self.as_vec_mut(), 53u16);
        PushDlDpipeHeaderFields {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
    pub fn push_dpipe_header_global(mut self, value: u8) -> Self {
        push_header(self.as_vec_mut(), 54u16, 1 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_dpipe_header_index(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 55u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Attribute may repeat multiple times (treat it as array)"]
    pub fn nested_dpipe_field(mut self) -> PushDlDpipeField<Self> {
        let header_offset = push_nested_header(self.as_vec_mut(), 56u16);
        PushDlDpipeField {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
    pub fn push_dpipe_field_name(mut self, value: &CStr) -> Self {
        push_header(
            self.as_vec_mut(),
            57u16,
            value.to_bytes_with_nul().len() as u16,
        );
        self.as_vec_mut().extend(value.to_bytes_with_nul());
        self
    }
    pub fn push_dpipe_field_name_bytes(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 57u16, (value.len() + 1) as u16);
        self.as_vec_mut().extend(value);
        self.as_vec_mut().push(0);
        self
    }
    pub fn push_dpipe_field_id(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 58u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_dpipe_field_bitwidth(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 59u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Associated type: [`DpipeFieldMappingType`] (enum)"]
    pub fn push_dpipe_field_mapping_type(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 60u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_pad(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 61u16, value.len() as u16);
        self.as_vec_mut().extend(value);
        self
    }
    #[doc = "Associated type: [`EswitchEncapMode`] (enum)"]
    pub fn push_eswitch_encap_mode(mut self, value: u8) -> Self {
        push_header(self.as_vec_mut(), 62u16, 1 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn nested_resource_list(mut self) -> PushDlResourceList<Self> {
        let header_offset = push_nested_header(self.as_vec_mut(), 63u16);
        PushDlResourceList {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
    #[doc = "Attribute may repeat multiple times (treat it as array)"]
    pub fn nested_resource(mut self) -> PushDlResource<Self> {
        let header_offset = push_nested_header(self.as_vec_mut(), 64u16);
        PushDlResource {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
    pub fn push_resource_name(mut self, value: &CStr) -> Self {
        push_header(
            self.as_vec_mut(),
            65u16,
            value.to_bytes_with_nul().len() as u16,
        );
        self.as_vec_mut().extend(value.to_bytes_with_nul());
        self
    }
    pub fn push_resource_name_bytes(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 65u16, (value.len() + 1) as u16);
        self.as_vec_mut().extend(value);
        self.as_vec_mut().push(0);
        self
    }
    pub fn push_resource_id(mut self, value: u64) -> Self {
        push_header(self.as_vec_mut(), 66u16, 8 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_resource_size(mut self, value: u64) -> Self {
        push_header(self.as_vec_mut(), 67u16, 8 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_resource_size_new(mut self, value: u64) -> Self {
        push_header(self.as_vec_mut(), 68u16, 8 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_resource_size_valid(mut self, value: u8) -> Self {
        push_header(self.as_vec_mut(), 69u16, 1 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_resource_size_min(mut self, value: u64) -> Self {
        push_header(self.as_vec_mut(), 70u16, 8 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_resource_size_max(mut self, value: u64) -> Self {
        push_header(self.as_vec_mut(), 71u16, 8 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_resource_size_gran(mut self, value: u64) -> Self {
        push_header(self.as_vec_mut(), 72u16, 8 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Associated type: [`ResourceUnit`] (enum)"]
    pub fn push_resource_unit(mut self, value: u8) -> Self {
        push_header(self.as_vec_mut(), 73u16, 1 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_resource_occ(mut self, value: u64) -> Self {
        push_header(self.as_vec_mut(), 74u16, 8 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_dpipe_table_resource_id(mut self, value: u64) -> Self {
        push_header(self.as_vec_mut(), 75u16, 8 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_dpipe_table_resource_units(mut self, value: u64) -> Self {
        push_header(self.as_vec_mut(), 76u16, 8 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Associated type: [`PortFlavour`] (enum)"]
    pub fn push_port_flavour(mut self, value: u16) -> Self {
        push_header(self.as_vec_mut(), 77u16, 2 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_port_number(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 78u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_port_split_subport_number(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 79u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn nested_param(mut self) -> PushDlParam<Self> {
        let header_offset = push_nested_header(self.as_vec_mut(), 80u16);
        PushDlParam {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
    pub fn push_param_name(mut self, value: &CStr) -> Self {
        push_header(
            self.as_vec_mut(),
            81u16,
            value.to_bytes_with_nul().len() as u16,
        );
        self.as_vec_mut().extend(value.to_bytes_with_nul());
        self
    }
    pub fn push_param_name_bytes(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 81u16, (value.len() + 1) as u16);
        self.as_vec_mut().extend(value);
        self.as_vec_mut().push(0);
        self
    }
    pub fn push_param_generic(mut self, value: ()) -> Self {
        push_header(self.as_vec_mut(), 82u16, 0 as u16);
        self
    }
    #[doc = "Associated type: [`VarAttrType`] (enum)"]
    pub fn push_param_type(mut self, value: u8) -> Self {
        push_header(self.as_vec_mut(), 83u16, 1 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Associated type: [`ParamCmode`] (enum)"]
    pub fn push_param_value_cmode(mut self, value: u8) -> Self {
        push_header(self.as_vec_mut(), 87u16, 1 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_region_name(mut self, value: &CStr) -> Self {
        push_header(
            self.as_vec_mut(),
            88u16,
            value.to_bytes_with_nul().len() as u16,
        );
        self.as_vec_mut().extend(value.to_bytes_with_nul());
        self
    }
    pub fn push_region_name_bytes(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 88u16, (value.len() + 1) as u16);
        self.as_vec_mut().extend(value);
        self.as_vec_mut().push(0);
        self
    }
    pub fn push_region_size(mut self, value: u64) -> Self {
        push_header(self.as_vec_mut(), 89u16, 8 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn nested_region_snapshots(mut self) -> PushDlRegionSnapshots<Self> {
        let header_offset = push_nested_header(self.as_vec_mut(), 90u16);
        PushDlRegionSnapshots {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
    pub fn nested_region_snapshot(mut self) -> PushDlRegionSnapshot<Self> {
        let header_offset = push_nested_header(self.as_vec_mut(), 91u16);
        PushDlRegionSnapshot {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
    pub fn push_region_snapshot_id(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 92u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn nested_region_chunks(mut self) -> PushDlRegionChunks<Self> {
        let header_offset = push_nested_header(self.as_vec_mut(), 93u16);
        PushDlRegionChunks {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
    pub fn nested_region_chunk(mut self) -> PushDlRegionChunk<Self> {
        let header_offset = push_nested_header(self.as_vec_mut(), 94u16);
        PushDlRegionChunk {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
    pub fn push_region_chunk_data(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 95u16, value.len() as u16);
        self.as_vec_mut().extend(value);
        self
    }
    pub fn push_region_chunk_addr(mut self, value: u64) -> Self {
        push_header(self.as_vec_mut(), 96u16, 8 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_region_chunk_len(mut self, value: u64) -> Self {
        push_header(self.as_vec_mut(), 97u16, 8 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_info_driver_name(mut self, value: &CStr) -> Self {
        push_header(
            self.as_vec_mut(),
            98u16,
            value.to_bytes_with_nul().len() as u16,
        );
        self.as_vec_mut().extend(value.to_bytes_with_nul());
        self
    }
    pub fn push_info_driver_name_bytes(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 98u16, (value.len() + 1) as u16);
        self.as_vec_mut().extend(value);
        self.as_vec_mut().push(0);
        self
    }
    pub fn push_info_serial_number(mut self, value: &CStr) -> Self {
        push_header(
            self.as_vec_mut(),
            99u16,
            value.to_bytes_with_nul().len() as u16,
        );
        self.as_vec_mut().extend(value.to_bytes_with_nul());
        self
    }
    pub fn push_info_serial_number_bytes(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 99u16, (value.len() + 1) as u16);
        self.as_vec_mut().extend(value);
        self.as_vec_mut().push(0);
        self
    }
    #[doc = "Attribute may repeat multiple times (treat it as array)"]
    pub fn nested_info_version_fixed(mut self) -> PushDlInfoVersion<Self> {
        let header_offset = push_nested_header(self.as_vec_mut(), 100u16);
        PushDlInfoVersion {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
    #[doc = "Attribute may repeat multiple times (treat it as array)"]
    pub fn nested_info_version_running(mut self) -> PushDlInfoVersion<Self> {
        let header_offset = push_nested_header(self.as_vec_mut(), 101u16);
        PushDlInfoVersion {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
    #[doc = "Attribute may repeat multiple times (treat it as array)"]
    pub fn nested_info_version_stored(mut self) -> PushDlInfoVersion<Self> {
        let header_offset = push_nested_header(self.as_vec_mut(), 102u16);
        PushDlInfoVersion {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
    pub fn push_info_version_name(mut self, value: &CStr) -> Self {
        push_header(
            self.as_vec_mut(),
            103u16,
            value.to_bytes_with_nul().len() as u16,
        );
        self.as_vec_mut().extend(value.to_bytes_with_nul());
        self
    }
    pub fn push_info_version_name_bytes(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 103u16, (value.len() + 1) as u16);
        self.as_vec_mut().extend(value);
        self.as_vec_mut().push(0);
        self
    }
    pub fn push_info_version_value(mut self, value: &CStr) -> Self {
        push_header(
            self.as_vec_mut(),
            104u16,
            value.to_bytes_with_nul().len() as u16,
        );
        self.as_vec_mut().extend(value.to_bytes_with_nul());
        self
    }
    pub fn push_info_version_value_bytes(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 104u16, (value.len() + 1) as u16);
        self.as_vec_mut().extend(value);
        self.as_vec_mut().push(0);
        self
    }
    pub fn push_sb_pool_cell_size(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 105u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn nested_fmsg(mut self) -> PushDlFmsg<Self> {
        let header_offset = push_nested_header(self.as_vec_mut(), 106u16);
        PushDlFmsg {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
    pub fn push_fmsg_obj_nest_start(mut self, value: ()) -> Self {
        push_header(self.as_vec_mut(), 107u16, 0 as u16);
        self
    }
    pub fn push_fmsg_pair_nest_start(mut self, value: ()) -> Self {
        push_header(self.as_vec_mut(), 108u16, 0 as u16);
        self
    }
    pub fn push_fmsg_arr_nest_start(mut self, value: ()) -> Self {
        push_header(self.as_vec_mut(), 109u16, 0 as u16);
        self
    }
    pub fn push_fmsg_nest_end(mut self, value: ()) -> Self {
        push_header(self.as_vec_mut(), 110u16, 0 as u16);
        self
    }
    pub fn push_fmsg_obj_name(mut self, value: &CStr) -> Self {
        push_header(
            self.as_vec_mut(),
            111u16,
            value.to_bytes_with_nul().len() as u16,
        );
        self.as_vec_mut().extend(value.to_bytes_with_nul());
        self
    }
    pub fn push_fmsg_obj_name_bytes(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 111u16, (value.len() + 1) as u16);
        self.as_vec_mut().extend(value);
        self.as_vec_mut().push(0);
        self
    }
    #[doc = "Associated type: [`VarAttrType`] (enum)"]
    pub fn push_fmsg_obj_value_type(mut self, value: u8) -> Self {
        push_header(self.as_vec_mut(), 112u16, 1 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn nested_health_reporter(mut self) -> PushDlHealthReporter<Self> {
        let header_offset = push_nested_header(self.as_vec_mut(), 114u16);
        PushDlHealthReporter {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
    pub fn push_health_reporter_name(mut self, value: &CStr) -> Self {
        push_header(
            self.as_vec_mut(),
            115u16,
            value.to_bytes_with_nul().len() as u16,
        );
        self.as_vec_mut().extend(value.to_bytes_with_nul());
        self
    }
    pub fn push_health_reporter_name_bytes(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 115u16, (value.len() + 1) as u16);
        self.as_vec_mut().extend(value);
        self.as_vec_mut().push(0);
        self
    }
    pub fn push_health_reporter_state(mut self, value: u8) -> Self {
        push_header(self.as_vec_mut(), 116u16, 1 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_health_reporter_err_count(mut self, value: u64) -> Self {
        push_header(self.as_vec_mut(), 117u16, 8 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_health_reporter_recover_count(mut self, value: u64) -> Self {
        push_header(self.as_vec_mut(), 118u16, 8 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_health_reporter_dump_ts(mut self, value: u64) -> Self {
        push_header(self.as_vec_mut(), 119u16, 8 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_health_reporter_graceful_period(mut self, value: u64) -> Self {
        push_header(self.as_vec_mut(), 120u16, 8 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_health_reporter_auto_recover(mut self, value: u8) -> Self {
        push_header(self.as_vec_mut(), 121u16, 1 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_flash_update_file_name(mut self, value: &CStr) -> Self {
        push_header(
            self.as_vec_mut(),
            122u16,
            value.to_bytes_with_nul().len() as u16,
        );
        self.as_vec_mut().extend(value.to_bytes_with_nul());
        self
    }
    pub fn push_flash_update_file_name_bytes(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 122u16, (value.len() + 1) as u16);
        self.as_vec_mut().extend(value);
        self.as_vec_mut().push(0);
        self
    }
    pub fn push_flash_update_component(mut self, value: &CStr) -> Self {
        push_header(
            self.as_vec_mut(),
            123u16,
            value.to_bytes_with_nul().len() as u16,
        );
        self.as_vec_mut().extend(value.to_bytes_with_nul());
        self
    }
    pub fn push_flash_update_component_bytes(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 123u16, (value.len() + 1) as u16);
        self.as_vec_mut().extend(value);
        self.as_vec_mut().push(0);
        self
    }
    pub fn push_flash_update_status_msg(mut self, value: &CStr) -> Self {
        push_header(
            self.as_vec_mut(),
            124u16,
            value.to_bytes_with_nul().len() as u16,
        );
        self.as_vec_mut().extend(value.to_bytes_with_nul());
        self
    }
    pub fn push_flash_update_status_msg_bytes(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 124u16, (value.len() + 1) as u16);
        self.as_vec_mut().extend(value);
        self.as_vec_mut().push(0);
        self
    }
    pub fn push_flash_update_status_done(mut self, value: u64) -> Self {
        push_header(self.as_vec_mut(), 125u16, 8 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_flash_update_status_total(mut self, value: u64) -> Self {
        push_header(self.as_vec_mut(), 126u16, 8 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_port_pci_pf_number(mut self, value: u16) -> Self {
        push_header(self.as_vec_mut(), 127u16, 2 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_port_pci_vf_number(mut self, value: u16) -> Self {
        push_header(self.as_vec_mut(), 128u16, 2 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn nested_stats(mut self) -> PushDlAttrStats<Self> {
        let header_offset = push_nested_header(self.as_vec_mut(), 129u16);
        PushDlAttrStats {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
    pub fn push_trap_name(mut self, value: &CStr) -> Self {
        push_header(
            self.as_vec_mut(),
            130u16,
            value.to_bytes_with_nul().len() as u16,
        );
        self.as_vec_mut().extend(value.to_bytes_with_nul());
        self
    }
    pub fn push_trap_name_bytes(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 130u16, (value.len() + 1) as u16);
        self.as_vec_mut().extend(value);
        self.as_vec_mut().push(0);
        self
    }
    #[doc = "Associated type: [`TrapAction`] (enum)"]
    pub fn push_trap_action(mut self, value: u8) -> Self {
        push_header(self.as_vec_mut(), 131u16, 1 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Associated type: [`TrapType`] (enum)"]
    pub fn push_trap_type(mut self, value: u8) -> Self {
        push_header(self.as_vec_mut(), 132u16, 1 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_trap_generic(mut self, value: ()) -> Self {
        push_header(self.as_vec_mut(), 133u16, 0 as u16);
        self
    }
    pub fn nested_trap_metadata(mut self) -> PushDlTrapMetadata<Self> {
        let header_offset = push_nested_header(self.as_vec_mut(), 134u16);
        PushDlTrapMetadata {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
    pub fn push_trap_group_name(mut self, value: &CStr) -> Self {
        push_header(
            self.as_vec_mut(),
            135u16,
            value.to_bytes_with_nul().len() as u16,
        );
        self.as_vec_mut().extend(value.to_bytes_with_nul());
        self
    }
    pub fn push_trap_group_name_bytes(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 135u16, (value.len() + 1) as u16);
        self.as_vec_mut().extend(value);
        self.as_vec_mut().push(0);
        self
    }
    pub fn push_reload_failed(mut self, value: u8) -> Self {
        push_header(self.as_vec_mut(), 136u16, 1 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_health_reporter_dump_ts_ns(mut self, value: u64) -> Self {
        push_header(self.as_vec_mut(), 137u16, 8 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_netns_fd(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 138u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_netns_pid(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 139u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_netns_id(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 140u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_health_reporter_auto_dump(mut self, value: u8) -> Self {
        push_header(self.as_vec_mut(), 141u16, 1 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_trap_policer_id(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 142u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_trap_policer_rate(mut self, value: u64) -> Self {
        push_header(self.as_vec_mut(), 143u16, 8 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_trap_policer_burst(mut self, value: u64) -> Self {
        push_header(self.as_vec_mut(), 144u16, 8 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn nested_port_function(mut self) -> PushDlPortFunction<Self> {
        let header_offset = push_nested_header(self.as_vec_mut(), 145u16);
        PushDlPortFunction {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
    pub fn push_info_board_serial_number(mut self, value: &CStr) -> Self {
        push_header(
            self.as_vec_mut(),
            146u16,
            value.to_bytes_with_nul().len() as u16,
        );
        self.as_vec_mut().extend(value.to_bytes_with_nul());
        self
    }
    pub fn push_info_board_serial_number_bytes(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 146u16, (value.len() + 1) as u16);
        self.as_vec_mut().extend(value);
        self.as_vec_mut().push(0);
        self
    }
    pub fn push_port_lanes(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 147u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_port_splittable(mut self, value: u8) -> Self {
        push_header(self.as_vec_mut(), 148u16, 1 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_port_external(mut self, value: u8) -> Self {
        push_header(self.as_vec_mut(), 149u16, 1 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_port_controller_number(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 150u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_flash_update_status_timeout(mut self, value: u64) -> Self {
        push_header(self.as_vec_mut(), 151u16, 8 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Associated type: [`FlashOverwrite`] (1 bit per enumeration)"]
    pub fn push_flash_update_overwrite_mask(mut self, value: BuiltinBitfield32) -> Self {
        push_header(self.as_vec_mut(), 152u16, value.as_slice().len() as u16);
        self.as_vec_mut().extend(value.as_slice());
        self
    }
    #[doc = "Associated type: [`ReloadAction`] (enum)"]
    pub fn push_reload_action(mut self, value: u8) -> Self {
        push_header(self.as_vec_mut(), 153u16, 1 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Associated type: [`ReloadAction`] (1 bit per enumeration)"]
    pub fn push_reload_actions_performed(mut self, value: BuiltinBitfield32) -> Self {
        push_header(self.as_vec_mut(), 154u16, value.as_slice().len() as u16);
        self.as_vec_mut().extend(value.as_slice());
        self
    }
    #[doc = "Associated type: [`ReloadAction`] (1 bit per enumeration)"]
    pub fn push_reload_limits(mut self, value: BuiltinBitfield32) -> Self {
        push_header(self.as_vec_mut(), 155u16, value.as_slice().len() as u16);
        self.as_vec_mut().extend(value.as_slice());
        self
    }
    pub fn nested_dev_stats(mut self) -> PushDlDevStats<Self> {
        let header_offset = push_nested_header(self.as_vec_mut(), 156u16);
        PushDlDevStats {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
    pub fn nested_reload_stats(mut self) -> PushDlReloadStats<Self> {
        let header_offset = push_nested_header(self.as_vec_mut(), 157u16);
        PushDlReloadStats {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
    #[doc = "Attribute may repeat multiple times (treat it as array)"]
    pub fn nested_reload_stats_entry(mut self) -> PushDlReloadStatsEntry<Self> {
        let header_offset = push_nested_header(self.as_vec_mut(), 158u16);
        PushDlReloadStatsEntry {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
    pub fn push_reload_stats_limit(mut self, value: u8) -> Self {
        push_header(self.as_vec_mut(), 159u16, 1 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_reload_stats_value(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 160u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn nested_remote_reload_stats(mut self) -> PushDlReloadStats<Self> {
        let header_offset = push_nested_header(self.as_vec_mut(), 161u16);
        PushDlReloadStats {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
    #[doc = "Attribute may repeat multiple times (treat it as array)"]
    pub fn nested_reload_action_info(mut self) -> PushDlReloadActInfo<Self> {
        let header_offset = push_nested_header(self.as_vec_mut(), 162u16);
        PushDlReloadActInfo {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
    #[doc = "Attribute may repeat multiple times (treat it as array)"]
    pub fn nested_reload_action_stats(mut self) -> PushDlReloadActStats<Self> {
        let header_offset = push_nested_header(self.as_vec_mut(), 163u16);
        PushDlReloadActStats {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
    pub fn push_port_pci_sf_number(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 164u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Associated type: [`RateType`] (enum)"]
    pub fn push_rate_type(mut self, value: u16) -> Self {
        push_header(self.as_vec_mut(), 165u16, 2 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_rate_tx_share(mut self, value: u64) -> Self {
        push_header(self.as_vec_mut(), 166u16, 8 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_rate_tx_max(mut self, value: u64) -> Self {
        push_header(self.as_vec_mut(), 167u16, 8 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_rate_node_name(mut self, value: &CStr) -> Self {
        push_header(
            self.as_vec_mut(),
            168u16,
            value.to_bytes_with_nul().len() as u16,
        );
        self.as_vec_mut().extend(value.to_bytes_with_nul());
        self
    }
    pub fn push_rate_node_name_bytes(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 168u16, (value.len() + 1) as u16);
        self.as_vec_mut().extend(value);
        self.as_vec_mut().push(0);
        self
    }
    pub fn push_rate_parent_node_name(mut self, value: &CStr) -> Self {
        push_header(
            self.as_vec_mut(),
            169u16,
            value.to_bytes_with_nul().len() as u16,
        );
        self.as_vec_mut().extend(value.to_bytes_with_nul());
        self
    }
    pub fn push_rate_parent_node_name_bytes(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 169u16, (value.len() + 1) as u16);
        self.as_vec_mut().extend(value);
        self.as_vec_mut().push(0);
        self
    }
    pub fn push_region_max_snapshots(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 170u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_linecard_index(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 171u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_linecard_state(mut self, value: u8) -> Self {
        push_header(self.as_vec_mut(), 172u16, 1 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_linecard_type(mut self, value: &CStr) -> Self {
        push_header(
            self.as_vec_mut(),
            173u16,
            value.to_bytes_with_nul().len() as u16,
        );
        self.as_vec_mut().extend(value.to_bytes_with_nul());
        self
    }
    pub fn push_linecard_type_bytes(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 173u16, (value.len() + 1) as u16);
        self.as_vec_mut().extend(value);
        self.as_vec_mut().push(0);
        self
    }
    pub fn nested_linecard_supported_types(mut self) -> PushDlLinecardSupportedTypes<Self> {
        let header_offset = push_nested_header(self.as_vec_mut(), 174u16);
        PushDlLinecardSupportedTypes {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
    pub fn nested_selftests(mut self) -> PushDlSelftestId<Self> {
        let header_offset = push_nested_header(self.as_vec_mut(), 176u16);
        PushDlSelftestId {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
    pub fn push_rate_tx_priority(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 177u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_rate_tx_weight(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 178u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_region_direct(mut self, value: ()) -> Self {
        push_header(self.as_vec_mut(), 179u16, 0 as u16);
        self
    }
    #[doc = "Attribute may repeat multiple times (treat it as array)"]
    pub fn nested_rate_tc_bws(mut self) -> PushDlRateTcBws<Self> {
        let header_offset = push_nested_header(self.as_vec_mut(), 180u16);
        PushDlRateTcBws {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
    #[doc = "Time (in msec) for recoveries before starting the grace period.\n"]
    pub fn push_health_reporter_burst_period(mut self, value: u64) -> Self {
        push_header(self.as_vec_mut(), 181u16, 8 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Request restoring parameter to its default value.\n"]
    pub fn push_param_reset_default(mut self, value: ()) -> Self {
        push_header(self.as_vec_mut(), 183u16, 0 as u16);
        self
    }
    #[doc = "Unique devlink instance index.\n"]
    pub fn push_index(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 184u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Bitmask selecting which resource classes to include in a resource-dump\nresponse. Bit 0 (dev) selects device-level resources; bit 1 (port)\nselects port-level resources. When absent all classes are returned.\n\nAssociated type: [`ResourceScope`] (1 bit per enumeration)"]
    pub fn push_resource_scope_mask(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 185u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
}
impl<Prev: Pusher> Drop for PushDevlink<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushDlDevStats<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushDlDevStats<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushDlDevStats<Prev> {
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
    pub fn nested_reload_stats(mut self) -> PushDlReloadStats<Self> {
        let header_offset = push_nested_header(self.as_vec_mut(), 157u16);
        PushDlReloadStats {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
    pub fn nested_remote_reload_stats(mut self) -> PushDlReloadStats<Self> {
        let header_offset = push_nested_header(self.as_vec_mut(), 161u16);
        PushDlReloadStats {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
}
impl<Prev: Pusher> Drop for PushDlDevStats<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushDlReloadStats<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushDlReloadStats<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushDlReloadStats<Prev> {
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
    #[doc = "Attribute may repeat multiple times (treat it as array)"]
    pub fn nested_reload_action_info(mut self) -> PushDlReloadActInfo<Self> {
        let header_offset = push_nested_header(self.as_vec_mut(), 162u16);
        PushDlReloadActInfo {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
}
impl<Prev: Pusher> Drop for PushDlReloadStats<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushDlReloadActInfo<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushDlReloadActInfo<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushDlReloadActInfo<Prev> {
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
    #[doc = "Associated type: [`ReloadAction`] (enum)"]
    pub fn push_reload_action(mut self, value: u8) -> Self {
        push_header(self.as_vec_mut(), 153u16, 1 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Attribute may repeat multiple times (treat it as array)"]
    pub fn nested_reload_action_stats(mut self) -> PushDlReloadActStats<Self> {
        let header_offset = push_nested_header(self.as_vec_mut(), 163u16);
        PushDlReloadActStats {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
}
impl<Prev: Pusher> Drop for PushDlReloadActInfo<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushDlReloadActStats<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushDlReloadActStats<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushDlReloadActStats<Prev> {
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
    #[doc = "Attribute may repeat multiple times (treat it as array)"]
    pub fn nested_reload_stats_entry(mut self) -> PushDlReloadStatsEntry<Self> {
        let header_offset = push_nested_header(self.as_vec_mut(), 158u16);
        PushDlReloadStatsEntry {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
}
impl<Prev: Pusher> Drop for PushDlReloadActStats<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushDlReloadStatsEntry<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushDlReloadStatsEntry<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushDlReloadStatsEntry<Prev> {
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
    pub fn push_reload_stats_limit(mut self, value: u8) -> Self {
        push_header(self.as_vec_mut(), 159u16, 1 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_reload_stats_value(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 160u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
}
impl<Prev: Pusher> Drop for PushDlReloadStatsEntry<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushDlInfoVersion<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushDlInfoVersion<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushDlInfoVersion<Prev> {
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
    pub fn push_info_version_name(mut self, value: &CStr) -> Self {
        push_header(
            self.as_vec_mut(),
            103u16,
            value.to_bytes_with_nul().len() as u16,
        );
        self.as_vec_mut().extend(value.to_bytes_with_nul());
        self
    }
    pub fn push_info_version_name_bytes(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 103u16, (value.len() + 1) as u16);
        self.as_vec_mut().extend(value);
        self.as_vec_mut().push(0);
        self
    }
    pub fn push_info_version_value(mut self, value: &CStr) -> Self {
        push_header(
            self.as_vec_mut(),
            104u16,
            value.to_bytes_with_nul().len() as u16,
        );
        self.as_vec_mut().extend(value.to_bytes_with_nul());
        self
    }
    pub fn push_info_version_value_bytes(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 104u16, (value.len() + 1) as u16);
        self.as_vec_mut().extend(value);
        self.as_vec_mut().push(0);
        self
    }
}
impl<Prev: Pusher> Drop for PushDlInfoVersion<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushDlPortFunction<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushDlPortFunction<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushDlPortFunction<Prev> {
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
    pub fn push_hw_addr(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 1u16, value.len() as u16);
        self.as_vec_mut().extend(value);
        self
    }
    #[doc = "Associated type: [`PortFnState`] (enum)"]
    pub fn push_state(mut self, value: u8) -> Self {
        push_header(self.as_vec_mut(), 2u16, 1 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Associated type: [`PortFnOpstate`] (enum)"]
    pub fn push_opstate(mut self, value: u8) -> Self {
        push_header(self.as_vec_mut(), 3u16, 1 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Associated type: [`PortFnAttrCap`] (1 bit per enumeration)"]
    pub fn push_caps(mut self, value: BuiltinBitfield32) -> Self {
        push_header(self.as_vec_mut(), 4u16, value.as_slice().len() as u16);
        self.as_vec_mut().extend(value.as_slice());
        self
    }
}
impl<Prev: Pusher> Drop for PushDlPortFunction<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushDlDpipeTables<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushDlDpipeTables<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushDlDpipeTables<Prev> {
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
    #[doc = "Attribute may repeat multiple times (treat it as array)"]
    pub fn nested_dpipe_table(mut self) -> PushDlDpipeTable<Self> {
        let header_offset = push_nested_header(self.as_vec_mut(), 28u16);
        PushDlDpipeTable {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
}
impl<Prev: Pusher> Drop for PushDlDpipeTables<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushDlDpipeTable<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushDlDpipeTable<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushDlDpipeTable<Prev> {
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
    pub fn push_dpipe_table_name(mut self, value: &CStr) -> Self {
        push_header(
            self.as_vec_mut(),
            29u16,
            value.to_bytes_with_nul().len() as u16,
        );
        self.as_vec_mut().extend(value.to_bytes_with_nul());
        self
    }
    pub fn push_dpipe_table_name_bytes(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 29u16, (value.len() + 1) as u16);
        self.as_vec_mut().extend(value);
        self.as_vec_mut().push(0);
        self
    }
    pub fn push_dpipe_table_size(mut self, value: u64) -> Self {
        push_header(self.as_vec_mut(), 30u16, 8 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn nested_dpipe_table_matches(mut self) -> PushDlDpipeTableMatches<Self> {
        let header_offset = push_nested_header(self.as_vec_mut(), 31u16);
        PushDlDpipeTableMatches {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
    pub fn nested_dpipe_table_actions(mut self) -> PushDlDpipeTableActions<Self> {
        let header_offset = push_nested_header(self.as_vec_mut(), 32u16);
        PushDlDpipeTableActions {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
    pub fn push_dpipe_table_counters_enabled(mut self, value: u8) -> Self {
        push_header(self.as_vec_mut(), 33u16, 1 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_dpipe_table_resource_id(mut self, value: u64) -> Self {
        push_header(self.as_vec_mut(), 75u16, 8 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_dpipe_table_resource_units(mut self, value: u64) -> Self {
        push_header(self.as_vec_mut(), 76u16, 8 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
}
impl<Prev: Pusher> Drop for PushDlDpipeTable<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushDlDpipeTableMatches<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushDlDpipeTableMatches<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushDlDpipeTableMatches<Prev> {
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
    #[doc = "Attribute may repeat multiple times (treat it as array)"]
    pub fn nested_dpipe_match(mut self) -> PushDlDpipeMatch<Self> {
        let header_offset = push_nested_header(self.as_vec_mut(), 40u16);
        PushDlDpipeMatch {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
}
impl<Prev: Pusher> Drop for PushDlDpipeTableMatches<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushDlDpipeTableActions<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushDlDpipeTableActions<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushDlDpipeTableActions<Prev> {
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
    #[doc = "Attribute may repeat multiple times (treat it as array)"]
    pub fn nested_dpipe_action(mut self) -> PushDlDpipeAction<Self> {
        let header_offset = push_nested_header(self.as_vec_mut(), 43u16);
        PushDlDpipeAction {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
}
impl<Prev: Pusher> Drop for PushDlDpipeTableActions<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushDlDpipeEntries<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushDlDpipeEntries<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushDlDpipeEntries<Prev> {
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
    #[doc = "Attribute may repeat multiple times (treat it as array)"]
    pub fn nested_dpipe_entry(mut self) -> PushDlDpipeEntry<Self> {
        let header_offset = push_nested_header(self.as_vec_mut(), 35u16);
        PushDlDpipeEntry {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
}
impl<Prev: Pusher> Drop for PushDlDpipeEntries<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushDlDpipeEntry<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushDlDpipeEntry<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushDlDpipeEntry<Prev> {
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
    pub fn push_dpipe_entry_index(mut self, value: u64) -> Self {
        push_header(self.as_vec_mut(), 36u16, 8 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn nested_dpipe_entry_match_values(mut self) -> PushDlDpipeEntryMatchValues<Self> {
        let header_offset = push_nested_header(self.as_vec_mut(), 37u16);
        PushDlDpipeEntryMatchValues {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
    pub fn nested_dpipe_entry_action_values(mut self) -> PushDlDpipeEntryActionValues<Self> {
        let header_offset = push_nested_header(self.as_vec_mut(), 38u16);
        PushDlDpipeEntryActionValues {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
    pub fn push_dpipe_entry_counter(mut self, value: u64) -> Self {
        push_header(self.as_vec_mut(), 39u16, 8 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
}
impl<Prev: Pusher> Drop for PushDlDpipeEntry<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushDlDpipeEntryMatchValues<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushDlDpipeEntryMatchValues<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushDlDpipeEntryMatchValues<Prev> {
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
    #[doc = "Attribute may repeat multiple times (treat it as array)"]
    pub fn nested_dpipe_match_value(mut self) -> PushDlDpipeMatchValue<Self> {
        let header_offset = push_nested_header(self.as_vec_mut(), 41u16);
        PushDlDpipeMatchValue {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
}
impl<Prev: Pusher> Drop for PushDlDpipeEntryMatchValues<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushDlDpipeEntryActionValues<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushDlDpipeEntryActionValues<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushDlDpipeEntryActionValues<Prev> {
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
    #[doc = "Attribute may repeat multiple times (treat it as array)"]
    pub fn nested_dpipe_action_value(mut self) -> PushDlDpipeActionValue<Self> {
        let header_offset = push_nested_header(self.as_vec_mut(), 44u16);
        PushDlDpipeActionValue {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
}
impl<Prev: Pusher> Drop for PushDlDpipeEntryActionValues<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushDlDpipeMatch<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushDlDpipeMatch<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushDlDpipeMatch<Prev> {
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
    #[doc = "Associated type: [`DpipeMatchType`] (enum)"]
    pub fn push_dpipe_match_type(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 42u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Associated type: [`DpipeHeaderId`] (enum)"]
    pub fn push_dpipe_header_id(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 52u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_dpipe_header_global(mut self, value: u8) -> Self {
        push_header(self.as_vec_mut(), 54u16, 1 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_dpipe_header_index(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 55u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_dpipe_field_id(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 58u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
}
impl<Prev: Pusher> Drop for PushDlDpipeMatch<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushDlDpipeMatchValue<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushDlDpipeMatchValue<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushDlDpipeMatchValue<Prev> {
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
    #[doc = "Attribute may repeat multiple times (treat it as array)"]
    pub fn nested_dpipe_match(mut self) -> PushDlDpipeMatch<Self> {
        let header_offset = push_nested_header(self.as_vec_mut(), 40u16);
        PushDlDpipeMatch {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
    pub fn push_dpipe_value(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 46u16, value.len() as u16);
        self.as_vec_mut().extend(value);
        self
    }
    pub fn push_dpipe_value_mask(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 47u16, value.len() as u16);
        self.as_vec_mut().extend(value);
        self
    }
    pub fn push_dpipe_value_mapping(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 48u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
}
impl<Prev: Pusher> Drop for PushDlDpipeMatchValue<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushDlDpipeAction<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushDlDpipeAction<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushDlDpipeAction<Prev> {
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
    #[doc = "Associated type: [`DpipeActionType`] (enum)"]
    pub fn push_dpipe_action_type(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 45u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Associated type: [`DpipeHeaderId`] (enum)"]
    pub fn push_dpipe_header_id(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 52u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_dpipe_header_global(mut self, value: u8) -> Self {
        push_header(self.as_vec_mut(), 54u16, 1 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_dpipe_header_index(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 55u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_dpipe_field_id(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 58u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
}
impl<Prev: Pusher> Drop for PushDlDpipeAction<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushDlDpipeActionValue<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushDlDpipeActionValue<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushDlDpipeActionValue<Prev> {
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
    #[doc = "Attribute may repeat multiple times (treat it as array)"]
    pub fn nested_dpipe_action(mut self) -> PushDlDpipeAction<Self> {
        let header_offset = push_nested_header(self.as_vec_mut(), 43u16);
        PushDlDpipeAction {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
    pub fn push_dpipe_value(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 46u16, value.len() as u16);
        self.as_vec_mut().extend(value);
        self
    }
    pub fn push_dpipe_value_mask(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 47u16, value.len() as u16);
        self.as_vec_mut().extend(value);
        self
    }
    pub fn push_dpipe_value_mapping(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 48u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
}
impl<Prev: Pusher> Drop for PushDlDpipeActionValue<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushDlDpipeHeaders<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushDlDpipeHeaders<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushDlDpipeHeaders<Prev> {
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
    #[doc = "Attribute may repeat multiple times (treat it as array)"]
    pub fn nested_dpipe_header(mut self) -> PushDlDpipeHeader<Self> {
        let header_offset = push_nested_header(self.as_vec_mut(), 50u16);
        PushDlDpipeHeader {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
}
impl<Prev: Pusher> Drop for PushDlDpipeHeaders<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushDlDpipeHeader<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushDlDpipeHeader<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushDlDpipeHeader<Prev> {
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
    pub fn push_dpipe_header_name(mut self, value: &CStr) -> Self {
        push_header(
            self.as_vec_mut(),
            51u16,
            value.to_bytes_with_nul().len() as u16,
        );
        self.as_vec_mut().extend(value.to_bytes_with_nul());
        self
    }
    pub fn push_dpipe_header_name_bytes(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 51u16, (value.len() + 1) as u16);
        self.as_vec_mut().extend(value);
        self.as_vec_mut().push(0);
        self
    }
    #[doc = "Associated type: [`DpipeHeaderId`] (enum)"]
    pub fn push_dpipe_header_id(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 52u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn nested_dpipe_header_fields(mut self) -> PushDlDpipeHeaderFields<Self> {
        let header_offset = push_nested_header(self.as_vec_mut(), 53u16);
        PushDlDpipeHeaderFields {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
    pub fn push_dpipe_header_global(mut self, value: u8) -> Self {
        push_header(self.as_vec_mut(), 54u16, 1 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
}
impl<Prev: Pusher> Drop for PushDlDpipeHeader<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushDlDpipeHeaderFields<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushDlDpipeHeaderFields<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushDlDpipeHeaderFields<Prev> {
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
    #[doc = "Attribute may repeat multiple times (treat it as array)"]
    pub fn nested_dpipe_field(mut self) -> PushDlDpipeField<Self> {
        let header_offset = push_nested_header(self.as_vec_mut(), 56u16);
        PushDlDpipeField {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
}
impl<Prev: Pusher> Drop for PushDlDpipeHeaderFields<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushDlDpipeField<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushDlDpipeField<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushDlDpipeField<Prev> {
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
    pub fn push_dpipe_field_name(mut self, value: &CStr) -> Self {
        push_header(
            self.as_vec_mut(),
            57u16,
            value.to_bytes_with_nul().len() as u16,
        );
        self.as_vec_mut().extend(value.to_bytes_with_nul());
        self
    }
    pub fn push_dpipe_field_name_bytes(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 57u16, (value.len() + 1) as u16);
        self.as_vec_mut().extend(value);
        self.as_vec_mut().push(0);
        self
    }
    pub fn push_dpipe_field_id(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 58u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_dpipe_field_bitwidth(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 59u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Associated type: [`DpipeFieldMappingType`] (enum)"]
    pub fn push_dpipe_field_mapping_type(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 60u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
}
impl<Prev: Pusher> Drop for PushDlDpipeField<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushDlResource<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushDlResource<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushDlResource<Prev> {
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
    pub fn push_resource_name(mut self, value: &CStr) -> Self {
        push_header(
            self.as_vec_mut(),
            65u16,
            value.to_bytes_with_nul().len() as u16,
        );
        self.as_vec_mut().extend(value.to_bytes_with_nul());
        self
    }
    pub fn push_resource_name_bytes(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 65u16, (value.len() + 1) as u16);
        self.as_vec_mut().extend(value);
        self.as_vec_mut().push(0);
        self
    }
    pub fn push_resource_id(mut self, value: u64) -> Self {
        push_header(self.as_vec_mut(), 66u16, 8 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_resource_size(mut self, value: u64) -> Self {
        push_header(self.as_vec_mut(), 67u16, 8 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_resource_size_new(mut self, value: u64) -> Self {
        push_header(self.as_vec_mut(), 68u16, 8 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_resource_size_valid(mut self, value: u8) -> Self {
        push_header(self.as_vec_mut(), 69u16, 1 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_resource_size_min(mut self, value: u64) -> Self {
        push_header(self.as_vec_mut(), 70u16, 8 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_resource_size_max(mut self, value: u64) -> Self {
        push_header(self.as_vec_mut(), 71u16, 8 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_resource_size_gran(mut self, value: u64) -> Self {
        push_header(self.as_vec_mut(), 72u16, 8 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Associated type: [`ResourceUnit`] (enum)"]
    pub fn push_resource_unit(mut self, value: u8) -> Self {
        push_header(self.as_vec_mut(), 73u16, 1 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_resource_occ(mut self, value: u64) -> Self {
        push_header(self.as_vec_mut(), 74u16, 8 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
}
impl<Prev: Pusher> Drop for PushDlResource<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushDlResourceList<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushDlResourceList<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushDlResourceList<Prev> {
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
    #[doc = "Attribute may repeat multiple times (treat it as array)"]
    pub fn nested_resource(mut self) -> PushDlResource<Self> {
        let header_offset = push_nested_header(self.as_vec_mut(), 64u16);
        PushDlResource {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
}
impl<Prev: Pusher> Drop for PushDlResourceList<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushDlParam<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushDlParam<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushDlParam<Prev> {
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
    pub fn push_param_name(mut self, value: &CStr) -> Self {
        push_header(
            self.as_vec_mut(),
            81u16,
            value.to_bytes_with_nul().len() as u16,
        );
        self.as_vec_mut().extend(value.to_bytes_with_nul());
        self
    }
    pub fn push_param_name_bytes(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 81u16, (value.len() + 1) as u16);
        self.as_vec_mut().extend(value);
        self.as_vec_mut().push(0);
        self
    }
    pub fn push_param_generic(mut self, value: ()) -> Self {
        push_header(self.as_vec_mut(), 82u16, 0 as u16);
        self
    }
    #[doc = "Associated type: [`VarAttrType`] (enum)"]
    pub fn push_param_type(mut self, value: u8) -> Self {
        push_header(self.as_vec_mut(), 83u16, 1 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
}
impl<Prev: Pusher> Drop for PushDlParam<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushDlRegionSnapshots<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushDlRegionSnapshots<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushDlRegionSnapshots<Prev> {
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
    pub fn nested_region_snapshot(mut self) -> PushDlRegionSnapshot<Self> {
        let header_offset = push_nested_header(self.as_vec_mut(), 91u16);
        PushDlRegionSnapshot {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
}
impl<Prev: Pusher> Drop for PushDlRegionSnapshots<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushDlRegionSnapshot<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushDlRegionSnapshot<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushDlRegionSnapshot<Prev> {
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
    pub fn push_region_snapshot_id(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 92u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
}
impl<Prev: Pusher> Drop for PushDlRegionSnapshot<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushDlRegionChunks<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushDlRegionChunks<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushDlRegionChunks<Prev> {
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
    pub fn nested_region_chunk(mut self) -> PushDlRegionChunk<Self> {
        let header_offset = push_nested_header(self.as_vec_mut(), 94u16);
        PushDlRegionChunk {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
}
impl<Prev: Pusher> Drop for PushDlRegionChunks<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushDlRegionChunk<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushDlRegionChunk<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushDlRegionChunk<Prev> {
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
    pub fn push_region_chunk_data(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 95u16, value.len() as u16);
        self.as_vec_mut().extend(value);
        self
    }
    pub fn push_region_chunk_addr(mut self, value: u64) -> Self {
        push_header(self.as_vec_mut(), 96u16, 8 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
}
impl<Prev: Pusher> Drop for PushDlRegionChunk<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushDlFmsg<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushDlFmsg<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushDlFmsg<Prev> {
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
    pub fn push_fmsg_obj_nest_start(mut self, value: ()) -> Self {
        push_header(self.as_vec_mut(), 107u16, 0 as u16);
        self
    }
    pub fn push_fmsg_pair_nest_start(mut self, value: ()) -> Self {
        push_header(self.as_vec_mut(), 108u16, 0 as u16);
        self
    }
    pub fn push_fmsg_arr_nest_start(mut self, value: ()) -> Self {
        push_header(self.as_vec_mut(), 109u16, 0 as u16);
        self
    }
    pub fn push_fmsg_nest_end(mut self, value: ()) -> Self {
        push_header(self.as_vec_mut(), 110u16, 0 as u16);
        self
    }
    pub fn push_fmsg_obj_name(mut self, value: &CStr) -> Self {
        push_header(
            self.as_vec_mut(),
            111u16,
            value.to_bytes_with_nul().len() as u16,
        );
        self.as_vec_mut().extend(value.to_bytes_with_nul());
        self
    }
    pub fn push_fmsg_obj_name_bytes(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 111u16, (value.len() + 1) as u16);
        self.as_vec_mut().extend(value);
        self.as_vec_mut().push(0);
        self
    }
}
impl<Prev: Pusher> Drop for PushDlFmsg<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushDlHealthReporter<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushDlHealthReporter<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushDlHealthReporter<Prev> {
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
    pub fn push_health_reporter_name(mut self, value: &CStr) -> Self {
        push_header(
            self.as_vec_mut(),
            115u16,
            value.to_bytes_with_nul().len() as u16,
        );
        self.as_vec_mut().extend(value.to_bytes_with_nul());
        self
    }
    pub fn push_health_reporter_name_bytes(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 115u16, (value.len() + 1) as u16);
        self.as_vec_mut().extend(value);
        self.as_vec_mut().push(0);
        self
    }
    pub fn push_health_reporter_state(mut self, value: u8) -> Self {
        push_header(self.as_vec_mut(), 116u16, 1 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_health_reporter_err_count(mut self, value: u64) -> Self {
        push_header(self.as_vec_mut(), 117u16, 8 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_health_reporter_recover_count(mut self, value: u64) -> Self {
        push_header(self.as_vec_mut(), 118u16, 8 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_health_reporter_dump_ts(mut self, value: u64) -> Self {
        push_header(self.as_vec_mut(), 119u16, 8 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_health_reporter_graceful_period(mut self, value: u64) -> Self {
        push_header(self.as_vec_mut(), 120u16, 8 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_health_reporter_auto_recover(mut self, value: u8) -> Self {
        push_header(self.as_vec_mut(), 121u16, 1 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_health_reporter_dump_ts_ns(mut self, value: u64) -> Self {
        push_header(self.as_vec_mut(), 137u16, 8 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_health_reporter_auto_dump(mut self, value: u8) -> Self {
        push_header(self.as_vec_mut(), 141u16, 1 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Time (in msec) for recoveries before starting the grace period.\n"]
    pub fn push_health_reporter_burst_period(mut self, value: u64) -> Self {
        push_header(self.as_vec_mut(), 181u16, 8 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
}
impl<Prev: Pusher> Drop for PushDlHealthReporter<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushDlAttrStats<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushDlAttrStats<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushDlAttrStats<Prev> {
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
    pub fn push_stats_rx_packets(mut self, value: u64) -> Self {
        push_header(self.as_vec_mut(), 0u16, 8 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_stats_rx_bytes(mut self, value: u64) -> Self {
        push_header(self.as_vec_mut(), 1u16, 8 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_stats_rx_dropped(mut self, value: u64) -> Self {
        push_header(self.as_vec_mut(), 2u16, 8 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
}
impl<Prev: Pusher> Drop for PushDlAttrStats<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushDlTrapMetadata<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushDlTrapMetadata<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushDlTrapMetadata<Prev> {
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
    pub fn push_trap_metadata_type_in_port(mut self, value: ()) -> Self {
        push_header(self.as_vec_mut(), 0u16, 0 as u16);
        self
    }
    pub fn push_trap_metadata_type_fa_cookie(mut self, value: ()) -> Self {
        push_header(self.as_vec_mut(), 1u16, 0 as u16);
        self
    }
}
impl<Prev: Pusher> Drop for PushDlTrapMetadata<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushDlLinecardSupportedTypes<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushDlLinecardSupportedTypes<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushDlLinecardSupportedTypes<Prev> {
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
    pub fn push_linecard_type(mut self, value: &CStr) -> Self {
        push_header(
            self.as_vec_mut(),
            173u16,
            value.to_bytes_with_nul().len() as u16,
        );
        self.as_vec_mut().extend(value.to_bytes_with_nul());
        self
    }
    pub fn push_linecard_type_bytes(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 173u16, (value.len() + 1) as u16);
        self.as_vec_mut().extend(value);
        self.as_vec_mut().push(0);
        self
    }
}
impl<Prev: Pusher> Drop for PushDlLinecardSupportedTypes<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushDlSelftestId<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushDlSelftestId<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushDlSelftestId<Prev> {
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
    pub fn push_flash(mut self, value: ()) -> Self {
        push_header(self.as_vec_mut(), 1u16, 0 as u16);
        self
    }
}
impl<Prev: Pusher> Drop for PushDlSelftestId<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushDlRateTcBws<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushDlRateTcBws<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushDlRateTcBws<Prev> {
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
    pub fn push_index(mut self, value: u8) -> Self {
        push_header(self.as_vec_mut(), 1u16, 1 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Specifies the bandwidth share assigned to the Traffic Class. The\nbandwidth for the traffic class is determined in proportion to the sum\nof the shares of all configured classes.\n"]
    pub fn push_bw(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 2u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
}
impl<Prev: Pusher> Drop for PushDlRateTcBws<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
#[doc = "Get devlink instances.\n\nReply attributes:\n- [.get_bus_name()](IterableDevlink::get_bus_name)\n- [.get_dev_name()](IterableDevlink::get_dev_name)\n- [.get_reload_failed()](IterableDevlink::get_reload_failed)\n- [.get_dev_stats()](IterableDevlink::get_dev_stats)\n- [.get_index()](IterableDevlink::get_index)\n\n"]
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
    pub fn encode_request<'buf>(buf: &'buf mut Vec<u8>) -> PushDevlink<&'buf mut Vec<u8>> {
        Self::write_header(buf);
        PushDevlink::new(buf)
    }
    pub fn encode(&mut self) -> PushDevlink<&mut Vec<u8>> {
        PushDevlink::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushDevlink<RequestBuf<'r>> {
        PushDevlink::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> IterableDevlink<'a> {
        let (_header, attrs) = buf.split_at(buf.len().min(BuiltinNfgenmsg::len()));
        IterableDevlink::with_loc(attrs, buf.as_ptr() as usize)
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
        Protocol::Generic("devlink".as_bytes())
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = IterableDevlink<'buf>;
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
#[doc = "Get devlink instances.\n\nRequest attributes:\n- [.push_bus_name()](PushDevlink::push_bus_name)\n- [.push_dev_name()](PushDevlink::push_dev_name)\n- [.push_index()](PushDevlink::push_index)\n\nReply attributes:\n- [.get_bus_name()](IterableDevlink::get_bus_name)\n- [.get_dev_name()](IterableDevlink::get_dev_name)\n- [.get_reload_failed()](IterableDevlink::get_reload_failed)\n- [.get_dev_stats()](IterableDevlink::get_dev_stats)\n- [.get_index()](IterableDevlink::get_index)\n\n"]
#[derive(Debug)]
pub struct OpGetDo<'r> {
    request: Request<'r>,
}
impl<'r> OpGetDo<'r> {
    pub fn new(mut request: Request<'r>) -> Self {
        Self::write_header(request.buf_mut());
        Self { request: request }
    }
    pub fn encode_request<'buf>(buf: &'buf mut Vec<u8>) -> PushDevlink<&'buf mut Vec<u8>> {
        Self::write_header(buf);
        PushDevlink::new(buf)
    }
    pub fn encode(&mut self) -> PushDevlink<&mut Vec<u8>> {
        PushDevlink::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushDevlink<RequestBuf<'r>> {
        PushDevlink::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> IterableDevlink<'a> {
        let (_header, attrs) = buf.split_at(buf.len().min(BuiltinNfgenmsg::len()));
        IterableDevlink::with_loc(attrs, buf.as_ptr() as usize)
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
        Protocol::Generic("devlink".as_bytes())
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = IterableDevlink<'buf>;
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
#[doc = "Get devlink port instances.\n\nRequest attributes:\n- [.push_bus_name()](PushDevlink::push_bus_name)\n- [.push_dev_name()](PushDevlink::push_dev_name)\n- [.push_index()](PushDevlink::push_index)\n\nReply attributes:\n- [.get_bus_name()](IterableDevlink::get_bus_name)\n- [.get_dev_name()](IterableDevlink::get_dev_name)\n- [.get_port_index()](IterableDevlink::get_port_index)\n- [.get_index()](IterableDevlink::get_index)\n\n"]
#[derive(Debug)]
pub struct OpPortGetDump<'r> {
    request: Request<'r>,
}
impl<'r> OpPortGetDump<'r> {
    pub fn new(mut request: Request<'r>) -> Self {
        Self::write_header(request.buf_mut());
        Self {
            request: request.set_dump(),
        }
    }
    pub fn encode_request<'buf>(buf: &'buf mut Vec<u8>) -> PushDevlink<&'buf mut Vec<u8>> {
        Self::write_header(buf);
        PushDevlink::new(buf)
    }
    pub fn encode(&mut self) -> PushDevlink<&mut Vec<u8>> {
        PushDevlink::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushDevlink<RequestBuf<'r>> {
        PushDevlink::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> IterableDevlink<'a> {
        let (_header, attrs) = buf.split_at(buf.len().min(BuiltinNfgenmsg::len()));
        IterableDevlink::with_loc(attrs, buf.as_ptr() as usize)
    }
    fn write_header<Prev: Pusher>(prev: &mut Prev) {
        let mut header = BuiltinNfgenmsg::new();
        header.cmd = 5u8;
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
impl NetlinkRequest for OpPortGetDump<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Generic("devlink".as_bytes())
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = IterableDevlink<'buf>;
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
#[doc = "Get devlink port instances.\n\nRequest attributes:\n- [.push_bus_name()](PushDevlink::push_bus_name)\n- [.push_dev_name()](PushDevlink::push_dev_name)\n- [.push_port_index()](PushDevlink::push_port_index)\n- [.push_index()](PushDevlink::push_index)\n\nReply attributes:\n- [.get_bus_name()](IterableDevlink::get_bus_name)\n- [.get_dev_name()](IterableDevlink::get_dev_name)\n- [.get_port_index()](IterableDevlink::get_port_index)\n- [.get_index()](IterableDevlink::get_index)\n\n"]
#[derive(Debug)]
pub struct OpPortGetDo<'r> {
    request: Request<'r>,
}
impl<'r> OpPortGetDo<'r> {
    pub fn new(mut request: Request<'r>) -> Self {
        Self::write_header(request.buf_mut());
        Self { request: request }
    }
    pub fn encode_request<'buf>(buf: &'buf mut Vec<u8>) -> PushDevlink<&'buf mut Vec<u8>> {
        Self::write_header(buf);
        PushDevlink::new(buf)
    }
    pub fn encode(&mut self) -> PushDevlink<&mut Vec<u8>> {
        PushDevlink::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushDevlink<RequestBuf<'r>> {
        PushDevlink::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> IterableDevlink<'a> {
        let (_header, attrs) = buf.split_at(buf.len().min(BuiltinNfgenmsg::len()));
        IterableDevlink::with_loc(attrs, buf.as_ptr() as usize)
    }
    fn write_header<Prev: Pusher>(prev: &mut Prev) {
        let mut header = BuiltinNfgenmsg::new();
        header.cmd = 5u8;
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
impl NetlinkRequest for OpPortGetDo<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Generic("devlink".as_bytes())
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = IterableDevlink<'buf>;
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
#[doc = "Set devlink port instances.\n\nFlags: admin-perm\n\nRequest attributes:\n- [.push_bus_name()](PushDevlink::push_bus_name)\n- [.push_dev_name()](PushDevlink::push_dev_name)\n- [.push_port_index()](PushDevlink::push_port_index)\n- [.push_port_type()](PushDevlink::push_port_type)\n- [.nested_port_function()](PushDevlink::nested_port_function)\n- [.push_index()](PushDevlink::push_index)\n\n"]
#[derive(Debug)]
pub struct OpPortSetDo<'r> {
    request: Request<'r>,
}
impl<'r> OpPortSetDo<'r> {
    pub fn new(mut request: Request<'r>) -> Self {
        Self::write_header(request.buf_mut());
        Self { request: request }
    }
    pub fn encode_request<'buf>(buf: &'buf mut Vec<u8>) -> PushDevlink<&'buf mut Vec<u8>> {
        Self::write_header(buf);
        PushDevlink::new(buf)
    }
    pub fn encode(&mut self) -> PushDevlink<&mut Vec<u8>> {
        PushDevlink::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushDevlink<RequestBuf<'r>> {
        PushDevlink::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> IterableDevlink<'a> {
        let (_header, attrs) = buf.split_at(buf.len().min(BuiltinNfgenmsg::len()));
        IterableDevlink::with_loc(attrs, buf.as_ptr() as usize)
    }
    fn write_header<Prev: Pusher>(prev: &mut Prev) {
        let mut header = BuiltinNfgenmsg::new();
        header.cmd = 6u8;
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
impl NetlinkRequest for OpPortSetDo<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Generic("devlink".as_bytes())
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = IterableDevlink<'buf>;
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
#[doc = "Create devlink port instances.\n\nFlags: admin-perm\n\nRequest attributes:\n- [.push_bus_name()](PushDevlink::push_bus_name)\n- [.push_dev_name()](PushDevlink::push_dev_name)\n- [.push_port_index()](PushDevlink::push_port_index)\n- [.push_port_flavour()](PushDevlink::push_port_flavour)\n- [.push_port_pci_pf_number()](PushDevlink::push_port_pci_pf_number)\n- [.push_port_controller_number()](PushDevlink::push_port_controller_number)\n- [.push_port_pci_sf_number()](PushDevlink::push_port_pci_sf_number)\n- [.push_index()](PushDevlink::push_index)\n\nReply attributes:\n- [.get_bus_name()](IterableDevlink::get_bus_name)\n- [.get_dev_name()](IterableDevlink::get_dev_name)\n- [.get_port_index()](IterableDevlink::get_port_index)\n- [.get_index()](IterableDevlink::get_index)\n\n"]
#[derive(Debug)]
pub struct OpPortNewDo<'r> {
    request: Request<'r>,
}
impl<'r> OpPortNewDo<'r> {
    pub fn new(mut request: Request<'r>) -> Self {
        Self::write_header(request.buf_mut());
        Self { request: request }
    }
    pub fn encode_request<'buf>(buf: &'buf mut Vec<u8>) -> PushDevlink<&'buf mut Vec<u8>> {
        Self::write_header(buf);
        PushDevlink::new(buf)
    }
    pub fn encode(&mut self) -> PushDevlink<&mut Vec<u8>> {
        PushDevlink::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushDevlink<RequestBuf<'r>> {
        PushDevlink::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> IterableDevlink<'a> {
        let (_header, attrs) = buf.split_at(buf.len().min(BuiltinNfgenmsg::len()));
        IterableDevlink::with_loc(attrs, buf.as_ptr() as usize)
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
impl NetlinkRequest for OpPortNewDo<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Generic("devlink".as_bytes())
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = IterableDevlink<'buf>;
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
#[doc = "Delete devlink port instances.\n\nFlags: admin-perm\n\nRequest attributes:\n- [.push_bus_name()](PushDevlink::push_bus_name)\n- [.push_dev_name()](PushDevlink::push_dev_name)\n- [.push_port_index()](PushDevlink::push_port_index)\n- [.push_index()](PushDevlink::push_index)\n\n"]
#[derive(Debug)]
pub struct OpPortDelDo<'r> {
    request: Request<'r>,
}
impl<'r> OpPortDelDo<'r> {
    pub fn new(mut request: Request<'r>) -> Self {
        Self::write_header(request.buf_mut());
        Self { request: request }
    }
    pub fn encode_request<'buf>(buf: &'buf mut Vec<u8>) -> PushDevlink<&'buf mut Vec<u8>> {
        Self::write_header(buf);
        PushDevlink::new(buf)
    }
    pub fn encode(&mut self) -> PushDevlink<&mut Vec<u8>> {
        PushDevlink::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushDevlink<RequestBuf<'r>> {
        PushDevlink::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> IterableDevlink<'a> {
        let (_header, attrs) = buf.split_at(buf.len().min(BuiltinNfgenmsg::len()));
        IterableDevlink::with_loc(attrs, buf.as_ptr() as usize)
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
impl NetlinkRequest for OpPortDelDo<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Generic("devlink".as_bytes())
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = IterableDevlink<'buf>;
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
#[doc = "Split devlink port instances.\n\nFlags: admin-perm\n\nRequest attributes:\n- [.push_bus_name()](PushDevlink::push_bus_name)\n- [.push_dev_name()](PushDevlink::push_dev_name)\n- [.push_port_index()](PushDevlink::push_port_index)\n- [.push_port_split_count()](PushDevlink::push_port_split_count)\n- [.push_index()](PushDevlink::push_index)\n\n"]
#[derive(Debug)]
pub struct OpPortSplitDo<'r> {
    request: Request<'r>,
}
impl<'r> OpPortSplitDo<'r> {
    pub fn new(mut request: Request<'r>) -> Self {
        Self::write_header(request.buf_mut());
        Self { request: request }
    }
    pub fn encode_request<'buf>(buf: &'buf mut Vec<u8>) -> PushDevlink<&'buf mut Vec<u8>> {
        Self::write_header(buf);
        PushDevlink::new(buf)
    }
    pub fn encode(&mut self) -> PushDevlink<&mut Vec<u8>> {
        PushDevlink::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushDevlink<RequestBuf<'r>> {
        PushDevlink::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> IterableDevlink<'a> {
        let (_header, attrs) = buf.split_at(buf.len().min(BuiltinNfgenmsg::len()));
        IterableDevlink::with_loc(attrs, buf.as_ptr() as usize)
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
impl NetlinkRequest for OpPortSplitDo<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Generic("devlink".as_bytes())
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = IterableDevlink<'buf>;
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
#[doc = "Unplit devlink port instances.\n\nFlags: admin-perm\n\nRequest attributes:\n- [.push_bus_name()](PushDevlink::push_bus_name)\n- [.push_dev_name()](PushDevlink::push_dev_name)\n- [.push_port_index()](PushDevlink::push_port_index)\n- [.push_index()](PushDevlink::push_index)\n\n"]
#[derive(Debug)]
pub struct OpPortUnsplitDo<'r> {
    request: Request<'r>,
}
impl<'r> OpPortUnsplitDo<'r> {
    pub fn new(mut request: Request<'r>) -> Self {
        Self::write_header(request.buf_mut());
        Self { request: request }
    }
    pub fn encode_request<'buf>(buf: &'buf mut Vec<u8>) -> PushDevlink<&'buf mut Vec<u8>> {
        Self::write_header(buf);
        PushDevlink::new(buf)
    }
    pub fn encode(&mut self) -> PushDevlink<&mut Vec<u8>> {
        PushDevlink::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushDevlink<RequestBuf<'r>> {
        PushDevlink::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> IterableDevlink<'a> {
        let (_header, attrs) = buf.split_at(buf.len().min(BuiltinNfgenmsg::len()));
        IterableDevlink::with_loc(attrs, buf.as_ptr() as usize)
    }
    fn write_header<Prev: Pusher>(prev: &mut Prev) {
        let mut header = BuiltinNfgenmsg::new();
        header.cmd = 10u8;
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
impl NetlinkRequest for OpPortUnsplitDo<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Generic("devlink".as_bytes())
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = IterableDevlink<'buf>;
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
#[doc = "Get shared buffer instances.\n\nRequest attributes:\n- [.push_bus_name()](PushDevlink::push_bus_name)\n- [.push_dev_name()](PushDevlink::push_dev_name)\n- [.push_index()](PushDevlink::push_index)\n\nReply attributes:\n- [.get_bus_name()](IterableDevlink::get_bus_name)\n- [.get_dev_name()](IterableDevlink::get_dev_name)\n- [.get_sb_index()](IterableDevlink::get_sb_index)\n- [.get_index()](IterableDevlink::get_index)\n\n"]
#[derive(Debug)]
pub struct OpSbGetDump<'r> {
    request: Request<'r>,
}
impl<'r> OpSbGetDump<'r> {
    pub fn new(mut request: Request<'r>) -> Self {
        Self::write_header(request.buf_mut());
        Self {
            request: request.set_dump(),
        }
    }
    pub fn encode_request<'buf>(buf: &'buf mut Vec<u8>) -> PushDevlink<&'buf mut Vec<u8>> {
        Self::write_header(buf);
        PushDevlink::new(buf)
    }
    pub fn encode(&mut self) -> PushDevlink<&mut Vec<u8>> {
        PushDevlink::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushDevlink<RequestBuf<'r>> {
        PushDevlink::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> IterableDevlink<'a> {
        let (_header, attrs) = buf.split_at(buf.len().min(BuiltinNfgenmsg::len()));
        IterableDevlink::with_loc(attrs, buf.as_ptr() as usize)
    }
    fn write_header<Prev: Pusher>(prev: &mut Prev) {
        let mut header = BuiltinNfgenmsg::new();
        header.cmd = 11u8;
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
impl NetlinkRequest for OpSbGetDump<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Generic("devlink".as_bytes())
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = IterableDevlink<'buf>;
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
#[doc = "Get shared buffer instances.\n\nRequest attributes:\n- [.push_bus_name()](PushDevlink::push_bus_name)\n- [.push_dev_name()](PushDevlink::push_dev_name)\n- [.push_sb_index()](PushDevlink::push_sb_index)\n- [.push_index()](PushDevlink::push_index)\n\nReply attributes:\n- [.get_bus_name()](IterableDevlink::get_bus_name)\n- [.get_dev_name()](IterableDevlink::get_dev_name)\n- [.get_sb_index()](IterableDevlink::get_sb_index)\n- [.get_index()](IterableDevlink::get_index)\n\n"]
#[derive(Debug)]
pub struct OpSbGetDo<'r> {
    request: Request<'r>,
}
impl<'r> OpSbGetDo<'r> {
    pub fn new(mut request: Request<'r>) -> Self {
        Self::write_header(request.buf_mut());
        Self { request: request }
    }
    pub fn encode_request<'buf>(buf: &'buf mut Vec<u8>) -> PushDevlink<&'buf mut Vec<u8>> {
        Self::write_header(buf);
        PushDevlink::new(buf)
    }
    pub fn encode(&mut self) -> PushDevlink<&mut Vec<u8>> {
        PushDevlink::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushDevlink<RequestBuf<'r>> {
        PushDevlink::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> IterableDevlink<'a> {
        let (_header, attrs) = buf.split_at(buf.len().min(BuiltinNfgenmsg::len()));
        IterableDevlink::with_loc(attrs, buf.as_ptr() as usize)
    }
    fn write_header<Prev: Pusher>(prev: &mut Prev) {
        let mut header = BuiltinNfgenmsg::new();
        header.cmd = 11u8;
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
impl NetlinkRequest for OpSbGetDo<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Generic("devlink".as_bytes())
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = IterableDevlink<'buf>;
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
#[doc = "Get shared buffer pool instances.\n\nRequest attributes:\n- [.push_bus_name()](PushDevlink::push_bus_name)\n- [.push_dev_name()](PushDevlink::push_dev_name)\n- [.push_index()](PushDevlink::push_index)\n\nReply attributes:\n- [.get_bus_name()](IterableDevlink::get_bus_name)\n- [.get_dev_name()](IterableDevlink::get_dev_name)\n- [.get_sb_index()](IterableDevlink::get_sb_index)\n- [.get_sb_pool_index()](IterableDevlink::get_sb_pool_index)\n- [.get_index()](IterableDevlink::get_index)\n\n"]
#[derive(Debug)]
pub struct OpSbPoolGetDump<'r> {
    request: Request<'r>,
}
impl<'r> OpSbPoolGetDump<'r> {
    pub fn new(mut request: Request<'r>) -> Self {
        Self::write_header(request.buf_mut());
        Self {
            request: request.set_dump(),
        }
    }
    pub fn encode_request<'buf>(buf: &'buf mut Vec<u8>) -> PushDevlink<&'buf mut Vec<u8>> {
        Self::write_header(buf);
        PushDevlink::new(buf)
    }
    pub fn encode(&mut self) -> PushDevlink<&mut Vec<u8>> {
        PushDevlink::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushDevlink<RequestBuf<'r>> {
        PushDevlink::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> IterableDevlink<'a> {
        let (_header, attrs) = buf.split_at(buf.len().min(BuiltinNfgenmsg::len()));
        IterableDevlink::with_loc(attrs, buf.as_ptr() as usize)
    }
    fn write_header<Prev: Pusher>(prev: &mut Prev) {
        let mut header = BuiltinNfgenmsg::new();
        header.cmd = 15u8;
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
impl NetlinkRequest for OpSbPoolGetDump<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Generic("devlink".as_bytes())
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = IterableDevlink<'buf>;
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
#[doc = "Get shared buffer pool instances.\n\nRequest attributes:\n- [.push_bus_name()](PushDevlink::push_bus_name)\n- [.push_dev_name()](PushDevlink::push_dev_name)\n- [.push_sb_index()](PushDevlink::push_sb_index)\n- [.push_sb_pool_index()](PushDevlink::push_sb_pool_index)\n- [.push_index()](PushDevlink::push_index)\n\nReply attributes:\n- [.get_bus_name()](IterableDevlink::get_bus_name)\n- [.get_dev_name()](IterableDevlink::get_dev_name)\n- [.get_sb_index()](IterableDevlink::get_sb_index)\n- [.get_sb_pool_index()](IterableDevlink::get_sb_pool_index)\n- [.get_index()](IterableDevlink::get_index)\n\n"]
#[derive(Debug)]
pub struct OpSbPoolGetDo<'r> {
    request: Request<'r>,
}
impl<'r> OpSbPoolGetDo<'r> {
    pub fn new(mut request: Request<'r>) -> Self {
        Self::write_header(request.buf_mut());
        Self { request: request }
    }
    pub fn encode_request<'buf>(buf: &'buf mut Vec<u8>) -> PushDevlink<&'buf mut Vec<u8>> {
        Self::write_header(buf);
        PushDevlink::new(buf)
    }
    pub fn encode(&mut self) -> PushDevlink<&mut Vec<u8>> {
        PushDevlink::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushDevlink<RequestBuf<'r>> {
        PushDevlink::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> IterableDevlink<'a> {
        let (_header, attrs) = buf.split_at(buf.len().min(BuiltinNfgenmsg::len()));
        IterableDevlink::with_loc(attrs, buf.as_ptr() as usize)
    }
    fn write_header<Prev: Pusher>(prev: &mut Prev) {
        let mut header = BuiltinNfgenmsg::new();
        header.cmd = 15u8;
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
impl NetlinkRequest for OpSbPoolGetDo<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Generic("devlink".as_bytes())
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = IterableDevlink<'buf>;
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
#[doc = "Set shared buffer pool instances.\n\nFlags: admin-perm\n\nRequest attributes:\n- [.push_bus_name()](PushDevlink::push_bus_name)\n- [.push_dev_name()](PushDevlink::push_dev_name)\n- [.push_sb_index()](PushDevlink::push_sb_index)\n- [.push_sb_pool_index()](PushDevlink::push_sb_pool_index)\n- [.push_sb_pool_size()](PushDevlink::push_sb_pool_size)\n- [.push_sb_pool_threshold_type()](PushDevlink::push_sb_pool_threshold_type)\n- [.push_index()](PushDevlink::push_index)\n\n"]
#[derive(Debug)]
pub struct OpSbPoolSetDo<'r> {
    request: Request<'r>,
}
impl<'r> OpSbPoolSetDo<'r> {
    pub fn new(mut request: Request<'r>) -> Self {
        Self::write_header(request.buf_mut());
        Self { request: request }
    }
    pub fn encode_request<'buf>(buf: &'buf mut Vec<u8>) -> PushDevlink<&'buf mut Vec<u8>> {
        Self::write_header(buf);
        PushDevlink::new(buf)
    }
    pub fn encode(&mut self) -> PushDevlink<&mut Vec<u8>> {
        PushDevlink::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushDevlink<RequestBuf<'r>> {
        PushDevlink::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> IterableDevlink<'a> {
        let (_header, attrs) = buf.split_at(buf.len().min(BuiltinNfgenmsg::len()));
        IterableDevlink::with_loc(attrs, buf.as_ptr() as usize)
    }
    fn write_header<Prev: Pusher>(prev: &mut Prev) {
        let mut header = BuiltinNfgenmsg::new();
        header.cmd = 16u8;
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
impl NetlinkRequest for OpSbPoolSetDo<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Generic("devlink".as_bytes())
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = IterableDevlink<'buf>;
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
#[doc = "Get shared buffer port-pool combinations and threshold.\n\nRequest attributes:\n- [.push_bus_name()](PushDevlink::push_bus_name)\n- [.push_dev_name()](PushDevlink::push_dev_name)\n- [.push_index()](PushDevlink::push_index)\n\nReply attributes:\n- [.get_bus_name()](IterableDevlink::get_bus_name)\n- [.get_dev_name()](IterableDevlink::get_dev_name)\n- [.get_port_index()](IterableDevlink::get_port_index)\n- [.get_sb_index()](IterableDevlink::get_sb_index)\n- [.get_sb_pool_index()](IterableDevlink::get_sb_pool_index)\n- [.get_index()](IterableDevlink::get_index)\n\n"]
#[derive(Debug)]
pub struct OpSbPortPoolGetDump<'r> {
    request: Request<'r>,
}
impl<'r> OpSbPortPoolGetDump<'r> {
    pub fn new(mut request: Request<'r>) -> Self {
        Self::write_header(request.buf_mut());
        Self {
            request: request.set_dump(),
        }
    }
    pub fn encode_request<'buf>(buf: &'buf mut Vec<u8>) -> PushDevlink<&'buf mut Vec<u8>> {
        Self::write_header(buf);
        PushDevlink::new(buf)
    }
    pub fn encode(&mut self) -> PushDevlink<&mut Vec<u8>> {
        PushDevlink::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushDevlink<RequestBuf<'r>> {
        PushDevlink::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> IterableDevlink<'a> {
        let (_header, attrs) = buf.split_at(buf.len().min(BuiltinNfgenmsg::len()));
        IterableDevlink::with_loc(attrs, buf.as_ptr() as usize)
    }
    fn write_header<Prev: Pusher>(prev: &mut Prev) {
        let mut header = BuiltinNfgenmsg::new();
        header.cmd = 19u8;
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
impl NetlinkRequest for OpSbPortPoolGetDump<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Generic("devlink".as_bytes())
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = IterableDevlink<'buf>;
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
#[doc = "Get shared buffer port-pool combinations and threshold.\n\nRequest attributes:\n- [.push_bus_name()](PushDevlink::push_bus_name)\n- [.push_dev_name()](PushDevlink::push_dev_name)\n- [.push_port_index()](PushDevlink::push_port_index)\n- [.push_sb_index()](PushDevlink::push_sb_index)\n- [.push_sb_pool_index()](PushDevlink::push_sb_pool_index)\n- [.push_index()](PushDevlink::push_index)\n\nReply attributes:\n- [.get_bus_name()](IterableDevlink::get_bus_name)\n- [.get_dev_name()](IterableDevlink::get_dev_name)\n- [.get_port_index()](IterableDevlink::get_port_index)\n- [.get_sb_index()](IterableDevlink::get_sb_index)\n- [.get_sb_pool_index()](IterableDevlink::get_sb_pool_index)\n- [.get_index()](IterableDevlink::get_index)\n\n"]
#[derive(Debug)]
pub struct OpSbPortPoolGetDo<'r> {
    request: Request<'r>,
}
impl<'r> OpSbPortPoolGetDo<'r> {
    pub fn new(mut request: Request<'r>) -> Self {
        Self::write_header(request.buf_mut());
        Self { request: request }
    }
    pub fn encode_request<'buf>(buf: &'buf mut Vec<u8>) -> PushDevlink<&'buf mut Vec<u8>> {
        Self::write_header(buf);
        PushDevlink::new(buf)
    }
    pub fn encode(&mut self) -> PushDevlink<&mut Vec<u8>> {
        PushDevlink::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushDevlink<RequestBuf<'r>> {
        PushDevlink::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> IterableDevlink<'a> {
        let (_header, attrs) = buf.split_at(buf.len().min(BuiltinNfgenmsg::len()));
        IterableDevlink::with_loc(attrs, buf.as_ptr() as usize)
    }
    fn write_header<Prev: Pusher>(prev: &mut Prev) {
        let mut header = BuiltinNfgenmsg::new();
        header.cmd = 19u8;
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
impl NetlinkRequest for OpSbPortPoolGetDo<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Generic("devlink".as_bytes())
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = IterableDevlink<'buf>;
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
#[doc = "Set shared buffer port-pool combinations and threshold.\n\nFlags: admin-perm\n\nRequest attributes:\n- [.push_bus_name()](PushDevlink::push_bus_name)\n- [.push_dev_name()](PushDevlink::push_dev_name)\n- [.push_port_index()](PushDevlink::push_port_index)\n- [.push_sb_index()](PushDevlink::push_sb_index)\n- [.push_sb_pool_index()](PushDevlink::push_sb_pool_index)\n- [.push_sb_threshold()](PushDevlink::push_sb_threshold)\n- [.push_index()](PushDevlink::push_index)\n\n"]
#[derive(Debug)]
pub struct OpSbPortPoolSetDo<'r> {
    request: Request<'r>,
}
impl<'r> OpSbPortPoolSetDo<'r> {
    pub fn new(mut request: Request<'r>) -> Self {
        Self::write_header(request.buf_mut());
        Self { request: request }
    }
    pub fn encode_request<'buf>(buf: &'buf mut Vec<u8>) -> PushDevlink<&'buf mut Vec<u8>> {
        Self::write_header(buf);
        PushDevlink::new(buf)
    }
    pub fn encode(&mut self) -> PushDevlink<&mut Vec<u8>> {
        PushDevlink::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushDevlink<RequestBuf<'r>> {
        PushDevlink::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> IterableDevlink<'a> {
        let (_header, attrs) = buf.split_at(buf.len().min(BuiltinNfgenmsg::len()));
        IterableDevlink::with_loc(attrs, buf.as_ptr() as usize)
    }
    fn write_header<Prev: Pusher>(prev: &mut Prev) {
        let mut header = BuiltinNfgenmsg::new();
        header.cmd = 20u8;
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
impl NetlinkRequest for OpSbPortPoolSetDo<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Generic("devlink".as_bytes())
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = IterableDevlink<'buf>;
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
#[doc = "Get shared buffer port-TC to pool bindings and threshold.\n\nRequest attributes:\n- [.push_bus_name()](PushDevlink::push_bus_name)\n- [.push_dev_name()](PushDevlink::push_dev_name)\n- [.push_index()](PushDevlink::push_index)\n\nReply attributes:\n- [.get_bus_name()](IterableDevlink::get_bus_name)\n- [.get_dev_name()](IterableDevlink::get_dev_name)\n- [.get_port_index()](IterableDevlink::get_port_index)\n- [.get_sb_index()](IterableDevlink::get_sb_index)\n- [.get_sb_pool_type()](IterableDevlink::get_sb_pool_type)\n- [.get_sb_tc_index()](IterableDevlink::get_sb_tc_index)\n- [.get_index()](IterableDevlink::get_index)\n\n"]
#[derive(Debug)]
pub struct OpSbTcPoolBindGetDump<'r> {
    request: Request<'r>,
}
impl<'r> OpSbTcPoolBindGetDump<'r> {
    pub fn new(mut request: Request<'r>) -> Self {
        Self::write_header(request.buf_mut());
        Self {
            request: request.set_dump(),
        }
    }
    pub fn encode_request<'buf>(buf: &'buf mut Vec<u8>) -> PushDevlink<&'buf mut Vec<u8>> {
        Self::write_header(buf);
        PushDevlink::new(buf)
    }
    pub fn encode(&mut self) -> PushDevlink<&mut Vec<u8>> {
        PushDevlink::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushDevlink<RequestBuf<'r>> {
        PushDevlink::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> IterableDevlink<'a> {
        let (_header, attrs) = buf.split_at(buf.len().min(BuiltinNfgenmsg::len()));
        IterableDevlink::with_loc(attrs, buf.as_ptr() as usize)
    }
    fn write_header<Prev: Pusher>(prev: &mut Prev) {
        let mut header = BuiltinNfgenmsg::new();
        header.cmd = 23u8;
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
impl NetlinkRequest for OpSbTcPoolBindGetDump<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Generic("devlink".as_bytes())
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = IterableDevlink<'buf>;
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
#[doc = "Get shared buffer port-TC to pool bindings and threshold.\n\nRequest attributes:\n- [.push_bus_name()](PushDevlink::push_bus_name)\n- [.push_dev_name()](PushDevlink::push_dev_name)\n- [.push_port_index()](PushDevlink::push_port_index)\n- [.push_sb_index()](PushDevlink::push_sb_index)\n- [.push_sb_pool_type()](PushDevlink::push_sb_pool_type)\n- [.push_sb_tc_index()](PushDevlink::push_sb_tc_index)\n- [.push_index()](PushDevlink::push_index)\n\nReply attributes:\n- [.get_bus_name()](IterableDevlink::get_bus_name)\n- [.get_dev_name()](IterableDevlink::get_dev_name)\n- [.get_port_index()](IterableDevlink::get_port_index)\n- [.get_sb_index()](IterableDevlink::get_sb_index)\n- [.get_sb_pool_type()](IterableDevlink::get_sb_pool_type)\n- [.get_sb_tc_index()](IterableDevlink::get_sb_tc_index)\n- [.get_index()](IterableDevlink::get_index)\n\n"]
#[derive(Debug)]
pub struct OpSbTcPoolBindGetDo<'r> {
    request: Request<'r>,
}
impl<'r> OpSbTcPoolBindGetDo<'r> {
    pub fn new(mut request: Request<'r>) -> Self {
        Self::write_header(request.buf_mut());
        Self { request: request }
    }
    pub fn encode_request<'buf>(buf: &'buf mut Vec<u8>) -> PushDevlink<&'buf mut Vec<u8>> {
        Self::write_header(buf);
        PushDevlink::new(buf)
    }
    pub fn encode(&mut self) -> PushDevlink<&mut Vec<u8>> {
        PushDevlink::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushDevlink<RequestBuf<'r>> {
        PushDevlink::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> IterableDevlink<'a> {
        let (_header, attrs) = buf.split_at(buf.len().min(BuiltinNfgenmsg::len()));
        IterableDevlink::with_loc(attrs, buf.as_ptr() as usize)
    }
    fn write_header<Prev: Pusher>(prev: &mut Prev) {
        let mut header = BuiltinNfgenmsg::new();
        header.cmd = 23u8;
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
impl NetlinkRequest for OpSbTcPoolBindGetDo<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Generic("devlink".as_bytes())
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = IterableDevlink<'buf>;
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
#[doc = "Set shared buffer port-TC to pool bindings and threshold.\n\nFlags: admin-perm\n\nRequest attributes:\n- [.push_bus_name()](PushDevlink::push_bus_name)\n- [.push_dev_name()](PushDevlink::push_dev_name)\n- [.push_port_index()](PushDevlink::push_port_index)\n- [.push_sb_index()](PushDevlink::push_sb_index)\n- [.push_sb_pool_index()](PushDevlink::push_sb_pool_index)\n- [.push_sb_pool_type()](PushDevlink::push_sb_pool_type)\n- [.push_sb_threshold()](PushDevlink::push_sb_threshold)\n- [.push_sb_tc_index()](PushDevlink::push_sb_tc_index)\n- [.push_index()](PushDevlink::push_index)\n\n"]
#[derive(Debug)]
pub struct OpSbTcPoolBindSetDo<'r> {
    request: Request<'r>,
}
impl<'r> OpSbTcPoolBindSetDo<'r> {
    pub fn new(mut request: Request<'r>) -> Self {
        Self::write_header(request.buf_mut());
        Self { request: request }
    }
    pub fn encode_request<'buf>(buf: &'buf mut Vec<u8>) -> PushDevlink<&'buf mut Vec<u8>> {
        Self::write_header(buf);
        PushDevlink::new(buf)
    }
    pub fn encode(&mut self) -> PushDevlink<&mut Vec<u8>> {
        PushDevlink::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushDevlink<RequestBuf<'r>> {
        PushDevlink::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> IterableDevlink<'a> {
        let (_header, attrs) = buf.split_at(buf.len().min(BuiltinNfgenmsg::len()));
        IterableDevlink::with_loc(attrs, buf.as_ptr() as usize)
    }
    fn write_header<Prev: Pusher>(prev: &mut Prev) {
        let mut header = BuiltinNfgenmsg::new();
        header.cmd = 24u8;
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
impl NetlinkRequest for OpSbTcPoolBindSetDo<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Generic("devlink".as_bytes())
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = IterableDevlink<'buf>;
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
#[doc = "Take occupancy snapshot of shared buffer.\n\nFlags: admin-perm\n\nRequest attributes:\n- [.push_bus_name()](PushDevlink::push_bus_name)\n- [.push_dev_name()](PushDevlink::push_dev_name)\n- [.push_sb_index()](PushDevlink::push_sb_index)\n- [.push_index()](PushDevlink::push_index)\n\n"]
#[derive(Debug)]
pub struct OpSbOccSnapshotDo<'r> {
    request: Request<'r>,
}
impl<'r> OpSbOccSnapshotDo<'r> {
    pub fn new(mut request: Request<'r>) -> Self {
        Self::write_header(request.buf_mut());
        Self { request: request }
    }
    pub fn encode_request<'buf>(buf: &'buf mut Vec<u8>) -> PushDevlink<&'buf mut Vec<u8>> {
        Self::write_header(buf);
        PushDevlink::new(buf)
    }
    pub fn encode(&mut self) -> PushDevlink<&mut Vec<u8>> {
        PushDevlink::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushDevlink<RequestBuf<'r>> {
        PushDevlink::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> IterableDevlink<'a> {
        let (_header, attrs) = buf.split_at(buf.len().min(BuiltinNfgenmsg::len()));
        IterableDevlink::with_loc(attrs, buf.as_ptr() as usize)
    }
    fn write_header<Prev: Pusher>(prev: &mut Prev) {
        let mut header = BuiltinNfgenmsg::new();
        header.cmd = 27u8;
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
impl NetlinkRequest for OpSbOccSnapshotDo<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Generic("devlink".as_bytes())
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = IterableDevlink<'buf>;
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
#[doc = "Clear occupancy watermarks of shared buffer.\n\nFlags: admin-perm\n\nRequest attributes:\n- [.push_bus_name()](PushDevlink::push_bus_name)\n- [.push_dev_name()](PushDevlink::push_dev_name)\n- [.push_sb_index()](PushDevlink::push_sb_index)\n- [.push_index()](PushDevlink::push_index)\n\n"]
#[derive(Debug)]
pub struct OpSbOccMaxClearDo<'r> {
    request: Request<'r>,
}
impl<'r> OpSbOccMaxClearDo<'r> {
    pub fn new(mut request: Request<'r>) -> Self {
        Self::write_header(request.buf_mut());
        Self { request: request }
    }
    pub fn encode_request<'buf>(buf: &'buf mut Vec<u8>) -> PushDevlink<&'buf mut Vec<u8>> {
        Self::write_header(buf);
        PushDevlink::new(buf)
    }
    pub fn encode(&mut self) -> PushDevlink<&mut Vec<u8>> {
        PushDevlink::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushDevlink<RequestBuf<'r>> {
        PushDevlink::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> IterableDevlink<'a> {
        let (_header, attrs) = buf.split_at(buf.len().min(BuiltinNfgenmsg::len()));
        IterableDevlink::with_loc(attrs, buf.as_ptr() as usize)
    }
    fn write_header<Prev: Pusher>(prev: &mut Prev) {
        let mut header = BuiltinNfgenmsg::new();
        header.cmd = 28u8;
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
impl NetlinkRequest for OpSbOccMaxClearDo<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Generic("devlink".as_bytes())
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = IterableDevlink<'buf>;
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
#[doc = "Get eswitch attributes.\n\nFlags: admin-perm\n\nRequest attributes:\n- [.push_bus_name()](PushDevlink::push_bus_name)\n- [.push_dev_name()](PushDevlink::push_dev_name)\n- [.push_index()](PushDevlink::push_index)\n\nReply attributes:\n- [.get_bus_name()](IterableDevlink::get_bus_name)\n- [.get_dev_name()](IterableDevlink::get_dev_name)\n- [.get_eswitch_mode()](IterableDevlink::get_eswitch_mode)\n- [.get_eswitch_inline_mode()](IterableDevlink::get_eswitch_inline_mode)\n- [.get_eswitch_encap_mode()](IterableDevlink::get_eswitch_encap_mode)\n- [.get_index()](IterableDevlink::get_index)\n\n"]
#[derive(Debug)]
pub struct OpEswitchGetDo<'r> {
    request: Request<'r>,
}
impl<'r> OpEswitchGetDo<'r> {
    pub fn new(mut request: Request<'r>) -> Self {
        Self::write_header(request.buf_mut());
        Self { request: request }
    }
    pub fn encode_request<'buf>(buf: &'buf mut Vec<u8>) -> PushDevlink<&'buf mut Vec<u8>> {
        Self::write_header(buf);
        PushDevlink::new(buf)
    }
    pub fn encode(&mut self) -> PushDevlink<&mut Vec<u8>> {
        PushDevlink::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushDevlink<RequestBuf<'r>> {
        PushDevlink::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> IterableDevlink<'a> {
        let (_header, attrs) = buf.split_at(buf.len().min(BuiltinNfgenmsg::len()));
        IterableDevlink::with_loc(attrs, buf.as_ptr() as usize)
    }
    fn write_header<Prev: Pusher>(prev: &mut Prev) {
        let mut header = BuiltinNfgenmsg::new();
        header.cmd = 29u8;
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
impl NetlinkRequest for OpEswitchGetDo<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Generic("devlink".as_bytes())
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = IterableDevlink<'buf>;
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
#[doc = "Set eswitch attributes.\n\nFlags: admin-perm\n\nRequest attributes:\n- [.push_bus_name()](PushDevlink::push_bus_name)\n- [.push_dev_name()](PushDevlink::push_dev_name)\n- [.push_eswitch_mode()](PushDevlink::push_eswitch_mode)\n- [.push_eswitch_inline_mode()](PushDevlink::push_eswitch_inline_mode)\n- [.push_eswitch_encap_mode()](PushDevlink::push_eswitch_encap_mode)\n- [.push_index()](PushDevlink::push_index)\n\n"]
#[derive(Debug)]
pub struct OpEswitchSetDo<'r> {
    request: Request<'r>,
}
impl<'r> OpEswitchSetDo<'r> {
    pub fn new(mut request: Request<'r>) -> Self {
        Self::write_header(request.buf_mut());
        Self { request: request }
    }
    pub fn encode_request<'buf>(buf: &'buf mut Vec<u8>) -> PushDevlink<&'buf mut Vec<u8>> {
        Self::write_header(buf);
        PushDevlink::new(buf)
    }
    pub fn encode(&mut self) -> PushDevlink<&mut Vec<u8>> {
        PushDevlink::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushDevlink<RequestBuf<'r>> {
        PushDevlink::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> IterableDevlink<'a> {
        let (_header, attrs) = buf.split_at(buf.len().min(BuiltinNfgenmsg::len()));
        IterableDevlink::with_loc(attrs, buf.as_ptr() as usize)
    }
    fn write_header<Prev: Pusher>(prev: &mut Prev) {
        let mut header = BuiltinNfgenmsg::new();
        header.cmd = 30u8;
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
impl NetlinkRequest for OpEswitchSetDo<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Generic("devlink".as_bytes())
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = IterableDevlink<'buf>;
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
#[doc = "Get dpipe table attributes.\n\nRequest attributes:\n- [.push_bus_name()](PushDevlink::push_bus_name)\n- [.push_dev_name()](PushDevlink::push_dev_name)\n- [.push_dpipe_table_name()](PushDevlink::push_dpipe_table_name)\n- [.push_index()](PushDevlink::push_index)\n\nReply attributes:\n- [.get_bus_name()](IterableDevlink::get_bus_name)\n- [.get_dev_name()](IterableDevlink::get_dev_name)\n- [.get_dpipe_tables()](IterableDevlink::get_dpipe_tables)\n- [.get_index()](IterableDevlink::get_index)\n\n"]
#[derive(Debug)]
pub struct OpDpipeTableGetDo<'r> {
    request: Request<'r>,
}
impl<'r> OpDpipeTableGetDo<'r> {
    pub fn new(mut request: Request<'r>) -> Self {
        Self::write_header(request.buf_mut());
        Self { request: request }
    }
    pub fn encode_request<'buf>(buf: &'buf mut Vec<u8>) -> PushDevlink<&'buf mut Vec<u8>> {
        Self::write_header(buf);
        PushDevlink::new(buf)
    }
    pub fn encode(&mut self) -> PushDevlink<&mut Vec<u8>> {
        PushDevlink::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushDevlink<RequestBuf<'r>> {
        PushDevlink::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> IterableDevlink<'a> {
        let (_header, attrs) = buf.split_at(buf.len().min(BuiltinNfgenmsg::len()));
        IterableDevlink::with_loc(attrs, buf.as_ptr() as usize)
    }
    fn write_header<Prev: Pusher>(prev: &mut Prev) {
        let mut header = BuiltinNfgenmsg::new();
        header.cmd = 31u8;
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
impl NetlinkRequest for OpDpipeTableGetDo<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Generic("devlink".as_bytes())
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = IterableDevlink<'buf>;
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
#[doc = "Get dpipe entries attributes.\n\nRequest attributes:\n- [.push_bus_name()](PushDevlink::push_bus_name)\n- [.push_dev_name()](PushDevlink::push_dev_name)\n- [.push_dpipe_table_name()](PushDevlink::push_dpipe_table_name)\n- [.push_index()](PushDevlink::push_index)\n\nReply attributes:\n- [.get_bus_name()](IterableDevlink::get_bus_name)\n- [.get_dev_name()](IterableDevlink::get_dev_name)\n- [.get_dpipe_entries()](IterableDevlink::get_dpipe_entries)\n- [.get_index()](IterableDevlink::get_index)\n\n"]
#[derive(Debug)]
pub struct OpDpipeEntriesGetDo<'r> {
    request: Request<'r>,
}
impl<'r> OpDpipeEntriesGetDo<'r> {
    pub fn new(mut request: Request<'r>) -> Self {
        Self::write_header(request.buf_mut());
        Self { request: request }
    }
    pub fn encode_request<'buf>(buf: &'buf mut Vec<u8>) -> PushDevlink<&'buf mut Vec<u8>> {
        Self::write_header(buf);
        PushDevlink::new(buf)
    }
    pub fn encode(&mut self) -> PushDevlink<&mut Vec<u8>> {
        PushDevlink::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushDevlink<RequestBuf<'r>> {
        PushDevlink::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> IterableDevlink<'a> {
        let (_header, attrs) = buf.split_at(buf.len().min(BuiltinNfgenmsg::len()));
        IterableDevlink::with_loc(attrs, buf.as_ptr() as usize)
    }
    fn write_header<Prev: Pusher>(prev: &mut Prev) {
        let mut header = BuiltinNfgenmsg::new();
        header.cmd = 32u8;
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
impl NetlinkRequest for OpDpipeEntriesGetDo<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Generic("devlink".as_bytes())
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = IterableDevlink<'buf>;
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
#[doc = "Get dpipe headers attributes.\n\nRequest attributes:\n- [.push_bus_name()](PushDevlink::push_bus_name)\n- [.push_dev_name()](PushDevlink::push_dev_name)\n- [.push_index()](PushDevlink::push_index)\n\nReply attributes:\n- [.get_bus_name()](IterableDevlink::get_bus_name)\n- [.get_dev_name()](IterableDevlink::get_dev_name)\n- [.get_dpipe_headers()](IterableDevlink::get_dpipe_headers)\n- [.get_index()](IterableDevlink::get_index)\n\n"]
#[derive(Debug)]
pub struct OpDpipeHeadersGetDo<'r> {
    request: Request<'r>,
}
impl<'r> OpDpipeHeadersGetDo<'r> {
    pub fn new(mut request: Request<'r>) -> Self {
        Self::write_header(request.buf_mut());
        Self { request: request }
    }
    pub fn encode_request<'buf>(buf: &'buf mut Vec<u8>) -> PushDevlink<&'buf mut Vec<u8>> {
        Self::write_header(buf);
        PushDevlink::new(buf)
    }
    pub fn encode(&mut self) -> PushDevlink<&mut Vec<u8>> {
        PushDevlink::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushDevlink<RequestBuf<'r>> {
        PushDevlink::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> IterableDevlink<'a> {
        let (_header, attrs) = buf.split_at(buf.len().min(BuiltinNfgenmsg::len()));
        IterableDevlink::with_loc(attrs, buf.as_ptr() as usize)
    }
    fn write_header<Prev: Pusher>(prev: &mut Prev) {
        let mut header = BuiltinNfgenmsg::new();
        header.cmd = 33u8;
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
impl NetlinkRequest for OpDpipeHeadersGetDo<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Generic("devlink".as_bytes())
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = IterableDevlink<'buf>;
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
#[doc = "Set dpipe counter attributes.\n\nFlags: admin-perm\n\nRequest attributes:\n- [.push_bus_name()](PushDevlink::push_bus_name)\n- [.push_dev_name()](PushDevlink::push_dev_name)\n- [.push_dpipe_table_name()](PushDevlink::push_dpipe_table_name)\n- [.push_dpipe_table_counters_enabled()](PushDevlink::push_dpipe_table_counters_enabled)\n- [.push_index()](PushDevlink::push_index)\n\n"]
#[derive(Debug)]
pub struct OpDpipeTableCountersSetDo<'r> {
    request: Request<'r>,
}
impl<'r> OpDpipeTableCountersSetDo<'r> {
    pub fn new(mut request: Request<'r>) -> Self {
        Self::write_header(request.buf_mut());
        Self { request: request }
    }
    pub fn encode_request<'buf>(buf: &'buf mut Vec<u8>) -> PushDevlink<&'buf mut Vec<u8>> {
        Self::write_header(buf);
        PushDevlink::new(buf)
    }
    pub fn encode(&mut self) -> PushDevlink<&mut Vec<u8>> {
        PushDevlink::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushDevlink<RequestBuf<'r>> {
        PushDevlink::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> IterableDevlink<'a> {
        let (_header, attrs) = buf.split_at(buf.len().min(BuiltinNfgenmsg::len()));
        IterableDevlink::with_loc(attrs, buf.as_ptr() as usize)
    }
    fn write_header<Prev: Pusher>(prev: &mut Prev) {
        let mut header = BuiltinNfgenmsg::new();
        header.cmd = 34u8;
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
impl NetlinkRequest for OpDpipeTableCountersSetDo<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Generic("devlink".as_bytes())
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = IterableDevlink<'buf>;
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
#[doc = "Set resource attributes.\n\nFlags: admin-perm\n\nRequest attributes:\n- [.push_bus_name()](PushDevlink::push_bus_name)\n- [.push_dev_name()](PushDevlink::push_dev_name)\n- [.push_resource_id()](PushDevlink::push_resource_id)\n- [.push_resource_size()](PushDevlink::push_resource_size)\n- [.push_index()](PushDevlink::push_index)\n\n"]
#[derive(Debug)]
pub struct OpResourceSetDo<'r> {
    request: Request<'r>,
}
impl<'r> OpResourceSetDo<'r> {
    pub fn new(mut request: Request<'r>) -> Self {
        Self::write_header(request.buf_mut());
        Self { request: request }
    }
    pub fn encode_request<'buf>(buf: &'buf mut Vec<u8>) -> PushDevlink<&'buf mut Vec<u8>> {
        Self::write_header(buf);
        PushDevlink::new(buf)
    }
    pub fn encode(&mut self) -> PushDevlink<&mut Vec<u8>> {
        PushDevlink::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushDevlink<RequestBuf<'r>> {
        PushDevlink::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> IterableDevlink<'a> {
        let (_header, attrs) = buf.split_at(buf.len().min(BuiltinNfgenmsg::len()));
        IterableDevlink::with_loc(attrs, buf.as_ptr() as usize)
    }
    fn write_header<Prev: Pusher>(prev: &mut Prev) {
        let mut header = BuiltinNfgenmsg::new();
        header.cmd = 35u8;
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
impl NetlinkRequest for OpResourceSetDo<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Generic("devlink".as_bytes())
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = IterableDevlink<'buf>;
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
#[doc = "Get resource attributes.\n\nRequest attributes:\n- [.push_bus_name()](PushDevlink::push_bus_name)\n- [.push_dev_name()](PushDevlink::push_dev_name)\n- [.push_index()](PushDevlink::push_index)\n- [.push_resource_scope_mask()](PushDevlink::push_resource_scope_mask)\n\nReply attributes:\n- [.get_bus_name()](IterableDevlink::get_bus_name)\n- [.get_dev_name()](IterableDevlink::get_dev_name)\n- [.get_port_index()](IterableDevlink::get_port_index)\n- [.get_resource_list()](IterableDevlink::get_resource_list)\n- [.get_index()](IterableDevlink::get_index)\n\n"]
#[derive(Debug)]
pub struct OpResourceDumpDump<'r> {
    request: Request<'r>,
}
impl<'r> OpResourceDumpDump<'r> {
    pub fn new(mut request: Request<'r>) -> Self {
        Self::write_header(request.buf_mut());
        Self {
            request: request.set_dump(),
        }
    }
    pub fn encode_request<'buf>(buf: &'buf mut Vec<u8>) -> PushDevlink<&'buf mut Vec<u8>> {
        Self::write_header(buf);
        PushDevlink::new(buf)
    }
    pub fn encode(&mut self) -> PushDevlink<&mut Vec<u8>> {
        PushDevlink::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushDevlink<RequestBuf<'r>> {
        PushDevlink::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> IterableDevlink<'a> {
        let (_header, attrs) = buf.split_at(buf.len().min(BuiltinNfgenmsg::len()));
        IterableDevlink::with_loc(attrs, buf.as_ptr() as usize)
    }
    fn write_header<Prev: Pusher>(prev: &mut Prev) {
        let mut header = BuiltinNfgenmsg::new();
        header.cmd = 36u8;
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
impl NetlinkRequest for OpResourceDumpDump<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Generic("devlink".as_bytes())
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = IterableDevlink<'buf>;
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
#[doc = "Get resource attributes.\n\nRequest attributes:\n- [.push_bus_name()](PushDevlink::push_bus_name)\n- [.push_dev_name()](PushDevlink::push_dev_name)\n- [.push_port_index()](PushDevlink::push_port_index)\n- [.push_index()](PushDevlink::push_index)\n\nReply attributes:\n- [.get_bus_name()](IterableDevlink::get_bus_name)\n- [.get_dev_name()](IterableDevlink::get_dev_name)\n- [.get_port_index()](IterableDevlink::get_port_index)\n- [.get_resource_list()](IterableDevlink::get_resource_list)\n- [.get_index()](IterableDevlink::get_index)\n\n"]
#[derive(Debug)]
pub struct OpResourceDumpDo<'r> {
    request: Request<'r>,
}
impl<'r> OpResourceDumpDo<'r> {
    pub fn new(mut request: Request<'r>) -> Self {
        Self::write_header(request.buf_mut());
        Self { request: request }
    }
    pub fn encode_request<'buf>(buf: &'buf mut Vec<u8>) -> PushDevlink<&'buf mut Vec<u8>> {
        Self::write_header(buf);
        PushDevlink::new(buf)
    }
    pub fn encode(&mut self) -> PushDevlink<&mut Vec<u8>> {
        PushDevlink::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushDevlink<RequestBuf<'r>> {
        PushDevlink::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> IterableDevlink<'a> {
        let (_header, attrs) = buf.split_at(buf.len().min(BuiltinNfgenmsg::len()));
        IterableDevlink::with_loc(attrs, buf.as_ptr() as usize)
    }
    fn write_header<Prev: Pusher>(prev: &mut Prev) {
        let mut header = BuiltinNfgenmsg::new();
        header.cmd = 36u8;
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
impl NetlinkRequest for OpResourceDumpDo<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Generic("devlink".as_bytes())
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = IterableDevlink<'buf>;
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
#[doc = "Reload devlink.\n\nFlags: admin-perm\n\nRequest attributes:\n- [.push_bus_name()](PushDevlink::push_bus_name)\n- [.push_dev_name()](PushDevlink::push_dev_name)\n- [.push_netns_fd()](PushDevlink::push_netns_fd)\n- [.push_netns_pid()](PushDevlink::push_netns_pid)\n- [.push_netns_id()](PushDevlink::push_netns_id)\n- [.push_reload_action()](PushDevlink::push_reload_action)\n- [.push_reload_limits()](PushDevlink::push_reload_limits)\n- [.push_index()](PushDevlink::push_index)\n\nReply attributes:\n- [.get_bus_name()](IterableDevlink::get_bus_name)\n- [.get_dev_name()](IterableDevlink::get_dev_name)\n- [.get_reload_actions_performed()](IterableDevlink::get_reload_actions_performed)\n- [.get_index()](IterableDevlink::get_index)\n\n"]
#[derive(Debug)]
pub struct OpReloadDo<'r> {
    request: Request<'r>,
}
impl<'r> OpReloadDo<'r> {
    pub fn new(mut request: Request<'r>) -> Self {
        Self::write_header(request.buf_mut());
        Self { request: request }
    }
    pub fn encode_request<'buf>(buf: &'buf mut Vec<u8>) -> PushDevlink<&'buf mut Vec<u8>> {
        Self::write_header(buf);
        PushDevlink::new(buf)
    }
    pub fn encode(&mut self) -> PushDevlink<&mut Vec<u8>> {
        PushDevlink::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushDevlink<RequestBuf<'r>> {
        PushDevlink::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> IterableDevlink<'a> {
        let (_header, attrs) = buf.split_at(buf.len().min(BuiltinNfgenmsg::len()));
        IterableDevlink::with_loc(attrs, buf.as_ptr() as usize)
    }
    fn write_header<Prev: Pusher>(prev: &mut Prev) {
        let mut header = BuiltinNfgenmsg::new();
        header.cmd = 37u8;
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
impl NetlinkRequest for OpReloadDo<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Generic("devlink".as_bytes())
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = IterableDevlink<'buf>;
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
#[doc = "Get param instances.\n\nRequest attributes:\n- [.push_bus_name()](PushDevlink::push_bus_name)\n- [.push_dev_name()](PushDevlink::push_dev_name)\n- [.push_index()](PushDevlink::push_index)\n\nReply attributes:\n- [.get_bus_name()](IterableDevlink::get_bus_name)\n- [.get_dev_name()](IterableDevlink::get_dev_name)\n- [.get_param_name()](IterableDevlink::get_param_name)\n- [.get_index()](IterableDevlink::get_index)\n\n"]
#[derive(Debug)]
pub struct OpParamGetDump<'r> {
    request: Request<'r>,
}
impl<'r> OpParamGetDump<'r> {
    pub fn new(mut request: Request<'r>) -> Self {
        Self::write_header(request.buf_mut());
        Self {
            request: request.set_dump(),
        }
    }
    pub fn encode_request<'buf>(buf: &'buf mut Vec<u8>) -> PushDevlink<&'buf mut Vec<u8>> {
        Self::write_header(buf);
        PushDevlink::new(buf)
    }
    pub fn encode(&mut self) -> PushDevlink<&mut Vec<u8>> {
        PushDevlink::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushDevlink<RequestBuf<'r>> {
        PushDevlink::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> IterableDevlink<'a> {
        let (_header, attrs) = buf.split_at(buf.len().min(BuiltinNfgenmsg::len()));
        IterableDevlink::with_loc(attrs, buf.as_ptr() as usize)
    }
    fn write_header<Prev: Pusher>(prev: &mut Prev) {
        let mut header = BuiltinNfgenmsg::new();
        header.cmd = 38u8;
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
impl NetlinkRequest for OpParamGetDump<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Generic("devlink".as_bytes())
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = IterableDevlink<'buf>;
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
#[doc = "Get param instances.\n\nRequest attributes:\n- [.push_bus_name()](PushDevlink::push_bus_name)\n- [.push_dev_name()](PushDevlink::push_dev_name)\n- [.push_param_name()](PushDevlink::push_param_name)\n- [.push_index()](PushDevlink::push_index)\n\nReply attributes:\n- [.get_bus_name()](IterableDevlink::get_bus_name)\n- [.get_dev_name()](IterableDevlink::get_dev_name)\n- [.get_param_name()](IterableDevlink::get_param_name)\n- [.get_index()](IterableDevlink::get_index)\n\n"]
#[derive(Debug)]
pub struct OpParamGetDo<'r> {
    request: Request<'r>,
}
impl<'r> OpParamGetDo<'r> {
    pub fn new(mut request: Request<'r>) -> Self {
        Self::write_header(request.buf_mut());
        Self { request: request }
    }
    pub fn encode_request<'buf>(buf: &'buf mut Vec<u8>) -> PushDevlink<&'buf mut Vec<u8>> {
        Self::write_header(buf);
        PushDevlink::new(buf)
    }
    pub fn encode(&mut self) -> PushDevlink<&mut Vec<u8>> {
        PushDevlink::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushDevlink<RequestBuf<'r>> {
        PushDevlink::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> IterableDevlink<'a> {
        let (_header, attrs) = buf.split_at(buf.len().min(BuiltinNfgenmsg::len()));
        IterableDevlink::with_loc(attrs, buf.as_ptr() as usize)
    }
    fn write_header<Prev: Pusher>(prev: &mut Prev) {
        let mut header = BuiltinNfgenmsg::new();
        header.cmd = 38u8;
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
impl NetlinkRequest for OpParamGetDo<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Generic("devlink".as_bytes())
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = IterableDevlink<'buf>;
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
#[doc = "Set param instances.\n\nFlags: admin-perm\n\nRequest attributes:\n- [.push_bus_name()](PushDevlink::push_bus_name)\n- [.push_dev_name()](PushDevlink::push_dev_name)\n- [.push_param_name()](PushDevlink::push_param_name)\n- [.push_param_type()](PushDevlink::push_param_type)\n- [.push_param_value_cmode()](PushDevlink::push_param_value_cmode)\n- [.push_param_reset_default()](PushDevlink::push_param_reset_default)\n- [.push_index()](PushDevlink::push_index)\n\n"]
#[derive(Debug)]
pub struct OpParamSetDo<'r> {
    request: Request<'r>,
}
impl<'r> OpParamSetDo<'r> {
    pub fn new(mut request: Request<'r>) -> Self {
        Self::write_header(request.buf_mut());
        Self { request: request }
    }
    pub fn encode_request<'buf>(buf: &'buf mut Vec<u8>) -> PushDevlink<&'buf mut Vec<u8>> {
        Self::write_header(buf);
        PushDevlink::new(buf)
    }
    pub fn encode(&mut self) -> PushDevlink<&mut Vec<u8>> {
        PushDevlink::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushDevlink<RequestBuf<'r>> {
        PushDevlink::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> IterableDevlink<'a> {
        let (_header, attrs) = buf.split_at(buf.len().min(BuiltinNfgenmsg::len()));
        IterableDevlink::with_loc(attrs, buf.as_ptr() as usize)
    }
    fn write_header<Prev: Pusher>(prev: &mut Prev) {
        let mut header = BuiltinNfgenmsg::new();
        header.cmd = 39u8;
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
impl NetlinkRequest for OpParamSetDo<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Generic("devlink".as_bytes())
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = IterableDevlink<'buf>;
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
#[doc = "Get region instances.\n\nRequest attributes:\n- [.push_bus_name()](PushDevlink::push_bus_name)\n- [.push_dev_name()](PushDevlink::push_dev_name)\n- [.push_index()](PushDevlink::push_index)\n\nReply attributes:\n- [.get_bus_name()](IterableDevlink::get_bus_name)\n- [.get_dev_name()](IterableDevlink::get_dev_name)\n- [.get_port_index()](IterableDevlink::get_port_index)\n- [.get_region_name()](IterableDevlink::get_region_name)\n- [.get_index()](IterableDevlink::get_index)\n\n"]
#[derive(Debug)]
pub struct OpRegionGetDump<'r> {
    request: Request<'r>,
}
impl<'r> OpRegionGetDump<'r> {
    pub fn new(mut request: Request<'r>) -> Self {
        Self::write_header(request.buf_mut());
        Self {
            request: request.set_dump(),
        }
    }
    pub fn encode_request<'buf>(buf: &'buf mut Vec<u8>) -> PushDevlink<&'buf mut Vec<u8>> {
        Self::write_header(buf);
        PushDevlink::new(buf)
    }
    pub fn encode(&mut self) -> PushDevlink<&mut Vec<u8>> {
        PushDevlink::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushDevlink<RequestBuf<'r>> {
        PushDevlink::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> IterableDevlink<'a> {
        let (_header, attrs) = buf.split_at(buf.len().min(BuiltinNfgenmsg::len()));
        IterableDevlink::with_loc(attrs, buf.as_ptr() as usize)
    }
    fn write_header<Prev: Pusher>(prev: &mut Prev) {
        let mut header = BuiltinNfgenmsg::new();
        header.cmd = 42u8;
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
impl NetlinkRequest for OpRegionGetDump<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Generic("devlink".as_bytes())
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = IterableDevlink<'buf>;
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
#[doc = "Get region instances.\n\nRequest attributes:\n- [.push_bus_name()](PushDevlink::push_bus_name)\n- [.push_dev_name()](PushDevlink::push_dev_name)\n- [.push_port_index()](PushDevlink::push_port_index)\n- [.push_region_name()](PushDevlink::push_region_name)\n- [.push_index()](PushDevlink::push_index)\n\nReply attributes:\n- [.get_bus_name()](IterableDevlink::get_bus_name)\n- [.get_dev_name()](IterableDevlink::get_dev_name)\n- [.get_port_index()](IterableDevlink::get_port_index)\n- [.get_region_name()](IterableDevlink::get_region_name)\n- [.get_index()](IterableDevlink::get_index)\n\n"]
#[derive(Debug)]
pub struct OpRegionGetDo<'r> {
    request: Request<'r>,
}
impl<'r> OpRegionGetDo<'r> {
    pub fn new(mut request: Request<'r>) -> Self {
        Self::write_header(request.buf_mut());
        Self { request: request }
    }
    pub fn encode_request<'buf>(buf: &'buf mut Vec<u8>) -> PushDevlink<&'buf mut Vec<u8>> {
        Self::write_header(buf);
        PushDevlink::new(buf)
    }
    pub fn encode(&mut self) -> PushDevlink<&mut Vec<u8>> {
        PushDevlink::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushDevlink<RequestBuf<'r>> {
        PushDevlink::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> IterableDevlink<'a> {
        let (_header, attrs) = buf.split_at(buf.len().min(BuiltinNfgenmsg::len()));
        IterableDevlink::with_loc(attrs, buf.as_ptr() as usize)
    }
    fn write_header<Prev: Pusher>(prev: &mut Prev) {
        let mut header = BuiltinNfgenmsg::new();
        header.cmd = 42u8;
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
impl NetlinkRequest for OpRegionGetDo<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Generic("devlink".as_bytes())
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = IterableDevlink<'buf>;
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
#[doc = "Create region snapshot.\n\nFlags: admin-perm\n\nRequest attributes:\n- [.push_bus_name()](PushDevlink::push_bus_name)\n- [.push_dev_name()](PushDevlink::push_dev_name)\n- [.push_port_index()](PushDevlink::push_port_index)\n- [.push_region_name()](PushDevlink::push_region_name)\n- [.push_region_snapshot_id()](PushDevlink::push_region_snapshot_id)\n- [.push_index()](PushDevlink::push_index)\n\nReply attributes:\n- [.get_bus_name()](IterableDevlink::get_bus_name)\n- [.get_dev_name()](IterableDevlink::get_dev_name)\n- [.get_port_index()](IterableDevlink::get_port_index)\n- [.get_region_name()](IterableDevlink::get_region_name)\n- [.get_region_snapshot_id()](IterableDevlink::get_region_snapshot_id)\n- [.get_index()](IterableDevlink::get_index)\n\n"]
#[derive(Debug)]
pub struct OpRegionNewDo<'r> {
    request: Request<'r>,
}
impl<'r> OpRegionNewDo<'r> {
    pub fn new(mut request: Request<'r>) -> Self {
        Self::write_header(request.buf_mut());
        Self { request: request }
    }
    pub fn encode_request<'buf>(buf: &'buf mut Vec<u8>) -> PushDevlink<&'buf mut Vec<u8>> {
        Self::write_header(buf);
        PushDevlink::new(buf)
    }
    pub fn encode(&mut self) -> PushDevlink<&mut Vec<u8>> {
        PushDevlink::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushDevlink<RequestBuf<'r>> {
        PushDevlink::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> IterableDevlink<'a> {
        let (_header, attrs) = buf.split_at(buf.len().min(BuiltinNfgenmsg::len()));
        IterableDevlink::with_loc(attrs, buf.as_ptr() as usize)
    }
    fn write_header<Prev: Pusher>(prev: &mut Prev) {
        let mut header = BuiltinNfgenmsg::new();
        header.cmd = 44u8;
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
impl NetlinkRequest for OpRegionNewDo<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Generic("devlink".as_bytes())
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = IterableDevlink<'buf>;
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
#[doc = "Delete region snapshot.\n\nFlags: admin-perm\n\nRequest attributes:\n- [.push_bus_name()](PushDevlink::push_bus_name)\n- [.push_dev_name()](PushDevlink::push_dev_name)\n- [.push_port_index()](PushDevlink::push_port_index)\n- [.push_region_name()](PushDevlink::push_region_name)\n- [.push_region_snapshot_id()](PushDevlink::push_region_snapshot_id)\n- [.push_index()](PushDevlink::push_index)\n\n"]
#[derive(Debug)]
pub struct OpRegionDelDo<'r> {
    request: Request<'r>,
}
impl<'r> OpRegionDelDo<'r> {
    pub fn new(mut request: Request<'r>) -> Self {
        Self::write_header(request.buf_mut());
        Self { request: request }
    }
    pub fn encode_request<'buf>(buf: &'buf mut Vec<u8>) -> PushDevlink<&'buf mut Vec<u8>> {
        Self::write_header(buf);
        PushDevlink::new(buf)
    }
    pub fn encode(&mut self) -> PushDevlink<&mut Vec<u8>> {
        PushDevlink::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushDevlink<RequestBuf<'r>> {
        PushDevlink::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> IterableDevlink<'a> {
        let (_header, attrs) = buf.split_at(buf.len().min(BuiltinNfgenmsg::len()));
        IterableDevlink::with_loc(attrs, buf.as_ptr() as usize)
    }
    fn write_header<Prev: Pusher>(prev: &mut Prev) {
        let mut header = BuiltinNfgenmsg::new();
        header.cmd = 45u8;
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
impl NetlinkRequest for OpRegionDelDo<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Generic("devlink".as_bytes())
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = IterableDevlink<'buf>;
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
#[doc = "Read region data.\n\nFlags: admin-perm\n\nRequest attributes:\n- [.push_bus_name()](PushDevlink::push_bus_name)\n- [.push_dev_name()](PushDevlink::push_dev_name)\n- [.push_port_index()](PushDevlink::push_port_index)\n- [.push_region_name()](PushDevlink::push_region_name)\n- [.push_region_snapshot_id()](PushDevlink::push_region_snapshot_id)\n- [.push_region_chunk_addr()](PushDevlink::push_region_chunk_addr)\n- [.push_region_chunk_len()](PushDevlink::push_region_chunk_len)\n- [.push_region_direct()](PushDevlink::push_region_direct)\n- [.push_index()](PushDevlink::push_index)\n\nReply attributes:\n- [.get_bus_name()](IterableDevlink::get_bus_name)\n- [.get_dev_name()](IterableDevlink::get_dev_name)\n- [.get_port_index()](IterableDevlink::get_port_index)\n- [.get_region_name()](IterableDevlink::get_region_name)\n- [.get_index()](IterableDevlink::get_index)\n\n"]
#[derive(Debug)]
pub struct OpRegionReadDump<'r> {
    request: Request<'r>,
}
impl<'r> OpRegionReadDump<'r> {
    pub fn new(mut request: Request<'r>) -> Self {
        Self::write_header(request.buf_mut());
        Self {
            request: request.set_dump(),
        }
    }
    pub fn encode_request<'buf>(buf: &'buf mut Vec<u8>) -> PushDevlink<&'buf mut Vec<u8>> {
        Self::write_header(buf);
        PushDevlink::new(buf)
    }
    pub fn encode(&mut self) -> PushDevlink<&mut Vec<u8>> {
        PushDevlink::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushDevlink<RequestBuf<'r>> {
        PushDevlink::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> IterableDevlink<'a> {
        let (_header, attrs) = buf.split_at(buf.len().min(BuiltinNfgenmsg::len()));
        IterableDevlink::with_loc(attrs, buf.as_ptr() as usize)
    }
    fn write_header<Prev: Pusher>(prev: &mut Prev) {
        let mut header = BuiltinNfgenmsg::new();
        header.cmd = 46u8;
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
impl NetlinkRequest for OpRegionReadDump<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Generic("devlink".as_bytes())
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = IterableDevlink<'buf>;
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
#[doc = "Get port param instances.\n\nReply attributes:\n- [.get_bus_name()](IterableDevlink::get_bus_name)\n- [.get_dev_name()](IterableDevlink::get_dev_name)\n- [.get_port_index()](IterableDevlink::get_port_index)\n- [.get_index()](IterableDevlink::get_index)\n\n"]
#[derive(Debug)]
pub struct OpPortParamGetDump<'r> {
    request: Request<'r>,
}
impl<'r> OpPortParamGetDump<'r> {
    pub fn new(mut request: Request<'r>) -> Self {
        Self::write_header(request.buf_mut());
        Self {
            request: request.set_dump(),
        }
    }
    pub fn encode_request<'buf>(buf: &'buf mut Vec<u8>) -> PushDevlink<&'buf mut Vec<u8>> {
        Self::write_header(buf);
        PushDevlink::new(buf)
    }
    pub fn encode(&mut self) -> PushDevlink<&mut Vec<u8>> {
        PushDevlink::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushDevlink<RequestBuf<'r>> {
        PushDevlink::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> IterableDevlink<'a> {
        let (_header, attrs) = buf.split_at(buf.len().min(BuiltinNfgenmsg::len()));
        IterableDevlink::with_loc(attrs, buf.as_ptr() as usize)
    }
    fn write_header<Prev: Pusher>(prev: &mut Prev) {
        let mut header = BuiltinNfgenmsg::new();
        header.cmd = 47u8;
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
impl NetlinkRequest for OpPortParamGetDump<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Generic("devlink".as_bytes())
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = IterableDevlink<'buf>;
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
#[doc = "Get port param instances.\n\nRequest attributes:\n- [.push_bus_name()](PushDevlink::push_bus_name)\n- [.push_dev_name()](PushDevlink::push_dev_name)\n- [.push_port_index()](PushDevlink::push_port_index)\n- [.push_index()](PushDevlink::push_index)\n\nReply attributes:\n- [.get_bus_name()](IterableDevlink::get_bus_name)\n- [.get_dev_name()](IterableDevlink::get_dev_name)\n- [.get_port_index()](IterableDevlink::get_port_index)\n- [.get_index()](IterableDevlink::get_index)\n\n"]
#[derive(Debug)]
pub struct OpPortParamGetDo<'r> {
    request: Request<'r>,
}
impl<'r> OpPortParamGetDo<'r> {
    pub fn new(mut request: Request<'r>) -> Self {
        Self::write_header(request.buf_mut());
        Self { request: request }
    }
    pub fn encode_request<'buf>(buf: &'buf mut Vec<u8>) -> PushDevlink<&'buf mut Vec<u8>> {
        Self::write_header(buf);
        PushDevlink::new(buf)
    }
    pub fn encode(&mut self) -> PushDevlink<&mut Vec<u8>> {
        PushDevlink::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushDevlink<RequestBuf<'r>> {
        PushDevlink::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> IterableDevlink<'a> {
        let (_header, attrs) = buf.split_at(buf.len().min(BuiltinNfgenmsg::len()));
        IterableDevlink::with_loc(attrs, buf.as_ptr() as usize)
    }
    fn write_header<Prev: Pusher>(prev: &mut Prev) {
        let mut header = BuiltinNfgenmsg::new();
        header.cmd = 47u8;
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
impl NetlinkRequest for OpPortParamGetDo<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Generic("devlink".as_bytes())
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = IterableDevlink<'buf>;
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
#[doc = "Set port param instances.\n\nFlags: admin-perm\n\nRequest attributes:\n- [.push_bus_name()](PushDevlink::push_bus_name)\n- [.push_dev_name()](PushDevlink::push_dev_name)\n- [.push_port_index()](PushDevlink::push_port_index)\n- [.push_index()](PushDevlink::push_index)\n\n"]
#[derive(Debug)]
pub struct OpPortParamSetDo<'r> {
    request: Request<'r>,
}
impl<'r> OpPortParamSetDo<'r> {
    pub fn new(mut request: Request<'r>) -> Self {
        Self::write_header(request.buf_mut());
        Self { request: request }
    }
    pub fn encode_request<'buf>(buf: &'buf mut Vec<u8>) -> PushDevlink<&'buf mut Vec<u8>> {
        Self::write_header(buf);
        PushDevlink::new(buf)
    }
    pub fn encode(&mut self) -> PushDevlink<&mut Vec<u8>> {
        PushDevlink::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushDevlink<RequestBuf<'r>> {
        PushDevlink::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> IterableDevlink<'a> {
        let (_header, attrs) = buf.split_at(buf.len().min(BuiltinNfgenmsg::len()));
        IterableDevlink::with_loc(attrs, buf.as_ptr() as usize)
    }
    fn write_header<Prev: Pusher>(prev: &mut Prev) {
        let mut header = BuiltinNfgenmsg::new();
        header.cmd = 48u8;
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
impl NetlinkRequest for OpPortParamSetDo<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Generic("devlink".as_bytes())
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = IterableDevlink<'buf>;
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
#[doc = "Get device information, like driver name, hardware and firmware versions\netc.\n\nReply attributes:\n- [.get_bus_name()](IterableDevlink::get_bus_name)\n- [.get_dev_name()](IterableDevlink::get_dev_name)\n- [.get_info_driver_name()](IterableDevlink::get_info_driver_name)\n- [.get_info_serial_number()](IterableDevlink::get_info_serial_number)\n- [.get_info_version_fixed()](IterableDevlink::get_info_version_fixed)\n- [.get_info_version_running()](IterableDevlink::get_info_version_running)\n- [.get_info_version_stored()](IterableDevlink::get_info_version_stored)\n- [.get_info_board_serial_number()](IterableDevlink::get_info_board_serial_number)\n- [.get_index()](IterableDevlink::get_index)\n\n"]
#[derive(Debug)]
pub struct OpInfoGetDump<'r> {
    request: Request<'r>,
}
impl<'r> OpInfoGetDump<'r> {
    pub fn new(mut request: Request<'r>) -> Self {
        Self::write_header(request.buf_mut());
        Self {
            request: request.set_dump(),
        }
    }
    pub fn encode_request<'buf>(buf: &'buf mut Vec<u8>) -> PushDevlink<&'buf mut Vec<u8>> {
        Self::write_header(buf);
        PushDevlink::new(buf)
    }
    pub fn encode(&mut self) -> PushDevlink<&mut Vec<u8>> {
        PushDevlink::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushDevlink<RequestBuf<'r>> {
        PushDevlink::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> IterableDevlink<'a> {
        let (_header, attrs) = buf.split_at(buf.len().min(BuiltinNfgenmsg::len()));
        IterableDevlink::with_loc(attrs, buf.as_ptr() as usize)
    }
    fn write_header<Prev: Pusher>(prev: &mut Prev) {
        let mut header = BuiltinNfgenmsg::new();
        header.cmd = 51u8;
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
impl NetlinkRequest for OpInfoGetDump<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Generic("devlink".as_bytes())
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = IterableDevlink<'buf>;
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
#[doc = "Get device information, like driver name, hardware and firmware versions\netc.\n\nRequest attributes:\n- [.push_bus_name()](PushDevlink::push_bus_name)\n- [.push_dev_name()](PushDevlink::push_dev_name)\n- [.push_index()](PushDevlink::push_index)\n\nReply attributes:\n- [.get_bus_name()](IterableDevlink::get_bus_name)\n- [.get_dev_name()](IterableDevlink::get_dev_name)\n- [.get_info_driver_name()](IterableDevlink::get_info_driver_name)\n- [.get_info_serial_number()](IterableDevlink::get_info_serial_number)\n- [.get_info_version_fixed()](IterableDevlink::get_info_version_fixed)\n- [.get_info_version_running()](IterableDevlink::get_info_version_running)\n- [.get_info_version_stored()](IterableDevlink::get_info_version_stored)\n- [.get_info_board_serial_number()](IterableDevlink::get_info_board_serial_number)\n- [.get_index()](IterableDevlink::get_index)\n\n"]
#[derive(Debug)]
pub struct OpInfoGetDo<'r> {
    request: Request<'r>,
}
impl<'r> OpInfoGetDo<'r> {
    pub fn new(mut request: Request<'r>) -> Self {
        Self::write_header(request.buf_mut());
        Self { request: request }
    }
    pub fn encode_request<'buf>(buf: &'buf mut Vec<u8>) -> PushDevlink<&'buf mut Vec<u8>> {
        Self::write_header(buf);
        PushDevlink::new(buf)
    }
    pub fn encode(&mut self) -> PushDevlink<&mut Vec<u8>> {
        PushDevlink::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushDevlink<RequestBuf<'r>> {
        PushDevlink::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> IterableDevlink<'a> {
        let (_header, attrs) = buf.split_at(buf.len().min(BuiltinNfgenmsg::len()));
        IterableDevlink::with_loc(attrs, buf.as_ptr() as usize)
    }
    fn write_header<Prev: Pusher>(prev: &mut Prev) {
        let mut header = BuiltinNfgenmsg::new();
        header.cmd = 51u8;
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
impl NetlinkRequest for OpInfoGetDo<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Generic("devlink".as_bytes())
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = IterableDevlink<'buf>;
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
#[doc = "Get health reporter instances.\n\nRequest attributes:\n- [.push_bus_name()](PushDevlink::push_bus_name)\n- [.push_dev_name()](PushDevlink::push_dev_name)\n- [.push_port_index()](PushDevlink::push_port_index)\n- [.push_index()](PushDevlink::push_index)\n\nReply attributes:\n- [.get_bus_name()](IterableDevlink::get_bus_name)\n- [.get_dev_name()](IterableDevlink::get_dev_name)\n- [.get_port_index()](IterableDevlink::get_port_index)\n- [.get_health_reporter_name()](IterableDevlink::get_health_reporter_name)\n- [.get_index()](IterableDevlink::get_index)\n\n"]
#[derive(Debug)]
pub struct OpHealthReporterGetDump<'r> {
    request: Request<'r>,
}
impl<'r> OpHealthReporterGetDump<'r> {
    pub fn new(mut request: Request<'r>) -> Self {
        Self::write_header(request.buf_mut());
        Self {
            request: request.set_dump(),
        }
    }
    pub fn encode_request<'buf>(buf: &'buf mut Vec<u8>) -> PushDevlink<&'buf mut Vec<u8>> {
        Self::write_header(buf);
        PushDevlink::new(buf)
    }
    pub fn encode(&mut self) -> PushDevlink<&mut Vec<u8>> {
        PushDevlink::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushDevlink<RequestBuf<'r>> {
        PushDevlink::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> IterableDevlink<'a> {
        let (_header, attrs) = buf.split_at(buf.len().min(BuiltinNfgenmsg::len()));
        IterableDevlink::with_loc(attrs, buf.as_ptr() as usize)
    }
    fn write_header<Prev: Pusher>(prev: &mut Prev) {
        let mut header = BuiltinNfgenmsg::new();
        header.cmd = 52u8;
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
impl NetlinkRequest for OpHealthReporterGetDump<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Generic("devlink".as_bytes())
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = IterableDevlink<'buf>;
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
#[doc = "Get health reporter instances.\n\nRequest attributes:\n- [.push_bus_name()](PushDevlink::push_bus_name)\n- [.push_dev_name()](PushDevlink::push_dev_name)\n- [.push_port_index()](PushDevlink::push_port_index)\n- [.push_health_reporter_name()](PushDevlink::push_health_reporter_name)\n- [.push_index()](PushDevlink::push_index)\n\nReply attributes:\n- [.get_bus_name()](IterableDevlink::get_bus_name)\n- [.get_dev_name()](IterableDevlink::get_dev_name)\n- [.get_port_index()](IterableDevlink::get_port_index)\n- [.get_health_reporter_name()](IterableDevlink::get_health_reporter_name)\n- [.get_index()](IterableDevlink::get_index)\n\n"]
#[derive(Debug)]
pub struct OpHealthReporterGetDo<'r> {
    request: Request<'r>,
}
impl<'r> OpHealthReporterGetDo<'r> {
    pub fn new(mut request: Request<'r>) -> Self {
        Self::write_header(request.buf_mut());
        Self { request: request }
    }
    pub fn encode_request<'buf>(buf: &'buf mut Vec<u8>) -> PushDevlink<&'buf mut Vec<u8>> {
        Self::write_header(buf);
        PushDevlink::new(buf)
    }
    pub fn encode(&mut self) -> PushDevlink<&mut Vec<u8>> {
        PushDevlink::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushDevlink<RequestBuf<'r>> {
        PushDevlink::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> IterableDevlink<'a> {
        let (_header, attrs) = buf.split_at(buf.len().min(BuiltinNfgenmsg::len()));
        IterableDevlink::with_loc(attrs, buf.as_ptr() as usize)
    }
    fn write_header<Prev: Pusher>(prev: &mut Prev) {
        let mut header = BuiltinNfgenmsg::new();
        header.cmd = 52u8;
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
impl NetlinkRequest for OpHealthReporterGetDo<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Generic("devlink".as_bytes())
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = IterableDevlink<'buf>;
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
#[doc = "Set health reporter instances.\n\nFlags: admin-perm\n\nRequest attributes:\n- [.push_bus_name()](PushDevlink::push_bus_name)\n- [.push_dev_name()](PushDevlink::push_dev_name)\n- [.push_port_index()](PushDevlink::push_port_index)\n- [.push_health_reporter_name()](PushDevlink::push_health_reporter_name)\n- [.push_health_reporter_graceful_period()](PushDevlink::push_health_reporter_graceful_period)\n- [.push_health_reporter_auto_recover()](PushDevlink::push_health_reporter_auto_recover)\n- [.push_health_reporter_auto_dump()](PushDevlink::push_health_reporter_auto_dump)\n- [.push_health_reporter_burst_period()](PushDevlink::push_health_reporter_burst_period)\n- [.push_index()](PushDevlink::push_index)\n\n"]
#[derive(Debug)]
pub struct OpHealthReporterSetDo<'r> {
    request: Request<'r>,
}
impl<'r> OpHealthReporterSetDo<'r> {
    pub fn new(mut request: Request<'r>) -> Self {
        Self::write_header(request.buf_mut());
        Self { request: request }
    }
    pub fn encode_request<'buf>(buf: &'buf mut Vec<u8>) -> PushDevlink<&'buf mut Vec<u8>> {
        Self::write_header(buf);
        PushDevlink::new(buf)
    }
    pub fn encode(&mut self) -> PushDevlink<&mut Vec<u8>> {
        PushDevlink::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushDevlink<RequestBuf<'r>> {
        PushDevlink::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> IterableDevlink<'a> {
        let (_header, attrs) = buf.split_at(buf.len().min(BuiltinNfgenmsg::len()));
        IterableDevlink::with_loc(attrs, buf.as_ptr() as usize)
    }
    fn write_header<Prev: Pusher>(prev: &mut Prev) {
        let mut header = BuiltinNfgenmsg::new();
        header.cmd = 53u8;
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
impl NetlinkRequest for OpHealthReporterSetDo<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Generic("devlink".as_bytes())
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = IterableDevlink<'buf>;
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
#[doc = "Recover health reporter instances.\n\nFlags: admin-perm\n\nRequest attributes:\n- [.push_bus_name()](PushDevlink::push_bus_name)\n- [.push_dev_name()](PushDevlink::push_dev_name)\n- [.push_port_index()](PushDevlink::push_port_index)\n- [.push_health_reporter_name()](PushDevlink::push_health_reporter_name)\n- [.push_index()](PushDevlink::push_index)\n\n"]
#[derive(Debug)]
pub struct OpHealthReporterRecoverDo<'r> {
    request: Request<'r>,
}
impl<'r> OpHealthReporterRecoverDo<'r> {
    pub fn new(mut request: Request<'r>) -> Self {
        Self::write_header(request.buf_mut());
        Self { request: request }
    }
    pub fn encode_request<'buf>(buf: &'buf mut Vec<u8>) -> PushDevlink<&'buf mut Vec<u8>> {
        Self::write_header(buf);
        PushDevlink::new(buf)
    }
    pub fn encode(&mut self) -> PushDevlink<&mut Vec<u8>> {
        PushDevlink::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushDevlink<RequestBuf<'r>> {
        PushDevlink::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> IterableDevlink<'a> {
        let (_header, attrs) = buf.split_at(buf.len().min(BuiltinNfgenmsg::len()));
        IterableDevlink::with_loc(attrs, buf.as_ptr() as usize)
    }
    fn write_header<Prev: Pusher>(prev: &mut Prev) {
        let mut header = BuiltinNfgenmsg::new();
        header.cmd = 54u8;
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
impl NetlinkRequest for OpHealthReporterRecoverDo<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Generic("devlink".as_bytes())
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = IterableDevlink<'buf>;
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
#[doc = "Diagnose health reporter instances.\n\nFlags: admin-perm\n\nRequest attributes:\n- [.push_bus_name()](PushDevlink::push_bus_name)\n- [.push_dev_name()](PushDevlink::push_dev_name)\n- [.push_port_index()](PushDevlink::push_port_index)\n- [.push_health_reporter_name()](PushDevlink::push_health_reporter_name)\n- [.push_index()](PushDevlink::push_index)\n\n"]
#[derive(Debug)]
pub struct OpHealthReporterDiagnoseDo<'r> {
    request: Request<'r>,
}
impl<'r> OpHealthReporterDiagnoseDo<'r> {
    pub fn new(mut request: Request<'r>) -> Self {
        Self::write_header(request.buf_mut());
        Self { request: request }
    }
    pub fn encode_request<'buf>(buf: &'buf mut Vec<u8>) -> PushDevlink<&'buf mut Vec<u8>> {
        Self::write_header(buf);
        PushDevlink::new(buf)
    }
    pub fn encode(&mut self) -> PushDevlink<&mut Vec<u8>> {
        PushDevlink::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushDevlink<RequestBuf<'r>> {
        PushDevlink::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> IterableDevlink<'a> {
        let (_header, attrs) = buf.split_at(buf.len().min(BuiltinNfgenmsg::len()));
        IterableDevlink::with_loc(attrs, buf.as_ptr() as usize)
    }
    fn write_header<Prev: Pusher>(prev: &mut Prev) {
        let mut header = BuiltinNfgenmsg::new();
        header.cmd = 55u8;
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
impl NetlinkRequest for OpHealthReporterDiagnoseDo<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Generic("devlink".as_bytes())
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = IterableDevlink<'buf>;
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
#[doc = "Dump health reporter instances.\n\nFlags: admin-perm\n\nRequest attributes:\n- [.push_bus_name()](PushDevlink::push_bus_name)\n- [.push_dev_name()](PushDevlink::push_dev_name)\n- [.push_port_index()](PushDevlink::push_port_index)\n- [.push_health_reporter_name()](PushDevlink::push_health_reporter_name)\n- [.push_index()](PushDevlink::push_index)\n\nReply attributes:\n- [.get_fmsg()](IterableDevlink::get_fmsg)\n\n"]
#[derive(Debug)]
pub struct OpHealthReporterDumpGetDump<'r> {
    request: Request<'r>,
}
impl<'r> OpHealthReporterDumpGetDump<'r> {
    pub fn new(mut request: Request<'r>) -> Self {
        Self::write_header(request.buf_mut());
        Self {
            request: request.set_dump(),
        }
    }
    pub fn encode_request<'buf>(buf: &'buf mut Vec<u8>) -> PushDevlink<&'buf mut Vec<u8>> {
        Self::write_header(buf);
        PushDevlink::new(buf)
    }
    pub fn encode(&mut self) -> PushDevlink<&mut Vec<u8>> {
        PushDevlink::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushDevlink<RequestBuf<'r>> {
        PushDevlink::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> IterableDevlink<'a> {
        let (_header, attrs) = buf.split_at(buf.len().min(BuiltinNfgenmsg::len()));
        IterableDevlink::with_loc(attrs, buf.as_ptr() as usize)
    }
    fn write_header<Prev: Pusher>(prev: &mut Prev) {
        let mut header = BuiltinNfgenmsg::new();
        header.cmd = 56u8;
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
impl NetlinkRequest for OpHealthReporterDumpGetDump<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Generic("devlink".as_bytes())
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = IterableDevlink<'buf>;
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
#[doc = "Clear dump of health reporter instances.\n\nFlags: admin-perm\n\nRequest attributes:\n- [.push_bus_name()](PushDevlink::push_bus_name)\n- [.push_dev_name()](PushDevlink::push_dev_name)\n- [.push_port_index()](PushDevlink::push_port_index)\n- [.push_health_reporter_name()](PushDevlink::push_health_reporter_name)\n- [.push_index()](PushDevlink::push_index)\n\n"]
#[derive(Debug)]
pub struct OpHealthReporterDumpClearDo<'r> {
    request: Request<'r>,
}
impl<'r> OpHealthReporterDumpClearDo<'r> {
    pub fn new(mut request: Request<'r>) -> Self {
        Self::write_header(request.buf_mut());
        Self { request: request }
    }
    pub fn encode_request<'buf>(buf: &'buf mut Vec<u8>) -> PushDevlink<&'buf mut Vec<u8>> {
        Self::write_header(buf);
        PushDevlink::new(buf)
    }
    pub fn encode(&mut self) -> PushDevlink<&mut Vec<u8>> {
        PushDevlink::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushDevlink<RequestBuf<'r>> {
        PushDevlink::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> IterableDevlink<'a> {
        let (_header, attrs) = buf.split_at(buf.len().min(BuiltinNfgenmsg::len()));
        IterableDevlink::with_loc(attrs, buf.as_ptr() as usize)
    }
    fn write_header<Prev: Pusher>(prev: &mut Prev) {
        let mut header = BuiltinNfgenmsg::new();
        header.cmd = 57u8;
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
impl NetlinkRequest for OpHealthReporterDumpClearDo<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Generic("devlink".as_bytes())
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = IterableDevlink<'buf>;
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
#[doc = "Flash update devlink instances.\n\nFlags: admin-perm\n\nRequest attributes:\n- [.push_bus_name()](PushDevlink::push_bus_name)\n- [.push_dev_name()](PushDevlink::push_dev_name)\n- [.push_flash_update_file_name()](PushDevlink::push_flash_update_file_name)\n- [.push_flash_update_component()](PushDevlink::push_flash_update_component)\n- [.push_flash_update_overwrite_mask()](PushDevlink::push_flash_update_overwrite_mask)\n- [.push_index()](PushDevlink::push_index)\n\n"]
#[derive(Debug)]
pub struct OpFlashUpdateDo<'r> {
    request: Request<'r>,
}
impl<'r> OpFlashUpdateDo<'r> {
    pub fn new(mut request: Request<'r>) -> Self {
        Self::write_header(request.buf_mut());
        Self { request: request }
    }
    pub fn encode_request<'buf>(buf: &'buf mut Vec<u8>) -> PushDevlink<&'buf mut Vec<u8>> {
        Self::write_header(buf);
        PushDevlink::new(buf)
    }
    pub fn encode(&mut self) -> PushDevlink<&mut Vec<u8>> {
        PushDevlink::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushDevlink<RequestBuf<'r>> {
        PushDevlink::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> IterableDevlink<'a> {
        let (_header, attrs) = buf.split_at(buf.len().min(BuiltinNfgenmsg::len()));
        IterableDevlink::with_loc(attrs, buf.as_ptr() as usize)
    }
    fn write_header<Prev: Pusher>(prev: &mut Prev) {
        let mut header = BuiltinNfgenmsg::new();
        header.cmd = 58u8;
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
impl NetlinkRequest for OpFlashUpdateDo<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Generic("devlink".as_bytes())
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = IterableDevlink<'buf>;
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
#[doc = "Get trap instances.\n\nRequest attributes:\n- [.push_bus_name()](PushDevlink::push_bus_name)\n- [.push_dev_name()](PushDevlink::push_dev_name)\n- [.push_index()](PushDevlink::push_index)\n\nReply attributes:\n- [.get_bus_name()](IterableDevlink::get_bus_name)\n- [.get_dev_name()](IterableDevlink::get_dev_name)\n- [.get_trap_name()](IterableDevlink::get_trap_name)\n- [.get_index()](IterableDevlink::get_index)\n\n"]
#[derive(Debug)]
pub struct OpTrapGetDump<'r> {
    request: Request<'r>,
}
impl<'r> OpTrapGetDump<'r> {
    pub fn new(mut request: Request<'r>) -> Self {
        Self::write_header(request.buf_mut());
        Self {
            request: request.set_dump(),
        }
    }
    pub fn encode_request<'buf>(buf: &'buf mut Vec<u8>) -> PushDevlink<&'buf mut Vec<u8>> {
        Self::write_header(buf);
        PushDevlink::new(buf)
    }
    pub fn encode(&mut self) -> PushDevlink<&mut Vec<u8>> {
        PushDevlink::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushDevlink<RequestBuf<'r>> {
        PushDevlink::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> IterableDevlink<'a> {
        let (_header, attrs) = buf.split_at(buf.len().min(BuiltinNfgenmsg::len()));
        IterableDevlink::with_loc(attrs, buf.as_ptr() as usize)
    }
    fn write_header<Prev: Pusher>(prev: &mut Prev) {
        let mut header = BuiltinNfgenmsg::new();
        header.cmd = 61u8;
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
impl NetlinkRequest for OpTrapGetDump<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Generic("devlink".as_bytes())
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = IterableDevlink<'buf>;
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
#[doc = "Get trap instances.\n\nRequest attributes:\n- [.push_bus_name()](PushDevlink::push_bus_name)\n- [.push_dev_name()](PushDevlink::push_dev_name)\n- [.push_trap_name()](PushDevlink::push_trap_name)\n- [.push_index()](PushDevlink::push_index)\n\nReply attributes:\n- [.get_bus_name()](IterableDevlink::get_bus_name)\n- [.get_dev_name()](IterableDevlink::get_dev_name)\n- [.get_trap_name()](IterableDevlink::get_trap_name)\n- [.get_index()](IterableDevlink::get_index)\n\n"]
#[derive(Debug)]
pub struct OpTrapGetDo<'r> {
    request: Request<'r>,
}
impl<'r> OpTrapGetDo<'r> {
    pub fn new(mut request: Request<'r>) -> Self {
        Self::write_header(request.buf_mut());
        Self { request: request }
    }
    pub fn encode_request<'buf>(buf: &'buf mut Vec<u8>) -> PushDevlink<&'buf mut Vec<u8>> {
        Self::write_header(buf);
        PushDevlink::new(buf)
    }
    pub fn encode(&mut self) -> PushDevlink<&mut Vec<u8>> {
        PushDevlink::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushDevlink<RequestBuf<'r>> {
        PushDevlink::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> IterableDevlink<'a> {
        let (_header, attrs) = buf.split_at(buf.len().min(BuiltinNfgenmsg::len()));
        IterableDevlink::with_loc(attrs, buf.as_ptr() as usize)
    }
    fn write_header<Prev: Pusher>(prev: &mut Prev) {
        let mut header = BuiltinNfgenmsg::new();
        header.cmd = 61u8;
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
impl NetlinkRequest for OpTrapGetDo<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Generic("devlink".as_bytes())
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = IterableDevlink<'buf>;
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
#[doc = "Set trap instances.\n\nFlags: admin-perm\n\nRequest attributes:\n- [.push_bus_name()](PushDevlink::push_bus_name)\n- [.push_dev_name()](PushDevlink::push_dev_name)\n- [.push_trap_name()](PushDevlink::push_trap_name)\n- [.push_trap_action()](PushDevlink::push_trap_action)\n- [.push_index()](PushDevlink::push_index)\n\n"]
#[derive(Debug)]
pub struct OpTrapSetDo<'r> {
    request: Request<'r>,
}
impl<'r> OpTrapSetDo<'r> {
    pub fn new(mut request: Request<'r>) -> Self {
        Self::write_header(request.buf_mut());
        Self { request: request }
    }
    pub fn encode_request<'buf>(buf: &'buf mut Vec<u8>) -> PushDevlink<&'buf mut Vec<u8>> {
        Self::write_header(buf);
        PushDevlink::new(buf)
    }
    pub fn encode(&mut self) -> PushDevlink<&mut Vec<u8>> {
        PushDevlink::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushDevlink<RequestBuf<'r>> {
        PushDevlink::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> IterableDevlink<'a> {
        let (_header, attrs) = buf.split_at(buf.len().min(BuiltinNfgenmsg::len()));
        IterableDevlink::with_loc(attrs, buf.as_ptr() as usize)
    }
    fn write_header<Prev: Pusher>(prev: &mut Prev) {
        let mut header = BuiltinNfgenmsg::new();
        header.cmd = 62u8;
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
impl NetlinkRequest for OpTrapSetDo<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Generic("devlink".as_bytes())
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = IterableDevlink<'buf>;
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
#[doc = "Get trap group instances.\n\nRequest attributes:\n- [.push_bus_name()](PushDevlink::push_bus_name)\n- [.push_dev_name()](PushDevlink::push_dev_name)\n- [.push_index()](PushDevlink::push_index)\n\nReply attributes:\n- [.get_bus_name()](IterableDevlink::get_bus_name)\n- [.get_dev_name()](IterableDevlink::get_dev_name)\n- [.get_trap_group_name()](IterableDevlink::get_trap_group_name)\n- [.get_index()](IterableDevlink::get_index)\n\n"]
#[derive(Debug)]
pub struct OpTrapGroupGetDump<'r> {
    request: Request<'r>,
}
impl<'r> OpTrapGroupGetDump<'r> {
    pub fn new(mut request: Request<'r>) -> Self {
        Self::write_header(request.buf_mut());
        Self {
            request: request.set_dump(),
        }
    }
    pub fn encode_request<'buf>(buf: &'buf mut Vec<u8>) -> PushDevlink<&'buf mut Vec<u8>> {
        Self::write_header(buf);
        PushDevlink::new(buf)
    }
    pub fn encode(&mut self) -> PushDevlink<&mut Vec<u8>> {
        PushDevlink::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushDevlink<RequestBuf<'r>> {
        PushDevlink::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> IterableDevlink<'a> {
        let (_header, attrs) = buf.split_at(buf.len().min(BuiltinNfgenmsg::len()));
        IterableDevlink::with_loc(attrs, buf.as_ptr() as usize)
    }
    fn write_header<Prev: Pusher>(prev: &mut Prev) {
        let mut header = BuiltinNfgenmsg::new();
        header.cmd = 65u8;
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
impl NetlinkRequest for OpTrapGroupGetDump<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Generic("devlink".as_bytes())
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = IterableDevlink<'buf>;
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
#[doc = "Get trap group instances.\n\nRequest attributes:\n- [.push_bus_name()](PushDevlink::push_bus_name)\n- [.push_dev_name()](PushDevlink::push_dev_name)\n- [.push_trap_group_name()](PushDevlink::push_trap_group_name)\n- [.push_index()](PushDevlink::push_index)\n\nReply attributes:\n- [.get_bus_name()](IterableDevlink::get_bus_name)\n- [.get_dev_name()](IterableDevlink::get_dev_name)\n- [.get_trap_group_name()](IterableDevlink::get_trap_group_name)\n- [.get_index()](IterableDevlink::get_index)\n\n"]
#[derive(Debug)]
pub struct OpTrapGroupGetDo<'r> {
    request: Request<'r>,
}
impl<'r> OpTrapGroupGetDo<'r> {
    pub fn new(mut request: Request<'r>) -> Self {
        Self::write_header(request.buf_mut());
        Self { request: request }
    }
    pub fn encode_request<'buf>(buf: &'buf mut Vec<u8>) -> PushDevlink<&'buf mut Vec<u8>> {
        Self::write_header(buf);
        PushDevlink::new(buf)
    }
    pub fn encode(&mut self) -> PushDevlink<&mut Vec<u8>> {
        PushDevlink::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushDevlink<RequestBuf<'r>> {
        PushDevlink::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> IterableDevlink<'a> {
        let (_header, attrs) = buf.split_at(buf.len().min(BuiltinNfgenmsg::len()));
        IterableDevlink::with_loc(attrs, buf.as_ptr() as usize)
    }
    fn write_header<Prev: Pusher>(prev: &mut Prev) {
        let mut header = BuiltinNfgenmsg::new();
        header.cmd = 65u8;
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
impl NetlinkRequest for OpTrapGroupGetDo<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Generic("devlink".as_bytes())
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = IterableDevlink<'buf>;
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
#[doc = "Set trap group instances.\n\nFlags: admin-perm\n\nRequest attributes:\n- [.push_bus_name()](PushDevlink::push_bus_name)\n- [.push_dev_name()](PushDevlink::push_dev_name)\n- [.push_trap_action()](PushDevlink::push_trap_action)\n- [.push_trap_group_name()](PushDevlink::push_trap_group_name)\n- [.push_trap_policer_id()](PushDevlink::push_trap_policer_id)\n- [.push_index()](PushDevlink::push_index)\n\n"]
#[derive(Debug)]
pub struct OpTrapGroupSetDo<'r> {
    request: Request<'r>,
}
impl<'r> OpTrapGroupSetDo<'r> {
    pub fn new(mut request: Request<'r>) -> Self {
        Self::write_header(request.buf_mut());
        Self { request: request }
    }
    pub fn encode_request<'buf>(buf: &'buf mut Vec<u8>) -> PushDevlink<&'buf mut Vec<u8>> {
        Self::write_header(buf);
        PushDevlink::new(buf)
    }
    pub fn encode(&mut self) -> PushDevlink<&mut Vec<u8>> {
        PushDevlink::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushDevlink<RequestBuf<'r>> {
        PushDevlink::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> IterableDevlink<'a> {
        let (_header, attrs) = buf.split_at(buf.len().min(BuiltinNfgenmsg::len()));
        IterableDevlink::with_loc(attrs, buf.as_ptr() as usize)
    }
    fn write_header<Prev: Pusher>(prev: &mut Prev) {
        let mut header = BuiltinNfgenmsg::new();
        header.cmd = 66u8;
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
impl NetlinkRequest for OpTrapGroupSetDo<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Generic("devlink".as_bytes())
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = IterableDevlink<'buf>;
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
#[doc = "Get trap policer instances.\n\nRequest attributes:\n- [.push_bus_name()](PushDevlink::push_bus_name)\n- [.push_dev_name()](PushDevlink::push_dev_name)\n- [.push_index()](PushDevlink::push_index)\n\nReply attributes:\n- [.get_bus_name()](IterableDevlink::get_bus_name)\n- [.get_dev_name()](IterableDevlink::get_dev_name)\n- [.get_trap_policer_id()](IterableDevlink::get_trap_policer_id)\n- [.get_index()](IterableDevlink::get_index)\n\n"]
#[derive(Debug)]
pub struct OpTrapPolicerGetDump<'r> {
    request: Request<'r>,
}
impl<'r> OpTrapPolicerGetDump<'r> {
    pub fn new(mut request: Request<'r>) -> Self {
        Self::write_header(request.buf_mut());
        Self {
            request: request.set_dump(),
        }
    }
    pub fn encode_request<'buf>(buf: &'buf mut Vec<u8>) -> PushDevlink<&'buf mut Vec<u8>> {
        Self::write_header(buf);
        PushDevlink::new(buf)
    }
    pub fn encode(&mut self) -> PushDevlink<&mut Vec<u8>> {
        PushDevlink::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushDevlink<RequestBuf<'r>> {
        PushDevlink::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> IterableDevlink<'a> {
        let (_header, attrs) = buf.split_at(buf.len().min(BuiltinNfgenmsg::len()));
        IterableDevlink::with_loc(attrs, buf.as_ptr() as usize)
    }
    fn write_header<Prev: Pusher>(prev: &mut Prev) {
        let mut header = BuiltinNfgenmsg::new();
        header.cmd = 69u8;
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
impl NetlinkRequest for OpTrapPolicerGetDump<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Generic("devlink".as_bytes())
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = IterableDevlink<'buf>;
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
#[doc = "Get trap policer instances.\n\nRequest attributes:\n- [.push_bus_name()](PushDevlink::push_bus_name)\n- [.push_dev_name()](PushDevlink::push_dev_name)\n- [.push_trap_policer_id()](PushDevlink::push_trap_policer_id)\n- [.push_index()](PushDevlink::push_index)\n\nReply attributes:\n- [.get_bus_name()](IterableDevlink::get_bus_name)\n- [.get_dev_name()](IterableDevlink::get_dev_name)\n- [.get_trap_policer_id()](IterableDevlink::get_trap_policer_id)\n- [.get_index()](IterableDevlink::get_index)\n\n"]
#[derive(Debug)]
pub struct OpTrapPolicerGetDo<'r> {
    request: Request<'r>,
}
impl<'r> OpTrapPolicerGetDo<'r> {
    pub fn new(mut request: Request<'r>) -> Self {
        Self::write_header(request.buf_mut());
        Self { request: request }
    }
    pub fn encode_request<'buf>(buf: &'buf mut Vec<u8>) -> PushDevlink<&'buf mut Vec<u8>> {
        Self::write_header(buf);
        PushDevlink::new(buf)
    }
    pub fn encode(&mut self) -> PushDevlink<&mut Vec<u8>> {
        PushDevlink::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushDevlink<RequestBuf<'r>> {
        PushDevlink::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> IterableDevlink<'a> {
        let (_header, attrs) = buf.split_at(buf.len().min(BuiltinNfgenmsg::len()));
        IterableDevlink::with_loc(attrs, buf.as_ptr() as usize)
    }
    fn write_header<Prev: Pusher>(prev: &mut Prev) {
        let mut header = BuiltinNfgenmsg::new();
        header.cmd = 69u8;
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
impl NetlinkRequest for OpTrapPolicerGetDo<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Generic("devlink".as_bytes())
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = IterableDevlink<'buf>;
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
#[doc = "Get trap policer instances.\n\nFlags: admin-perm\n\nRequest attributes:\n- [.push_bus_name()](PushDevlink::push_bus_name)\n- [.push_dev_name()](PushDevlink::push_dev_name)\n- [.push_trap_policer_id()](PushDevlink::push_trap_policer_id)\n- [.push_trap_policer_rate()](PushDevlink::push_trap_policer_rate)\n- [.push_trap_policer_burst()](PushDevlink::push_trap_policer_burst)\n- [.push_index()](PushDevlink::push_index)\n\n"]
#[derive(Debug)]
pub struct OpTrapPolicerSetDo<'r> {
    request: Request<'r>,
}
impl<'r> OpTrapPolicerSetDo<'r> {
    pub fn new(mut request: Request<'r>) -> Self {
        Self::write_header(request.buf_mut());
        Self { request: request }
    }
    pub fn encode_request<'buf>(buf: &'buf mut Vec<u8>) -> PushDevlink<&'buf mut Vec<u8>> {
        Self::write_header(buf);
        PushDevlink::new(buf)
    }
    pub fn encode(&mut self) -> PushDevlink<&mut Vec<u8>> {
        PushDevlink::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushDevlink<RequestBuf<'r>> {
        PushDevlink::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> IterableDevlink<'a> {
        let (_header, attrs) = buf.split_at(buf.len().min(BuiltinNfgenmsg::len()));
        IterableDevlink::with_loc(attrs, buf.as_ptr() as usize)
    }
    fn write_header<Prev: Pusher>(prev: &mut Prev) {
        let mut header = BuiltinNfgenmsg::new();
        header.cmd = 70u8;
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
impl NetlinkRequest for OpTrapPolicerSetDo<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Generic("devlink".as_bytes())
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = IterableDevlink<'buf>;
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
#[doc = "Test health reporter instances.\n\nFlags: admin-perm\n\nRequest attributes:\n- [.push_bus_name()](PushDevlink::push_bus_name)\n- [.push_dev_name()](PushDevlink::push_dev_name)\n- [.push_port_index()](PushDevlink::push_port_index)\n- [.push_health_reporter_name()](PushDevlink::push_health_reporter_name)\n- [.push_index()](PushDevlink::push_index)\n\n"]
#[derive(Debug)]
pub struct OpHealthReporterTestDo<'r> {
    request: Request<'r>,
}
impl<'r> OpHealthReporterTestDo<'r> {
    pub fn new(mut request: Request<'r>) -> Self {
        Self::write_header(request.buf_mut());
        Self { request: request }
    }
    pub fn encode_request<'buf>(buf: &'buf mut Vec<u8>) -> PushDevlink<&'buf mut Vec<u8>> {
        Self::write_header(buf);
        PushDevlink::new(buf)
    }
    pub fn encode(&mut self) -> PushDevlink<&mut Vec<u8>> {
        PushDevlink::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushDevlink<RequestBuf<'r>> {
        PushDevlink::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> IterableDevlink<'a> {
        let (_header, attrs) = buf.split_at(buf.len().min(BuiltinNfgenmsg::len()));
        IterableDevlink::with_loc(attrs, buf.as_ptr() as usize)
    }
    fn write_header<Prev: Pusher>(prev: &mut Prev) {
        let mut header = BuiltinNfgenmsg::new();
        header.cmd = 73u8;
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
impl NetlinkRequest for OpHealthReporterTestDo<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Generic("devlink".as_bytes())
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = IterableDevlink<'buf>;
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
#[doc = "Get rate instances.\n\nRequest attributes:\n- [.push_bus_name()](PushDevlink::push_bus_name)\n- [.push_dev_name()](PushDevlink::push_dev_name)\n- [.push_index()](PushDevlink::push_index)\n\nReply attributes:\n- [.get_bus_name()](IterableDevlink::get_bus_name)\n- [.get_dev_name()](IterableDevlink::get_dev_name)\n- [.get_port_index()](IterableDevlink::get_port_index)\n- [.get_rate_node_name()](IterableDevlink::get_rate_node_name)\n- [.get_index()](IterableDevlink::get_index)\n\n"]
#[derive(Debug)]
pub struct OpRateGetDump<'r> {
    request: Request<'r>,
}
impl<'r> OpRateGetDump<'r> {
    pub fn new(mut request: Request<'r>) -> Self {
        Self::write_header(request.buf_mut());
        Self {
            request: request.set_dump(),
        }
    }
    pub fn encode_request<'buf>(buf: &'buf mut Vec<u8>) -> PushDevlink<&'buf mut Vec<u8>> {
        Self::write_header(buf);
        PushDevlink::new(buf)
    }
    pub fn encode(&mut self) -> PushDevlink<&mut Vec<u8>> {
        PushDevlink::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushDevlink<RequestBuf<'r>> {
        PushDevlink::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> IterableDevlink<'a> {
        let (_header, attrs) = buf.split_at(buf.len().min(BuiltinNfgenmsg::len()));
        IterableDevlink::with_loc(attrs, buf.as_ptr() as usize)
    }
    fn write_header<Prev: Pusher>(prev: &mut Prev) {
        let mut header = BuiltinNfgenmsg::new();
        header.cmd = 74u8;
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
impl NetlinkRequest for OpRateGetDump<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Generic("devlink".as_bytes())
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = IterableDevlink<'buf>;
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
#[doc = "Get rate instances.\n\nRequest attributes:\n- [.push_bus_name()](PushDevlink::push_bus_name)\n- [.push_dev_name()](PushDevlink::push_dev_name)\n- [.push_port_index()](PushDevlink::push_port_index)\n- [.push_rate_node_name()](PushDevlink::push_rate_node_name)\n- [.push_index()](PushDevlink::push_index)\n\nReply attributes:\n- [.get_bus_name()](IterableDevlink::get_bus_name)\n- [.get_dev_name()](IterableDevlink::get_dev_name)\n- [.get_port_index()](IterableDevlink::get_port_index)\n- [.get_rate_node_name()](IterableDevlink::get_rate_node_name)\n- [.get_index()](IterableDevlink::get_index)\n\n"]
#[derive(Debug)]
pub struct OpRateGetDo<'r> {
    request: Request<'r>,
}
impl<'r> OpRateGetDo<'r> {
    pub fn new(mut request: Request<'r>) -> Self {
        Self::write_header(request.buf_mut());
        Self { request: request }
    }
    pub fn encode_request<'buf>(buf: &'buf mut Vec<u8>) -> PushDevlink<&'buf mut Vec<u8>> {
        Self::write_header(buf);
        PushDevlink::new(buf)
    }
    pub fn encode(&mut self) -> PushDevlink<&mut Vec<u8>> {
        PushDevlink::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushDevlink<RequestBuf<'r>> {
        PushDevlink::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> IterableDevlink<'a> {
        let (_header, attrs) = buf.split_at(buf.len().min(BuiltinNfgenmsg::len()));
        IterableDevlink::with_loc(attrs, buf.as_ptr() as usize)
    }
    fn write_header<Prev: Pusher>(prev: &mut Prev) {
        let mut header = BuiltinNfgenmsg::new();
        header.cmd = 74u8;
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
impl NetlinkRequest for OpRateGetDo<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Generic("devlink".as_bytes())
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = IterableDevlink<'buf>;
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
#[doc = "Set rate instances.\n\nFlags: admin-perm\n\nRequest attributes:\n- [.push_bus_name()](PushDevlink::push_bus_name)\n- [.push_dev_name()](PushDevlink::push_dev_name)\n- [.push_rate_tx_share()](PushDevlink::push_rate_tx_share)\n- [.push_rate_tx_max()](PushDevlink::push_rate_tx_max)\n- [.push_rate_node_name()](PushDevlink::push_rate_node_name)\n- [.push_rate_parent_node_name()](PushDevlink::push_rate_parent_node_name)\n- [.push_rate_tx_priority()](PushDevlink::push_rate_tx_priority)\n- [.push_rate_tx_weight()](PushDevlink::push_rate_tx_weight)\n- [.nested_rate_tc_bws()](PushDevlink::nested_rate_tc_bws)\n- [.push_index()](PushDevlink::push_index)\n\n"]
#[derive(Debug)]
pub struct OpRateSetDo<'r> {
    request: Request<'r>,
}
impl<'r> OpRateSetDo<'r> {
    pub fn new(mut request: Request<'r>) -> Self {
        Self::write_header(request.buf_mut());
        Self { request: request }
    }
    pub fn encode_request<'buf>(buf: &'buf mut Vec<u8>) -> PushDevlink<&'buf mut Vec<u8>> {
        Self::write_header(buf);
        PushDevlink::new(buf)
    }
    pub fn encode(&mut self) -> PushDevlink<&mut Vec<u8>> {
        PushDevlink::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushDevlink<RequestBuf<'r>> {
        PushDevlink::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> IterableDevlink<'a> {
        let (_header, attrs) = buf.split_at(buf.len().min(BuiltinNfgenmsg::len()));
        IterableDevlink::with_loc(attrs, buf.as_ptr() as usize)
    }
    fn write_header<Prev: Pusher>(prev: &mut Prev) {
        let mut header = BuiltinNfgenmsg::new();
        header.cmd = 75u8;
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
impl NetlinkRequest for OpRateSetDo<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Generic("devlink".as_bytes())
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = IterableDevlink<'buf>;
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
#[doc = "Create rate instances.\n\nFlags: admin-perm\n\nRequest attributes:\n- [.push_bus_name()](PushDevlink::push_bus_name)\n- [.push_dev_name()](PushDevlink::push_dev_name)\n- [.push_rate_tx_share()](PushDevlink::push_rate_tx_share)\n- [.push_rate_tx_max()](PushDevlink::push_rate_tx_max)\n- [.push_rate_node_name()](PushDevlink::push_rate_node_name)\n- [.push_rate_parent_node_name()](PushDevlink::push_rate_parent_node_name)\n- [.push_rate_tx_priority()](PushDevlink::push_rate_tx_priority)\n- [.push_rate_tx_weight()](PushDevlink::push_rate_tx_weight)\n- [.nested_rate_tc_bws()](PushDevlink::nested_rate_tc_bws)\n- [.push_index()](PushDevlink::push_index)\n\n"]
#[derive(Debug)]
pub struct OpRateNewDo<'r> {
    request: Request<'r>,
}
impl<'r> OpRateNewDo<'r> {
    pub fn new(mut request: Request<'r>) -> Self {
        Self::write_header(request.buf_mut());
        Self { request: request }
    }
    pub fn encode_request<'buf>(buf: &'buf mut Vec<u8>) -> PushDevlink<&'buf mut Vec<u8>> {
        Self::write_header(buf);
        PushDevlink::new(buf)
    }
    pub fn encode(&mut self) -> PushDevlink<&mut Vec<u8>> {
        PushDevlink::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushDevlink<RequestBuf<'r>> {
        PushDevlink::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> IterableDevlink<'a> {
        let (_header, attrs) = buf.split_at(buf.len().min(BuiltinNfgenmsg::len()));
        IterableDevlink::with_loc(attrs, buf.as_ptr() as usize)
    }
    fn write_header<Prev: Pusher>(prev: &mut Prev) {
        let mut header = BuiltinNfgenmsg::new();
        header.cmd = 76u8;
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
impl NetlinkRequest for OpRateNewDo<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Generic("devlink".as_bytes())
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = IterableDevlink<'buf>;
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
#[doc = "Delete rate instances.\n\nFlags: admin-perm\n\nRequest attributes:\n- [.push_bus_name()](PushDevlink::push_bus_name)\n- [.push_dev_name()](PushDevlink::push_dev_name)\n- [.push_rate_node_name()](PushDevlink::push_rate_node_name)\n- [.push_index()](PushDevlink::push_index)\n\n"]
#[derive(Debug)]
pub struct OpRateDelDo<'r> {
    request: Request<'r>,
}
impl<'r> OpRateDelDo<'r> {
    pub fn new(mut request: Request<'r>) -> Self {
        Self::write_header(request.buf_mut());
        Self { request: request }
    }
    pub fn encode_request<'buf>(buf: &'buf mut Vec<u8>) -> PushDevlink<&'buf mut Vec<u8>> {
        Self::write_header(buf);
        PushDevlink::new(buf)
    }
    pub fn encode(&mut self) -> PushDevlink<&mut Vec<u8>> {
        PushDevlink::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushDevlink<RequestBuf<'r>> {
        PushDevlink::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> IterableDevlink<'a> {
        let (_header, attrs) = buf.split_at(buf.len().min(BuiltinNfgenmsg::len()));
        IterableDevlink::with_loc(attrs, buf.as_ptr() as usize)
    }
    fn write_header<Prev: Pusher>(prev: &mut Prev) {
        let mut header = BuiltinNfgenmsg::new();
        header.cmd = 77u8;
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
impl NetlinkRequest for OpRateDelDo<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Generic("devlink".as_bytes())
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = IterableDevlink<'buf>;
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
#[doc = "Get line card instances.\n\nRequest attributes:\n- [.push_bus_name()](PushDevlink::push_bus_name)\n- [.push_dev_name()](PushDevlink::push_dev_name)\n- [.push_index()](PushDevlink::push_index)\n\nReply attributes:\n- [.get_bus_name()](IterableDevlink::get_bus_name)\n- [.get_dev_name()](IterableDevlink::get_dev_name)\n- [.get_linecard_index()](IterableDevlink::get_linecard_index)\n- [.get_index()](IterableDevlink::get_index)\n\n"]
#[derive(Debug)]
pub struct OpLinecardGetDump<'r> {
    request: Request<'r>,
}
impl<'r> OpLinecardGetDump<'r> {
    pub fn new(mut request: Request<'r>) -> Self {
        Self::write_header(request.buf_mut());
        Self {
            request: request.set_dump(),
        }
    }
    pub fn encode_request<'buf>(buf: &'buf mut Vec<u8>) -> PushDevlink<&'buf mut Vec<u8>> {
        Self::write_header(buf);
        PushDevlink::new(buf)
    }
    pub fn encode(&mut self) -> PushDevlink<&mut Vec<u8>> {
        PushDevlink::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushDevlink<RequestBuf<'r>> {
        PushDevlink::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> IterableDevlink<'a> {
        let (_header, attrs) = buf.split_at(buf.len().min(BuiltinNfgenmsg::len()));
        IterableDevlink::with_loc(attrs, buf.as_ptr() as usize)
    }
    fn write_header<Prev: Pusher>(prev: &mut Prev) {
        let mut header = BuiltinNfgenmsg::new();
        header.cmd = 78u8;
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
impl NetlinkRequest for OpLinecardGetDump<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Generic("devlink".as_bytes())
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = IterableDevlink<'buf>;
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
#[doc = "Get line card instances.\n\nRequest attributes:\n- [.push_bus_name()](PushDevlink::push_bus_name)\n- [.push_dev_name()](PushDevlink::push_dev_name)\n- [.push_linecard_index()](PushDevlink::push_linecard_index)\n- [.push_index()](PushDevlink::push_index)\n\nReply attributes:\n- [.get_bus_name()](IterableDevlink::get_bus_name)\n- [.get_dev_name()](IterableDevlink::get_dev_name)\n- [.get_linecard_index()](IterableDevlink::get_linecard_index)\n- [.get_index()](IterableDevlink::get_index)\n\n"]
#[derive(Debug)]
pub struct OpLinecardGetDo<'r> {
    request: Request<'r>,
}
impl<'r> OpLinecardGetDo<'r> {
    pub fn new(mut request: Request<'r>) -> Self {
        Self::write_header(request.buf_mut());
        Self { request: request }
    }
    pub fn encode_request<'buf>(buf: &'buf mut Vec<u8>) -> PushDevlink<&'buf mut Vec<u8>> {
        Self::write_header(buf);
        PushDevlink::new(buf)
    }
    pub fn encode(&mut self) -> PushDevlink<&mut Vec<u8>> {
        PushDevlink::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushDevlink<RequestBuf<'r>> {
        PushDevlink::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> IterableDevlink<'a> {
        let (_header, attrs) = buf.split_at(buf.len().min(BuiltinNfgenmsg::len()));
        IterableDevlink::with_loc(attrs, buf.as_ptr() as usize)
    }
    fn write_header<Prev: Pusher>(prev: &mut Prev) {
        let mut header = BuiltinNfgenmsg::new();
        header.cmd = 78u8;
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
impl NetlinkRequest for OpLinecardGetDo<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Generic("devlink".as_bytes())
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = IterableDevlink<'buf>;
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
#[doc = "Set line card instances.\n\nFlags: admin-perm\n\nRequest attributes:\n- [.push_bus_name()](PushDevlink::push_bus_name)\n- [.push_dev_name()](PushDevlink::push_dev_name)\n- [.push_linecard_index()](PushDevlink::push_linecard_index)\n- [.push_linecard_type()](PushDevlink::push_linecard_type)\n- [.push_index()](PushDevlink::push_index)\n\n"]
#[derive(Debug)]
pub struct OpLinecardSetDo<'r> {
    request: Request<'r>,
}
impl<'r> OpLinecardSetDo<'r> {
    pub fn new(mut request: Request<'r>) -> Self {
        Self::write_header(request.buf_mut());
        Self { request: request }
    }
    pub fn encode_request<'buf>(buf: &'buf mut Vec<u8>) -> PushDevlink<&'buf mut Vec<u8>> {
        Self::write_header(buf);
        PushDevlink::new(buf)
    }
    pub fn encode(&mut self) -> PushDevlink<&mut Vec<u8>> {
        PushDevlink::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushDevlink<RequestBuf<'r>> {
        PushDevlink::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> IterableDevlink<'a> {
        let (_header, attrs) = buf.split_at(buf.len().min(BuiltinNfgenmsg::len()));
        IterableDevlink::with_loc(attrs, buf.as_ptr() as usize)
    }
    fn write_header<Prev: Pusher>(prev: &mut Prev) {
        let mut header = BuiltinNfgenmsg::new();
        header.cmd = 79u8;
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
impl NetlinkRequest for OpLinecardSetDo<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Generic("devlink".as_bytes())
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = IterableDevlink<'buf>;
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
#[doc = "Get device selftest instances.\n\nReply attributes:\n- [.get_bus_name()](IterableDevlink::get_bus_name)\n- [.get_dev_name()](IterableDevlink::get_dev_name)\n- [.get_index()](IterableDevlink::get_index)\n\n"]
#[derive(Debug)]
pub struct OpSelftestsGetDump<'r> {
    request: Request<'r>,
}
impl<'r> OpSelftestsGetDump<'r> {
    pub fn new(mut request: Request<'r>) -> Self {
        Self::write_header(request.buf_mut());
        Self {
            request: request.set_dump(),
        }
    }
    pub fn encode_request<'buf>(buf: &'buf mut Vec<u8>) -> PushDevlink<&'buf mut Vec<u8>> {
        Self::write_header(buf);
        PushDevlink::new(buf)
    }
    pub fn encode(&mut self) -> PushDevlink<&mut Vec<u8>> {
        PushDevlink::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushDevlink<RequestBuf<'r>> {
        PushDevlink::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> IterableDevlink<'a> {
        let (_header, attrs) = buf.split_at(buf.len().min(BuiltinNfgenmsg::len()));
        IterableDevlink::with_loc(attrs, buf.as_ptr() as usize)
    }
    fn write_header<Prev: Pusher>(prev: &mut Prev) {
        let mut header = BuiltinNfgenmsg::new();
        header.cmd = 82u8;
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
impl NetlinkRequest for OpSelftestsGetDump<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Generic("devlink".as_bytes())
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = IterableDevlink<'buf>;
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
#[doc = "Get device selftest instances.\n\nRequest attributes:\n- [.push_bus_name()](PushDevlink::push_bus_name)\n- [.push_dev_name()](PushDevlink::push_dev_name)\n- [.push_index()](PushDevlink::push_index)\n\nReply attributes:\n- [.get_bus_name()](IterableDevlink::get_bus_name)\n- [.get_dev_name()](IterableDevlink::get_dev_name)\n- [.get_index()](IterableDevlink::get_index)\n\n"]
#[derive(Debug)]
pub struct OpSelftestsGetDo<'r> {
    request: Request<'r>,
}
impl<'r> OpSelftestsGetDo<'r> {
    pub fn new(mut request: Request<'r>) -> Self {
        Self::write_header(request.buf_mut());
        Self { request: request }
    }
    pub fn encode_request<'buf>(buf: &'buf mut Vec<u8>) -> PushDevlink<&'buf mut Vec<u8>> {
        Self::write_header(buf);
        PushDevlink::new(buf)
    }
    pub fn encode(&mut self) -> PushDevlink<&mut Vec<u8>> {
        PushDevlink::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushDevlink<RequestBuf<'r>> {
        PushDevlink::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> IterableDevlink<'a> {
        let (_header, attrs) = buf.split_at(buf.len().min(BuiltinNfgenmsg::len()));
        IterableDevlink::with_loc(attrs, buf.as_ptr() as usize)
    }
    fn write_header<Prev: Pusher>(prev: &mut Prev) {
        let mut header = BuiltinNfgenmsg::new();
        header.cmd = 82u8;
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
impl NetlinkRequest for OpSelftestsGetDo<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Generic("devlink".as_bytes())
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = IterableDevlink<'buf>;
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
#[doc = "Run device selftest instances.\n\nFlags: admin-perm\n\nRequest attributes:\n- [.push_bus_name()](PushDevlink::push_bus_name)\n- [.push_dev_name()](PushDevlink::push_dev_name)\n- [.nested_selftests()](PushDevlink::nested_selftests)\n- [.push_index()](PushDevlink::push_index)\n\n"]
#[derive(Debug)]
pub struct OpSelftestsRunDo<'r> {
    request: Request<'r>,
}
impl<'r> OpSelftestsRunDo<'r> {
    pub fn new(mut request: Request<'r>) -> Self {
        Self::write_header(request.buf_mut());
        Self { request: request }
    }
    pub fn encode_request<'buf>(buf: &'buf mut Vec<u8>) -> PushDevlink<&'buf mut Vec<u8>> {
        Self::write_header(buf);
        PushDevlink::new(buf)
    }
    pub fn encode(&mut self) -> PushDevlink<&mut Vec<u8>> {
        PushDevlink::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushDevlink<RequestBuf<'r>> {
        PushDevlink::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> IterableDevlink<'a> {
        let (_header, attrs) = buf.split_at(buf.len().min(BuiltinNfgenmsg::len()));
        IterableDevlink::with_loc(attrs, buf.as_ptr() as usize)
    }
    fn write_header<Prev: Pusher>(prev: &mut Prev) {
        let mut header = BuiltinNfgenmsg::new();
        header.cmd = 83u8;
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
impl NetlinkRequest for OpSelftestsRunDo<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Generic("devlink".as_bytes())
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = IterableDevlink<'buf>;
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
#[doc = "Set notification messages socket filter.\n\nRequest attributes:\n- [.push_bus_name()](PushDevlink::push_bus_name)\n- [.push_dev_name()](PushDevlink::push_dev_name)\n- [.push_port_index()](PushDevlink::push_port_index)\n- [.push_index()](PushDevlink::push_index)\n\n"]
#[derive(Debug)]
pub struct OpNotifyFilterSetDo<'r> {
    request: Request<'r>,
}
impl<'r> OpNotifyFilterSetDo<'r> {
    pub fn new(mut request: Request<'r>) -> Self {
        Self::write_header(request.buf_mut());
        Self { request: request }
    }
    pub fn encode_request<'buf>(buf: &'buf mut Vec<u8>) -> PushDevlink<&'buf mut Vec<u8>> {
        Self::write_header(buf);
        PushDevlink::new(buf)
    }
    pub fn encode(&mut self) -> PushDevlink<&mut Vec<u8>> {
        PushDevlink::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushDevlink<RequestBuf<'r>> {
        PushDevlink::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> IterableDevlink<'a> {
        let (_header, attrs) = buf.split_at(buf.len().min(BuiltinNfgenmsg::len()));
        IterableDevlink::with_loc(attrs, buf.as_ptr() as usize)
    }
    fn write_header<Prev: Pusher>(prev: &mut Prev) {
        let mut header = BuiltinNfgenmsg::new();
        header.cmd = 84u8;
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
impl NetlinkRequest for OpNotifyFilterSetDo<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Generic("devlink".as_bytes())
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = IterableDevlink<'buf>;
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
    #[doc = "Get devlink instances.\n\nReply attributes:\n- [.get_bus_name()](IterableDevlink::get_bus_name)\n- [.get_dev_name()](IterableDevlink::get_dev_name)\n- [.get_reload_failed()](IterableDevlink::get_reload_failed)\n- [.get_dev_stats()](IterableDevlink::get_dev_stats)\n- [.get_index()](IterableDevlink::get_index)\n\n"]
    pub fn op_get_dump(self) -> OpGetDump<'buf> {
        let mut res = OpGetDump::new(self);
        res.request
            .do_writeback(res.protocol(), "op-get-dump", OpGetDump::lookup);
        res
    }
    #[doc = "Get devlink instances.\n\nRequest attributes:\n- [.push_bus_name()](PushDevlink::push_bus_name)\n- [.push_dev_name()](PushDevlink::push_dev_name)\n- [.push_index()](PushDevlink::push_index)\n\nReply attributes:\n- [.get_bus_name()](IterableDevlink::get_bus_name)\n- [.get_dev_name()](IterableDevlink::get_dev_name)\n- [.get_reload_failed()](IterableDevlink::get_reload_failed)\n- [.get_dev_stats()](IterableDevlink::get_dev_stats)\n- [.get_index()](IterableDevlink::get_index)\n\n"]
    pub fn op_get_do(self) -> OpGetDo<'buf> {
        let mut res = OpGetDo::new(self);
        res.request
            .do_writeback(res.protocol(), "op-get-do", OpGetDo::lookup);
        res
    }
    #[doc = "Get devlink port instances.\n\nRequest attributes:\n- [.push_bus_name()](PushDevlink::push_bus_name)\n- [.push_dev_name()](PushDevlink::push_dev_name)\n- [.push_index()](PushDevlink::push_index)\n\nReply attributes:\n- [.get_bus_name()](IterableDevlink::get_bus_name)\n- [.get_dev_name()](IterableDevlink::get_dev_name)\n- [.get_port_index()](IterableDevlink::get_port_index)\n- [.get_index()](IterableDevlink::get_index)\n\n"]
    pub fn op_port_get_dump(self) -> OpPortGetDump<'buf> {
        let mut res = OpPortGetDump::new(self);
        res.request
            .do_writeback(res.protocol(), "op-port-get-dump", OpPortGetDump::lookup);
        res
    }
    #[doc = "Get devlink port instances.\n\nRequest attributes:\n- [.push_bus_name()](PushDevlink::push_bus_name)\n- [.push_dev_name()](PushDevlink::push_dev_name)\n- [.push_port_index()](PushDevlink::push_port_index)\n- [.push_index()](PushDevlink::push_index)\n\nReply attributes:\n- [.get_bus_name()](IterableDevlink::get_bus_name)\n- [.get_dev_name()](IterableDevlink::get_dev_name)\n- [.get_port_index()](IterableDevlink::get_port_index)\n- [.get_index()](IterableDevlink::get_index)\n\n"]
    pub fn op_port_get_do(self) -> OpPortGetDo<'buf> {
        let mut res = OpPortGetDo::new(self);
        res.request
            .do_writeback(res.protocol(), "op-port-get-do", OpPortGetDo::lookup);
        res
    }
    #[doc = "Set devlink port instances.\n\nFlags: admin-perm\n\nRequest attributes:\n- [.push_bus_name()](PushDevlink::push_bus_name)\n- [.push_dev_name()](PushDevlink::push_dev_name)\n- [.push_port_index()](PushDevlink::push_port_index)\n- [.push_port_type()](PushDevlink::push_port_type)\n- [.nested_port_function()](PushDevlink::nested_port_function)\n- [.push_index()](PushDevlink::push_index)\n\n"]
    pub fn op_port_set_do(self) -> OpPortSetDo<'buf> {
        let mut res = OpPortSetDo::new(self);
        res.request
            .do_writeback(res.protocol(), "op-port-set-do", OpPortSetDo::lookup);
        res
    }
    #[doc = "Create devlink port instances.\n\nFlags: admin-perm\n\nRequest attributes:\n- [.push_bus_name()](PushDevlink::push_bus_name)\n- [.push_dev_name()](PushDevlink::push_dev_name)\n- [.push_port_index()](PushDevlink::push_port_index)\n- [.push_port_flavour()](PushDevlink::push_port_flavour)\n- [.push_port_pci_pf_number()](PushDevlink::push_port_pci_pf_number)\n- [.push_port_controller_number()](PushDevlink::push_port_controller_number)\n- [.push_port_pci_sf_number()](PushDevlink::push_port_pci_sf_number)\n- [.push_index()](PushDevlink::push_index)\n\nReply attributes:\n- [.get_bus_name()](IterableDevlink::get_bus_name)\n- [.get_dev_name()](IterableDevlink::get_dev_name)\n- [.get_port_index()](IterableDevlink::get_port_index)\n- [.get_index()](IterableDevlink::get_index)\n\n"]
    pub fn op_port_new_do(self) -> OpPortNewDo<'buf> {
        let mut res = OpPortNewDo::new(self);
        res.request
            .do_writeback(res.protocol(), "op-port-new-do", OpPortNewDo::lookup);
        res
    }
    #[doc = "Delete devlink port instances.\n\nFlags: admin-perm\n\nRequest attributes:\n- [.push_bus_name()](PushDevlink::push_bus_name)\n- [.push_dev_name()](PushDevlink::push_dev_name)\n- [.push_port_index()](PushDevlink::push_port_index)\n- [.push_index()](PushDevlink::push_index)\n\n"]
    pub fn op_port_del_do(self) -> OpPortDelDo<'buf> {
        let mut res = OpPortDelDo::new(self);
        res.request
            .do_writeback(res.protocol(), "op-port-del-do", OpPortDelDo::lookup);
        res
    }
    #[doc = "Split devlink port instances.\n\nFlags: admin-perm\n\nRequest attributes:\n- [.push_bus_name()](PushDevlink::push_bus_name)\n- [.push_dev_name()](PushDevlink::push_dev_name)\n- [.push_port_index()](PushDevlink::push_port_index)\n- [.push_port_split_count()](PushDevlink::push_port_split_count)\n- [.push_index()](PushDevlink::push_index)\n\n"]
    pub fn op_port_split_do(self) -> OpPortSplitDo<'buf> {
        let mut res = OpPortSplitDo::new(self);
        res.request
            .do_writeback(res.protocol(), "op-port-split-do", OpPortSplitDo::lookup);
        res
    }
    #[doc = "Unplit devlink port instances.\n\nFlags: admin-perm\n\nRequest attributes:\n- [.push_bus_name()](PushDevlink::push_bus_name)\n- [.push_dev_name()](PushDevlink::push_dev_name)\n- [.push_port_index()](PushDevlink::push_port_index)\n- [.push_index()](PushDevlink::push_index)\n\n"]
    pub fn op_port_unsplit_do(self) -> OpPortUnsplitDo<'buf> {
        let mut res = OpPortUnsplitDo::new(self);
        res.request.do_writeback(
            res.protocol(),
            "op-port-unsplit-do",
            OpPortUnsplitDo::lookup,
        );
        res
    }
    #[doc = "Get shared buffer instances.\n\nRequest attributes:\n- [.push_bus_name()](PushDevlink::push_bus_name)\n- [.push_dev_name()](PushDevlink::push_dev_name)\n- [.push_index()](PushDevlink::push_index)\n\nReply attributes:\n- [.get_bus_name()](IterableDevlink::get_bus_name)\n- [.get_dev_name()](IterableDevlink::get_dev_name)\n- [.get_sb_index()](IterableDevlink::get_sb_index)\n- [.get_index()](IterableDevlink::get_index)\n\n"]
    pub fn op_sb_get_dump(self) -> OpSbGetDump<'buf> {
        let mut res = OpSbGetDump::new(self);
        res.request
            .do_writeback(res.protocol(), "op-sb-get-dump", OpSbGetDump::lookup);
        res
    }
    #[doc = "Get shared buffer instances.\n\nRequest attributes:\n- [.push_bus_name()](PushDevlink::push_bus_name)\n- [.push_dev_name()](PushDevlink::push_dev_name)\n- [.push_sb_index()](PushDevlink::push_sb_index)\n- [.push_index()](PushDevlink::push_index)\n\nReply attributes:\n- [.get_bus_name()](IterableDevlink::get_bus_name)\n- [.get_dev_name()](IterableDevlink::get_dev_name)\n- [.get_sb_index()](IterableDevlink::get_sb_index)\n- [.get_index()](IterableDevlink::get_index)\n\n"]
    pub fn op_sb_get_do(self) -> OpSbGetDo<'buf> {
        let mut res = OpSbGetDo::new(self);
        res.request
            .do_writeback(res.protocol(), "op-sb-get-do", OpSbGetDo::lookup);
        res
    }
    #[doc = "Get shared buffer pool instances.\n\nRequest attributes:\n- [.push_bus_name()](PushDevlink::push_bus_name)\n- [.push_dev_name()](PushDevlink::push_dev_name)\n- [.push_index()](PushDevlink::push_index)\n\nReply attributes:\n- [.get_bus_name()](IterableDevlink::get_bus_name)\n- [.get_dev_name()](IterableDevlink::get_dev_name)\n- [.get_sb_index()](IterableDevlink::get_sb_index)\n- [.get_sb_pool_index()](IterableDevlink::get_sb_pool_index)\n- [.get_index()](IterableDevlink::get_index)\n\n"]
    pub fn op_sb_pool_get_dump(self) -> OpSbPoolGetDump<'buf> {
        let mut res = OpSbPoolGetDump::new(self);
        res.request.do_writeback(
            res.protocol(),
            "op-sb-pool-get-dump",
            OpSbPoolGetDump::lookup,
        );
        res
    }
    #[doc = "Get shared buffer pool instances.\n\nRequest attributes:\n- [.push_bus_name()](PushDevlink::push_bus_name)\n- [.push_dev_name()](PushDevlink::push_dev_name)\n- [.push_sb_index()](PushDevlink::push_sb_index)\n- [.push_sb_pool_index()](PushDevlink::push_sb_pool_index)\n- [.push_index()](PushDevlink::push_index)\n\nReply attributes:\n- [.get_bus_name()](IterableDevlink::get_bus_name)\n- [.get_dev_name()](IterableDevlink::get_dev_name)\n- [.get_sb_index()](IterableDevlink::get_sb_index)\n- [.get_sb_pool_index()](IterableDevlink::get_sb_pool_index)\n- [.get_index()](IterableDevlink::get_index)\n\n"]
    pub fn op_sb_pool_get_do(self) -> OpSbPoolGetDo<'buf> {
        let mut res = OpSbPoolGetDo::new(self);
        res.request
            .do_writeback(res.protocol(), "op-sb-pool-get-do", OpSbPoolGetDo::lookup);
        res
    }
    #[doc = "Set shared buffer pool instances.\n\nFlags: admin-perm\n\nRequest attributes:\n- [.push_bus_name()](PushDevlink::push_bus_name)\n- [.push_dev_name()](PushDevlink::push_dev_name)\n- [.push_sb_index()](PushDevlink::push_sb_index)\n- [.push_sb_pool_index()](PushDevlink::push_sb_pool_index)\n- [.push_sb_pool_size()](PushDevlink::push_sb_pool_size)\n- [.push_sb_pool_threshold_type()](PushDevlink::push_sb_pool_threshold_type)\n- [.push_index()](PushDevlink::push_index)\n\n"]
    pub fn op_sb_pool_set_do(self) -> OpSbPoolSetDo<'buf> {
        let mut res = OpSbPoolSetDo::new(self);
        res.request
            .do_writeback(res.protocol(), "op-sb-pool-set-do", OpSbPoolSetDo::lookup);
        res
    }
    #[doc = "Get shared buffer port-pool combinations and threshold.\n\nRequest attributes:\n- [.push_bus_name()](PushDevlink::push_bus_name)\n- [.push_dev_name()](PushDevlink::push_dev_name)\n- [.push_index()](PushDevlink::push_index)\n\nReply attributes:\n- [.get_bus_name()](IterableDevlink::get_bus_name)\n- [.get_dev_name()](IterableDevlink::get_dev_name)\n- [.get_port_index()](IterableDevlink::get_port_index)\n- [.get_sb_index()](IterableDevlink::get_sb_index)\n- [.get_sb_pool_index()](IterableDevlink::get_sb_pool_index)\n- [.get_index()](IterableDevlink::get_index)\n\n"]
    pub fn op_sb_port_pool_get_dump(self) -> OpSbPortPoolGetDump<'buf> {
        let mut res = OpSbPortPoolGetDump::new(self);
        res.request.do_writeback(
            res.protocol(),
            "op-sb-port-pool-get-dump",
            OpSbPortPoolGetDump::lookup,
        );
        res
    }
    #[doc = "Get shared buffer port-pool combinations and threshold.\n\nRequest attributes:\n- [.push_bus_name()](PushDevlink::push_bus_name)\n- [.push_dev_name()](PushDevlink::push_dev_name)\n- [.push_port_index()](PushDevlink::push_port_index)\n- [.push_sb_index()](PushDevlink::push_sb_index)\n- [.push_sb_pool_index()](PushDevlink::push_sb_pool_index)\n- [.push_index()](PushDevlink::push_index)\n\nReply attributes:\n- [.get_bus_name()](IterableDevlink::get_bus_name)\n- [.get_dev_name()](IterableDevlink::get_dev_name)\n- [.get_port_index()](IterableDevlink::get_port_index)\n- [.get_sb_index()](IterableDevlink::get_sb_index)\n- [.get_sb_pool_index()](IterableDevlink::get_sb_pool_index)\n- [.get_index()](IterableDevlink::get_index)\n\n"]
    pub fn op_sb_port_pool_get_do(self) -> OpSbPortPoolGetDo<'buf> {
        let mut res = OpSbPortPoolGetDo::new(self);
        res.request.do_writeback(
            res.protocol(),
            "op-sb-port-pool-get-do",
            OpSbPortPoolGetDo::lookup,
        );
        res
    }
    #[doc = "Set shared buffer port-pool combinations and threshold.\n\nFlags: admin-perm\n\nRequest attributes:\n- [.push_bus_name()](PushDevlink::push_bus_name)\n- [.push_dev_name()](PushDevlink::push_dev_name)\n- [.push_port_index()](PushDevlink::push_port_index)\n- [.push_sb_index()](PushDevlink::push_sb_index)\n- [.push_sb_pool_index()](PushDevlink::push_sb_pool_index)\n- [.push_sb_threshold()](PushDevlink::push_sb_threshold)\n- [.push_index()](PushDevlink::push_index)\n\n"]
    pub fn op_sb_port_pool_set_do(self) -> OpSbPortPoolSetDo<'buf> {
        let mut res = OpSbPortPoolSetDo::new(self);
        res.request.do_writeback(
            res.protocol(),
            "op-sb-port-pool-set-do",
            OpSbPortPoolSetDo::lookup,
        );
        res
    }
    #[doc = "Get shared buffer port-TC to pool bindings and threshold.\n\nRequest attributes:\n- [.push_bus_name()](PushDevlink::push_bus_name)\n- [.push_dev_name()](PushDevlink::push_dev_name)\n- [.push_index()](PushDevlink::push_index)\n\nReply attributes:\n- [.get_bus_name()](IterableDevlink::get_bus_name)\n- [.get_dev_name()](IterableDevlink::get_dev_name)\n- [.get_port_index()](IterableDevlink::get_port_index)\n- [.get_sb_index()](IterableDevlink::get_sb_index)\n- [.get_sb_pool_type()](IterableDevlink::get_sb_pool_type)\n- [.get_sb_tc_index()](IterableDevlink::get_sb_tc_index)\n- [.get_index()](IterableDevlink::get_index)\n\n"]
    pub fn op_sb_tc_pool_bind_get_dump(self) -> OpSbTcPoolBindGetDump<'buf> {
        let mut res = OpSbTcPoolBindGetDump::new(self);
        res.request.do_writeback(
            res.protocol(),
            "op-sb-tc-pool-bind-get-dump",
            OpSbTcPoolBindGetDump::lookup,
        );
        res
    }
    #[doc = "Get shared buffer port-TC to pool bindings and threshold.\n\nRequest attributes:\n- [.push_bus_name()](PushDevlink::push_bus_name)\n- [.push_dev_name()](PushDevlink::push_dev_name)\n- [.push_port_index()](PushDevlink::push_port_index)\n- [.push_sb_index()](PushDevlink::push_sb_index)\n- [.push_sb_pool_type()](PushDevlink::push_sb_pool_type)\n- [.push_sb_tc_index()](PushDevlink::push_sb_tc_index)\n- [.push_index()](PushDevlink::push_index)\n\nReply attributes:\n- [.get_bus_name()](IterableDevlink::get_bus_name)\n- [.get_dev_name()](IterableDevlink::get_dev_name)\n- [.get_port_index()](IterableDevlink::get_port_index)\n- [.get_sb_index()](IterableDevlink::get_sb_index)\n- [.get_sb_pool_type()](IterableDevlink::get_sb_pool_type)\n- [.get_sb_tc_index()](IterableDevlink::get_sb_tc_index)\n- [.get_index()](IterableDevlink::get_index)\n\n"]
    pub fn op_sb_tc_pool_bind_get_do(self) -> OpSbTcPoolBindGetDo<'buf> {
        let mut res = OpSbTcPoolBindGetDo::new(self);
        res.request.do_writeback(
            res.protocol(),
            "op-sb-tc-pool-bind-get-do",
            OpSbTcPoolBindGetDo::lookup,
        );
        res
    }
    #[doc = "Set shared buffer port-TC to pool bindings and threshold.\n\nFlags: admin-perm\n\nRequest attributes:\n- [.push_bus_name()](PushDevlink::push_bus_name)\n- [.push_dev_name()](PushDevlink::push_dev_name)\n- [.push_port_index()](PushDevlink::push_port_index)\n- [.push_sb_index()](PushDevlink::push_sb_index)\n- [.push_sb_pool_index()](PushDevlink::push_sb_pool_index)\n- [.push_sb_pool_type()](PushDevlink::push_sb_pool_type)\n- [.push_sb_threshold()](PushDevlink::push_sb_threshold)\n- [.push_sb_tc_index()](PushDevlink::push_sb_tc_index)\n- [.push_index()](PushDevlink::push_index)\n\n"]
    pub fn op_sb_tc_pool_bind_set_do(self) -> OpSbTcPoolBindSetDo<'buf> {
        let mut res = OpSbTcPoolBindSetDo::new(self);
        res.request.do_writeback(
            res.protocol(),
            "op-sb-tc-pool-bind-set-do",
            OpSbTcPoolBindSetDo::lookup,
        );
        res
    }
    #[doc = "Take occupancy snapshot of shared buffer.\n\nFlags: admin-perm\n\nRequest attributes:\n- [.push_bus_name()](PushDevlink::push_bus_name)\n- [.push_dev_name()](PushDevlink::push_dev_name)\n- [.push_sb_index()](PushDevlink::push_sb_index)\n- [.push_index()](PushDevlink::push_index)\n\n"]
    pub fn op_sb_occ_snapshot_do(self) -> OpSbOccSnapshotDo<'buf> {
        let mut res = OpSbOccSnapshotDo::new(self);
        res.request.do_writeback(
            res.protocol(),
            "op-sb-occ-snapshot-do",
            OpSbOccSnapshotDo::lookup,
        );
        res
    }
    #[doc = "Clear occupancy watermarks of shared buffer.\n\nFlags: admin-perm\n\nRequest attributes:\n- [.push_bus_name()](PushDevlink::push_bus_name)\n- [.push_dev_name()](PushDevlink::push_dev_name)\n- [.push_sb_index()](PushDevlink::push_sb_index)\n- [.push_index()](PushDevlink::push_index)\n\n"]
    pub fn op_sb_occ_max_clear_do(self) -> OpSbOccMaxClearDo<'buf> {
        let mut res = OpSbOccMaxClearDo::new(self);
        res.request.do_writeback(
            res.protocol(),
            "op-sb-occ-max-clear-do",
            OpSbOccMaxClearDo::lookup,
        );
        res
    }
    #[doc = "Get eswitch attributes.\n\nFlags: admin-perm\n\nRequest attributes:\n- [.push_bus_name()](PushDevlink::push_bus_name)\n- [.push_dev_name()](PushDevlink::push_dev_name)\n- [.push_index()](PushDevlink::push_index)\n\nReply attributes:\n- [.get_bus_name()](IterableDevlink::get_bus_name)\n- [.get_dev_name()](IterableDevlink::get_dev_name)\n- [.get_eswitch_mode()](IterableDevlink::get_eswitch_mode)\n- [.get_eswitch_inline_mode()](IterableDevlink::get_eswitch_inline_mode)\n- [.get_eswitch_encap_mode()](IterableDevlink::get_eswitch_encap_mode)\n- [.get_index()](IterableDevlink::get_index)\n\n"]
    pub fn op_eswitch_get_do(self) -> OpEswitchGetDo<'buf> {
        let mut res = OpEswitchGetDo::new(self);
        res.request
            .do_writeback(res.protocol(), "op-eswitch-get-do", OpEswitchGetDo::lookup);
        res
    }
    #[doc = "Set eswitch attributes.\n\nFlags: admin-perm\n\nRequest attributes:\n- [.push_bus_name()](PushDevlink::push_bus_name)\n- [.push_dev_name()](PushDevlink::push_dev_name)\n- [.push_eswitch_mode()](PushDevlink::push_eswitch_mode)\n- [.push_eswitch_inline_mode()](PushDevlink::push_eswitch_inline_mode)\n- [.push_eswitch_encap_mode()](PushDevlink::push_eswitch_encap_mode)\n- [.push_index()](PushDevlink::push_index)\n\n"]
    pub fn op_eswitch_set_do(self) -> OpEswitchSetDo<'buf> {
        let mut res = OpEswitchSetDo::new(self);
        res.request
            .do_writeback(res.protocol(), "op-eswitch-set-do", OpEswitchSetDo::lookup);
        res
    }
    #[doc = "Get dpipe table attributes.\n\nRequest attributes:\n- [.push_bus_name()](PushDevlink::push_bus_name)\n- [.push_dev_name()](PushDevlink::push_dev_name)\n- [.push_dpipe_table_name()](PushDevlink::push_dpipe_table_name)\n- [.push_index()](PushDevlink::push_index)\n\nReply attributes:\n- [.get_bus_name()](IterableDevlink::get_bus_name)\n- [.get_dev_name()](IterableDevlink::get_dev_name)\n- [.get_dpipe_tables()](IterableDevlink::get_dpipe_tables)\n- [.get_index()](IterableDevlink::get_index)\n\n"]
    pub fn op_dpipe_table_get_do(self) -> OpDpipeTableGetDo<'buf> {
        let mut res = OpDpipeTableGetDo::new(self);
        res.request.do_writeback(
            res.protocol(),
            "op-dpipe-table-get-do",
            OpDpipeTableGetDo::lookup,
        );
        res
    }
    #[doc = "Get dpipe entries attributes.\n\nRequest attributes:\n- [.push_bus_name()](PushDevlink::push_bus_name)\n- [.push_dev_name()](PushDevlink::push_dev_name)\n- [.push_dpipe_table_name()](PushDevlink::push_dpipe_table_name)\n- [.push_index()](PushDevlink::push_index)\n\nReply attributes:\n- [.get_bus_name()](IterableDevlink::get_bus_name)\n- [.get_dev_name()](IterableDevlink::get_dev_name)\n- [.get_dpipe_entries()](IterableDevlink::get_dpipe_entries)\n- [.get_index()](IterableDevlink::get_index)\n\n"]
    pub fn op_dpipe_entries_get_do(self) -> OpDpipeEntriesGetDo<'buf> {
        let mut res = OpDpipeEntriesGetDo::new(self);
        res.request.do_writeback(
            res.protocol(),
            "op-dpipe-entries-get-do",
            OpDpipeEntriesGetDo::lookup,
        );
        res
    }
    #[doc = "Get dpipe headers attributes.\n\nRequest attributes:\n- [.push_bus_name()](PushDevlink::push_bus_name)\n- [.push_dev_name()](PushDevlink::push_dev_name)\n- [.push_index()](PushDevlink::push_index)\n\nReply attributes:\n- [.get_bus_name()](IterableDevlink::get_bus_name)\n- [.get_dev_name()](IterableDevlink::get_dev_name)\n- [.get_dpipe_headers()](IterableDevlink::get_dpipe_headers)\n- [.get_index()](IterableDevlink::get_index)\n\n"]
    pub fn op_dpipe_headers_get_do(self) -> OpDpipeHeadersGetDo<'buf> {
        let mut res = OpDpipeHeadersGetDo::new(self);
        res.request.do_writeback(
            res.protocol(),
            "op-dpipe-headers-get-do",
            OpDpipeHeadersGetDo::lookup,
        );
        res
    }
    #[doc = "Set dpipe counter attributes.\n\nFlags: admin-perm\n\nRequest attributes:\n- [.push_bus_name()](PushDevlink::push_bus_name)\n- [.push_dev_name()](PushDevlink::push_dev_name)\n- [.push_dpipe_table_name()](PushDevlink::push_dpipe_table_name)\n- [.push_dpipe_table_counters_enabled()](PushDevlink::push_dpipe_table_counters_enabled)\n- [.push_index()](PushDevlink::push_index)\n\n"]
    pub fn op_dpipe_table_counters_set_do(self) -> OpDpipeTableCountersSetDo<'buf> {
        let mut res = OpDpipeTableCountersSetDo::new(self);
        res.request.do_writeback(
            res.protocol(),
            "op-dpipe-table-counters-set-do",
            OpDpipeTableCountersSetDo::lookup,
        );
        res
    }
    #[doc = "Set resource attributes.\n\nFlags: admin-perm\n\nRequest attributes:\n- [.push_bus_name()](PushDevlink::push_bus_name)\n- [.push_dev_name()](PushDevlink::push_dev_name)\n- [.push_resource_id()](PushDevlink::push_resource_id)\n- [.push_resource_size()](PushDevlink::push_resource_size)\n- [.push_index()](PushDevlink::push_index)\n\n"]
    pub fn op_resource_set_do(self) -> OpResourceSetDo<'buf> {
        let mut res = OpResourceSetDo::new(self);
        res.request.do_writeback(
            res.protocol(),
            "op-resource-set-do",
            OpResourceSetDo::lookup,
        );
        res
    }
    #[doc = "Get resource attributes.\n\nRequest attributes:\n- [.push_bus_name()](PushDevlink::push_bus_name)\n- [.push_dev_name()](PushDevlink::push_dev_name)\n- [.push_index()](PushDevlink::push_index)\n- [.push_resource_scope_mask()](PushDevlink::push_resource_scope_mask)\n\nReply attributes:\n- [.get_bus_name()](IterableDevlink::get_bus_name)\n- [.get_dev_name()](IterableDevlink::get_dev_name)\n- [.get_port_index()](IterableDevlink::get_port_index)\n- [.get_resource_list()](IterableDevlink::get_resource_list)\n- [.get_index()](IterableDevlink::get_index)\n\n"]
    pub fn op_resource_dump_dump(self) -> OpResourceDumpDump<'buf> {
        let mut res = OpResourceDumpDump::new(self);
        res.request.do_writeback(
            res.protocol(),
            "op-resource-dump-dump",
            OpResourceDumpDump::lookup,
        );
        res
    }
    #[doc = "Get resource attributes.\n\nRequest attributes:\n- [.push_bus_name()](PushDevlink::push_bus_name)\n- [.push_dev_name()](PushDevlink::push_dev_name)\n- [.push_port_index()](PushDevlink::push_port_index)\n- [.push_index()](PushDevlink::push_index)\n\nReply attributes:\n- [.get_bus_name()](IterableDevlink::get_bus_name)\n- [.get_dev_name()](IterableDevlink::get_dev_name)\n- [.get_port_index()](IterableDevlink::get_port_index)\n- [.get_resource_list()](IterableDevlink::get_resource_list)\n- [.get_index()](IterableDevlink::get_index)\n\n"]
    pub fn op_resource_dump_do(self) -> OpResourceDumpDo<'buf> {
        let mut res = OpResourceDumpDo::new(self);
        res.request.do_writeback(
            res.protocol(),
            "op-resource-dump-do",
            OpResourceDumpDo::lookup,
        );
        res
    }
    #[doc = "Reload devlink.\n\nFlags: admin-perm\n\nRequest attributes:\n- [.push_bus_name()](PushDevlink::push_bus_name)\n- [.push_dev_name()](PushDevlink::push_dev_name)\n- [.push_netns_fd()](PushDevlink::push_netns_fd)\n- [.push_netns_pid()](PushDevlink::push_netns_pid)\n- [.push_netns_id()](PushDevlink::push_netns_id)\n- [.push_reload_action()](PushDevlink::push_reload_action)\n- [.push_reload_limits()](PushDevlink::push_reload_limits)\n- [.push_index()](PushDevlink::push_index)\n\nReply attributes:\n- [.get_bus_name()](IterableDevlink::get_bus_name)\n- [.get_dev_name()](IterableDevlink::get_dev_name)\n- [.get_reload_actions_performed()](IterableDevlink::get_reload_actions_performed)\n- [.get_index()](IterableDevlink::get_index)\n\n"]
    pub fn op_reload_do(self) -> OpReloadDo<'buf> {
        let mut res = OpReloadDo::new(self);
        res.request
            .do_writeback(res.protocol(), "op-reload-do", OpReloadDo::lookup);
        res
    }
    #[doc = "Get param instances.\n\nRequest attributes:\n- [.push_bus_name()](PushDevlink::push_bus_name)\n- [.push_dev_name()](PushDevlink::push_dev_name)\n- [.push_index()](PushDevlink::push_index)\n\nReply attributes:\n- [.get_bus_name()](IterableDevlink::get_bus_name)\n- [.get_dev_name()](IterableDevlink::get_dev_name)\n- [.get_param_name()](IterableDevlink::get_param_name)\n- [.get_index()](IterableDevlink::get_index)\n\n"]
    pub fn op_param_get_dump(self) -> OpParamGetDump<'buf> {
        let mut res = OpParamGetDump::new(self);
        res.request
            .do_writeback(res.protocol(), "op-param-get-dump", OpParamGetDump::lookup);
        res
    }
    #[doc = "Get param instances.\n\nRequest attributes:\n- [.push_bus_name()](PushDevlink::push_bus_name)\n- [.push_dev_name()](PushDevlink::push_dev_name)\n- [.push_param_name()](PushDevlink::push_param_name)\n- [.push_index()](PushDevlink::push_index)\n\nReply attributes:\n- [.get_bus_name()](IterableDevlink::get_bus_name)\n- [.get_dev_name()](IterableDevlink::get_dev_name)\n- [.get_param_name()](IterableDevlink::get_param_name)\n- [.get_index()](IterableDevlink::get_index)\n\n"]
    pub fn op_param_get_do(self) -> OpParamGetDo<'buf> {
        let mut res = OpParamGetDo::new(self);
        res.request
            .do_writeback(res.protocol(), "op-param-get-do", OpParamGetDo::lookup);
        res
    }
    #[doc = "Set param instances.\n\nFlags: admin-perm\n\nRequest attributes:\n- [.push_bus_name()](PushDevlink::push_bus_name)\n- [.push_dev_name()](PushDevlink::push_dev_name)\n- [.push_param_name()](PushDevlink::push_param_name)\n- [.push_param_type()](PushDevlink::push_param_type)\n- [.push_param_value_cmode()](PushDevlink::push_param_value_cmode)\n- [.push_param_reset_default()](PushDevlink::push_param_reset_default)\n- [.push_index()](PushDevlink::push_index)\n\n"]
    pub fn op_param_set_do(self) -> OpParamSetDo<'buf> {
        let mut res = OpParamSetDo::new(self);
        res.request
            .do_writeback(res.protocol(), "op-param-set-do", OpParamSetDo::lookup);
        res
    }
    #[doc = "Get region instances.\n\nRequest attributes:\n- [.push_bus_name()](PushDevlink::push_bus_name)\n- [.push_dev_name()](PushDevlink::push_dev_name)\n- [.push_index()](PushDevlink::push_index)\n\nReply attributes:\n- [.get_bus_name()](IterableDevlink::get_bus_name)\n- [.get_dev_name()](IterableDevlink::get_dev_name)\n- [.get_port_index()](IterableDevlink::get_port_index)\n- [.get_region_name()](IterableDevlink::get_region_name)\n- [.get_index()](IterableDevlink::get_index)\n\n"]
    pub fn op_region_get_dump(self) -> OpRegionGetDump<'buf> {
        let mut res = OpRegionGetDump::new(self);
        res.request.do_writeback(
            res.protocol(),
            "op-region-get-dump",
            OpRegionGetDump::lookup,
        );
        res
    }
    #[doc = "Get region instances.\n\nRequest attributes:\n- [.push_bus_name()](PushDevlink::push_bus_name)\n- [.push_dev_name()](PushDevlink::push_dev_name)\n- [.push_port_index()](PushDevlink::push_port_index)\n- [.push_region_name()](PushDevlink::push_region_name)\n- [.push_index()](PushDevlink::push_index)\n\nReply attributes:\n- [.get_bus_name()](IterableDevlink::get_bus_name)\n- [.get_dev_name()](IterableDevlink::get_dev_name)\n- [.get_port_index()](IterableDevlink::get_port_index)\n- [.get_region_name()](IterableDevlink::get_region_name)\n- [.get_index()](IterableDevlink::get_index)\n\n"]
    pub fn op_region_get_do(self) -> OpRegionGetDo<'buf> {
        let mut res = OpRegionGetDo::new(self);
        res.request
            .do_writeback(res.protocol(), "op-region-get-do", OpRegionGetDo::lookup);
        res
    }
    #[doc = "Create region snapshot.\n\nFlags: admin-perm\n\nRequest attributes:\n- [.push_bus_name()](PushDevlink::push_bus_name)\n- [.push_dev_name()](PushDevlink::push_dev_name)\n- [.push_port_index()](PushDevlink::push_port_index)\n- [.push_region_name()](PushDevlink::push_region_name)\n- [.push_region_snapshot_id()](PushDevlink::push_region_snapshot_id)\n- [.push_index()](PushDevlink::push_index)\n\nReply attributes:\n- [.get_bus_name()](IterableDevlink::get_bus_name)\n- [.get_dev_name()](IterableDevlink::get_dev_name)\n- [.get_port_index()](IterableDevlink::get_port_index)\n- [.get_region_name()](IterableDevlink::get_region_name)\n- [.get_region_snapshot_id()](IterableDevlink::get_region_snapshot_id)\n- [.get_index()](IterableDevlink::get_index)\n\n"]
    pub fn op_region_new_do(self) -> OpRegionNewDo<'buf> {
        let mut res = OpRegionNewDo::new(self);
        res.request
            .do_writeback(res.protocol(), "op-region-new-do", OpRegionNewDo::lookup);
        res
    }
    #[doc = "Delete region snapshot.\n\nFlags: admin-perm\n\nRequest attributes:\n- [.push_bus_name()](PushDevlink::push_bus_name)\n- [.push_dev_name()](PushDevlink::push_dev_name)\n- [.push_port_index()](PushDevlink::push_port_index)\n- [.push_region_name()](PushDevlink::push_region_name)\n- [.push_region_snapshot_id()](PushDevlink::push_region_snapshot_id)\n- [.push_index()](PushDevlink::push_index)\n\n"]
    pub fn op_region_del_do(self) -> OpRegionDelDo<'buf> {
        let mut res = OpRegionDelDo::new(self);
        res.request
            .do_writeback(res.protocol(), "op-region-del-do", OpRegionDelDo::lookup);
        res
    }
    #[doc = "Read region data.\n\nFlags: admin-perm\n\nRequest attributes:\n- [.push_bus_name()](PushDevlink::push_bus_name)\n- [.push_dev_name()](PushDevlink::push_dev_name)\n- [.push_port_index()](PushDevlink::push_port_index)\n- [.push_region_name()](PushDevlink::push_region_name)\n- [.push_region_snapshot_id()](PushDevlink::push_region_snapshot_id)\n- [.push_region_chunk_addr()](PushDevlink::push_region_chunk_addr)\n- [.push_region_chunk_len()](PushDevlink::push_region_chunk_len)\n- [.push_region_direct()](PushDevlink::push_region_direct)\n- [.push_index()](PushDevlink::push_index)\n\nReply attributes:\n- [.get_bus_name()](IterableDevlink::get_bus_name)\n- [.get_dev_name()](IterableDevlink::get_dev_name)\n- [.get_port_index()](IterableDevlink::get_port_index)\n- [.get_region_name()](IterableDevlink::get_region_name)\n- [.get_index()](IterableDevlink::get_index)\n\n"]
    pub fn op_region_read_dump(self) -> OpRegionReadDump<'buf> {
        let mut res = OpRegionReadDump::new(self);
        res.request.do_writeback(
            res.protocol(),
            "op-region-read-dump",
            OpRegionReadDump::lookup,
        );
        res
    }
    #[doc = "Get port param instances.\n\nReply attributes:\n- [.get_bus_name()](IterableDevlink::get_bus_name)\n- [.get_dev_name()](IterableDevlink::get_dev_name)\n- [.get_port_index()](IterableDevlink::get_port_index)\n- [.get_index()](IterableDevlink::get_index)\n\n"]
    pub fn op_port_param_get_dump(self) -> OpPortParamGetDump<'buf> {
        let mut res = OpPortParamGetDump::new(self);
        res.request.do_writeback(
            res.protocol(),
            "op-port-param-get-dump",
            OpPortParamGetDump::lookup,
        );
        res
    }
    #[doc = "Get port param instances.\n\nRequest attributes:\n- [.push_bus_name()](PushDevlink::push_bus_name)\n- [.push_dev_name()](PushDevlink::push_dev_name)\n- [.push_port_index()](PushDevlink::push_port_index)\n- [.push_index()](PushDevlink::push_index)\n\nReply attributes:\n- [.get_bus_name()](IterableDevlink::get_bus_name)\n- [.get_dev_name()](IterableDevlink::get_dev_name)\n- [.get_port_index()](IterableDevlink::get_port_index)\n- [.get_index()](IterableDevlink::get_index)\n\n"]
    pub fn op_port_param_get_do(self) -> OpPortParamGetDo<'buf> {
        let mut res = OpPortParamGetDo::new(self);
        res.request.do_writeback(
            res.protocol(),
            "op-port-param-get-do",
            OpPortParamGetDo::lookup,
        );
        res
    }
    #[doc = "Set port param instances.\n\nFlags: admin-perm\n\nRequest attributes:\n- [.push_bus_name()](PushDevlink::push_bus_name)\n- [.push_dev_name()](PushDevlink::push_dev_name)\n- [.push_port_index()](PushDevlink::push_port_index)\n- [.push_index()](PushDevlink::push_index)\n\n"]
    pub fn op_port_param_set_do(self) -> OpPortParamSetDo<'buf> {
        let mut res = OpPortParamSetDo::new(self);
        res.request.do_writeback(
            res.protocol(),
            "op-port-param-set-do",
            OpPortParamSetDo::lookup,
        );
        res
    }
    #[doc = "Get device information, like driver name, hardware and firmware versions\netc.\n\nReply attributes:\n- [.get_bus_name()](IterableDevlink::get_bus_name)\n- [.get_dev_name()](IterableDevlink::get_dev_name)\n- [.get_info_driver_name()](IterableDevlink::get_info_driver_name)\n- [.get_info_serial_number()](IterableDevlink::get_info_serial_number)\n- [.get_info_version_fixed()](IterableDevlink::get_info_version_fixed)\n- [.get_info_version_running()](IterableDevlink::get_info_version_running)\n- [.get_info_version_stored()](IterableDevlink::get_info_version_stored)\n- [.get_info_board_serial_number()](IterableDevlink::get_info_board_serial_number)\n- [.get_index()](IterableDevlink::get_index)\n\n"]
    pub fn op_info_get_dump(self) -> OpInfoGetDump<'buf> {
        let mut res = OpInfoGetDump::new(self);
        res.request
            .do_writeback(res.protocol(), "op-info-get-dump", OpInfoGetDump::lookup);
        res
    }
    #[doc = "Get device information, like driver name, hardware and firmware versions\netc.\n\nRequest attributes:\n- [.push_bus_name()](PushDevlink::push_bus_name)\n- [.push_dev_name()](PushDevlink::push_dev_name)\n- [.push_index()](PushDevlink::push_index)\n\nReply attributes:\n- [.get_bus_name()](IterableDevlink::get_bus_name)\n- [.get_dev_name()](IterableDevlink::get_dev_name)\n- [.get_info_driver_name()](IterableDevlink::get_info_driver_name)\n- [.get_info_serial_number()](IterableDevlink::get_info_serial_number)\n- [.get_info_version_fixed()](IterableDevlink::get_info_version_fixed)\n- [.get_info_version_running()](IterableDevlink::get_info_version_running)\n- [.get_info_version_stored()](IterableDevlink::get_info_version_stored)\n- [.get_info_board_serial_number()](IterableDevlink::get_info_board_serial_number)\n- [.get_index()](IterableDevlink::get_index)\n\n"]
    pub fn op_info_get_do(self) -> OpInfoGetDo<'buf> {
        let mut res = OpInfoGetDo::new(self);
        res.request
            .do_writeback(res.protocol(), "op-info-get-do", OpInfoGetDo::lookup);
        res
    }
    #[doc = "Get health reporter instances.\n\nRequest attributes:\n- [.push_bus_name()](PushDevlink::push_bus_name)\n- [.push_dev_name()](PushDevlink::push_dev_name)\n- [.push_port_index()](PushDevlink::push_port_index)\n- [.push_index()](PushDevlink::push_index)\n\nReply attributes:\n- [.get_bus_name()](IterableDevlink::get_bus_name)\n- [.get_dev_name()](IterableDevlink::get_dev_name)\n- [.get_port_index()](IterableDevlink::get_port_index)\n- [.get_health_reporter_name()](IterableDevlink::get_health_reporter_name)\n- [.get_index()](IterableDevlink::get_index)\n\n"]
    pub fn op_health_reporter_get_dump(self) -> OpHealthReporterGetDump<'buf> {
        let mut res = OpHealthReporterGetDump::new(self);
        res.request.do_writeback(
            res.protocol(),
            "op-health-reporter-get-dump",
            OpHealthReporterGetDump::lookup,
        );
        res
    }
    #[doc = "Get health reporter instances.\n\nRequest attributes:\n- [.push_bus_name()](PushDevlink::push_bus_name)\n- [.push_dev_name()](PushDevlink::push_dev_name)\n- [.push_port_index()](PushDevlink::push_port_index)\n- [.push_health_reporter_name()](PushDevlink::push_health_reporter_name)\n- [.push_index()](PushDevlink::push_index)\n\nReply attributes:\n- [.get_bus_name()](IterableDevlink::get_bus_name)\n- [.get_dev_name()](IterableDevlink::get_dev_name)\n- [.get_port_index()](IterableDevlink::get_port_index)\n- [.get_health_reporter_name()](IterableDevlink::get_health_reporter_name)\n- [.get_index()](IterableDevlink::get_index)\n\n"]
    pub fn op_health_reporter_get_do(self) -> OpHealthReporterGetDo<'buf> {
        let mut res = OpHealthReporterGetDo::new(self);
        res.request.do_writeback(
            res.protocol(),
            "op-health-reporter-get-do",
            OpHealthReporterGetDo::lookup,
        );
        res
    }
    #[doc = "Set health reporter instances.\n\nFlags: admin-perm\n\nRequest attributes:\n- [.push_bus_name()](PushDevlink::push_bus_name)\n- [.push_dev_name()](PushDevlink::push_dev_name)\n- [.push_port_index()](PushDevlink::push_port_index)\n- [.push_health_reporter_name()](PushDevlink::push_health_reporter_name)\n- [.push_health_reporter_graceful_period()](PushDevlink::push_health_reporter_graceful_period)\n- [.push_health_reporter_auto_recover()](PushDevlink::push_health_reporter_auto_recover)\n- [.push_health_reporter_auto_dump()](PushDevlink::push_health_reporter_auto_dump)\n- [.push_health_reporter_burst_period()](PushDevlink::push_health_reporter_burst_period)\n- [.push_index()](PushDevlink::push_index)\n\n"]
    pub fn op_health_reporter_set_do(self) -> OpHealthReporterSetDo<'buf> {
        let mut res = OpHealthReporterSetDo::new(self);
        res.request.do_writeback(
            res.protocol(),
            "op-health-reporter-set-do",
            OpHealthReporterSetDo::lookup,
        );
        res
    }
    #[doc = "Recover health reporter instances.\n\nFlags: admin-perm\n\nRequest attributes:\n- [.push_bus_name()](PushDevlink::push_bus_name)\n- [.push_dev_name()](PushDevlink::push_dev_name)\n- [.push_port_index()](PushDevlink::push_port_index)\n- [.push_health_reporter_name()](PushDevlink::push_health_reporter_name)\n- [.push_index()](PushDevlink::push_index)\n\n"]
    pub fn op_health_reporter_recover_do(self) -> OpHealthReporterRecoverDo<'buf> {
        let mut res = OpHealthReporterRecoverDo::new(self);
        res.request.do_writeback(
            res.protocol(),
            "op-health-reporter-recover-do",
            OpHealthReporterRecoverDo::lookup,
        );
        res
    }
    #[doc = "Diagnose health reporter instances.\n\nFlags: admin-perm\n\nRequest attributes:\n- [.push_bus_name()](PushDevlink::push_bus_name)\n- [.push_dev_name()](PushDevlink::push_dev_name)\n- [.push_port_index()](PushDevlink::push_port_index)\n- [.push_health_reporter_name()](PushDevlink::push_health_reporter_name)\n- [.push_index()](PushDevlink::push_index)\n\n"]
    pub fn op_health_reporter_diagnose_do(self) -> OpHealthReporterDiagnoseDo<'buf> {
        let mut res = OpHealthReporterDiagnoseDo::new(self);
        res.request.do_writeback(
            res.protocol(),
            "op-health-reporter-diagnose-do",
            OpHealthReporterDiagnoseDo::lookup,
        );
        res
    }
    #[doc = "Dump health reporter instances.\n\nFlags: admin-perm\n\nRequest attributes:\n- [.push_bus_name()](PushDevlink::push_bus_name)\n- [.push_dev_name()](PushDevlink::push_dev_name)\n- [.push_port_index()](PushDevlink::push_port_index)\n- [.push_health_reporter_name()](PushDevlink::push_health_reporter_name)\n- [.push_index()](PushDevlink::push_index)\n\nReply attributes:\n- [.get_fmsg()](IterableDevlink::get_fmsg)\n\n"]
    pub fn op_health_reporter_dump_get_dump(self) -> OpHealthReporterDumpGetDump<'buf> {
        let mut res = OpHealthReporterDumpGetDump::new(self);
        res.request.do_writeback(
            res.protocol(),
            "op-health-reporter-dump-get-dump",
            OpHealthReporterDumpGetDump::lookup,
        );
        res
    }
    #[doc = "Clear dump of health reporter instances.\n\nFlags: admin-perm\n\nRequest attributes:\n- [.push_bus_name()](PushDevlink::push_bus_name)\n- [.push_dev_name()](PushDevlink::push_dev_name)\n- [.push_port_index()](PushDevlink::push_port_index)\n- [.push_health_reporter_name()](PushDevlink::push_health_reporter_name)\n- [.push_index()](PushDevlink::push_index)\n\n"]
    pub fn op_health_reporter_dump_clear_do(self) -> OpHealthReporterDumpClearDo<'buf> {
        let mut res = OpHealthReporterDumpClearDo::new(self);
        res.request.do_writeback(
            res.protocol(),
            "op-health-reporter-dump-clear-do",
            OpHealthReporterDumpClearDo::lookup,
        );
        res
    }
    #[doc = "Flash update devlink instances.\n\nFlags: admin-perm\n\nRequest attributes:\n- [.push_bus_name()](PushDevlink::push_bus_name)\n- [.push_dev_name()](PushDevlink::push_dev_name)\n- [.push_flash_update_file_name()](PushDevlink::push_flash_update_file_name)\n- [.push_flash_update_component()](PushDevlink::push_flash_update_component)\n- [.push_flash_update_overwrite_mask()](PushDevlink::push_flash_update_overwrite_mask)\n- [.push_index()](PushDevlink::push_index)\n\n"]
    pub fn op_flash_update_do(self) -> OpFlashUpdateDo<'buf> {
        let mut res = OpFlashUpdateDo::new(self);
        res.request.do_writeback(
            res.protocol(),
            "op-flash-update-do",
            OpFlashUpdateDo::lookup,
        );
        res
    }
    #[doc = "Get trap instances.\n\nRequest attributes:\n- [.push_bus_name()](PushDevlink::push_bus_name)\n- [.push_dev_name()](PushDevlink::push_dev_name)\n- [.push_index()](PushDevlink::push_index)\n\nReply attributes:\n- [.get_bus_name()](IterableDevlink::get_bus_name)\n- [.get_dev_name()](IterableDevlink::get_dev_name)\n- [.get_trap_name()](IterableDevlink::get_trap_name)\n- [.get_index()](IterableDevlink::get_index)\n\n"]
    pub fn op_trap_get_dump(self) -> OpTrapGetDump<'buf> {
        let mut res = OpTrapGetDump::new(self);
        res.request
            .do_writeback(res.protocol(), "op-trap-get-dump", OpTrapGetDump::lookup);
        res
    }
    #[doc = "Get trap instances.\n\nRequest attributes:\n- [.push_bus_name()](PushDevlink::push_bus_name)\n- [.push_dev_name()](PushDevlink::push_dev_name)\n- [.push_trap_name()](PushDevlink::push_trap_name)\n- [.push_index()](PushDevlink::push_index)\n\nReply attributes:\n- [.get_bus_name()](IterableDevlink::get_bus_name)\n- [.get_dev_name()](IterableDevlink::get_dev_name)\n- [.get_trap_name()](IterableDevlink::get_trap_name)\n- [.get_index()](IterableDevlink::get_index)\n\n"]
    pub fn op_trap_get_do(self) -> OpTrapGetDo<'buf> {
        let mut res = OpTrapGetDo::new(self);
        res.request
            .do_writeback(res.protocol(), "op-trap-get-do", OpTrapGetDo::lookup);
        res
    }
    #[doc = "Set trap instances.\n\nFlags: admin-perm\n\nRequest attributes:\n- [.push_bus_name()](PushDevlink::push_bus_name)\n- [.push_dev_name()](PushDevlink::push_dev_name)\n- [.push_trap_name()](PushDevlink::push_trap_name)\n- [.push_trap_action()](PushDevlink::push_trap_action)\n- [.push_index()](PushDevlink::push_index)\n\n"]
    pub fn op_trap_set_do(self) -> OpTrapSetDo<'buf> {
        let mut res = OpTrapSetDo::new(self);
        res.request
            .do_writeback(res.protocol(), "op-trap-set-do", OpTrapSetDo::lookup);
        res
    }
    #[doc = "Get trap group instances.\n\nRequest attributes:\n- [.push_bus_name()](PushDevlink::push_bus_name)\n- [.push_dev_name()](PushDevlink::push_dev_name)\n- [.push_index()](PushDevlink::push_index)\n\nReply attributes:\n- [.get_bus_name()](IterableDevlink::get_bus_name)\n- [.get_dev_name()](IterableDevlink::get_dev_name)\n- [.get_trap_group_name()](IterableDevlink::get_trap_group_name)\n- [.get_index()](IterableDevlink::get_index)\n\n"]
    pub fn op_trap_group_get_dump(self) -> OpTrapGroupGetDump<'buf> {
        let mut res = OpTrapGroupGetDump::new(self);
        res.request.do_writeback(
            res.protocol(),
            "op-trap-group-get-dump",
            OpTrapGroupGetDump::lookup,
        );
        res
    }
    #[doc = "Get trap group instances.\n\nRequest attributes:\n- [.push_bus_name()](PushDevlink::push_bus_name)\n- [.push_dev_name()](PushDevlink::push_dev_name)\n- [.push_trap_group_name()](PushDevlink::push_trap_group_name)\n- [.push_index()](PushDevlink::push_index)\n\nReply attributes:\n- [.get_bus_name()](IterableDevlink::get_bus_name)\n- [.get_dev_name()](IterableDevlink::get_dev_name)\n- [.get_trap_group_name()](IterableDevlink::get_trap_group_name)\n- [.get_index()](IterableDevlink::get_index)\n\n"]
    pub fn op_trap_group_get_do(self) -> OpTrapGroupGetDo<'buf> {
        let mut res = OpTrapGroupGetDo::new(self);
        res.request.do_writeback(
            res.protocol(),
            "op-trap-group-get-do",
            OpTrapGroupGetDo::lookup,
        );
        res
    }
    #[doc = "Set trap group instances.\n\nFlags: admin-perm\n\nRequest attributes:\n- [.push_bus_name()](PushDevlink::push_bus_name)\n- [.push_dev_name()](PushDevlink::push_dev_name)\n- [.push_trap_action()](PushDevlink::push_trap_action)\n- [.push_trap_group_name()](PushDevlink::push_trap_group_name)\n- [.push_trap_policer_id()](PushDevlink::push_trap_policer_id)\n- [.push_index()](PushDevlink::push_index)\n\n"]
    pub fn op_trap_group_set_do(self) -> OpTrapGroupSetDo<'buf> {
        let mut res = OpTrapGroupSetDo::new(self);
        res.request.do_writeback(
            res.protocol(),
            "op-trap-group-set-do",
            OpTrapGroupSetDo::lookup,
        );
        res
    }
    #[doc = "Get trap policer instances.\n\nRequest attributes:\n- [.push_bus_name()](PushDevlink::push_bus_name)\n- [.push_dev_name()](PushDevlink::push_dev_name)\n- [.push_index()](PushDevlink::push_index)\n\nReply attributes:\n- [.get_bus_name()](IterableDevlink::get_bus_name)\n- [.get_dev_name()](IterableDevlink::get_dev_name)\n- [.get_trap_policer_id()](IterableDevlink::get_trap_policer_id)\n- [.get_index()](IterableDevlink::get_index)\n\n"]
    pub fn op_trap_policer_get_dump(self) -> OpTrapPolicerGetDump<'buf> {
        let mut res = OpTrapPolicerGetDump::new(self);
        res.request.do_writeback(
            res.protocol(),
            "op-trap-policer-get-dump",
            OpTrapPolicerGetDump::lookup,
        );
        res
    }
    #[doc = "Get trap policer instances.\n\nRequest attributes:\n- [.push_bus_name()](PushDevlink::push_bus_name)\n- [.push_dev_name()](PushDevlink::push_dev_name)\n- [.push_trap_policer_id()](PushDevlink::push_trap_policer_id)\n- [.push_index()](PushDevlink::push_index)\n\nReply attributes:\n- [.get_bus_name()](IterableDevlink::get_bus_name)\n- [.get_dev_name()](IterableDevlink::get_dev_name)\n- [.get_trap_policer_id()](IterableDevlink::get_trap_policer_id)\n- [.get_index()](IterableDevlink::get_index)\n\n"]
    pub fn op_trap_policer_get_do(self) -> OpTrapPolicerGetDo<'buf> {
        let mut res = OpTrapPolicerGetDo::new(self);
        res.request.do_writeback(
            res.protocol(),
            "op-trap-policer-get-do",
            OpTrapPolicerGetDo::lookup,
        );
        res
    }
    #[doc = "Get trap policer instances.\n\nFlags: admin-perm\n\nRequest attributes:\n- [.push_bus_name()](PushDevlink::push_bus_name)\n- [.push_dev_name()](PushDevlink::push_dev_name)\n- [.push_trap_policer_id()](PushDevlink::push_trap_policer_id)\n- [.push_trap_policer_rate()](PushDevlink::push_trap_policer_rate)\n- [.push_trap_policer_burst()](PushDevlink::push_trap_policer_burst)\n- [.push_index()](PushDevlink::push_index)\n\n"]
    pub fn op_trap_policer_set_do(self) -> OpTrapPolicerSetDo<'buf> {
        let mut res = OpTrapPolicerSetDo::new(self);
        res.request.do_writeback(
            res.protocol(),
            "op-trap-policer-set-do",
            OpTrapPolicerSetDo::lookup,
        );
        res
    }
    #[doc = "Test health reporter instances.\n\nFlags: admin-perm\n\nRequest attributes:\n- [.push_bus_name()](PushDevlink::push_bus_name)\n- [.push_dev_name()](PushDevlink::push_dev_name)\n- [.push_port_index()](PushDevlink::push_port_index)\n- [.push_health_reporter_name()](PushDevlink::push_health_reporter_name)\n- [.push_index()](PushDevlink::push_index)\n\n"]
    pub fn op_health_reporter_test_do(self) -> OpHealthReporterTestDo<'buf> {
        let mut res = OpHealthReporterTestDo::new(self);
        res.request.do_writeback(
            res.protocol(),
            "op-health-reporter-test-do",
            OpHealthReporterTestDo::lookup,
        );
        res
    }
    #[doc = "Get rate instances.\n\nRequest attributes:\n- [.push_bus_name()](PushDevlink::push_bus_name)\n- [.push_dev_name()](PushDevlink::push_dev_name)\n- [.push_index()](PushDevlink::push_index)\n\nReply attributes:\n- [.get_bus_name()](IterableDevlink::get_bus_name)\n- [.get_dev_name()](IterableDevlink::get_dev_name)\n- [.get_port_index()](IterableDevlink::get_port_index)\n- [.get_rate_node_name()](IterableDevlink::get_rate_node_name)\n- [.get_index()](IterableDevlink::get_index)\n\n"]
    pub fn op_rate_get_dump(self) -> OpRateGetDump<'buf> {
        let mut res = OpRateGetDump::new(self);
        res.request
            .do_writeback(res.protocol(), "op-rate-get-dump", OpRateGetDump::lookup);
        res
    }
    #[doc = "Get rate instances.\n\nRequest attributes:\n- [.push_bus_name()](PushDevlink::push_bus_name)\n- [.push_dev_name()](PushDevlink::push_dev_name)\n- [.push_port_index()](PushDevlink::push_port_index)\n- [.push_rate_node_name()](PushDevlink::push_rate_node_name)\n- [.push_index()](PushDevlink::push_index)\n\nReply attributes:\n- [.get_bus_name()](IterableDevlink::get_bus_name)\n- [.get_dev_name()](IterableDevlink::get_dev_name)\n- [.get_port_index()](IterableDevlink::get_port_index)\n- [.get_rate_node_name()](IterableDevlink::get_rate_node_name)\n- [.get_index()](IterableDevlink::get_index)\n\n"]
    pub fn op_rate_get_do(self) -> OpRateGetDo<'buf> {
        let mut res = OpRateGetDo::new(self);
        res.request
            .do_writeback(res.protocol(), "op-rate-get-do", OpRateGetDo::lookup);
        res
    }
    #[doc = "Set rate instances.\n\nFlags: admin-perm\n\nRequest attributes:\n- [.push_bus_name()](PushDevlink::push_bus_name)\n- [.push_dev_name()](PushDevlink::push_dev_name)\n- [.push_rate_tx_share()](PushDevlink::push_rate_tx_share)\n- [.push_rate_tx_max()](PushDevlink::push_rate_tx_max)\n- [.push_rate_node_name()](PushDevlink::push_rate_node_name)\n- [.push_rate_parent_node_name()](PushDevlink::push_rate_parent_node_name)\n- [.push_rate_tx_priority()](PushDevlink::push_rate_tx_priority)\n- [.push_rate_tx_weight()](PushDevlink::push_rate_tx_weight)\n- [.nested_rate_tc_bws()](PushDevlink::nested_rate_tc_bws)\n- [.push_index()](PushDevlink::push_index)\n\n"]
    pub fn op_rate_set_do(self) -> OpRateSetDo<'buf> {
        let mut res = OpRateSetDo::new(self);
        res.request
            .do_writeback(res.protocol(), "op-rate-set-do", OpRateSetDo::lookup);
        res
    }
    #[doc = "Create rate instances.\n\nFlags: admin-perm\n\nRequest attributes:\n- [.push_bus_name()](PushDevlink::push_bus_name)\n- [.push_dev_name()](PushDevlink::push_dev_name)\n- [.push_rate_tx_share()](PushDevlink::push_rate_tx_share)\n- [.push_rate_tx_max()](PushDevlink::push_rate_tx_max)\n- [.push_rate_node_name()](PushDevlink::push_rate_node_name)\n- [.push_rate_parent_node_name()](PushDevlink::push_rate_parent_node_name)\n- [.push_rate_tx_priority()](PushDevlink::push_rate_tx_priority)\n- [.push_rate_tx_weight()](PushDevlink::push_rate_tx_weight)\n- [.nested_rate_tc_bws()](PushDevlink::nested_rate_tc_bws)\n- [.push_index()](PushDevlink::push_index)\n\n"]
    pub fn op_rate_new_do(self) -> OpRateNewDo<'buf> {
        let mut res = OpRateNewDo::new(self);
        res.request
            .do_writeback(res.protocol(), "op-rate-new-do", OpRateNewDo::lookup);
        res
    }
    #[doc = "Delete rate instances.\n\nFlags: admin-perm\n\nRequest attributes:\n- [.push_bus_name()](PushDevlink::push_bus_name)\n- [.push_dev_name()](PushDevlink::push_dev_name)\n- [.push_rate_node_name()](PushDevlink::push_rate_node_name)\n- [.push_index()](PushDevlink::push_index)\n\n"]
    pub fn op_rate_del_do(self) -> OpRateDelDo<'buf> {
        let mut res = OpRateDelDo::new(self);
        res.request
            .do_writeback(res.protocol(), "op-rate-del-do", OpRateDelDo::lookup);
        res
    }
    #[doc = "Get line card instances.\n\nRequest attributes:\n- [.push_bus_name()](PushDevlink::push_bus_name)\n- [.push_dev_name()](PushDevlink::push_dev_name)\n- [.push_index()](PushDevlink::push_index)\n\nReply attributes:\n- [.get_bus_name()](IterableDevlink::get_bus_name)\n- [.get_dev_name()](IterableDevlink::get_dev_name)\n- [.get_linecard_index()](IterableDevlink::get_linecard_index)\n- [.get_index()](IterableDevlink::get_index)\n\n"]
    pub fn op_linecard_get_dump(self) -> OpLinecardGetDump<'buf> {
        let mut res = OpLinecardGetDump::new(self);
        res.request.do_writeback(
            res.protocol(),
            "op-linecard-get-dump",
            OpLinecardGetDump::lookup,
        );
        res
    }
    #[doc = "Get line card instances.\n\nRequest attributes:\n- [.push_bus_name()](PushDevlink::push_bus_name)\n- [.push_dev_name()](PushDevlink::push_dev_name)\n- [.push_linecard_index()](PushDevlink::push_linecard_index)\n- [.push_index()](PushDevlink::push_index)\n\nReply attributes:\n- [.get_bus_name()](IterableDevlink::get_bus_name)\n- [.get_dev_name()](IterableDevlink::get_dev_name)\n- [.get_linecard_index()](IterableDevlink::get_linecard_index)\n- [.get_index()](IterableDevlink::get_index)\n\n"]
    pub fn op_linecard_get_do(self) -> OpLinecardGetDo<'buf> {
        let mut res = OpLinecardGetDo::new(self);
        res.request.do_writeback(
            res.protocol(),
            "op-linecard-get-do",
            OpLinecardGetDo::lookup,
        );
        res
    }
    #[doc = "Set line card instances.\n\nFlags: admin-perm\n\nRequest attributes:\n- [.push_bus_name()](PushDevlink::push_bus_name)\n- [.push_dev_name()](PushDevlink::push_dev_name)\n- [.push_linecard_index()](PushDevlink::push_linecard_index)\n- [.push_linecard_type()](PushDevlink::push_linecard_type)\n- [.push_index()](PushDevlink::push_index)\n\n"]
    pub fn op_linecard_set_do(self) -> OpLinecardSetDo<'buf> {
        let mut res = OpLinecardSetDo::new(self);
        res.request.do_writeback(
            res.protocol(),
            "op-linecard-set-do",
            OpLinecardSetDo::lookup,
        );
        res
    }
    #[doc = "Get device selftest instances.\n\nReply attributes:\n- [.get_bus_name()](IterableDevlink::get_bus_name)\n- [.get_dev_name()](IterableDevlink::get_dev_name)\n- [.get_index()](IterableDevlink::get_index)\n\n"]
    pub fn op_selftests_get_dump(self) -> OpSelftestsGetDump<'buf> {
        let mut res = OpSelftestsGetDump::new(self);
        res.request.do_writeback(
            res.protocol(),
            "op-selftests-get-dump",
            OpSelftestsGetDump::lookup,
        );
        res
    }
    #[doc = "Get device selftest instances.\n\nRequest attributes:\n- [.push_bus_name()](PushDevlink::push_bus_name)\n- [.push_dev_name()](PushDevlink::push_dev_name)\n- [.push_index()](PushDevlink::push_index)\n\nReply attributes:\n- [.get_bus_name()](IterableDevlink::get_bus_name)\n- [.get_dev_name()](IterableDevlink::get_dev_name)\n- [.get_index()](IterableDevlink::get_index)\n\n"]
    pub fn op_selftests_get_do(self) -> OpSelftestsGetDo<'buf> {
        let mut res = OpSelftestsGetDo::new(self);
        res.request.do_writeback(
            res.protocol(),
            "op-selftests-get-do",
            OpSelftestsGetDo::lookup,
        );
        res
    }
    #[doc = "Run device selftest instances.\n\nFlags: admin-perm\n\nRequest attributes:\n- [.push_bus_name()](PushDevlink::push_bus_name)\n- [.push_dev_name()](PushDevlink::push_dev_name)\n- [.nested_selftests()](PushDevlink::nested_selftests)\n- [.push_index()](PushDevlink::push_index)\n\n"]
    pub fn op_selftests_run_do(self) -> OpSelftestsRunDo<'buf> {
        let mut res = OpSelftestsRunDo::new(self);
        res.request.do_writeback(
            res.protocol(),
            "op-selftests-run-do",
            OpSelftestsRunDo::lookup,
        );
        res
    }
    #[doc = "Set notification messages socket filter.\n\nRequest attributes:\n- [.push_bus_name()](PushDevlink::push_bus_name)\n- [.push_dev_name()](PushDevlink::push_dev_name)\n- [.push_port_index()](PushDevlink::push_port_index)\n- [.push_index()](PushDevlink::push_index)\n\n"]
    pub fn op_notify_filter_set_do(self) -> OpNotifyFilterSetDo<'buf> {
        let mut res = OpNotifyFilterSetDo::new(self);
        res.request.do_writeback(
            res.protocol(),
            "op-notify-filter-set-do",
            OpNotifyFilterSetDo::lookup,
        );
        res
    }
}
#[cfg(test)]
mod generated_tests {
    use super::*;
    #[test]
    fn tests() {
        let _ = IterableDevlink::get_bus_name;
        let _ = IterableDevlink::get_dev_name;
        let _ = IterableDevlink::get_dev_stats;
        let _ = IterableDevlink::get_dpipe_entries;
        let _ = IterableDevlink::get_dpipe_headers;
        let _ = IterableDevlink::get_dpipe_tables;
        let _ = IterableDevlink::get_eswitch_encap_mode;
        let _ = IterableDevlink::get_eswitch_inline_mode;
        let _ = IterableDevlink::get_eswitch_mode;
        let _ = IterableDevlink::get_fmsg;
        let _ = IterableDevlink::get_health_reporter_name;
        let _ = IterableDevlink::get_index;
        let _ = IterableDevlink::get_info_board_serial_number;
        let _ = IterableDevlink::get_info_driver_name;
        let _ = IterableDevlink::get_info_serial_number;
        let _ = IterableDevlink::get_info_version_fixed;
        let _ = IterableDevlink::get_info_version_running;
        let _ = IterableDevlink::get_info_version_stored;
        let _ = IterableDevlink::get_linecard_index;
        let _ = IterableDevlink::get_param_name;
        let _ = IterableDevlink::get_port_index;
        let _ = IterableDevlink::get_rate_node_name;
        let _ = IterableDevlink::get_region_name;
        let _ = IterableDevlink::get_region_snapshot_id;
        let _ = IterableDevlink::get_reload_actions_performed;
        let _ = IterableDevlink::get_reload_failed;
        let _ = IterableDevlink::get_resource_list;
        let _ = IterableDevlink::get_sb_index;
        let _ = IterableDevlink::get_sb_pool_index;
        let _ = IterableDevlink::get_sb_pool_type;
        let _ = IterableDevlink::get_sb_tc_index;
        let _ = IterableDevlink::get_trap_group_name;
        let _ = IterableDevlink::get_trap_name;
        let _ = IterableDevlink::get_trap_policer_id;
        let _ = PushDevlink::<&mut Vec<u8>>::nested_port_function;
        let _ = PushDevlink::<&mut Vec<u8>>::nested_rate_tc_bws;
        let _ = PushDevlink::<&mut Vec<u8>>::nested_selftests;
        let _ = PushDevlink::<&mut Vec<u8>>::push_bus_name;
        let _ = PushDevlink::<&mut Vec<u8>>::push_dev_name;
        let _ = PushDevlink::<&mut Vec<u8>>::push_dpipe_table_counters_enabled;
        let _ = PushDevlink::<&mut Vec<u8>>::push_dpipe_table_name;
        let _ = PushDevlink::<&mut Vec<u8>>::push_eswitch_encap_mode;
        let _ = PushDevlink::<&mut Vec<u8>>::push_eswitch_inline_mode;
        let _ = PushDevlink::<&mut Vec<u8>>::push_eswitch_mode;
        let _ = PushDevlink::<&mut Vec<u8>>::push_flash_update_component;
        let _ = PushDevlink::<&mut Vec<u8>>::push_flash_update_file_name;
        let _ = PushDevlink::<&mut Vec<u8>>::push_flash_update_overwrite_mask;
        let _ = PushDevlink::<&mut Vec<u8>>::push_health_reporter_auto_dump;
        let _ = PushDevlink::<&mut Vec<u8>>::push_health_reporter_auto_recover;
        let _ = PushDevlink::<&mut Vec<u8>>::push_health_reporter_burst_period;
        let _ = PushDevlink::<&mut Vec<u8>>::push_health_reporter_graceful_period;
        let _ = PushDevlink::<&mut Vec<u8>>::push_health_reporter_name;
        let _ = PushDevlink::<&mut Vec<u8>>::push_index;
        let _ = PushDevlink::<&mut Vec<u8>>::push_linecard_index;
        let _ = PushDevlink::<&mut Vec<u8>>::push_linecard_type;
        let _ = PushDevlink::<&mut Vec<u8>>::push_netns_fd;
        let _ = PushDevlink::<&mut Vec<u8>>::push_netns_id;
        let _ = PushDevlink::<&mut Vec<u8>>::push_netns_pid;
        let _ = PushDevlink::<&mut Vec<u8>>::push_param_name;
        let _ = PushDevlink::<&mut Vec<u8>>::push_param_reset_default;
        let _ = PushDevlink::<&mut Vec<u8>>::push_param_type;
        let _ = PushDevlink::<&mut Vec<u8>>::push_param_value_cmode;
        let _ = PushDevlink::<&mut Vec<u8>>::push_port_controller_number;
        let _ = PushDevlink::<&mut Vec<u8>>::push_port_flavour;
        let _ = PushDevlink::<&mut Vec<u8>>::push_port_index;
        let _ = PushDevlink::<&mut Vec<u8>>::push_port_pci_pf_number;
        let _ = PushDevlink::<&mut Vec<u8>>::push_port_pci_sf_number;
        let _ = PushDevlink::<&mut Vec<u8>>::push_port_split_count;
        let _ = PushDevlink::<&mut Vec<u8>>::push_port_type;
        let _ = PushDevlink::<&mut Vec<u8>>::push_rate_node_name;
        let _ = PushDevlink::<&mut Vec<u8>>::push_rate_parent_node_name;
        let _ = PushDevlink::<&mut Vec<u8>>::push_rate_tx_max;
        let _ = PushDevlink::<&mut Vec<u8>>::push_rate_tx_priority;
        let _ = PushDevlink::<&mut Vec<u8>>::push_rate_tx_share;
        let _ = PushDevlink::<&mut Vec<u8>>::push_rate_tx_weight;
        let _ = PushDevlink::<&mut Vec<u8>>::push_region_chunk_addr;
        let _ = PushDevlink::<&mut Vec<u8>>::push_region_chunk_len;
        let _ = PushDevlink::<&mut Vec<u8>>::push_region_direct;
        let _ = PushDevlink::<&mut Vec<u8>>::push_region_name;
        let _ = PushDevlink::<&mut Vec<u8>>::push_region_snapshot_id;
        let _ = PushDevlink::<&mut Vec<u8>>::push_reload_action;
        let _ = PushDevlink::<&mut Vec<u8>>::push_reload_limits;
        let _ = PushDevlink::<&mut Vec<u8>>::push_resource_id;
        let _ = PushDevlink::<&mut Vec<u8>>::push_resource_scope_mask;
        let _ = PushDevlink::<&mut Vec<u8>>::push_resource_size;
        let _ = PushDevlink::<&mut Vec<u8>>::push_sb_index;
        let _ = PushDevlink::<&mut Vec<u8>>::push_sb_pool_index;
        let _ = PushDevlink::<&mut Vec<u8>>::push_sb_pool_size;
        let _ = PushDevlink::<&mut Vec<u8>>::push_sb_pool_threshold_type;
        let _ = PushDevlink::<&mut Vec<u8>>::push_sb_pool_type;
        let _ = PushDevlink::<&mut Vec<u8>>::push_sb_tc_index;
        let _ = PushDevlink::<&mut Vec<u8>>::push_sb_threshold;
        let _ = PushDevlink::<&mut Vec<u8>>::push_trap_action;
        let _ = PushDevlink::<&mut Vec<u8>>::push_trap_group_name;
        let _ = PushDevlink::<&mut Vec<u8>>::push_trap_name;
        let _ = PushDevlink::<&mut Vec<u8>>::push_trap_policer_burst;
        let _ = PushDevlink::<&mut Vec<u8>>::push_trap_policer_id;
        let _ = PushDevlink::<&mut Vec<u8>>::push_trap_policer_rate;
    }
}
