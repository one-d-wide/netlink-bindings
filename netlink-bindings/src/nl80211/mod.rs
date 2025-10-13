#![doc = "Netlink API for 802.11 wireless devices"]
#![allow(clippy::all)]
#![allow(unused_imports)]
#![allow(unused_assignments)]
#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(irrefutable_let_patterns)]
#![allow(unreachable_code)]
#![allow(unreachable_patterns)]
use crate::builtin::{PushBuiltinBitfield32, PushBuiltinNfgenmsg, PushDummy, PushNlmsghdr};
use crate::{
    consts,
    traits::{NetlinkRequest, Protocol},
    utils::*,
};
pub const PROTONAME: &CStr = c"nl80211";
#[doc = "Enum - defines an integer enumeration, with values for each entry incrementing by 1, (e.g. 0, 1, 2, 3)"]
#[derive(Debug, Clone, Copy)]
pub enum Commands {
    Unspec = 0,
    GetWiphy = 1,
    SetWiphy = 2,
    NewWiphy = 3,
    DelWiphy = 4,
    GetInterface = 5,
    SetInterface = 6,
    NewInterface = 7,
    DelInterface = 8,
    GetKey = 9,
    SetKey = 10,
    NewKey = 11,
    DelKey = 12,
    GetBeacon = 13,
    SetBeacon = 14,
    NewBeacon = 15,
    DelBeacon = 16,
    GetStation = 17,
    SetStation = 18,
    NewStation = 19,
    DelStation = 20,
    GetMpath = 21,
    SetMpath = 22,
    NewMpath = 23,
    DelMpath = 24,
    SetBss = 25,
    SetReg = 26,
    ReqSetReg = 27,
    GetMeshConfig = 28,
    SetMeshConfig = 29,
    SetMgmtExtraIe = 30,
    GetReg = 31,
    GetScan = 32,
    TriggerScan = 33,
    NewScanResults = 34,
    ScanAborted = 35,
    RegChange = 36,
    Authenticate = 37,
    Associate = 38,
    Deauthenticate = 39,
    Disassociate = 40,
    MichaelMicFailure = 41,
    RegBeaconHint = 42,
    JoinIbss = 43,
    LeaveIbss = 44,
    Testmode = 45,
    Connect = 46,
    Roam = 47,
    Disconnect = 48,
    SetWiphyNetns = 49,
    GetSurvey = 50,
    NewSurveyResults = 51,
    SetPmksa = 52,
    DelPmksa = 53,
    FlushPmksa = 54,
    RemainOnChannel = 55,
    CancelRemainOnChannel = 56,
    SetTxBitrateMask = 57,
    RegisterAction = 58,
    Action = 59,
    ActionTxStatus = 60,
    SetPowerSave = 61,
    GetPowerSave = 62,
    SetCqm = 63,
    NotifyCqm = 64,
    SetChannel = 65,
    SetWdsPeer = 66,
    FrameWaitCancel = 67,
    JoinMesh = 68,
    LeaveMesh = 69,
    UnprotDeauthenticate = 70,
    UnprotDisassociate = 71,
    NewPeerCandidate = 72,
    GetWowlan = 73,
    SetWowlan = 74,
    StartSchedScan = 75,
    StopSchedScan = 76,
    SchedScanResults = 77,
    SchedScanStopped = 78,
    SetRekeyOffload = 79,
    PmksaCandidate = 80,
    TdlsOper = 81,
    TdlsMgmt = 82,
    UnexpectedFrame = 83,
    ProbeClient = 84,
    RegisterBeacons = 85,
    Unexpected4AddrFrame = 86,
    SetNoackMap = 87,
    ChSwitchNotify = 88,
    StartP2pDevice = 89,
    StopP2pDevice = 90,
    ConnFailed = 91,
    SetMcastRate = 92,
    SetMacAcl = 93,
    RadarDetect = 94,
    GetProtocolFeatures = 95,
    UpdateFtIes = 96,
    FtEvent = 97,
    CritProtocolStart = 98,
    CritProtocolStop = 99,
    GetCoalesce = 100,
    SetCoalesce = 101,
    ChannelSwitch = 102,
    Vendor = 103,
    SetQosMap = 104,
    AddTxTs = 105,
    DelTxTs = 106,
    GetMpp = 107,
    JoinOcb = 108,
    LeaveOcb = 109,
    ChSwitchStartedNotify = 110,
    TdlsChannelSwitch = 111,
    TdlsCancelChannelSwitch = 112,
    WiphyRegChange = 113,
    AbortScan = 114,
    StartNan = 115,
    StopNan = 116,
    AddNanFunction = 117,
    DelNanFunction = 118,
    ChangeNanConfig = 119,
    NanMatch = 120,
    SetMulticastToUnicast = 121,
    UpdateConnectParams = 122,
    SetPmk = 123,
    DelPmk = 124,
    PortAuthorized = 125,
    ReloadRegdb = 126,
    ExternalAuth = 127,
    StaOpmodeChanged = 128,
    ControlPortFrame = 129,
    GetFtmResponderStats = 130,
    PeerMeasurementStart = 131,
    PeerMeasurementResult = 132,
    PeerMeasurementComplete = 133,
    NotifyRadar = 134,
    UpdateOweInfo = 135,
    ProbeMeshLink = 136,
    SetTidConfig = 137,
    UnprotBeacon = 138,
    ControlPortFrameTxStatus = 139,
    SetSarSpecs = 140,
    ObssColorCollision = 141,
    ColorChangeRequest = 142,
    ColorChangeStarted = 143,
    ColorChangeAborted = 144,
    ColorChangeCompleted = 145,
    SetFilsAad = 146,
    AssocComeback = 147,
    AddLink = 148,
    RemoveLink = 149,
    AddLinkSta = 150,
    ModifyLinkSta = 151,
    RemoveLinkSta = 152,
    SetHwTimestamp = 153,
    LinksRemoved = 154,
    SetTidToLinkMapping = 155,
}
impl Commands {
    pub fn from_value(value: u64) -> Option<Self> {
        Some(match value {
            0 => Self::Unspec,
            1 => Self::GetWiphy,
            2 => Self::SetWiphy,
            3 => Self::NewWiphy,
            4 => Self::DelWiphy,
            5 => Self::GetInterface,
            6 => Self::SetInterface,
            7 => Self::NewInterface,
            8 => Self::DelInterface,
            9 => Self::GetKey,
            10 => Self::SetKey,
            11 => Self::NewKey,
            12 => Self::DelKey,
            13 => Self::GetBeacon,
            14 => Self::SetBeacon,
            15 => Self::NewBeacon,
            16 => Self::DelBeacon,
            17 => Self::GetStation,
            18 => Self::SetStation,
            19 => Self::NewStation,
            20 => Self::DelStation,
            21 => Self::GetMpath,
            22 => Self::SetMpath,
            23 => Self::NewMpath,
            24 => Self::DelMpath,
            25 => Self::SetBss,
            26 => Self::SetReg,
            27 => Self::ReqSetReg,
            28 => Self::GetMeshConfig,
            29 => Self::SetMeshConfig,
            30 => Self::SetMgmtExtraIe,
            31 => Self::GetReg,
            32 => Self::GetScan,
            33 => Self::TriggerScan,
            34 => Self::NewScanResults,
            35 => Self::ScanAborted,
            36 => Self::RegChange,
            37 => Self::Authenticate,
            38 => Self::Associate,
            39 => Self::Deauthenticate,
            40 => Self::Disassociate,
            41 => Self::MichaelMicFailure,
            42 => Self::RegBeaconHint,
            43 => Self::JoinIbss,
            44 => Self::LeaveIbss,
            45 => Self::Testmode,
            46 => Self::Connect,
            47 => Self::Roam,
            48 => Self::Disconnect,
            49 => Self::SetWiphyNetns,
            50 => Self::GetSurvey,
            51 => Self::NewSurveyResults,
            52 => Self::SetPmksa,
            53 => Self::DelPmksa,
            54 => Self::FlushPmksa,
            55 => Self::RemainOnChannel,
            56 => Self::CancelRemainOnChannel,
            57 => Self::SetTxBitrateMask,
            58 => Self::RegisterAction,
            59 => Self::Action,
            60 => Self::ActionTxStatus,
            61 => Self::SetPowerSave,
            62 => Self::GetPowerSave,
            63 => Self::SetCqm,
            64 => Self::NotifyCqm,
            65 => Self::SetChannel,
            66 => Self::SetWdsPeer,
            67 => Self::FrameWaitCancel,
            68 => Self::JoinMesh,
            69 => Self::LeaveMesh,
            70 => Self::UnprotDeauthenticate,
            71 => Self::UnprotDisassociate,
            72 => Self::NewPeerCandidate,
            73 => Self::GetWowlan,
            74 => Self::SetWowlan,
            75 => Self::StartSchedScan,
            76 => Self::StopSchedScan,
            77 => Self::SchedScanResults,
            78 => Self::SchedScanStopped,
            79 => Self::SetRekeyOffload,
            80 => Self::PmksaCandidate,
            81 => Self::TdlsOper,
            82 => Self::TdlsMgmt,
            83 => Self::UnexpectedFrame,
            84 => Self::ProbeClient,
            85 => Self::RegisterBeacons,
            86 => Self::Unexpected4AddrFrame,
            87 => Self::SetNoackMap,
            88 => Self::ChSwitchNotify,
            89 => Self::StartP2pDevice,
            90 => Self::StopP2pDevice,
            91 => Self::ConnFailed,
            92 => Self::SetMcastRate,
            93 => Self::SetMacAcl,
            94 => Self::RadarDetect,
            95 => Self::GetProtocolFeatures,
            96 => Self::UpdateFtIes,
            97 => Self::FtEvent,
            98 => Self::CritProtocolStart,
            99 => Self::CritProtocolStop,
            100 => Self::GetCoalesce,
            101 => Self::SetCoalesce,
            102 => Self::ChannelSwitch,
            103 => Self::Vendor,
            104 => Self::SetQosMap,
            105 => Self::AddTxTs,
            106 => Self::DelTxTs,
            107 => Self::GetMpp,
            108 => Self::JoinOcb,
            109 => Self::LeaveOcb,
            110 => Self::ChSwitchStartedNotify,
            111 => Self::TdlsChannelSwitch,
            112 => Self::TdlsCancelChannelSwitch,
            113 => Self::WiphyRegChange,
            114 => Self::AbortScan,
            115 => Self::StartNan,
            116 => Self::StopNan,
            117 => Self::AddNanFunction,
            118 => Self::DelNanFunction,
            119 => Self::ChangeNanConfig,
            120 => Self::NanMatch,
            121 => Self::SetMulticastToUnicast,
            122 => Self::UpdateConnectParams,
            123 => Self::SetPmk,
            124 => Self::DelPmk,
            125 => Self::PortAuthorized,
            126 => Self::ReloadRegdb,
            127 => Self::ExternalAuth,
            128 => Self::StaOpmodeChanged,
            129 => Self::ControlPortFrame,
            130 => Self::GetFtmResponderStats,
            131 => Self::PeerMeasurementStart,
            132 => Self::PeerMeasurementResult,
            133 => Self::PeerMeasurementComplete,
            134 => Self::NotifyRadar,
            135 => Self::UpdateOweInfo,
            136 => Self::ProbeMeshLink,
            137 => Self::SetTidConfig,
            138 => Self::UnprotBeacon,
            139 => Self::ControlPortFrameTxStatus,
            140 => Self::SetSarSpecs,
            141 => Self::ObssColorCollision,
            142 => Self::ColorChangeRequest,
            143 => Self::ColorChangeStarted,
            144 => Self::ColorChangeAborted,
            145 => Self::ColorChangeCompleted,
            146 => Self::SetFilsAad,
            147 => Self::AssocComeback,
            148 => Self::AddLink,
            149 => Self::RemoveLink,
            150 => Self::AddLinkSta,
            151 => Self::ModifyLinkSta,
            152 => Self::RemoveLinkSta,
            153 => Self::SetHwTimestamp,
            154 => Self::LinksRemoved,
            155 => Self::SetTidToLinkMapping,
            _ => return None,
        })
    }
}
#[doc = "Flags - defines an integer enumeration, with values for each entry occupying a bit, starting from bit 0, (e.g. 1, 2, 4, 8)"]
#[derive(Debug, Clone, Copy)]
pub enum FeatureFlags {
    SkTxStatus = 1 << 0,
    HtIbss = 1 << 1,
    InactivityTimer = 1 << 2,
    CellBaseRegHints = 1 << 3,
    P2pDeviceNeedsChannel = 1 << 4,
    Sae = 1 << 5,
    LowPriorityScan = 1 << 6,
    ScanFlush = 1 << 7,
    ApScan = 1 << 8,
    VifTxpower = 1 << 9,
    NeedObssScan = 1 << 10,
    P2pGoCtwin = 1 << 11,
    P2pGoOppps = 1 << 12,
    Reserved = 1 << 13,
    AdvertiseChanLimits = 1 << 14,
    FullApClientState = 1 << 15,
    UserspaceMpm = 1 << 16,
    ActiveMonitor = 1 << 17,
    ApModeChanWidthChange = 1 << 18,
    DsParamSetIeInProbes = 1 << 19,
    WfaTpcIeInProbes = 1 << 20,
    Quiet = 1 << 21,
    TxPowerInsertion = 1 << 22,
    AcktoEstimation = 1 << 23,
    StaticSmps = 1 << 24,
    DynamicSmps = 1 << 25,
    SupportsWmmAdmission = 1 << 26,
    MacOnCreate = 1 << 27,
    TdlsChannelSwitch = 1 << 28,
    ScanRandomMacAddr = 1 << 29,
    SchedScanRandomMacAddr = 1 << 30,
    NoRandomMacAddr = 1 << 31,
}
impl FeatureFlags {
    pub fn from_value(value: u64) -> Option<Self> {
        Some(match value {
            n if n == 1 << 0 => Self::SkTxStatus,
            n if n == 1 << 1 => Self::HtIbss,
            n if n == 1 << 2 => Self::InactivityTimer,
            n if n == 1 << 3 => Self::CellBaseRegHints,
            n if n == 1 << 4 => Self::P2pDeviceNeedsChannel,
            n if n == 1 << 5 => Self::Sae,
            n if n == 1 << 6 => Self::LowPriorityScan,
            n if n == 1 << 7 => Self::ScanFlush,
            n if n == 1 << 8 => Self::ApScan,
            n if n == 1 << 9 => Self::VifTxpower,
            n if n == 1 << 10 => Self::NeedObssScan,
            n if n == 1 << 11 => Self::P2pGoCtwin,
            n if n == 1 << 12 => Self::P2pGoOppps,
            n if n == 1 << 13 => Self::Reserved,
            n if n == 1 << 14 => Self::AdvertiseChanLimits,
            n if n == 1 << 15 => Self::FullApClientState,
            n if n == 1 << 16 => Self::UserspaceMpm,
            n if n == 1 << 17 => Self::ActiveMonitor,
            n if n == 1 << 18 => Self::ApModeChanWidthChange,
            n if n == 1 << 19 => Self::DsParamSetIeInProbes,
            n if n == 1 << 20 => Self::WfaTpcIeInProbes,
            n if n == 1 << 21 => Self::Quiet,
            n if n == 1 << 22 => Self::TxPowerInsertion,
            n if n == 1 << 23 => Self::AcktoEstimation,
            n if n == 1 << 24 => Self::StaticSmps,
            n if n == 1 << 25 => Self::DynamicSmps,
            n if n == 1 << 26 => Self::SupportsWmmAdmission,
            n if n == 1 << 27 => Self::MacOnCreate,
            n if n == 1 << 28 => Self::TdlsChannelSwitch,
            n if n == 1 << 29 => Self::ScanRandomMacAddr,
            n if n == 1 << 30 => Self::SchedScanRandomMacAddr,
            n if n == 1 << 31 => Self::NoRandomMacAddr,
            _ => return None,
        })
    }
}
#[doc = "Enum - defines an integer enumeration, with values for each entry incrementing by 1, (e.g. 0, 1, 2, 3)"]
#[derive(Debug, Clone, Copy)]
pub enum ChannelType {
    NoHt = 0,
    Ht20 = 1,
    Ht40minus = 2,
    Ht40plus = 3,
}
impl ChannelType {
    pub fn from_value(value: u64) -> Option<Self> {
        Some(match value {
            0 => Self::NoHt,
            1 => Self::Ht20,
            2 => Self::Ht40minus,
            3 => Self::Ht40plus,
            _ => return None,
        })
    }
}
#[doc = "Flags - defines an integer enumeration, with values for each entry occupying a bit, starting from bit 0, (e.g. 1, 2, 4, 8)"]
#[derive(Debug, Clone, Copy)]
pub enum ProtocolFeatures {
    SplitWiphyDump = 1 << 0,
}
impl ProtocolFeatures {
    pub fn from_value(value: u64) -> Option<Self> {
        Some(match value {
            n if n == 1 << 0 => Self::SplitWiphyDump,
            _ => return None,
        })
    }
}
#[derive(Clone)]
pub enum Nl80211Attrs<'a> {
    Wiphy(u32),
    WiphyName(&'a CStr),
    Ifindex(u32),
    Ifname(&'a CStr),
    Iftype(u32),
    Mac(&'a [u8]),
    KeyData(&'a [u8]),
    KeyIdx(u8),
    KeyCipher(u32),
    KeySeq(&'a [u8]),
    KeyDefault(()),
    BeaconInterval(u32),
    DtimPeriod(u32),
    BeaconHead(&'a [u8]),
    BeaconTail(&'a [u8]),
    StaAid(u16),
    StaFlags(&'a [u8]),
    StaListenInterval(u16),
    StaSupportedRates(&'a [u8]),
    StaVlan(u32),
    StaInfo(&'a [u8]),
    WiphyBands(IterableWiphyBands<'a>),
    MntrFlags(&'a [u8]),
    MeshId(&'a [u8]),
    StaPlinkAction(u8),
    MpathNextHop(&'a [u8]),
    MpathInfo(&'a [u8]),
    BssCtsProt(u8),
    BssShortPreamble(u8),
    BssShortSlotTime(u8),
    HtCapability(&'a [u8]),
    SupportedIftypes(IterableSupportedIftypes<'a>),
    RegAlpha2(&'a [u8]),
    RegRules(&'a [u8]),
    MeshConfig(&'a [u8]),
    BssBasicRates(&'a [u8]),
    WiphyTxqParams(&'a [u8]),
    WiphyFreq(u32),
    #[doc = "Associated type: \"ChannelType\" (enum)"]
    WiphyChannelType(u32),
    KeyDefaultMgmt(()),
    MgmtSubtype(u8),
    Ie(&'a [u8]),
    MaxNumScanSsids(u8),
    ScanFrequencies(&'a [u8]),
    ScanSsids(&'a [u8]),
    Generation(u32),
    Bss(&'a [u8]),
    RegInitiator(u8),
    RegType(u8),
    Frame(&'a [u8]),
    Ssid(&'a [u8]),
    AuthType(u32),
    ReasonCode(u16),
    KeyType(u32),
    MaxScanIeLen(u16),
    CipherSuites(&'a [u8]),
    FreqBefore(&'a [u8]),
    FreqAfter(&'a [u8]),
    FreqFixed(()),
    WiphyRetryShort(u8),
    WiphyRetryLong(u8),
    WiphyFragThreshold(u32),
    WiphyRtsThreshold(u32),
    TimedOut(()),
    UseMfp(u32),
    StaFlags2(PushStaFlagUpdate),
    ControlPort(()),
    Testdata(&'a [u8]),
    Privacy(()),
    DisconnectedByAp(()),
    StatusCode(u16),
    CipherSuitesPairwise(&'a [u8]),
    CipherSuiteGroup(u32),
    WpaVersions(u32),
    AkmSuites(&'a [u8]),
    ReqIe(&'a [u8]),
    RespIe(&'a [u8]),
    PrevBssid(&'a [u8]),
    Key(&'a [u8]),
    Keys(&'a [u8]),
    Pid(u32),
    _4addr(u8),
    SurveyInfo(&'a [u8]),
    Pmkid(&'a [u8]),
    MaxNumPmkids(u8),
    Duration(u32),
    Cookie(u64),
    WiphyCoverageClass(u8),
    TxRates(&'a [u8]),
    FrameMatch(&'a [u8]),
    Ack(()),
    PsState(u32),
    Cqm(&'a [u8]),
    LocalStateChange(()),
    ApIsolate(u8),
    WiphyTxPowerSetting(u32),
    WiphyTxPowerLevel(u32),
    TxFrameTypes(IterableIftypeAttrs<'a>),
    RxFrameTypes(IterableIftypeAttrs<'a>),
    FrameType(u16),
    ControlPortEthertype(()),
    ControlPortNoEncrypt(()),
    SupportIbssRsn(()),
    WiphyAntennaTx(u32),
    WiphyAntennaRx(u32),
    McastRate(u32),
    OffchannelTxOk(()),
    BssHtOpmode(u16),
    KeyDefaultTypes(&'a [u8]),
    MaxRemainOnChannelDuration(u32),
    MeshSetup(&'a [u8]),
    WiphyAntennaAvailTx(u32),
    WiphyAntennaAvailRx(u32),
    SupportMeshAuth(()),
    StaPlinkState(u8),
    WowlanTriggers(&'a [u8]),
    WowlanTriggersSupported(IterableWowlanTriggersAttrs<'a>),
    SchedScanInterval(u32),
    InterfaceCombinations(&'a [u8]),
    SoftwareIftypes(IterableSupportedIftypes<'a>),
    RekeyData(&'a [u8]),
    MaxNumSchedScanSsids(u8),
    MaxSchedScanIeLen(u16),
    ScanSuppRates(&'a [u8]),
    HiddenSsid(u32),
    IeProbeResp(&'a [u8]),
    IeAssocResp(&'a [u8]),
    StaWme(&'a [u8]),
    SupportApUapsd(()),
    RoamSupport(()),
    SchedScanMatch(&'a [u8]),
    MaxMatchSets(u8),
    PmksaCandidate(&'a [u8]),
    TxNoCckRate(()),
    TdlsAction(u8),
    TdlsDialogToken(u8),
    TdlsOperation(u8),
    TdlsSupport(()),
    TdlsExternalSetup(()),
    DeviceApSme(u32),
    DontWaitForAck(()),
    #[doc = "Associated type: \"FeatureFlags\" (1 bit per enumeration)"]
    FeatureFlags(u32),
    ProbeRespOffload(u32),
    ProbeResp(&'a [u8]),
    DfsRegion(u8),
    DisableHt(()),
    HtCapabilityMask(&'a [u8]),
    NoackMap(u16),
    InactivityTimeout(u16),
    RxSignalDbm(u32),
    BgScanPeriod(u16),
    Wdev(u64),
    UserRegHintType(u32),
    ConnFailedReason(u32),
    AuthData(&'a [u8]),
    VhtCapability(&'a [u8]),
    ScanFlags(u32),
    ChannelWidth(u32),
    CenterFreq1(u32),
    CenterFreq2(u32),
    P2pCtwindow(u8),
    P2pOppps(u8),
    LocalMeshPowerMode(u32),
    AclPolicy(u32),
    MacAddrs(&'a [u8]),
    MacAclMax(u32),
    RadarEvent(u32),
    ExtCapa(&'a [u8]),
    ExtCapaMask(&'a [u8]),
    StaCapability(u16),
    StaExtCapability(&'a [u8]),
    #[doc = "Associated type: \"ProtocolFeatures\" (enum)"]
    ProtocolFeatures(u32),
    SplitWiphyDump(()),
    DisableVht(()),
    VhtCapabilityMask(&'a [u8]),
    Mdid(u16),
    IeRic(&'a [u8]),
    CritProtId(u16),
    MaxCritProtDuration(u16),
    PeerAid(u16),
    CoalesceRule(&'a [u8]),
    ChSwitchCount(u32),
    ChSwitchBlockTx(()),
    CsaIes(&'a [u8]),
    CntdwnOffsBeacon(&'a [u8]),
    CntdwnOffsPresp(&'a [u8]),
    RxmgmtFlags(&'a [u8]),
    StaSupportedChannels(&'a [u8]),
    StaSupportedOperClasses(&'a [u8]),
    HandleDfs(()),
    Support5Mhz(()),
    Support10Mhz(()),
    OpmodeNotif(u8),
    VendorId(u32),
    VendorSubcmd(u32),
    VendorData(&'a [u8]),
    VendorEvents(&'a [u8]),
    QosMap(&'a [u8]),
    MacHint(&'a [u8]),
    WiphyFreqHint(u32),
    MaxApAssocSta(u32),
    TdlsPeerCapability(u32),
    SocketOwner(()),
    CsaCOffsetsTx(&'a [u8]),
    MaxCsaCounters(u8),
    TdlsInitiator(()),
    UseRrm(()),
    WiphyDynAck(()),
    Tsid(u8),
    UserPrio(u8),
    AdmittedTime(u16),
    SmpsMode(u8),
    OperClass(u8),
    MacMask(&'a [u8]),
    WiphySelfManagedReg(()),
    ExtFeatures(&'a [u8]),
    SurveyRadioStats(&'a [u8]),
    NetnsFd(u32),
    SchedScanDelay(u32),
    RegIndoor(()),
    MaxNumSchedScanPlans(u32),
    MaxScanPlanInterval(u32),
    MaxScanPlanIterations(u32),
    SchedScanPlans(&'a [u8]),
    Pbss(()),
    BssSelect(&'a [u8]),
    StaSupportP2pPs(u8),
    Pad(&'a [u8]),
    IftypeExtCapa(&'a [u8]),
    MuMimoGroupData(&'a [u8]),
    MuMimoFollowMacAddr(&'a [u8]),
    ScanStartTimeTsf(u64),
    ScanStartTimeTsfBssid(&'a [u8]),
    MeasurementDuration(u16),
    MeasurementDurationMandatory(()),
    MeshPeerAid(u16),
    NanMasterPref(u8),
    Bands(u32),
    NanFunc(&'a [u8]),
    NanMatch(&'a [u8]),
    FilsKek(&'a [u8]),
    FilsNonces(&'a [u8]),
    MulticastToUnicastEnabled(()),
    Bssid(&'a [u8]),
    SchedScanRelativeRssi(i8),
    SchedScanRssiAdjust(&'a [u8]),
    TimeoutReason(u32),
    FilsErpUsername(&'a [u8]),
    FilsErpRealm(&'a [u8]),
    FilsErpNextSeqNum(u16),
    FilsErpRrk(&'a [u8]),
    FilsCacheId(&'a [u8]),
    Pmk(&'a [u8]),
    SchedScanMulti(()),
    SchedScanMaxReqs(u32),
    Want1x4wayHs(()),
    Pmkr0Name(&'a [u8]),
    PortAuthorized(&'a [u8]),
    ExternalAuthAction(u32),
    ExternalAuthSupport(()),
    Nss(u8),
    AckSignal(i32),
    ControlPortOverNl80211(()),
    TxqStats(IterableTxqStatsAttrs<'a>),
    TxqLimit(u32),
    TxqMemoryLimit(u32),
    TxqQuantum(u32),
    HeCapability(&'a [u8]),
    FtmResponder(&'a [u8]),
    FtmResponderStats(&'a [u8]),
    Timeout(u32),
    PeerMeasurements(&'a [u8]),
    AirtimeWeight(u16),
    StaTxPowerSetting(u8),
    StaTxPower(i16),
    SaePassword(&'a [u8]),
    TwtResponder(()),
    HeObssPd(&'a [u8]),
    WiphyEdmgChannels(u8),
    WiphyEdmgBwConfig(u8),
    VlanId(u16),
    HeBssColor(&'a [u8]),
    IftypeAkmSuites(&'a [u8]),
    TidConfig(&'a [u8]),
    ControlPortNoPreauth(()),
    PmkLifetime(u32),
    PmkReauthThreshold(u8),
    ReceiveMulticast(()),
    WiphyFreqOffset(u32),
    CenterFreq1Offset(u32),
    ScanFreqKhz(&'a [u8]),
    He6ghzCapability(&'a [u8]),
    FilsDiscovery(&'a [u8]),
    UnsolBcastProbeResp(&'a [u8]),
    S1gCapability(&'a [u8]),
    S1gCapabilityMask(&'a [u8]),
    SaePwe(u8),
    ReconnectRequested(&'a [u8]),
    SarSpec(&'a [u8]),
    DisableHe(()),
    ObssColorBitmap(u64),
    ColorChangeCount(u8),
    ColorChangeColor(u8),
    ColorChangeElems(&'a [u8]),
    MbssidConfig(&'a [u8]),
    MbssidElems(&'a [u8]),
    RadarBackground(()),
    ApSettingsFlags(u32),
    EhtCapability(&'a [u8]),
    DisableEht(()),
    MloLinks(&'a [u8]),
    MloLinkId(u8),
    MldAddr(&'a [u8]),
    MloSupport(()),
    MaxNumAkmSuites(&'a [u8]),
    EmlCapability(u16),
    MldCapaAndOps(u16),
    TxHwTimestamp(u64),
    RxHwTimestamp(u64),
    TdBitmap(&'a [u8]),
    PunctBitmap(u32),
    MaxHwTimestampPeers(u16),
    HwTimestampEnabled(()),
    EmaRnrElems(&'a [u8]),
    MloLinkDisabled(()),
    BssDumpIncludeUseData(()),
    MloTtlmDlink(u16),
    MloTtlmUlink(u16),
    AssocSppAmsdu(()),
    WiphyRadios(&'a [u8]),
    WiphyInterfaceCombinations(&'a [u8]),
    VifRadioMask(u32),
}
impl<'a> IterableNl80211Attrs<'a> {
    pub fn get_wiphy(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::Wiphy(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "Wiphy",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_wiphy_name(&self) -> Result<&'a CStr, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::WiphyName(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "WiphyName",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_ifindex(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::Ifindex(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "Ifindex",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_ifname(&self) -> Result<&'a CStr, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::Ifname(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "Ifname",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_iftype(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::Iftype(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "Iftype",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_mac(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::Mac(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "Mac",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_key_data(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::KeyData(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "KeyData",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_key_idx(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::KeyIdx(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "KeyIdx",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_key_cipher(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::KeyCipher(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "KeyCipher",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_key_seq(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::KeySeq(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "KeySeq",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_key_default(&self) -> Result<(), ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::KeyDefault(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "KeyDefault",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_beacon_interval(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::BeaconInterval(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "BeaconInterval",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_dtim_period(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::DtimPeriod(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "DtimPeriod",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_beacon_head(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::BeaconHead(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "BeaconHead",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_beacon_tail(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::BeaconTail(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "BeaconTail",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_sta_aid(&self) -> Result<u16, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::StaAid(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "StaAid",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_sta_flags(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::StaFlags(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "StaFlags",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_sta_listen_interval(&self) -> Result<u16, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::StaListenInterval(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "StaListenInterval",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_sta_supported_rates(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::StaSupportedRates(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "StaSupportedRates",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_sta_vlan(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::StaVlan(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "StaVlan",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_sta_info(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::StaInfo(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "StaInfo",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_wiphy_bands(&self) -> Result<IterableWiphyBands<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::WiphyBands(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "WiphyBands",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_mntr_flags(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::MntrFlags(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "MntrFlags",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_mesh_id(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::MeshId(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "MeshId",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_sta_plink_action(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::StaPlinkAction(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "StaPlinkAction",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_mpath_next_hop(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::MpathNextHop(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "MpathNextHop",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_mpath_info(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::MpathInfo(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "MpathInfo",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_bss_cts_prot(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::BssCtsProt(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "BssCtsProt",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_bss_short_preamble(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::BssShortPreamble(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "BssShortPreamble",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_bss_short_slot_time(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::BssShortSlotTime(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "BssShortSlotTime",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_ht_capability(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::HtCapability(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "HtCapability",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_supported_iftypes(&self) -> Result<IterableSupportedIftypes<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::SupportedIftypes(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "SupportedIftypes",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_reg_alpha2(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::RegAlpha2(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "RegAlpha2",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_reg_rules(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::RegRules(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "RegRules",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_mesh_config(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::MeshConfig(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "MeshConfig",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_bss_basic_rates(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::BssBasicRates(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "BssBasicRates",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_wiphy_txq_params(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::WiphyTxqParams(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "WiphyTxqParams",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_wiphy_freq(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::WiphyFreq(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "WiphyFreq",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Associated type: \"ChannelType\" (enum)"]
    pub fn get_wiphy_channel_type(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::WiphyChannelType(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "WiphyChannelType",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_key_default_mgmt(&self) -> Result<(), ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::KeyDefaultMgmt(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "KeyDefaultMgmt",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_mgmt_subtype(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::MgmtSubtype(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "MgmtSubtype",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_ie(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::Ie(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "Ie",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_max_num_scan_ssids(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::MaxNumScanSsids(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "MaxNumScanSsids",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_scan_frequencies(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::ScanFrequencies(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "ScanFrequencies",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_scan_ssids(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::ScanSsids(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "ScanSsids",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_generation(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::Generation(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "Generation",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_bss(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::Bss(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "Bss",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_reg_initiator(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::RegInitiator(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "RegInitiator",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_reg_type(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::RegType(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "RegType",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_frame(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::Frame(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "Frame",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_ssid(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::Ssid(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "Ssid",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_auth_type(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::AuthType(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "AuthType",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_reason_code(&self) -> Result<u16, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::ReasonCode(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "ReasonCode",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_key_type(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::KeyType(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "KeyType",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_max_scan_ie_len(&self) -> Result<u16, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::MaxScanIeLen(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "MaxScanIeLen",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_cipher_suites(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::CipherSuites(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "CipherSuites",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_freq_before(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::FreqBefore(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "FreqBefore",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_freq_after(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::FreqAfter(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "FreqAfter",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_freq_fixed(&self) -> Result<(), ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::FreqFixed(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "FreqFixed",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_wiphy_retry_short(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::WiphyRetryShort(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "WiphyRetryShort",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_wiphy_retry_long(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::WiphyRetryLong(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "WiphyRetryLong",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_wiphy_frag_threshold(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::WiphyFragThreshold(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "WiphyFragThreshold",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_wiphy_rts_threshold(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::WiphyRtsThreshold(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "WiphyRtsThreshold",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_timed_out(&self) -> Result<(), ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::TimedOut(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "TimedOut",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_use_mfp(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::UseMfp(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "UseMfp",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_sta_flags2(&self) -> Result<PushStaFlagUpdate, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::StaFlags2(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "StaFlags2",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_control_port(&self) -> Result<(), ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::ControlPort(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "ControlPort",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_testdata(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::Testdata(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "Testdata",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_privacy(&self) -> Result<(), ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::Privacy(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "Privacy",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_disconnected_by_ap(&self) -> Result<(), ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::DisconnectedByAp(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "DisconnectedByAp",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_status_code(&self) -> Result<u16, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::StatusCode(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "StatusCode",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_cipher_suites_pairwise(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::CipherSuitesPairwise(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "CipherSuitesPairwise",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_cipher_suite_group(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::CipherSuiteGroup(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "CipherSuiteGroup",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_wpa_versions(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::WpaVersions(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "WpaVersions",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_akm_suites(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::AkmSuites(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "AkmSuites",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_req_ie(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::ReqIe(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "ReqIe",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_resp_ie(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::RespIe(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "RespIe",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_prev_bssid(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::PrevBssid(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "PrevBssid",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_key(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::Key(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "Key",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_keys(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::Keys(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "Keys",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_pid(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::Pid(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "Pid",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_4addr(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::_4addr(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "4addr",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_survey_info(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::SurveyInfo(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "SurveyInfo",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_pmkid(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::Pmkid(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "Pmkid",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_max_num_pmkids(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::MaxNumPmkids(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "MaxNumPmkids",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_duration(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::Duration(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "Duration",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_cookie(&self) -> Result<u64, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::Cookie(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "Cookie",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_wiphy_coverage_class(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::WiphyCoverageClass(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "WiphyCoverageClass",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_tx_rates(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::TxRates(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "TxRates",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_frame_match(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::FrameMatch(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "FrameMatch",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_ack(&self) -> Result<(), ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::Ack(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "Ack",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_ps_state(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::PsState(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "PsState",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_cqm(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::Cqm(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "Cqm",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_local_state_change(&self) -> Result<(), ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::LocalStateChange(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "LocalStateChange",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_ap_isolate(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::ApIsolate(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "ApIsolate",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_wiphy_tx_power_setting(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::WiphyTxPowerSetting(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "WiphyTxPowerSetting",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_wiphy_tx_power_level(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::WiphyTxPowerLevel(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "WiphyTxPowerLevel",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_tx_frame_types(&self) -> Result<IterableIftypeAttrs<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::TxFrameTypes(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "TxFrameTypes",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_rx_frame_types(&self) -> Result<IterableIftypeAttrs<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::RxFrameTypes(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "RxFrameTypes",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_frame_type(&self) -> Result<u16, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::FrameType(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "FrameType",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_control_port_ethertype(&self) -> Result<(), ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::ControlPortEthertype(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "ControlPortEthertype",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_control_port_no_encrypt(&self) -> Result<(), ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::ControlPortNoEncrypt(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "ControlPortNoEncrypt",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_support_ibss_rsn(&self) -> Result<(), ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::SupportIbssRsn(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "SupportIbssRsn",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_wiphy_antenna_tx(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::WiphyAntennaTx(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "WiphyAntennaTx",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_wiphy_antenna_rx(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::WiphyAntennaRx(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "WiphyAntennaRx",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_mcast_rate(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::McastRate(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "McastRate",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_offchannel_tx_ok(&self) -> Result<(), ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::OffchannelTxOk(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "OffchannelTxOk",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_bss_ht_opmode(&self) -> Result<u16, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::BssHtOpmode(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "BssHtOpmode",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_key_default_types(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::KeyDefaultTypes(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "KeyDefaultTypes",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_max_remain_on_channel_duration(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::MaxRemainOnChannelDuration(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "MaxRemainOnChannelDuration",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_mesh_setup(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::MeshSetup(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "MeshSetup",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_wiphy_antenna_avail_tx(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::WiphyAntennaAvailTx(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "WiphyAntennaAvailTx",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_wiphy_antenna_avail_rx(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::WiphyAntennaAvailRx(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "WiphyAntennaAvailRx",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_support_mesh_auth(&self) -> Result<(), ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::SupportMeshAuth(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "SupportMeshAuth",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_sta_plink_state(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::StaPlinkState(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "StaPlinkState",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_wowlan_triggers(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::WowlanTriggers(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "WowlanTriggers",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_wowlan_triggers_supported(
        &self,
    ) -> Result<IterableWowlanTriggersAttrs<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::WowlanTriggersSupported(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "WowlanTriggersSupported",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_sched_scan_interval(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::SchedScanInterval(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "SchedScanInterval",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_interface_combinations(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::InterfaceCombinations(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "InterfaceCombinations",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_software_iftypes(&self) -> Result<IterableSupportedIftypes<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::SoftwareIftypes(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "SoftwareIftypes",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_rekey_data(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::RekeyData(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "RekeyData",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_max_num_sched_scan_ssids(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::MaxNumSchedScanSsids(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "MaxNumSchedScanSsids",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_max_sched_scan_ie_len(&self) -> Result<u16, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::MaxSchedScanIeLen(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "MaxSchedScanIeLen",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_scan_supp_rates(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::ScanSuppRates(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "ScanSuppRates",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_hidden_ssid(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::HiddenSsid(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "HiddenSsid",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_ie_probe_resp(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::IeProbeResp(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "IeProbeResp",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_ie_assoc_resp(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::IeAssocResp(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "IeAssocResp",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_sta_wme(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::StaWme(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "StaWme",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_support_ap_uapsd(&self) -> Result<(), ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::SupportApUapsd(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "SupportApUapsd",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_roam_support(&self) -> Result<(), ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::RoamSupport(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "RoamSupport",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_sched_scan_match(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::SchedScanMatch(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "SchedScanMatch",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_max_match_sets(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::MaxMatchSets(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "MaxMatchSets",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_pmksa_candidate(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::PmksaCandidate(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "PmksaCandidate",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_tx_no_cck_rate(&self) -> Result<(), ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::TxNoCckRate(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "TxNoCckRate",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_tdls_action(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::TdlsAction(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "TdlsAction",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_tdls_dialog_token(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::TdlsDialogToken(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "TdlsDialogToken",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_tdls_operation(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::TdlsOperation(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "TdlsOperation",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_tdls_support(&self) -> Result<(), ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::TdlsSupport(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "TdlsSupport",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_tdls_external_setup(&self) -> Result<(), ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::TdlsExternalSetup(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "TdlsExternalSetup",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_device_ap_sme(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::DeviceApSme(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "DeviceApSme",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_dont_wait_for_ack(&self) -> Result<(), ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::DontWaitForAck(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "DontWaitForAck",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Associated type: \"FeatureFlags\" (1 bit per enumeration)"]
    pub fn get_feature_flags(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::FeatureFlags(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "FeatureFlags",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_probe_resp_offload(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::ProbeRespOffload(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "ProbeRespOffload",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_probe_resp(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::ProbeResp(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "ProbeResp",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_dfs_region(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::DfsRegion(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "DfsRegion",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_disable_ht(&self) -> Result<(), ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::DisableHt(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "DisableHt",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_ht_capability_mask(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::HtCapabilityMask(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "HtCapabilityMask",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_noack_map(&self) -> Result<u16, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::NoackMap(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "NoackMap",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_inactivity_timeout(&self) -> Result<u16, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::InactivityTimeout(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "InactivityTimeout",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_rx_signal_dbm(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::RxSignalDbm(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "RxSignalDbm",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_bg_scan_period(&self) -> Result<u16, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::BgScanPeriod(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "BgScanPeriod",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_wdev(&self) -> Result<u64, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::Wdev(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "Wdev",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_user_reg_hint_type(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::UserRegHintType(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "UserRegHintType",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_conn_failed_reason(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::ConnFailedReason(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "ConnFailedReason",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_auth_data(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::AuthData(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "AuthData",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_vht_capability(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::VhtCapability(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "VhtCapability",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_scan_flags(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::ScanFlags(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "ScanFlags",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_channel_width(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::ChannelWidth(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "ChannelWidth",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_center_freq1(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::CenterFreq1(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "CenterFreq1",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_center_freq2(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::CenterFreq2(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "CenterFreq2",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_p2p_ctwindow(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::P2pCtwindow(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "P2pCtwindow",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_p2p_oppps(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::P2pOppps(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "P2pOppps",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_local_mesh_power_mode(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::LocalMeshPowerMode(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "LocalMeshPowerMode",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_acl_policy(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::AclPolicy(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "AclPolicy",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_mac_addrs(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::MacAddrs(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "MacAddrs",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_mac_acl_max(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::MacAclMax(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "MacAclMax",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_radar_event(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::RadarEvent(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "RadarEvent",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_ext_capa(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::ExtCapa(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "ExtCapa",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_ext_capa_mask(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::ExtCapaMask(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "ExtCapaMask",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_sta_capability(&self) -> Result<u16, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::StaCapability(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "StaCapability",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_sta_ext_capability(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::StaExtCapability(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "StaExtCapability",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Associated type: \"ProtocolFeatures\" (enum)"]
    pub fn get_protocol_features(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::ProtocolFeatures(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "ProtocolFeatures",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_split_wiphy_dump(&self) -> Result<(), ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::SplitWiphyDump(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "SplitWiphyDump",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_disable_vht(&self) -> Result<(), ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::DisableVht(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "DisableVht",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_vht_capability_mask(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::VhtCapabilityMask(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "VhtCapabilityMask",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_mdid(&self) -> Result<u16, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::Mdid(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "Mdid",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_ie_ric(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::IeRic(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "IeRic",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_crit_prot_id(&self) -> Result<u16, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::CritProtId(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "CritProtId",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_max_crit_prot_duration(&self) -> Result<u16, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::MaxCritProtDuration(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "MaxCritProtDuration",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_peer_aid(&self) -> Result<u16, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::PeerAid(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "PeerAid",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_coalesce_rule(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::CoalesceRule(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "CoalesceRule",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_ch_switch_count(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::ChSwitchCount(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "ChSwitchCount",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_ch_switch_block_tx(&self) -> Result<(), ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::ChSwitchBlockTx(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "ChSwitchBlockTx",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_csa_ies(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::CsaIes(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "CsaIes",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_cntdwn_offs_beacon(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::CntdwnOffsBeacon(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "CntdwnOffsBeacon",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_cntdwn_offs_presp(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::CntdwnOffsPresp(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "CntdwnOffsPresp",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_rxmgmt_flags(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::RxmgmtFlags(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "RxmgmtFlags",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_sta_supported_channels(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::StaSupportedChannels(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "StaSupportedChannels",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_sta_supported_oper_classes(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::StaSupportedOperClasses(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "StaSupportedOperClasses",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_handle_dfs(&self) -> Result<(), ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::HandleDfs(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "HandleDfs",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_support_5_mhz(&self) -> Result<(), ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::Support5Mhz(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "Support5Mhz",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_support_10_mhz(&self) -> Result<(), ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::Support10Mhz(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "Support10Mhz",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_opmode_notif(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::OpmodeNotif(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "OpmodeNotif",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_vendor_id(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::VendorId(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "VendorId",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_vendor_subcmd(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::VendorSubcmd(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "VendorSubcmd",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_vendor_data(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::VendorData(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "VendorData",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_vendor_events(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::VendorEvents(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "VendorEvents",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_qos_map(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::QosMap(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "QosMap",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_mac_hint(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::MacHint(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "MacHint",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_wiphy_freq_hint(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::WiphyFreqHint(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "WiphyFreqHint",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_max_ap_assoc_sta(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::MaxApAssocSta(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "MaxApAssocSta",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_tdls_peer_capability(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::TdlsPeerCapability(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "TdlsPeerCapability",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_socket_owner(&self) -> Result<(), ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::SocketOwner(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "SocketOwner",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_csa_c_offsets_tx(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::CsaCOffsetsTx(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "CsaCOffsetsTx",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_max_csa_counters(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::MaxCsaCounters(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "MaxCsaCounters",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_tdls_initiator(&self) -> Result<(), ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::TdlsInitiator(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "TdlsInitiator",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_use_rrm(&self) -> Result<(), ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::UseRrm(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "UseRrm",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_wiphy_dyn_ack(&self) -> Result<(), ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::WiphyDynAck(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "WiphyDynAck",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_tsid(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::Tsid(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "Tsid",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_user_prio(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::UserPrio(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "UserPrio",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_admitted_time(&self) -> Result<u16, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::AdmittedTime(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "AdmittedTime",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_smps_mode(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::SmpsMode(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "SmpsMode",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_oper_class(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::OperClass(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "OperClass",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_mac_mask(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::MacMask(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "MacMask",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_wiphy_self_managed_reg(&self) -> Result<(), ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::WiphySelfManagedReg(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "WiphySelfManagedReg",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_ext_features(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::ExtFeatures(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "ExtFeatures",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_survey_radio_stats(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::SurveyRadioStats(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "SurveyRadioStats",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_netns_fd(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::NetnsFd(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "NetnsFd",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_sched_scan_delay(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::SchedScanDelay(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "SchedScanDelay",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_reg_indoor(&self) -> Result<(), ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::RegIndoor(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "RegIndoor",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_max_num_sched_scan_plans(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::MaxNumSchedScanPlans(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "MaxNumSchedScanPlans",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_max_scan_plan_interval(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::MaxScanPlanInterval(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "MaxScanPlanInterval",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_max_scan_plan_iterations(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::MaxScanPlanIterations(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "MaxScanPlanIterations",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_sched_scan_plans(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::SchedScanPlans(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "SchedScanPlans",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_pbss(&self) -> Result<(), ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::Pbss(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "Pbss",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_bss_select(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::BssSelect(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "BssSelect",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_sta_support_p2p_ps(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::StaSupportP2pPs(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "StaSupportP2pPs",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_pad(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::Pad(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "Pad",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_iftype_ext_capa(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::IftypeExtCapa(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "IftypeExtCapa",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_mu_mimo_group_data(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::MuMimoGroupData(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "MuMimoGroupData",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_mu_mimo_follow_mac_addr(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::MuMimoFollowMacAddr(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "MuMimoFollowMacAddr",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_scan_start_time_tsf(&self) -> Result<u64, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::ScanStartTimeTsf(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "ScanStartTimeTsf",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_scan_start_time_tsf_bssid(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::ScanStartTimeTsfBssid(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "ScanStartTimeTsfBssid",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_measurement_duration(&self) -> Result<u16, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::MeasurementDuration(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "MeasurementDuration",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_measurement_duration_mandatory(&self) -> Result<(), ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::MeasurementDurationMandatory(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "MeasurementDurationMandatory",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_mesh_peer_aid(&self) -> Result<u16, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::MeshPeerAid(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "MeshPeerAid",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_nan_master_pref(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::NanMasterPref(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "NanMasterPref",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_bands(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::Bands(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "Bands",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_nan_func(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::NanFunc(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "NanFunc",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_nan_match(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::NanMatch(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "NanMatch",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_fils_kek(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::FilsKek(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "FilsKek",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_fils_nonces(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::FilsNonces(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "FilsNonces",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_multicast_to_unicast_enabled(&self) -> Result<(), ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::MulticastToUnicastEnabled(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "MulticastToUnicastEnabled",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_bssid(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::Bssid(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "Bssid",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_sched_scan_relative_rssi(&self) -> Result<i8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::SchedScanRelativeRssi(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "SchedScanRelativeRssi",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_sched_scan_rssi_adjust(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::SchedScanRssiAdjust(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "SchedScanRssiAdjust",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_timeout_reason(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::TimeoutReason(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "TimeoutReason",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_fils_erp_username(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::FilsErpUsername(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "FilsErpUsername",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_fils_erp_realm(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::FilsErpRealm(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "FilsErpRealm",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_fils_erp_next_seq_num(&self) -> Result<u16, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::FilsErpNextSeqNum(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "FilsErpNextSeqNum",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_fils_erp_rrk(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::FilsErpRrk(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "FilsErpRrk",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_fils_cache_id(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::FilsCacheId(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "FilsCacheId",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_pmk(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::Pmk(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "Pmk",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_sched_scan_multi(&self) -> Result<(), ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::SchedScanMulti(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "SchedScanMulti",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_sched_scan_max_reqs(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::SchedScanMaxReqs(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "SchedScanMaxReqs",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_want_1x_4way_hs(&self) -> Result<(), ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::Want1x4wayHs(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "Want1x4wayHs",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_pmkr0_name(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::Pmkr0Name(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "Pmkr0Name",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_port_authorized(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::PortAuthorized(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "PortAuthorized",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_external_auth_action(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::ExternalAuthAction(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "ExternalAuthAction",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_external_auth_support(&self) -> Result<(), ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::ExternalAuthSupport(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "ExternalAuthSupport",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_nss(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::Nss(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "Nss",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_ack_signal(&self) -> Result<i32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::AckSignal(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "AckSignal",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_control_port_over_nl80211(&self) -> Result<(), ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::ControlPortOverNl80211(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "ControlPortOverNl80211",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_txq_stats(&self) -> Result<IterableTxqStatsAttrs<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::TxqStats(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "TxqStats",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_txq_limit(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::TxqLimit(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "TxqLimit",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_txq_memory_limit(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::TxqMemoryLimit(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "TxqMemoryLimit",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_txq_quantum(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::TxqQuantum(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "TxqQuantum",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_he_capability(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::HeCapability(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "HeCapability",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_ftm_responder(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::FtmResponder(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "FtmResponder",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_ftm_responder_stats(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::FtmResponderStats(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "FtmResponderStats",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_timeout(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::Timeout(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "Timeout",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_peer_measurements(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::PeerMeasurements(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "PeerMeasurements",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_airtime_weight(&self) -> Result<u16, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::AirtimeWeight(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "AirtimeWeight",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_sta_tx_power_setting(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::StaTxPowerSetting(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "StaTxPowerSetting",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_sta_tx_power(&self) -> Result<i16, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::StaTxPower(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "StaTxPower",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_sae_password(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::SaePassword(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "SaePassword",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_twt_responder(&self) -> Result<(), ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::TwtResponder(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "TwtResponder",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_he_obss_pd(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::HeObssPd(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "HeObssPd",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_wiphy_edmg_channels(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::WiphyEdmgChannels(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "WiphyEdmgChannels",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_wiphy_edmg_bw_config(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::WiphyEdmgBwConfig(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "WiphyEdmgBwConfig",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_vlan_id(&self) -> Result<u16, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::VlanId(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "VlanId",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_he_bss_color(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::HeBssColor(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "HeBssColor",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_iftype_akm_suites(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::IftypeAkmSuites(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "IftypeAkmSuites",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_tid_config(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::TidConfig(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "TidConfig",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_control_port_no_preauth(&self) -> Result<(), ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::ControlPortNoPreauth(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "ControlPortNoPreauth",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_pmk_lifetime(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::PmkLifetime(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "PmkLifetime",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_pmk_reauth_threshold(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::PmkReauthThreshold(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "PmkReauthThreshold",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_receive_multicast(&self) -> Result<(), ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::ReceiveMulticast(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "ReceiveMulticast",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_wiphy_freq_offset(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::WiphyFreqOffset(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "WiphyFreqOffset",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_center_freq1_offset(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::CenterFreq1Offset(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "CenterFreq1Offset",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_scan_freq_khz(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::ScanFreqKhz(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "ScanFreqKhz",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_he_6ghz_capability(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::He6ghzCapability(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "He6ghzCapability",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_fils_discovery(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::FilsDiscovery(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "FilsDiscovery",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_unsol_bcast_probe_resp(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::UnsolBcastProbeResp(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "UnsolBcastProbeResp",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_s1g_capability(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::S1gCapability(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "S1gCapability",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_s1g_capability_mask(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::S1gCapabilityMask(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "S1gCapabilityMask",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_sae_pwe(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::SaePwe(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "SaePwe",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_reconnect_requested(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::ReconnectRequested(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "ReconnectRequested",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_sar_spec(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::SarSpec(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "SarSpec",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_disable_he(&self) -> Result<(), ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::DisableHe(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "DisableHe",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_obss_color_bitmap(&self) -> Result<u64, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::ObssColorBitmap(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "ObssColorBitmap",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_color_change_count(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::ColorChangeCount(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "ColorChangeCount",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_color_change_color(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::ColorChangeColor(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "ColorChangeColor",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_color_change_elems(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::ColorChangeElems(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "ColorChangeElems",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_mbssid_config(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::MbssidConfig(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "MbssidConfig",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_mbssid_elems(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::MbssidElems(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "MbssidElems",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_radar_background(&self) -> Result<(), ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::RadarBackground(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "RadarBackground",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_ap_settings_flags(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::ApSettingsFlags(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "ApSettingsFlags",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_eht_capability(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::EhtCapability(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "EhtCapability",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_disable_eht(&self) -> Result<(), ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::DisableEht(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "DisableEht",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_mlo_links(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::MloLinks(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "MloLinks",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_mlo_link_id(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::MloLinkId(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "MloLinkId",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_mld_addr(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::MldAddr(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "MldAddr",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_mlo_support(&self) -> Result<(), ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::MloSupport(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "MloSupport",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_max_num_akm_suites(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::MaxNumAkmSuites(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "MaxNumAkmSuites",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_eml_capability(&self) -> Result<u16, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::EmlCapability(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "EmlCapability",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_mld_capa_and_ops(&self) -> Result<u16, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::MldCapaAndOps(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "MldCapaAndOps",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_tx_hw_timestamp(&self) -> Result<u64, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::TxHwTimestamp(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "TxHwTimestamp",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_rx_hw_timestamp(&self) -> Result<u64, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::RxHwTimestamp(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "RxHwTimestamp",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_td_bitmap(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::TdBitmap(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "TdBitmap",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_punct_bitmap(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::PunctBitmap(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "PunctBitmap",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_max_hw_timestamp_peers(&self) -> Result<u16, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::MaxHwTimestampPeers(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "MaxHwTimestampPeers",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_hw_timestamp_enabled(&self) -> Result<(), ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::HwTimestampEnabled(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "HwTimestampEnabled",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_ema_rnr_elems(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::EmaRnrElems(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "EmaRnrElems",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_mlo_link_disabled(&self) -> Result<(), ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::MloLinkDisabled(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "MloLinkDisabled",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_bss_dump_include_use_data(&self) -> Result<(), ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::BssDumpIncludeUseData(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "BssDumpIncludeUseData",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_mlo_ttlm_dlink(&self) -> Result<u16, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::MloTtlmDlink(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "MloTtlmDlink",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_mlo_ttlm_ulink(&self) -> Result<u16, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::MloTtlmUlink(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "MloTtlmUlink",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_assoc_spp_amsdu(&self) -> Result<(), ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::AssocSppAmsdu(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "AssocSppAmsdu",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_wiphy_radios(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::WiphyRadios(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "WiphyRadios",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_wiphy_interface_combinations(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::WiphyInterfaceCombinations(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "WiphyInterfaceCombinations",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_vif_radio_mask(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Nl80211Attrs::VifRadioMask(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Nl80211Attrs",
            "VifRadioMask",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl<'a> Nl80211Attrs<'a> {
    pub fn new(buf: &'a [u8]) -> IterableNl80211Attrs<'a> {
        IterableNl80211Attrs::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        let res = match r#type {
            1u16 => "Wiphy",
            2u16 => "WiphyName",
            3u16 => "Ifindex",
            4u16 => "Ifname",
            5u16 => "Iftype",
            6u16 => "Mac",
            7u16 => "KeyData",
            8u16 => "KeyIdx",
            9u16 => "KeyCipher",
            10u16 => "KeySeq",
            11u16 => "KeyDefault",
            12u16 => "BeaconInterval",
            13u16 => "DtimPeriod",
            14u16 => "BeaconHead",
            15u16 => "BeaconTail",
            16u16 => "StaAid",
            17u16 => "StaFlags",
            18u16 => "StaListenInterval",
            19u16 => "StaSupportedRates",
            20u16 => "StaVlan",
            21u16 => "StaInfo",
            22u16 => "WiphyBands",
            23u16 => "MntrFlags",
            24u16 => "MeshId",
            25u16 => "StaPlinkAction",
            26u16 => "MpathNextHop",
            27u16 => "MpathInfo",
            28u16 => "BssCtsProt",
            29u16 => "BssShortPreamble",
            30u16 => "BssShortSlotTime",
            31u16 => "HtCapability",
            32u16 => "SupportedIftypes",
            33u16 => "RegAlpha2",
            34u16 => "RegRules",
            35u16 => "MeshConfig",
            36u16 => "BssBasicRates",
            37u16 => "WiphyTxqParams",
            38u16 => "WiphyFreq",
            39u16 => "WiphyChannelType",
            40u16 => "KeyDefaultMgmt",
            41u16 => "MgmtSubtype",
            42u16 => "Ie",
            43u16 => "MaxNumScanSsids",
            44u16 => "ScanFrequencies",
            45u16 => "ScanSsids",
            46u16 => "Generation",
            47u16 => "Bss",
            48u16 => "RegInitiator",
            49u16 => "RegType",
            50u16 => "Frame",
            51u16 => "Ssid",
            52u16 => "AuthType",
            53u16 => "ReasonCode",
            54u16 => "KeyType",
            55u16 => "MaxScanIeLen",
            56u16 => "CipherSuites",
            57u16 => "FreqBefore",
            58u16 => "FreqAfter",
            59u16 => "FreqFixed",
            60u16 => "WiphyRetryShort",
            61u16 => "WiphyRetryLong",
            62u16 => "WiphyFragThreshold",
            63u16 => "WiphyRtsThreshold",
            64u16 => "TimedOut",
            65u16 => "UseMfp",
            66u16 => "StaFlags2",
            67u16 => "ControlPort",
            68u16 => "Testdata",
            69u16 => "Privacy",
            70u16 => "DisconnectedByAp",
            71u16 => "StatusCode",
            72u16 => "CipherSuitesPairwise",
            73u16 => "CipherSuiteGroup",
            74u16 => "WpaVersions",
            75u16 => "AkmSuites",
            76u16 => "ReqIe",
            77u16 => "RespIe",
            78u16 => "PrevBssid",
            79u16 => "Key",
            80u16 => "Keys",
            81u16 => "Pid",
            82u16 => "4addr",
            83u16 => "SurveyInfo",
            84u16 => "Pmkid",
            85u16 => "MaxNumPmkids",
            86u16 => "Duration",
            87u16 => "Cookie",
            88u16 => "WiphyCoverageClass",
            89u16 => "TxRates",
            90u16 => "FrameMatch",
            91u16 => "Ack",
            92u16 => "PsState",
            93u16 => "Cqm",
            94u16 => "LocalStateChange",
            95u16 => "ApIsolate",
            96u16 => "WiphyTxPowerSetting",
            97u16 => "WiphyTxPowerLevel",
            98u16 => "TxFrameTypes",
            99u16 => "RxFrameTypes",
            100u16 => "FrameType",
            101u16 => "ControlPortEthertype",
            102u16 => "ControlPortNoEncrypt",
            103u16 => "SupportIbssRsn",
            104u16 => "WiphyAntennaTx",
            105u16 => "WiphyAntennaRx",
            106u16 => "McastRate",
            107u16 => "OffchannelTxOk",
            108u16 => "BssHtOpmode",
            109u16 => "KeyDefaultTypes",
            110u16 => "MaxRemainOnChannelDuration",
            111u16 => "MeshSetup",
            112u16 => "WiphyAntennaAvailTx",
            113u16 => "WiphyAntennaAvailRx",
            114u16 => "SupportMeshAuth",
            115u16 => "StaPlinkState",
            116u16 => "WowlanTriggers",
            117u16 => "WowlanTriggersSupported",
            118u16 => "SchedScanInterval",
            119u16 => "InterfaceCombinations",
            120u16 => "SoftwareIftypes",
            121u16 => "RekeyData",
            122u16 => "MaxNumSchedScanSsids",
            123u16 => "MaxSchedScanIeLen",
            124u16 => "ScanSuppRates",
            125u16 => "HiddenSsid",
            126u16 => "IeProbeResp",
            127u16 => "IeAssocResp",
            128u16 => "StaWme",
            129u16 => "SupportApUapsd",
            130u16 => "RoamSupport",
            131u16 => "SchedScanMatch",
            132u16 => "MaxMatchSets",
            133u16 => "PmksaCandidate",
            134u16 => "TxNoCckRate",
            135u16 => "TdlsAction",
            136u16 => "TdlsDialogToken",
            137u16 => "TdlsOperation",
            138u16 => "TdlsSupport",
            139u16 => "TdlsExternalSetup",
            140u16 => "DeviceApSme",
            141u16 => "DontWaitForAck",
            142u16 => "FeatureFlags",
            143u16 => "ProbeRespOffload",
            144u16 => "ProbeResp",
            145u16 => "DfsRegion",
            146u16 => "DisableHt",
            147u16 => "HtCapabilityMask",
            148u16 => "NoackMap",
            149u16 => "InactivityTimeout",
            150u16 => "RxSignalDbm",
            151u16 => "BgScanPeriod",
            152u16 => "Wdev",
            153u16 => "UserRegHintType",
            154u16 => "ConnFailedReason",
            155u16 => "AuthData",
            156u16 => "VhtCapability",
            157u16 => "ScanFlags",
            158u16 => "ChannelWidth",
            159u16 => "CenterFreq1",
            160u16 => "CenterFreq2",
            161u16 => "P2pCtwindow",
            162u16 => "P2pOppps",
            163u16 => "LocalMeshPowerMode",
            164u16 => "AclPolicy",
            165u16 => "MacAddrs",
            166u16 => "MacAclMax",
            167u16 => "RadarEvent",
            168u16 => "ExtCapa",
            169u16 => "ExtCapaMask",
            170u16 => "StaCapability",
            171u16 => "StaExtCapability",
            172u16 => "ProtocolFeatures",
            173u16 => "SplitWiphyDump",
            174u16 => "DisableVht",
            175u16 => "VhtCapabilityMask",
            176u16 => "Mdid",
            177u16 => "IeRic",
            178u16 => "CritProtId",
            179u16 => "MaxCritProtDuration",
            180u16 => "PeerAid",
            181u16 => "CoalesceRule",
            182u16 => "ChSwitchCount",
            183u16 => "ChSwitchBlockTx",
            184u16 => "CsaIes",
            185u16 => "CntdwnOffsBeacon",
            186u16 => "CntdwnOffsPresp",
            187u16 => "RxmgmtFlags",
            188u16 => "StaSupportedChannels",
            189u16 => "StaSupportedOperClasses",
            190u16 => "HandleDfs",
            191u16 => "Support5Mhz",
            192u16 => "Support10Mhz",
            193u16 => "OpmodeNotif",
            194u16 => "VendorId",
            195u16 => "VendorSubcmd",
            196u16 => "VendorData",
            197u16 => "VendorEvents",
            198u16 => "QosMap",
            199u16 => "MacHint",
            200u16 => "WiphyFreqHint",
            201u16 => "MaxApAssocSta",
            202u16 => "TdlsPeerCapability",
            203u16 => "SocketOwner",
            204u16 => "CsaCOffsetsTx",
            205u16 => "MaxCsaCounters",
            206u16 => "TdlsInitiator",
            207u16 => "UseRrm",
            208u16 => "WiphyDynAck",
            209u16 => "Tsid",
            210u16 => "UserPrio",
            211u16 => "AdmittedTime",
            212u16 => "SmpsMode",
            213u16 => "OperClass",
            214u16 => "MacMask",
            215u16 => "WiphySelfManagedReg",
            216u16 => "ExtFeatures",
            217u16 => "SurveyRadioStats",
            218u16 => "NetnsFd",
            219u16 => "SchedScanDelay",
            220u16 => "RegIndoor",
            221u16 => "MaxNumSchedScanPlans",
            222u16 => "MaxScanPlanInterval",
            223u16 => "MaxScanPlanIterations",
            224u16 => "SchedScanPlans",
            225u16 => "Pbss",
            226u16 => "BssSelect",
            227u16 => "StaSupportP2pPs",
            228u16 => "Pad",
            229u16 => "IftypeExtCapa",
            230u16 => "MuMimoGroupData",
            231u16 => "MuMimoFollowMacAddr",
            232u16 => "ScanStartTimeTsf",
            233u16 => "ScanStartTimeTsfBssid",
            234u16 => "MeasurementDuration",
            235u16 => "MeasurementDurationMandatory",
            236u16 => "MeshPeerAid",
            237u16 => "NanMasterPref",
            238u16 => "Bands",
            239u16 => "NanFunc",
            240u16 => "NanMatch",
            241u16 => "FilsKek",
            242u16 => "FilsNonces",
            243u16 => "MulticastToUnicastEnabled",
            244u16 => "Bssid",
            245u16 => "SchedScanRelativeRssi",
            246u16 => "SchedScanRssiAdjust",
            247u16 => "TimeoutReason",
            248u16 => "FilsErpUsername",
            249u16 => "FilsErpRealm",
            250u16 => "FilsErpNextSeqNum",
            251u16 => "FilsErpRrk",
            252u16 => "FilsCacheId",
            253u16 => "Pmk",
            254u16 => "SchedScanMulti",
            255u16 => "SchedScanMaxReqs",
            256u16 => "Want1x4wayHs",
            257u16 => "Pmkr0Name",
            258u16 => "PortAuthorized",
            259u16 => "ExternalAuthAction",
            260u16 => "ExternalAuthSupport",
            261u16 => "Nss",
            262u16 => "AckSignal",
            263u16 => "ControlPortOverNl80211",
            264u16 => "TxqStats",
            265u16 => "TxqLimit",
            266u16 => "TxqMemoryLimit",
            267u16 => "TxqQuantum",
            268u16 => "HeCapability",
            269u16 => "FtmResponder",
            270u16 => "FtmResponderStats",
            271u16 => "Timeout",
            272u16 => "PeerMeasurements",
            273u16 => "AirtimeWeight",
            274u16 => "StaTxPowerSetting",
            275u16 => "StaTxPower",
            276u16 => "SaePassword",
            277u16 => "TwtResponder",
            278u16 => "HeObssPd",
            279u16 => "WiphyEdmgChannels",
            280u16 => "WiphyEdmgBwConfig",
            281u16 => "VlanId",
            282u16 => "HeBssColor",
            283u16 => "IftypeAkmSuites",
            284u16 => "TidConfig",
            285u16 => "ControlPortNoPreauth",
            286u16 => "PmkLifetime",
            287u16 => "PmkReauthThreshold",
            288u16 => "ReceiveMulticast",
            289u16 => "WiphyFreqOffset",
            290u16 => "CenterFreq1Offset",
            291u16 => "ScanFreqKhz",
            292u16 => "He6ghzCapability",
            293u16 => "FilsDiscovery",
            294u16 => "UnsolBcastProbeResp",
            295u16 => "S1gCapability",
            296u16 => "S1gCapabilityMask",
            297u16 => "SaePwe",
            298u16 => "ReconnectRequested",
            299u16 => "SarSpec",
            300u16 => "DisableHe",
            301u16 => "ObssColorBitmap",
            302u16 => "ColorChangeCount",
            303u16 => "ColorChangeColor",
            304u16 => "ColorChangeElems",
            305u16 => "MbssidConfig",
            306u16 => "MbssidElems",
            307u16 => "RadarBackground",
            308u16 => "ApSettingsFlags",
            309u16 => "EhtCapability",
            310u16 => "DisableEht",
            311u16 => "MloLinks",
            312u16 => "MloLinkId",
            313u16 => "MldAddr",
            314u16 => "MloSupport",
            315u16 => "MaxNumAkmSuites",
            316u16 => "EmlCapability",
            317u16 => "MldCapaAndOps",
            318u16 => "TxHwTimestamp",
            319u16 => "RxHwTimestamp",
            320u16 => "TdBitmap",
            321u16 => "PunctBitmap",
            322u16 => "MaxHwTimestampPeers",
            323u16 => "HwTimestampEnabled",
            324u16 => "EmaRnrElems",
            325u16 => "MloLinkDisabled",
            326u16 => "BssDumpIncludeUseData",
            327u16 => "MloTtlmDlink",
            328u16 => "MloTtlmUlink",
            329u16 => "AssocSppAmsdu",
            330u16 => "WiphyRadios",
            331u16 => "WiphyInterfaceCombinations",
            332u16 => "VifRadioMask",
            _ => return None,
        };
        Some(res)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableNl80211Attrs<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableNl80211Attrs<'a> {
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
impl<'a> Iterator for IterableNl80211Attrs<'a> {
    type Item = Result<Nl80211Attrs<'a>, ErrorContext>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.buf.len() == self.pos {
            return None;
        }
        let pos = self.pos;
        let mut r#type = None;
        while let Some((header, next)) = chop_header(self.buf, &mut self.pos) {
            r#type = Some(header.r#type);
            let res = match header.r#type {
                1u16 => Nl80211Attrs::Wiphy({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                2u16 => Nl80211Attrs::WiphyName({
                    let res = CStr::from_bytes_with_nul(next).ok();
                    let Some(val) = res else { break };
                    val
                }),
                3u16 => Nl80211Attrs::Ifindex({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                4u16 => Nl80211Attrs::Ifname({
                    let res = CStr::from_bytes_with_nul(next).ok();
                    let Some(val) = res else { break };
                    val
                }),
                5u16 => Nl80211Attrs::Iftype({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                6u16 => Nl80211Attrs::Mac({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                7u16 => Nl80211Attrs::KeyData({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                8u16 => Nl80211Attrs::KeyIdx({
                    let res = parse_u8(next);
                    let Some(val) = res else { break };
                    val
                }),
                9u16 => Nl80211Attrs::KeyCipher({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                10u16 => Nl80211Attrs::KeySeq({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                11u16 => Nl80211Attrs::KeyDefault(()),
                12u16 => Nl80211Attrs::BeaconInterval({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                13u16 => Nl80211Attrs::DtimPeriod({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                14u16 => Nl80211Attrs::BeaconHead({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                15u16 => Nl80211Attrs::BeaconTail({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                16u16 => Nl80211Attrs::StaAid({
                    let res = parse_u16(next);
                    let Some(val) = res else { break };
                    val
                }),
                17u16 => Nl80211Attrs::StaFlags({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                18u16 => Nl80211Attrs::StaListenInterval({
                    let res = parse_u16(next);
                    let Some(val) = res else { break };
                    val
                }),
                19u16 => Nl80211Attrs::StaSupportedRates({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                20u16 => Nl80211Attrs::StaVlan({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                21u16 => Nl80211Attrs::StaInfo({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                22u16 => Nl80211Attrs::WiphyBands({
                    let res = Some(IterableWiphyBands::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                23u16 => Nl80211Attrs::MntrFlags({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                24u16 => Nl80211Attrs::MeshId({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                25u16 => Nl80211Attrs::StaPlinkAction({
                    let res = parse_u8(next);
                    let Some(val) = res else { break };
                    val
                }),
                26u16 => Nl80211Attrs::MpathNextHop({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                27u16 => Nl80211Attrs::MpathInfo({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                28u16 => Nl80211Attrs::BssCtsProt({
                    let res = parse_u8(next);
                    let Some(val) = res else { break };
                    val
                }),
                29u16 => Nl80211Attrs::BssShortPreamble({
                    let res = parse_u8(next);
                    let Some(val) = res else { break };
                    val
                }),
                30u16 => Nl80211Attrs::BssShortSlotTime({
                    let res = parse_u8(next);
                    let Some(val) = res else { break };
                    val
                }),
                31u16 => Nl80211Attrs::HtCapability({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                32u16 => Nl80211Attrs::SupportedIftypes({
                    let res = Some(IterableSupportedIftypes::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                33u16 => Nl80211Attrs::RegAlpha2({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                34u16 => Nl80211Attrs::RegRules({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                35u16 => Nl80211Attrs::MeshConfig({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                36u16 => Nl80211Attrs::BssBasicRates({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                37u16 => Nl80211Attrs::WiphyTxqParams({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                38u16 => Nl80211Attrs::WiphyFreq({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                39u16 => Nl80211Attrs::WiphyChannelType({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                40u16 => Nl80211Attrs::KeyDefaultMgmt(()),
                41u16 => Nl80211Attrs::MgmtSubtype({
                    let res = parse_u8(next);
                    let Some(val) = res else { break };
                    val
                }),
                42u16 => Nl80211Attrs::Ie({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                43u16 => Nl80211Attrs::MaxNumScanSsids({
                    let res = parse_u8(next);
                    let Some(val) = res else { break };
                    val
                }),
                44u16 => Nl80211Attrs::ScanFrequencies({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                45u16 => Nl80211Attrs::ScanSsids({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                46u16 => Nl80211Attrs::Generation({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                47u16 => Nl80211Attrs::Bss({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                48u16 => Nl80211Attrs::RegInitiator({
                    let res = parse_u8(next);
                    let Some(val) = res else { break };
                    val
                }),
                49u16 => Nl80211Attrs::RegType({
                    let res = parse_u8(next);
                    let Some(val) = res else { break };
                    val
                }),
                50u16 => Nl80211Attrs::Frame({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                51u16 => Nl80211Attrs::Ssid({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                52u16 => Nl80211Attrs::AuthType({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                53u16 => Nl80211Attrs::ReasonCode({
                    let res = parse_u16(next);
                    let Some(val) = res else { break };
                    val
                }),
                54u16 => Nl80211Attrs::KeyType({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                55u16 => Nl80211Attrs::MaxScanIeLen({
                    let res = parse_u16(next);
                    let Some(val) = res else { break };
                    val
                }),
                56u16 => Nl80211Attrs::CipherSuites({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                57u16 => Nl80211Attrs::FreqBefore({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                58u16 => Nl80211Attrs::FreqAfter({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                59u16 => Nl80211Attrs::FreqFixed(()),
                60u16 => Nl80211Attrs::WiphyRetryShort({
                    let res = parse_u8(next);
                    let Some(val) = res else { break };
                    val
                }),
                61u16 => Nl80211Attrs::WiphyRetryLong({
                    let res = parse_u8(next);
                    let Some(val) = res else { break };
                    val
                }),
                62u16 => Nl80211Attrs::WiphyFragThreshold({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                63u16 => Nl80211Attrs::WiphyRtsThreshold({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                64u16 => Nl80211Attrs::TimedOut(()),
                65u16 => Nl80211Attrs::UseMfp({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                66u16 => Nl80211Attrs::StaFlags2({
                    let res = PushStaFlagUpdate::new_from_slice(next);
                    let Some(val) = res else { break };
                    val
                }),
                67u16 => Nl80211Attrs::ControlPort(()),
                68u16 => Nl80211Attrs::Testdata({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                69u16 => Nl80211Attrs::Privacy(()),
                70u16 => Nl80211Attrs::DisconnectedByAp(()),
                71u16 => Nl80211Attrs::StatusCode({
                    let res = parse_u16(next);
                    let Some(val) = res else { break };
                    val
                }),
                72u16 => Nl80211Attrs::CipherSuitesPairwise({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                73u16 => Nl80211Attrs::CipherSuiteGroup({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                74u16 => Nl80211Attrs::WpaVersions({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                75u16 => Nl80211Attrs::AkmSuites({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                76u16 => Nl80211Attrs::ReqIe({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                77u16 => Nl80211Attrs::RespIe({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                78u16 => Nl80211Attrs::PrevBssid({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                79u16 => Nl80211Attrs::Key({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                80u16 => Nl80211Attrs::Keys({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                81u16 => Nl80211Attrs::Pid({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                82u16 => Nl80211Attrs::_4addr({
                    let res = parse_u8(next);
                    let Some(val) = res else { break };
                    val
                }),
                83u16 => Nl80211Attrs::SurveyInfo({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                84u16 => Nl80211Attrs::Pmkid({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                85u16 => Nl80211Attrs::MaxNumPmkids({
                    let res = parse_u8(next);
                    let Some(val) = res else { break };
                    val
                }),
                86u16 => Nl80211Attrs::Duration({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                87u16 => Nl80211Attrs::Cookie({
                    let res = parse_u64(next);
                    let Some(val) = res else { break };
                    val
                }),
                88u16 => Nl80211Attrs::WiphyCoverageClass({
                    let res = parse_u8(next);
                    let Some(val) = res else { break };
                    val
                }),
                89u16 => Nl80211Attrs::TxRates({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                90u16 => Nl80211Attrs::FrameMatch({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                91u16 => Nl80211Attrs::Ack(()),
                92u16 => Nl80211Attrs::PsState({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                93u16 => Nl80211Attrs::Cqm({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                94u16 => Nl80211Attrs::LocalStateChange(()),
                95u16 => Nl80211Attrs::ApIsolate({
                    let res = parse_u8(next);
                    let Some(val) = res else { break };
                    val
                }),
                96u16 => Nl80211Attrs::WiphyTxPowerSetting({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                97u16 => Nl80211Attrs::WiphyTxPowerLevel({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                98u16 => Nl80211Attrs::TxFrameTypes({
                    let res = Some(IterableIftypeAttrs::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                99u16 => Nl80211Attrs::RxFrameTypes({
                    let res = Some(IterableIftypeAttrs::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                100u16 => Nl80211Attrs::FrameType({
                    let res = parse_u16(next);
                    let Some(val) = res else { break };
                    val
                }),
                101u16 => Nl80211Attrs::ControlPortEthertype(()),
                102u16 => Nl80211Attrs::ControlPortNoEncrypt(()),
                103u16 => Nl80211Attrs::SupportIbssRsn(()),
                104u16 => Nl80211Attrs::WiphyAntennaTx({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                105u16 => Nl80211Attrs::WiphyAntennaRx({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                106u16 => Nl80211Attrs::McastRate({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                107u16 => Nl80211Attrs::OffchannelTxOk(()),
                108u16 => Nl80211Attrs::BssHtOpmode({
                    let res = parse_u16(next);
                    let Some(val) = res else { break };
                    val
                }),
                109u16 => Nl80211Attrs::KeyDefaultTypes({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                110u16 => Nl80211Attrs::MaxRemainOnChannelDuration({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                111u16 => Nl80211Attrs::MeshSetup({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                112u16 => Nl80211Attrs::WiphyAntennaAvailTx({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                113u16 => Nl80211Attrs::WiphyAntennaAvailRx({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                114u16 => Nl80211Attrs::SupportMeshAuth(()),
                115u16 => Nl80211Attrs::StaPlinkState({
                    let res = parse_u8(next);
                    let Some(val) = res else { break };
                    val
                }),
                116u16 => Nl80211Attrs::WowlanTriggers({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                117u16 => Nl80211Attrs::WowlanTriggersSupported({
                    let res = Some(IterableWowlanTriggersAttrs::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                118u16 => Nl80211Attrs::SchedScanInterval({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                119u16 => Nl80211Attrs::InterfaceCombinations({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                120u16 => Nl80211Attrs::SoftwareIftypes({
                    let res = Some(IterableSupportedIftypes::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                121u16 => Nl80211Attrs::RekeyData({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                122u16 => Nl80211Attrs::MaxNumSchedScanSsids({
                    let res = parse_u8(next);
                    let Some(val) = res else { break };
                    val
                }),
                123u16 => Nl80211Attrs::MaxSchedScanIeLen({
                    let res = parse_u16(next);
                    let Some(val) = res else { break };
                    val
                }),
                124u16 => Nl80211Attrs::ScanSuppRates({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                125u16 => Nl80211Attrs::HiddenSsid({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                126u16 => Nl80211Attrs::IeProbeResp({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                127u16 => Nl80211Attrs::IeAssocResp({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                128u16 => Nl80211Attrs::StaWme({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                129u16 => Nl80211Attrs::SupportApUapsd(()),
                130u16 => Nl80211Attrs::RoamSupport(()),
                131u16 => Nl80211Attrs::SchedScanMatch({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                132u16 => Nl80211Attrs::MaxMatchSets({
                    let res = parse_u8(next);
                    let Some(val) = res else { break };
                    val
                }),
                133u16 => Nl80211Attrs::PmksaCandidate({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                134u16 => Nl80211Attrs::TxNoCckRate(()),
                135u16 => Nl80211Attrs::TdlsAction({
                    let res = parse_u8(next);
                    let Some(val) = res else { break };
                    val
                }),
                136u16 => Nl80211Attrs::TdlsDialogToken({
                    let res = parse_u8(next);
                    let Some(val) = res else { break };
                    val
                }),
                137u16 => Nl80211Attrs::TdlsOperation({
                    let res = parse_u8(next);
                    let Some(val) = res else { break };
                    val
                }),
                138u16 => Nl80211Attrs::TdlsSupport(()),
                139u16 => Nl80211Attrs::TdlsExternalSetup(()),
                140u16 => Nl80211Attrs::DeviceApSme({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                141u16 => Nl80211Attrs::DontWaitForAck(()),
                142u16 => Nl80211Attrs::FeatureFlags({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                143u16 => Nl80211Attrs::ProbeRespOffload({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                144u16 => Nl80211Attrs::ProbeResp({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                145u16 => Nl80211Attrs::DfsRegion({
                    let res = parse_u8(next);
                    let Some(val) = res else { break };
                    val
                }),
                146u16 => Nl80211Attrs::DisableHt(()),
                147u16 => Nl80211Attrs::HtCapabilityMask({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                148u16 => Nl80211Attrs::NoackMap({
                    let res = parse_u16(next);
                    let Some(val) = res else { break };
                    val
                }),
                149u16 => Nl80211Attrs::InactivityTimeout({
                    let res = parse_u16(next);
                    let Some(val) = res else { break };
                    val
                }),
                150u16 => Nl80211Attrs::RxSignalDbm({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                151u16 => Nl80211Attrs::BgScanPeriod({
                    let res = parse_u16(next);
                    let Some(val) = res else { break };
                    val
                }),
                152u16 => Nl80211Attrs::Wdev({
                    let res = parse_u64(next);
                    let Some(val) = res else { break };
                    val
                }),
                153u16 => Nl80211Attrs::UserRegHintType({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                154u16 => Nl80211Attrs::ConnFailedReason({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                155u16 => Nl80211Attrs::AuthData({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                156u16 => Nl80211Attrs::VhtCapability({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                157u16 => Nl80211Attrs::ScanFlags({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                158u16 => Nl80211Attrs::ChannelWidth({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                159u16 => Nl80211Attrs::CenterFreq1({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                160u16 => Nl80211Attrs::CenterFreq2({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                161u16 => Nl80211Attrs::P2pCtwindow({
                    let res = parse_u8(next);
                    let Some(val) = res else { break };
                    val
                }),
                162u16 => Nl80211Attrs::P2pOppps({
                    let res = parse_u8(next);
                    let Some(val) = res else { break };
                    val
                }),
                163u16 => Nl80211Attrs::LocalMeshPowerMode({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                164u16 => Nl80211Attrs::AclPolicy({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                165u16 => Nl80211Attrs::MacAddrs({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                166u16 => Nl80211Attrs::MacAclMax({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                167u16 => Nl80211Attrs::RadarEvent({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                168u16 => Nl80211Attrs::ExtCapa({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                169u16 => Nl80211Attrs::ExtCapaMask({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                170u16 => Nl80211Attrs::StaCapability({
                    let res = parse_u16(next);
                    let Some(val) = res else { break };
                    val
                }),
                171u16 => Nl80211Attrs::StaExtCapability({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                172u16 => Nl80211Attrs::ProtocolFeatures({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                173u16 => Nl80211Attrs::SplitWiphyDump(()),
                174u16 => Nl80211Attrs::DisableVht(()),
                175u16 => Nl80211Attrs::VhtCapabilityMask({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                176u16 => Nl80211Attrs::Mdid({
                    let res = parse_u16(next);
                    let Some(val) = res else { break };
                    val
                }),
                177u16 => Nl80211Attrs::IeRic({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                178u16 => Nl80211Attrs::CritProtId({
                    let res = parse_u16(next);
                    let Some(val) = res else { break };
                    val
                }),
                179u16 => Nl80211Attrs::MaxCritProtDuration({
                    let res = parse_u16(next);
                    let Some(val) = res else { break };
                    val
                }),
                180u16 => Nl80211Attrs::PeerAid({
                    let res = parse_u16(next);
                    let Some(val) = res else { break };
                    val
                }),
                181u16 => Nl80211Attrs::CoalesceRule({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                182u16 => Nl80211Attrs::ChSwitchCount({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                183u16 => Nl80211Attrs::ChSwitchBlockTx(()),
                184u16 => Nl80211Attrs::CsaIes({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                185u16 => Nl80211Attrs::CntdwnOffsBeacon({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                186u16 => Nl80211Attrs::CntdwnOffsPresp({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                187u16 => Nl80211Attrs::RxmgmtFlags({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                188u16 => Nl80211Attrs::StaSupportedChannels({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                189u16 => Nl80211Attrs::StaSupportedOperClasses({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                190u16 => Nl80211Attrs::HandleDfs(()),
                191u16 => Nl80211Attrs::Support5Mhz(()),
                192u16 => Nl80211Attrs::Support10Mhz(()),
                193u16 => Nl80211Attrs::OpmodeNotif({
                    let res = parse_u8(next);
                    let Some(val) = res else { break };
                    val
                }),
                194u16 => Nl80211Attrs::VendorId({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                195u16 => Nl80211Attrs::VendorSubcmd({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                196u16 => Nl80211Attrs::VendorData({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                197u16 => Nl80211Attrs::VendorEvents({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                198u16 => Nl80211Attrs::QosMap({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                199u16 => Nl80211Attrs::MacHint({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                200u16 => Nl80211Attrs::WiphyFreqHint({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                201u16 => Nl80211Attrs::MaxApAssocSta({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                202u16 => Nl80211Attrs::TdlsPeerCapability({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                203u16 => Nl80211Attrs::SocketOwner(()),
                204u16 => Nl80211Attrs::CsaCOffsetsTx({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                205u16 => Nl80211Attrs::MaxCsaCounters({
                    let res = parse_u8(next);
                    let Some(val) = res else { break };
                    val
                }),
                206u16 => Nl80211Attrs::TdlsInitiator(()),
                207u16 => Nl80211Attrs::UseRrm(()),
                208u16 => Nl80211Attrs::WiphyDynAck(()),
                209u16 => Nl80211Attrs::Tsid({
                    let res = parse_u8(next);
                    let Some(val) = res else { break };
                    val
                }),
                210u16 => Nl80211Attrs::UserPrio({
                    let res = parse_u8(next);
                    let Some(val) = res else { break };
                    val
                }),
                211u16 => Nl80211Attrs::AdmittedTime({
                    let res = parse_u16(next);
                    let Some(val) = res else { break };
                    val
                }),
                212u16 => Nl80211Attrs::SmpsMode({
                    let res = parse_u8(next);
                    let Some(val) = res else { break };
                    val
                }),
                213u16 => Nl80211Attrs::OperClass({
                    let res = parse_u8(next);
                    let Some(val) = res else { break };
                    val
                }),
                214u16 => Nl80211Attrs::MacMask({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                215u16 => Nl80211Attrs::WiphySelfManagedReg(()),
                216u16 => Nl80211Attrs::ExtFeatures({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                217u16 => Nl80211Attrs::SurveyRadioStats({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                218u16 => Nl80211Attrs::NetnsFd({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                219u16 => Nl80211Attrs::SchedScanDelay({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                220u16 => Nl80211Attrs::RegIndoor(()),
                221u16 => Nl80211Attrs::MaxNumSchedScanPlans({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                222u16 => Nl80211Attrs::MaxScanPlanInterval({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                223u16 => Nl80211Attrs::MaxScanPlanIterations({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                224u16 => Nl80211Attrs::SchedScanPlans({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                225u16 => Nl80211Attrs::Pbss(()),
                226u16 => Nl80211Attrs::BssSelect({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                227u16 => Nl80211Attrs::StaSupportP2pPs({
                    let res = parse_u8(next);
                    let Some(val) = res else { break };
                    val
                }),
                228u16 => Nl80211Attrs::Pad({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                229u16 => Nl80211Attrs::IftypeExtCapa({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                230u16 => Nl80211Attrs::MuMimoGroupData({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                231u16 => Nl80211Attrs::MuMimoFollowMacAddr({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                232u16 => Nl80211Attrs::ScanStartTimeTsf({
                    let res = parse_u64(next);
                    let Some(val) = res else { break };
                    val
                }),
                233u16 => Nl80211Attrs::ScanStartTimeTsfBssid({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                234u16 => Nl80211Attrs::MeasurementDuration({
                    let res = parse_u16(next);
                    let Some(val) = res else { break };
                    val
                }),
                235u16 => Nl80211Attrs::MeasurementDurationMandatory(()),
                236u16 => Nl80211Attrs::MeshPeerAid({
                    let res = parse_u16(next);
                    let Some(val) = res else { break };
                    val
                }),
                237u16 => Nl80211Attrs::NanMasterPref({
                    let res = parse_u8(next);
                    let Some(val) = res else { break };
                    val
                }),
                238u16 => Nl80211Attrs::Bands({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                239u16 => Nl80211Attrs::NanFunc({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                240u16 => Nl80211Attrs::NanMatch({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                241u16 => Nl80211Attrs::FilsKek({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                242u16 => Nl80211Attrs::FilsNonces({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                243u16 => Nl80211Attrs::MulticastToUnicastEnabled(()),
                244u16 => Nl80211Attrs::Bssid({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                245u16 => Nl80211Attrs::SchedScanRelativeRssi({
                    let res = parse_i8(next);
                    let Some(val) = res else { break };
                    val
                }),
                246u16 => Nl80211Attrs::SchedScanRssiAdjust({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                247u16 => Nl80211Attrs::TimeoutReason({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                248u16 => Nl80211Attrs::FilsErpUsername({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                249u16 => Nl80211Attrs::FilsErpRealm({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                250u16 => Nl80211Attrs::FilsErpNextSeqNum({
                    let res = parse_u16(next);
                    let Some(val) = res else { break };
                    val
                }),
                251u16 => Nl80211Attrs::FilsErpRrk({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                252u16 => Nl80211Attrs::FilsCacheId({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                253u16 => Nl80211Attrs::Pmk({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                254u16 => Nl80211Attrs::SchedScanMulti(()),
                255u16 => Nl80211Attrs::SchedScanMaxReqs({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                256u16 => Nl80211Attrs::Want1x4wayHs(()),
                257u16 => Nl80211Attrs::Pmkr0Name({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                258u16 => Nl80211Attrs::PortAuthorized({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                259u16 => Nl80211Attrs::ExternalAuthAction({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                260u16 => Nl80211Attrs::ExternalAuthSupport(()),
                261u16 => Nl80211Attrs::Nss({
                    let res = parse_u8(next);
                    let Some(val) = res else { break };
                    val
                }),
                262u16 => Nl80211Attrs::AckSignal({
                    let res = parse_i32(next);
                    let Some(val) = res else { break };
                    val
                }),
                263u16 => Nl80211Attrs::ControlPortOverNl80211(()),
                264u16 => Nl80211Attrs::TxqStats({
                    let res = Some(IterableTxqStatsAttrs::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                265u16 => Nl80211Attrs::TxqLimit({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                266u16 => Nl80211Attrs::TxqMemoryLimit({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                267u16 => Nl80211Attrs::TxqQuantum({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                268u16 => Nl80211Attrs::HeCapability({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                269u16 => Nl80211Attrs::FtmResponder({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                270u16 => Nl80211Attrs::FtmResponderStats({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                271u16 => Nl80211Attrs::Timeout({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                272u16 => Nl80211Attrs::PeerMeasurements({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                273u16 => Nl80211Attrs::AirtimeWeight({
                    let res = parse_u16(next);
                    let Some(val) = res else { break };
                    val
                }),
                274u16 => Nl80211Attrs::StaTxPowerSetting({
                    let res = parse_u8(next);
                    let Some(val) = res else { break };
                    val
                }),
                275u16 => Nl80211Attrs::StaTxPower({
                    let res = parse_i16(next);
                    let Some(val) = res else { break };
                    val
                }),
                276u16 => Nl80211Attrs::SaePassword({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                277u16 => Nl80211Attrs::TwtResponder(()),
                278u16 => Nl80211Attrs::HeObssPd({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                279u16 => Nl80211Attrs::WiphyEdmgChannels({
                    let res = parse_u8(next);
                    let Some(val) = res else { break };
                    val
                }),
                280u16 => Nl80211Attrs::WiphyEdmgBwConfig({
                    let res = parse_u8(next);
                    let Some(val) = res else { break };
                    val
                }),
                281u16 => Nl80211Attrs::VlanId({
                    let res = parse_u16(next);
                    let Some(val) = res else { break };
                    val
                }),
                282u16 => Nl80211Attrs::HeBssColor({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                283u16 => Nl80211Attrs::IftypeAkmSuites({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                284u16 => Nl80211Attrs::TidConfig({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                285u16 => Nl80211Attrs::ControlPortNoPreauth(()),
                286u16 => Nl80211Attrs::PmkLifetime({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                287u16 => Nl80211Attrs::PmkReauthThreshold({
                    let res = parse_u8(next);
                    let Some(val) = res else { break };
                    val
                }),
                288u16 => Nl80211Attrs::ReceiveMulticast(()),
                289u16 => Nl80211Attrs::WiphyFreqOffset({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                290u16 => Nl80211Attrs::CenterFreq1Offset({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                291u16 => Nl80211Attrs::ScanFreqKhz({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                292u16 => Nl80211Attrs::He6ghzCapability({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                293u16 => Nl80211Attrs::FilsDiscovery({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                294u16 => Nl80211Attrs::UnsolBcastProbeResp({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                295u16 => Nl80211Attrs::S1gCapability({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                296u16 => Nl80211Attrs::S1gCapabilityMask({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                297u16 => Nl80211Attrs::SaePwe({
                    let res = parse_u8(next);
                    let Some(val) = res else { break };
                    val
                }),
                298u16 => Nl80211Attrs::ReconnectRequested({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                299u16 => Nl80211Attrs::SarSpec({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                300u16 => Nl80211Attrs::DisableHe(()),
                301u16 => Nl80211Attrs::ObssColorBitmap({
                    let res = parse_u64(next);
                    let Some(val) = res else { break };
                    val
                }),
                302u16 => Nl80211Attrs::ColorChangeCount({
                    let res = parse_u8(next);
                    let Some(val) = res else { break };
                    val
                }),
                303u16 => Nl80211Attrs::ColorChangeColor({
                    let res = parse_u8(next);
                    let Some(val) = res else { break };
                    val
                }),
                304u16 => Nl80211Attrs::ColorChangeElems({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                305u16 => Nl80211Attrs::MbssidConfig({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                306u16 => Nl80211Attrs::MbssidElems({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                307u16 => Nl80211Attrs::RadarBackground(()),
                308u16 => Nl80211Attrs::ApSettingsFlags({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                309u16 => Nl80211Attrs::EhtCapability({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                310u16 => Nl80211Attrs::DisableEht(()),
                311u16 => Nl80211Attrs::MloLinks({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                312u16 => Nl80211Attrs::MloLinkId({
                    let res = parse_u8(next);
                    let Some(val) = res else { break };
                    val
                }),
                313u16 => Nl80211Attrs::MldAddr({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                314u16 => Nl80211Attrs::MloSupport(()),
                315u16 => Nl80211Attrs::MaxNumAkmSuites({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                316u16 => Nl80211Attrs::EmlCapability({
                    let res = parse_u16(next);
                    let Some(val) = res else { break };
                    val
                }),
                317u16 => Nl80211Attrs::MldCapaAndOps({
                    let res = parse_u16(next);
                    let Some(val) = res else { break };
                    val
                }),
                318u16 => Nl80211Attrs::TxHwTimestamp({
                    let res = parse_u64(next);
                    let Some(val) = res else { break };
                    val
                }),
                319u16 => Nl80211Attrs::RxHwTimestamp({
                    let res = parse_u64(next);
                    let Some(val) = res else { break };
                    val
                }),
                320u16 => Nl80211Attrs::TdBitmap({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                321u16 => Nl80211Attrs::PunctBitmap({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                322u16 => Nl80211Attrs::MaxHwTimestampPeers({
                    let res = parse_u16(next);
                    let Some(val) = res else { break };
                    val
                }),
                323u16 => Nl80211Attrs::HwTimestampEnabled(()),
                324u16 => Nl80211Attrs::EmaRnrElems({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                325u16 => Nl80211Attrs::MloLinkDisabled(()),
                326u16 => Nl80211Attrs::BssDumpIncludeUseData(()),
                327u16 => Nl80211Attrs::MloTtlmDlink({
                    let res = parse_u16(next);
                    let Some(val) = res else { break };
                    val
                }),
                328u16 => Nl80211Attrs::MloTtlmUlink({
                    let res = parse_u16(next);
                    let Some(val) = res else { break };
                    val
                }),
                329u16 => Nl80211Attrs::AssocSppAmsdu(()),
                330u16 => Nl80211Attrs::WiphyRadios({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                331u16 => Nl80211Attrs::WiphyInterfaceCombinations({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                332u16 => Nl80211Attrs::VifRadioMask({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                n => {
                    if cfg!(any(test, feature = "deny-unknown-attrs")) {
                        break;
                    } else {
                        continue;
                    }
                }
            };
            return Some(Ok(res));
        }
        Some(Err(ErrorContext::new(
            "Nl80211Attrs",
            r#type.and_then(|t| Nl80211Attrs::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl<'a> std::fmt::Debug for IterableNl80211Attrs<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("Nl80211Attrs");
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
                Nl80211Attrs::Wiphy(val) => fmt.field("Wiphy", &val),
                Nl80211Attrs::WiphyName(val) => fmt.field("WiphyName", &val),
                Nl80211Attrs::Ifindex(val) => fmt.field("Ifindex", &val),
                Nl80211Attrs::Ifname(val) => fmt.field("Ifname", &val),
                Nl80211Attrs::Iftype(val) => fmt.field("Iftype", &val),
                Nl80211Attrs::Mac(val) => fmt.field("Mac", &val),
                Nl80211Attrs::KeyData(val) => fmt.field("KeyData", &val),
                Nl80211Attrs::KeyIdx(val) => fmt.field("KeyIdx", &val),
                Nl80211Attrs::KeyCipher(val) => fmt.field("KeyCipher", &val),
                Nl80211Attrs::KeySeq(val) => fmt.field("KeySeq", &val),
                Nl80211Attrs::KeyDefault(val) => fmt.field("KeyDefault", &val),
                Nl80211Attrs::BeaconInterval(val) => fmt.field("BeaconInterval", &val),
                Nl80211Attrs::DtimPeriod(val) => fmt.field("DtimPeriod", &val),
                Nl80211Attrs::BeaconHead(val) => fmt.field("BeaconHead", &val),
                Nl80211Attrs::BeaconTail(val) => fmt.field("BeaconTail", &val),
                Nl80211Attrs::StaAid(val) => fmt.field("StaAid", &val),
                Nl80211Attrs::StaFlags(val) => fmt.field("StaFlags", &val),
                Nl80211Attrs::StaListenInterval(val) => fmt.field("StaListenInterval", &val),
                Nl80211Attrs::StaSupportedRates(val) => fmt.field("StaSupportedRates", &val),
                Nl80211Attrs::StaVlan(val) => fmt.field("StaVlan", &val),
                Nl80211Attrs::StaInfo(val) => fmt.field("StaInfo", &val),
                Nl80211Attrs::WiphyBands(val) => fmt.field("WiphyBands", &val),
                Nl80211Attrs::MntrFlags(val) => fmt.field("MntrFlags", &val),
                Nl80211Attrs::MeshId(val) => fmt.field("MeshId", &val),
                Nl80211Attrs::StaPlinkAction(val) => fmt.field("StaPlinkAction", &val),
                Nl80211Attrs::MpathNextHop(val) => fmt.field("MpathNextHop", &val),
                Nl80211Attrs::MpathInfo(val) => fmt.field("MpathInfo", &val),
                Nl80211Attrs::BssCtsProt(val) => fmt.field("BssCtsProt", &val),
                Nl80211Attrs::BssShortPreamble(val) => fmt.field("BssShortPreamble", &val),
                Nl80211Attrs::BssShortSlotTime(val) => fmt.field("BssShortSlotTime", &val),
                Nl80211Attrs::HtCapability(val) => fmt.field("HtCapability", &val),
                Nl80211Attrs::SupportedIftypes(val) => fmt.field("SupportedIftypes", &val),
                Nl80211Attrs::RegAlpha2(val) => fmt.field("RegAlpha2", &val),
                Nl80211Attrs::RegRules(val) => fmt.field("RegRules", &val),
                Nl80211Attrs::MeshConfig(val) => fmt.field("MeshConfig", &val),
                Nl80211Attrs::BssBasicRates(val) => fmt.field("BssBasicRates", &val),
                Nl80211Attrs::WiphyTxqParams(val) => fmt.field("WiphyTxqParams", &val),
                Nl80211Attrs::WiphyFreq(val) => fmt.field("WiphyFreq", &val),
                Nl80211Attrs::WiphyChannelType(val) => fmt.field(
                    "WiphyChannelType",
                    &FormatEnum(val.into(), ChannelType::from_value),
                ),
                Nl80211Attrs::KeyDefaultMgmt(val) => fmt.field("KeyDefaultMgmt", &val),
                Nl80211Attrs::MgmtSubtype(val) => fmt.field("MgmtSubtype", &val),
                Nl80211Attrs::Ie(val) => fmt.field("Ie", &val),
                Nl80211Attrs::MaxNumScanSsids(val) => fmt.field("MaxNumScanSsids", &val),
                Nl80211Attrs::ScanFrequencies(val) => fmt.field("ScanFrequencies", &val),
                Nl80211Attrs::ScanSsids(val) => fmt.field("ScanSsids", &val),
                Nl80211Attrs::Generation(val) => fmt.field("Generation", &val),
                Nl80211Attrs::Bss(val) => fmt.field("Bss", &val),
                Nl80211Attrs::RegInitiator(val) => fmt.field("RegInitiator", &val),
                Nl80211Attrs::RegType(val) => fmt.field("RegType", &val),
                Nl80211Attrs::Frame(val) => fmt.field("Frame", &val),
                Nl80211Attrs::Ssid(val) => fmt.field("Ssid", &val),
                Nl80211Attrs::AuthType(val) => fmt.field("AuthType", &val),
                Nl80211Attrs::ReasonCode(val) => fmt.field("ReasonCode", &val),
                Nl80211Attrs::KeyType(val) => fmt.field("KeyType", &val),
                Nl80211Attrs::MaxScanIeLen(val) => fmt.field("MaxScanIeLen", &val),
                Nl80211Attrs::CipherSuites(val) => fmt.field("CipherSuites", &FormatHex(val)),
                Nl80211Attrs::FreqBefore(val) => fmt.field("FreqBefore", &val),
                Nl80211Attrs::FreqAfter(val) => fmt.field("FreqAfter", &val),
                Nl80211Attrs::FreqFixed(val) => fmt.field("FreqFixed", &val),
                Nl80211Attrs::WiphyRetryShort(val) => fmt.field("WiphyRetryShort", &val),
                Nl80211Attrs::WiphyRetryLong(val) => fmt.field("WiphyRetryLong", &val),
                Nl80211Attrs::WiphyFragThreshold(val) => fmt.field("WiphyFragThreshold", &val),
                Nl80211Attrs::WiphyRtsThreshold(val) => fmt.field("WiphyRtsThreshold", &val),
                Nl80211Attrs::TimedOut(val) => fmt.field("TimedOut", &val),
                Nl80211Attrs::UseMfp(val) => fmt.field("UseMfp", &val),
                Nl80211Attrs::StaFlags2(val) => fmt.field("StaFlags2", &val),
                Nl80211Attrs::ControlPort(val) => fmt.field("ControlPort", &val),
                Nl80211Attrs::Testdata(val) => fmt.field("Testdata", &val),
                Nl80211Attrs::Privacy(val) => fmt.field("Privacy", &val),
                Nl80211Attrs::DisconnectedByAp(val) => fmt.field("DisconnectedByAp", &val),
                Nl80211Attrs::StatusCode(val) => fmt.field("StatusCode", &val),
                Nl80211Attrs::CipherSuitesPairwise(val) => fmt.field("CipherSuitesPairwise", &val),
                Nl80211Attrs::CipherSuiteGroup(val) => fmt.field("CipherSuiteGroup", &val),
                Nl80211Attrs::WpaVersions(val) => fmt.field("WpaVersions", &val),
                Nl80211Attrs::AkmSuites(val) => fmt.field("AkmSuites", &val),
                Nl80211Attrs::ReqIe(val) => fmt.field("ReqIe", &val),
                Nl80211Attrs::RespIe(val) => fmt.field("RespIe", &val),
                Nl80211Attrs::PrevBssid(val) => fmt.field("PrevBssid", &val),
                Nl80211Attrs::Key(val) => fmt.field("Key", &val),
                Nl80211Attrs::Keys(val) => fmt.field("Keys", &val),
                Nl80211Attrs::Pid(val) => fmt.field("Pid", &val),
                Nl80211Attrs::_4addr(val) => fmt.field("_4addr", &val),
                Nl80211Attrs::SurveyInfo(val) => fmt.field("SurveyInfo", &val),
                Nl80211Attrs::Pmkid(val) => fmt.field("Pmkid", &val),
                Nl80211Attrs::MaxNumPmkids(val) => fmt.field("MaxNumPmkids", &val),
                Nl80211Attrs::Duration(val) => fmt.field("Duration", &val),
                Nl80211Attrs::Cookie(val) => fmt.field("Cookie", &val),
                Nl80211Attrs::WiphyCoverageClass(val) => fmt.field("WiphyCoverageClass", &val),
                Nl80211Attrs::TxRates(val) => fmt.field("TxRates", &val),
                Nl80211Attrs::FrameMatch(val) => fmt.field("FrameMatch", &val),
                Nl80211Attrs::Ack(val) => fmt.field("Ack", &val),
                Nl80211Attrs::PsState(val) => fmt.field("PsState", &val),
                Nl80211Attrs::Cqm(val) => fmt.field("Cqm", &val),
                Nl80211Attrs::LocalStateChange(val) => fmt.field("LocalStateChange", &val),
                Nl80211Attrs::ApIsolate(val) => fmt.field("ApIsolate", &val),
                Nl80211Attrs::WiphyTxPowerSetting(val) => fmt.field("WiphyTxPowerSetting", &val),
                Nl80211Attrs::WiphyTxPowerLevel(val) => fmt.field("WiphyTxPowerLevel", &val),
                Nl80211Attrs::TxFrameTypes(val) => fmt.field("TxFrameTypes", &val),
                Nl80211Attrs::RxFrameTypes(val) => fmt.field("RxFrameTypes", &val),
                Nl80211Attrs::FrameType(val) => fmt.field("FrameType", &val),
                Nl80211Attrs::ControlPortEthertype(val) => fmt.field("ControlPortEthertype", &val),
                Nl80211Attrs::ControlPortNoEncrypt(val) => fmt.field("ControlPortNoEncrypt", &val),
                Nl80211Attrs::SupportIbssRsn(val) => fmt.field("SupportIbssRsn", &val),
                Nl80211Attrs::WiphyAntennaTx(val) => fmt.field("WiphyAntennaTx", &val),
                Nl80211Attrs::WiphyAntennaRx(val) => fmt.field("WiphyAntennaRx", &val),
                Nl80211Attrs::McastRate(val) => fmt.field("McastRate", &val),
                Nl80211Attrs::OffchannelTxOk(val) => fmt.field("OffchannelTxOk", &val),
                Nl80211Attrs::BssHtOpmode(val) => fmt.field("BssHtOpmode", &val),
                Nl80211Attrs::KeyDefaultTypes(val) => fmt.field("KeyDefaultTypes", &val),
                Nl80211Attrs::MaxRemainOnChannelDuration(val) => {
                    fmt.field("MaxRemainOnChannelDuration", &val)
                }
                Nl80211Attrs::MeshSetup(val) => fmt.field("MeshSetup", &val),
                Nl80211Attrs::WiphyAntennaAvailTx(val) => fmt.field("WiphyAntennaAvailTx", &val),
                Nl80211Attrs::WiphyAntennaAvailRx(val) => fmt.field("WiphyAntennaAvailRx", &val),
                Nl80211Attrs::SupportMeshAuth(val) => fmt.field("SupportMeshAuth", &val),
                Nl80211Attrs::StaPlinkState(val) => fmt.field("StaPlinkState", &val),
                Nl80211Attrs::WowlanTriggers(val) => fmt.field("WowlanTriggers", &val),
                Nl80211Attrs::WowlanTriggersSupported(val) => {
                    fmt.field("WowlanTriggersSupported", &val)
                }
                Nl80211Attrs::SchedScanInterval(val) => fmt.field("SchedScanInterval", &val),
                Nl80211Attrs::InterfaceCombinations(val) => {
                    fmt.field("InterfaceCombinations", &val)
                }
                Nl80211Attrs::SoftwareIftypes(val) => fmt.field("SoftwareIftypes", &val),
                Nl80211Attrs::RekeyData(val) => fmt.field("RekeyData", &val),
                Nl80211Attrs::MaxNumSchedScanSsids(val) => fmt.field("MaxNumSchedScanSsids", &val),
                Nl80211Attrs::MaxSchedScanIeLen(val) => fmt.field("MaxSchedScanIeLen", &val),
                Nl80211Attrs::ScanSuppRates(val) => fmt.field("ScanSuppRates", &val),
                Nl80211Attrs::HiddenSsid(val) => fmt.field("HiddenSsid", &val),
                Nl80211Attrs::IeProbeResp(val) => fmt.field("IeProbeResp", &val),
                Nl80211Attrs::IeAssocResp(val) => fmt.field("IeAssocResp", &val),
                Nl80211Attrs::StaWme(val) => fmt.field("StaWme", &val),
                Nl80211Attrs::SupportApUapsd(val) => fmt.field("SupportApUapsd", &val),
                Nl80211Attrs::RoamSupport(val) => fmt.field("RoamSupport", &val),
                Nl80211Attrs::SchedScanMatch(val) => fmt.field("SchedScanMatch", &val),
                Nl80211Attrs::MaxMatchSets(val) => fmt.field("MaxMatchSets", &val),
                Nl80211Attrs::PmksaCandidate(val) => fmt.field("PmksaCandidate", &val),
                Nl80211Attrs::TxNoCckRate(val) => fmt.field("TxNoCckRate", &val),
                Nl80211Attrs::TdlsAction(val) => fmt.field("TdlsAction", &val),
                Nl80211Attrs::TdlsDialogToken(val) => fmt.field("TdlsDialogToken", &val),
                Nl80211Attrs::TdlsOperation(val) => fmt.field("TdlsOperation", &val),
                Nl80211Attrs::TdlsSupport(val) => fmt.field("TdlsSupport", &val),
                Nl80211Attrs::TdlsExternalSetup(val) => fmt.field("TdlsExternalSetup", &val),
                Nl80211Attrs::DeviceApSme(val) => fmt.field("DeviceApSme", &val),
                Nl80211Attrs::DontWaitForAck(val) => fmt.field("DontWaitForAck", &val),
                Nl80211Attrs::FeatureFlags(val) => fmt.field(
                    "FeatureFlags",
                    &FormatFlags(val.into(), FeatureFlags::from_value),
                ),
                Nl80211Attrs::ProbeRespOffload(val) => fmt.field("ProbeRespOffload", &val),
                Nl80211Attrs::ProbeResp(val) => fmt.field("ProbeResp", &val),
                Nl80211Attrs::DfsRegion(val) => fmt.field("DfsRegion", &val),
                Nl80211Attrs::DisableHt(val) => fmt.field("DisableHt", &val),
                Nl80211Attrs::HtCapabilityMask(val) => fmt.field("HtCapabilityMask", &val),
                Nl80211Attrs::NoackMap(val) => fmt.field("NoackMap", &val),
                Nl80211Attrs::InactivityTimeout(val) => fmt.field("InactivityTimeout", &val),
                Nl80211Attrs::RxSignalDbm(val) => fmt.field("RxSignalDbm", &val),
                Nl80211Attrs::BgScanPeriod(val) => fmt.field("BgScanPeriod", &val),
                Nl80211Attrs::Wdev(val) => fmt.field("Wdev", &val),
                Nl80211Attrs::UserRegHintType(val) => fmt.field("UserRegHintType", &val),
                Nl80211Attrs::ConnFailedReason(val) => fmt.field("ConnFailedReason", &val),
                Nl80211Attrs::AuthData(val) => fmt.field("AuthData", &val),
                Nl80211Attrs::VhtCapability(val) => fmt.field("VhtCapability", &val),
                Nl80211Attrs::ScanFlags(val) => fmt.field("ScanFlags", &val),
                Nl80211Attrs::ChannelWidth(val) => fmt.field("ChannelWidth", &val),
                Nl80211Attrs::CenterFreq1(val) => fmt.field("CenterFreq1", &val),
                Nl80211Attrs::CenterFreq2(val) => fmt.field("CenterFreq2", &val),
                Nl80211Attrs::P2pCtwindow(val) => fmt.field("P2pCtwindow", &val),
                Nl80211Attrs::P2pOppps(val) => fmt.field("P2pOppps", &val),
                Nl80211Attrs::LocalMeshPowerMode(val) => fmt.field("LocalMeshPowerMode", &val),
                Nl80211Attrs::AclPolicy(val) => fmt.field("AclPolicy", &val),
                Nl80211Attrs::MacAddrs(val) => fmt.field("MacAddrs", &val),
                Nl80211Attrs::MacAclMax(val) => fmt.field("MacAclMax", &val),
                Nl80211Attrs::RadarEvent(val) => fmt.field("RadarEvent", &val),
                Nl80211Attrs::ExtCapa(val) => fmt.field("ExtCapa", &val),
                Nl80211Attrs::ExtCapaMask(val) => fmt.field("ExtCapaMask", &val),
                Nl80211Attrs::StaCapability(val) => fmt.field("StaCapability", &val),
                Nl80211Attrs::StaExtCapability(val) => fmt.field("StaExtCapability", &val),
                Nl80211Attrs::ProtocolFeatures(val) => fmt.field(
                    "ProtocolFeatures",
                    &FormatFlags(val.into(), ProtocolFeatures::from_value),
                ),
                Nl80211Attrs::SplitWiphyDump(val) => fmt.field("SplitWiphyDump", &val),
                Nl80211Attrs::DisableVht(val) => fmt.field("DisableVht", &val),
                Nl80211Attrs::VhtCapabilityMask(val) => fmt.field("VhtCapabilityMask", &val),
                Nl80211Attrs::Mdid(val) => fmt.field("Mdid", &val),
                Nl80211Attrs::IeRic(val) => fmt.field("IeRic", &val),
                Nl80211Attrs::CritProtId(val) => fmt.field("CritProtId", &val),
                Nl80211Attrs::MaxCritProtDuration(val) => fmt.field("MaxCritProtDuration", &val),
                Nl80211Attrs::PeerAid(val) => fmt.field("PeerAid", &val),
                Nl80211Attrs::CoalesceRule(val) => fmt.field("CoalesceRule", &val),
                Nl80211Attrs::ChSwitchCount(val) => fmt.field("ChSwitchCount", &val),
                Nl80211Attrs::ChSwitchBlockTx(val) => fmt.field("ChSwitchBlockTx", &val),
                Nl80211Attrs::CsaIes(val) => fmt.field("CsaIes", &val),
                Nl80211Attrs::CntdwnOffsBeacon(val) => fmt.field("CntdwnOffsBeacon", &val),
                Nl80211Attrs::CntdwnOffsPresp(val) => fmt.field("CntdwnOffsPresp", &val),
                Nl80211Attrs::RxmgmtFlags(val) => fmt.field("RxmgmtFlags", &val),
                Nl80211Attrs::StaSupportedChannels(val) => fmt.field("StaSupportedChannels", &val),
                Nl80211Attrs::StaSupportedOperClasses(val) => {
                    fmt.field("StaSupportedOperClasses", &val)
                }
                Nl80211Attrs::HandleDfs(val) => fmt.field("HandleDfs", &val),
                Nl80211Attrs::Support5Mhz(val) => fmt.field("Support5Mhz", &val),
                Nl80211Attrs::Support10Mhz(val) => fmt.field("Support10Mhz", &val),
                Nl80211Attrs::OpmodeNotif(val) => fmt.field("OpmodeNotif", &val),
                Nl80211Attrs::VendorId(val) => fmt.field("VendorId", &val),
                Nl80211Attrs::VendorSubcmd(val) => fmt.field("VendorSubcmd", &val),
                Nl80211Attrs::VendorData(val) => fmt.field("VendorData", &val),
                Nl80211Attrs::VendorEvents(val) => fmt.field("VendorEvents", &val),
                Nl80211Attrs::QosMap(val) => fmt.field("QosMap", &val),
                Nl80211Attrs::MacHint(val) => fmt.field("MacHint", &val),
                Nl80211Attrs::WiphyFreqHint(val) => fmt.field("WiphyFreqHint", &val),
                Nl80211Attrs::MaxApAssocSta(val) => fmt.field("MaxApAssocSta", &val),
                Nl80211Attrs::TdlsPeerCapability(val) => fmt.field("TdlsPeerCapability", &val),
                Nl80211Attrs::SocketOwner(val) => fmt.field("SocketOwner", &val),
                Nl80211Attrs::CsaCOffsetsTx(val) => fmt.field("CsaCOffsetsTx", &val),
                Nl80211Attrs::MaxCsaCounters(val) => fmt.field("MaxCsaCounters", &val),
                Nl80211Attrs::TdlsInitiator(val) => fmt.field("TdlsInitiator", &val),
                Nl80211Attrs::UseRrm(val) => fmt.field("UseRrm", &val),
                Nl80211Attrs::WiphyDynAck(val) => fmt.field("WiphyDynAck", &val),
                Nl80211Attrs::Tsid(val) => fmt.field("Tsid", &val),
                Nl80211Attrs::UserPrio(val) => fmt.field("UserPrio", &val),
                Nl80211Attrs::AdmittedTime(val) => fmt.field("AdmittedTime", &val),
                Nl80211Attrs::SmpsMode(val) => fmt.field("SmpsMode", &val),
                Nl80211Attrs::OperClass(val) => fmt.field("OperClass", &val),
                Nl80211Attrs::MacMask(val) => fmt.field("MacMask", &val),
                Nl80211Attrs::WiphySelfManagedReg(val) => fmt.field("WiphySelfManagedReg", &val),
                Nl80211Attrs::ExtFeatures(val) => fmt.field("ExtFeatures", &val),
                Nl80211Attrs::SurveyRadioStats(val) => fmt.field("SurveyRadioStats", &val),
                Nl80211Attrs::NetnsFd(val) => fmt.field("NetnsFd", &val),
                Nl80211Attrs::SchedScanDelay(val) => fmt.field("SchedScanDelay", &val),
                Nl80211Attrs::RegIndoor(val) => fmt.field("RegIndoor", &val),
                Nl80211Attrs::MaxNumSchedScanPlans(val) => fmt.field("MaxNumSchedScanPlans", &val),
                Nl80211Attrs::MaxScanPlanInterval(val) => fmt.field("MaxScanPlanInterval", &val),
                Nl80211Attrs::MaxScanPlanIterations(val) => {
                    fmt.field("MaxScanPlanIterations", &val)
                }
                Nl80211Attrs::SchedScanPlans(val) => fmt.field("SchedScanPlans", &val),
                Nl80211Attrs::Pbss(val) => fmt.field("Pbss", &val),
                Nl80211Attrs::BssSelect(val) => fmt.field("BssSelect", &val),
                Nl80211Attrs::StaSupportP2pPs(val) => fmt.field("StaSupportP2pPs", &val),
                Nl80211Attrs::Pad(val) => fmt.field("Pad", &val),
                Nl80211Attrs::IftypeExtCapa(val) => fmt.field("IftypeExtCapa", &val),
                Nl80211Attrs::MuMimoGroupData(val) => fmt.field("MuMimoGroupData", &val),
                Nl80211Attrs::MuMimoFollowMacAddr(val) => fmt.field("MuMimoFollowMacAddr", &val),
                Nl80211Attrs::ScanStartTimeTsf(val) => fmt.field("ScanStartTimeTsf", &val),
                Nl80211Attrs::ScanStartTimeTsfBssid(val) => {
                    fmt.field("ScanStartTimeTsfBssid", &val)
                }
                Nl80211Attrs::MeasurementDuration(val) => fmt.field("MeasurementDuration", &val),
                Nl80211Attrs::MeasurementDurationMandatory(val) => {
                    fmt.field("MeasurementDurationMandatory", &val)
                }
                Nl80211Attrs::MeshPeerAid(val) => fmt.field("MeshPeerAid", &val),
                Nl80211Attrs::NanMasterPref(val) => fmt.field("NanMasterPref", &val),
                Nl80211Attrs::Bands(val) => fmt.field("Bands", &val),
                Nl80211Attrs::NanFunc(val) => fmt.field("NanFunc", &val),
                Nl80211Attrs::NanMatch(val) => fmt.field("NanMatch", &val),
                Nl80211Attrs::FilsKek(val) => fmt.field("FilsKek", &val),
                Nl80211Attrs::FilsNonces(val) => fmt.field("FilsNonces", &val),
                Nl80211Attrs::MulticastToUnicastEnabled(val) => {
                    fmt.field("MulticastToUnicastEnabled", &val)
                }
                Nl80211Attrs::Bssid(val) => fmt.field("Bssid", &val),
                Nl80211Attrs::SchedScanRelativeRssi(val) => {
                    fmt.field("SchedScanRelativeRssi", &val)
                }
                Nl80211Attrs::SchedScanRssiAdjust(val) => fmt.field("SchedScanRssiAdjust", &val),
                Nl80211Attrs::TimeoutReason(val) => fmt.field("TimeoutReason", &val),
                Nl80211Attrs::FilsErpUsername(val) => fmt.field("FilsErpUsername", &val),
                Nl80211Attrs::FilsErpRealm(val) => fmt.field("FilsErpRealm", &val),
                Nl80211Attrs::FilsErpNextSeqNum(val) => fmt.field("FilsErpNextSeqNum", &val),
                Nl80211Attrs::FilsErpRrk(val) => fmt.field("FilsErpRrk", &val),
                Nl80211Attrs::FilsCacheId(val) => fmt.field("FilsCacheId", &val),
                Nl80211Attrs::Pmk(val) => fmt.field("Pmk", &val),
                Nl80211Attrs::SchedScanMulti(val) => fmt.field("SchedScanMulti", &val),
                Nl80211Attrs::SchedScanMaxReqs(val) => fmt.field("SchedScanMaxReqs", &val),
                Nl80211Attrs::Want1x4wayHs(val) => fmt.field("Want1x4wayHs", &val),
                Nl80211Attrs::Pmkr0Name(val) => fmt.field("Pmkr0Name", &val),
                Nl80211Attrs::PortAuthorized(val) => fmt.field("PortAuthorized", &val),
                Nl80211Attrs::ExternalAuthAction(val) => fmt.field("ExternalAuthAction", &val),
                Nl80211Attrs::ExternalAuthSupport(val) => fmt.field("ExternalAuthSupport", &val),
                Nl80211Attrs::Nss(val) => fmt.field("Nss", &val),
                Nl80211Attrs::AckSignal(val) => fmt.field("AckSignal", &val),
                Nl80211Attrs::ControlPortOverNl80211(val) => {
                    fmt.field("ControlPortOverNl80211", &val)
                }
                Nl80211Attrs::TxqStats(val) => fmt.field("TxqStats", &val),
                Nl80211Attrs::TxqLimit(val) => fmt.field("TxqLimit", &val),
                Nl80211Attrs::TxqMemoryLimit(val) => fmt.field("TxqMemoryLimit", &val),
                Nl80211Attrs::TxqQuantum(val) => fmt.field("TxqQuantum", &val),
                Nl80211Attrs::HeCapability(val) => fmt.field("HeCapability", &val),
                Nl80211Attrs::FtmResponder(val) => fmt.field("FtmResponder", &val),
                Nl80211Attrs::FtmResponderStats(val) => fmt.field("FtmResponderStats", &val),
                Nl80211Attrs::Timeout(val) => fmt.field("Timeout", &val),
                Nl80211Attrs::PeerMeasurements(val) => fmt.field("PeerMeasurements", &val),
                Nl80211Attrs::AirtimeWeight(val) => fmt.field("AirtimeWeight", &val),
                Nl80211Attrs::StaTxPowerSetting(val) => fmt.field("StaTxPowerSetting", &val),
                Nl80211Attrs::StaTxPower(val) => fmt.field("StaTxPower", &val),
                Nl80211Attrs::SaePassword(val) => fmt.field("SaePassword", &val),
                Nl80211Attrs::TwtResponder(val) => fmt.field("TwtResponder", &val),
                Nl80211Attrs::HeObssPd(val) => fmt.field("HeObssPd", &val),
                Nl80211Attrs::WiphyEdmgChannels(val) => fmt.field("WiphyEdmgChannels", &val),
                Nl80211Attrs::WiphyEdmgBwConfig(val) => fmt.field("WiphyEdmgBwConfig", &val),
                Nl80211Attrs::VlanId(val) => fmt.field("VlanId", &val),
                Nl80211Attrs::HeBssColor(val) => fmt.field("HeBssColor", &val),
                Nl80211Attrs::IftypeAkmSuites(val) => fmt.field("IftypeAkmSuites", &val),
                Nl80211Attrs::TidConfig(val) => fmt.field("TidConfig", &val),
                Nl80211Attrs::ControlPortNoPreauth(val) => fmt.field("ControlPortNoPreauth", &val),
                Nl80211Attrs::PmkLifetime(val) => fmt.field("PmkLifetime", &val),
                Nl80211Attrs::PmkReauthThreshold(val) => fmt.field("PmkReauthThreshold", &val),
                Nl80211Attrs::ReceiveMulticast(val) => fmt.field("ReceiveMulticast", &val),
                Nl80211Attrs::WiphyFreqOffset(val) => fmt.field("WiphyFreqOffset", &val),
                Nl80211Attrs::CenterFreq1Offset(val) => fmt.field("CenterFreq1Offset", &val),
                Nl80211Attrs::ScanFreqKhz(val) => fmt.field("ScanFreqKhz", &val),
                Nl80211Attrs::He6ghzCapability(val) => fmt.field("He6ghzCapability", &val),
                Nl80211Attrs::FilsDiscovery(val) => fmt.field("FilsDiscovery", &val),
                Nl80211Attrs::UnsolBcastProbeResp(val) => fmt.field("UnsolBcastProbeResp", &val),
                Nl80211Attrs::S1gCapability(val) => fmt.field("S1gCapability", &val),
                Nl80211Attrs::S1gCapabilityMask(val) => fmt.field("S1gCapabilityMask", &val),
                Nl80211Attrs::SaePwe(val) => fmt.field("SaePwe", &val),
                Nl80211Attrs::ReconnectRequested(val) => fmt.field("ReconnectRequested", &val),
                Nl80211Attrs::SarSpec(val) => fmt.field("SarSpec", &val),
                Nl80211Attrs::DisableHe(val) => fmt.field("DisableHe", &val),
                Nl80211Attrs::ObssColorBitmap(val) => fmt.field("ObssColorBitmap", &val),
                Nl80211Attrs::ColorChangeCount(val) => fmt.field("ColorChangeCount", &val),
                Nl80211Attrs::ColorChangeColor(val) => fmt.field("ColorChangeColor", &val),
                Nl80211Attrs::ColorChangeElems(val) => fmt.field("ColorChangeElems", &val),
                Nl80211Attrs::MbssidConfig(val) => fmt.field("MbssidConfig", &val),
                Nl80211Attrs::MbssidElems(val) => fmt.field("MbssidElems", &val),
                Nl80211Attrs::RadarBackground(val) => fmt.field("RadarBackground", &val),
                Nl80211Attrs::ApSettingsFlags(val) => fmt.field("ApSettingsFlags", &val),
                Nl80211Attrs::EhtCapability(val) => fmt.field("EhtCapability", &val),
                Nl80211Attrs::DisableEht(val) => fmt.field("DisableEht", &val),
                Nl80211Attrs::MloLinks(val) => fmt.field("MloLinks", &val),
                Nl80211Attrs::MloLinkId(val) => fmt.field("MloLinkId", &val),
                Nl80211Attrs::MldAddr(val) => fmt.field("MldAddr", &val),
                Nl80211Attrs::MloSupport(val) => fmt.field("MloSupport", &val),
                Nl80211Attrs::MaxNumAkmSuites(val) => fmt.field("MaxNumAkmSuites", &val),
                Nl80211Attrs::EmlCapability(val) => fmt.field("EmlCapability", &val),
                Nl80211Attrs::MldCapaAndOps(val) => fmt.field("MldCapaAndOps", &val),
                Nl80211Attrs::TxHwTimestamp(val) => fmt.field("TxHwTimestamp", &val),
                Nl80211Attrs::RxHwTimestamp(val) => fmt.field("RxHwTimestamp", &val),
                Nl80211Attrs::TdBitmap(val) => fmt.field("TdBitmap", &val),
                Nl80211Attrs::PunctBitmap(val) => fmt.field("PunctBitmap", &val),
                Nl80211Attrs::MaxHwTimestampPeers(val) => fmt.field("MaxHwTimestampPeers", &val),
                Nl80211Attrs::HwTimestampEnabled(val) => fmt.field("HwTimestampEnabled", &val),
                Nl80211Attrs::EmaRnrElems(val) => fmt.field("EmaRnrElems", &val),
                Nl80211Attrs::MloLinkDisabled(val) => fmt.field("MloLinkDisabled", &val),
                Nl80211Attrs::BssDumpIncludeUseData(val) => {
                    fmt.field("BssDumpIncludeUseData", &val)
                }
                Nl80211Attrs::MloTtlmDlink(val) => fmt.field("MloTtlmDlink", &val),
                Nl80211Attrs::MloTtlmUlink(val) => fmt.field("MloTtlmUlink", &val),
                Nl80211Attrs::AssocSppAmsdu(val) => fmt.field("AssocSppAmsdu", &val),
                Nl80211Attrs::WiphyRadios(val) => fmt.field("WiphyRadios", &val),
                Nl80211Attrs::WiphyInterfaceCombinations(val) => {
                    fmt.field("WiphyInterfaceCombinations", &val)
                }
                Nl80211Attrs::VifRadioMask(val) => fmt.field("VifRadioMask", &val),
            };
        }
        fmt.finish()
    }
}
impl IterableNl80211Attrs<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if cur == offset {
            stack.push(("Nl80211Attrs", offset));
            return (
                stack,
                missing_type.and_then(|t| Nl80211Attrs::attr_from_type(t)),
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
                Nl80211Attrs::Wiphy(val) => {
                    if last_off == offset {
                        stack.push(("Wiphy", last_off));
                        break;
                    }
                }
                Nl80211Attrs::WiphyName(val) => {
                    if last_off == offset {
                        stack.push(("WiphyName", last_off));
                        break;
                    }
                }
                Nl80211Attrs::Ifindex(val) => {
                    if last_off == offset {
                        stack.push(("Ifindex", last_off));
                        break;
                    }
                }
                Nl80211Attrs::Ifname(val) => {
                    if last_off == offset {
                        stack.push(("Ifname", last_off));
                        break;
                    }
                }
                Nl80211Attrs::Iftype(val) => {
                    if last_off == offset {
                        stack.push(("Iftype", last_off));
                        break;
                    }
                }
                Nl80211Attrs::Mac(val) => {
                    if last_off == offset {
                        stack.push(("Mac", last_off));
                        break;
                    }
                }
                Nl80211Attrs::KeyData(val) => {
                    if last_off == offset {
                        stack.push(("KeyData", last_off));
                        break;
                    }
                }
                Nl80211Attrs::KeyIdx(val) => {
                    if last_off == offset {
                        stack.push(("KeyIdx", last_off));
                        break;
                    }
                }
                Nl80211Attrs::KeyCipher(val) => {
                    if last_off == offset {
                        stack.push(("KeyCipher", last_off));
                        break;
                    }
                }
                Nl80211Attrs::KeySeq(val) => {
                    if last_off == offset {
                        stack.push(("KeySeq", last_off));
                        break;
                    }
                }
                Nl80211Attrs::KeyDefault(val) => {
                    if last_off == offset {
                        stack.push(("KeyDefault", last_off));
                        break;
                    }
                }
                Nl80211Attrs::BeaconInterval(val) => {
                    if last_off == offset {
                        stack.push(("BeaconInterval", last_off));
                        break;
                    }
                }
                Nl80211Attrs::DtimPeriod(val) => {
                    if last_off == offset {
                        stack.push(("DtimPeriod", last_off));
                        break;
                    }
                }
                Nl80211Attrs::BeaconHead(val) => {
                    if last_off == offset {
                        stack.push(("BeaconHead", last_off));
                        break;
                    }
                }
                Nl80211Attrs::BeaconTail(val) => {
                    if last_off == offset {
                        stack.push(("BeaconTail", last_off));
                        break;
                    }
                }
                Nl80211Attrs::StaAid(val) => {
                    if last_off == offset {
                        stack.push(("StaAid", last_off));
                        break;
                    }
                }
                Nl80211Attrs::StaFlags(val) => {
                    if last_off == offset {
                        stack.push(("StaFlags", last_off));
                        break;
                    }
                }
                Nl80211Attrs::StaListenInterval(val) => {
                    if last_off == offset {
                        stack.push(("StaListenInterval", last_off));
                        break;
                    }
                }
                Nl80211Attrs::StaSupportedRates(val) => {
                    if last_off == offset {
                        stack.push(("StaSupportedRates", last_off));
                        break;
                    }
                }
                Nl80211Attrs::StaVlan(val) => {
                    if last_off == offset {
                        stack.push(("StaVlan", last_off));
                        break;
                    }
                }
                Nl80211Attrs::StaInfo(val) => {
                    if last_off == offset {
                        stack.push(("StaInfo", last_off));
                        break;
                    }
                }
                Nl80211Attrs::WiphyBands(val) => {
                    (stack, missing) = val.lookup_attr(offset, missing_type);
                    if !stack.is_empty() {
                        break;
                    }
                }
                Nl80211Attrs::MntrFlags(val) => {
                    if last_off == offset {
                        stack.push(("MntrFlags", last_off));
                        break;
                    }
                }
                Nl80211Attrs::MeshId(val) => {
                    if last_off == offset {
                        stack.push(("MeshId", last_off));
                        break;
                    }
                }
                Nl80211Attrs::StaPlinkAction(val) => {
                    if last_off == offset {
                        stack.push(("StaPlinkAction", last_off));
                        break;
                    }
                }
                Nl80211Attrs::MpathNextHop(val) => {
                    if last_off == offset {
                        stack.push(("MpathNextHop", last_off));
                        break;
                    }
                }
                Nl80211Attrs::MpathInfo(val) => {
                    if last_off == offset {
                        stack.push(("MpathInfo", last_off));
                        break;
                    }
                }
                Nl80211Attrs::BssCtsProt(val) => {
                    if last_off == offset {
                        stack.push(("BssCtsProt", last_off));
                        break;
                    }
                }
                Nl80211Attrs::BssShortPreamble(val) => {
                    if last_off == offset {
                        stack.push(("BssShortPreamble", last_off));
                        break;
                    }
                }
                Nl80211Attrs::BssShortSlotTime(val) => {
                    if last_off == offset {
                        stack.push(("BssShortSlotTime", last_off));
                        break;
                    }
                }
                Nl80211Attrs::HtCapability(val) => {
                    if last_off == offset {
                        stack.push(("HtCapability", last_off));
                        break;
                    }
                }
                Nl80211Attrs::SupportedIftypes(val) => {
                    (stack, missing) = val.lookup_attr(offset, missing_type);
                    if !stack.is_empty() {
                        break;
                    }
                }
                Nl80211Attrs::RegAlpha2(val) => {
                    if last_off == offset {
                        stack.push(("RegAlpha2", last_off));
                        break;
                    }
                }
                Nl80211Attrs::RegRules(val) => {
                    if last_off == offset {
                        stack.push(("RegRules", last_off));
                        break;
                    }
                }
                Nl80211Attrs::MeshConfig(val) => {
                    if last_off == offset {
                        stack.push(("MeshConfig", last_off));
                        break;
                    }
                }
                Nl80211Attrs::BssBasicRates(val) => {
                    if last_off == offset {
                        stack.push(("BssBasicRates", last_off));
                        break;
                    }
                }
                Nl80211Attrs::WiphyTxqParams(val) => {
                    if last_off == offset {
                        stack.push(("WiphyTxqParams", last_off));
                        break;
                    }
                }
                Nl80211Attrs::WiphyFreq(val) => {
                    if last_off == offset {
                        stack.push(("WiphyFreq", last_off));
                        break;
                    }
                }
                Nl80211Attrs::WiphyChannelType(val) => {
                    if last_off == offset {
                        stack.push(("WiphyChannelType", last_off));
                        break;
                    }
                }
                Nl80211Attrs::KeyDefaultMgmt(val) => {
                    if last_off == offset {
                        stack.push(("KeyDefaultMgmt", last_off));
                        break;
                    }
                }
                Nl80211Attrs::MgmtSubtype(val) => {
                    if last_off == offset {
                        stack.push(("MgmtSubtype", last_off));
                        break;
                    }
                }
                Nl80211Attrs::Ie(val) => {
                    if last_off == offset {
                        stack.push(("Ie", last_off));
                        break;
                    }
                }
                Nl80211Attrs::MaxNumScanSsids(val) => {
                    if last_off == offset {
                        stack.push(("MaxNumScanSsids", last_off));
                        break;
                    }
                }
                Nl80211Attrs::ScanFrequencies(val) => {
                    if last_off == offset {
                        stack.push(("ScanFrequencies", last_off));
                        break;
                    }
                }
                Nl80211Attrs::ScanSsids(val) => {
                    if last_off == offset {
                        stack.push(("ScanSsids", last_off));
                        break;
                    }
                }
                Nl80211Attrs::Generation(val) => {
                    if last_off == offset {
                        stack.push(("Generation", last_off));
                        break;
                    }
                }
                Nl80211Attrs::Bss(val) => {
                    if last_off == offset {
                        stack.push(("Bss", last_off));
                        break;
                    }
                }
                Nl80211Attrs::RegInitiator(val) => {
                    if last_off == offset {
                        stack.push(("RegInitiator", last_off));
                        break;
                    }
                }
                Nl80211Attrs::RegType(val) => {
                    if last_off == offset {
                        stack.push(("RegType", last_off));
                        break;
                    }
                }
                Nl80211Attrs::Frame(val) => {
                    if last_off == offset {
                        stack.push(("Frame", last_off));
                        break;
                    }
                }
                Nl80211Attrs::Ssid(val) => {
                    if last_off == offset {
                        stack.push(("Ssid", last_off));
                        break;
                    }
                }
                Nl80211Attrs::AuthType(val) => {
                    if last_off == offset {
                        stack.push(("AuthType", last_off));
                        break;
                    }
                }
                Nl80211Attrs::ReasonCode(val) => {
                    if last_off == offset {
                        stack.push(("ReasonCode", last_off));
                        break;
                    }
                }
                Nl80211Attrs::KeyType(val) => {
                    if last_off == offset {
                        stack.push(("KeyType", last_off));
                        break;
                    }
                }
                Nl80211Attrs::MaxScanIeLen(val) => {
                    if last_off == offset {
                        stack.push(("MaxScanIeLen", last_off));
                        break;
                    }
                }
                Nl80211Attrs::CipherSuites(val) => {
                    if last_off == offset {
                        stack.push(("CipherSuites", last_off));
                        break;
                    }
                }
                Nl80211Attrs::FreqBefore(val) => {
                    if last_off == offset {
                        stack.push(("FreqBefore", last_off));
                        break;
                    }
                }
                Nl80211Attrs::FreqAfter(val) => {
                    if last_off == offset {
                        stack.push(("FreqAfter", last_off));
                        break;
                    }
                }
                Nl80211Attrs::FreqFixed(val) => {
                    if last_off == offset {
                        stack.push(("FreqFixed", last_off));
                        break;
                    }
                }
                Nl80211Attrs::WiphyRetryShort(val) => {
                    if last_off == offset {
                        stack.push(("WiphyRetryShort", last_off));
                        break;
                    }
                }
                Nl80211Attrs::WiphyRetryLong(val) => {
                    if last_off == offset {
                        stack.push(("WiphyRetryLong", last_off));
                        break;
                    }
                }
                Nl80211Attrs::WiphyFragThreshold(val) => {
                    if last_off == offset {
                        stack.push(("WiphyFragThreshold", last_off));
                        break;
                    }
                }
                Nl80211Attrs::WiphyRtsThreshold(val) => {
                    if last_off == offset {
                        stack.push(("WiphyRtsThreshold", last_off));
                        break;
                    }
                }
                Nl80211Attrs::TimedOut(val) => {
                    if last_off == offset {
                        stack.push(("TimedOut", last_off));
                        break;
                    }
                }
                Nl80211Attrs::UseMfp(val) => {
                    if last_off == offset {
                        stack.push(("UseMfp", last_off));
                        break;
                    }
                }
                Nl80211Attrs::StaFlags2(val) => {
                    if last_off == offset {
                        stack.push(("StaFlags2", last_off));
                        break;
                    }
                }
                Nl80211Attrs::ControlPort(val) => {
                    if last_off == offset {
                        stack.push(("ControlPort", last_off));
                        break;
                    }
                }
                Nl80211Attrs::Testdata(val) => {
                    if last_off == offset {
                        stack.push(("Testdata", last_off));
                        break;
                    }
                }
                Nl80211Attrs::Privacy(val) => {
                    if last_off == offset {
                        stack.push(("Privacy", last_off));
                        break;
                    }
                }
                Nl80211Attrs::DisconnectedByAp(val) => {
                    if last_off == offset {
                        stack.push(("DisconnectedByAp", last_off));
                        break;
                    }
                }
                Nl80211Attrs::StatusCode(val) => {
                    if last_off == offset {
                        stack.push(("StatusCode", last_off));
                        break;
                    }
                }
                Nl80211Attrs::CipherSuitesPairwise(val) => {
                    if last_off == offset {
                        stack.push(("CipherSuitesPairwise", last_off));
                        break;
                    }
                }
                Nl80211Attrs::CipherSuiteGroup(val) => {
                    if last_off == offset {
                        stack.push(("CipherSuiteGroup", last_off));
                        break;
                    }
                }
                Nl80211Attrs::WpaVersions(val) => {
                    if last_off == offset {
                        stack.push(("WpaVersions", last_off));
                        break;
                    }
                }
                Nl80211Attrs::AkmSuites(val) => {
                    if last_off == offset {
                        stack.push(("AkmSuites", last_off));
                        break;
                    }
                }
                Nl80211Attrs::ReqIe(val) => {
                    if last_off == offset {
                        stack.push(("ReqIe", last_off));
                        break;
                    }
                }
                Nl80211Attrs::RespIe(val) => {
                    if last_off == offset {
                        stack.push(("RespIe", last_off));
                        break;
                    }
                }
                Nl80211Attrs::PrevBssid(val) => {
                    if last_off == offset {
                        stack.push(("PrevBssid", last_off));
                        break;
                    }
                }
                Nl80211Attrs::Key(val) => {
                    if last_off == offset {
                        stack.push(("Key", last_off));
                        break;
                    }
                }
                Nl80211Attrs::Keys(val) => {
                    if last_off == offset {
                        stack.push(("Keys", last_off));
                        break;
                    }
                }
                Nl80211Attrs::Pid(val) => {
                    if last_off == offset {
                        stack.push(("Pid", last_off));
                        break;
                    }
                }
                Nl80211Attrs::_4addr(val) => {
                    if last_off == offset {
                        stack.push(("4addr", last_off));
                        break;
                    }
                }
                Nl80211Attrs::SurveyInfo(val) => {
                    if last_off == offset {
                        stack.push(("SurveyInfo", last_off));
                        break;
                    }
                }
                Nl80211Attrs::Pmkid(val) => {
                    if last_off == offset {
                        stack.push(("Pmkid", last_off));
                        break;
                    }
                }
                Nl80211Attrs::MaxNumPmkids(val) => {
                    if last_off == offset {
                        stack.push(("MaxNumPmkids", last_off));
                        break;
                    }
                }
                Nl80211Attrs::Duration(val) => {
                    if last_off == offset {
                        stack.push(("Duration", last_off));
                        break;
                    }
                }
                Nl80211Attrs::Cookie(val) => {
                    if last_off == offset {
                        stack.push(("Cookie", last_off));
                        break;
                    }
                }
                Nl80211Attrs::WiphyCoverageClass(val) => {
                    if last_off == offset {
                        stack.push(("WiphyCoverageClass", last_off));
                        break;
                    }
                }
                Nl80211Attrs::TxRates(val) => {
                    if last_off == offset {
                        stack.push(("TxRates", last_off));
                        break;
                    }
                }
                Nl80211Attrs::FrameMatch(val) => {
                    if last_off == offset {
                        stack.push(("FrameMatch", last_off));
                        break;
                    }
                }
                Nl80211Attrs::Ack(val) => {
                    if last_off == offset {
                        stack.push(("Ack", last_off));
                        break;
                    }
                }
                Nl80211Attrs::PsState(val) => {
                    if last_off == offset {
                        stack.push(("PsState", last_off));
                        break;
                    }
                }
                Nl80211Attrs::Cqm(val) => {
                    if last_off == offset {
                        stack.push(("Cqm", last_off));
                        break;
                    }
                }
                Nl80211Attrs::LocalStateChange(val) => {
                    if last_off == offset {
                        stack.push(("LocalStateChange", last_off));
                        break;
                    }
                }
                Nl80211Attrs::ApIsolate(val) => {
                    if last_off == offset {
                        stack.push(("ApIsolate", last_off));
                        break;
                    }
                }
                Nl80211Attrs::WiphyTxPowerSetting(val) => {
                    if last_off == offset {
                        stack.push(("WiphyTxPowerSetting", last_off));
                        break;
                    }
                }
                Nl80211Attrs::WiphyTxPowerLevel(val) => {
                    if last_off == offset {
                        stack.push(("WiphyTxPowerLevel", last_off));
                        break;
                    }
                }
                Nl80211Attrs::TxFrameTypes(val) => {
                    (stack, missing) = val.lookup_attr(offset, missing_type);
                    if !stack.is_empty() {
                        break;
                    }
                }
                Nl80211Attrs::RxFrameTypes(val) => {
                    (stack, missing) = val.lookup_attr(offset, missing_type);
                    if !stack.is_empty() {
                        break;
                    }
                }
                Nl80211Attrs::FrameType(val) => {
                    if last_off == offset {
                        stack.push(("FrameType", last_off));
                        break;
                    }
                }
                Nl80211Attrs::ControlPortEthertype(val) => {
                    if last_off == offset {
                        stack.push(("ControlPortEthertype", last_off));
                        break;
                    }
                }
                Nl80211Attrs::ControlPortNoEncrypt(val) => {
                    if last_off == offset {
                        stack.push(("ControlPortNoEncrypt", last_off));
                        break;
                    }
                }
                Nl80211Attrs::SupportIbssRsn(val) => {
                    if last_off == offset {
                        stack.push(("SupportIbssRsn", last_off));
                        break;
                    }
                }
                Nl80211Attrs::WiphyAntennaTx(val) => {
                    if last_off == offset {
                        stack.push(("WiphyAntennaTx", last_off));
                        break;
                    }
                }
                Nl80211Attrs::WiphyAntennaRx(val) => {
                    if last_off == offset {
                        stack.push(("WiphyAntennaRx", last_off));
                        break;
                    }
                }
                Nl80211Attrs::McastRate(val) => {
                    if last_off == offset {
                        stack.push(("McastRate", last_off));
                        break;
                    }
                }
                Nl80211Attrs::OffchannelTxOk(val) => {
                    if last_off == offset {
                        stack.push(("OffchannelTxOk", last_off));
                        break;
                    }
                }
                Nl80211Attrs::BssHtOpmode(val) => {
                    if last_off == offset {
                        stack.push(("BssHtOpmode", last_off));
                        break;
                    }
                }
                Nl80211Attrs::KeyDefaultTypes(val) => {
                    if last_off == offset {
                        stack.push(("KeyDefaultTypes", last_off));
                        break;
                    }
                }
                Nl80211Attrs::MaxRemainOnChannelDuration(val) => {
                    if last_off == offset {
                        stack.push(("MaxRemainOnChannelDuration", last_off));
                        break;
                    }
                }
                Nl80211Attrs::MeshSetup(val) => {
                    if last_off == offset {
                        stack.push(("MeshSetup", last_off));
                        break;
                    }
                }
                Nl80211Attrs::WiphyAntennaAvailTx(val) => {
                    if last_off == offset {
                        stack.push(("WiphyAntennaAvailTx", last_off));
                        break;
                    }
                }
                Nl80211Attrs::WiphyAntennaAvailRx(val) => {
                    if last_off == offset {
                        stack.push(("WiphyAntennaAvailRx", last_off));
                        break;
                    }
                }
                Nl80211Attrs::SupportMeshAuth(val) => {
                    if last_off == offset {
                        stack.push(("SupportMeshAuth", last_off));
                        break;
                    }
                }
                Nl80211Attrs::StaPlinkState(val) => {
                    if last_off == offset {
                        stack.push(("StaPlinkState", last_off));
                        break;
                    }
                }
                Nl80211Attrs::WowlanTriggers(val) => {
                    if last_off == offset {
                        stack.push(("WowlanTriggers", last_off));
                        break;
                    }
                }
                Nl80211Attrs::WowlanTriggersSupported(val) => {
                    (stack, missing) = val.lookup_attr(offset, missing_type);
                    if !stack.is_empty() {
                        break;
                    }
                }
                Nl80211Attrs::SchedScanInterval(val) => {
                    if last_off == offset {
                        stack.push(("SchedScanInterval", last_off));
                        break;
                    }
                }
                Nl80211Attrs::InterfaceCombinations(val) => {
                    if last_off == offset {
                        stack.push(("InterfaceCombinations", last_off));
                        break;
                    }
                }
                Nl80211Attrs::SoftwareIftypes(val) => {
                    (stack, missing) = val.lookup_attr(offset, missing_type);
                    if !stack.is_empty() {
                        break;
                    }
                }
                Nl80211Attrs::RekeyData(val) => {
                    if last_off == offset {
                        stack.push(("RekeyData", last_off));
                        break;
                    }
                }
                Nl80211Attrs::MaxNumSchedScanSsids(val) => {
                    if last_off == offset {
                        stack.push(("MaxNumSchedScanSsids", last_off));
                        break;
                    }
                }
                Nl80211Attrs::MaxSchedScanIeLen(val) => {
                    if last_off == offset {
                        stack.push(("MaxSchedScanIeLen", last_off));
                        break;
                    }
                }
                Nl80211Attrs::ScanSuppRates(val) => {
                    if last_off == offset {
                        stack.push(("ScanSuppRates", last_off));
                        break;
                    }
                }
                Nl80211Attrs::HiddenSsid(val) => {
                    if last_off == offset {
                        stack.push(("HiddenSsid", last_off));
                        break;
                    }
                }
                Nl80211Attrs::IeProbeResp(val) => {
                    if last_off == offset {
                        stack.push(("IeProbeResp", last_off));
                        break;
                    }
                }
                Nl80211Attrs::IeAssocResp(val) => {
                    if last_off == offset {
                        stack.push(("IeAssocResp", last_off));
                        break;
                    }
                }
                Nl80211Attrs::StaWme(val) => {
                    if last_off == offset {
                        stack.push(("StaWme", last_off));
                        break;
                    }
                }
                Nl80211Attrs::SupportApUapsd(val) => {
                    if last_off == offset {
                        stack.push(("SupportApUapsd", last_off));
                        break;
                    }
                }
                Nl80211Attrs::RoamSupport(val) => {
                    if last_off == offset {
                        stack.push(("RoamSupport", last_off));
                        break;
                    }
                }
                Nl80211Attrs::SchedScanMatch(val) => {
                    if last_off == offset {
                        stack.push(("SchedScanMatch", last_off));
                        break;
                    }
                }
                Nl80211Attrs::MaxMatchSets(val) => {
                    if last_off == offset {
                        stack.push(("MaxMatchSets", last_off));
                        break;
                    }
                }
                Nl80211Attrs::PmksaCandidate(val) => {
                    if last_off == offset {
                        stack.push(("PmksaCandidate", last_off));
                        break;
                    }
                }
                Nl80211Attrs::TxNoCckRate(val) => {
                    if last_off == offset {
                        stack.push(("TxNoCckRate", last_off));
                        break;
                    }
                }
                Nl80211Attrs::TdlsAction(val) => {
                    if last_off == offset {
                        stack.push(("TdlsAction", last_off));
                        break;
                    }
                }
                Nl80211Attrs::TdlsDialogToken(val) => {
                    if last_off == offset {
                        stack.push(("TdlsDialogToken", last_off));
                        break;
                    }
                }
                Nl80211Attrs::TdlsOperation(val) => {
                    if last_off == offset {
                        stack.push(("TdlsOperation", last_off));
                        break;
                    }
                }
                Nl80211Attrs::TdlsSupport(val) => {
                    if last_off == offset {
                        stack.push(("TdlsSupport", last_off));
                        break;
                    }
                }
                Nl80211Attrs::TdlsExternalSetup(val) => {
                    if last_off == offset {
                        stack.push(("TdlsExternalSetup", last_off));
                        break;
                    }
                }
                Nl80211Attrs::DeviceApSme(val) => {
                    if last_off == offset {
                        stack.push(("DeviceApSme", last_off));
                        break;
                    }
                }
                Nl80211Attrs::DontWaitForAck(val) => {
                    if last_off == offset {
                        stack.push(("DontWaitForAck", last_off));
                        break;
                    }
                }
                Nl80211Attrs::FeatureFlags(val) => {
                    if last_off == offset {
                        stack.push(("FeatureFlags", last_off));
                        break;
                    }
                }
                Nl80211Attrs::ProbeRespOffload(val) => {
                    if last_off == offset {
                        stack.push(("ProbeRespOffload", last_off));
                        break;
                    }
                }
                Nl80211Attrs::ProbeResp(val) => {
                    if last_off == offset {
                        stack.push(("ProbeResp", last_off));
                        break;
                    }
                }
                Nl80211Attrs::DfsRegion(val) => {
                    if last_off == offset {
                        stack.push(("DfsRegion", last_off));
                        break;
                    }
                }
                Nl80211Attrs::DisableHt(val) => {
                    if last_off == offset {
                        stack.push(("DisableHt", last_off));
                        break;
                    }
                }
                Nl80211Attrs::HtCapabilityMask(val) => {
                    if last_off == offset {
                        stack.push(("HtCapabilityMask", last_off));
                        break;
                    }
                }
                Nl80211Attrs::NoackMap(val) => {
                    if last_off == offset {
                        stack.push(("NoackMap", last_off));
                        break;
                    }
                }
                Nl80211Attrs::InactivityTimeout(val) => {
                    if last_off == offset {
                        stack.push(("InactivityTimeout", last_off));
                        break;
                    }
                }
                Nl80211Attrs::RxSignalDbm(val) => {
                    if last_off == offset {
                        stack.push(("RxSignalDbm", last_off));
                        break;
                    }
                }
                Nl80211Attrs::BgScanPeriod(val) => {
                    if last_off == offset {
                        stack.push(("BgScanPeriod", last_off));
                        break;
                    }
                }
                Nl80211Attrs::Wdev(val) => {
                    if last_off == offset {
                        stack.push(("Wdev", last_off));
                        break;
                    }
                }
                Nl80211Attrs::UserRegHintType(val) => {
                    if last_off == offset {
                        stack.push(("UserRegHintType", last_off));
                        break;
                    }
                }
                Nl80211Attrs::ConnFailedReason(val) => {
                    if last_off == offset {
                        stack.push(("ConnFailedReason", last_off));
                        break;
                    }
                }
                Nl80211Attrs::AuthData(val) => {
                    if last_off == offset {
                        stack.push(("AuthData", last_off));
                        break;
                    }
                }
                Nl80211Attrs::VhtCapability(val) => {
                    if last_off == offset {
                        stack.push(("VhtCapability", last_off));
                        break;
                    }
                }
                Nl80211Attrs::ScanFlags(val) => {
                    if last_off == offset {
                        stack.push(("ScanFlags", last_off));
                        break;
                    }
                }
                Nl80211Attrs::ChannelWidth(val) => {
                    if last_off == offset {
                        stack.push(("ChannelWidth", last_off));
                        break;
                    }
                }
                Nl80211Attrs::CenterFreq1(val) => {
                    if last_off == offset {
                        stack.push(("CenterFreq1", last_off));
                        break;
                    }
                }
                Nl80211Attrs::CenterFreq2(val) => {
                    if last_off == offset {
                        stack.push(("CenterFreq2", last_off));
                        break;
                    }
                }
                Nl80211Attrs::P2pCtwindow(val) => {
                    if last_off == offset {
                        stack.push(("P2pCtwindow", last_off));
                        break;
                    }
                }
                Nl80211Attrs::P2pOppps(val) => {
                    if last_off == offset {
                        stack.push(("P2pOppps", last_off));
                        break;
                    }
                }
                Nl80211Attrs::LocalMeshPowerMode(val) => {
                    if last_off == offset {
                        stack.push(("LocalMeshPowerMode", last_off));
                        break;
                    }
                }
                Nl80211Attrs::AclPolicy(val) => {
                    if last_off == offset {
                        stack.push(("AclPolicy", last_off));
                        break;
                    }
                }
                Nl80211Attrs::MacAddrs(val) => {
                    if last_off == offset {
                        stack.push(("MacAddrs", last_off));
                        break;
                    }
                }
                Nl80211Attrs::MacAclMax(val) => {
                    if last_off == offset {
                        stack.push(("MacAclMax", last_off));
                        break;
                    }
                }
                Nl80211Attrs::RadarEvent(val) => {
                    if last_off == offset {
                        stack.push(("RadarEvent", last_off));
                        break;
                    }
                }
                Nl80211Attrs::ExtCapa(val) => {
                    if last_off == offset {
                        stack.push(("ExtCapa", last_off));
                        break;
                    }
                }
                Nl80211Attrs::ExtCapaMask(val) => {
                    if last_off == offset {
                        stack.push(("ExtCapaMask", last_off));
                        break;
                    }
                }
                Nl80211Attrs::StaCapability(val) => {
                    if last_off == offset {
                        stack.push(("StaCapability", last_off));
                        break;
                    }
                }
                Nl80211Attrs::StaExtCapability(val) => {
                    if last_off == offset {
                        stack.push(("StaExtCapability", last_off));
                        break;
                    }
                }
                Nl80211Attrs::ProtocolFeatures(val) => {
                    if last_off == offset {
                        stack.push(("ProtocolFeatures", last_off));
                        break;
                    }
                }
                Nl80211Attrs::SplitWiphyDump(val) => {
                    if last_off == offset {
                        stack.push(("SplitWiphyDump", last_off));
                        break;
                    }
                }
                Nl80211Attrs::DisableVht(val) => {
                    if last_off == offset {
                        stack.push(("DisableVht", last_off));
                        break;
                    }
                }
                Nl80211Attrs::VhtCapabilityMask(val) => {
                    if last_off == offset {
                        stack.push(("VhtCapabilityMask", last_off));
                        break;
                    }
                }
                Nl80211Attrs::Mdid(val) => {
                    if last_off == offset {
                        stack.push(("Mdid", last_off));
                        break;
                    }
                }
                Nl80211Attrs::IeRic(val) => {
                    if last_off == offset {
                        stack.push(("IeRic", last_off));
                        break;
                    }
                }
                Nl80211Attrs::CritProtId(val) => {
                    if last_off == offset {
                        stack.push(("CritProtId", last_off));
                        break;
                    }
                }
                Nl80211Attrs::MaxCritProtDuration(val) => {
                    if last_off == offset {
                        stack.push(("MaxCritProtDuration", last_off));
                        break;
                    }
                }
                Nl80211Attrs::PeerAid(val) => {
                    if last_off == offset {
                        stack.push(("PeerAid", last_off));
                        break;
                    }
                }
                Nl80211Attrs::CoalesceRule(val) => {
                    if last_off == offset {
                        stack.push(("CoalesceRule", last_off));
                        break;
                    }
                }
                Nl80211Attrs::ChSwitchCount(val) => {
                    if last_off == offset {
                        stack.push(("ChSwitchCount", last_off));
                        break;
                    }
                }
                Nl80211Attrs::ChSwitchBlockTx(val) => {
                    if last_off == offset {
                        stack.push(("ChSwitchBlockTx", last_off));
                        break;
                    }
                }
                Nl80211Attrs::CsaIes(val) => {
                    if last_off == offset {
                        stack.push(("CsaIes", last_off));
                        break;
                    }
                }
                Nl80211Attrs::CntdwnOffsBeacon(val) => {
                    if last_off == offset {
                        stack.push(("CntdwnOffsBeacon", last_off));
                        break;
                    }
                }
                Nl80211Attrs::CntdwnOffsPresp(val) => {
                    if last_off == offset {
                        stack.push(("CntdwnOffsPresp", last_off));
                        break;
                    }
                }
                Nl80211Attrs::RxmgmtFlags(val) => {
                    if last_off == offset {
                        stack.push(("RxmgmtFlags", last_off));
                        break;
                    }
                }
                Nl80211Attrs::StaSupportedChannels(val) => {
                    if last_off == offset {
                        stack.push(("StaSupportedChannels", last_off));
                        break;
                    }
                }
                Nl80211Attrs::StaSupportedOperClasses(val) => {
                    if last_off == offset {
                        stack.push(("StaSupportedOperClasses", last_off));
                        break;
                    }
                }
                Nl80211Attrs::HandleDfs(val) => {
                    if last_off == offset {
                        stack.push(("HandleDfs", last_off));
                        break;
                    }
                }
                Nl80211Attrs::Support5Mhz(val) => {
                    if last_off == offset {
                        stack.push(("Support5Mhz", last_off));
                        break;
                    }
                }
                Nl80211Attrs::Support10Mhz(val) => {
                    if last_off == offset {
                        stack.push(("Support10Mhz", last_off));
                        break;
                    }
                }
                Nl80211Attrs::OpmodeNotif(val) => {
                    if last_off == offset {
                        stack.push(("OpmodeNotif", last_off));
                        break;
                    }
                }
                Nl80211Attrs::VendorId(val) => {
                    if last_off == offset {
                        stack.push(("VendorId", last_off));
                        break;
                    }
                }
                Nl80211Attrs::VendorSubcmd(val) => {
                    if last_off == offset {
                        stack.push(("VendorSubcmd", last_off));
                        break;
                    }
                }
                Nl80211Attrs::VendorData(val) => {
                    if last_off == offset {
                        stack.push(("VendorData", last_off));
                        break;
                    }
                }
                Nl80211Attrs::VendorEvents(val) => {
                    if last_off == offset {
                        stack.push(("VendorEvents", last_off));
                        break;
                    }
                }
                Nl80211Attrs::QosMap(val) => {
                    if last_off == offset {
                        stack.push(("QosMap", last_off));
                        break;
                    }
                }
                Nl80211Attrs::MacHint(val) => {
                    if last_off == offset {
                        stack.push(("MacHint", last_off));
                        break;
                    }
                }
                Nl80211Attrs::WiphyFreqHint(val) => {
                    if last_off == offset {
                        stack.push(("WiphyFreqHint", last_off));
                        break;
                    }
                }
                Nl80211Attrs::MaxApAssocSta(val) => {
                    if last_off == offset {
                        stack.push(("MaxApAssocSta", last_off));
                        break;
                    }
                }
                Nl80211Attrs::TdlsPeerCapability(val) => {
                    if last_off == offset {
                        stack.push(("TdlsPeerCapability", last_off));
                        break;
                    }
                }
                Nl80211Attrs::SocketOwner(val) => {
                    if last_off == offset {
                        stack.push(("SocketOwner", last_off));
                        break;
                    }
                }
                Nl80211Attrs::CsaCOffsetsTx(val) => {
                    if last_off == offset {
                        stack.push(("CsaCOffsetsTx", last_off));
                        break;
                    }
                }
                Nl80211Attrs::MaxCsaCounters(val) => {
                    if last_off == offset {
                        stack.push(("MaxCsaCounters", last_off));
                        break;
                    }
                }
                Nl80211Attrs::TdlsInitiator(val) => {
                    if last_off == offset {
                        stack.push(("TdlsInitiator", last_off));
                        break;
                    }
                }
                Nl80211Attrs::UseRrm(val) => {
                    if last_off == offset {
                        stack.push(("UseRrm", last_off));
                        break;
                    }
                }
                Nl80211Attrs::WiphyDynAck(val) => {
                    if last_off == offset {
                        stack.push(("WiphyDynAck", last_off));
                        break;
                    }
                }
                Nl80211Attrs::Tsid(val) => {
                    if last_off == offset {
                        stack.push(("Tsid", last_off));
                        break;
                    }
                }
                Nl80211Attrs::UserPrio(val) => {
                    if last_off == offset {
                        stack.push(("UserPrio", last_off));
                        break;
                    }
                }
                Nl80211Attrs::AdmittedTime(val) => {
                    if last_off == offset {
                        stack.push(("AdmittedTime", last_off));
                        break;
                    }
                }
                Nl80211Attrs::SmpsMode(val) => {
                    if last_off == offset {
                        stack.push(("SmpsMode", last_off));
                        break;
                    }
                }
                Nl80211Attrs::OperClass(val) => {
                    if last_off == offset {
                        stack.push(("OperClass", last_off));
                        break;
                    }
                }
                Nl80211Attrs::MacMask(val) => {
                    if last_off == offset {
                        stack.push(("MacMask", last_off));
                        break;
                    }
                }
                Nl80211Attrs::WiphySelfManagedReg(val) => {
                    if last_off == offset {
                        stack.push(("WiphySelfManagedReg", last_off));
                        break;
                    }
                }
                Nl80211Attrs::ExtFeatures(val) => {
                    if last_off == offset {
                        stack.push(("ExtFeatures", last_off));
                        break;
                    }
                }
                Nl80211Attrs::SurveyRadioStats(val) => {
                    if last_off == offset {
                        stack.push(("SurveyRadioStats", last_off));
                        break;
                    }
                }
                Nl80211Attrs::NetnsFd(val) => {
                    if last_off == offset {
                        stack.push(("NetnsFd", last_off));
                        break;
                    }
                }
                Nl80211Attrs::SchedScanDelay(val) => {
                    if last_off == offset {
                        stack.push(("SchedScanDelay", last_off));
                        break;
                    }
                }
                Nl80211Attrs::RegIndoor(val) => {
                    if last_off == offset {
                        stack.push(("RegIndoor", last_off));
                        break;
                    }
                }
                Nl80211Attrs::MaxNumSchedScanPlans(val) => {
                    if last_off == offset {
                        stack.push(("MaxNumSchedScanPlans", last_off));
                        break;
                    }
                }
                Nl80211Attrs::MaxScanPlanInterval(val) => {
                    if last_off == offset {
                        stack.push(("MaxScanPlanInterval", last_off));
                        break;
                    }
                }
                Nl80211Attrs::MaxScanPlanIterations(val) => {
                    if last_off == offset {
                        stack.push(("MaxScanPlanIterations", last_off));
                        break;
                    }
                }
                Nl80211Attrs::SchedScanPlans(val) => {
                    if last_off == offset {
                        stack.push(("SchedScanPlans", last_off));
                        break;
                    }
                }
                Nl80211Attrs::Pbss(val) => {
                    if last_off == offset {
                        stack.push(("Pbss", last_off));
                        break;
                    }
                }
                Nl80211Attrs::BssSelect(val) => {
                    if last_off == offset {
                        stack.push(("BssSelect", last_off));
                        break;
                    }
                }
                Nl80211Attrs::StaSupportP2pPs(val) => {
                    if last_off == offset {
                        stack.push(("StaSupportP2pPs", last_off));
                        break;
                    }
                }
                Nl80211Attrs::Pad(val) => {
                    if last_off == offset {
                        stack.push(("Pad", last_off));
                        break;
                    }
                }
                Nl80211Attrs::IftypeExtCapa(val) => {
                    if last_off == offset {
                        stack.push(("IftypeExtCapa", last_off));
                        break;
                    }
                }
                Nl80211Attrs::MuMimoGroupData(val) => {
                    if last_off == offset {
                        stack.push(("MuMimoGroupData", last_off));
                        break;
                    }
                }
                Nl80211Attrs::MuMimoFollowMacAddr(val) => {
                    if last_off == offset {
                        stack.push(("MuMimoFollowMacAddr", last_off));
                        break;
                    }
                }
                Nl80211Attrs::ScanStartTimeTsf(val) => {
                    if last_off == offset {
                        stack.push(("ScanStartTimeTsf", last_off));
                        break;
                    }
                }
                Nl80211Attrs::ScanStartTimeTsfBssid(val) => {
                    if last_off == offset {
                        stack.push(("ScanStartTimeTsfBssid", last_off));
                        break;
                    }
                }
                Nl80211Attrs::MeasurementDuration(val) => {
                    if last_off == offset {
                        stack.push(("MeasurementDuration", last_off));
                        break;
                    }
                }
                Nl80211Attrs::MeasurementDurationMandatory(val) => {
                    if last_off == offset {
                        stack.push(("MeasurementDurationMandatory", last_off));
                        break;
                    }
                }
                Nl80211Attrs::MeshPeerAid(val) => {
                    if last_off == offset {
                        stack.push(("MeshPeerAid", last_off));
                        break;
                    }
                }
                Nl80211Attrs::NanMasterPref(val) => {
                    if last_off == offset {
                        stack.push(("NanMasterPref", last_off));
                        break;
                    }
                }
                Nl80211Attrs::Bands(val) => {
                    if last_off == offset {
                        stack.push(("Bands", last_off));
                        break;
                    }
                }
                Nl80211Attrs::NanFunc(val) => {
                    if last_off == offset {
                        stack.push(("NanFunc", last_off));
                        break;
                    }
                }
                Nl80211Attrs::NanMatch(val) => {
                    if last_off == offset {
                        stack.push(("NanMatch", last_off));
                        break;
                    }
                }
                Nl80211Attrs::FilsKek(val) => {
                    if last_off == offset {
                        stack.push(("FilsKek", last_off));
                        break;
                    }
                }
                Nl80211Attrs::FilsNonces(val) => {
                    if last_off == offset {
                        stack.push(("FilsNonces", last_off));
                        break;
                    }
                }
                Nl80211Attrs::MulticastToUnicastEnabled(val) => {
                    if last_off == offset {
                        stack.push(("MulticastToUnicastEnabled", last_off));
                        break;
                    }
                }
                Nl80211Attrs::Bssid(val) => {
                    if last_off == offset {
                        stack.push(("Bssid", last_off));
                        break;
                    }
                }
                Nl80211Attrs::SchedScanRelativeRssi(val) => {
                    if last_off == offset {
                        stack.push(("SchedScanRelativeRssi", last_off));
                        break;
                    }
                }
                Nl80211Attrs::SchedScanRssiAdjust(val) => {
                    if last_off == offset {
                        stack.push(("SchedScanRssiAdjust", last_off));
                        break;
                    }
                }
                Nl80211Attrs::TimeoutReason(val) => {
                    if last_off == offset {
                        stack.push(("TimeoutReason", last_off));
                        break;
                    }
                }
                Nl80211Attrs::FilsErpUsername(val) => {
                    if last_off == offset {
                        stack.push(("FilsErpUsername", last_off));
                        break;
                    }
                }
                Nl80211Attrs::FilsErpRealm(val) => {
                    if last_off == offset {
                        stack.push(("FilsErpRealm", last_off));
                        break;
                    }
                }
                Nl80211Attrs::FilsErpNextSeqNum(val) => {
                    if last_off == offset {
                        stack.push(("FilsErpNextSeqNum", last_off));
                        break;
                    }
                }
                Nl80211Attrs::FilsErpRrk(val) => {
                    if last_off == offset {
                        stack.push(("FilsErpRrk", last_off));
                        break;
                    }
                }
                Nl80211Attrs::FilsCacheId(val) => {
                    if last_off == offset {
                        stack.push(("FilsCacheId", last_off));
                        break;
                    }
                }
                Nl80211Attrs::Pmk(val) => {
                    if last_off == offset {
                        stack.push(("Pmk", last_off));
                        break;
                    }
                }
                Nl80211Attrs::SchedScanMulti(val) => {
                    if last_off == offset {
                        stack.push(("SchedScanMulti", last_off));
                        break;
                    }
                }
                Nl80211Attrs::SchedScanMaxReqs(val) => {
                    if last_off == offset {
                        stack.push(("SchedScanMaxReqs", last_off));
                        break;
                    }
                }
                Nl80211Attrs::Want1x4wayHs(val) => {
                    if last_off == offset {
                        stack.push(("Want1x4wayHs", last_off));
                        break;
                    }
                }
                Nl80211Attrs::Pmkr0Name(val) => {
                    if last_off == offset {
                        stack.push(("Pmkr0Name", last_off));
                        break;
                    }
                }
                Nl80211Attrs::PortAuthorized(val) => {
                    if last_off == offset {
                        stack.push(("PortAuthorized", last_off));
                        break;
                    }
                }
                Nl80211Attrs::ExternalAuthAction(val) => {
                    if last_off == offset {
                        stack.push(("ExternalAuthAction", last_off));
                        break;
                    }
                }
                Nl80211Attrs::ExternalAuthSupport(val) => {
                    if last_off == offset {
                        stack.push(("ExternalAuthSupport", last_off));
                        break;
                    }
                }
                Nl80211Attrs::Nss(val) => {
                    if last_off == offset {
                        stack.push(("Nss", last_off));
                        break;
                    }
                }
                Nl80211Attrs::AckSignal(val) => {
                    if last_off == offset {
                        stack.push(("AckSignal", last_off));
                        break;
                    }
                }
                Nl80211Attrs::ControlPortOverNl80211(val) => {
                    if last_off == offset {
                        stack.push(("ControlPortOverNl80211", last_off));
                        break;
                    }
                }
                Nl80211Attrs::TxqStats(val) => {
                    (stack, missing) = val.lookup_attr(offset, missing_type);
                    if !stack.is_empty() {
                        break;
                    }
                }
                Nl80211Attrs::TxqLimit(val) => {
                    if last_off == offset {
                        stack.push(("TxqLimit", last_off));
                        break;
                    }
                }
                Nl80211Attrs::TxqMemoryLimit(val) => {
                    if last_off == offset {
                        stack.push(("TxqMemoryLimit", last_off));
                        break;
                    }
                }
                Nl80211Attrs::TxqQuantum(val) => {
                    if last_off == offset {
                        stack.push(("TxqQuantum", last_off));
                        break;
                    }
                }
                Nl80211Attrs::HeCapability(val) => {
                    if last_off == offset {
                        stack.push(("HeCapability", last_off));
                        break;
                    }
                }
                Nl80211Attrs::FtmResponder(val) => {
                    if last_off == offset {
                        stack.push(("FtmResponder", last_off));
                        break;
                    }
                }
                Nl80211Attrs::FtmResponderStats(val) => {
                    if last_off == offset {
                        stack.push(("FtmResponderStats", last_off));
                        break;
                    }
                }
                Nl80211Attrs::Timeout(val) => {
                    if last_off == offset {
                        stack.push(("Timeout", last_off));
                        break;
                    }
                }
                Nl80211Attrs::PeerMeasurements(val) => {
                    if last_off == offset {
                        stack.push(("PeerMeasurements", last_off));
                        break;
                    }
                }
                Nl80211Attrs::AirtimeWeight(val) => {
                    if last_off == offset {
                        stack.push(("AirtimeWeight", last_off));
                        break;
                    }
                }
                Nl80211Attrs::StaTxPowerSetting(val) => {
                    if last_off == offset {
                        stack.push(("StaTxPowerSetting", last_off));
                        break;
                    }
                }
                Nl80211Attrs::StaTxPower(val) => {
                    if last_off == offset {
                        stack.push(("StaTxPower", last_off));
                        break;
                    }
                }
                Nl80211Attrs::SaePassword(val) => {
                    if last_off == offset {
                        stack.push(("SaePassword", last_off));
                        break;
                    }
                }
                Nl80211Attrs::TwtResponder(val) => {
                    if last_off == offset {
                        stack.push(("TwtResponder", last_off));
                        break;
                    }
                }
                Nl80211Attrs::HeObssPd(val) => {
                    if last_off == offset {
                        stack.push(("HeObssPd", last_off));
                        break;
                    }
                }
                Nl80211Attrs::WiphyEdmgChannels(val) => {
                    if last_off == offset {
                        stack.push(("WiphyEdmgChannels", last_off));
                        break;
                    }
                }
                Nl80211Attrs::WiphyEdmgBwConfig(val) => {
                    if last_off == offset {
                        stack.push(("WiphyEdmgBwConfig", last_off));
                        break;
                    }
                }
                Nl80211Attrs::VlanId(val) => {
                    if last_off == offset {
                        stack.push(("VlanId", last_off));
                        break;
                    }
                }
                Nl80211Attrs::HeBssColor(val) => {
                    if last_off == offset {
                        stack.push(("HeBssColor", last_off));
                        break;
                    }
                }
                Nl80211Attrs::IftypeAkmSuites(val) => {
                    if last_off == offset {
                        stack.push(("IftypeAkmSuites", last_off));
                        break;
                    }
                }
                Nl80211Attrs::TidConfig(val) => {
                    if last_off == offset {
                        stack.push(("TidConfig", last_off));
                        break;
                    }
                }
                Nl80211Attrs::ControlPortNoPreauth(val) => {
                    if last_off == offset {
                        stack.push(("ControlPortNoPreauth", last_off));
                        break;
                    }
                }
                Nl80211Attrs::PmkLifetime(val) => {
                    if last_off == offset {
                        stack.push(("PmkLifetime", last_off));
                        break;
                    }
                }
                Nl80211Attrs::PmkReauthThreshold(val) => {
                    if last_off == offset {
                        stack.push(("PmkReauthThreshold", last_off));
                        break;
                    }
                }
                Nl80211Attrs::ReceiveMulticast(val) => {
                    if last_off == offset {
                        stack.push(("ReceiveMulticast", last_off));
                        break;
                    }
                }
                Nl80211Attrs::WiphyFreqOffset(val) => {
                    if last_off == offset {
                        stack.push(("WiphyFreqOffset", last_off));
                        break;
                    }
                }
                Nl80211Attrs::CenterFreq1Offset(val) => {
                    if last_off == offset {
                        stack.push(("CenterFreq1Offset", last_off));
                        break;
                    }
                }
                Nl80211Attrs::ScanFreqKhz(val) => {
                    if last_off == offset {
                        stack.push(("ScanFreqKhz", last_off));
                        break;
                    }
                }
                Nl80211Attrs::He6ghzCapability(val) => {
                    if last_off == offset {
                        stack.push(("He6ghzCapability", last_off));
                        break;
                    }
                }
                Nl80211Attrs::FilsDiscovery(val) => {
                    if last_off == offset {
                        stack.push(("FilsDiscovery", last_off));
                        break;
                    }
                }
                Nl80211Attrs::UnsolBcastProbeResp(val) => {
                    if last_off == offset {
                        stack.push(("UnsolBcastProbeResp", last_off));
                        break;
                    }
                }
                Nl80211Attrs::S1gCapability(val) => {
                    if last_off == offset {
                        stack.push(("S1gCapability", last_off));
                        break;
                    }
                }
                Nl80211Attrs::S1gCapabilityMask(val) => {
                    if last_off == offset {
                        stack.push(("S1gCapabilityMask", last_off));
                        break;
                    }
                }
                Nl80211Attrs::SaePwe(val) => {
                    if last_off == offset {
                        stack.push(("SaePwe", last_off));
                        break;
                    }
                }
                Nl80211Attrs::ReconnectRequested(val) => {
                    if last_off == offset {
                        stack.push(("ReconnectRequested", last_off));
                        break;
                    }
                }
                Nl80211Attrs::SarSpec(val) => {
                    if last_off == offset {
                        stack.push(("SarSpec", last_off));
                        break;
                    }
                }
                Nl80211Attrs::DisableHe(val) => {
                    if last_off == offset {
                        stack.push(("DisableHe", last_off));
                        break;
                    }
                }
                Nl80211Attrs::ObssColorBitmap(val) => {
                    if last_off == offset {
                        stack.push(("ObssColorBitmap", last_off));
                        break;
                    }
                }
                Nl80211Attrs::ColorChangeCount(val) => {
                    if last_off == offset {
                        stack.push(("ColorChangeCount", last_off));
                        break;
                    }
                }
                Nl80211Attrs::ColorChangeColor(val) => {
                    if last_off == offset {
                        stack.push(("ColorChangeColor", last_off));
                        break;
                    }
                }
                Nl80211Attrs::ColorChangeElems(val) => {
                    if last_off == offset {
                        stack.push(("ColorChangeElems", last_off));
                        break;
                    }
                }
                Nl80211Attrs::MbssidConfig(val) => {
                    if last_off == offset {
                        stack.push(("MbssidConfig", last_off));
                        break;
                    }
                }
                Nl80211Attrs::MbssidElems(val) => {
                    if last_off == offset {
                        stack.push(("MbssidElems", last_off));
                        break;
                    }
                }
                Nl80211Attrs::RadarBackground(val) => {
                    if last_off == offset {
                        stack.push(("RadarBackground", last_off));
                        break;
                    }
                }
                Nl80211Attrs::ApSettingsFlags(val) => {
                    if last_off == offset {
                        stack.push(("ApSettingsFlags", last_off));
                        break;
                    }
                }
                Nl80211Attrs::EhtCapability(val) => {
                    if last_off == offset {
                        stack.push(("EhtCapability", last_off));
                        break;
                    }
                }
                Nl80211Attrs::DisableEht(val) => {
                    if last_off == offset {
                        stack.push(("DisableEht", last_off));
                        break;
                    }
                }
                Nl80211Attrs::MloLinks(val) => {
                    if last_off == offset {
                        stack.push(("MloLinks", last_off));
                        break;
                    }
                }
                Nl80211Attrs::MloLinkId(val) => {
                    if last_off == offset {
                        stack.push(("MloLinkId", last_off));
                        break;
                    }
                }
                Nl80211Attrs::MldAddr(val) => {
                    if last_off == offset {
                        stack.push(("MldAddr", last_off));
                        break;
                    }
                }
                Nl80211Attrs::MloSupport(val) => {
                    if last_off == offset {
                        stack.push(("MloSupport", last_off));
                        break;
                    }
                }
                Nl80211Attrs::MaxNumAkmSuites(val) => {
                    if last_off == offset {
                        stack.push(("MaxNumAkmSuites", last_off));
                        break;
                    }
                }
                Nl80211Attrs::EmlCapability(val) => {
                    if last_off == offset {
                        stack.push(("EmlCapability", last_off));
                        break;
                    }
                }
                Nl80211Attrs::MldCapaAndOps(val) => {
                    if last_off == offset {
                        stack.push(("MldCapaAndOps", last_off));
                        break;
                    }
                }
                Nl80211Attrs::TxHwTimestamp(val) => {
                    if last_off == offset {
                        stack.push(("TxHwTimestamp", last_off));
                        break;
                    }
                }
                Nl80211Attrs::RxHwTimestamp(val) => {
                    if last_off == offset {
                        stack.push(("RxHwTimestamp", last_off));
                        break;
                    }
                }
                Nl80211Attrs::TdBitmap(val) => {
                    if last_off == offset {
                        stack.push(("TdBitmap", last_off));
                        break;
                    }
                }
                Nl80211Attrs::PunctBitmap(val) => {
                    if last_off == offset {
                        stack.push(("PunctBitmap", last_off));
                        break;
                    }
                }
                Nl80211Attrs::MaxHwTimestampPeers(val) => {
                    if last_off == offset {
                        stack.push(("MaxHwTimestampPeers", last_off));
                        break;
                    }
                }
                Nl80211Attrs::HwTimestampEnabled(val) => {
                    if last_off == offset {
                        stack.push(("HwTimestampEnabled", last_off));
                        break;
                    }
                }
                Nl80211Attrs::EmaRnrElems(val) => {
                    if last_off == offset {
                        stack.push(("EmaRnrElems", last_off));
                        break;
                    }
                }
                Nl80211Attrs::MloLinkDisabled(val) => {
                    if last_off == offset {
                        stack.push(("MloLinkDisabled", last_off));
                        break;
                    }
                }
                Nl80211Attrs::BssDumpIncludeUseData(val) => {
                    if last_off == offset {
                        stack.push(("BssDumpIncludeUseData", last_off));
                        break;
                    }
                }
                Nl80211Attrs::MloTtlmDlink(val) => {
                    if last_off == offset {
                        stack.push(("MloTtlmDlink", last_off));
                        break;
                    }
                }
                Nl80211Attrs::MloTtlmUlink(val) => {
                    if last_off == offset {
                        stack.push(("MloTtlmUlink", last_off));
                        break;
                    }
                }
                Nl80211Attrs::AssocSppAmsdu(val) => {
                    if last_off == offset {
                        stack.push(("AssocSppAmsdu", last_off));
                        break;
                    }
                }
                Nl80211Attrs::WiphyRadios(val) => {
                    if last_off == offset {
                        stack.push(("WiphyRadios", last_off));
                        break;
                    }
                }
                Nl80211Attrs::WiphyInterfaceCombinations(val) => {
                    if last_off == offset {
                        stack.push(("WiphyInterfaceCombinations", last_off));
                        break;
                    }
                }
                Nl80211Attrs::VifRadioMask(val) => {
                    if last_off == offset {
                        stack.push(("VifRadioMask", last_off));
                        break;
                    }
                }
                _ => {}
            };
            last_off = cur + attrs.pos;
        }
        if !stack.is_empty() {
            stack.push(("Nl80211Attrs", cur));
        }
        (stack, missing)
    }
}
#[derive(Clone)]
pub enum FrameTypeAttrs {
    FrameType(u16),
}
impl<'a> IterableFrameTypeAttrs<'a> {
    pub fn get_frame_type(&self) -> Result<u16, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let FrameTypeAttrs::FrameType(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "FrameTypeAttrs",
            "FrameType",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl FrameTypeAttrs {
    pub fn new(buf: &'_ [u8]) -> IterableFrameTypeAttrs<'_> {
        IterableFrameTypeAttrs::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        let res = match r#type {
            1u16 => "FrameType",
            _ => return None,
        };
        Some(res)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableFrameTypeAttrs<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableFrameTypeAttrs<'a> {
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
impl<'a> Iterator for IterableFrameTypeAttrs<'a> {
    type Item = Result<FrameTypeAttrs, ErrorContext>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.buf.len() == self.pos {
            return None;
        }
        let pos = self.pos;
        let mut r#type = None;
        while let Some((header, next)) = chop_header(self.buf, &mut self.pos) {
            r#type = Some(header.r#type);
            let res = match header.r#type {
                1u16 => FrameTypeAttrs::FrameType({
                    let res = parse_u16(next);
                    let Some(val) = res else { break };
                    val
                }),
                n => {
                    if cfg!(any(test, feature = "deny-unknown-attrs")) {
                        break;
                    } else {
                        continue;
                    }
                }
            };
            return Some(Ok(res));
        }
        Some(Err(ErrorContext::new(
            "FrameTypeAttrs",
            r#type.and_then(|t| FrameTypeAttrs::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl std::fmt::Debug for IterableFrameTypeAttrs<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("FrameTypeAttrs");
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
                FrameTypeAttrs::FrameType(val) => fmt.field("FrameType", &val),
            };
        }
        fmt.finish()
    }
}
impl IterableFrameTypeAttrs<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if cur == offset {
            stack.push(("FrameTypeAttrs", offset));
            return (
                stack,
                missing_type.and_then(|t| FrameTypeAttrs::attr_from_type(t)),
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
                FrameTypeAttrs::FrameType(val) => {
                    if last_off == offset {
                        stack.push(("FrameType", last_off));
                        break;
                    }
                }
                _ => {}
            };
            last_off = cur + attrs.pos;
        }
        if !stack.is_empty() {
            stack.push(("FrameTypeAttrs", cur));
        }
        (stack, None)
    }
}
#[derive(Clone)]
pub enum WiphyBands<'a> {
    #[doc = "2.4 GHz ISM band"]
    _2ghz(IterableBandAttrs<'a>),
    #[doc = "around 5 GHz band (4.9 - 5.7 GHz)"]
    _5ghz(IterableBandAttrs<'a>),
    #[doc = "around 60 GHz band (58.32 - 69.12 GHz)"]
    _60ghz(IterableBandAttrs<'a>),
    _6ghz(IterableBandAttrs<'a>),
    S1ghz(IterableBandAttrs<'a>),
    Lc(IterableBandAttrs<'a>),
}
impl<'a> IterableWiphyBands<'a> {
    #[doc = "2.4 GHz ISM band"]
    pub fn get_2ghz(&self) -> Result<IterableBandAttrs<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let WiphyBands::_2ghz(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "WiphyBands",
            "2ghz",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "around 5 GHz band (4.9 - 5.7 GHz)"]
    pub fn get_5ghz(&self) -> Result<IterableBandAttrs<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let WiphyBands::_5ghz(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "WiphyBands",
            "5ghz",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "around 60 GHz band (58.32 - 69.12 GHz)"]
    pub fn get_60ghz(&self) -> Result<IterableBandAttrs<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let WiphyBands::_60ghz(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "WiphyBands",
            "60ghz",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_6ghz(&self) -> Result<IterableBandAttrs<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let WiphyBands::_6ghz(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "WiphyBands",
            "6ghz",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_s1ghz(&self) -> Result<IterableBandAttrs<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let WiphyBands::S1ghz(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "WiphyBands",
            "S1ghz",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_lc(&self) -> Result<IterableBandAttrs<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let WiphyBands::Lc(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "WiphyBands",
            "Lc",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl<'a> WiphyBands<'a> {
    pub fn new(buf: &'a [u8]) -> IterableWiphyBands<'a> {
        IterableWiphyBands::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        let res = match r#type {
            0u16 => "2ghz",
            1u16 => "5ghz",
            2u16 => "60ghz",
            3u16 => "6ghz",
            4u16 => "S1ghz",
            5u16 => "Lc",
            _ => return None,
        };
        Some(res)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableWiphyBands<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableWiphyBands<'a> {
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
impl<'a> Iterator for IterableWiphyBands<'a> {
    type Item = Result<WiphyBands<'a>, ErrorContext>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.buf.len() == self.pos {
            return None;
        }
        let pos = self.pos;
        let mut r#type = None;
        while let Some((header, next)) = chop_header(self.buf, &mut self.pos) {
            r#type = Some(header.r#type);
            let res = match header.r#type {
                0u16 => WiphyBands::_2ghz({
                    let res = Some(IterableBandAttrs::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                1u16 => WiphyBands::_5ghz({
                    let res = Some(IterableBandAttrs::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                2u16 => WiphyBands::_60ghz({
                    let res = Some(IterableBandAttrs::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                3u16 => WiphyBands::_6ghz({
                    let res = Some(IterableBandAttrs::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                4u16 => WiphyBands::S1ghz({
                    let res = Some(IterableBandAttrs::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                5u16 => WiphyBands::Lc({
                    let res = Some(IterableBandAttrs::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                n => {
                    if cfg!(any(test, feature = "deny-unknown-attrs")) {
                        break;
                    } else {
                        continue;
                    }
                }
            };
            return Some(Ok(res));
        }
        Some(Err(ErrorContext::new(
            "WiphyBands",
            r#type.and_then(|t| WiphyBands::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl<'a> std::fmt::Debug for IterableWiphyBands<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("WiphyBands");
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
                WiphyBands::_2ghz(val) => fmt.field("_2ghz", &val),
                WiphyBands::_5ghz(val) => fmt.field("_5ghz", &val),
                WiphyBands::_60ghz(val) => fmt.field("_60ghz", &val),
                WiphyBands::_6ghz(val) => fmt.field("_6ghz", &val),
                WiphyBands::S1ghz(val) => fmt.field("S1ghz", &val),
                WiphyBands::Lc(val) => fmt.field("Lc", &val),
            };
        }
        fmt.finish()
    }
}
impl IterableWiphyBands<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if cur == offset {
            stack.push(("WiphyBands", offset));
            return (
                stack,
                missing_type.and_then(|t| WiphyBands::attr_from_type(t)),
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
                WiphyBands::_2ghz(val) => {
                    (stack, missing) = val.lookup_attr(offset, missing_type);
                    if !stack.is_empty() {
                        break;
                    }
                }
                WiphyBands::_5ghz(val) => {
                    (stack, missing) = val.lookup_attr(offset, missing_type);
                    if !stack.is_empty() {
                        break;
                    }
                }
                WiphyBands::_60ghz(val) => {
                    (stack, missing) = val.lookup_attr(offset, missing_type);
                    if !stack.is_empty() {
                        break;
                    }
                }
                WiphyBands::_6ghz(val) => {
                    (stack, missing) = val.lookup_attr(offset, missing_type);
                    if !stack.is_empty() {
                        break;
                    }
                }
                WiphyBands::S1ghz(val) => {
                    (stack, missing) = val.lookup_attr(offset, missing_type);
                    if !stack.is_empty() {
                        break;
                    }
                }
                WiphyBands::Lc(val) => {
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
            stack.push(("WiphyBands", cur));
        }
        (stack, missing)
    }
}
#[derive(Clone)]
pub enum BandAttrs<'a> {
    Freqs(IterableArrayFrequencyAttrs<'a>),
    Rates(IterableArrayBitrateAttrs<'a>),
    HtMcsSet(&'a [u8]),
    HtCapa(u16),
    HtAmpduFactor(u8),
    HtAmpduDensity(u8),
    VhtMcsSet(&'a [u8]),
    VhtCapa(u32),
    IftypeData(IterableArrayIftypeDataAttrs<'a>),
    EdmgChannels(&'a [u8]),
    EdmgBwConfig(&'a [u8]),
    S1gMcsNssSet(&'a [u8]),
    S1gCapa(&'a [u8]),
}
impl<'a> IterableBandAttrs<'a> {
    pub fn get_freqs(
        &self,
    ) -> Result<
        ArrayIterable<IterableArrayFrequencyAttrs<'a>, IterableFrequencyAttrs<'a>>,
        ErrorContext,
    > {
        for attr in self.clone() {
            if let BandAttrs::Freqs(val) = attr? {
                return Ok(ArrayIterable::new(val));
            }
        }
        Err(ErrorContext::new_missing(
            "BandAttrs",
            "Freqs",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_rates(
        &self,
    ) -> Result<ArrayIterable<IterableArrayBitrateAttrs<'a>, IterableBitrateAttrs<'a>>, ErrorContext>
    {
        for attr in self.clone() {
            if let BandAttrs::Rates(val) = attr? {
                return Ok(ArrayIterable::new(val));
            }
        }
        Err(ErrorContext::new_missing(
            "BandAttrs",
            "Rates",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_ht_mcs_set(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let BandAttrs::HtMcsSet(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "BandAttrs",
            "HtMcsSet",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_ht_capa(&self) -> Result<u16, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let BandAttrs::HtCapa(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "BandAttrs",
            "HtCapa",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_ht_ampdu_factor(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let BandAttrs::HtAmpduFactor(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "BandAttrs",
            "HtAmpduFactor",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_ht_ampdu_density(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let BandAttrs::HtAmpduDensity(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "BandAttrs",
            "HtAmpduDensity",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_vht_mcs_set(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let BandAttrs::VhtMcsSet(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "BandAttrs",
            "VhtMcsSet",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_vht_capa(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let BandAttrs::VhtCapa(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "BandAttrs",
            "VhtCapa",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_iftype_data(
        &self,
    ) -> Result<
        ArrayIterable<IterableArrayIftypeDataAttrs<'a>, IterableIftypeDataAttrs<'a>>,
        ErrorContext,
    > {
        for attr in self.clone() {
            if let BandAttrs::IftypeData(val) = attr? {
                return Ok(ArrayIterable::new(val));
            }
        }
        Err(ErrorContext::new_missing(
            "BandAttrs",
            "IftypeData",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_edmg_channels(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let BandAttrs::EdmgChannels(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "BandAttrs",
            "EdmgChannels",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_edmg_bw_config(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let BandAttrs::EdmgBwConfig(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "BandAttrs",
            "EdmgBwConfig",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_s1g_mcs_nss_set(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let BandAttrs::S1gMcsNssSet(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "BandAttrs",
            "S1gMcsNssSet",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_s1g_capa(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let BandAttrs::S1gCapa(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "BandAttrs",
            "S1gCapa",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl<'a> FrequencyAttrs<'a> {
    pub fn new_array(buf: &[u8]) -> IterableArrayFrequencyAttrs<'_> {
        IterableArrayFrequencyAttrs::with_loc(buf, buf.as_ptr() as usize)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableArrayFrequencyAttrs<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableArrayFrequencyAttrs<'a> {
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
impl<'a> Iterator for IterableArrayFrequencyAttrs<'a> {
    type Item = Result<IterableFrequencyAttrs<'a>, ErrorContext>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.buf.len() == self.pos {
            return None;
        }
        while let Some((header, next)) = chop_header(self.buf, &mut self.pos) {
            {
                return Some(Ok(IterableFrequencyAttrs::with_loc(next, self.orig_loc)));
            }
        }
        Some(Err(ErrorContext::new(
            "FrequencyAttrs",
            None,
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(self.pos) as usize,
        )))
    }
}
impl BitrateAttrs {
    pub fn new_array(buf: &[u8]) -> IterableArrayBitrateAttrs<'_> {
        IterableArrayBitrateAttrs::with_loc(buf, buf.as_ptr() as usize)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableArrayBitrateAttrs<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableArrayBitrateAttrs<'a> {
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
impl<'a> Iterator for IterableArrayBitrateAttrs<'a> {
    type Item = Result<IterableBitrateAttrs<'a>, ErrorContext>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.buf.len() == self.pos {
            return None;
        }
        while let Some((header, next)) = chop_header(self.buf, &mut self.pos) {
            {
                return Some(Ok(IterableBitrateAttrs::with_loc(next, self.orig_loc)));
            }
        }
        Some(Err(ErrorContext::new(
            "BitrateAttrs",
            None,
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(self.pos) as usize,
        )))
    }
}
impl<'a> IftypeDataAttrs<'a> {
    pub fn new_array(buf: &[u8]) -> IterableArrayIftypeDataAttrs<'_> {
        IterableArrayIftypeDataAttrs::with_loc(buf, buf.as_ptr() as usize)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableArrayIftypeDataAttrs<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableArrayIftypeDataAttrs<'a> {
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
impl<'a> Iterator for IterableArrayIftypeDataAttrs<'a> {
    type Item = Result<IterableIftypeDataAttrs<'a>, ErrorContext>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.buf.len() == self.pos {
            return None;
        }
        while let Some((header, next)) = chop_header(self.buf, &mut self.pos) {
            {
                return Some(Ok(IterableIftypeDataAttrs::with_loc(next, self.orig_loc)));
            }
        }
        Some(Err(ErrorContext::new(
            "IftypeDataAttrs",
            None,
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(self.pos) as usize,
        )))
    }
}
impl<'a> BandAttrs<'a> {
    pub fn new(buf: &'a [u8]) -> IterableBandAttrs<'a> {
        IterableBandAttrs::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        let res = match r#type {
            1u16 => "Freqs",
            2u16 => "Rates",
            3u16 => "HtMcsSet",
            4u16 => "HtCapa",
            5u16 => "HtAmpduFactor",
            6u16 => "HtAmpduDensity",
            7u16 => "VhtMcsSet",
            8u16 => "VhtCapa",
            9u16 => "IftypeData",
            10u16 => "EdmgChannels",
            11u16 => "EdmgBwConfig",
            12u16 => "S1gMcsNssSet",
            13u16 => "S1gCapa",
            _ => return None,
        };
        Some(res)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableBandAttrs<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableBandAttrs<'a> {
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
impl<'a> Iterator for IterableBandAttrs<'a> {
    type Item = Result<BandAttrs<'a>, ErrorContext>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.buf.len() == self.pos {
            return None;
        }
        let pos = self.pos;
        let mut r#type = None;
        while let Some((header, next)) = chop_header(self.buf, &mut self.pos) {
            r#type = Some(header.r#type);
            let res = match header.r#type {
                1u16 => BandAttrs::Freqs({
                    let res = Some(IterableArrayFrequencyAttrs::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                2u16 => BandAttrs::Rates({
                    let res = Some(IterableArrayBitrateAttrs::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                3u16 => BandAttrs::HtMcsSet({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                4u16 => BandAttrs::HtCapa({
                    let res = parse_u16(next);
                    let Some(val) = res else { break };
                    val
                }),
                5u16 => BandAttrs::HtAmpduFactor({
                    let res = parse_u8(next);
                    let Some(val) = res else { break };
                    val
                }),
                6u16 => BandAttrs::HtAmpduDensity({
                    let res = parse_u8(next);
                    let Some(val) = res else { break };
                    val
                }),
                7u16 => BandAttrs::VhtMcsSet({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                8u16 => BandAttrs::VhtCapa({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                9u16 => BandAttrs::IftypeData({
                    let res = Some(IterableArrayIftypeDataAttrs::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                10u16 => BandAttrs::EdmgChannels({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                11u16 => BandAttrs::EdmgBwConfig({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                12u16 => BandAttrs::S1gMcsNssSet({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                13u16 => BandAttrs::S1gCapa({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                n => {
                    if cfg!(any(test, feature = "deny-unknown-attrs")) {
                        break;
                    } else {
                        continue;
                    }
                }
            };
            return Some(Ok(res));
        }
        Some(Err(ErrorContext::new(
            "BandAttrs",
            r#type.and_then(|t| BandAttrs::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl std::fmt::Debug for IterableArrayFrequencyAttrs<'_> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt.debug_list()
            .entries(self.clone().map(FlattenErrorContext))
            .finish()
    }
}
impl std::fmt::Debug for IterableArrayBitrateAttrs<'_> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt.debug_list()
            .entries(self.clone().map(FlattenErrorContext))
            .finish()
    }
}
impl std::fmt::Debug for IterableArrayIftypeDataAttrs<'_> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt.debug_list()
            .entries(self.clone().map(FlattenErrorContext))
            .finish()
    }
}
impl<'a> std::fmt::Debug for IterableBandAttrs<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("BandAttrs");
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
                BandAttrs::Freqs(val) => fmt.field("Freqs", &val),
                BandAttrs::Rates(val) => fmt.field("Rates", &val),
                BandAttrs::HtMcsSet(val) => fmt.field("HtMcsSet", &val),
                BandAttrs::HtCapa(val) => fmt.field("HtCapa", &val),
                BandAttrs::HtAmpduFactor(val) => fmt.field("HtAmpduFactor", &val),
                BandAttrs::HtAmpduDensity(val) => fmt.field("HtAmpduDensity", &val),
                BandAttrs::VhtMcsSet(val) => fmt.field("VhtMcsSet", &val),
                BandAttrs::VhtCapa(val) => fmt.field("VhtCapa", &val),
                BandAttrs::IftypeData(val) => fmt.field("IftypeData", &val),
                BandAttrs::EdmgChannels(val) => fmt.field("EdmgChannels", &val),
                BandAttrs::EdmgBwConfig(val) => fmt.field("EdmgBwConfig", &val),
                BandAttrs::S1gMcsNssSet(val) => fmt.field("S1gMcsNssSet", &val),
                BandAttrs::S1gCapa(val) => fmt.field("S1gCapa", &val),
            };
        }
        fmt.finish()
    }
}
impl IterableBandAttrs<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if cur == offset {
            stack.push(("BandAttrs", offset));
            return (
                stack,
                missing_type.and_then(|t| BandAttrs::attr_from_type(t)),
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
                BandAttrs::Freqs(val) => {
                    for entry in val {
                        let Ok(attr) = entry else { break };
                        (stack, missing) = attr.lookup_attr(offset, missing_type);
                        if !stack.is_empty() {
                            break;
                        }
                    }
                    if !stack.is_empty() {
                        stack.push(("Freqs", last_off));
                        break;
                    }
                }
                BandAttrs::Rates(val) => {
                    for entry in val {
                        let Ok(attr) = entry else { break };
                        (stack, missing) = attr.lookup_attr(offset, missing_type);
                        if !stack.is_empty() {
                            break;
                        }
                    }
                    if !stack.is_empty() {
                        stack.push(("Rates", last_off));
                        break;
                    }
                }
                BandAttrs::HtMcsSet(val) => {
                    if last_off == offset {
                        stack.push(("HtMcsSet", last_off));
                        break;
                    }
                }
                BandAttrs::HtCapa(val) => {
                    if last_off == offset {
                        stack.push(("HtCapa", last_off));
                        break;
                    }
                }
                BandAttrs::HtAmpduFactor(val) => {
                    if last_off == offset {
                        stack.push(("HtAmpduFactor", last_off));
                        break;
                    }
                }
                BandAttrs::HtAmpduDensity(val) => {
                    if last_off == offset {
                        stack.push(("HtAmpduDensity", last_off));
                        break;
                    }
                }
                BandAttrs::VhtMcsSet(val) => {
                    if last_off == offset {
                        stack.push(("VhtMcsSet", last_off));
                        break;
                    }
                }
                BandAttrs::VhtCapa(val) => {
                    if last_off == offset {
                        stack.push(("VhtCapa", last_off));
                        break;
                    }
                }
                BandAttrs::IftypeData(val) => {
                    for entry in val {
                        let Ok(attr) = entry else { break };
                        (stack, missing) = attr.lookup_attr(offset, missing_type);
                        if !stack.is_empty() {
                            break;
                        }
                    }
                    if !stack.is_empty() {
                        stack.push(("IftypeData", last_off));
                        break;
                    }
                }
                BandAttrs::EdmgChannels(val) => {
                    if last_off == offset {
                        stack.push(("EdmgChannels", last_off));
                        break;
                    }
                }
                BandAttrs::EdmgBwConfig(val) => {
                    if last_off == offset {
                        stack.push(("EdmgBwConfig", last_off));
                        break;
                    }
                }
                BandAttrs::S1gMcsNssSet(val) => {
                    if last_off == offset {
                        stack.push(("S1gMcsNssSet", last_off));
                        break;
                    }
                }
                BandAttrs::S1gCapa(val) => {
                    if last_off == offset {
                        stack.push(("S1gCapa", last_off));
                        break;
                    }
                }
                _ => {}
            };
            last_off = cur + attrs.pos;
        }
        if !stack.is_empty() {
            stack.push(("BandAttrs", cur));
        }
        (stack, missing)
    }
}
#[derive(Clone)]
pub enum BitrateAttrs {
    Rate(u32),
    _2ghzShortpreamble(()),
}
impl<'a> IterableBitrateAttrs<'a> {
    pub fn get_rate(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let BitrateAttrs::Rate(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "BitrateAttrs",
            "Rate",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_2ghz_shortpreamble(&self) -> Result<(), ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let BitrateAttrs::_2ghzShortpreamble(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "BitrateAttrs",
            "2ghzShortpreamble",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl BitrateAttrs {
    pub fn new(buf: &'_ [u8]) -> IterableBitrateAttrs<'_> {
        IterableBitrateAttrs::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        let res = match r#type {
            1u16 => "Rate",
            2u16 => "2ghzShortpreamble",
            _ => return None,
        };
        Some(res)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableBitrateAttrs<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableBitrateAttrs<'a> {
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
impl<'a> Iterator for IterableBitrateAttrs<'a> {
    type Item = Result<BitrateAttrs, ErrorContext>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.buf.len() == self.pos {
            return None;
        }
        let pos = self.pos;
        let mut r#type = None;
        while let Some((header, next)) = chop_header(self.buf, &mut self.pos) {
            r#type = Some(header.r#type);
            let res = match header.r#type {
                1u16 => BitrateAttrs::Rate({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                2u16 => BitrateAttrs::_2ghzShortpreamble(()),
                n => {
                    if cfg!(any(test, feature = "deny-unknown-attrs")) {
                        break;
                    } else {
                        continue;
                    }
                }
            };
            return Some(Ok(res));
        }
        Some(Err(ErrorContext::new(
            "BitrateAttrs",
            r#type.and_then(|t| BitrateAttrs::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl std::fmt::Debug for IterableBitrateAttrs<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("BitrateAttrs");
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
                BitrateAttrs::Rate(val) => fmt.field("Rate", &val),
                BitrateAttrs::_2ghzShortpreamble(val) => fmt.field("_2ghzShortpreamble", &val),
            };
        }
        fmt.finish()
    }
}
impl IterableBitrateAttrs<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if cur == offset {
            stack.push(("BitrateAttrs", offset));
            return (
                stack,
                missing_type.and_then(|t| BitrateAttrs::attr_from_type(t)),
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
                BitrateAttrs::Rate(val) => {
                    if last_off == offset {
                        stack.push(("Rate", last_off));
                        break;
                    }
                }
                BitrateAttrs::_2ghzShortpreamble(val) => {
                    if last_off == offset {
                        stack.push(("2ghzShortpreamble", last_off));
                        break;
                    }
                }
                _ => {}
            };
            last_off = cur + attrs.pos;
        }
        if !stack.is_empty() {
            stack.push(("BitrateAttrs", cur));
        }
        (stack, None)
    }
}
#[derive(Clone)]
pub enum FrequencyAttrs<'a> {
    Freq(u32),
    Disabled(()),
    NoIr(()),
    NoIbss(()),
    Radar(()),
    MaxTxPower(u32),
    DfsState(u32),
    DfsTime(&'a [u8]),
    NoHt40Minus(&'a [u8]),
    NoHt40Plus(&'a [u8]),
    No80mhz(&'a [u8]),
    No160mhz(&'a [u8]),
    DfsCacTime(&'a [u8]),
    IndoorOnly(&'a [u8]),
    IrConcurrent(&'a [u8]),
    No20mhz(&'a [u8]),
    No10mhz(&'a [u8]),
    Wmm(IterableArrayWmmAttrs<'a>),
    NoHe(&'a [u8]),
    Offset(u32),
    _1mhz(&'a [u8]),
    _2mhz(&'a [u8]),
    _4mhz(&'a [u8]),
    _8mhz(&'a [u8]),
    _16mhz(&'a [u8]),
    No320mhz(&'a [u8]),
    NoEht(&'a [u8]),
    Psd(&'a [u8]),
    DfsConcurrent(&'a [u8]),
    No6ghzVlpClient(&'a [u8]),
    No6ghzAfcClient(&'a [u8]),
    CanMonitor(&'a [u8]),
    Allow6ghzVlpAp(&'a [u8]),
}
impl<'a> IterableFrequencyAttrs<'a> {
    pub fn get_freq(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let FrequencyAttrs::Freq(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "FrequencyAttrs",
            "Freq",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_disabled(&self) -> Result<(), ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let FrequencyAttrs::Disabled(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "FrequencyAttrs",
            "Disabled",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_no_ir(&self) -> Result<(), ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let FrequencyAttrs::NoIr(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "FrequencyAttrs",
            "NoIr",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_no_ibss(&self) -> Result<(), ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let FrequencyAttrs::NoIbss(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "FrequencyAttrs",
            "NoIbss",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_radar(&self) -> Result<(), ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let FrequencyAttrs::Radar(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "FrequencyAttrs",
            "Radar",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_max_tx_power(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let FrequencyAttrs::MaxTxPower(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "FrequencyAttrs",
            "MaxTxPower",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_dfs_state(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let FrequencyAttrs::DfsState(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "FrequencyAttrs",
            "DfsState",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_dfs_time(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let FrequencyAttrs::DfsTime(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "FrequencyAttrs",
            "DfsTime",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_no_ht40_minus(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let FrequencyAttrs::NoHt40Minus(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "FrequencyAttrs",
            "NoHt40Minus",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_no_ht40_plus(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let FrequencyAttrs::NoHt40Plus(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "FrequencyAttrs",
            "NoHt40Plus",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_no_80mhz(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let FrequencyAttrs::No80mhz(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "FrequencyAttrs",
            "No80mhz",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_no_160mhz(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let FrequencyAttrs::No160mhz(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "FrequencyAttrs",
            "No160mhz",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_dfs_cac_time(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let FrequencyAttrs::DfsCacTime(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "FrequencyAttrs",
            "DfsCacTime",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_indoor_only(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let FrequencyAttrs::IndoorOnly(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "FrequencyAttrs",
            "IndoorOnly",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_ir_concurrent(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let FrequencyAttrs::IrConcurrent(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "FrequencyAttrs",
            "IrConcurrent",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_no_20mhz(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let FrequencyAttrs::No20mhz(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "FrequencyAttrs",
            "No20mhz",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_no_10mhz(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let FrequencyAttrs::No10mhz(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "FrequencyAttrs",
            "No10mhz",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_wmm(
        &self,
    ) -> Result<ArrayIterable<IterableArrayWmmAttrs<'a>, IterableWmmAttrs<'a>>, ErrorContext> {
        for attr in self.clone() {
            if let FrequencyAttrs::Wmm(val) = attr? {
                return Ok(ArrayIterable::new(val));
            }
        }
        Err(ErrorContext::new_missing(
            "FrequencyAttrs",
            "Wmm",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_no_he(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let FrequencyAttrs::NoHe(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "FrequencyAttrs",
            "NoHe",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_offset(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let FrequencyAttrs::Offset(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "FrequencyAttrs",
            "Offset",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_1mhz(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let FrequencyAttrs::_1mhz(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "FrequencyAttrs",
            "1mhz",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_2mhz(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let FrequencyAttrs::_2mhz(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "FrequencyAttrs",
            "2mhz",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_4mhz(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let FrequencyAttrs::_4mhz(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "FrequencyAttrs",
            "4mhz",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_8mhz(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let FrequencyAttrs::_8mhz(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "FrequencyAttrs",
            "8mhz",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_16mhz(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let FrequencyAttrs::_16mhz(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "FrequencyAttrs",
            "16mhz",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_no_320mhz(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let FrequencyAttrs::No320mhz(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "FrequencyAttrs",
            "No320mhz",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_no_eht(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let FrequencyAttrs::NoEht(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "FrequencyAttrs",
            "NoEht",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_psd(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let FrequencyAttrs::Psd(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "FrequencyAttrs",
            "Psd",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_dfs_concurrent(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let FrequencyAttrs::DfsConcurrent(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "FrequencyAttrs",
            "DfsConcurrent",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_no_6ghz_vlp_client(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let FrequencyAttrs::No6ghzVlpClient(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "FrequencyAttrs",
            "No6ghzVlpClient",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_no_6ghz_afc_client(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let FrequencyAttrs::No6ghzAfcClient(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "FrequencyAttrs",
            "No6ghzAfcClient",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_can_monitor(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let FrequencyAttrs::CanMonitor(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "FrequencyAttrs",
            "CanMonitor",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_allow_6ghz_vlp_ap(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let FrequencyAttrs::Allow6ghzVlpAp(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "FrequencyAttrs",
            "Allow6ghzVlpAp",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl WmmAttrs {
    pub fn new_array(buf: &[u8]) -> IterableArrayWmmAttrs<'_> {
        IterableArrayWmmAttrs::with_loc(buf, buf.as_ptr() as usize)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableArrayWmmAttrs<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableArrayWmmAttrs<'a> {
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
impl<'a> Iterator for IterableArrayWmmAttrs<'a> {
    type Item = Result<IterableWmmAttrs<'a>, ErrorContext>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.buf.len() == self.pos {
            return None;
        }
        while let Some((header, next)) = chop_header(self.buf, &mut self.pos) {
            {
                return Some(Ok(IterableWmmAttrs::with_loc(next, self.orig_loc)));
            }
        }
        Some(Err(ErrorContext::new(
            "WmmAttrs",
            None,
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(self.pos) as usize,
        )))
    }
}
impl<'a> FrequencyAttrs<'a> {
    pub fn new(buf: &'a [u8]) -> IterableFrequencyAttrs<'a> {
        IterableFrequencyAttrs::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        let res = match r#type {
            1u16 => "Freq",
            2u16 => "Disabled",
            3u16 => "NoIr",
            4u16 => "NoIbss",
            5u16 => "Radar",
            6u16 => "MaxTxPower",
            7u16 => "DfsState",
            8u16 => "DfsTime",
            9u16 => "NoHt40Minus",
            10u16 => "NoHt40Plus",
            11u16 => "No80mhz",
            12u16 => "No160mhz",
            13u16 => "DfsCacTime",
            14u16 => "IndoorOnly",
            15u16 => "IrConcurrent",
            16u16 => "No20mhz",
            17u16 => "No10mhz",
            18u16 => "Wmm",
            19u16 => "NoHe",
            20u16 => "Offset",
            21u16 => "1mhz",
            22u16 => "2mhz",
            23u16 => "4mhz",
            24u16 => "8mhz",
            25u16 => "16mhz",
            26u16 => "No320mhz",
            27u16 => "NoEht",
            28u16 => "Psd",
            29u16 => "DfsConcurrent",
            30u16 => "No6ghzVlpClient",
            31u16 => "No6ghzAfcClient",
            32u16 => "CanMonitor",
            33u16 => "Allow6ghzVlpAp",
            _ => return None,
        };
        Some(res)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableFrequencyAttrs<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableFrequencyAttrs<'a> {
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
impl<'a> Iterator for IterableFrequencyAttrs<'a> {
    type Item = Result<FrequencyAttrs<'a>, ErrorContext>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.buf.len() == self.pos {
            return None;
        }
        let pos = self.pos;
        let mut r#type = None;
        while let Some((header, next)) = chop_header(self.buf, &mut self.pos) {
            r#type = Some(header.r#type);
            let res = match header.r#type {
                1u16 => FrequencyAttrs::Freq({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                2u16 => FrequencyAttrs::Disabled(()),
                3u16 => FrequencyAttrs::NoIr(()),
                4u16 => FrequencyAttrs::NoIbss(()),
                5u16 => FrequencyAttrs::Radar(()),
                6u16 => FrequencyAttrs::MaxTxPower({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                7u16 => FrequencyAttrs::DfsState({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                8u16 => FrequencyAttrs::DfsTime({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                9u16 => FrequencyAttrs::NoHt40Minus({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                10u16 => FrequencyAttrs::NoHt40Plus({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                11u16 => FrequencyAttrs::No80mhz({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                12u16 => FrequencyAttrs::No160mhz({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                13u16 => FrequencyAttrs::DfsCacTime({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                14u16 => FrequencyAttrs::IndoorOnly({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                15u16 => FrequencyAttrs::IrConcurrent({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                16u16 => FrequencyAttrs::No20mhz({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                17u16 => FrequencyAttrs::No10mhz({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                18u16 => FrequencyAttrs::Wmm({
                    let res = Some(IterableArrayWmmAttrs::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                19u16 => FrequencyAttrs::NoHe({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                20u16 => FrequencyAttrs::Offset({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                21u16 => FrequencyAttrs::_1mhz({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                22u16 => FrequencyAttrs::_2mhz({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                23u16 => FrequencyAttrs::_4mhz({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                24u16 => FrequencyAttrs::_8mhz({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                25u16 => FrequencyAttrs::_16mhz({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                26u16 => FrequencyAttrs::No320mhz({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                27u16 => FrequencyAttrs::NoEht({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                28u16 => FrequencyAttrs::Psd({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                29u16 => FrequencyAttrs::DfsConcurrent({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                30u16 => FrequencyAttrs::No6ghzVlpClient({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                31u16 => FrequencyAttrs::No6ghzAfcClient({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                32u16 => FrequencyAttrs::CanMonitor({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                33u16 => FrequencyAttrs::Allow6ghzVlpAp({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                n => {
                    if cfg!(any(test, feature = "deny-unknown-attrs")) {
                        break;
                    } else {
                        continue;
                    }
                }
            };
            return Some(Ok(res));
        }
        Some(Err(ErrorContext::new(
            "FrequencyAttrs",
            r#type.and_then(|t| FrequencyAttrs::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl std::fmt::Debug for IterableArrayWmmAttrs<'_> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt.debug_list()
            .entries(self.clone().map(FlattenErrorContext))
            .finish()
    }
}
impl<'a> std::fmt::Debug for IterableFrequencyAttrs<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("FrequencyAttrs");
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
                FrequencyAttrs::Freq(val) => fmt.field("Freq", &val),
                FrequencyAttrs::Disabled(val) => fmt.field("Disabled", &val),
                FrequencyAttrs::NoIr(val) => fmt.field("NoIr", &val),
                FrequencyAttrs::NoIbss(val) => fmt.field("NoIbss", &val),
                FrequencyAttrs::Radar(val) => fmt.field("Radar", &val),
                FrequencyAttrs::MaxTxPower(val) => fmt.field("MaxTxPower", &val),
                FrequencyAttrs::DfsState(val) => fmt.field("DfsState", &val),
                FrequencyAttrs::DfsTime(val) => fmt.field("DfsTime", &val),
                FrequencyAttrs::NoHt40Minus(val) => fmt.field("NoHt40Minus", &val),
                FrequencyAttrs::NoHt40Plus(val) => fmt.field("NoHt40Plus", &val),
                FrequencyAttrs::No80mhz(val) => fmt.field("No80mhz", &val),
                FrequencyAttrs::No160mhz(val) => fmt.field("No160mhz", &val),
                FrequencyAttrs::DfsCacTime(val) => fmt.field("DfsCacTime", &val),
                FrequencyAttrs::IndoorOnly(val) => fmt.field("IndoorOnly", &val),
                FrequencyAttrs::IrConcurrent(val) => fmt.field("IrConcurrent", &val),
                FrequencyAttrs::No20mhz(val) => fmt.field("No20mhz", &val),
                FrequencyAttrs::No10mhz(val) => fmt.field("No10mhz", &val),
                FrequencyAttrs::Wmm(val) => fmt.field("Wmm", &val),
                FrequencyAttrs::NoHe(val) => fmt.field("NoHe", &val),
                FrequencyAttrs::Offset(val) => fmt.field("Offset", &val),
                FrequencyAttrs::_1mhz(val) => fmt.field("_1mhz", &val),
                FrequencyAttrs::_2mhz(val) => fmt.field("_2mhz", &val),
                FrequencyAttrs::_4mhz(val) => fmt.field("_4mhz", &val),
                FrequencyAttrs::_8mhz(val) => fmt.field("_8mhz", &val),
                FrequencyAttrs::_16mhz(val) => fmt.field("_16mhz", &val),
                FrequencyAttrs::No320mhz(val) => fmt.field("No320mhz", &val),
                FrequencyAttrs::NoEht(val) => fmt.field("NoEht", &val),
                FrequencyAttrs::Psd(val) => fmt.field("Psd", &val),
                FrequencyAttrs::DfsConcurrent(val) => fmt.field("DfsConcurrent", &val),
                FrequencyAttrs::No6ghzVlpClient(val) => fmt.field("No6ghzVlpClient", &val),
                FrequencyAttrs::No6ghzAfcClient(val) => fmt.field("No6ghzAfcClient", &val),
                FrequencyAttrs::CanMonitor(val) => fmt.field("CanMonitor", &val),
                FrequencyAttrs::Allow6ghzVlpAp(val) => fmt.field("Allow6ghzVlpAp", &val),
            };
        }
        fmt.finish()
    }
}
impl IterableFrequencyAttrs<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if cur == offset {
            stack.push(("FrequencyAttrs", offset));
            return (
                stack,
                missing_type.and_then(|t| FrequencyAttrs::attr_from_type(t)),
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
                FrequencyAttrs::Freq(val) => {
                    if last_off == offset {
                        stack.push(("Freq", last_off));
                        break;
                    }
                }
                FrequencyAttrs::Disabled(val) => {
                    if last_off == offset {
                        stack.push(("Disabled", last_off));
                        break;
                    }
                }
                FrequencyAttrs::NoIr(val) => {
                    if last_off == offset {
                        stack.push(("NoIr", last_off));
                        break;
                    }
                }
                FrequencyAttrs::NoIbss(val) => {
                    if last_off == offset {
                        stack.push(("NoIbss", last_off));
                        break;
                    }
                }
                FrequencyAttrs::Radar(val) => {
                    if last_off == offset {
                        stack.push(("Radar", last_off));
                        break;
                    }
                }
                FrequencyAttrs::MaxTxPower(val) => {
                    if last_off == offset {
                        stack.push(("MaxTxPower", last_off));
                        break;
                    }
                }
                FrequencyAttrs::DfsState(val) => {
                    if last_off == offset {
                        stack.push(("DfsState", last_off));
                        break;
                    }
                }
                FrequencyAttrs::DfsTime(val) => {
                    if last_off == offset {
                        stack.push(("DfsTime", last_off));
                        break;
                    }
                }
                FrequencyAttrs::NoHt40Minus(val) => {
                    if last_off == offset {
                        stack.push(("NoHt40Minus", last_off));
                        break;
                    }
                }
                FrequencyAttrs::NoHt40Plus(val) => {
                    if last_off == offset {
                        stack.push(("NoHt40Plus", last_off));
                        break;
                    }
                }
                FrequencyAttrs::No80mhz(val) => {
                    if last_off == offset {
                        stack.push(("No80mhz", last_off));
                        break;
                    }
                }
                FrequencyAttrs::No160mhz(val) => {
                    if last_off == offset {
                        stack.push(("No160mhz", last_off));
                        break;
                    }
                }
                FrequencyAttrs::DfsCacTime(val) => {
                    if last_off == offset {
                        stack.push(("DfsCacTime", last_off));
                        break;
                    }
                }
                FrequencyAttrs::IndoorOnly(val) => {
                    if last_off == offset {
                        stack.push(("IndoorOnly", last_off));
                        break;
                    }
                }
                FrequencyAttrs::IrConcurrent(val) => {
                    if last_off == offset {
                        stack.push(("IrConcurrent", last_off));
                        break;
                    }
                }
                FrequencyAttrs::No20mhz(val) => {
                    if last_off == offset {
                        stack.push(("No20mhz", last_off));
                        break;
                    }
                }
                FrequencyAttrs::No10mhz(val) => {
                    if last_off == offset {
                        stack.push(("No10mhz", last_off));
                        break;
                    }
                }
                FrequencyAttrs::Wmm(val) => {
                    for entry in val {
                        let Ok(attr) = entry else { break };
                        (stack, missing) = attr.lookup_attr(offset, missing_type);
                        if !stack.is_empty() {
                            break;
                        }
                    }
                    if !stack.is_empty() {
                        stack.push(("Wmm", last_off));
                        break;
                    }
                }
                FrequencyAttrs::NoHe(val) => {
                    if last_off == offset {
                        stack.push(("NoHe", last_off));
                        break;
                    }
                }
                FrequencyAttrs::Offset(val) => {
                    if last_off == offset {
                        stack.push(("Offset", last_off));
                        break;
                    }
                }
                FrequencyAttrs::_1mhz(val) => {
                    if last_off == offset {
                        stack.push(("1mhz", last_off));
                        break;
                    }
                }
                FrequencyAttrs::_2mhz(val) => {
                    if last_off == offset {
                        stack.push(("2mhz", last_off));
                        break;
                    }
                }
                FrequencyAttrs::_4mhz(val) => {
                    if last_off == offset {
                        stack.push(("4mhz", last_off));
                        break;
                    }
                }
                FrequencyAttrs::_8mhz(val) => {
                    if last_off == offset {
                        stack.push(("8mhz", last_off));
                        break;
                    }
                }
                FrequencyAttrs::_16mhz(val) => {
                    if last_off == offset {
                        stack.push(("16mhz", last_off));
                        break;
                    }
                }
                FrequencyAttrs::No320mhz(val) => {
                    if last_off == offset {
                        stack.push(("No320mhz", last_off));
                        break;
                    }
                }
                FrequencyAttrs::NoEht(val) => {
                    if last_off == offset {
                        stack.push(("NoEht", last_off));
                        break;
                    }
                }
                FrequencyAttrs::Psd(val) => {
                    if last_off == offset {
                        stack.push(("Psd", last_off));
                        break;
                    }
                }
                FrequencyAttrs::DfsConcurrent(val) => {
                    if last_off == offset {
                        stack.push(("DfsConcurrent", last_off));
                        break;
                    }
                }
                FrequencyAttrs::No6ghzVlpClient(val) => {
                    if last_off == offset {
                        stack.push(("No6ghzVlpClient", last_off));
                        break;
                    }
                }
                FrequencyAttrs::No6ghzAfcClient(val) => {
                    if last_off == offset {
                        stack.push(("No6ghzAfcClient", last_off));
                        break;
                    }
                }
                FrequencyAttrs::CanMonitor(val) => {
                    if last_off == offset {
                        stack.push(("CanMonitor", last_off));
                        break;
                    }
                }
                FrequencyAttrs::Allow6ghzVlpAp(val) => {
                    if last_off == offset {
                        stack.push(("Allow6ghzVlpAp", last_off));
                        break;
                    }
                }
                _ => {}
            };
            last_off = cur + attrs.pos;
        }
        if !stack.is_empty() {
            stack.push(("FrequencyAttrs", cur));
        }
        (stack, missing)
    }
}
#[derive(Clone)]
pub enum IfaceLimitAttributes<'a> {
    Max(u32),
    Types(IterableSupportedIftypes<'a>),
}
impl<'a> IterableIfaceLimitAttributes<'a> {
    pub fn get_max(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let IfaceLimitAttributes::Max(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "IfaceLimitAttributes",
            "Max",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_types(&self) -> Result<IterableSupportedIftypes<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let IfaceLimitAttributes::Types(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "IfaceLimitAttributes",
            "Types",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl<'a> IfaceLimitAttributes<'a> {
    pub fn new(buf: &'a [u8]) -> IterableIfaceLimitAttributes<'a> {
        IterableIfaceLimitAttributes::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        let res = match r#type {
            1u16 => "Max",
            2u16 => "Types",
            _ => return None,
        };
        Some(res)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableIfaceLimitAttributes<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableIfaceLimitAttributes<'a> {
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
impl<'a> Iterator for IterableIfaceLimitAttributes<'a> {
    type Item = Result<IfaceLimitAttributes<'a>, ErrorContext>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.buf.len() == self.pos {
            return None;
        }
        let pos = self.pos;
        let mut r#type = None;
        while let Some((header, next)) = chop_header(self.buf, &mut self.pos) {
            r#type = Some(header.r#type);
            let res = match header.r#type {
                1u16 => IfaceLimitAttributes::Max({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                2u16 => IfaceLimitAttributes::Types({
                    let res = Some(IterableSupportedIftypes::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                n => {
                    if cfg!(any(test, feature = "deny-unknown-attrs")) {
                        break;
                    } else {
                        continue;
                    }
                }
            };
            return Some(Ok(res));
        }
        Some(Err(ErrorContext::new(
            "IfaceLimitAttributes",
            r#type.and_then(|t| IfaceLimitAttributes::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl<'a> std::fmt::Debug for IterableIfaceLimitAttributes<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("IfaceLimitAttributes");
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
                IfaceLimitAttributes::Max(val) => fmt.field("Max", &val),
                IfaceLimitAttributes::Types(val) => fmt.field("Types", &val),
            };
        }
        fmt.finish()
    }
}
impl IterableIfaceLimitAttributes<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if cur == offset {
            stack.push(("IfaceLimitAttributes", offset));
            return (
                stack,
                missing_type.and_then(|t| IfaceLimitAttributes::attr_from_type(t)),
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
                IfaceLimitAttributes::Max(val) => {
                    if last_off == offset {
                        stack.push(("Max", last_off));
                        break;
                    }
                }
                IfaceLimitAttributes::Types(val) => {
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
            stack.push(("IfaceLimitAttributes", cur));
        }
        (stack, missing)
    }
}
#[derive(Clone)]
pub enum IftypeDataAttrs<'a> {
    Iftypes(&'a [u8]),
    HeCapMac(&'a [u8]),
    HeCapPhy(&'a [u8]),
    HeCapMcsSet(&'a [u8]),
    HeCapPpe(&'a [u8]),
    He6ghzCapa(&'a [u8]),
    VendorElems(&'a [u8]),
    EhtCapMac(&'a [u8]),
    EhtCapPhy(&'a [u8]),
    EhtCapMcsSet(&'a [u8]),
    EhtCapPpe(&'a [u8]),
}
impl<'a> IterableIftypeDataAttrs<'a> {
    pub fn get_iftypes(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let IftypeDataAttrs::Iftypes(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "IftypeDataAttrs",
            "Iftypes",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_he_cap_mac(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let IftypeDataAttrs::HeCapMac(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "IftypeDataAttrs",
            "HeCapMac",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_he_cap_phy(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let IftypeDataAttrs::HeCapPhy(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "IftypeDataAttrs",
            "HeCapPhy",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_he_cap_mcs_set(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let IftypeDataAttrs::HeCapMcsSet(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "IftypeDataAttrs",
            "HeCapMcsSet",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_he_cap_ppe(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let IftypeDataAttrs::HeCapPpe(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "IftypeDataAttrs",
            "HeCapPpe",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_he_6ghz_capa(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let IftypeDataAttrs::He6ghzCapa(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "IftypeDataAttrs",
            "He6ghzCapa",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_vendor_elems(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let IftypeDataAttrs::VendorElems(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "IftypeDataAttrs",
            "VendorElems",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_eht_cap_mac(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let IftypeDataAttrs::EhtCapMac(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "IftypeDataAttrs",
            "EhtCapMac",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_eht_cap_phy(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let IftypeDataAttrs::EhtCapPhy(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "IftypeDataAttrs",
            "EhtCapPhy",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_eht_cap_mcs_set(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let IftypeDataAttrs::EhtCapMcsSet(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "IftypeDataAttrs",
            "EhtCapMcsSet",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_eht_cap_ppe(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let IftypeDataAttrs::EhtCapPpe(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "IftypeDataAttrs",
            "EhtCapPpe",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl<'a> IftypeDataAttrs<'a> {
    pub fn new(buf: &'a [u8]) -> IterableIftypeDataAttrs<'a> {
        IterableIftypeDataAttrs::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        let res = match r#type {
            1u16 => "Iftypes",
            2u16 => "HeCapMac",
            3u16 => "HeCapPhy",
            4u16 => "HeCapMcsSet",
            5u16 => "HeCapPpe",
            6u16 => "He6ghzCapa",
            7u16 => "VendorElems",
            8u16 => "EhtCapMac",
            9u16 => "EhtCapPhy",
            10u16 => "EhtCapMcsSet",
            11u16 => "EhtCapPpe",
            _ => return None,
        };
        Some(res)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableIftypeDataAttrs<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableIftypeDataAttrs<'a> {
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
impl<'a> Iterator for IterableIftypeDataAttrs<'a> {
    type Item = Result<IftypeDataAttrs<'a>, ErrorContext>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.buf.len() == self.pos {
            return None;
        }
        let pos = self.pos;
        let mut r#type = None;
        while let Some((header, next)) = chop_header(self.buf, &mut self.pos) {
            r#type = Some(header.r#type);
            let res = match header.r#type {
                1u16 => IftypeDataAttrs::Iftypes({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                2u16 => IftypeDataAttrs::HeCapMac({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                3u16 => IftypeDataAttrs::HeCapPhy({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                4u16 => IftypeDataAttrs::HeCapMcsSet({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                5u16 => IftypeDataAttrs::HeCapPpe({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                6u16 => IftypeDataAttrs::He6ghzCapa({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                7u16 => IftypeDataAttrs::VendorElems({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                8u16 => IftypeDataAttrs::EhtCapMac({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                9u16 => IftypeDataAttrs::EhtCapPhy({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                10u16 => IftypeDataAttrs::EhtCapMcsSet({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                11u16 => IftypeDataAttrs::EhtCapPpe({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                n => {
                    if cfg!(any(test, feature = "deny-unknown-attrs")) {
                        break;
                    } else {
                        continue;
                    }
                }
            };
            return Some(Ok(res));
        }
        Some(Err(ErrorContext::new(
            "IftypeDataAttrs",
            r#type.and_then(|t| IftypeDataAttrs::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl<'a> std::fmt::Debug for IterableIftypeDataAttrs<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("IftypeDataAttrs");
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
                IftypeDataAttrs::Iftypes(val) => fmt.field("Iftypes", &val),
                IftypeDataAttrs::HeCapMac(val) => fmt.field("HeCapMac", &val),
                IftypeDataAttrs::HeCapPhy(val) => fmt.field("HeCapPhy", &val),
                IftypeDataAttrs::HeCapMcsSet(val) => fmt.field("HeCapMcsSet", &val),
                IftypeDataAttrs::HeCapPpe(val) => fmt.field("HeCapPpe", &val),
                IftypeDataAttrs::He6ghzCapa(val) => fmt.field("He6ghzCapa", &val),
                IftypeDataAttrs::VendorElems(val) => fmt.field("VendorElems", &val),
                IftypeDataAttrs::EhtCapMac(val) => fmt.field("EhtCapMac", &val),
                IftypeDataAttrs::EhtCapPhy(val) => fmt.field("EhtCapPhy", &val),
                IftypeDataAttrs::EhtCapMcsSet(val) => fmt.field("EhtCapMcsSet", &val),
                IftypeDataAttrs::EhtCapPpe(val) => fmt.field("EhtCapPpe", &val),
            };
        }
        fmt.finish()
    }
}
impl IterableIftypeDataAttrs<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if cur == offset {
            stack.push(("IftypeDataAttrs", offset));
            return (
                stack,
                missing_type.and_then(|t| IftypeDataAttrs::attr_from_type(t)),
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
                IftypeDataAttrs::Iftypes(val) => {
                    if last_off == offset {
                        stack.push(("Iftypes", last_off));
                        break;
                    }
                }
                IftypeDataAttrs::HeCapMac(val) => {
                    if last_off == offset {
                        stack.push(("HeCapMac", last_off));
                        break;
                    }
                }
                IftypeDataAttrs::HeCapPhy(val) => {
                    if last_off == offset {
                        stack.push(("HeCapPhy", last_off));
                        break;
                    }
                }
                IftypeDataAttrs::HeCapMcsSet(val) => {
                    if last_off == offset {
                        stack.push(("HeCapMcsSet", last_off));
                        break;
                    }
                }
                IftypeDataAttrs::HeCapPpe(val) => {
                    if last_off == offset {
                        stack.push(("HeCapPpe", last_off));
                        break;
                    }
                }
                IftypeDataAttrs::He6ghzCapa(val) => {
                    if last_off == offset {
                        stack.push(("He6ghzCapa", last_off));
                        break;
                    }
                }
                IftypeDataAttrs::VendorElems(val) => {
                    if last_off == offset {
                        stack.push(("VendorElems", last_off));
                        break;
                    }
                }
                IftypeDataAttrs::EhtCapMac(val) => {
                    if last_off == offset {
                        stack.push(("EhtCapMac", last_off));
                        break;
                    }
                }
                IftypeDataAttrs::EhtCapPhy(val) => {
                    if last_off == offset {
                        stack.push(("EhtCapPhy", last_off));
                        break;
                    }
                }
                IftypeDataAttrs::EhtCapMcsSet(val) => {
                    if last_off == offset {
                        stack.push(("EhtCapMcsSet", last_off));
                        break;
                    }
                }
                IftypeDataAttrs::EhtCapPpe(val) => {
                    if last_off == offset {
                        stack.push(("EhtCapPpe", last_off));
                        break;
                    }
                }
                _ => {}
            };
            last_off = cur + attrs.pos;
        }
        if !stack.is_empty() {
            stack.push(("IftypeDataAttrs", cur));
        }
        (stack, None)
    }
}
#[derive(Clone)]
pub enum IftypeAttrs<'a> {
    Unspecified(IterableFrameTypeAttrs<'a>),
    Adhoc(IterableFrameTypeAttrs<'a>),
    Station(IterableFrameTypeAttrs<'a>),
    Ap(IterableFrameTypeAttrs<'a>),
    ApVlan(IterableFrameTypeAttrs<'a>),
    Wds(IterableFrameTypeAttrs<'a>),
    Monitor(IterableFrameTypeAttrs<'a>),
    MeshPoint(IterableFrameTypeAttrs<'a>),
    P2pClient(IterableFrameTypeAttrs<'a>),
    P2pGo(IterableFrameTypeAttrs<'a>),
    P2pDevice(IterableFrameTypeAttrs<'a>),
    Ocb(IterableFrameTypeAttrs<'a>),
    Nan(IterableFrameTypeAttrs<'a>),
}
impl<'a> IterableIftypeAttrs<'a> {
    pub fn get_unspecified(&self) -> Result<IterableFrameTypeAttrs<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let IftypeAttrs::Unspecified(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "IftypeAttrs",
            "Unspecified",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_adhoc(&self) -> Result<IterableFrameTypeAttrs<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let IftypeAttrs::Adhoc(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "IftypeAttrs",
            "Adhoc",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_station(&self) -> Result<IterableFrameTypeAttrs<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let IftypeAttrs::Station(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "IftypeAttrs",
            "Station",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_ap(&self) -> Result<IterableFrameTypeAttrs<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let IftypeAttrs::Ap(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "IftypeAttrs",
            "Ap",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_ap_vlan(&self) -> Result<IterableFrameTypeAttrs<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let IftypeAttrs::ApVlan(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "IftypeAttrs",
            "ApVlan",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_wds(&self) -> Result<IterableFrameTypeAttrs<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let IftypeAttrs::Wds(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "IftypeAttrs",
            "Wds",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_monitor(&self) -> Result<IterableFrameTypeAttrs<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let IftypeAttrs::Monitor(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "IftypeAttrs",
            "Monitor",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_mesh_point(&self) -> Result<IterableFrameTypeAttrs<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let IftypeAttrs::MeshPoint(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "IftypeAttrs",
            "MeshPoint",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_p2p_client(&self) -> Result<IterableFrameTypeAttrs<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let IftypeAttrs::P2pClient(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "IftypeAttrs",
            "P2pClient",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_p2p_go(&self) -> Result<IterableFrameTypeAttrs<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let IftypeAttrs::P2pGo(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "IftypeAttrs",
            "P2pGo",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_p2p_device(&self) -> Result<IterableFrameTypeAttrs<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let IftypeAttrs::P2pDevice(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "IftypeAttrs",
            "P2pDevice",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_ocb(&self) -> Result<IterableFrameTypeAttrs<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let IftypeAttrs::Ocb(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "IftypeAttrs",
            "Ocb",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_nan(&self) -> Result<IterableFrameTypeAttrs<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let IftypeAttrs::Nan(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "IftypeAttrs",
            "Nan",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl<'a> IftypeAttrs<'a> {
    pub fn new(buf: &'a [u8]) -> IterableIftypeAttrs<'a> {
        IterableIftypeAttrs::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        let res = match r#type {
            0u16 => "Unspecified",
            1u16 => "Adhoc",
            2u16 => "Station",
            3u16 => "Ap",
            4u16 => "ApVlan",
            5u16 => "Wds",
            6u16 => "Monitor",
            7u16 => "MeshPoint",
            8u16 => "P2pClient",
            9u16 => "P2pGo",
            10u16 => "P2pDevice",
            11u16 => "Ocb",
            12u16 => "Nan",
            _ => return None,
        };
        Some(res)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableIftypeAttrs<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableIftypeAttrs<'a> {
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
impl<'a> Iterator for IterableIftypeAttrs<'a> {
    type Item = Result<IftypeAttrs<'a>, ErrorContext>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.buf.len() == self.pos {
            return None;
        }
        let pos = self.pos;
        let mut r#type = None;
        while let Some((header, next)) = chop_header(self.buf, &mut self.pos) {
            r#type = Some(header.r#type);
            let res = match header.r#type {
                0u16 => IftypeAttrs::Unspecified({
                    let res = Some(IterableFrameTypeAttrs::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                1u16 => IftypeAttrs::Adhoc({
                    let res = Some(IterableFrameTypeAttrs::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                2u16 => IftypeAttrs::Station({
                    let res = Some(IterableFrameTypeAttrs::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                3u16 => IftypeAttrs::Ap({
                    let res = Some(IterableFrameTypeAttrs::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                4u16 => IftypeAttrs::ApVlan({
                    let res = Some(IterableFrameTypeAttrs::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                5u16 => IftypeAttrs::Wds({
                    let res = Some(IterableFrameTypeAttrs::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                6u16 => IftypeAttrs::Monitor({
                    let res = Some(IterableFrameTypeAttrs::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                7u16 => IftypeAttrs::MeshPoint({
                    let res = Some(IterableFrameTypeAttrs::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                8u16 => IftypeAttrs::P2pClient({
                    let res = Some(IterableFrameTypeAttrs::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                9u16 => IftypeAttrs::P2pGo({
                    let res = Some(IterableFrameTypeAttrs::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                10u16 => IftypeAttrs::P2pDevice({
                    let res = Some(IterableFrameTypeAttrs::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                11u16 => IftypeAttrs::Ocb({
                    let res = Some(IterableFrameTypeAttrs::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                12u16 => IftypeAttrs::Nan({
                    let res = Some(IterableFrameTypeAttrs::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                n => {
                    if cfg!(any(test, feature = "deny-unknown-attrs")) {
                        break;
                    } else {
                        continue;
                    }
                }
            };
            return Some(Ok(res));
        }
        Some(Err(ErrorContext::new(
            "IftypeAttrs",
            r#type.and_then(|t| IftypeAttrs::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl<'a> std::fmt::Debug for IterableIftypeAttrs<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("IftypeAttrs");
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
                IftypeAttrs::Unspecified(val) => fmt.field("Unspecified", &val),
                IftypeAttrs::Adhoc(val) => fmt.field("Adhoc", &val),
                IftypeAttrs::Station(val) => fmt.field("Station", &val),
                IftypeAttrs::Ap(val) => fmt.field("Ap", &val),
                IftypeAttrs::ApVlan(val) => fmt.field("ApVlan", &val),
                IftypeAttrs::Wds(val) => fmt.field("Wds", &val),
                IftypeAttrs::Monitor(val) => fmt.field("Monitor", &val),
                IftypeAttrs::MeshPoint(val) => fmt.field("MeshPoint", &val),
                IftypeAttrs::P2pClient(val) => fmt.field("P2pClient", &val),
                IftypeAttrs::P2pGo(val) => fmt.field("P2pGo", &val),
                IftypeAttrs::P2pDevice(val) => fmt.field("P2pDevice", &val),
                IftypeAttrs::Ocb(val) => fmt.field("Ocb", &val),
                IftypeAttrs::Nan(val) => fmt.field("Nan", &val),
            };
        }
        fmt.finish()
    }
}
impl IterableIftypeAttrs<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if cur == offset {
            stack.push(("IftypeAttrs", offset));
            return (
                stack,
                missing_type.and_then(|t| IftypeAttrs::attr_from_type(t)),
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
                IftypeAttrs::Unspecified(val) => {
                    (stack, missing) = val.lookup_attr(offset, missing_type);
                    if !stack.is_empty() {
                        break;
                    }
                }
                IftypeAttrs::Adhoc(val) => {
                    (stack, missing) = val.lookup_attr(offset, missing_type);
                    if !stack.is_empty() {
                        break;
                    }
                }
                IftypeAttrs::Station(val) => {
                    (stack, missing) = val.lookup_attr(offset, missing_type);
                    if !stack.is_empty() {
                        break;
                    }
                }
                IftypeAttrs::Ap(val) => {
                    (stack, missing) = val.lookup_attr(offset, missing_type);
                    if !stack.is_empty() {
                        break;
                    }
                }
                IftypeAttrs::ApVlan(val) => {
                    (stack, missing) = val.lookup_attr(offset, missing_type);
                    if !stack.is_empty() {
                        break;
                    }
                }
                IftypeAttrs::Wds(val) => {
                    (stack, missing) = val.lookup_attr(offset, missing_type);
                    if !stack.is_empty() {
                        break;
                    }
                }
                IftypeAttrs::Monitor(val) => {
                    (stack, missing) = val.lookup_attr(offset, missing_type);
                    if !stack.is_empty() {
                        break;
                    }
                }
                IftypeAttrs::MeshPoint(val) => {
                    (stack, missing) = val.lookup_attr(offset, missing_type);
                    if !stack.is_empty() {
                        break;
                    }
                }
                IftypeAttrs::P2pClient(val) => {
                    (stack, missing) = val.lookup_attr(offset, missing_type);
                    if !stack.is_empty() {
                        break;
                    }
                }
                IftypeAttrs::P2pGo(val) => {
                    (stack, missing) = val.lookup_attr(offset, missing_type);
                    if !stack.is_empty() {
                        break;
                    }
                }
                IftypeAttrs::P2pDevice(val) => {
                    (stack, missing) = val.lookup_attr(offset, missing_type);
                    if !stack.is_empty() {
                        break;
                    }
                }
                IftypeAttrs::Ocb(val) => {
                    (stack, missing) = val.lookup_attr(offset, missing_type);
                    if !stack.is_empty() {
                        break;
                    }
                }
                IftypeAttrs::Nan(val) => {
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
            stack.push(("IftypeAttrs", cur));
        }
        (stack, missing)
    }
}
#[derive(Clone)]
pub enum SarSpecs {
    Power(i32),
    RangeIndex(u32),
    StartFreq(u32),
    EndFreq(u32),
}
impl<'a> IterableSarSpecs<'a> {
    pub fn get_power(&self) -> Result<i32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let SarSpecs::Power(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "SarSpecs",
            "Power",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_range_index(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let SarSpecs::RangeIndex(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "SarSpecs",
            "RangeIndex",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_start_freq(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let SarSpecs::StartFreq(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "SarSpecs",
            "StartFreq",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_end_freq(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let SarSpecs::EndFreq(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "SarSpecs",
            "EndFreq",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl SarSpecs {
    pub fn new(buf: &'_ [u8]) -> IterableSarSpecs<'_> {
        IterableSarSpecs::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        let res = match r#type {
            1u16 => "Power",
            2u16 => "RangeIndex",
            3u16 => "StartFreq",
            4u16 => "EndFreq",
            _ => return None,
        };
        Some(res)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableSarSpecs<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableSarSpecs<'a> {
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
impl<'a> Iterator for IterableSarSpecs<'a> {
    type Item = Result<SarSpecs, ErrorContext>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.buf.len() == self.pos {
            return None;
        }
        let pos = self.pos;
        let mut r#type = None;
        while let Some((header, next)) = chop_header(self.buf, &mut self.pos) {
            r#type = Some(header.r#type);
            let res = match header.r#type {
                1u16 => SarSpecs::Power({
                    let res = parse_i32(next);
                    let Some(val) = res else { break };
                    val
                }),
                2u16 => SarSpecs::RangeIndex({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                3u16 => SarSpecs::StartFreq({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                4u16 => SarSpecs::EndFreq({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                n => {
                    if cfg!(any(test, feature = "deny-unknown-attrs")) {
                        break;
                    } else {
                        continue;
                    }
                }
            };
            return Some(Ok(res));
        }
        Some(Err(ErrorContext::new(
            "SarSpecs",
            r#type.and_then(|t| SarSpecs::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl std::fmt::Debug for IterableSarSpecs<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("SarSpecs");
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
                SarSpecs::Power(val) => fmt.field("Power", &val),
                SarSpecs::RangeIndex(val) => fmt.field("RangeIndex", &val),
                SarSpecs::StartFreq(val) => fmt.field("StartFreq", &val),
                SarSpecs::EndFreq(val) => fmt.field("EndFreq", &val),
            };
        }
        fmt.finish()
    }
}
impl IterableSarSpecs<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if cur == offset {
            stack.push(("SarSpecs", offset));
            return (
                stack,
                missing_type.and_then(|t| SarSpecs::attr_from_type(t)),
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
                SarSpecs::Power(val) => {
                    if last_off == offset {
                        stack.push(("Power", last_off));
                        break;
                    }
                }
                SarSpecs::RangeIndex(val) => {
                    if last_off == offset {
                        stack.push(("RangeIndex", last_off));
                        break;
                    }
                }
                SarSpecs::StartFreq(val) => {
                    if last_off == offset {
                        stack.push(("StartFreq", last_off));
                        break;
                    }
                }
                SarSpecs::EndFreq(val) => {
                    if last_off == offset {
                        stack.push(("EndFreq", last_off));
                        break;
                    }
                }
                _ => {}
            };
            last_off = cur + attrs.pos;
        }
        if !stack.is_empty() {
            stack.push(("SarSpecs", cur));
        }
        (stack, None)
    }
}
#[derive(Clone)]
pub enum SupportedIftypes {
    Adhoc(()),
    Station(()),
    Ap(()),
    ApVlan(()),
    Wds(()),
    Monitor(()),
    MeshPoint(()),
    P2pClient(()),
    P2pGo(()),
    P2pDevice(()),
    Ocb(()),
    Nan(()),
}
impl<'a> IterableSupportedIftypes<'a> {
    pub fn get_adhoc(&self) -> Result<(), ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let SupportedIftypes::Adhoc(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "SupportedIftypes",
            "Adhoc",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_station(&self) -> Result<(), ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let SupportedIftypes::Station(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "SupportedIftypes",
            "Station",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_ap(&self) -> Result<(), ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let SupportedIftypes::Ap(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "SupportedIftypes",
            "Ap",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_ap_vlan(&self) -> Result<(), ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let SupportedIftypes::ApVlan(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "SupportedIftypes",
            "ApVlan",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_wds(&self) -> Result<(), ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let SupportedIftypes::Wds(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "SupportedIftypes",
            "Wds",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_monitor(&self) -> Result<(), ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let SupportedIftypes::Monitor(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "SupportedIftypes",
            "Monitor",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_mesh_point(&self) -> Result<(), ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let SupportedIftypes::MeshPoint(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "SupportedIftypes",
            "MeshPoint",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_p2p_client(&self) -> Result<(), ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let SupportedIftypes::P2pClient(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "SupportedIftypes",
            "P2pClient",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_p2p_go(&self) -> Result<(), ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let SupportedIftypes::P2pGo(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "SupportedIftypes",
            "P2pGo",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_p2p_device(&self) -> Result<(), ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let SupportedIftypes::P2pDevice(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "SupportedIftypes",
            "P2pDevice",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_ocb(&self) -> Result<(), ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let SupportedIftypes::Ocb(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "SupportedIftypes",
            "Ocb",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_nan(&self) -> Result<(), ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let SupportedIftypes::Nan(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "SupportedIftypes",
            "Nan",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl SupportedIftypes {
    pub fn new(buf: &'_ [u8]) -> IterableSupportedIftypes<'_> {
        IterableSupportedIftypes::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        let res = match r#type {
            1u16 => "Adhoc",
            2u16 => "Station",
            3u16 => "Ap",
            4u16 => "ApVlan",
            5u16 => "Wds",
            6u16 => "Monitor",
            7u16 => "MeshPoint",
            8u16 => "P2pClient",
            9u16 => "P2pGo",
            10u16 => "P2pDevice",
            11u16 => "Ocb",
            12u16 => "Nan",
            _ => return None,
        };
        Some(res)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableSupportedIftypes<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableSupportedIftypes<'a> {
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
impl<'a> Iterator for IterableSupportedIftypes<'a> {
    type Item = Result<SupportedIftypes, ErrorContext>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.buf.len() == self.pos {
            return None;
        }
        let pos = self.pos;
        let mut r#type = None;
        while let Some((header, next)) = chop_header(self.buf, &mut self.pos) {
            r#type = Some(header.r#type);
            let res = match header.r#type {
                1u16 => SupportedIftypes::Adhoc(()),
                2u16 => SupportedIftypes::Station(()),
                3u16 => SupportedIftypes::Ap(()),
                4u16 => SupportedIftypes::ApVlan(()),
                5u16 => SupportedIftypes::Wds(()),
                6u16 => SupportedIftypes::Monitor(()),
                7u16 => SupportedIftypes::MeshPoint(()),
                8u16 => SupportedIftypes::P2pClient(()),
                9u16 => SupportedIftypes::P2pGo(()),
                10u16 => SupportedIftypes::P2pDevice(()),
                11u16 => SupportedIftypes::Ocb(()),
                12u16 => SupportedIftypes::Nan(()),
                n => {
                    if cfg!(any(test, feature = "deny-unknown-attrs")) {
                        break;
                    } else {
                        continue;
                    }
                }
            };
            return Some(Ok(res));
        }
        Some(Err(ErrorContext::new(
            "SupportedIftypes",
            r#type.and_then(|t| SupportedIftypes::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl std::fmt::Debug for IterableSupportedIftypes<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("SupportedIftypes");
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
                SupportedIftypes::Adhoc(val) => fmt.field("Adhoc", &val),
                SupportedIftypes::Station(val) => fmt.field("Station", &val),
                SupportedIftypes::Ap(val) => fmt.field("Ap", &val),
                SupportedIftypes::ApVlan(val) => fmt.field("ApVlan", &val),
                SupportedIftypes::Wds(val) => fmt.field("Wds", &val),
                SupportedIftypes::Monitor(val) => fmt.field("Monitor", &val),
                SupportedIftypes::MeshPoint(val) => fmt.field("MeshPoint", &val),
                SupportedIftypes::P2pClient(val) => fmt.field("P2pClient", &val),
                SupportedIftypes::P2pGo(val) => fmt.field("P2pGo", &val),
                SupportedIftypes::P2pDevice(val) => fmt.field("P2pDevice", &val),
                SupportedIftypes::Ocb(val) => fmt.field("Ocb", &val),
                SupportedIftypes::Nan(val) => fmt.field("Nan", &val),
            };
        }
        fmt.finish()
    }
}
impl IterableSupportedIftypes<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if cur == offset {
            stack.push(("SupportedIftypes", offset));
            return (
                stack,
                missing_type.and_then(|t| SupportedIftypes::attr_from_type(t)),
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
                SupportedIftypes::Adhoc(val) => {
                    if last_off == offset {
                        stack.push(("Adhoc", last_off));
                        break;
                    }
                }
                SupportedIftypes::Station(val) => {
                    if last_off == offset {
                        stack.push(("Station", last_off));
                        break;
                    }
                }
                SupportedIftypes::Ap(val) => {
                    if last_off == offset {
                        stack.push(("Ap", last_off));
                        break;
                    }
                }
                SupportedIftypes::ApVlan(val) => {
                    if last_off == offset {
                        stack.push(("ApVlan", last_off));
                        break;
                    }
                }
                SupportedIftypes::Wds(val) => {
                    if last_off == offset {
                        stack.push(("Wds", last_off));
                        break;
                    }
                }
                SupportedIftypes::Monitor(val) => {
                    if last_off == offset {
                        stack.push(("Monitor", last_off));
                        break;
                    }
                }
                SupportedIftypes::MeshPoint(val) => {
                    if last_off == offset {
                        stack.push(("MeshPoint", last_off));
                        break;
                    }
                }
                SupportedIftypes::P2pClient(val) => {
                    if last_off == offset {
                        stack.push(("P2pClient", last_off));
                        break;
                    }
                }
                SupportedIftypes::P2pGo(val) => {
                    if last_off == offset {
                        stack.push(("P2pGo", last_off));
                        break;
                    }
                }
                SupportedIftypes::P2pDevice(val) => {
                    if last_off == offset {
                        stack.push(("P2pDevice", last_off));
                        break;
                    }
                }
                SupportedIftypes::Ocb(val) => {
                    if last_off == offset {
                        stack.push(("Ocb", last_off));
                        break;
                    }
                }
                SupportedIftypes::Nan(val) => {
                    if last_off == offset {
                        stack.push(("Nan", last_off));
                        break;
                    }
                }
                _ => {}
            };
            last_off = cur + attrs.pos;
        }
        if !stack.is_empty() {
            stack.push(("SupportedIftypes", cur));
        }
        (stack, None)
    }
}
#[derive(Clone)]
pub enum TxqStatsAttrs {
    BacklogBytes(u32),
    BacklogPackets(u32),
    Flows(u32),
    Drops(u32),
    EcnMarks(u32),
    Overlimit(u32),
    Overmemory(u32),
    Collisions(u32),
    TxBytes(u32),
    TxPackets(u32),
    MaxFlows(u32),
}
impl<'a> IterableTxqStatsAttrs<'a> {
    pub fn get_backlog_bytes(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let TxqStatsAttrs::BacklogBytes(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "TxqStatsAttrs",
            "BacklogBytes",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_backlog_packets(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let TxqStatsAttrs::BacklogPackets(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "TxqStatsAttrs",
            "BacklogPackets",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_flows(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let TxqStatsAttrs::Flows(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "TxqStatsAttrs",
            "Flows",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_drops(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let TxqStatsAttrs::Drops(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "TxqStatsAttrs",
            "Drops",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_ecn_marks(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let TxqStatsAttrs::EcnMarks(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "TxqStatsAttrs",
            "EcnMarks",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_overlimit(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let TxqStatsAttrs::Overlimit(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "TxqStatsAttrs",
            "Overlimit",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_overmemory(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let TxqStatsAttrs::Overmemory(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "TxqStatsAttrs",
            "Overmemory",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_collisions(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let TxqStatsAttrs::Collisions(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "TxqStatsAttrs",
            "Collisions",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_tx_bytes(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let TxqStatsAttrs::TxBytes(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "TxqStatsAttrs",
            "TxBytes",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_tx_packets(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let TxqStatsAttrs::TxPackets(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "TxqStatsAttrs",
            "TxPackets",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_max_flows(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let TxqStatsAttrs::MaxFlows(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "TxqStatsAttrs",
            "MaxFlows",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl TxqStatsAttrs {
    pub fn new(buf: &'_ [u8]) -> IterableTxqStatsAttrs<'_> {
        IterableTxqStatsAttrs::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        let res = match r#type {
            1u16 => "BacklogBytes",
            2u16 => "BacklogPackets",
            3u16 => "Flows",
            4u16 => "Drops",
            5u16 => "EcnMarks",
            6u16 => "Overlimit",
            7u16 => "Overmemory",
            8u16 => "Collisions",
            9u16 => "TxBytes",
            10u16 => "TxPackets",
            11u16 => "MaxFlows",
            _ => return None,
        };
        Some(res)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableTxqStatsAttrs<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableTxqStatsAttrs<'a> {
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
impl<'a> Iterator for IterableTxqStatsAttrs<'a> {
    type Item = Result<TxqStatsAttrs, ErrorContext>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.buf.len() == self.pos {
            return None;
        }
        let pos = self.pos;
        let mut r#type = None;
        while let Some((header, next)) = chop_header(self.buf, &mut self.pos) {
            r#type = Some(header.r#type);
            let res = match header.r#type {
                1u16 => TxqStatsAttrs::BacklogBytes({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                2u16 => TxqStatsAttrs::BacklogPackets({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                3u16 => TxqStatsAttrs::Flows({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                4u16 => TxqStatsAttrs::Drops({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                5u16 => TxqStatsAttrs::EcnMarks({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                6u16 => TxqStatsAttrs::Overlimit({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                7u16 => TxqStatsAttrs::Overmemory({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                8u16 => TxqStatsAttrs::Collisions({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                9u16 => TxqStatsAttrs::TxBytes({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                10u16 => TxqStatsAttrs::TxPackets({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                11u16 => TxqStatsAttrs::MaxFlows({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                n => {
                    if cfg!(any(test, feature = "deny-unknown-attrs")) {
                        break;
                    } else {
                        continue;
                    }
                }
            };
            return Some(Ok(res));
        }
        Some(Err(ErrorContext::new(
            "TxqStatsAttrs",
            r#type.and_then(|t| TxqStatsAttrs::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl std::fmt::Debug for IterableTxqStatsAttrs<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("TxqStatsAttrs");
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
                TxqStatsAttrs::BacklogBytes(val) => fmt.field("BacklogBytes", &val),
                TxqStatsAttrs::BacklogPackets(val) => fmt.field("BacklogPackets", &val),
                TxqStatsAttrs::Flows(val) => fmt.field("Flows", &val),
                TxqStatsAttrs::Drops(val) => fmt.field("Drops", &val),
                TxqStatsAttrs::EcnMarks(val) => fmt.field("EcnMarks", &val),
                TxqStatsAttrs::Overlimit(val) => fmt.field("Overlimit", &val),
                TxqStatsAttrs::Overmemory(val) => fmt.field("Overmemory", &val),
                TxqStatsAttrs::Collisions(val) => fmt.field("Collisions", &val),
                TxqStatsAttrs::TxBytes(val) => fmt.field("TxBytes", &val),
                TxqStatsAttrs::TxPackets(val) => fmt.field("TxPackets", &val),
                TxqStatsAttrs::MaxFlows(val) => fmt.field("MaxFlows", &val),
            };
        }
        fmt.finish()
    }
}
impl IterableTxqStatsAttrs<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if cur == offset {
            stack.push(("TxqStatsAttrs", offset));
            return (
                stack,
                missing_type.and_then(|t| TxqStatsAttrs::attr_from_type(t)),
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
                TxqStatsAttrs::BacklogBytes(val) => {
                    if last_off == offset {
                        stack.push(("BacklogBytes", last_off));
                        break;
                    }
                }
                TxqStatsAttrs::BacklogPackets(val) => {
                    if last_off == offset {
                        stack.push(("BacklogPackets", last_off));
                        break;
                    }
                }
                TxqStatsAttrs::Flows(val) => {
                    if last_off == offset {
                        stack.push(("Flows", last_off));
                        break;
                    }
                }
                TxqStatsAttrs::Drops(val) => {
                    if last_off == offset {
                        stack.push(("Drops", last_off));
                        break;
                    }
                }
                TxqStatsAttrs::EcnMarks(val) => {
                    if last_off == offset {
                        stack.push(("EcnMarks", last_off));
                        break;
                    }
                }
                TxqStatsAttrs::Overlimit(val) => {
                    if last_off == offset {
                        stack.push(("Overlimit", last_off));
                        break;
                    }
                }
                TxqStatsAttrs::Overmemory(val) => {
                    if last_off == offset {
                        stack.push(("Overmemory", last_off));
                        break;
                    }
                }
                TxqStatsAttrs::Collisions(val) => {
                    if last_off == offset {
                        stack.push(("Collisions", last_off));
                        break;
                    }
                }
                TxqStatsAttrs::TxBytes(val) => {
                    if last_off == offset {
                        stack.push(("TxBytes", last_off));
                        break;
                    }
                }
                TxqStatsAttrs::TxPackets(val) => {
                    if last_off == offset {
                        stack.push(("TxPackets", last_off));
                        break;
                    }
                }
                TxqStatsAttrs::MaxFlows(val) => {
                    if last_off == offset {
                        stack.push(("MaxFlows", last_off));
                        break;
                    }
                }
                _ => {}
            };
            last_off = cur + attrs.pos;
        }
        if !stack.is_empty() {
            stack.push(("TxqStatsAttrs", cur));
        }
        (stack, None)
    }
}
#[derive(Clone)]
pub enum WmmAttrs {
    CwMin(u16),
    CwMax(u16),
    Aifsn(u8),
    Txop(u16),
}
impl<'a> IterableWmmAttrs<'a> {
    pub fn get_cw_min(&self) -> Result<u16, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let WmmAttrs::CwMin(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "WmmAttrs",
            "CwMin",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_cw_max(&self) -> Result<u16, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let WmmAttrs::CwMax(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "WmmAttrs",
            "CwMax",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_aifsn(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let WmmAttrs::Aifsn(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "WmmAttrs",
            "Aifsn",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_txop(&self) -> Result<u16, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let WmmAttrs::Txop(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "WmmAttrs",
            "Txop",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl WmmAttrs {
    pub fn new(buf: &'_ [u8]) -> IterableWmmAttrs<'_> {
        IterableWmmAttrs::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        let res = match r#type {
            1u16 => "CwMin",
            2u16 => "CwMax",
            3u16 => "Aifsn",
            4u16 => "Txop",
            _ => return None,
        };
        Some(res)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableWmmAttrs<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableWmmAttrs<'a> {
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
impl<'a> Iterator for IterableWmmAttrs<'a> {
    type Item = Result<WmmAttrs, ErrorContext>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.buf.len() == self.pos {
            return None;
        }
        let pos = self.pos;
        let mut r#type = None;
        while let Some((header, next)) = chop_header(self.buf, &mut self.pos) {
            r#type = Some(header.r#type);
            let res = match header.r#type {
                1u16 => WmmAttrs::CwMin({
                    let res = parse_u16(next);
                    let Some(val) = res else { break };
                    val
                }),
                2u16 => WmmAttrs::CwMax({
                    let res = parse_u16(next);
                    let Some(val) = res else { break };
                    val
                }),
                3u16 => WmmAttrs::Aifsn({
                    let res = parse_u8(next);
                    let Some(val) = res else { break };
                    val
                }),
                4u16 => WmmAttrs::Txop({
                    let res = parse_u16(next);
                    let Some(val) = res else { break };
                    val
                }),
                n => {
                    if cfg!(any(test, feature = "deny-unknown-attrs")) {
                        break;
                    } else {
                        continue;
                    }
                }
            };
            return Some(Ok(res));
        }
        Some(Err(ErrorContext::new(
            "WmmAttrs",
            r#type.and_then(|t| WmmAttrs::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl std::fmt::Debug for IterableWmmAttrs<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("WmmAttrs");
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
                WmmAttrs::CwMin(val) => fmt.field("CwMin", &val),
                WmmAttrs::CwMax(val) => fmt.field("CwMax", &val),
                WmmAttrs::Aifsn(val) => fmt.field("Aifsn", &val),
                WmmAttrs::Txop(val) => fmt.field("Txop", &val),
            };
        }
        fmt.finish()
    }
}
impl IterableWmmAttrs<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if cur == offset {
            stack.push(("WmmAttrs", offset));
            return (
                stack,
                missing_type.and_then(|t| WmmAttrs::attr_from_type(t)),
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
                WmmAttrs::CwMin(val) => {
                    if last_off == offset {
                        stack.push(("CwMin", last_off));
                        break;
                    }
                }
                WmmAttrs::CwMax(val) => {
                    if last_off == offset {
                        stack.push(("CwMax", last_off));
                        break;
                    }
                }
                WmmAttrs::Aifsn(val) => {
                    if last_off == offset {
                        stack.push(("Aifsn", last_off));
                        break;
                    }
                }
                WmmAttrs::Txop(val) => {
                    if last_off == offset {
                        stack.push(("Txop", last_off));
                        break;
                    }
                }
                _ => {}
            };
            last_off = cur + attrs.pos;
        }
        if !stack.is_empty() {
            stack.push(("WmmAttrs", cur));
        }
        (stack, None)
    }
}
#[derive(Clone)]
pub enum WowlanTriggersAttrs {
    Any(()),
    Disconnect(()),
    MagicPkt(()),
    PktPattern(()),
    GtkRekeySupported(()),
    GtkRekeyFailure(()),
    EapIdentRequest(()),
    _4wayHandshake(()),
    RfkillRelease(()),
    WakeupPkt80211(()),
    WakeupPkt80211Len(()),
    WakeupPkt8023(()),
    WakeupPkt8023Len(()),
    TcpConnection(()),
    WakeupTcpMatch(()),
    WakeupTcpConnlost(()),
    WakeupTcpNomoretokens(()),
    NetDetect(()),
    NetDetectResults(()),
    UnprotectedDeauthDisassoc(()),
}
impl<'a> IterableWowlanTriggersAttrs<'a> {
    pub fn get_any(&self) -> Result<(), ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let WowlanTriggersAttrs::Any(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "WowlanTriggersAttrs",
            "Any",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_disconnect(&self) -> Result<(), ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let WowlanTriggersAttrs::Disconnect(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "WowlanTriggersAttrs",
            "Disconnect",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_magic_pkt(&self) -> Result<(), ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let WowlanTriggersAttrs::MagicPkt(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "WowlanTriggersAttrs",
            "MagicPkt",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_pkt_pattern(&self) -> Result<(), ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let WowlanTriggersAttrs::PktPattern(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "WowlanTriggersAttrs",
            "PktPattern",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_gtk_rekey_supported(&self) -> Result<(), ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let WowlanTriggersAttrs::GtkRekeySupported(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "WowlanTriggersAttrs",
            "GtkRekeySupported",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_gtk_rekey_failure(&self) -> Result<(), ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let WowlanTriggersAttrs::GtkRekeyFailure(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "WowlanTriggersAttrs",
            "GtkRekeyFailure",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_eap_ident_request(&self) -> Result<(), ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let WowlanTriggersAttrs::EapIdentRequest(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "WowlanTriggersAttrs",
            "EapIdentRequest",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_4way_handshake(&self) -> Result<(), ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let WowlanTriggersAttrs::_4wayHandshake(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "WowlanTriggersAttrs",
            "4wayHandshake",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_rfkill_release(&self) -> Result<(), ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let WowlanTriggersAttrs::RfkillRelease(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "WowlanTriggersAttrs",
            "RfkillRelease",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_wakeup_pkt_80211(&self) -> Result<(), ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let WowlanTriggersAttrs::WakeupPkt80211(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "WowlanTriggersAttrs",
            "WakeupPkt80211",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_wakeup_pkt_80211_len(&self) -> Result<(), ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let WowlanTriggersAttrs::WakeupPkt80211Len(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "WowlanTriggersAttrs",
            "WakeupPkt80211Len",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_wakeup_pkt_8023(&self) -> Result<(), ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let WowlanTriggersAttrs::WakeupPkt8023(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "WowlanTriggersAttrs",
            "WakeupPkt8023",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_wakeup_pkt_8023_len(&self) -> Result<(), ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let WowlanTriggersAttrs::WakeupPkt8023Len(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "WowlanTriggersAttrs",
            "WakeupPkt8023Len",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_tcp_connection(&self) -> Result<(), ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let WowlanTriggersAttrs::TcpConnection(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "WowlanTriggersAttrs",
            "TcpConnection",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_wakeup_tcp_match(&self) -> Result<(), ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let WowlanTriggersAttrs::WakeupTcpMatch(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "WowlanTriggersAttrs",
            "WakeupTcpMatch",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_wakeup_tcp_connlost(&self) -> Result<(), ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let WowlanTriggersAttrs::WakeupTcpConnlost(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "WowlanTriggersAttrs",
            "WakeupTcpConnlost",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_wakeup_tcp_nomoretokens(&self) -> Result<(), ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let WowlanTriggersAttrs::WakeupTcpNomoretokens(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "WowlanTriggersAttrs",
            "WakeupTcpNomoretokens",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_net_detect(&self) -> Result<(), ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let WowlanTriggersAttrs::NetDetect(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "WowlanTriggersAttrs",
            "NetDetect",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_net_detect_results(&self) -> Result<(), ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let WowlanTriggersAttrs::NetDetectResults(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "WowlanTriggersAttrs",
            "NetDetectResults",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_unprotected_deauth_disassoc(&self) -> Result<(), ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let WowlanTriggersAttrs::UnprotectedDeauthDisassoc(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "WowlanTriggersAttrs",
            "UnprotectedDeauthDisassoc",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl WowlanTriggersAttrs {
    pub fn new(buf: &'_ [u8]) -> IterableWowlanTriggersAttrs<'_> {
        IterableWowlanTriggersAttrs::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        let res = match r#type {
            1u16 => "Any",
            2u16 => "Disconnect",
            3u16 => "MagicPkt",
            4u16 => "PktPattern",
            5u16 => "GtkRekeySupported",
            6u16 => "GtkRekeyFailure",
            7u16 => "EapIdentRequest",
            8u16 => "4wayHandshake",
            9u16 => "RfkillRelease",
            10u16 => "WakeupPkt80211",
            11u16 => "WakeupPkt80211Len",
            12u16 => "WakeupPkt8023",
            13u16 => "WakeupPkt8023Len",
            14u16 => "TcpConnection",
            15u16 => "WakeupTcpMatch",
            16u16 => "WakeupTcpConnlost",
            17u16 => "WakeupTcpNomoretokens",
            18u16 => "NetDetect",
            19u16 => "NetDetectResults",
            20u16 => "UnprotectedDeauthDisassoc",
            _ => return None,
        };
        Some(res)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableWowlanTriggersAttrs<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableWowlanTriggersAttrs<'a> {
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
impl<'a> Iterator for IterableWowlanTriggersAttrs<'a> {
    type Item = Result<WowlanTriggersAttrs, ErrorContext>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.buf.len() == self.pos {
            return None;
        }
        let pos = self.pos;
        let mut r#type = None;
        while let Some((header, next)) = chop_header(self.buf, &mut self.pos) {
            r#type = Some(header.r#type);
            let res = match header.r#type {
                1u16 => WowlanTriggersAttrs::Any(()),
                2u16 => WowlanTriggersAttrs::Disconnect(()),
                3u16 => WowlanTriggersAttrs::MagicPkt(()),
                4u16 => WowlanTriggersAttrs::PktPattern(()),
                5u16 => WowlanTriggersAttrs::GtkRekeySupported(()),
                6u16 => WowlanTriggersAttrs::GtkRekeyFailure(()),
                7u16 => WowlanTriggersAttrs::EapIdentRequest(()),
                8u16 => WowlanTriggersAttrs::_4wayHandshake(()),
                9u16 => WowlanTriggersAttrs::RfkillRelease(()),
                10u16 => WowlanTriggersAttrs::WakeupPkt80211(()),
                11u16 => WowlanTriggersAttrs::WakeupPkt80211Len(()),
                12u16 => WowlanTriggersAttrs::WakeupPkt8023(()),
                13u16 => WowlanTriggersAttrs::WakeupPkt8023Len(()),
                14u16 => WowlanTriggersAttrs::TcpConnection(()),
                15u16 => WowlanTriggersAttrs::WakeupTcpMatch(()),
                16u16 => WowlanTriggersAttrs::WakeupTcpConnlost(()),
                17u16 => WowlanTriggersAttrs::WakeupTcpNomoretokens(()),
                18u16 => WowlanTriggersAttrs::NetDetect(()),
                19u16 => WowlanTriggersAttrs::NetDetectResults(()),
                20u16 => WowlanTriggersAttrs::UnprotectedDeauthDisassoc(()),
                n => {
                    if cfg!(any(test, feature = "deny-unknown-attrs")) {
                        break;
                    } else {
                        continue;
                    }
                }
            };
            return Some(Ok(res));
        }
        Some(Err(ErrorContext::new(
            "WowlanTriggersAttrs",
            r#type.and_then(|t| WowlanTriggersAttrs::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl std::fmt::Debug for IterableWowlanTriggersAttrs<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("WowlanTriggersAttrs");
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
                WowlanTriggersAttrs::Any(val) => fmt.field("Any", &val),
                WowlanTriggersAttrs::Disconnect(val) => fmt.field("Disconnect", &val),
                WowlanTriggersAttrs::MagicPkt(val) => fmt.field("MagicPkt", &val),
                WowlanTriggersAttrs::PktPattern(val) => fmt.field("PktPattern", &val),
                WowlanTriggersAttrs::GtkRekeySupported(val) => fmt.field("GtkRekeySupported", &val),
                WowlanTriggersAttrs::GtkRekeyFailure(val) => fmt.field("GtkRekeyFailure", &val),
                WowlanTriggersAttrs::EapIdentRequest(val) => fmt.field("EapIdentRequest", &val),
                WowlanTriggersAttrs::_4wayHandshake(val) => fmt.field("_4wayHandshake", &val),
                WowlanTriggersAttrs::RfkillRelease(val) => fmt.field("RfkillRelease", &val),
                WowlanTriggersAttrs::WakeupPkt80211(val) => fmt.field("WakeupPkt80211", &val),
                WowlanTriggersAttrs::WakeupPkt80211Len(val) => fmt.field("WakeupPkt80211Len", &val),
                WowlanTriggersAttrs::WakeupPkt8023(val) => fmt.field("WakeupPkt8023", &val),
                WowlanTriggersAttrs::WakeupPkt8023Len(val) => fmt.field("WakeupPkt8023Len", &val),
                WowlanTriggersAttrs::TcpConnection(val) => fmt.field("TcpConnection", &val),
                WowlanTriggersAttrs::WakeupTcpMatch(val) => fmt.field("WakeupTcpMatch", &val),
                WowlanTriggersAttrs::WakeupTcpConnlost(val) => fmt.field("WakeupTcpConnlost", &val),
                WowlanTriggersAttrs::WakeupTcpNomoretokens(val) => {
                    fmt.field("WakeupTcpNomoretokens", &val)
                }
                WowlanTriggersAttrs::NetDetect(val) => fmt.field("NetDetect", &val),
                WowlanTriggersAttrs::NetDetectResults(val) => fmt.field("NetDetectResults", &val),
                WowlanTriggersAttrs::UnprotectedDeauthDisassoc(val) => {
                    fmt.field("UnprotectedDeauthDisassoc", &val)
                }
            };
        }
        fmt.finish()
    }
}
impl IterableWowlanTriggersAttrs<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if cur == offset {
            stack.push(("WowlanTriggersAttrs", offset));
            return (
                stack,
                missing_type.and_then(|t| WowlanTriggersAttrs::attr_from_type(t)),
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
                WowlanTriggersAttrs::Any(val) => {
                    if last_off == offset {
                        stack.push(("Any", last_off));
                        break;
                    }
                }
                WowlanTriggersAttrs::Disconnect(val) => {
                    if last_off == offset {
                        stack.push(("Disconnect", last_off));
                        break;
                    }
                }
                WowlanTriggersAttrs::MagicPkt(val) => {
                    if last_off == offset {
                        stack.push(("MagicPkt", last_off));
                        break;
                    }
                }
                WowlanTriggersAttrs::PktPattern(val) => {
                    if last_off == offset {
                        stack.push(("PktPattern", last_off));
                        break;
                    }
                }
                WowlanTriggersAttrs::GtkRekeySupported(val) => {
                    if last_off == offset {
                        stack.push(("GtkRekeySupported", last_off));
                        break;
                    }
                }
                WowlanTriggersAttrs::GtkRekeyFailure(val) => {
                    if last_off == offset {
                        stack.push(("GtkRekeyFailure", last_off));
                        break;
                    }
                }
                WowlanTriggersAttrs::EapIdentRequest(val) => {
                    if last_off == offset {
                        stack.push(("EapIdentRequest", last_off));
                        break;
                    }
                }
                WowlanTriggersAttrs::_4wayHandshake(val) => {
                    if last_off == offset {
                        stack.push(("4wayHandshake", last_off));
                        break;
                    }
                }
                WowlanTriggersAttrs::RfkillRelease(val) => {
                    if last_off == offset {
                        stack.push(("RfkillRelease", last_off));
                        break;
                    }
                }
                WowlanTriggersAttrs::WakeupPkt80211(val) => {
                    if last_off == offset {
                        stack.push(("WakeupPkt80211", last_off));
                        break;
                    }
                }
                WowlanTriggersAttrs::WakeupPkt80211Len(val) => {
                    if last_off == offset {
                        stack.push(("WakeupPkt80211Len", last_off));
                        break;
                    }
                }
                WowlanTriggersAttrs::WakeupPkt8023(val) => {
                    if last_off == offset {
                        stack.push(("WakeupPkt8023", last_off));
                        break;
                    }
                }
                WowlanTriggersAttrs::WakeupPkt8023Len(val) => {
                    if last_off == offset {
                        stack.push(("WakeupPkt8023Len", last_off));
                        break;
                    }
                }
                WowlanTriggersAttrs::TcpConnection(val) => {
                    if last_off == offset {
                        stack.push(("TcpConnection", last_off));
                        break;
                    }
                }
                WowlanTriggersAttrs::WakeupTcpMatch(val) => {
                    if last_off == offset {
                        stack.push(("WakeupTcpMatch", last_off));
                        break;
                    }
                }
                WowlanTriggersAttrs::WakeupTcpConnlost(val) => {
                    if last_off == offset {
                        stack.push(("WakeupTcpConnlost", last_off));
                        break;
                    }
                }
                WowlanTriggersAttrs::WakeupTcpNomoretokens(val) => {
                    if last_off == offset {
                        stack.push(("WakeupTcpNomoretokens", last_off));
                        break;
                    }
                }
                WowlanTriggersAttrs::NetDetect(val) => {
                    if last_off == offset {
                        stack.push(("NetDetect", last_off));
                        break;
                    }
                }
                WowlanTriggersAttrs::NetDetectResults(val) => {
                    if last_off == offset {
                        stack.push(("NetDetectResults", last_off));
                        break;
                    }
                }
                WowlanTriggersAttrs::UnprotectedDeauthDisassoc(val) => {
                    if last_off == offset {
                        stack.push(("UnprotectedDeauthDisassoc", last_off));
                        break;
                    }
                }
                _ => {}
            };
            last_off = cur + attrs.pos;
        }
        if !stack.is_empty() {
            stack.push(("WowlanTriggersAttrs", cur));
        }
        (stack, None)
    }
}
pub struct PushNl80211Attrs<Prev: Rec> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Rec> Rec for PushNl80211Attrs<Prev> {
    fn as_rec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_rec_mut()
    }
}
impl<Prev: Rec> PushNl80211Attrs<Prev> {
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
    pub fn push_wiphy(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 1u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_wiphy_name(mut self, value: &CStr) -> Self {
        push_header(
            self.as_rec_mut(),
            2u16,
            value.to_bytes_with_nul().len() as u16,
        );
        self.as_rec_mut().extend(value.to_bytes_with_nul());
        self
    }
    pub fn push_wiphy_name_bytes(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 2u16, (value.len() + 1) as u16);
        self.as_rec_mut().extend(value);
        self.as_rec_mut().push(0);
        self
    }
    pub fn push_ifindex(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 3u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_ifname(mut self, value: &CStr) -> Self {
        push_header(
            self.as_rec_mut(),
            4u16,
            value.to_bytes_with_nul().len() as u16,
        );
        self.as_rec_mut().extend(value.to_bytes_with_nul());
        self
    }
    pub fn push_ifname_bytes(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 4u16, (value.len() + 1) as u16);
        self.as_rec_mut().extend(value);
        self.as_rec_mut().push(0);
        self
    }
    pub fn push_iftype(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 5u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_mac(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 6u16, value.len() as u16);
        self.as_rec_mut().extend(value);
        self
    }
    pub fn push_key_data(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 7u16, value.len() as u16);
        self.as_rec_mut().extend(value);
        self
    }
    pub fn push_key_idx(mut self, value: u8) -> Self {
        push_header(self.as_rec_mut(), 8u16, 1 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_key_cipher(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 9u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_key_seq(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 10u16, value.len() as u16);
        self.as_rec_mut().extend(value);
        self
    }
    pub fn push_key_default(mut self, value: ()) -> Self {
        push_header(self.as_rec_mut(), 11u16, 0 as u16);
        self
    }
    pub fn push_beacon_interval(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 12u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_dtim_period(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 13u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_beacon_head(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 14u16, value.len() as u16);
        self.as_rec_mut().extend(value);
        self
    }
    pub fn push_beacon_tail(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 15u16, value.len() as u16);
        self.as_rec_mut().extend(value);
        self
    }
    pub fn push_sta_aid(mut self, value: u16) -> Self {
        push_header(self.as_rec_mut(), 16u16, 2 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_sta_flags(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 17u16, value.len() as u16);
        self.as_rec_mut().extend(value);
        self
    }
    pub fn push_sta_listen_interval(mut self, value: u16) -> Self {
        push_header(self.as_rec_mut(), 18u16, 2 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_sta_supported_rates(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 19u16, value.len() as u16);
        self.as_rec_mut().extend(value);
        self
    }
    pub fn push_sta_vlan(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 20u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_sta_info(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 21u16, value.len() as u16);
        self.as_rec_mut().extend(value);
        self
    }
    pub fn nested_wiphy_bands(mut self) -> PushWiphyBands<Self> {
        let header_offset = push_nested_header(self.as_rec_mut(), 22u16);
        PushWiphyBands {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
    pub fn push_mntr_flags(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 23u16, value.len() as u16);
        self.as_rec_mut().extend(value);
        self
    }
    pub fn push_mesh_id(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 24u16, value.len() as u16);
        self.as_rec_mut().extend(value);
        self
    }
    pub fn push_sta_plink_action(mut self, value: u8) -> Self {
        push_header(self.as_rec_mut(), 25u16, 1 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_mpath_next_hop(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 26u16, value.len() as u16);
        self.as_rec_mut().extend(value);
        self
    }
    pub fn push_mpath_info(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 27u16, value.len() as u16);
        self.as_rec_mut().extend(value);
        self
    }
    pub fn push_bss_cts_prot(mut self, value: u8) -> Self {
        push_header(self.as_rec_mut(), 28u16, 1 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_bss_short_preamble(mut self, value: u8) -> Self {
        push_header(self.as_rec_mut(), 29u16, 1 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_bss_short_slot_time(mut self, value: u8) -> Self {
        push_header(self.as_rec_mut(), 30u16, 1 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_ht_capability(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 31u16, value.len() as u16);
        self.as_rec_mut().extend(value);
        self
    }
    pub fn nested_supported_iftypes(mut self) -> PushSupportedIftypes<Self> {
        let header_offset = push_nested_header(self.as_rec_mut(), 32u16);
        PushSupportedIftypes {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
    pub fn push_reg_alpha2(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 33u16, value.len() as u16);
        self.as_rec_mut().extend(value);
        self
    }
    pub fn push_reg_rules(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 34u16, value.len() as u16);
        self.as_rec_mut().extend(value);
        self
    }
    pub fn push_mesh_config(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 35u16, value.len() as u16);
        self.as_rec_mut().extend(value);
        self
    }
    pub fn push_bss_basic_rates(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 36u16, value.len() as u16);
        self.as_rec_mut().extend(value);
        self
    }
    pub fn push_wiphy_txq_params(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 37u16, value.len() as u16);
        self.as_rec_mut().extend(value);
        self
    }
    pub fn push_wiphy_freq(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 38u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Associated type: \"ChannelType\" (enum)"]
    pub fn push_wiphy_channel_type(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 39u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_key_default_mgmt(mut self, value: ()) -> Self {
        push_header(self.as_rec_mut(), 40u16, 0 as u16);
        self
    }
    pub fn push_mgmt_subtype(mut self, value: u8) -> Self {
        push_header(self.as_rec_mut(), 41u16, 1 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_ie(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 42u16, value.len() as u16);
        self.as_rec_mut().extend(value);
        self
    }
    pub fn push_max_num_scan_ssids(mut self, value: u8) -> Self {
        push_header(self.as_rec_mut(), 43u16, 1 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_scan_frequencies(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 44u16, value.len() as u16);
        self.as_rec_mut().extend(value);
        self
    }
    pub fn push_scan_ssids(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 45u16, value.len() as u16);
        self.as_rec_mut().extend(value);
        self
    }
    pub fn push_generation(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 46u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_bss(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 47u16, value.len() as u16);
        self.as_rec_mut().extend(value);
        self
    }
    pub fn push_reg_initiator(mut self, value: u8) -> Self {
        push_header(self.as_rec_mut(), 48u16, 1 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_reg_type(mut self, value: u8) -> Self {
        push_header(self.as_rec_mut(), 49u16, 1 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_frame(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 50u16, value.len() as u16);
        self.as_rec_mut().extend(value);
        self
    }
    pub fn push_ssid(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 51u16, value.len() as u16);
        self.as_rec_mut().extend(value);
        self
    }
    pub fn push_auth_type(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 52u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_reason_code(mut self, value: u16) -> Self {
        push_header(self.as_rec_mut(), 53u16, 2 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_key_type(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 54u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_max_scan_ie_len(mut self, value: u16) -> Self {
        push_header(self.as_rec_mut(), 55u16, 2 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_cipher_suites(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 56u16, value.len() as u16);
        self.as_rec_mut().extend(value);
        self
    }
    pub fn push_freq_before(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 57u16, value.len() as u16);
        self.as_rec_mut().extend(value);
        self
    }
    pub fn push_freq_after(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 58u16, value.len() as u16);
        self.as_rec_mut().extend(value);
        self
    }
    pub fn push_freq_fixed(mut self, value: ()) -> Self {
        push_header(self.as_rec_mut(), 59u16, 0 as u16);
        self
    }
    pub fn push_wiphy_retry_short(mut self, value: u8) -> Self {
        push_header(self.as_rec_mut(), 60u16, 1 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_wiphy_retry_long(mut self, value: u8) -> Self {
        push_header(self.as_rec_mut(), 61u16, 1 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_wiphy_frag_threshold(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 62u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_wiphy_rts_threshold(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 63u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_timed_out(mut self, value: ()) -> Self {
        push_header(self.as_rec_mut(), 64u16, 0 as u16);
        self
    }
    pub fn push_use_mfp(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 65u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_sta_flags2(mut self, value: PushStaFlagUpdate) -> Self {
        push_header(self.as_rec_mut(), 66u16, value.as_slice().len() as u16);
        self.as_rec_mut().extend(value.as_slice());
        self
    }
    pub fn push_control_port(mut self, value: ()) -> Self {
        push_header(self.as_rec_mut(), 67u16, 0 as u16);
        self
    }
    pub fn push_testdata(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 68u16, value.len() as u16);
        self.as_rec_mut().extend(value);
        self
    }
    pub fn push_privacy(mut self, value: ()) -> Self {
        push_header(self.as_rec_mut(), 69u16, 0 as u16);
        self
    }
    pub fn push_disconnected_by_ap(mut self, value: ()) -> Self {
        push_header(self.as_rec_mut(), 70u16, 0 as u16);
        self
    }
    pub fn push_status_code(mut self, value: u16) -> Self {
        push_header(self.as_rec_mut(), 71u16, 2 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_cipher_suites_pairwise(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 72u16, value.len() as u16);
        self.as_rec_mut().extend(value);
        self
    }
    pub fn push_cipher_suite_group(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 73u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_wpa_versions(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 74u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_akm_suites(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 75u16, value.len() as u16);
        self.as_rec_mut().extend(value);
        self
    }
    pub fn push_req_ie(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 76u16, value.len() as u16);
        self.as_rec_mut().extend(value);
        self
    }
    pub fn push_resp_ie(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 77u16, value.len() as u16);
        self.as_rec_mut().extend(value);
        self
    }
    pub fn push_prev_bssid(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 78u16, value.len() as u16);
        self.as_rec_mut().extend(value);
        self
    }
    pub fn push_key(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 79u16, value.len() as u16);
        self.as_rec_mut().extend(value);
        self
    }
    pub fn push_keys(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 80u16, value.len() as u16);
        self.as_rec_mut().extend(value);
        self
    }
    pub fn push_pid(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 81u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_4addr(mut self, value: u8) -> Self {
        push_header(self.as_rec_mut(), 82u16, 1 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_survey_info(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 83u16, value.len() as u16);
        self.as_rec_mut().extend(value);
        self
    }
    pub fn push_pmkid(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 84u16, value.len() as u16);
        self.as_rec_mut().extend(value);
        self
    }
    pub fn push_max_num_pmkids(mut self, value: u8) -> Self {
        push_header(self.as_rec_mut(), 85u16, 1 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_duration(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 86u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_cookie(mut self, value: u64) -> Self {
        push_header(self.as_rec_mut(), 87u16, 8 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_wiphy_coverage_class(mut self, value: u8) -> Self {
        push_header(self.as_rec_mut(), 88u16, 1 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_tx_rates(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 89u16, value.len() as u16);
        self.as_rec_mut().extend(value);
        self
    }
    pub fn push_frame_match(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 90u16, value.len() as u16);
        self.as_rec_mut().extend(value);
        self
    }
    pub fn push_ack(mut self, value: ()) -> Self {
        push_header(self.as_rec_mut(), 91u16, 0 as u16);
        self
    }
    pub fn push_ps_state(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 92u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_cqm(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 93u16, value.len() as u16);
        self.as_rec_mut().extend(value);
        self
    }
    pub fn push_local_state_change(mut self, value: ()) -> Self {
        push_header(self.as_rec_mut(), 94u16, 0 as u16);
        self
    }
    pub fn push_ap_isolate(mut self, value: u8) -> Self {
        push_header(self.as_rec_mut(), 95u16, 1 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_wiphy_tx_power_setting(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 96u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_wiphy_tx_power_level(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 97u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn nested_tx_frame_types(mut self) -> PushIftypeAttrs<Self> {
        let header_offset = push_nested_header(self.as_rec_mut(), 98u16);
        PushIftypeAttrs {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
    pub fn nested_rx_frame_types(mut self) -> PushIftypeAttrs<Self> {
        let header_offset = push_nested_header(self.as_rec_mut(), 99u16);
        PushIftypeAttrs {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
    pub fn push_frame_type(mut self, value: u16) -> Self {
        push_header(self.as_rec_mut(), 100u16, 2 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_control_port_ethertype(mut self, value: ()) -> Self {
        push_header(self.as_rec_mut(), 101u16, 0 as u16);
        self
    }
    pub fn push_control_port_no_encrypt(mut self, value: ()) -> Self {
        push_header(self.as_rec_mut(), 102u16, 0 as u16);
        self
    }
    pub fn push_support_ibss_rsn(mut self, value: ()) -> Self {
        push_header(self.as_rec_mut(), 103u16, 0 as u16);
        self
    }
    pub fn push_wiphy_antenna_tx(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 104u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_wiphy_antenna_rx(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 105u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_mcast_rate(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 106u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_offchannel_tx_ok(mut self, value: ()) -> Self {
        push_header(self.as_rec_mut(), 107u16, 0 as u16);
        self
    }
    pub fn push_bss_ht_opmode(mut self, value: u16) -> Self {
        push_header(self.as_rec_mut(), 108u16, 2 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_key_default_types(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 109u16, value.len() as u16);
        self.as_rec_mut().extend(value);
        self
    }
    pub fn push_max_remain_on_channel_duration(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 110u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_mesh_setup(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 111u16, value.len() as u16);
        self.as_rec_mut().extend(value);
        self
    }
    pub fn push_wiphy_antenna_avail_tx(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 112u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_wiphy_antenna_avail_rx(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 113u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_support_mesh_auth(mut self, value: ()) -> Self {
        push_header(self.as_rec_mut(), 114u16, 0 as u16);
        self
    }
    pub fn push_sta_plink_state(mut self, value: u8) -> Self {
        push_header(self.as_rec_mut(), 115u16, 1 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_wowlan_triggers(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 116u16, value.len() as u16);
        self.as_rec_mut().extend(value);
        self
    }
    pub fn nested_wowlan_triggers_supported(mut self) -> PushWowlanTriggersAttrs<Self> {
        let header_offset = push_nested_header(self.as_rec_mut(), 117u16);
        PushWowlanTriggersAttrs {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
    pub fn push_sched_scan_interval(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 118u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_interface_combinations(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 119u16, value.len() as u16);
        self.as_rec_mut().extend(value);
        self
    }
    pub fn nested_software_iftypes(mut self) -> PushSupportedIftypes<Self> {
        let header_offset = push_nested_header(self.as_rec_mut(), 120u16);
        PushSupportedIftypes {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
    pub fn push_rekey_data(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 121u16, value.len() as u16);
        self.as_rec_mut().extend(value);
        self
    }
    pub fn push_max_num_sched_scan_ssids(mut self, value: u8) -> Self {
        push_header(self.as_rec_mut(), 122u16, 1 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_max_sched_scan_ie_len(mut self, value: u16) -> Self {
        push_header(self.as_rec_mut(), 123u16, 2 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_scan_supp_rates(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 124u16, value.len() as u16);
        self.as_rec_mut().extend(value);
        self
    }
    pub fn push_hidden_ssid(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 125u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_ie_probe_resp(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 126u16, value.len() as u16);
        self.as_rec_mut().extend(value);
        self
    }
    pub fn push_ie_assoc_resp(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 127u16, value.len() as u16);
        self.as_rec_mut().extend(value);
        self
    }
    pub fn push_sta_wme(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 128u16, value.len() as u16);
        self.as_rec_mut().extend(value);
        self
    }
    pub fn push_support_ap_uapsd(mut self, value: ()) -> Self {
        push_header(self.as_rec_mut(), 129u16, 0 as u16);
        self
    }
    pub fn push_roam_support(mut self, value: ()) -> Self {
        push_header(self.as_rec_mut(), 130u16, 0 as u16);
        self
    }
    pub fn push_sched_scan_match(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 131u16, value.len() as u16);
        self.as_rec_mut().extend(value);
        self
    }
    pub fn push_max_match_sets(mut self, value: u8) -> Self {
        push_header(self.as_rec_mut(), 132u16, 1 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_pmksa_candidate(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 133u16, value.len() as u16);
        self.as_rec_mut().extend(value);
        self
    }
    pub fn push_tx_no_cck_rate(mut self, value: ()) -> Self {
        push_header(self.as_rec_mut(), 134u16, 0 as u16);
        self
    }
    pub fn push_tdls_action(mut self, value: u8) -> Self {
        push_header(self.as_rec_mut(), 135u16, 1 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_tdls_dialog_token(mut self, value: u8) -> Self {
        push_header(self.as_rec_mut(), 136u16, 1 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_tdls_operation(mut self, value: u8) -> Self {
        push_header(self.as_rec_mut(), 137u16, 1 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_tdls_support(mut self, value: ()) -> Self {
        push_header(self.as_rec_mut(), 138u16, 0 as u16);
        self
    }
    pub fn push_tdls_external_setup(mut self, value: ()) -> Self {
        push_header(self.as_rec_mut(), 139u16, 0 as u16);
        self
    }
    pub fn push_device_ap_sme(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 140u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_dont_wait_for_ack(mut self, value: ()) -> Self {
        push_header(self.as_rec_mut(), 141u16, 0 as u16);
        self
    }
    #[doc = "Associated type: \"FeatureFlags\" (1 bit per enumeration)"]
    pub fn push_feature_flags(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 142u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_probe_resp_offload(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 143u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_probe_resp(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 144u16, value.len() as u16);
        self.as_rec_mut().extend(value);
        self
    }
    pub fn push_dfs_region(mut self, value: u8) -> Self {
        push_header(self.as_rec_mut(), 145u16, 1 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_disable_ht(mut self, value: ()) -> Self {
        push_header(self.as_rec_mut(), 146u16, 0 as u16);
        self
    }
    pub fn push_ht_capability_mask(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 147u16, value.len() as u16);
        self.as_rec_mut().extend(value);
        self
    }
    pub fn push_noack_map(mut self, value: u16) -> Self {
        push_header(self.as_rec_mut(), 148u16, 2 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_inactivity_timeout(mut self, value: u16) -> Self {
        push_header(self.as_rec_mut(), 149u16, 2 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_rx_signal_dbm(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 150u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_bg_scan_period(mut self, value: u16) -> Self {
        push_header(self.as_rec_mut(), 151u16, 2 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_wdev(mut self, value: u64) -> Self {
        push_header(self.as_rec_mut(), 152u16, 8 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_user_reg_hint_type(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 153u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_conn_failed_reason(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 154u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_auth_data(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 155u16, value.len() as u16);
        self.as_rec_mut().extend(value);
        self
    }
    pub fn push_vht_capability(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 156u16, value.len() as u16);
        self.as_rec_mut().extend(value);
        self
    }
    pub fn push_scan_flags(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 157u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_channel_width(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 158u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_center_freq1(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 159u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_center_freq2(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 160u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_p2p_ctwindow(mut self, value: u8) -> Self {
        push_header(self.as_rec_mut(), 161u16, 1 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_p2p_oppps(mut self, value: u8) -> Self {
        push_header(self.as_rec_mut(), 162u16, 1 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_local_mesh_power_mode(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 163u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_acl_policy(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 164u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_mac_addrs(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 165u16, value.len() as u16);
        self.as_rec_mut().extend(value);
        self
    }
    pub fn push_mac_acl_max(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 166u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_radar_event(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 167u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_ext_capa(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 168u16, value.len() as u16);
        self.as_rec_mut().extend(value);
        self
    }
    pub fn push_ext_capa_mask(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 169u16, value.len() as u16);
        self.as_rec_mut().extend(value);
        self
    }
    pub fn push_sta_capability(mut self, value: u16) -> Self {
        push_header(self.as_rec_mut(), 170u16, 2 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_sta_ext_capability(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 171u16, value.len() as u16);
        self.as_rec_mut().extend(value);
        self
    }
    #[doc = "Associated type: \"ProtocolFeatures\" (enum)"]
    pub fn push_protocol_features(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 172u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_split_wiphy_dump(mut self, value: ()) -> Self {
        push_header(self.as_rec_mut(), 173u16, 0 as u16);
        self
    }
    pub fn push_disable_vht(mut self, value: ()) -> Self {
        push_header(self.as_rec_mut(), 174u16, 0 as u16);
        self
    }
    pub fn push_vht_capability_mask(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 175u16, value.len() as u16);
        self.as_rec_mut().extend(value);
        self
    }
    pub fn push_mdid(mut self, value: u16) -> Self {
        push_header(self.as_rec_mut(), 176u16, 2 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_ie_ric(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 177u16, value.len() as u16);
        self.as_rec_mut().extend(value);
        self
    }
    pub fn push_crit_prot_id(mut self, value: u16) -> Self {
        push_header(self.as_rec_mut(), 178u16, 2 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_max_crit_prot_duration(mut self, value: u16) -> Self {
        push_header(self.as_rec_mut(), 179u16, 2 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_peer_aid(mut self, value: u16) -> Self {
        push_header(self.as_rec_mut(), 180u16, 2 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_coalesce_rule(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 181u16, value.len() as u16);
        self.as_rec_mut().extend(value);
        self
    }
    pub fn push_ch_switch_count(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 182u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_ch_switch_block_tx(mut self, value: ()) -> Self {
        push_header(self.as_rec_mut(), 183u16, 0 as u16);
        self
    }
    pub fn push_csa_ies(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 184u16, value.len() as u16);
        self.as_rec_mut().extend(value);
        self
    }
    pub fn push_cntdwn_offs_beacon(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 185u16, value.len() as u16);
        self.as_rec_mut().extend(value);
        self
    }
    pub fn push_cntdwn_offs_presp(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 186u16, value.len() as u16);
        self.as_rec_mut().extend(value);
        self
    }
    pub fn push_rxmgmt_flags(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 187u16, value.len() as u16);
        self.as_rec_mut().extend(value);
        self
    }
    pub fn push_sta_supported_channels(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 188u16, value.len() as u16);
        self.as_rec_mut().extend(value);
        self
    }
    pub fn push_sta_supported_oper_classes(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 189u16, value.len() as u16);
        self.as_rec_mut().extend(value);
        self
    }
    pub fn push_handle_dfs(mut self, value: ()) -> Self {
        push_header(self.as_rec_mut(), 190u16, 0 as u16);
        self
    }
    pub fn push_support_5_mhz(mut self, value: ()) -> Self {
        push_header(self.as_rec_mut(), 191u16, 0 as u16);
        self
    }
    pub fn push_support_10_mhz(mut self, value: ()) -> Self {
        push_header(self.as_rec_mut(), 192u16, 0 as u16);
        self
    }
    pub fn push_opmode_notif(mut self, value: u8) -> Self {
        push_header(self.as_rec_mut(), 193u16, 1 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_vendor_id(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 194u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_vendor_subcmd(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 195u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_vendor_data(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 196u16, value.len() as u16);
        self.as_rec_mut().extend(value);
        self
    }
    pub fn push_vendor_events(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 197u16, value.len() as u16);
        self.as_rec_mut().extend(value);
        self
    }
    pub fn push_qos_map(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 198u16, value.len() as u16);
        self.as_rec_mut().extend(value);
        self
    }
    pub fn push_mac_hint(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 199u16, value.len() as u16);
        self.as_rec_mut().extend(value);
        self
    }
    pub fn push_wiphy_freq_hint(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 200u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_max_ap_assoc_sta(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 201u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_tdls_peer_capability(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 202u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_socket_owner(mut self, value: ()) -> Self {
        push_header(self.as_rec_mut(), 203u16, 0 as u16);
        self
    }
    pub fn push_csa_c_offsets_tx(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 204u16, value.len() as u16);
        self.as_rec_mut().extend(value);
        self
    }
    pub fn push_max_csa_counters(mut self, value: u8) -> Self {
        push_header(self.as_rec_mut(), 205u16, 1 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_tdls_initiator(mut self, value: ()) -> Self {
        push_header(self.as_rec_mut(), 206u16, 0 as u16);
        self
    }
    pub fn push_use_rrm(mut self, value: ()) -> Self {
        push_header(self.as_rec_mut(), 207u16, 0 as u16);
        self
    }
    pub fn push_wiphy_dyn_ack(mut self, value: ()) -> Self {
        push_header(self.as_rec_mut(), 208u16, 0 as u16);
        self
    }
    pub fn push_tsid(mut self, value: u8) -> Self {
        push_header(self.as_rec_mut(), 209u16, 1 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_user_prio(mut self, value: u8) -> Self {
        push_header(self.as_rec_mut(), 210u16, 1 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_admitted_time(mut self, value: u16) -> Self {
        push_header(self.as_rec_mut(), 211u16, 2 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_smps_mode(mut self, value: u8) -> Self {
        push_header(self.as_rec_mut(), 212u16, 1 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_oper_class(mut self, value: u8) -> Self {
        push_header(self.as_rec_mut(), 213u16, 1 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_mac_mask(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 214u16, value.len() as u16);
        self.as_rec_mut().extend(value);
        self
    }
    pub fn push_wiphy_self_managed_reg(mut self, value: ()) -> Self {
        push_header(self.as_rec_mut(), 215u16, 0 as u16);
        self
    }
    pub fn push_ext_features(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 216u16, value.len() as u16);
        self.as_rec_mut().extend(value);
        self
    }
    pub fn push_survey_radio_stats(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 217u16, value.len() as u16);
        self.as_rec_mut().extend(value);
        self
    }
    pub fn push_netns_fd(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 218u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_sched_scan_delay(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 219u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_reg_indoor(mut self, value: ()) -> Self {
        push_header(self.as_rec_mut(), 220u16, 0 as u16);
        self
    }
    pub fn push_max_num_sched_scan_plans(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 221u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_max_scan_plan_interval(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 222u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_max_scan_plan_iterations(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 223u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_sched_scan_plans(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 224u16, value.len() as u16);
        self.as_rec_mut().extend(value);
        self
    }
    pub fn push_pbss(mut self, value: ()) -> Self {
        push_header(self.as_rec_mut(), 225u16, 0 as u16);
        self
    }
    pub fn push_bss_select(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 226u16, value.len() as u16);
        self.as_rec_mut().extend(value);
        self
    }
    pub fn push_sta_support_p2p_ps(mut self, value: u8) -> Self {
        push_header(self.as_rec_mut(), 227u16, 1 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_pad(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 228u16, value.len() as u16);
        self.as_rec_mut().extend(value);
        self
    }
    pub fn push_iftype_ext_capa(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 229u16, value.len() as u16);
        self.as_rec_mut().extend(value);
        self
    }
    pub fn push_mu_mimo_group_data(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 230u16, value.len() as u16);
        self.as_rec_mut().extend(value);
        self
    }
    pub fn push_mu_mimo_follow_mac_addr(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 231u16, value.len() as u16);
        self.as_rec_mut().extend(value);
        self
    }
    pub fn push_scan_start_time_tsf(mut self, value: u64) -> Self {
        push_header(self.as_rec_mut(), 232u16, 8 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_scan_start_time_tsf_bssid(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 233u16, value.len() as u16);
        self.as_rec_mut().extend(value);
        self
    }
    pub fn push_measurement_duration(mut self, value: u16) -> Self {
        push_header(self.as_rec_mut(), 234u16, 2 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_measurement_duration_mandatory(mut self, value: ()) -> Self {
        push_header(self.as_rec_mut(), 235u16, 0 as u16);
        self
    }
    pub fn push_mesh_peer_aid(mut self, value: u16) -> Self {
        push_header(self.as_rec_mut(), 236u16, 2 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_nan_master_pref(mut self, value: u8) -> Self {
        push_header(self.as_rec_mut(), 237u16, 1 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_bands(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 238u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_nan_func(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 239u16, value.len() as u16);
        self.as_rec_mut().extend(value);
        self
    }
    pub fn push_nan_match(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 240u16, value.len() as u16);
        self.as_rec_mut().extend(value);
        self
    }
    pub fn push_fils_kek(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 241u16, value.len() as u16);
        self.as_rec_mut().extend(value);
        self
    }
    pub fn push_fils_nonces(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 242u16, value.len() as u16);
        self.as_rec_mut().extend(value);
        self
    }
    pub fn push_multicast_to_unicast_enabled(mut self, value: ()) -> Self {
        push_header(self.as_rec_mut(), 243u16, 0 as u16);
        self
    }
    pub fn push_bssid(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 244u16, value.len() as u16);
        self.as_rec_mut().extend(value);
        self
    }
    pub fn push_sched_scan_relative_rssi(mut self, value: i8) -> Self {
        push_header(self.as_rec_mut(), 245u16, 1 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_sched_scan_rssi_adjust(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 246u16, value.len() as u16);
        self.as_rec_mut().extend(value);
        self
    }
    pub fn push_timeout_reason(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 247u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_fils_erp_username(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 248u16, value.len() as u16);
        self.as_rec_mut().extend(value);
        self
    }
    pub fn push_fils_erp_realm(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 249u16, value.len() as u16);
        self.as_rec_mut().extend(value);
        self
    }
    pub fn push_fils_erp_next_seq_num(mut self, value: u16) -> Self {
        push_header(self.as_rec_mut(), 250u16, 2 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_fils_erp_rrk(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 251u16, value.len() as u16);
        self.as_rec_mut().extend(value);
        self
    }
    pub fn push_fils_cache_id(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 252u16, value.len() as u16);
        self.as_rec_mut().extend(value);
        self
    }
    pub fn push_pmk(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 253u16, value.len() as u16);
        self.as_rec_mut().extend(value);
        self
    }
    pub fn push_sched_scan_multi(mut self, value: ()) -> Self {
        push_header(self.as_rec_mut(), 254u16, 0 as u16);
        self
    }
    pub fn push_sched_scan_max_reqs(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 255u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_want_1x_4way_hs(mut self, value: ()) -> Self {
        push_header(self.as_rec_mut(), 256u16, 0 as u16);
        self
    }
    pub fn push_pmkr0_name(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 257u16, value.len() as u16);
        self.as_rec_mut().extend(value);
        self
    }
    pub fn push_port_authorized(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 258u16, value.len() as u16);
        self.as_rec_mut().extend(value);
        self
    }
    pub fn push_external_auth_action(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 259u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_external_auth_support(mut self, value: ()) -> Self {
        push_header(self.as_rec_mut(), 260u16, 0 as u16);
        self
    }
    pub fn push_nss(mut self, value: u8) -> Self {
        push_header(self.as_rec_mut(), 261u16, 1 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_ack_signal(mut self, value: i32) -> Self {
        push_header(self.as_rec_mut(), 262u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_control_port_over_nl80211(mut self, value: ()) -> Self {
        push_header(self.as_rec_mut(), 263u16, 0 as u16);
        self
    }
    pub fn nested_txq_stats(mut self) -> PushTxqStatsAttrs<Self> {
        let header_offset = push_nested_header(self.as_rec_mut(), 264u16);
        PushTxqStatsAttrs {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
    pub fn push_txq_limit(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 265u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_txq_memory_limit(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 266u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_txq_quantum(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 267u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_he_capability(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 268u16, value.len() as u16);
        self.as_rec_mut().extend(value);
        self
    }
    pub fn push_ftm_responder(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 269u16, value.len() as u16);
        self.as_rec_mut().extend(value);
        self
    }
    pub fn push_ftm_responder_stats(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 270u16, value.len() as u16);
        self.as_rec_mut().extend(value);
        self
    }
    pub fn push_timeout(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 271u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_peer_measurements(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 272u16, value.len() as u16);
        self.as_rec_mut().extend(value);
        self
    }
    pub fn push_airtime_weight(mut self, value: u16) -> Self {
        push_header(self.as_rec_mut(), 273u16, 2 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_sta_tx_power_setting(mut self, value: u8) -> Self {
        push_header(self.as_rec_mut(), 274u16, 1 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_sta_tx_power(mut self, value: i16) -> Self {
        push_header(self.as_rec_mut(), 275u16, 2 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_sae_password(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 276u16, value.len() as u16);
        self.as_rec_mut().extend(value);
        self
    }
    pub fn push_twt_responder(mut self, value: ()) -> Self {
        push_header(self.as_rec_mut(), 277u16, 0 as u16);
        self
    }
    pub fn push_he_obss_pd(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 278u16, value.len() as u16);
        self.as_rec_mut().extend(value);
        self
    }
    pub fn push_wiphy_edmg_channels(mut self, value: u8) -> Self {
        push_header(self.as_rec_mut(), 279u16, 1 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_wiphy_edmg_bw_config(mut self, value: u8) -> Self {
        push_header(self.as_rec_mut(), 280u16, 1 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_vlan_id(mut self, value: u16) -> Self {
        push_header(self.as_rec_mut(), 281u16, 2 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_he_bss_color(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 282u16, value.len() as u16);
        self.as_rec_mut().extend(value);
        self
    }
    pub fn push_iftype_akm_suites(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 283u16, value.len() as u16);
        self.as_rec_mut().extend(value);
        self
    }
    pub fn push_tid_config(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 284u16, value.len() as u16);
        self.as_rec_mut().extend(value);
        self
    }
    pub fn push_control_port_no_preauth(mut self, value: ()) -> Self {
        push_header(self.as_rec_mut(), 285u16, 0 as u16);
        self
    }
    pub fn push_pmk_lifetime(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 286u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_pmk_reauth_threshold(mut self, value: u8) -> Self {
        push_header(self.as_rec_mut(), 287u16, 1 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_receive_multicast(mut self, value: ()) -> Self {
        push_header(self.as_rec_mut(), 288u16, 0 as u16);
        self
    }
    pub fn push_wiphy_freq_offset(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 289u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_center_freq1_offset(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 290u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_scan_freq_khz(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 291u16, value.len() as u16);
        self.as_rec_mut().extend(value);
        self
    }
    pub fn push_he_6ghz_capability(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 292u16, value.len() as u16);
        self.as_rec_mut().extend(value);
        self
    }
    pub fn push_fils_discovery(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 293u16, value.len() as u16);
        self.as_rec_mut().extend(value);
        self
    }
    pub fn push_unsol_bcast_probe_resp(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 294u16, value.len() as u16);
        self.as_rec_mut().extend(value);
        self
    }
    pub fn push_s1g_capability(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 295u16, value.len() as u16);
        self.as_rec_mut().extend(value);
        self
    }
    pub fn push_s1g_capability_mask(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 296u16, value.len() as u16);
        self.as_rec_mut().extend(value);
        self
    }
    pub fn push_sae_pwe(mut self, value: u8) -> Self {
        push_header(self.as_rec_mut(), 297u16, 1 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_reconnect_requested(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 298u16, value.len() as u16);
        self.as_rec_mut().extend(value);
        self
    }
    pub fn push_sar_spec(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 299u16, value.len() as u16);
        self.as_rec_mut().extend(value);
        self
    }
    pub fn push_disable_he(mut self, value: ()) -> Self {
        push_header(self.as_rec_mut(), 300u16, 0 as u16);
        self
    }
    pub fn push_obss_color_bitmap(mut self, value: u64) -> Self {
        push_header(self.as_rec_mut(), 301u16, 8 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_color_change_count(mut self, value: u8) -> Self {
        push_header(self.as_rec_mut(), 302u16, 1 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_color_change_color(mut self, value: u8) -> Self {
        push_header(self.as_rec_mut(), 303u16, 1 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_color_change_elems(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 304u16, value.len() as u16);
        self.as_rec_mut().extend(value);
        self
    }
    pub fn push_mbssid_config(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 305u16, value.len() as u16);
        self.as_rec_mut().extend(value);
        self
    }
    pub fn push_mbssid_elems(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 306u16, value.len() as u16);
        self.as_rec_mut().extend(value);
        self
    }
    pub fn push_radar_background(mut self, value: ()) -> Self {
        push_header(self.as_rec_mut(), 307u16, 0 as u16);
        self
    }
    pub fn push_ap_settings_flags(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 308u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_eht_capability(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 309u16, value.len() as u16);
        self.as_rec_mut().extend(value);
        self
    }
    pub fn push_disable_eht(mut self, value: ()) -> Self {
        push_header(self.as_rec_mut(), 310u16, 0 as u16);
        self
    }
    pub fn push_mlo_links(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 311u16, value.len() as u16);
        self.as_rec_mut().extend(value);
        self
    }
    pub fn push_mlo_link_id(mut self, value: u8) -> Self {
        push_header(self.as_rec_mut(), 312u16, 1 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_mld_addr(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 313u16, value.len() as u16);
        self.as_rec_mut().extend(value);
        self
    }
    pub fn push_mlo_support(mut self, value: ()) -> Self {
        push_header(self.as_rec_mut(), 314u16, 0 as u16);
        self
    }
    pub fn push_max_num_akm_suites(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 315u16, value.len() as u16);
        self.as_rec_mut().extend(value);
        self
    }
    pub fn push_eml_capability(mut self, value: u16) -> Self {
        push_header(self.as_rec_mut(), 316u16, 2 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_mld_capa_and_ops(mut self, value: u16) -> Self {
        push_header(self.as_rec_mut(), 317u16, 2 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_tx_hw_timestamp(mut self, value: u64) -> Self {
        push_header(self.as_rec_mut(), 318u16, 8 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_rx_hw_timestamp(mut self, value: u64) -> Self {
        push_header(self.as_rec_mut(), 319u16, 8 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_td_bitmap(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 320u16, value.len() as u16);
        self.as_rec_mut().extend(value);
        self
    }
    pub fn push_punct_bitmap(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 321u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_max_hw_timestamp_peers(mut self, value: u16) -> Self {
        push_header(self.as_rec_mut(), 322u16, 2 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_hw_timestamp_enabled(mut self, value: ()) -> Self {
        push_header(self.as_rec_mut(), 323u16, 0 as u16);
        self
    }
    pub fn push_ema_rnr_elems(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 324u16, value.len() as u16);
        self.as_rec_mut().extend(value);
        self
    }
    pub fn push_mlo_link_disabled(mut self, value: ()) -> Self {
        push_header(self.as_rec_mut(), 325u16, 0 as u16);
        self
    }
    pub fn push_bss_dump_include_use_data(mut self, value: ()) -> Self {
        push_header(self.as_rec_mut(), 326u16, 0 as u16);
        self
    }
    pub fn push_mlo_ttlm_dlink(mut self, value: u16) -> Self {
        push_header(self.as_rec_mut(), 327u16, 2 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_mlo_ttlm_ulink(mut self, value: u16) -> Self {
        push_header(self.as_rec_mut(), 328u16, 2 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_assoc_spp_amsdu(mut self, value: ()) -> Self {
        push_header(self.as_rec_mut(), 329u16, 0 as u16);
        self
    }
    pub fn push_wiphy_radios(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 330u16, value.len() as u16);
        self.as_rec_mut().extend(value);
        self
    }
    pub fn push_wiphy_interface_combinations(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 331u16, value.len() as u16);
        self.as_rec_mut().extend(value);
        self
    }
    pub fn push_vif_radio_mask(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 332u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
}
impl<Prev: Rec> Drop for PushNl80211Attrs<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_rec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushFrameTypeAttrs<Prev: Rec> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Rec> Rec for PushFrameTypeAttrs<Prev> {
    fn as_rec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_rec_mut()
    }
}
impl<Prev: Rec> PushFrameTypeAttrs<Prev> {
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
    pub fn push_frame_type(mut self, value: u16) -> Self {
        push_header(self.as_rec_mut(), 1u16, 2 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
}
impl<Prev: Rec> Drop for PushFrameTypeAttrs<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_rec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushWiphyBands<Prev: Rec> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Rec> Rec for PushWiphyBands<Prev> {
    fn as_rec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_rec_mut()
    }
}
impl<Prev: Rec> PushWiphyBands<Prev> {
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
    #[doc = "2.4 GHz ISM band"]
    pub fn nested_2ghz(mut self) -> PushBandAttrs<Self> {
        let header_offset = push_nested_header(self.as_rec_mut(), 0u16);
        PushBandAttrs {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
    #[doc = "around 5 GHz band (4.9 - 5.7 GHz)"]
    pub fn nested_5ghz(mut self) -> PushBandAttrs<Self> {
        let header_offset = push_nested_header(self.as_rec_mut(), 1u16);
        PushBandAttrs {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
    #[doc = "around 60 GHz band (58.32 - 69.12 GHz)"]
    pub fn nested_60ghz(mut self) -> PushBandAttrs<Self> {
        let header_offset = push_nested_header(self.as_rec_mut(), 2u16);
        PushBandAttrs {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
    pub fn nested_6ghz(mut self) -> PushBandAttrs<Self> {
        let header_offset = push_nested_header(self.as_rec_mut(), 3u16);
        PushBandAttrs {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
    pub fn nested_s1ghz(mut self) -> PushBandAttrs<Self> {
        let header_offset = push_nested_header(self.as_rec_mut(), 4u16);
        PushBandAttrs {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
    pub fn nested_lc(mut self) -> PushBandAttrs<Self> {
        let header_offset = push_nested_header(self.as_rec_mut(), 5u16);
        PushBandAttrs {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
}
impl<Prev: Rec> Drop for PushWiphyBands<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_rec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushBandAttrs<Prev: Rec> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Rec> Rec for PushBandAttrs<Prev> {
    fn as_rec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_rec_mut()
    }
}
pub struct PushArrayFrequencyAttrs<Prev: Rec> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
    pub(crate) counter: u16,
}
impl<Prev: Rec> Rec for PushArrayFrequencyAttrs<Prev> {
    fn as_rec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_rec_mut()
    }
}
impl<Prev: Rec> PushArrayFrequencyAttrs<Prev> {
    pub fn new(prev: Prev) -> Self {
        Self {
            prev: Some(prev),
            header_offset: None,
            counter: 0,
        }
    }
    pub fn end_array(mut self) -> Prev {
        let mut prev = self.prev.take().unwrap();
        if let Some(header_offset) = &self.header_offset {
            finalize_nested_header(prev.as_rec_mut(), *header_offset);
        }
        prev
    }
    pub fn entry_nested(mut self) -> PushFrequencyAttrs<Self> {
        let index = self.counter;
        self.counter += 1;
        let header_offset = push_nested_header(self.as_rec_mut(), index);
        PushFrequencyAttrs {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
}
impl<Prev: Rec> Drop for PushArrayFrequencyAttrs<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_rec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushArrayBitrateAttrs<Prev: Rec> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
    pub(crate) counter: u16,
}
impl<Prev: Rec> Rec for PushArrayBitrateAttrs<Prev> {
    fn as_rec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_rec_mut()
    }
}
impl<Prev: Rec> PushArrayBitrateAttrs<Prev> {
    pub fn new(prev: Prev) -> Self {
        Self {
            prev: Some(prev),
            header_offset: None,
            counter: 0,
        }
    }
    pub fn end_array(mut self) -> Prev {
        let mut prev = self.prev.take().unwrap();
        if let Some(header_offset) = &self.header_offset {
            finalize_nested_header(prev.as_rec_mut(), *header_offset);
        }
        prev
    }
    pub fn entry_nested(mut self) -> PushBitrateAttrs<Self> {
        let index = self.counter;
        self.counter += 1;
        let header_offset = push_nested_header(self.as_rec_mut(), index);
        PushBitrateAttrs {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
}
impl<Prev: Rec> Drop for PushArrayBitrateAttrs<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_rec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushArrayIftypeDataAttrs<Prev: Rec> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
    pub(crate) counter: u16,
}
impl<Prev: Rec> Rec for PushArrayIftypeDataAttrs<Prev> {
    fn as_rec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_rec_mut()
    }
}
impl<Prev: Rec> PushArrayIftypeDataAttrs<Prev> {
    pub fn new(prev: Prev) -> Self {
        Self {
            prev: Some(prev),
            header_offset: None,
            counter: 0,
        }
    }
    pub fn end_array(mut self) -> Prev {
        let mut prev = self.prev.take().unwrap();
        if let Some(header_offset) = &self.header_offset {
            finalize_nested_header(prev.as_rec_mut(), *header_offset);
        }
        prev
    }
    pub fn entry_nested(mut self) -> PushIftypeDataAttrs<Self> {
        let index = self.counter;
        self.counter += 1;
        let header_offset = push_nested_header(self.as_rec_mut(), index);
        PushIftypeDataAttrs {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
}
impl<Prev: Rec> Drop for PushArrayIftypeDataAttrs<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_rec_mut(), *header_offset);
            }
        }
    }
}
impl<Prev: Rec> PushBandAttrs<Prev> {
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
    pub fn array_freqs(mut self) -> PushArrayFrequencyAttrs<Self> {
        let header_offset = push_nested_header(self.as_rec_mut(), 1u16);
        PushArrayFrequencyAttrs {
            prev: Some(self),
            header_offset: Some(header_offset),
            counter: 0,
        }
    }
    pub fn array_rates(mut self) -> PushArrayBitrateAttrs<Self> {
        let header_offset = push_nested_header(self.as_rec_mut(), 2u16);
        PushArrayBitrateAttrs {
            prev: Some(self),
            header_offset: Some(header_offset),
            counter: 0,
        }
    }
    pub fn push_ht_mcs_set(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 3u16, value.len() as u16);
        self.as_rec_mut().extend(value);
        self
    }
    pub fn push_ht_capa(mut self, value: u16) -> Self {
        push_header(self.as_rec_mut(), 4u16, 2 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_ht_ampdu_factor(mut self, value: u8) -> Self {
        push_header(self.as_rec_mut(), 5u16, 1 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_ht_ampdu_density(mut self, value: u8) -> Self {
        push_header(self.as_rec_mut(), 6u16, 1 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_vht_mcs_set(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 7u16, value.len() as u16);
        self.as_rec_mut().extend(value);
        self
    }
    pub fn push_vht_capa(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 8u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn array_iftype_data(mut self) -> PushArrayIftypeDataAttrs<Self> {
        let header_offset = push_nested_header(self.as_rec_mut(), 9u16);
        PushArrayIftypeDataAttrs {
            prev: Some(self),
            header_offset: Some(header_offset),
            counter: 0,
        }
    }
    pub fn push_edmg_channels(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 10u16, value.len() as u16);
        self.as_rec_mut().extend(value);
        self
    }
    pub fn push_edmg_bw_config(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 11u16, value.len() as u16);
        self.as_rec_mut().extend(value);
        self
    }
    pub fn push_s1g_mcs_nss_set(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 12u16, value.len() as u16);
        self.as_rec_mut().extend(value);
        self
    }
    pub fn push_s1g_capa(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 13u16, value.len() as u16);
        self.as_rec_mut().extend(value);
        self
    }
}
impl<Prev: Rec> Drop for PushBandAttrs<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_rec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushBitrateAttrs<Prev: Rec> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Rec> Rec for PushBitrateAttrs<Prev> {
    fn as_rec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_rec_mut()
    }
}
impl<Prev: Rec> PushBitrateAttrs<Prev> {
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
    pub fn push_rate(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 1u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_2ghz_shortpreamble(mut self, value: ()) -> Self {
        push_header(self.as_rec_mut(), 2u16, 0 as u16);
        self
    }
}
impl<Prev: Rec> Drop for PushBitrateAttrs<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_rec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushFrequencyAttrs<Prev: Rec> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Rec> Rec for PushFrequencyAttrs<Prev> {
    fn as_rec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_rec_mut()
    }
}
pub struct PushArrayWmmAttrs<Prev: Rec> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
    pub(crate) counter: u16,
}
impl<Prev: Rec> Rec for PushArrayWmmAttrs<Prev> {
    fn as_rec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_rec_mut()
    }
}
impl<Prev: Rec> PushArrayWmmAttrs<Prev> {
    pub fn new(prev: Prev) -> Self {
        Self {
            prev: Some(prev),
            header_offset: None,
            counter: 0,
        }
    }
    pub fn end_array(mut self) -> Prev {
        let mut prev = self.prev.take().unwrap();
        if let Some(header_offset) = &self.header_offset {
            finalize_nested_header(prev.as_rec_mut(), *header_offset);
        }
        prev
    }
    pub fn entry_nested(mut self) -> PushWmmAttrs<Self> {
        let index = self.counter;
        self.counter += 1;
        let header_offset = push_nested_header(self.as_rec_mut(), index);
        PushWmmAttrs {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
}
impl<Prev: Rec> Drop for PushArrayWmmAttrs<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_rec_mut(), *header_offset);
            }
        }
    }
}
impl<Prev: Rec> PushFrequencyAttrs<Prev> {
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
    pub fn push_freq(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 1u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_disabled(mut self, value: ()) -> Self {
        push_header(self.as_rec_mut(), 2u16, 0 as u16);
        self
    }
    pub fn push_no_ir(mut self, value: ()) -> Self {
        push_header(self.as_rec_mut(), 3u16, 0 as u16);
        self
    }
    pub fn push_no_ibss(mut self, value: ()) -> Self {
        push_header(self.as_rec_mut(), 4u16, 0 as u16);
        self
    }
    pub fn push_radar(mut self, value: ()) -> Self {
        push_header(self.as_rec_mut(), 5u16, 0 as u16);
        self
    }
    pub fn push_max_tx_power(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 6u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_dfs_state(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 7u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_dfs_time(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 8u16, value.len() as u16);
        self.as_rec_mut().extend(value);
        self
    }
    pub fn push_no_ht40_minus(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 9u16, value.len() as u16);
        self.as_rec_mut().extend(value);
        self
    }
    pub fn push_no_ht40_plus(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 10u16, value.len() as u16);
        self.as_rec_mut().extend(value);
        self
    }
    pub fn push_no_80mhz(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 11u16, value.len() as u16);
        self.as_rec_mut().extend(value);
        self
    }
    pub fn push_no_160mhz(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 12u16, value.len() as u16);
        self.as_rec_mut().extend(value);
        self
    }
    pub fn push_dfs_cac_time(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 13u16, value.len() as u16);
        self.as_rec_mut().extend(value);
        self
    }
    pub fn push_indoor_only(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 14u16, value.len() as u16);
        self.as_rec_mut().extend(value);
        self
    }
    pub fn push_ir_concurrent(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 15u16, value.len() as u16);
        self.as_rec_mut().extend(value);
        self
    }
    pub fn push_no_20mhz(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 16u16, value.len() as u16);
        self.as_rec_mut().extend(value);
        self
    }
    pub fn push_no_10mhz(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 17u16, value.len() as u16);
        self.as_rec_mut().extend(value);
        self
    }
    pub fn array_wmm(mut self) -> PushArrayWmmAttrs<Self> {
        let header_offset = push_nested_header(self.as_rec_mut(), 18u16);
        PushArrayWmmAttrs {
            prev: Some(self),
            header_offset: Some(header_offset),
            counter: 0,
        }
    }
    pub fn push_no_he(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 19u16, value.len() as u16);
        self.as_rec_mut().extend(value);
        self
    }
    pub fn push_offset(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 20u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_1mhz(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 21u16, value.len() as u16);
        self.as_rec_mut().extend(value);
        self
    }
    pub fn push_2mhz(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 22u16, value.len() as u16);
        self.as_rec_mut().extend(value);
        self
    }
    pub fn push_4mhz(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 23u16, value.len() as u16);
        self.as_rec_mut().extend(value);
        self
    }
    pub fn push_8mhz(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 24u16, value.len() as u16);
        self.as_rec_mut().extend(value);
        self
    }
    pub fn push_16mhz(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 25u16, value.len() as u16);
        self.as_rec_mut().extend(value);
        self
    }
    pub fn push_no_320mhz(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 26u16, value.len() as u16);
        self.as_rec_mut().extend(value);
        self
    }
    pub fn push_no_eht(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 27u16, value.len() as u16);
        self.as_rec_mut().extend(value);
        self
    }
    pub fn push_psd(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 28u16, value.len() as u16);
        self.as_rec_mut().extend(value);
        self
    }
    pub fn push_dfs_concurrent(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 29u16, value.len() as u16);
        self.as_rec_mut().extend(value);
        self
    }
    pub fn push_no_6ghz_vlp_client(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 30u16, value.len() as u16);
        self.as_rec_mut().extend(value);
        self
    }
    pub fn push_no_6ghz_afc_client(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 31u16, value.len() as u16);
        self.as_rec_mut().extend(value);
        self
    }
    pub fn push_can_monitor(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 32u16, value.len() as u16);
        self.as_rec_mut().extend(value);
        self
    }
    pub fn push_allow_6ghz_vlp_ap(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 33u16, value.len() as u16);
        self.as_rec_mut().extend(value);
        self
    }
}
impl<Prev: Rec> Drop for PushFrequencyAttrs<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_rec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushIfaceLimitAttributes<Prev: Rec> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Rec> Rec for PushIfaceLimitAttributes<Prev> {
    fn as_rec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_rec_mut()
    }
}
impl<Prev: Rec> PushIfaceLimitAttributes<Prev> {
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
    pub fn push_max(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 1u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn nested_types(mut self) -> PushSupportedIftypes<Self> {
        let header_offset = push_nested_header(self.as_rec_mut(), 2u16);
        PushSupportedIftypes {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
}
impl<Prev: Rec> Drop for PushIfaceLimitAttributes<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_rec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushIftypeDataAttrs<Prev: Rec> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Rec> Rec for PushIftypeDataAttrs<Prev> {
    fn as_rec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_rec_mut()
    }
}
impl<Prev: Rec> PushIftypeDataAttrs<Prev> {
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
    pub fn push_iftypes(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 1u16, value.len() as u16);
        self.as_rec_mut().extend(value);
        self
    }
    pub fn push_he_cap_mac(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 2u16, value.len() as u16);
        self.as_rec_mut().extend(value);
        self
    }
    pub fn push_he_cap_phy(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 3u16, value.len() as u16);
        self.as_rec_mut().extend(value);
        self
    }
    pub fn push_he_cap_mcs_set(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 4u16, value.len() as u16);
        self.as_rec_mut().extend(value);
        self
    }
    pub fn push_he_cap_ppe(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 5u16, value.len() as u16);
        self.as_rec_mut().extend(value);
        self
    }
    pub fn push_he_6ghz_capa(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 6u16, value.len() as u16);
        self.as_rec_mut().extend(value);
        self
    }
    pub fn push_vendor_elems(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 7u16, value.len() as u16);
        self.as_rec_mut().extend(value);
        self
    }
    pub fn push_eht_cap_mac(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 8u16, value.len() as u16);
        self.as_rec_mut().extend(value);
        self
    }
    pub fn push_eht_cap_phy(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 9u16, value.len() as u16);
        self.as_rec_mut().extend(value);
        self
    }
    pub fn push_eht_cap_mcs_set(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 10u16, value.len() as u16);
        self.as_rec_mut().extend(value);
        self
    }
    pub fn push_eht_cap_ppe(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 11u16, value.len() as u16);
        self.as_rec_mut().extend(value);
        self
    }
}
impl<Prev: Rec> Drop for PushIftypeDataAttrs<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_rec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushIftypeAttrs<Prev: Rec> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Rec> Rec for PushIftypeAttrs<Prev> {
    fn as_rec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_rec_mut()
    }
}
impl<Prev: Rec> PushIftypeAttrs<Prev> {
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
    pub fn nested_unspecified(mut self) -> PushFrameTypeAttrs<Self> {
        let header_offset = push_nested_header(self.as_rec_mut(), 0u16);
        PushFrameTypeAttrs {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
    pub fn nested_adhoc(mut self) -> PushFrameTypeAttrs<Self> {
        let header_offset = push_nested_header(self.as_rec_mut(), 1u16);
        PushFrameTypeAttrs {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
    pub fn nested_station(mut self) -> PushFrameTypeAttrs<Self> {
        let header_offset = push_nested_header(self.as_rec_mut(), 2u16);
        PushFrameTypeAttrs {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
    pub fn nested_ap(mut self) -> PushFrameTypeAttrs<Self> {
        let header_offset = push_nested_header(self.as_rec_mut(), 3u16);
        PushFrameTypeAttrs {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
    pub fn nested_ap_vlan(mut self) -> PushFrameTypeAttrs<Self> {
        let header_offset = push_nested_header(self.as_rec_mut(), 4u16);
        PushFrameTypeAttrs {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
    pub fn nested_wds(mut self) -> PushFrameTypeAttrs<Self> {
        let header_offset = push_nested_header(self.as_rec_mut(), 5u16);
        PushFrameTypeAttrs {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
    pub fn nested_monitor(mut self) -> PushFrameTypeAttrs<Self> {
        let header_offset = push_nested_header(self.as_rec_mut(), 6u16);
        PushFrameTypeAttrs {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
    pub fn nested_mesh_point(mut self) -> PushFrameTypeAttrs<Self> {
        let header_offset = push_nested_header(self.as_rec_mut(), 7u16);
        PushFrameTypeAttrs {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
    pub fn nested_p2p_client(mut self) -> PushFrameTypeAttrs<Self> {
        let header_offset = push_nested_header(self.as_rec_mut(), 8u16);
        PushFrameTypeAttrs {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
    pub fn nested_p2p_go(mut self) -> PushFrameTypeAttrs<Self> {
        let header_offset = push_nested_header(self.as_rec_mut(), 9u16);
        PushFrameTypeAttrs {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
    pub fn nested_p2p_device(mut self) -> PushFrameTypeAttrs<Self> {
        let header_offset = push_nested_header(self.as_rec_mut(), 10u16);
        PushFrameTypeAttrs {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
    pub fn nested_ocb(mut self) -> PushFrameTypeAttrs<Self> {
        let header_offset = push_nested_header(self.as_rec_mut(), 11u16);
        PushFrameTypeAttrs {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
    pub fn nested_nan(mut self) -> PushFrameTypeAttrs<Self> {
        let header_offset = push_nested_header(self.as_rec_mut(), 12u16);
        PushFrameTypeAttrs {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
}
impl<Prev: Rec> Drop for PushIftypeAttrs<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_rec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushSarSpecs<Prev: Rec> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Rec> Rec for PushSarSpecs<Prev> {
    fn as_rec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_rec_mut()
    }
}
impl<Prev: Rec> PushSarSpecs<Prev> {
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
    pub fn push_power(mut self, value: i32) -> Self {
        push_header(self.as_rec_mut(), 1u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_range_index(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 2u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_start_freq(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 3u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_end_freq(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 4u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
}
impl<Prev: Rec> Drop for PushSarSpecs<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_rec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushSupportedIftypes<Prev: Rec> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Rec> Rec for PushSupportedIftypes<Prev> {
    fn as_rec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_rec_mut()
    }
}
impl<Prev: Rec> PushSupportedIftypes<Prev> {
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
    pub fn push_adhoc(mut self, value: ()) -> Self {
        push_header(self.as_rec_mut(), 1u16, 0 as u16);
        self
    }
    pub fn push_station(mut self, value: ()) -> Self {
        push_header(self.as_rec_mut(), 2u16, 0 as u16);
        self
    }
    pub fn push_ap(mut self, value: ()) -> Self {
        push_header(self.as_rec_mut(), 3u16, 0 as u16);
        self
    }
    pub fn push_ap_vlan(mut self, value: ()) -> Self {
        push_header(self.as_rec_mut(), 4u16, 0 as u16);
        self
    }
    pub fn push_wds(mut self, value: ()) -> Self {
        push_header(self.as_rec_mut(), 5u16, 0 as u16);
        self
    }
    pub fn push_monitor(mut self, value: ()) -> Self {
        push_header(self.as_rec_mut(), 6u16, 0 as u16);
        self
    }
    pub fn push_mesh_point(mut self, value: ()) -> Self {
        push_header(self.as_rec_mut(), 7u16, 0 as u16);
        self
    }
    pub fn push_p2p_client(mut self, value: ()) -> Self {
        push_header(self.as_rec_mut(), 8u16, 0 as u16);
        self
    }
    pub fn push_p2p_go(mut self, value: ()) -> Self {
        push_header(self.as_rec_mut(), 9u16, 0 as u16);
        self
    }
    pub fn push_p2p_device(mut self, value: ()) -> Self {
        push_header(self.as_rec_mut(), 10u16, 0 as u16);
        self
    }
    pub fn push_ocb(mut self, value: ()) -> Self {
        push_header(self.as_rec_mut(), 11u16, 0 as u16);
        self
    }
    pub fn push_nan(mut self, value: ()) -> Self {
        push_header(self.as_rec_mut(), 12u16, 0 as u16);
        self
    }
}
impl<Prev: Rec> Drop for PushSupportedIftypes<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_rec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushTxqStatsAttrs<Prev: Rec> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Rec> Rec for PushTxqStatsAttrs<Prev> {
    fn as_rec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_rec_mut()
    }
}
impl<Prev: Rec> PushTxqStatsAttrs<Prev> {
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
    pub fn push_backlog_bytes(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 1u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_backlog_packets(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 2u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_flows(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 3u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_drops(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 4u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_ecn_marks(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 5u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_overlimit(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 6u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_overmemory(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 7u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_collisions(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 8u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_tx_bytes(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 9u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_tx_packets(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 10u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_max_flows(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 11u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
}
impl<Prev: Rec> Drop for PushTxqStatsAttrs<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_rec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushWmmAttrs<Prev: Rec> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Rec> Rec for PushWmmAttrs<Prev> {
    fn as_rec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_rec_mut()
    }
}
impl<Prev: Rec> PushWmmAttrs<Prev> {
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
    pub fn push_cw_min(mut self, value: u16) -> Self {
        push_header(self.as_rec_mut(), 1u16, 2 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_cw_max(mut self, value: u16) -> Self {
        push_header(self.as_rec_mut(), 2u16, 2 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_aifsn(mut self, value: u8) -> Self {
        push_header(self.as_rec_mut(), 3u16, 1 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_txop(mut self, value: u16) -> Self {
        push_header(self.as_rec_mut(), 4u16, 2 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
}
impl<Prev: Rec> Drop for PushWmmAttrs<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_rec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushWowlanTriggersAttrs<Prev: Rec> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Rec> Rec for PushWowlanTriggersAttrs<Prev> {
    fn as_rec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_rec_mut()
    }
}
impl<Prev: Rec> PushWowlanTriggersAttrs<Prev> {
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
    pub fn push_any(mut self, value: ()) -> Self {
        push_header(self.as_rec_mut(), 1u16, 0 as u16);
        self
    }
    pub fn push_disconnect(mut self, value: ()) -> Self {
        push_header(self.as_rec_mut(), 2u16, 0 as u16);
        self
    }
    pub fn push_magic_pkt(mut self, value: ()) -> Self {
        push_header(self.as_rec_mut(), 3u16, 0 as u16);
        self
    }
    pub fn push_pkt_pattern(mut self, value: ()) -> Self {
        push_header(self.as_rec_mut(), 4u16, 0 as u16);
        self
    }
    pub fn push_gtk_rekey_supported(mut self, value: ()) -> Self {
        push_header(self.as_rec_mut(), 5u16, 0 as u16);
        self
    }
    pub fn push_gtk_rekey_failure(mut self, value: ()) -> Self {
        push_header(self.as_rec_mut(), 6u16, 0 as u16);
        self
    }
    pub fn push_eap_ident_request(mut self, value: ()) -> Self {
        push_header(self.as_rec_mut(), 7u16, 0 as u16);
        self
    }
    pub fn push_4way_handshake(mut self, value: ()) -> Self {
        push_header(self.as_rec_mut(), 8u16, 0 as u16);
        self
    }
    pub fn push_rfkill_release(mut self, value: ()) -> Self {
        push_header(self.as_rec_mut(), 9u16, 0 as u16);
        self
    }
    pub fn push_wakeup_pkt_80211(mut self, value: ()) -> Self {
        push_header(self.as_rec_mut(), 10u16, 0 as u16);
        self
    }
    pub fn push_wakeup_pkt_80211_len(mut self, value: ()) -> Self {
        push_header(self.as_rec_mut(), 11u16, 0 as u16);
        self
    }
    pub fn push_wakeup_pkt_8023(mut self, value: ()) -> Self {
        push_header(self.as_rec_mut(), 12u16, 0 as u16);
        self
    }
    pub fn push_wakeup_pkt_8023_len(mut self, value: ()) -> Self {
        push_header(self.as_rec_mut(), 13u16, 0 as u16);
        self
    }
    pub fn push_tcp_connection(mut self, value: ()) -> Self {
        push_header(self.as_rec_mut(), 14u16, 0 as u16);
        self
    }
    pub fn push_wakeup_tcp_match(mut self, value: ()) -> Self {
        push_header(self.as_rec_mut(), 15u16, 0 as u16);
        self
    }
    pub fn push_wakeup_tcp_connlost(mut self, value: ()) -> Self {
        push_header(self.as_rec_mut(), 16u16, 0 as u16);
        self
    }
    pub fn push_wakeup_tcp_nomoretokens(mut self, value: ()) -> Self {
        push_header(self.as_rec_mut(), 17u16, 0 as u16);
        self
    }
    pub fn push_net_detect(mut self, value: ()) -> Self {
        push_header(self.as_rec_mut(), 18u16, 0 as u16);
        self
    }
    pub fn push_net_detect_results(mut self, value: ()) -> Self {
        push_header(self.as_rec_mut(), 19u16, 0 as u16);
        self
    }
    pub fn push_unprotected_deauth_disassoc(mut self, value: ()) -> Self {
        push_header(self.as_rec_mut(), 20u16, 0 as u16);
        self
    }
}
impl<Prev: Rec> Drop for PushWowlanTriggersAttrs<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_rec_mut(), *header_offset);
            }
        }
    }
}
#[derive(Clone)]
pub struct PushStaFlagUpdate {
    pub(crate) buf: [u8; 8usize],
}
#[doc = "Create zero-initialized struct"]
impl Default for PushStaFlagUpdate {
    fn default() -> Self {
        Self { buf: [0u8; 8usize] }
    }
}
impl PushStaFlagUpdate {
    #[doc = "Create zero-initialized struct"]
    pub fn new() -> Self {
        Default::default()
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
        8usize
    }
    pub fn mask(&self) -> u32 {
        parse_u32(&self.buf[0usize..4usize]).unwrap()
    }
    pub fn set_mask(&mut self, value: u32) {
        self.buf[0usize..4usize].copy_from_slice(&value.to_ne_bytes())
    }
    pub fn set(&self) -> u32 {
        parse_u32(&self.buf[4usize..8usize]).unwrap()
    }
    pub fn set_set(&mut self, value: u32) {
        self.buf[4usize..8usize].copy_from_slice(&value.to_ne_bytes())
    }
}
impl std::fmt::Debug for PushStaFlagUpdate {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt.debug_struct("StaFlagUpdate")
            .field("mask", &self.mask())
            .field("set", &self.set())
            .finish()
    }
}
#[doc = "Get information about a wiphy or dump a list of all wiphys. Requests to\ndump get-wiphy should unconditionally include the split-wiphy-dump flag\nin the request.\n"]
pub struct PushOpGetWiphyDumpRequest<Prev: Rec> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Rec> Rec for PushOpGetWiphyDumpRequest<Prev> {
    fn as_rec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_rec_mut()
    }
}
impl<Prev: Rec> PushOpGetWiphyDumpRequest<Prev> {
    pub fn new(mut prev: Prev) -> Self {
        Self::write_header(&mut prev);
        Self::new_without_header(prev)
    }
    fn new_without_header(prev: Prev) -> Self {
        Self {
            prev: Some(prev),
            header_offset: None,
        }
    }
    fn write_header(prev: &mut Prev) {
        let mut header = PushBuiltinNfgenmsg::new();
        header.set_cmd(1u8);
        header.set_version(1u8);
        prev.as_rec_mut().extend(header.as_slice());
    }
    pub fn end_nested(mut self) -> Prev {
        let mut prev = self.prev.take().unwrap();
        if let Some(header_offset) = &self.header_offset {
            finalize_nested_header(prev.as_rec_mut(), *header_offset);
        }
        prev
    }
    pub fn push_wiphy(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 1u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_ifindex(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 3u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_wdev(mut self, value: u64) -> Self {
        push_header(self.as_rec_mut(), 152u16, 8 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_split_wiphy_dump(mut self, value: ()) -> Self {
        push_header(self.as_rec_mut(), 173u16, 0 as u16);
        self
    }
}
impl<Prev: Rec> Drop for PushOpGetWiphyDumpRequest<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_rec_mut(), *header_offset);
            }
        }
    }
}
#[doc = "Get information about a wiphy or dump a list of all wiphys. Requests to\ndump get-wiphy should unconditionally include the split-wiphy-dump flag\nin the request.\n"]
#[derive(Clone)]
pub enum OpGetWiphyDumpRequest {
    Wiphy(u32),
    Ifindex(u32),
    Wdev(u64),
    SplitWiphyDump(()),
}
impl<'a> IterableOpGetWiphyDumpRequest<'a> {
    pub fn get_wiphy(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetWiphyDumpRequest::Wiphy(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpGetWiphyDumpRequest",
            "Wiphy",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_ifindex(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetWiphyDumpRequest::Ifindex(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpGetWiphyDumpRequest",
            "Ifindex",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_wdev(&self) -> Result<u64, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetWiphyDumpRequest::Wdev(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpGetWiphyDumpRequest",
            "Wdev",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_split_wiphy_dump(&self) -> Result<(), ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetWiphyDumpRequest::SplitWiphyDump(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpGetWiphyDumpRequest",
            "SplitWiphyDump",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl OpGetWiphyDumpRequest {
    pub fn new(buf: &'_ [u8]) -> IterableOpGetWiphyDumpRequest<'_> {
        let (_header, attrs) = buf.split_at(buf.len().min(PushBuiltinNfgenmsg::len()));
        IterableOpGetWiphyDumpRequest::with_loc(attrs, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        Nl80211Attrs::attr_from_type(r#type)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableOpGetWiphyDumpRequest<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableOpGetWiphyDumpRequest<'a> {
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
impl<'a> Iterator for IterableOpGetWiphyDumpRequest<'a> {
    type Item = Result<OpGetWiphyDumpRequest, ErrorContext>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.buf.len() == self.pos {
            return None;
        }
        let pos = self.pos;
        let mut r#type = None;
        while let Some((header, next)) = chop_header(self.buf, &mut self.pos) {
            r#type = Some(header.r#type);
            let res = match header.r#type {
                1u16 => OpGetWiphyDumpRequest::Wiphy({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                3u16 => OpGetWiphyDumpRequest::Ifindex({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                152u16 => OpGetWiphyDumpRequest::Wdev({
                    let res = parse_u64(next);
                    let Some(val) = res else { break };
                    val
                }),
                173u16 => OpGetWiphyDumpRequest::SplitWiphyDump(()),
                n => {
                    if cfg!(any(test, feature = "deny-unknown-attrs")) {
                        break;
                    } else {
                        continue;
                    }
                }
            };
            return Some(Ok(res));
        }
        Some(Err(ErrorContext::new(
            "OpGetWiphyDumpRequest",
            r#type.and_then(|t| OpGetWiphyDumpRequest::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl std::fmt::Debug for IterableOpGetWiphyDumpRequest<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("OpGetWiphyDumpRequest");
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
                OpGetWiphyDumpRequest::Wiphy(val) => fmt.field("Wiphy", &val),
                OpGetWiphyDumpRequest::Ifindex(val) => fmt.field("Ifindex", &val),
                OpGetWiphyDumpRequest::Wdev(val) => fmt.field("Wdev", &val),
                OpGetWiphyDumpRequest::SplitWiphyDump(val) => fmt.field("SplitWiphyDump", &val),
            };
        }
        fmt.finish()
    }
}
impl IterableOpGetWiphyDumpRequest<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if cur == offset + PushBuiltinNfgenmsg::len() {
            stack.push(("OpGetWiphyDumpRequest", offset));
            return (
                stack,
                missing_type.and_then(|t| OpGetWiphyDumpRequest::attr_from_type(t)),
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
                OpGetWiphyDumpRequest::Wiphy(val) => {
                    if last_off == offset {
                        stack.push(("Wiphy", last_off));
                        break;
                    }
                }
                OpGetWiphyDumpRequest::Ifindex(val) => {
                    if last_off == offset {
                        stack.push(("Ifindex", last_off));
                        break;
                    }
                }
                OpGetWiphyDumpRequest::Wdev(val) => {
                    if last_off == offset {
                        stack.push(("Wdev", last_off));
                        break;
                    }
                }
                OpGetWiphyDumpRequest::SplitWiphyDump(val) => {
                    if last_off == offset {
                        stack.push(("SplitWiphyDump", last_off));
                        break;
                    }
                }
                _ => {}
            };
            last_off = cur + attrs.pos;
        }
        if !stack.is_empty() {
            stack.push(("OpGetWiphyDumpRequest", cur));
        }
        (stack, None)
    }
}
#[doc = "Get information about a wiphy or dump a list of all wiphys. Requests to\ndump get-wiphy should unconditionally include the split-wiphy-dump flag\nin the request.\n"]
pub struct PushOpGetWiphyDumpReply<Prev: Rec> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Rec> Rec for PushOpGetWiphyDumpReply<Prev> {
    fn as_rec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_rec_mut()
    }
}
impl<Prev: Rec> PushOpGetWiphyDumpReply<Prev> {
    pub fn new(mut prev: Prev) -> Self {
        Self::write_header(&mut prev);
        Self::new_without_header(prev)
    }
    fn new_without_header(prev: Prev) -> Self {
        Self {
            prev: Some(prev),
            header_offset: None,
        }
    }
    fn write_header(prev: &mut Prev) {
        let mut header = PushBuiltinNfgenmsg::new();
        header.set_cmd(3u8);
        header.set_version(1u8);
        prev.as_rec_mut().extend(header.as_slice());
    }
    pub fn end_nested(mut self) -> Prev {
        let mut prev = self.prev.take().unwrap();
        if let Some(header_offset) = &self.header_offset {
            finalize_nested_header(prev.as_rec_mut(), *header_offset);
        }
        prev
    }
    pub fn push_wiphy(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 1u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_wiphy_name(mut self, value: &CStr) -> Self {
        push_header(
            self.as_rec_mut(),
            2u16,
            value.to_bytes_with_nul().len() as u16,
        );
        self.as_rec_mut().extend(value.to_bytes_with_nul());
        self
    }
    pub fn push_wiphy_name_bytes(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 2u16, (value.len() + 1) as u16);
        self.as_rec_mut().extend(value);
        self.as_rec_mut().push(0);
        self
    }
    pub fn push_mac(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 6u16, value.len() as u16);
        self.as_rec_mut().extend(value);
        self
    }
    pub fn nested_wiphy_bands(mut self) -> PushWiphyBands<Self> {
        let header_offset = push_nested_header(self.as_rec_mut(), 22u16);
        PushWiphyBands {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
    pub fn nested_supported_iftypes(mut self) -> PushSupportedIftypes<Self> {
        let header_offset = push_nested_header(self.as_rec_mut(), 32u16);
        PushSupportedIftypes {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
    pub fn push_max_num_scan_ssids(mut self, value: u8) -> Self {
        push_header(self.as_rec_mut(), 43u16, 1 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_generation(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 46u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_max_scan_ie_len(mut self, value: u16) -> Self {
        push_header(self.as_rec_mut(), 55u16, 2 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_cipher_suites(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 56u16, value.len() as u16);
        self.as_rec_mut().extend(value);
        self
    }
    pub fn push_wiphy_retry_short(mut self, value: u8) -> Self {
        push_header(self.as_rec_mut(), 60u16, 1 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_wiphy_retry_long(mut self, value: u8) -> Self {
        push_header(self.as_rec_mut(), 61u16, 1 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_wiphy_frag_threshold(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 62u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_wiphy_rts_threshold(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 63u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_max_num_pmkids(mut self, value: u8) -> Self {
        push_header(self.as_rec_mut(), 85u16, 1 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_wiphy_coverage_class(mut self, value: u8) -> Self {
        push_header(self.as_rec_mut(), 88u16, 1 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn nested_tx_frame_types(mut self) -> PushIftypeAttrs<Self> {
        let header_offset = push_nested_header(self.as_rec_mut(), 98u16);
        PushIftypeAttrs {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
    pub fn nested_rx_frame_types(mut self) -> PushIftypeAttrs<Self> {
        let header_offset = push_nested_header(self.as_rec_mut(), 99u16);
        PushIftypeAttrs {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
    pub fn push_control_port_ethertype(mut self, value: ()) -> Self {
        push_header(self.as_rec_mut(), 101u16, 0 as u16);
        self
    }
    pub fn push_wiphy_antenna_tx(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 104u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_wiphy_antenna_rx(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 105u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_offchannel_tx_ok(mut self, value: ()) -> Self {
        push_header(self.as_rec_mut(), 107u16, 0 as u16);
        self
    }
    pub fn push_max_remain_on_channel_duration(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 110u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_wiphy_antenna_avail_tx(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 112u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_wiphy_antenna_avail_rx(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 113u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn nested_wowlan_triggers_supported(mut self) -> PushWowlanTriggersAttrs<Self> {
        let header_offset = push_nested_header(self.as_rec_mut(), 117u16);
        PushWowlanTriggersAttrs {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
    pub fn push_interface_combinations(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 119u16, value.len() as u16);
        self.as_rec_mut().extend(value);
        self
    }
    pub fn nested_software_iftypes(mut self) -> PushSupportedIftypes<Self> {
        let header_offset = push_nested_header(self.as_rec_mut(), 120u16);
        PushSupportedIftypes {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
    pub fn push_max_num_sched_scan_ssids(mut self, value: u8) -> Self {
        push_header(self.as_rec_mut(), 122u16, 1 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_max_sched_scan_ie_len(mut self, value: u16) -> Self {
        push_header(self.as_rec_mut(), 123u16, 2 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_support_ap_uapsd(mut self, value: ()) -> Self {
        push_header(self.as_rec_mut(), 129u16, 0 as u16);
        self
    }
    pub fn push_max_match_sets(mut self, value: u8) -> Self {
        push_header(self.as_rec_mut(), 132u16, 1 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_tdls_support(mut self, value: ()) -> Self {
        push_header(self.as_rec_mut(), 138u16, 0 as u16);
        self
    }
    pub fn push_tdls_external_setup(mut self, value: ()) -> Self {
        push_header(self.as_rec_mut(), 139u16, 0 as u16);
        self
    }
    #[doc = "Associated type: \"FeatureFlags\" (1 bit per enumeration)"]
    pub fn push_feature_flags(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 142u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_ht_capability_mask(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 147u16, value.len() as u16);
        self.as_rec_mut().extend(value);
        self
    }
    pub fn push_ext_capa(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 168u16, value.len() as u16);
        self.as_rec_mut().extend(value);
        self
    }
    pub fn push_ext_capa_mask(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 169u16, value.len() as u16);
        self.as_rec_mut().extend(value);
        self
    }
    pub fn push_vht_capability_mask(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 175u16, value.len() as u16);
        self.as_rec_mut().extend(value);
        self
    }
    pub fn push_max_csa_counters(mut self, value: u8) -> Self {
        push_header(self.as_rec_mut(), 205u16, 1 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_ext_features(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 216u16, value.len() as u16);
        self.as_rec_mut().extend(value);
        self
    }
    pub fn push_max_num_sched_scan_plans(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 221u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_max_scan_plan_interval(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 222u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_max_scan_plan_iterations(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 223u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_bands(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 238u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_sched_scan_max_reqs(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 255u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn nested_txq_stats(mut self) -> PushTxqStatsAttrs<Self> {
        let header_offset = push_nested_header(self.as_rec_mut(), 264u16);
        PushTxqStatsAttrs {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
    pub fn push_txq_limit(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 265u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_txq_memory_limit(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 266u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_txq_quantum(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 267u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_sar_spec(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 299u16, value.len() as u16);
        self.as_rec_mut().extend(value);
        self
    }
    pub fn push_max_num_akm_suites(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 315u16, value.len() as u16);
        self.as_rec_mut().extend(value);
        self
    }
}
impl<Prev: Rec> Drop for PushOpGetWiphyDumpReply<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_rec_mut(), *header_offset);
            }
        }
    }
}
#[doc = "Get information about a wiphy or dump a list of all wiphys. Requests to\ndump get-wiphy should unconditionally include the split-wiphy-dump flag\nin the request.\n"]
#[derive(Clone)]
pub enum OpGetWiphyDumpReply<'a> {
    Wiphy(u32),
    WiphyName(&'a CStr),
    Mac(&'a [u8]),
    WiphyBands(IterableWiphyBands<'a>),
    SupportedIftypes(IterableSupportedIftypes<'a>),
    MaxNumScanSsids(u8),
    Generation(u32),
    MaxScanIeLen(u16),
    CipherSuites(&'a [u8]),
    WiphyRetryShort(u8),
    WiphyRetryLong(u8),
    WiphyFragThreshold(u32),
    WiphyRtsThreshold(u32),
    MaxNumPmkids(u8),
    WiphyCoverageClass(u8),
    TxFrameTypes(IterableIftypeAttrs<'a>),
    RxFrameTypes(IterableIftypeAttrs<'a>),
    ControlPortEthertype(()),
    WiphyAntennaTx(u32),
    WiphyAntennaRx(u32),
    OffchannelTxOk(()),
    MaxRemainOnChannelDuration(u32),
    WiphyAntennaAvailTx(u32),
    WiphyAntennaAvailRx(u32),
    WowlanTriggersSupported(IterableWowlanTriggersAttrs<'a>),
    InterfaceCombinations(&'a [u8]),
    SoftwareIftypes(IterableSupportedIftypes<'a>),
    MaxNumSchedScanSsids(u8),
    MaxSchedScanIeLen(u16),
    SupportApUapsd(()),
    MaxMatchSets(u8),
    TdlsSupport(()),
    TdlsExternalSetup(()),
    #[doc = "Associated type: \"FeatureFlags\" (1 bit per enumeration)"]
    FeatureFlags(u32),
    HtCapabilityMask(&'a [u8]),
    ExtCapa(&'a [u8]),
    ExtCapaMask(&'a [u8]),
    VhtCapabilityMask(&'a [u8]),
    MaxCsaCounters(u8),
    ExtFeatures(&'a [u8]),
    MaxNumSchedScanPlans(u32),
    MaxScanPlanInterval(u32),
    MaxScanPlanIterations(u32),
    Bands(u32),
    SchedScanMaxReqs(u32),
    TxqStats(IterableTxqStatsAttrs<'a>),
    TxqLimit(u32),
    TxqMemoryLimit(u32),
    TxqQuantum(u32),
    SarSpec(&'a [u8]),
    MaxNumAkmSuites(&'a [u8]),
}
impl<'a> IterableOpGetWiphyDumpReply<'a> {
    pub fn get_wiphy(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetWiphyDumpReply::Wiphy(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpGetWiphyDumpReply",
            "Wiphy",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_wiphy_name(&self) -> Result<&'a CStr, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetWiphyDumpReply::WiphyName(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpGetWiphyDumpReply",
            "WiphyName",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_mac(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetWiphyDumpReply::Mac(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpGetWiphyDumpReply",
            "Mac",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_wiphy_bands(&self) -> Result<IterableWiphyBands<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetWiphyDumpReply::WiphyBands(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpGetWiphyDumpReply",
            "WiphyBands",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_supported_iftypes(&self) -> Result<IterableSupportedIftypes<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetWiphyDumpReply::SupportedIftypes(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpGetWiphyDumpReply",
            "SupportedIftypes",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_max_num_scan_ssids(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetWiphyDumpReply::MaxNumScanSsids(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpGetWiphyDumpReply",
            "MaxNumScanSsids",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_generation(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetWiphyDumpReply::Generation(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpGetWiphyDumpReply",
            "Generation",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_max_scan_ie_len(&self) -> Result<u16, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetWiphyDumpReply::MaxScanIeLen(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpGetWiphyDumpReply",
            "MaxScanIeLen",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_cipher_suites(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetWiphyDumpReply::CipherSuites(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpGetWiphyDumpReply",
            "CipherSuites",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_wiphy_retry_short(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetWiphyDumpReply::WiphyRetryShort(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpGetWiphyDumpReply",
            "WiphyRetryShort",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_wiphy_retry_long(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetWiphyDumpReply::WiphyRetryLong(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpGetWiphyDumpReply",
            "WiphyRetryLong",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_wiphy_frag_threshold(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetWiphyDumpReply::WiphyFragThreshold(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpGetWiphyDumpReply",
            "WiphyFragThreshold",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_wiphy_rts_threshold(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetWiphyDumpReply::WiphyRtsThreshold(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpGetWiphyDumpReply",
            "WiphyRtsThreshold",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_max_num_pmkids(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetWiphyDumpReply::MaxNumPmkids(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpGetWiphyDumpReply",
            "MaxNumPmkids",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_wiphy_coverage_class(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetWiphyDumpReply::WiphyCoverageClass(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpGetWiphyDumpReply",
            "WiphyCoverageClass",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_tx_frame_types(&self) -> Result<IterableIftypeAttrs<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetWiphyDumpReply::TxFrameTypes(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpGetWiphyDumpReply",
            "TxFrameTypes",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_rx_frame_types(&self) -> Result<IterableIftypeAttrs<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetWiphyDumpReply::RxFrameTypes(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpGetWiphyDumpReply",
            "RxFrameTypes",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_control_port_ethertype(&self) -> Result<(), ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetWiphyDumpReply::ControlPortEthertype(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpGetWiphyDumpReply",
            "ControlPortEthertype",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_wiphy_antenna_tx(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetWiphyDumpReply::WiphyAntennaTx(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpGetWiphyDumpReply",
            "WiphyAntennaTx",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_wiphy_antenna_rx(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetWiphyDumpReply::WiphyAntennaRx(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpGetWiphyDumpReply",
            "WiphyAntennaRx",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_offchannel_tx_ok(&self) -> Result<(), ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetWiphyDumpReply::OffchannelTxOk(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpGetWiphyDumpReply",
            "OffchannelTxOk",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_max_remain_on_channel_duration(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetWiphyDumpReply::MaxRemainOnChannelDuration(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpGetWiphyDumpReply",
            "MaxRemainOnChannelDuration",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_wiphy_antenna_avail_tx(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetWiphyDumpReply::WiphyAntennaAvailTx(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpGetWiphyDumpReply",
            "WiphyAntennaAvailTx",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_wiphy_antenna_avail_rx(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetWiphyDumpReply::WiphyAntennaAvailRx(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpGetWiphyDumpReply",
            "WiphyAntennaAvailRx",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_wowlan_triggers_supported(
        &self,
    ) -> Result<IterableWowlanTriggersAttrs<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetWiphyDumpReply::WowlanTriggersSupported(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpGetWiphyDumpReply",
            "WowlanTriggersSupported",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_interface_combinations(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetWiphyDumpReply::InterfaceCombinations(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpGetWiphyDumpReply",
            "InterfaceCombinations",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_software_iftypes(&self) -> Result<IterableSupportedIftypes<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetWiphyDumpReply::SoftwareIftypes(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpGetWiphyDumpReply",
            "SoftwareIftypes",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_max_num_sched_scan_ssids(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetWiphyDumpReply::MaxNumSchedScanSsids(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpGetWiphyDumpReply",
            "MaxNumSchedScanSsids",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_max_sched_scan_ie_len(&self) -> Result<u16, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetWiphyDumpReply::MaxSchedScanIeLen(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpGetWiphyDumpReply",
            "MaxSchedScanIeLen",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_support_ap_uapsd(&self) -> Result<(), ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetWiphyDumpReply::SupportApUapsd(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpGetWiphyDumpReply",
            "SupportApUapsd",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_max_match_sets(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetWiphyDumpReply::MaxMatchSets(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpGetWiphyDumpReply",
            "MaxMatchSets",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_tdls_support(&self) -> Result<(), ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetWiphyDumpReply::TdlsSupport(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpGetWiphyDumpReply",
            "TdlsSupport",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_tdls_external_setup(&self) -> Result<(), ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetWiphyDumpReply::TdlsExternalSetup(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpGetWiphyDumpReply",
            "TdlsExternalSetup",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Associated type: \"FeatureFlags\" (1 bit per enumeration)"]
    pub fn get_feature_flags(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetWiphyDumpReply::FeatureFlags(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpGetWiphyDumpReply",
            "FeatureFlags",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_ht_capability_mask(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetWiphyDumpReply::HtCapabilityMask(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpGetWiphyDumpReply",
            "HtCapabilityMask",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_ext_capa(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetWiphyDumpReply::ExtCapa(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpGetWiphyDumpReply",
            "ExtCapa",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_ext_capa_mask(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetWiphyDumpReply::ExtCapaMask(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpGetWiphyDumpReply",
            "ExtCapaMask",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_vht_capability_mask(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetWiphyDumpReply::VhtCapabilityMask(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpGetWiphyDumpReply",
            "VhtCapabilityMask",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_max_csa_counters(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetWiphyDumpReply::MaxCsaCounters(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpGetWiphyDumpReply",
            "MaxCsaCounters",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_ext_features(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetWiphyDumpReply::ExtFeatures(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpGetWiphyDumpReply",
            "ExtFeatures",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_max_num_sched_scan_plans(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetWiphyDumpReply::MaxNumSchedScanPlans(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpGetWiphyDumpReply",
            "MaxNumSchedScanPlans",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_max_scan_plan_interval(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetWiphyDumpReply::MaxScanPlanInterval(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpGetWiphyDumpReply",
            "MaxScanPlanInterval",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_max_scan_plan_iterations(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetWiphyDumpReply::MaxScanPlanIterations(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpGetWiphyDumpReply",
            "MaxScanPlanIterations",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_bands(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetWiphyDumpReply::Bands(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpGetWiphyDumpReply",
            "Bands",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_sched_scan_max_reqs(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetWiphyDumpReply::SchedScanMaxReqs(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpGetWiphyDumpReply",
            "SchedScanMaxReqs",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_txq_stats(&self) -> Result<IterableTxqStatsAttrs<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetWiphyDumpReply::TxqStats(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpGetWiphyDumpReply",
            "TxqStats",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_txq_limit(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetWiphyDumpReply::TxqLimit(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpGetWiphyDumpReply",
            "TxqLimit",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_txq_memory_limit(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetWiphyDumpReply::TxqMemoryLimit(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpGetWiphyDumpReply",
            "TxqMemoryLimit",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_txq_quantum(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetWiphyDumpReply::TxqQuantum(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpGetWiphyDumpReply",
            "TxqQuantum",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_sar_spec(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetWiphyDumpReply::SarSpec(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpGetWiphyDumpReply",
            "SarSpec",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_max_num_akm_suites(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetWiphyDumpReply::MaxNumAkmSuites(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpGetWiphyDumpReply",
            "MaxNumAkmSuites",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl<'a> OpGetWiphyDumpReply<'a> {
    pub fn new(buf: &'a [u8]) -> IterableOpGetWiphyDumpReply<'a> {
        let (_header, attrs) = buf.split_at(buf.len().min(PushBuiltinNfgenmsg::len()));
        IterableOpGetWiphyDumpReply::with_loc(attrs, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        Nl80211Attrs::attr_from_type(r#type)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableOpGetWiphyDumpReply<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableOpGetWiphyDumpReply<'a> {
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
impl<'a> Iterator for IterableOpGetWiphyDumpReply<'a> {
    type Item = Result<OpGetWiphyDumpReply<'a>, ErrorContext>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.buf.len() == self.pos {
            return None;
        }
        let pos = self.pos;
        let mut r#type = None;
        while let Some((header, next)) = chop_header(self.buf, &mut self.pos) {
            r#type = Some(header.r#type);
            let res = match header.r#type {
                1u16 => OpGetWiphyDumpReply::Wiphy({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                2u16 => OpGetWiphyDumpReply::WiphyName({
                    let res = CStr::from_bytes_with_nul(next).ok();
                    let Some(val) = res else { break };
                    val
                }),
                6u16 => OpGetWiphyDumpReply::Mac({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                22u16 => OpGetWiphyDumpReply::WiphyBands({
                    let res = Some(IterableWiphyBands::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                32u16 => OpGetWiphyDumpReply::SupportedIftypes({
                    let res = Some(IterableSupportedIftypes::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                43u16 => OpGetWiphyDumpReply::MaxNumScanSsids({
                    let res = parse_u8(next);
                    let Some(val) = res else { break };
                    val
                }),
                46u16 => OpGetWiphyDumpReply::Generation({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                55u16 => OpGetWiphyDumpReply::MaxScanIeLen({
                    let res = parse_u16(next);
                    let Some(val) = res else { break };
                    val
                }),
                56u16 => OpGetWiphyDumpReply::CipherSuites({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                60u16 => OpGetWiphyDumpReply::WiphyRetryShort({
                    let res = parse_u8(next);
                    let Some(val) = res else { break };
                    val
                }),
                61u16 => OpGetWiphyDumpReply::WiphyRetryLong({
                    let res = parse_u8(next);
                    let Some(val) = res else { break };
                    val
                }),
                62u16 => OpGetWiphyDumpReply::WiphyFragThreshold({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                63u16 => OpGetWiphyDumpReply::WiphyRtsThreshold({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                85u16 => OpGetWiphyDumpReply::MaxNumPmkids({
                    let res = parse_u8(next);
                    let Some(val) = res else { break };
                    val
                }),
                88u16 => OpGetWiphyDumpReply::WiphyCoverageClass({
                    let res = parse_u8(next);
                    let Some(val) = res else { break };
                    val
                }),
                98u16 => OpGetWiphyDumpReply::TxFrameTypes({
                    let res = Some(IterableIftypeAttrs::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                99u16 => OpGetWiphyDumpReply::RxFrameTypes({
                    let res = Some(IterableIftypeAttrs::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                101u16 => OpGetWiphyDumpReply::ControlPortEthertype(()),
                104u16 => OpGetWiphyDumpReply::WiphyAntennaTx({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                105u16 => OpGetWiphyDumpReply::WiphyAntennaRx({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                107u16 => OpGetWiphyDumpReply::OffchannelTxOk(()),
                110u16 => OpGetWiphyDumpReply::MaxRemainOnChannelDuration({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                112u16 => OpGetWiphyDumpReply::WiphyAntennaAvailTx({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                113u16 => OpGetWiphyDumpReply::WiphyAntennaAvailRx({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                117u16 => OpGetWiphyDumpReply::WowlanTriggersSupported({
                    let res = Some(IterableWowlanTriggersAttrs::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                119u16 => OpGetWiphyDumpReply::InterfaceCombinations({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                120u16 => OpGetWiphyDumpReply::SoftwareIftypes({
                    let res = Some(IterableSupportedIftypes::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                122u16 => OpGetWiphyDumpReply::MaxNumSchedScanSsids({
                    let res = parse_u8(next);
                    let Some(val) = res else { break };
                    val
                }),
                123u16 => OpGetWiphyDumpReply::MaxSchedScanIeLen({
                    let res = parse_u16(next);
                    let Some(val) = res else { break };
                    val
                }),
                129u16 => OpGetWiphyDumpReply::SupportApUapsd(()),
                132u16 => OpGetWiphyDumpReply::MaxMatchSets({
                    let res = parse_u8(next);
                    let Some(val) = res else { break };
                    val
                }),
                138u16 => OpGetWiphyDumpReply::TdlsSupport(()),
                139u16 => OpGetWiphyDumpReply::TdlsExternalSetup(()),
                142u16 => OpGetWiphyDumpReply::FeatureFlags({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                147u16 => OpGetWiphyDumpReply::HtCapabilityMask({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                168u16 => OpGetWiphyDumpReply::ExtCapa({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                169u16 => OpGetWiphyDumpReply::ExtCapaMask({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                175u16 => OpGetWiphyDumpReply::VhtCapabilityMask({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                205u16 => OpGetWiphyDumpReply::MaxCsaCounters({
                    let res = parse_u8(next);
                    let Some(val) = res else { break };
                    val
                }),
                216u16 => OpGetWiphyDumpReply::ExtFeatures({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                221u16 => OpGetWiphyDumpReply::MaxNumSchedScanPlans({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                222u16 => OpGetWiphyDumpReply::MaxScanPlanInterval({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                223u16 => OpGetWiphyDumpReply::MaxScanPlanIterations({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                238u16 => OpGetWiphyDumpReply::Bands({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                255u16 => OpGetWiphyDumpReply::SchedScanMaxReqs({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                264u16 => OpGetWiphyDumpReply::TxqStats({
                    let res = Some(IterableTxqStatsAttrs::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                265u16 => OpGetWiphyDumpReply::TxqLimit({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                266u16 => OpGetWiphyDumpReply::TxqMemoryLimit({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                267u16 => OpGetWiphyDumpReply::TxqQuantum({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                299u16 => OpGetWiphyDumpReply::SarSpec({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                315u16 => OpGetWiphyDumpReply::MaxNumAkmSuites({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                n => {
                    if cfg!(any(test, feature = "deny-unknown-attrs")) {
                        break;
                    } else {
                        continue;
                    }
                }
            };
            return Some(Ok(res));
        }
        Some(Err(ErrorContext::new(
            "OpGetWiphyDumpReply",
            r#type.and_then(|t| OpGetWiphyDumpReply::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl<'a> std::fmt::Debug for IterableOpGetWiphyDumpReply<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("OpGetWiphyDumpReply");
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
                OpGetWiphyDumpReply::Wiphy(val) => fmt.field("Wiphy", &val),
                OpGetWiphyDumpReply::WiphyName(val) => fmt.field("WiphyName", &val),
                OpGetWiphyDumpReply::Mac(val) => fmt.field("Mac", &val),
                OpGetWiphyDumpReply::WiphyBands(val) => fmt.field("WiphyBands", &val),
                OpGetWiphyDumpReply::SupportedIftypes(val) => fmt.field("SupportedIftypes", &val),
                OpGetWiphyDumpReply::MaxNumScanSsids(val) => fmt.field("MaxNumScanSsids", &val),
                OpGetWiphyDumpReply::Generation(val) => fmt.field("Generation", &val),
                OpGetWiphyDumpReply::MaxScanIeLen(val) => fmt.field("MaxScanIeLen", &val),
                OpGetWiphyDumpReply::CipherSuites(val) => {
                    fmt.field("CipherSuites", &FormatHex(val))
                }
                OpGetWiphyDumpReply::WiphyRetryShort(val) => fmt.field("WiphyRetryShort", &val),
                OpGetWiphyDumpReply::WiphyRetryLong(val) => fmt.field("WiphyRetryLong", &val),
                OpGetWiphyDumpReply::WiphyFragThreshold(val) => {
                    fmt.field("WiphyFragThreshold", &val)
                }
                OpGetWiphyDumpReply::WiphyRtsThreshold(val) => fmt.field("WiphyRtsThreshold", &val),
                OpGetWiphyDumpReply::MaxNumPmkids(val) => fmt.field("MaxNumPmkids", &val),
                OpGetWiphyDumpReply::WiphyCoverageClass(val) => {
                    fmt.field("WiphyCoverageClass", &val)
                }
                OpGetWiphyDumpReply::TxFrameTypes(val) => fmt.field("TxFrameTypes", &val),
                OpGetWiphyDumpReply::RxFrameTypes(val) => fmt.field("RxFrameTypes", &val),
                OpGetWiphyDumpReply::ControlPortEthertype(val) => {
                    fmt.field("ControlPortEthertype", &val)
                }
                OpGetWiphyDumpReply::WiphyAntennaTx(val) => fmt.field("WiphyAntennaTx", &val),
                OpGetWiphyDumpReply::WiphyAntennaRx(val) => fmt.field("WiphyAntennaRx", &val),
                OpGetWiphyDumpReply::OffchannelTxOk(val) => fmt.field("OffchannelTxOk", &val),
                OpGetWiphyDumpReply::MaxRemainOnChannelDuration(val) => {
                    fmt.field("MaxRemainOnChannelDuration", &val)
                }
                OpGetWiphyDumpReply::WiphyAntennaAvailTx(val) => {
                    fmt.field("WiphyAntennaAvailTx", &val)
                }
                OpGetWiphyDumpReply::WiphyAntennaAvailRx(val) => {
                    fmt.field("WiphyAntennaAvailRx", &val)
                }
                OpGetWiphyDumpReply::WowlanTriggersSupported(val) => {
                    fmt.field("WowlanTriggersSupported", &val)
                }
                OpGetWiphyDumpReply::InterfaceCombinations(val) => {
                    fmt.field("InterfaceCombinations", &val)
                }
                OpGetWiphyDumpReply::SoftwareIftypes(val) => fmt.field("SoftwareIftypes", &val),
                OpGetWiphyDumpReply::MaxNumSchedScanSsids(val) => {
                    fmt.field("MaxNumSchedScanSsids", &val)
                }
                OpGetWiphyDumpReply::MaxSchedScanIeLen(val) => fmt.field("MaxSchedScanIeLen", &val),
                OpGetWiphyDumpReply::SupportApUapsd(val) => fmt.field("SupportApUapsd", &val),
                OpGetWiphyDumpReply::MaxMatchSets(val) => fmt.field("MaxMatchSets", &val),
                OpGetWiphyDumpReply::TdlsSupport(val) => fmt.field("TdlsSupport", &val),
                OpGetWiphyDumpReply::TdlsExternalSetup(val) => fmt.field("TdlsExternalSetup", &val),
                OpGetWiphyDumpReply::FeatureFlags(val) => fmt.field(
                    "FeatureFlags",
                    &FormatFlags(val.into(), FeatureFlags::from_value),
                ),
                OpGetWiphyDumpReply::HtCapabilityMask(val) => fmt.field("HtCapabilityMask", &val),
                OpGetWiphyDumpReply::ExtCapa(val) => fmt.field("ExtCapa", &val),
                OpGetWiphyDumpReply::ExtCapaMask(val) => fmt.field("ExtCapaMask", &val),
                OpGetWiphyDumpReply::VhtCapabilityMask(val) => fmt.field("VhtCapabilityMask", &val),
                OpGetWiphyDumpReply::MaxCsaCounters(val) => fmt.field("MaxCsaCounters", &val),
                OpGetWiphyDumpReply::ExtFeatures(val) => fmt.field("ExtFeatures", &val),
                OpGetWiphyDumpReply::MaxNumSchedScanPlans(val) => {
                    fmt.field("MaxNumSchedScanPlans", &val)
                }
                OpGetWiphyDumpReply::MaxScanPlanInterval(val) => {
                    fmt.field("MaxScanPlanInterval", &val)
                }
                OpGetWiphyDumpReply::MaxScanPlanIterations(val) => {
                    fmt.field("MaxScanPlanIterations", &val)
                }
                OpGetWiphyDumpReply::Bands(val) => fmt.field("Bands", &val),
                OpGetWiphyDumpReply::SchedScanMaxReqs(val) => fmt.field("SchedScanMaxReqs", &val),
                OpGetWiphyDumpReply::TxqStats(val) => fmt.field("TxqStats", &val),
                OpGetWiphyDumpReply::TxqLimit(val) => fmt.field("TxqLimit", &val),
                OpGetWiphyDumpReply::TxqMemoryLimit(val) => fmt.field("TxqMemoryLimit", &val),
                OpGetWiphyDumpReply::TxqQuantum(val) => fmt.field("TxqQuantum", &val),
                OpGetWiphyDumpReply::SarSpec(val) => fmt.field("SarSpec", &val),
                OpGetWiphyDumpReply::MaxNumAkmSuites(val) => fmt.field("MaxNumAkmSuites", &val),
            };
        }
        fmt.finish()
    }
}
impl IterableOpGetWiphyDumpReply<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if cur == offset + PushBuiltinNfgenmsg::len() {
            stack.push(("OpGetWiphyDumpReply", offset));
            return (
                stack,
                missing_type.and_then(|t| OpGetWiphyDumpReply::attr_from_type(t)),
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
                OpGetWiphyDumpReply::Wiphy(val) => {
                    if last_off == offset {
                        stack.push(("Wiphy", last_off));
                        break;
                    }
                }
                OpGetWiphyDumpReply::WiphyName(val) => {
                    if last_off == offset {
                        stack.push(("WiphyName", last_off));
                        break;
                    }
                }
                OpGetWiphyDumpReply::Mac(val) => {
                    if last_off == offset {
                        stack.push(("Mac", last_off));
                        break;
                    }
                }
                OpGetWiphyDumpReply::WiphyBands(val) => {
                    (stack, missing) = val.lookup_attr(offset, missing_type);
                    if !stack.is_empty() {
                        break;
                    }
                }
                OpGetWiphyDumpReply::SupportedIftypes(val) => {
                    (stack, missing) = val.lookup_attr(offset, missing_type);
                    if !stack.is_empty() {
                        break;
                    }
                }
                OpGetWiphyDumpReply::MaxNumScanSsids(val) => {
                    if last_off == offset {
                        stack.push(("MaxNumScanSsids", last_off));
                        break;
                    }
                }
                OpGetWiphyDumpReply::Generation(val) => {
                    if last_off == offset {
                        stack.push(("Generation", last_off));
                        break;
                    }
                }
                OpGetWiphyDumpReply::MaxScanIeLen(val) => {
                    if last_off == offset {
                        stack.push(("MaxScanIeLen", last_off));
                        break;
                    }
                }
                OpGetWiphyDumpReply::CipherSuites(val) => {
                    if last_off == offset {
                        stack.push(("CipherSuites", last_off));
                        break;
                    }
                }
                OpGetWiphyDumpReply::WiphyRetryShort(val) => {
                    if last_off == offset {
                        stack.push(("WiphyRetryShort", last_off));
                        break;
                    }
                }
                OpGetWiphyDumpReply::WiphyRetryLong(val) => {
                    if last_off == offset {
                        stack.push(("WiphyRetryLong", last_off));
                        break;
                    }
                }
                OpGetWiphyDumpReply::WiphyFragThreshold(val) => {
                    if last_off == offset {
                        stack.push(("WiphyFragThreshold", last_off));
                        break;
                    }
                }
                OpGetWiphyDumpReply::WiphyRtsThreshold(val) => {
                    if last_off == offset {
                        stack.push(("WiphyRtsThreshold", last_off));
                        break;
                    }
                }
                OpGetWiphyDumpReply::MaxNumPmkids(val) => {
                    if last_off == offset {
                        stack.push(("MaxNumPmkids", last_off));
                        break;
                    }
                }
                OpGetWiphyDumpReply::WiphyCoverageClass(val) => {
                    if last_off == offset {
                        stack.push(("WiphyCoverageClass", last_off));
                        break;
                    }
                }
                OpGetWiphyDumpReply::TxFrameTypes(val) => {
                    (stack, missing) = val.lookup_attr(offset, missing_type);
                    if !stack.is_empty() {
                        break;
                    }
                }
                OpGetWiphyDumpReply::RxFrameTypes(val) => {
                    (stack, missing) = val.lookup_attr(offset, missing_type);
                    if !stack.is_empty() {
                        break;
                    }
                }
                OpGetWiphyDumpReply::ControlPortEthertype(val) => {
                    if last_off == offset {
                        stack.push(("ControlPortEthertype", last_off));
                        break;
                    }
                }
                OpGetWiphyDumpReply::WiphyAntennaTx(val) => {
                    if last_off == offset {
                        stack.push(("WiphyAntennaTx", last_off));
                        break;
                    }
                }
                OpGetWiphyDumpReply::WiphyAntennaRx(val) => {
                    if last_off == offset {
                        stack.push(("WiphyAntennaRx", last_off));
                        break;
                    }
                }
                OpGetWiphyDumpReply::OffchannelTxOk(val) => {
                    if last_off == offset {
                        stack.push(("OffchannelTxOk", last_off));
                        break;
                    }
                }
                OpGetWiphyDumpReply::MaxRemainOnChannelDuration(val) => {
                    if last_off == offset {
                        stack.push(("MaxRemainOnChannelDuration", last_off));
                        break;
                    }
                }
                OpGetWiphyDumpReply::WiphyAntennaAvailTx(val) => {
                    if last_off == offset {
                        stack.push(("WiphyAntennaAvailTx", last_off));
                        break;
                    }
                }
                OpGetWiphyDumpReply::WiphyAntennaAvailRx(val) => {
                    if last_off == offset {
                        stack.push(("WiphyAntennaAvailRx", last_off));
                        break;
                    }
                }
                OpGetWiphyDumpReply::WowlanTriggersSupported(val) => {
                    (stack, missing) = val.lookup_attr(offset, missing_type);
                    if !stack.is_empty() {
                        break;
                    }
                }
                OpGetWiphyDumpReply::InterfaceCombinations(val) => {
                    if last_off == offset {
                        stack.push(("InterfaceCombinations", last_off));
                        break;
                    }
                }
                OpGetWiphyDumpReply::SoftwareIftypes(val) => {
                    (stack, missing) = val.lookup_attr(offset, missing_type);
                    if !stack.is_empty() {
                        break;
                    }
                }
                OpGetWiphyDumpReply::MaxNumSchedScanSsids(val) => {
                    if last_off == offset {
                        stack.push(("MaxNumSchedScanSsids", last_off));
                        break;
                    }
                }
                OpGetWiphyDumpReply::MaxSchedScanIeLen(val) => {
                    if last_off == offset {
                        stack.push(("MaxSchedScanIeLen", last_off));
                        break;
                    }
                }
                OpGetWiphyDumpReply::SupportApUapsd(val) => {
                    if last_off == offset {
                        stack.push(("SupportApUapsd", last_off));
                        break;
                    }
                }
                OpGetWiphyDumpReply::MaxMatchSets(val) => {
                    if last_off == offset {
                        stack.push(("MaxMatchSets", last_off));
                        break;
                    }
                }
                OpGetWiphyDumpReply::TdlsSupport(val) => {
                    if last_off == offset {
                        stack.push(("TdlsSupport", last_off));
                        break;
                    }
                }
                OpGetWiphyDumpReply::TdlsExternalSetup(val) => {
                    if last_off == offset {
                        stack.push(("TdlsExternalSetup", last_off));
                        break;
                    }
                }
                OpGetWiphyDumpReply::FeatureFlags(val) => {
                    if last_off == offset {
                        stack.push(("FeatureFlags", last_off));
                        break;
                    }
                }
                OpGetWiphyDumpReply::HtCapabilityMask(val) => {
                    if last_off == offset {
                        stack.push(("HtCapabilityMask", last_off));
                        break;
                    }
                }
                OpGetWiphyDumpReply::ExtCapa(val) => {
                    if last_off == offset {
                        stack.push(("ExtCapa", last_off));
                        break;
                    }
                }
                OpGetWiphyDumpReply::ExtCapaMask(val) => {
                    if last_off == offset {
                        stack.push(("ExtCapaMask", last_off));
                        break;
                    }
                }
                OpGetWiphyDumpReply::VhtCapabilityMask(val) => {
                    if last_off == offset {
                        stack.push(("VhtCapabilityMask", last_off));
                        break;
                    }
                }
                OpGetWiphyDumpReply::MaxCsaCounters(val) => {
                    if last_off == offset {
                        stack.push(("MaxCsaCounters", last_off));
                        break;
                    }
                }
                OpGetWiphyDumpReply::ExtFeatures(val) => {
                    if last_off == offset {
                        stack.push(("ExtFeatures", last_off));
                        break;
                    }
                }
                OpGetWiphyDumpReply::MaxNumSchedScanPlans(val) => {
                    if last_off == offset {
                        stack.push(("MaxNumSchedScanPlans", last_off));
                        break;
                    }
                }
                OpGetWiphyDumpReply::MaxScanPlanInterval(val) => {
                    if last_off == offset {
                        stack.push(("MaxScanPlanInterval", last_off));
                        break;
                    }
                }
                OpGetWiphyDumpReply::MaxScanPlanIterations(val) => {
                    if last_off == offset {
                        stack.push(("MaxScanPlanIterations", last_off));
                        break;
                    }
                }
                OpGetWiphyDumpReply::Bands(val) => {
                    if last_off == offset {
                        stack.push(("Bands", last_off));
                        break;
                    }
                }
                OpGetWiphyDumpReply::SchedScanMaxReqs(val) => {
                    if last_off == offset {
                        stack.push(("SchedScanMaxReqs", last_off));
                        break;
                    }
                }
                OpGetWiphyDumpReply::TxqStats(val) => {
                    (stack, missing) = val.lookup_attr(offset, missing_type);
                    if !stack.is_empty() {
                        break;
                    }
                }
                OpGetWiphyDumpReply::TxqLimit(val) => {
                    if last_off == offset {
                        stack.push(("TxqLimit", last_off));
                        break;
                    }
                }
                OpGetWiphyDumpReply::TxqMemoryLimit(val) => {
                    if last_off == offset {
                        stack.push(("TxqMemoryLimit", last_off));
                        break;
                    }
                }
                OpGetWiphyDumpReply::TxqQuantum(val) => {
                    if last_off == offset {
                        stack.push(("TxqQuantum", last_off));
                        break;
                    }
                }
                OpGetWiphyDumpReply::SarSpec(val) => {
                    if last_off == offset {
                        stack.push(("SarSpec", last_off));
                        break;
                    }
                }
                OpGetWiphyDumpReply::MaxNumAkmSuites(val) => {
                    if last_off == offset {
                        stack.push(("MaxNumAkmSuites", last_off));
                        break;
                    }
                }
                _ => {}
            };
            last_off = cur + attrs.pos;
        }
        if !stack.is_empty() {
            stack.push(("OpGetWiphyDumpReply", cur));
        }
        (stack, missing)
    }
}
#[derive(Debug)]
pub struct RequestOpGetWiphyDumpRequest<'r> {
    request: Request<'r>,
}
impl<'r> RequestOpGetWiphyDumpRequest<'r> {
    pub fn new(mut request: Request<'r>) -> Self {
        PushOpGetWiphyDumpRequest::write_header(&mut request.buf_mut());
        Self {
            request: request.set_dump(),
        }
    }
    pub fn encode(&mut self) -> PushOpGetWiphyDumpRequest<&mut Vec<u8>> {
        PushOpGetWiphyDumpRequest::new_without_header(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushOpGetWiphyDumpRequest<RequestBuf<'r>> {
        PushOpGetWiphyDumpRequest::new_without_header(self.request.buf)
    }
}
impl NetlinkRequest for RequestOpGetWiphyDumpRequest<'_> {
    type ReplyType<'buf> = IterableOpGetWiphyDumpReply<'buf>;
    fn protocol(&self) -> Protocol {
        Protocol::Generic("nl80211".as_bytes())
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    fn decode_reply<'buf>(buf: &'buf [u8]) -> Self::ReplyType<'buf> {
        OpGetWiphyDumpReply::new(buf)
    }
    fn lookup(
        buf: &[u8],
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        OpGetWiphyDumpRequest::new(buf).lookup_attr(offset, missing_type)
    }
}
#[doc = "Get information about a wiphy or dump a list of all wiphys. Requests to\ndump get-wiphy should unconditionally include the split-wiphy-dump flag\nin the request.\n"]
pub struct PushOpGetWiphyDoRequest<Prev: Rec> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Rec> Rec for PushOpGetWiphyDoRequest<Prev> {
    fn as_rec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_rec_mut()
    }
}
impl<Prev: Rec> PushOpGetWiphyDoRequest<Prev> {
    pub fn new(mut prev: Prev) -> Self {
        Self::write_header(&mut prev);
        Self::new_without_header(prev)
    }
    fn new_without_header(prev: Prev) -> Self {
        Self {
            prev: Some(prev),
            header_offset: None,
        }
    }
    fn write_header(prev: &mut Prev) {
        let mut header = PushBuiltinNfgenmsg::new();
        header.set_cmd(1u8);
        header.set_version(1u8);
        prev.as_rec_mut().extend(header.as_slice());
    }
    pub fn end_nested(mut self) -> Prev {
        let mut prev = self.prev.take().unwrap();
        if let Some(header_offset) = &self.header_offset {
            finalize_nested_header(prev.as_rec_mut(), *header_offset);
        }
        prev
    }
    pub fn push_wiphy(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 1u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_ifindex(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 3u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_wdev(mut self, value: u64) -> Self {
        push_header(self.as_rec_mut(), 152u16, 8 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
}
impl<Prev: Rec> Drop for PushOpGetWiphyDoRequest<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_rec_mut(), *header_offset);
            }
        }
    }
}
#[doc = "Get information about a wiphy or dump a list of all wiphys. Requests to\ndump get-wiphy should unconditionally include the split-wiphy-dump flag\nin the request.\n"]
#[derive(Clone)]
pub enum OpGetWiphyDoRequest {
    Wiphy(u32),
    Ifindex(u32),
    Wdev(u64),
}
impl<'a> IterableOpGetWiphyDoRequest<'a> {
    pub fn get_wiphy(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetWiphyDoRequest::Wiphy(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpGetWiphyDoRequest",
            "Wiphy",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_ifindex(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetWiphyDoRequest::Ifindex(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpGetWiphyDoRequest",
            "Ifindex",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_wdev(&self) -> Result<u64, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetWiphyDoRequest::Wdev(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpGetWiphyDoRequest",
            "Wdev",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl OpGetWiphyDoRequest {
    pub fn new(buf: &'_ [u8]) -> IterableOpGetWiphyDoRequest<'_> {
        let (_header, attrs) = buf.split_at(buf.len().min(PushBuiltinNfgenmsg::len()));
        IterableOpGetWiphyDoRequest::with_loc(attrs, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        Nl80211Attrs::attr_from_type(r#type)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableOpGetWiphyDoRequest<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableOpGetWiphyDoRequest<'a> {
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
impl<'a> Iterator for IterableOpGetWiphyDoRequest<'a> {
    type Item = Result<OpGetWiphyDoRequest, ErrorContext>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.buf.len() == self.pos {
            return None;
        }
        let pos = self.pos;
        let mut r#type = None;
        while let Some((header, next)) = chop_header(self.buf, &mut self.pos) {
            r#type = Some(header.r#type);
            let res = match header.r#type {
                1u16 => OpGetWiphyDoRequest::Wiphy({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                3u16 => OpGetWiphyDoRequest::Ifindex({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                152u16 => OpGetWiphyDoRequest::Wdev({
                    let res = parse_u64(next);
                    let Some(val) = res else { break };
                    val
                }),
                n => {
                    if cfg!(any(test, feature = "deny-unknown-attrs")) {
                        break;
                    } else {
                        continue;
                    }
                }
            };
            return Some(Ok(res));
        }
        Some(Err(ErrorContext::new(
            "OpGetWiphyDoRequest",
            r#type.and_then(|t| OpGetWiphyDoRequest::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl std::fmt::Debug for IterableOpGetWiphyDoRequest<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("OpGetWiphyDoRequest");
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
                OpGetWiphyDoRequest::Wiphy(val) => fmt.field("Wiphy", &val),
                OpGetWiphyDoRequest::Ifindex(val) => fmt.field("Ifindex", &val),
                OpGetWiphyDoRequest::Wdev(val) => fmt.field("Wdev", &val),
            };
        }
        fmt.finish()
    }
}
impl IterableOpGetWiphyDoRequest<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if cur == offset + PushBuiltinNfgenmsg::len() {
            stack.push(("OpGetWiphyDoRequest", offset));
            return (
                stack,
                missing_type.and_then(|t| OpGetWiphyDoRequest::attr_from_type(t)),
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
                OpGetWiphyDoRequest::Wiphy(val) => {
                    if last_off == offset {
                        stack.push(("Wiphy", last_off));
                        break;
                    }
                }
                OpGetWiphyDoRequest::Ifindex(val) => {
                    if last_off == offset {
                        stack.push(("Ifindex", last_off));
                        break;
                    }
                }
                OpGetWiphyDoRequest::Wdev(val) => {
                    if last_off == offset {
                        stack.push(("Wdev", last_off));
                        break;
                    }
                }
                _ => {}
            };
            last_off = cur + attrs.pos;
        }
        if !stack.is_empty() {
            stack.push(("OpGetWiphyDoRequest", cur));
        }
        (stack, None)
    }
}
#[doc = "Get information about a wiphy or dump a list of all wiphys. Requests to\ndump get-wiphy should unconditionally include the split-wiphy-dump flag\nin the request.\n"]
pub struct PushOpGetWiphyDoReply<Prev: Rec> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Rec> Rec for PushOpGetWiphyDoReply<Prev> {
    fn as_rec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_rec_mut()
    }
}
impl<Prev: Rec> PushOpGetWiphyDoReply<Prev> {
    pub fn new(mut prev: Prev) -> Self {
        Self::write_header(&mut prev);
        Self::new_without_header(prev)
    }
    fn new_without_header(prev: Prev) -> Self {
        Self {
            prev: Some(prev),
            header_offset: None,
        }
    }
    fn write_header(prev: &mut Prev) {
        let mut header = PushBuiltinNfgenmsg::new();
        header.set_cmd(3u8);
        header.set_version(1u8);
        prev.as_rec_mut().extend(header.as_slice());
    }
    pub fn end_nested(mut self) -> Prev {
        let mut prev = self.prev.take().unwrap();
        if let Some(header_offset) = &self.header_offset {
            finalize_nested_header(prev.as_rec_mut(), *header_offset);
        }
        prev
    }
    pub fn push_wiphy(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 1u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_wiphy_name(mut self, value: &CStr) -> Self {
        push_header(
            self.as_rec_mut(),
            2u16,
            value.to_bytes_with_nul().len() as u16,
        );
        self.as_rec_mut().extend(value.to_bytes_with_nul());
        self
    }
    pub fn push_wiphy_name_bytes(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 2u16, (value.len() + 1) as u16);
        self.as_rec_mut().extend(value);
        self.as_rec_mut().push(0);
        self
    }
    pub fn push_mac(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 6u16, value.len() as u16);
        self.as_rec_mut().extend(value);
        self
    }
    pub fn nested_wiphy_bands(mut self) -> PushWiphyBands<Self> {
        let header_offset = push_nested_header(self.as_rec_mut(), 22u16);
        PushWiphyBands {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
    pub fn nested_supported_iftypes(mut self) -> PushSupportedIftypes<Self> {
        let header_offset = push_nested_header(self.as_rec_mut(), 32u16);
        PushSupportedIftypes {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
    pub fn push_max_num_scan_ssids(mut self, value: u8) -> Self {
        push_header(self.as_rec_mut(), 43u16, 1 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_generation(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 46u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_max_scan_ie_len(mut self, value: u16) -> Self {
        push_header(self.as_rec_mut(), 55u16, 2 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_cipher_suites(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 56u16, value.len() as u16);
        self.as_rec_mut().extend(value);
        self
    }
    pub fn push_wiphy_retry_short(mut self, value: u8) -> Self {
        push_header(self.as_rec_mut(), 60u16, 1 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_wiphy_retry_long(mut self, value: u8) -> Self {
        push_header(self.as_rec_mut(), 61u16, 1 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_wiphy_frag_threshold(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 62u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_wiphy_rts_threshold(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 63u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_max_num_pmkids(mut self, value: u8) -> Self {
        push_header(self.as_rec_mut(), 85u16, 1 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_wiphy_coverage_class(mut self, value: u8) -> Self {
        push_header(self.as_rec_mut(), 88u16, 1 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn nested_tx_frame_types(mut self) -> PushIftypeAttrs<Self> {
        let header_offset = push_nested_header(self.as_rec_mut(), 98u16);
        PushIftypeAttrs {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
    pub fn nested_rx_frame_types(mut self) -> PushIftypeAttrs<Self> {
        let header_offset = push_nested_header(self.as_rec_mut(), 99u16);
        PushIftypeAttrs {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
    pub fn push_control_port_ethertype(mut self, value: ()) -> Self {
        push_header(self.as_rec_mut(), 101u16, 0 as u16);
        self
    }
    pub fn push_wiphy_antenna_tx(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 104u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_wiphy_antenna_rx(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 105u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_offchannel_tx_ok(mut self, value: ()) -> Self {
        push_header(self.as_rec_mut(), 107u16, 0 as u16);
        self
    }
    pub fn push_max_remain_on_channel_duration(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 110u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_wiphy_antenna_avail_tx(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 112u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_wiphy_antenna_avail_rx(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 113u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn nested_wowlan_triggers_supported(mut self) -> PushWowlanTriggersAttrs<Self> {
        let header_offset = push_nested_header(self.as_rec_mut(), 117u16);
        PushWowlanTriggersAttrs {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
    pub fn push_interface_combinations(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 119u16, value.len() as u16);
        self.as_rec_mut().extend(value);
        self
    }
    pub fn nested_software_iftypes(mut self) -> PushSupportedIftypes<Self> {
        let header_offset = push_nested_header(self.as_rec_mut(), 120u16);
        PushSupportedIftypes {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
    pub fn push_max_num_sched_scan_ssids(mut self, value: u8) -> Self {
        push_header(self.as_rec_mut(), 122u16, 1 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_max_sched_scan_ie_len(mut self, value: u16) -> Self {
        push_header(self.as_rec_mut(), 123u16, 2 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_support_ap_uapsd(mut self, value: ()) -> Self {
        push_header(self.as_rec_mut(), 129u16, 0 as u16);
        self
    }
    pub fn push_max_match_sets(mut self, value: u8) -> Self {
        push_header(self.as_rec_mut(), 132u16, 1 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_tdls_support(mut self, value: ()) -> Self {
        push_header(self.as_rec_mut(), 138u16, 0 as u16);
        self
    }
    pub fn push_tdls_external_setup(mut self, value: ()) -> Self {
        push_header(self.as_rec_mut(), 139u16, 0 as u16);
        self
    }
    #[doc = "Associated type: \"FeatureFlags\" (1 bit per enumeration)"]
    pub fn push_feature_flags(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 142u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_ht_capability_mask(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 147u16, value.len() as u16);
        self.as_rec_mut().extend(value);
        self
    }
    pub fn push_ext_capa(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 168u16, value.len() as u16);
        self.as_rec_mut().extend(value);
        self
    }
    pub fn push_ext_capa_mask(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 169u16, value.len() as u16);
        self.as_rec_mut().extend(value);
        self
    }
    pub fn push_vht_capability_mask(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 175u16, value.len() as u16);
        self.as_rec_mut().extend(value);
        self
    }
    pub fn push_max_csa_counters(mut self, value: u8) -> Self {
        push_header(self.as_rec_mut(), 205u16, 1 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_ext_features(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 216u16, value.len() as u16);
        self.as_rec_mut().extend(value);
        self
    }
    pub fn push_max_num_sched_scan_plans(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 221u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_max_scan_plan_interval(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 222u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_max_scan_plan_iterations(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 223u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_bands(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 238u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_sched_scan_max_reqs(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 255u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn nested_txq_stats(mut self) -> PushTxqStatsAttrs<Self> {
        let header_offset = push_nested_header(self.as_rec_mut(), 264u16);
        PushTxqStatsAttrs {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
    pub fn push_txq_limit(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 265u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_txq_memory_limit(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 266u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_txq_quantum(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 267u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_sar_spec(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 299u16, value.len() as u16);
        self.as_rec_mut().extend(value);
        self
    }
    pub fn push_max_num_akm_suites(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 315u16, value.len() as u16);
        self.as_rec_mut().extend(value);
        self
    }
}
impl<Prev: Rec> Drop for PushOpGetWiphyDoReply<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_rec_mut(), *header_offset);
            }
        }
    }
}
#[doc = "Get information about a wiphy or dump a list of all wiphys. Requests to\ndump get-wiphy should unconditionally include the split-wiphy-dump flag\nin the request.\n"]
#[derive(Clone)]
pub enum OpGetWiphyDoReply<'a> {
    Wiphy(u32),
    WiphyName(&'a CStr),
    Mac(&'a [u8]),
    WiphyBands(IterableWiphyBands<'a>),
    SupportedIftypes(IterableSupportedIftypes<'a>),
    MaxNumScanSsids(u8),
    Generation(u32),
    MaxScanIeLen(u16),
    CipherSuites(&'a [u8]),
    WiphyRetryShort(u8),
    WiphyRetryLong(u8),
    WiphyFragThreshold(u32),
    WiphyRtsThreshold(u32),
    MaxNumPmkids(u8),
    WiphyCoverageClass(u8),
    TxFrameTypes(IterableIftypeAttrs<'a>),
    RxFrameTypes(IterableIftypeAttrs<'a>),
    ControlPortEthertype(()),
    WiphyAntennaTx(u32),
    WiphyAntennaRx(u32),
    OffchannelTxOk(()),
    MaxRemainOnChannelDuration(u32),
    WiphyAntennaAvailTx(u32),
    WiphyAntennaAvailRx(u32),
    WowlanTriggersSupported(IterableWowlanTriggersAttrs<'a>),
    InterfaceCombinations(&'a [u8]),
    SoftwareIftypes(IterableSupportedIftypes<'a>),
    MaxNumSchedScanSsids(u8),
    MaxSchedScanIeLen(u16),
    SupportApUapsd(()),
    MaxMatchSets(u8),
    TdlsSupport(()),
    TdlsExternalSetup(()),
    #[doc = "Associated type: \"FeatureFlags\" (1 bit per enumeration)"]
    FeatureFlags(u32),
    HtCapabilityMask(&'a [u8]),
    ExtCapa(&'a [u8]),
    ExtCapaMask(&'a [u8]),
    VhtCapabilityMask(&'a [u8]),
    MaxCsaCounters(u8),
    ExtFeatures(&'a [u8]),
    MaxNumSchedScanPlans(u32),
    MaxScanPlanInterval(u32),
    MaxScanPlanIterations(u32),
    Bands(u32),
    SchedScanMaxReqs(u32),
    TxqStats(IterableTxqStatsAttrs<'a>),
    TxqLimit(u32),
    TxqMemoryLimit(u32),
    TxqQuantum(u32),
    SarSpec(&'a [u8]),
    MaxNumAkmSuites(&'a [u8]),
}
impl<'a> IterableOpGetWiphyDoReply<'a> {
    pub fn get_wiphy(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetWiphyDoReply::Wiphy(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpGetWiphyDoReply",
            "Wiphy",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_wiphy_name(&self) -> Result<&'a CStr, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetWiphyDoReply::WiphyName(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpGetWiphyDoReply",
            "WiphyName",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_mac(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetWiphyDoReply::Mac(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpGetWiphyDoReply",
            "Mac",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_wiphy_bands(&self) -> Result<IterableWiphyBands<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetWiphyDoReply::WiphyBands(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpGetWiphyDoReply",
            "WiphyBands",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_supported_iftypes(&self) -> Result<IterableSupportedIftypes<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetWiphyDoReply::SupportedIftypes(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpGetWiphyDoReply",
            "SupportedIftypes",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_max_num_scan_ssids(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetWiphyDoReply::MaxNumScanSsids(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpGetWiphyDoReply",
            "MaxNumScanSsids",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_generation(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetWiphyDoReply::Generation(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpGetWiphyDoReply",
            "Generation",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_max_scan_ie_len(&self) -> Result<u16, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetWiphyDoReply::MaxScanIeLen(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpGetWiphyDoReply",
            "MaxScanIeLen",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_cipher_suites(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetWiphyDoReply::CipherSuites(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpGetWiphyDoReply",
            "CipherSuites",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_wiphy_retry_short(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetWiphyDoReply::WiphyRetryShort(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpGetWiphyDoReply",
            "WiphyRetryShort",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_wiphy_retry_long(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetWiphyDoReply::WiphyRetryLong(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpGetWiphyDoReply",
            "WiphyRetryLong",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_wiphy_frag_threshold(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetWiphyDoReply::WiphyFragThreshold(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpGetWiphyDoReply",
            "WiphyFragThreshold",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_wiphy_rts_threshold(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetWiphyDoReply::WiphyRtsThreshold(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpGetWiphyDoReply",
            "WiphyRtsThreshold",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_max_num_pmkids(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetWiphyDoReply::MaxNumPmkids(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpGetWiphyDoReply",
            "MaxNumPmkids",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_wiphy_coverage_class(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetWiphyDoReply::WiphyCoverageClass(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpGetWiphyDoReply",
            "WiphyCoverageClass",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_tx_frame_types(&self) -> Result<IterableIftypeAttrs<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetWiphyDoReply::TxFrameTypes(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpGetWiphyDoReply",
            "TxFrameTypes",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_rx_frame_types(&self) -> Result<IterableIftypeAttrs<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetWiphyDoReply::RxFrameTypes(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpGetWiphyDoReply",
            "RxFrameTypes",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_control_port_ethertype(&self) -> Result<(), ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetWiphyDoReply::ControlPortEthertype(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpGetWiphyDoReply",
            "ControlPortEthertype",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_wiphy_antenna_tx(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetWiphyDoReply::WiphyAntennaTx(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpGetWiphyDoReply",
            "WiphyAntennaTx",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_wiphy_antenna_rx(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetWiphyDoReply::WiphyAntennaRx(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpGetWiphyDoReply",
            "WiphyAntennaRx",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_offchannel_tx_ok(&self) -> Result<(), ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetWiphyDoReply::OffchannelTxOk(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpGetWiphyDoReply",
            "OffchannelTxOk",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_max_remain_on_channel_duration(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetWiphyDoReply::MaxRemainOnChannelDuration(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpGetWiphyDoReply",
            "MaxRemainOnChannelDuration",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_wiphy_antenna_avail_tx(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetWiphyDoReply::WiphyAntennaAvailTx(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpGetWiphyDoReply",
            "WiphyAntennaAvailTx",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_wiphy_antenna_avail_rx(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetWiphyDoReply::WiphyAntennaAvailRx(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpGetWiphyDoReply",
            "WiphyAntennaAvailRx",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_wowlan_triggers_supported(
        &self,
    ) -> Result<IterableWowlanTriggersAttrs<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetWiphyDoReply::WowlanTriggersSupported(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpGetWiphyDoReply",
            "WowlanTriggersSupported",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_interface_combinations(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetWiphyDoReply::InterfaceCombinations(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpGetWiphyDoReply",
            "InterfaceCombinations",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_software_iftypes(&self) -> Result<IterableSupportedIftypes<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetWiphyDoReply::SoftwareIftypes(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpGetWiphyDoReply",
            "SoftwareIftypes",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_max_num_sched_scan_ssids(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetWiphyDoReply::MaxNumSchedScanSsids(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpGetWiphyDoReply",
            "MaxNumSchedScanSsids",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_max_sched_scan_ie_len(&self) -> Result<u16, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetWiphyDoReply::MaxSchedScanIeLen(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpGetWiphyDoReply",
            "MaxSchedScanIeLen",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_support_ap_uapsd(&self) -> Result<(), ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetWiphyDoReply::SupportApUapsd(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpGetWiphyDoReply",
            "SupportApUapsd",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_max_match_sets(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetWiphyDoReply::MaxMatchSets(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpGetWiphyDoReply",
            "MaxMatchSets",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_tdls_support(&self) -> Result<(), ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetWiphyDoReply::TdlsSupport(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpGetWiphyDoReply",
            "TdlsSupport",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_tdls_external_setup(&self) -> Result<(), ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetWiphyDoReply::TdlsExternalSetup(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpGetWiphyDoReply",
            "TdlsExternalSetup",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Associated type: \"FeatureFlags\" (1 bit per enumeration)"]
    pub fn get_feature_flags(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetWiphyDoReply::FeatureFlags(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpGetWiphyDoReply",
            "FeatureFlags",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_ht_capability_mask(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetWiphyDoReply::HtCapabilityMask(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpGetWiphyDoReply",
            "HtCapabilityMask",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_ext_capa(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetWiphyDoReply::ExtCapa(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpGetWiphyDoReply",
            "ExtCapa",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_ext_capa_mask(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetWiphyDoReply::ExtCapaMask(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpGetWiphyDoReply",
            "ExtCapaMask",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_vht_capability_mask(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetWiphyDoReply::VhtCapabilityMask(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpGetWiphyDoReply",
            "VhtCapabilityMask",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_max_csa_counters(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetWiphyDoReply::MaxCsaCounters(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpGetWiphyDoReply",
            "MaxCsaCounters",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_ext_features(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetWiphyDoReply::ExtFeatures(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpGetWiphyDoReply",
            "ExtFeatures",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_max_num_sched_scan_plans(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetWiphyDoReply::MaxNumSchedScanPlans(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpGetWiphyDoReply",
            "MaxNumSchedScanPlans",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_max_scan_plan_interval(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetWiphyDoReply::MaxScanPlanInterval(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpGetWiphyDoReply",
            "MaxScanPlanInterval",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_max_scan_plan_iterations(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetWiphyDoReply::MaxScanPlanIterations(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpGetWiphyDoReply",
            "MaxScanPlanIterations",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_bands(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetWiphyDoReply::Bands(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpGetWiphyDoReply",
            "Bands",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_sched_scan_max_reqs(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetWiphyDoReply::SchedScanMaxReqs(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpGetWiphyDoReply",
            "SchedScanMaxReqs",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_txq_stats(&self) -> Result<IterableTxqStatsAttrs<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetWiphyDoReply::TxqStats(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpGetWiphyDoReply",
            "TxqStats",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_txq_limit(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetWiphyDoReply::TxqLimit(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpGetWiphyDoReply",
            "TxqLimit",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_txq_memory_limit(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetWiphyDoReply::TxqMemoryLimit(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpGetWiphyDoReply",
            "TxqMemoryLimit",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_txq_quantum(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetWiphyDoReply::TxqQuantum(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpGetWiphyDoReply",
            "TxqQuantum",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_sar_spec(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetWiphyDoReply::SarSpec(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpGetWiphyDoReply",
            "SarSpec",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_max_num_akm_suites(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetWiphyDoReply::MaxNumAkmSuites(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpGetWiphyDoReply",
            "MaxNumAkmSuites",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl<'a> OpGetWiphyDoReply<'a> {
    pub fn new(buf: &'a [u8]) -> IterableOpGetWiphyDoReply<'a> {
        let (_header, attrs) = buf.split_at(buf.len().min(PushBuiltinNfgenmsg::len()));
        IterableOpGetWiphyDoReply::with_loc(attrs, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        Nl80211Attrs::attr_from_type(r#type)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableOpGetWiphyDoReply<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableOpGetWiphyDoReply<'a> {
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
impl<'a> Iterator for IterableOpGetWiphyDoReply<'a> {
    type Item = Result<OpGetWiphyDoReply<'a>, ErrorContext>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.buf.len() == self.pos {
            return None;
        }
        let pos = self.pos;
        let mut r#type = None;
        while let Some((header, next)) = chop_header(self.buf, &mut self.pos) {
            r#type = Some(header.r#type);
            let res = match header.r#type {
                1u16 => OpGetWiphyDoReply::Wiphy({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                2u16 => OpGetWiphyDoReply::WiphyName({
                    let res = CStr::from_bytes_with_nul(next).ok();
                    let Some(val) = res else { break };
                    val
                }),
                6u16 => OpGetWiphyDoReply::Mac({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                22u16 => OpGetWiphyDoReply::WiphyBands({
                    let res = Some(IterableWiphyBands::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                32u16 => OpGetWiphyDoReply::SupportedIftypes({
                    let res = Some(IterableSupportedIftypes::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                43u16 => OpGetWiphyDoReply::MaxNumScanSsids({
                    let res = parse_u8(next);
                    let Some(val) = res else { break };
                    val
                }),
                46u16 => OpGetWiphyDoReply::Generation({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                55u16 => OpGetWiphyDoReply::MaxScanIeLen({
                    let res = parse_u16(next);
                    let Some(val) = res else { break };
                    val
                }),
                56u16 => OpGetWiphyDoReply::CipherSuites({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                60u16 => OpGetWiphyDoReply::WiphyRetryShort({
                    let res = parse_u8(next);
                    let Some(val) = res else { break };
                    val
                }),
                61u16 => OpGetWiphyDoReply::WiphyRetryLong({
                    let res = parse_u8(next);
                    let Some(val) = res else { break };
                    val
                }),
                62u16 => OpGetWiphyDoReply::WiphyFragThreshold({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                63u16 => OpGetWiphyDoReply::WiphyRtsThreshold({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                85u16 => OpGetWiphyDoReply::MaxNumPmkids({
                    let res = parse_u8(next);
                    let Some(val) = res else { break };
                    val
                }),
                88u16 => OpGetWiphyDoReply::WiphyCoverageClass({
                    let res = parse_u8(next);
                    let Some(val) = res else { break };
                    val
                }),
                98u16 => OpGetWiphyDoReply::TxFrameTypes({
                    let res = Some(IterableIftypeAttrs::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                99u16 => OpGetWiphyDoReply::RxFrameTypes({
                    let res = Some(IterableIftypeAttrs::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                101u16 => OpGetWiphyDoReply::ControlPortEthertype(()),
                104u16 => OpGetWiphyDoReply::WiphyAntennaTx({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                105u16 => OpGetWiphyDoReply::WiphyAntennaRx({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                107u16 => OpGetWiphyDoReply::OffchannelTxOk(()),
                110u16 => OpGetWiphyDoReply::MaxRemainOnChannelDuration({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                112u16 => OpGetWiphyDoReply::WiphyAntennaAvailTx({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                113u16 => OpGetWiphyDoReply::WiphyAntennaAvailRx({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                117u16 => OpGetWiphyDoReply::WowlanTriggersSupported({
                    let res = Some(IterableWowlanTriggersAttrs::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                119u16 => OpGetWiphyDoReply::InterfaceCombinations({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                120u16 => OpGetWiphyDoReply::SoftwareIftypes({
                    let res = Some(IterableSupportedIftypes::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                122u16 => OpGetWiphyDoReply::MaxNumSchedScanSsids({
                    let res = parse_u8(next);
                    let Some(val) = res else { break };
                    val
                }),
                123u16 => OpGetWiphyDoReply::MaxSchedScanIeLen({
                    let res = parse_u16(next);
                    let Some(val) = res else { break };
                    val
                }),
                129u16 => OpGetWiphyDoReply::SupportApUapsd(()),
                132u16 => OpGetWiphyDoReply::MaxMatchSets({
                    let res = parse_u8(next);
                    let Some(val) = res else { break };
                    val
                }),
                138u16 => OpGetWiphyDoReply::TdlsSupport(()),
                139u16 => OpGetWiphyDoReply::TdlsExternalSetup(()),
                142u16 => OpGetWiphyDoReply::FeatureFlags({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                147u16 => OpGetWiphyDoReply::HtCapabilityMask({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                168u16 => OpGetWiphyDoReply::ExtCapa({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                169u16 => OpGetWiphyDoReply::ExtCapaMask({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                175u16 => OpGetWiphyDoReply::VhtCapabilityMask({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                205u16 => OpGetWiphyDoReply::MaxCsaCounters({
                    let res = parse_u8(next);
                    let Some(val) = res else { break };
                    val
                }),
                216u16 => OpGetWiphyDoReply::ExtFeatures({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                221u16 => OpGetWiphyDoReply::MaxNumSchedScanPlans({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                222u16 => OpGetWiphyDoReply::MaxScanPlanInterval({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                223u16 => OpGetWiphyDoReply::MaxScanPlanIterations({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                238u16 => OpGetWiphyDoReply::Bands({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                255u16 => OpGetWiphyDoReply::SchedScanMaxReqs({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                264u16 => OpGetWiphyDoReply::TxqStats({
                    let res = Some(IterableTxqStatsAttrs::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                265u16 => OpGetWiphyDoReply::TxqLimit({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                266u16 => OpGetWiphyDoReply::TxqMemoryLimit({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                267u16 => OpGetWiphyDoReply::TxqQuantum({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                299u16 => OpGetWiphyDoReply::SarSpec({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                315u16 => OpGetWiphyDoReply::MaxNumAkmSuites({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                n => {
                    if cfg!(any(test, feature = "deny-unknown-attrs")) {
                        break;
                    } else {
                        continue;
                    }
                }
            };
            return Some(Ok(res));
        }
        Some(Err(ErrorContext::new(
            "OpGetWiphyDoReply",
            r#type.and_then(|t| OpGetWiphyDoReply::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl<'a> std::fmt::Debug for IterableOpGetWiphyDoReply<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("OpGetWiphyDoReply");
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
                OpGetWiphyDoReply::Wiphy(val) => fmt.field("Wiphy", &val),
                OpGetWiphyDoReply::WiphyName(val) => fmt.field("WiphyName", &val),
                OpGetWiphyDoReply::Mac(val) => fmt.field("Mac", &val),
                OpGetWiphyDoReply::WiphyBands(val) => fmt.field("WiphyBands", &val),
                OpGetWiphyDoReply::SupportedIftypes(val) => fmt.field("SupportedIftypes", &val),
                OpGetWiphyDoReply::MaxNumScanSsids(val) => fmt.field("MaxNumScanSsids", &val),
                OpGetWiphyDoReply::Generation(val) => fmt.field("Generation", &val),
                OpGetWiphyDoReply::MaxScanIeLen(val) => fmt.field("MaxScanIeLen", &val),
                OpGetWiphyDoReply::CipherSuites(val) => fmt.field("CipherSuites", &FormatHex(val)),
                OpGetWiphyDoReply::WiphyRetryShort(val) => fmt.field("WiphyRetryShort", &val),
                OpGetWiphyDoReply::WiphyRetryLong(val) => fmt.field("WiphyRetryLong", &val),
                OpGetWiphyDoReply::WiphyFragThreshold(val) => fmt.field("WiphyFragThreshold", &val),
                OpGetWiphyDoReply::WiphyRtsThreshold(val) => fmt.field("WiphyRtsThreshold", &val),
                OpGetWiphyDoReply::MaxNumPmkids(val) => fmt.field("MaxNumPmkids", &val),
                OpGetWiphyDoReply::WiphyCoverageClass(val) => fmt.field("WiphyCoverageClass", &val),
                OpGetWiphyDoReply::TxFrameTypes(val) => fmt.field("TxFrameTypes", &val),
                OpGetWiphyDoReply::RxFrameTypes(val) => fmt.field("RxFrameTypes", &val),
                OpGetWiphyDoReply::ControlPortEthertype(val) => {
                    fmt.field("ControlPortEthertype", &val)
                }
                OpGetWiphyDoReply::WiphyAntennaTx(val) => fmt.field("WiphyAntennaTx", &val),
                OpGetWiphyDoReply::WiphyAntennaRx(val) => fmt.field("WiphyAntennaRx", &val),
                OpGetWiphyDoReply::OffchannelTxOk(val) => fmt.field("OffchannelTxOk", &val),
                OpGetWiphyDoReply::MaxRemainOnChannelDuration(val) => {
                    fmt.field("MaxRemainOnChannelDuration", &val)
                }
                OpGetWiphyDoReply::WiphyAntennaAvailTx(val) => {
                    fmt.field("WiphyAntennaAvailTx", &val)
                }
                OpGetWiphyDoReply::WiphyAntennaAvailRx(val) => {
                    fmt.field("WiphyAntennaAvailRx", &val)
                }
                OpGetWiphyDoReply::WowlanTriggersSupported(val) => {
                    fmt.field("WowlanTriggersSupported", &val)
                }
                OpGetWiphyDoReply::InterfaceCombinations(val) => {
                    fmt.field("InterfaceCombinations", &val)
                }
                OpGetWiphyDoReply::SoftwareIftypes(val) => fmt.field("SoftwareIftypes", &val),
                OpGetWiphyDoReply::MaxNumSchedScanSsids(val) => {
                    fmt.field("MaxNumSchedScanSsids", &val)
                }
                OpGetWiphyDoReply::MaxSchedScanIeLen(val) => fmt.field("MaxSchedScanIeLen", &val),
                OpGetWiphyDoReply::SupportApUapsd(val) => fmt.field("SupportApUapsd", &val),
                OpGetWiphyDoReply::MaxMatchSets(val) => fmt.field("MaxMatchSets", &val),
                OpGetWiphyDoReply::TdlsSupport(val) => fmt.field("TdlsSupport", &val),
                OpGetWiphyDoReply::TdlsExternalSetup(val) => fmt.field("TdlsExternalSetup", &val),
                OpGetWiphyDoReply::FeatureFlags(val) => fmt.field(
                    "FeatureFlags",
                    &FormatFlags(val.into(), FeatureFlags::from_value),
                ),
                OpGetWiphyDoReply::HtCapabilityMask(val) => fmt.field("HtCapabilityMask", &val),
                OpGetWiphyDoReply::ExtCapa(val) => fmt.field("ExtCapa", &val),
                OpGetWiphyDoReply::ExtCapaMask(val) => fmt.field("ExtCapaMask", &val),
                OpGetWiphyDoReply::VhtCapabilityMask(val) => fmt.field("VhtCapabilityMask", &val),
                OpGetWiphyDoReply::MaxCsaCounters(val) => fmt.field("MaxCsaCounters", &val),
                OpGetWiphyDoReply::ExtFeatures(val) => fmt.field("ExtFeatures", &val),
                OpGetWiphyDoReply::MaxNumSchedScanPlans(val) => {
                    fmt.field("MaxNumSchedScanPlans", &val)
                }
                OpGetWiphyDoReply::MaxScanPlanInterval(val) => {
                    fmt.field("MaxScanPlanInterval", &val)
                }
                OpGetWiphyDoReply::MaxScanPlanIterations(val) => {
                    fmt.field("MaxScanPlanIterations", &val)
                }
                OpGetWiphyDoReply::Bands(val) => fmt.field("Bands", &val),
                OpGetWiphyDoReply::SchedScanMaxReqs(val) => fmt.field("SchedScanMaxReqs", &val),
                OpGetWiphyDoReply::TxqStats(val) => fmt.field("TxqStats", &val),
                OpGetWiphyDoReply::TxqLimit(val) => fmt.field("TxqLimit", &val),
                OpGetWiphyDoReply::TxqMemoryLimit(val) => fmt.field("TxqMemoryLimit", &val),
                OpGetWiphyDoReply::TxqQuantum(val) => fmt.field("TxqQuantum", &val),
                OpGetWiphyDoReply::SarSpec(val) => fmt.field("SarSpec", &val),
                OpGetWiphyDoReply::MaxNumAkmSuites(val) => fmt.field("MaxNumAkmSuites", &val),
            };
        }
        fmt.finish()
    }
}
impl IterableOpGetWiphyDoReply<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if cur == offset + PushBuiltinNfgenmsg::len() {
            stack.push(("OpGetWiphyDoReply", offset));
            return (
                stack,
                missing_type.and_then(|t| OpGetWiphyDoReply::attr_from_type(t)),
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
                OpGetWiphyDoReply::Wiphy(val) => {
                    if last_off == offset {
                        stack.push(("Wiphy", last_off));
                        break;
                    }
                }
                OpGetWiphyDoReply::WiphyName(val) => {
                    if last_off == offset {
                        stack.push(("WiphyName", last_off));
                        break;
                    }
                }
                OpGetWiphyDoReply::Mac(val) => {
                    if last_off == offset {
                        stack.push(("Mac", last_off));
                        break;
                    }
                }
                OpGetWiphyDoReply::WiphyBands(val) => {
                    (stack, missing) = val.lookup_attr(offset, missing_type);
                    if !stack.is_empty() {
                        break;
                    }
                }
                OpGetWiphyDoReply::SupportedIftypes(val) => {
                    (stack, missing) = val.lookup_attr(offset, missing_type);
                    if !stack.is_empty() {
                        break;
                    }
                }
                OpGetWiphyDoReply::MaxNumScanSsids(val) => {
                    if last_off == offset {
                        stack.push(("MaxNumScanSsids", last_off));
                        break;
                    }
                }
                OpGetWiphyDoReply::Generation(val) => {
                    if last_off == offset {
                        stack.push(("Generation", last_off));
                        break;
                    }
                }
                OpGetWiphyDoReply::MaxScanIeLen(val) => {
                    if last_off == offset {
                        stack.push(("MaxScanIeLen", last_off));
                        break;
                    }
                }
                OpGetWiphyDoReply::CipherSuites(val) => {
                    if last_off == offset {
                        stack.push(("CipherSuites", last_off));
                        break;
                    }
                }
                OpGetWiphyDoReply::WiphyRetryShort(val) => {
                    if last_off == offset {
                        stack.push(("WiphyRetryShort", last_off));
                        break;
                    }
                }
                OpGetWiphyDoReply::WiphyRetryLong(val) => {
                    if last_off == offset {
                        stack.push(("WiphyRetryLong", last_off));
                        break;
                    }
                }
                OpGetWiphyDoReply::WiphyFragThreshold(val) => {
                    if last_off == offset {
                        stack.push(("WiphyFragThreshold", last_off));
                        break;
                    }
                }
                OpGetWiphyDoReply::WiphyRtsThreshold(val) => {
                    if last_off == offset {
                        stack.push(("WiphyRtsThreshold", last_off));
                        break;
                    }
                }
                OpGetWiphyDoReply::MaxNumPmkids(val) => {
                    if last_off == offset {
                        stack.push(("MaxNumPmkids", last_off));
                        break;
                    }
                }
                OpGetWiphyDoReply::WiphyCoverageClass(val) => {
                    if last_off == offset {
                        stack.push(("WiphyCoverageClass", last_off));
                        break;
                    }
                }
                OpGetWiphyDoReply::TxFrameTypes(val) => {
                    (stack, missing) = val.lookup_attr(offset, missing_type);
                    if !stack.is_empty() {
                        break;
                    }
                }
                OpGetWiphyDoReply::RxFrameTypes(val) => {
                    (stack, missing) = val.lookup_attr(offset, missing_type);
                    if !stack.is_empty() {
                        break;
                    }
                }
                OpGetWiphyDoReply::ControlPortEthertype(val) => {
                    if last_off == offset {
                        stack.push(("ControlPortEthertype", last_off));
                        break;
                    }
                }
                OpGetWiphyDoReply::WiphyAntennaTx(val) => {
                    if last_off == offset {
                        stack.push(("WiphyAntennaTx", last_off));
                        break;
                    }
                }
                OpGetWiphyDoReply::WiphyAntennaRx(val) => {
                    if last_off == offset {
                        stack.push(("WiphyAntennaRx", last_off));
                        break;
                    }
                }
                OpGetWiphyDoReply::OffchannelTxOk(val) => {
                    if last_off == offset {
                        stack.push(("OffchannelTxOk", last_off));
                        break;
                    }
                }
                OpGetWiphyDoReply::MaxRemainOnChannelDuration(val) => {
                    if last_off == offset {
                        stack.push(("MaxRemainOnChannelDuration", last_off));
                        break;
                    }
                }
                OpGetWiphyDoReply::WiphyAntennaAvailTx(val) => {
                    if last_off == offset {
                        stack.push(("WiphyAntennaAvailTx", last_off));
                        break;
                    }
                }
                OpGetWiphyDoReply::WiphyAntennaAvailRx(val) => {
                    if last_off == offset {
                        stack.push(("WiphyAntennaAvailRx", last_off));
                        break;
                    }
                }
                OpGetWiphyDoReply::WowlanTriggersSupported(val) => {
                    (stack, missing) = val.lookup_attr(offset, missing_type);
                    if !stack.is_empty() {
                        break;
                    }
                }
                OpGetWiphyDoReply::InterfaceCombinations(val) => {
                    if last_off == offset {
                        stack.push(("InterfaceCombinations", last_off));
                        break;
                    }
                }
                OpGetWiphyDoReply::SoftwareIftypes(val) => {
                    (stack, missing) = val.lookup_attr(offset, missing_type);
                    if !stack.is_empty() {
                        break;
                    }
                }
                OpGetWiphyDoReply::MaxNumSchedScanSsids(val) => {
                    if last_off == offset {
                        stack.push(("MaxNumSchedScanSsids", last_off));
                        break;
                    }
                }
                OpGetWiphyDoReply::MaxSchedScanIeLen(val) => {
                    if last_off == offset {
                        stack.push(("MaxSchedScanIeLen", last_off));
                        break;
                    }
                }
                OpGetWiphyDoReply::SupportApUapsd(val) => {
                    if last_off == offset {
                        stack.push(("SupportApUapsd", last_off));
                        break;
                    }
                }
                OpGetWiphyDoReply::MaxMatchSets(val) => {
                    if last_off == offset {
                        stack.push(("MaxMatchSets", last_off));
                        break;
                    }
                }
                OpGetWiphyDoReply::TdlsSupport(val) => {
                    if last_off == offset {
                        stack.push(("TdlsSupport", last_off));
                        break;
                    }
                }
                OpGetWiphyDoReply::TdlsExternalSetup(val) => {
                    if last_off == offset {
                        stack.push(("TdlsExternalSetup", last_off));
                        break;
                    }
                }
                OpGetWiphyDoReply::FeatureFlags(val) => {
                    if last_off == offset {
                        stack.push(("FeatureFlags", last_off));
                        break;
                    }
                }
                OpGetWiphyDoReply::HtCapabilityMask(val) => {
                    if last_off == offset {
                        stack.push(("HtCapabilityMask", last_off));
                        break;
                    }
                }
                OpGetWiphyDoReply::ExtCapa(val) => {
                    if last_off == offset {
                        stack.push(("ExtCapa", last_off));
                        break;
                    }
                }
                OpGetWiphyDoReply::ExtCapaMask(val) => {
                    if last_off == offset {
                        stack.push(("ExtCapaMask", last_off));
                        break;
                    }
                }
                OpGetWiphyDoReply::VhtCapabilityMask(val) => {
                    if last_off == offset {
                        stack.push(("VhtCapabilityMask", last_off));
                        break;
                    }
                }
                OpGetWiphyDoReply::MaxCsaCounters(val) => {
                    if last_off == offset {
                        stack.push(("MaxCsaCounters", last_off));
                        break;
                    }
                }
                OpGetWiphyDoReply::ExtFeatures(val) => {
                    if last_off == offset {
                        stack.push(("ExtFeatures", last_off));
                        break;
                    }
                }
                OpGetWiphyDoReply::MaxNumSchedScanPlans(val) => {
                    if last_off == offset {
                        stack.push(("MaxNumSchedScanPlans", last_off));
                        break;
                    }
                }
                OpGetWiphyDoReply::MaxScanPlanInterval(val) => {
                    if last_off == offset {
                        stack.push(("MaxScanPlanInterval", last_off));
                        break;
                    }
                }
                OpGetWiphyDoReply::MaxScanPlanIterations(val) => {
                    if last_off == offset {
                        stack.push(("MaxScanPlanIterations", last_off));
                        break;
                    }
                }
                OpGetWiphyDoReply::Bands(val) => {
                    if last_off == offset {
                        stack.push(("Bands", last_off));
                        break;
                    }
                }
                OpGetWiphyDoReply::SchedScanMaxReqs(val) => {
                    if last_off == offset {
                        stack.push(("SchedScanMaxReqs", last_off));
                        break;
                    }
                }
                OpGetWiphyDoReply::TxqStats(val) => {
                    (stack, missing) = val.lookup_attr(offset, missing_type);
                    if !stack.is_empty() {
                        break;
                    }
                }
                OpGetWiphyDoReply::TxqLimit(val) => {
                    if last_off == offset {
                        stack.push(("TxqLimit", last_off));
                        break;
                    }
                }
                OpGetWiphyDoReply::TxqMemoryLimit(val) => {
                    if last_off == offset {
                        stack.push(("TxqMemoryLimit", last_off));
                        break;
                    }
                }
                OpGetWiphyDoReply::TxqQuantum(val) => {
                    if last_off == offset {
                        stack.push(("TxqQuantum", last_off));
                        break;
                    }
                }
                OpGetWiphyDoReply::SarSpec(val) => {
                    if last_off == offset {
                        stack.push(("SarSpec", last_off));
                        break;
                    }
                }
                OpGetWiphyDoReply::MaxNumAkmSuites(val) => {
                    if last_off == offset {
                        stack.push(("MaxNumAkmSuites", last_off));
                        break;
                    }
                }
                _ => {}
            };
            last_off = cur + attrs.pos;
        }
        if !stack.is_empty() {
            stack.push(("OpGetWiphyDoReply", cur));
        }
        (stack, missing)
    }
}
#[derive(Debug)]
pub struct RequestOpGetWiphyDoRequest<'r> {
    request: Request<'r>,
}
impl<'r> RequestOpGetWiphyDoRequest<'r> {
    pub fn new(mut request: Request<'r>) -> Self {
        PushOpGetWiphyDoRequest::write_header(&mut request.buf_mut());
        Self { request: request }
    }
    pub fn encode(&mut self) -> PushOpGetWiphyDoRequest<&mut Vec<u8>> {
        PushOpGetWiphyDoRequest::new_without_header(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushOpGetWiphyDoRequest<RequestBuf<'r>> {
        PushOpGetWiphyDoRequest::new_without_header(self.request.buf)
    }
}
impl NetlinkRequest for RequestOpGetWiphyDoRequest<'_> {
    type ReplyType<'buf> = IterableOpGetWiphyDoReply<'buf>;
    fn protocol(&self) -> Protocol {
        Protocol::Generic("nl80211".as_bytes())
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    fn decode_reply<'buf>(buf: &'buf [u8]) -> Self::ReplyType<'buf> {
        OpGetWiphyDoReply::new(buf)
    }
    fn lookup(
        buf: &[u8],
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        OpGetWiphyDoRequest::new(buf).lookup_attr(offset, missing_type)
    }
}
#[doc = "Get information about an interface or dump a list of all interfaces"]
pub struct PushOpGetInterfaceDumpRequest<Prev: Rec> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Rec> Rec for PushOpGetInterfaceDumpRequest<Prev> {
    fn as_rec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_rec_mut()
    }
}
impl<Prev: Rec> PushOpGetInterfaceDumpRequest<Prev> {
    pub fn new(mut prev: Prev) -> Self {
        Self::write_header(&mut prev);
        Self::new_without_header(prev)
    }
    fn new_without_header(prev: Prev) -> Self {
        Self {
            prev: Some(prev),
            header_offset: None,
        }
    }
    fn write_header(prev: &mut Prev) {
        let mut header = PushBuiltinNfgenmsg::new();
        header.set_cmd(5u8);
        header.set_version(1u8);
        prev.as_rec_mut().extend(header.as_slice());
    }
    pub fn end_nested(mut self) -> Prev {
        let mut prev = self.prev.take().unwrap();
        if let Some(header_offset) = &self.header_offset {
            finalize_nested_header(prev.as_rec_mut(), *header_offset);
        }
        prev
    }
    pub fn push_ifname(mut self, value: &CStr) -> Self {
        push_header(
            self.as_rec_mut(),
            4u16,
            value.to_bytes_with_nul().len() as u16,
        );
        self.as_rec_mut().extend(value.to_bytes_with_nul());
        self
    }
    pub fn push_ifname_bytes(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 4u16, (value.len() + 1) as u16);
        self.as_rec_mut().extend(value);
        self.as_rec_mut().push(0);
        self
    }
}
impl<Prev: Rec> Drop for PushOpGetInterfaceDumpRequest<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_rec_mut(), *header_offset);
            }
        }
    }
}
#[doc = "Get information about an interface or dump a list of all interfaces"]
#[derive(Clone)]
pub enum OpGetInterfaceDumpRequest<'a> {
    Ifname(&'a CStr),
}
impl<'a> IterableOpGetInterfaceDumpRequest<'a> {
    pub fn get_ifname(&self) -> Result<&'a CStr, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetInterfaceDumpRequest::Ifname(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpGetInterfaceDumpRequest",
            "Ifname",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl<'a> OpGetInterfaceDumpRequest<'a> {
    pub fn new(buf: &'a [u8]) -> IterableOpGetInterfaceDumpRequest<'a> {
        let (_header, attrs) = buf.split_at(buf.len().min(PushBuiltinNfgenmsg::len()));
        IterableOpGetInterfaceDumpRequest::with_loc(attrs, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        Nl80211Attrs::attr_from_type(r#type)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableOpGetInterfaceDumpRequest<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableOpGetInterfaceDumpRequest<'a> {
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
impl<'a> Iterator for IterableOpGetInterfaceDumpRequest<'a> {
    type Item = Result<OpGetInterfaceDumpRequest<'a>, ErrorContext>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.buf.len() == self.pos {
            return None;
        }
        let pos = self.pos;
        let mut r#type = None;
        while let Some((header, next)) = chop_header(self.buf, &mut self.pos) {
            r#type = Some(header.r#type);
            let res = match header.r#type {
                4u16 => OpGetInterfaceDumpRequest::Ifname({
                    let res = CStr::from_bytes_with_nul(next).ok();
                    let Some(val) = res else { break };
                    val
                }),
                n => {
                    if cfg!(any(test, feature = "deny-unknown-attrs")) {
                        break;
                    } else {
                        continue;
                    }
                }
            };
            return Some(Ok(res));
        }
        Some(Err(ErrorContext::new(
            "OpGetInterfaceDumpRequest",
            r#type.and_then(|t| OpGetInterfaceDumpRequest::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl<'a> std::fmt::Debug for IterableOpGetInterfaceDumpRequest<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("OpGetInterfaceDumpRequest");
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
                OpGetInterfaceDumpRequest::Ifname(val) => fmt.field("Ifname", &val),
            };
        }
        fmt.finish()
    }
}
impl IterableOpGetInterfaceDumpRequest<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if cur == offset + PushBuiltinNfgenmsg::len() {
            stack.push(("OpGetInterfaceDumpRequest", offset));
            return (
                stack,
                missing_type.and_then(|t| OpGetInterfaceDumpRequest::attr_from_type(t)),
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
                OpGetInterfaceDumpRequest::Ifname(val) => {
                    if last_off == offset {
                        stack.push(("Ifname", last_off));
                        break;
                    }
                }
                _ => {}
            };
            last_off = cur + attrs.pos;
        }
        if !stack.is_empty() {
            stack.push(("OpGetInterfaceDumpRequest", cur));
        }
        (stack, None)
    }
}
#[doc = "Get information about an interface or dump a list of all interfaces"]
pub struct PushOpGetInterfaceDumpReply<Prev: Rec> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Rec> Rec for PushOpGetInterfaceDumpReply<Prev> {
    fn as_rec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_rec_mut()
    }
}
impl<Prev: Rec> PushOpGetInterfaceDumpReply<Prev> {
    pub fn new(mut prev: Prev) -> Self {
        Self::write_header(&mut prev);
        Self::new_without_header(prev)
    }
    fn new_without_header(prev: Prev) -> Self {
        Self {
            prev: Some(prev),
            header_offset: None,
        }
    }
    fn write_header(prev: &mut Prev) {
        let mut header = PushBuiltinNfgenmsg::new();
        header.set_cmd(7u8);
        header.set_version(1u8);
        prev.as_rec_mut().extend(header.as_slice());
    }
    pub fn end_nested(mut self) -> Prev {
        let mut prev = self.prev.take().unwrap();
        if let Some(header_offset) = &self.header_offset {
            finalize_nested_header(prev.as_rec_mut(), *header_offset);
        }
        prev
    }
    pub fn push_wiphy(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 1u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_ifindex(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 3u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_ifname(mut self, value: &CStr) -> Self {
        push_header(
            self.as_rec_mut(),
            4u16,
            value.to_bytes_with_nul().len() as u16,
        );
        self.as_rec_mut().extend(value.to_bytes_with_nul());
        self
    }
    pub fn push_ifname_bytes(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 4u16, (value.len() + 1) as u16);
        self.as_rec_mut().extend(value);
        self.as_rec_mut().push(0);
        self
    }
    pub fn push_iftype(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 5u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_mac(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 6u16, value.len() as u16);
        self.as_rec_mut().extend(value);
        self
    }
    pub fn push_generation(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 46u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_4addr(mut self, value: u8) -> Self {
        push_header(self.as_rec_mut(), 82u16, 1 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_wdev(mut self, value: u64) -> Self {
        push_header(self.as_rec_mut(), 152u16, 8 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn nested_txq_stats(mut self) -> PushTxqStatsAttrs<Self> {
        let header_offset = push_nested_header(self.as_rec_mut(), 264u16);
        PushTxqStatsAttrs {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
}
impl<Prev: Rec> Drop for PushOpGetInterfaceDumpReply<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_rec_mut(), *header_offset);
            }
        }
    }
}
#[doc = "Get information about an interface or dump a list of all interfaces"]
#[derive(Clone)]
pub enum OpGetInterfaceDumpReply<'a> {
    Wiphy(u32),
    Ifindex(u32),
    Ifname(&'a CStr),
    Iftype(u32),
    Mac(&'a [u8]),
    Generation(u32),
    _4addr(u8),
    Wdev(u64),
    TxqStats(IterableTxqStatsAttrs<'a>),
}
impl<'a> IterableOpGetInterfaceDumpReply<'a> {
    pub fn get_wiphy(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetInterfaceDumpReply::Wiphy(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpGetInterfaceDumpReply",
            "Wiphy",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_ifindex(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetInterfaceDumpReply::Ifindex(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpGetInterfaceDumpReply",
            "Ifindex",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_ifname(&self) -> Result<&'a CStr, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetInterfaceDumpReply::Ifname(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpGetInterfaceDumpReply",
            "Ifname",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_iftype(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetInterfaceDumpReply::Iftype(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpGetInterfaceDumpReply",
            "Iftype",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_mac(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetInterfaceDumpReply::Mac(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpGetInterfaceDumpReply",
            "Mac",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_generation(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetInterfaceDumpReply::Generation(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpGetInterfaceDumpReply",
            "Generation",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_4addr(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetInterfaceDumpReply::_4addr(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpGetInterfaceDumpReply",
            "4addr",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_wdev(&self) -> Result<u64, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetInterfaceDumpReply::Wdev(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpGetInterfaceDumpReply",
            "Wdev",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_txq_stats(&self) -> Result<IterableTxqStatsAttrs<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetInterfaceDumpReply::TxqStats(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpGetInterfaceDumpReply",
            "TxqStats",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl<'a> OpGetInterfaceDumpReply<'a> {
    pub fn new(buf: &'a [u8]) -> IterableOpGetInterfaceDumpReply<'a> {
        let (_header, attrs) = buf.split_at(buf.len().min(PushBuiltinNfgenmsg::len()));
        IterableOpGetInterfaceDumpReply::with_loc(attrs, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        Nl80211Attrs::attr_from_type(r#type)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableOpGetInterfaceDumpReply<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableOpGetInterfaceDumpReply<'a> {
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
impl<'a> Iterator for IterableOpGetInterfaceDumpReply<'a> {
    type Item = Result<OpGetInterfaceDumpReply<'a>, ErrorContext>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.buf.len() == self.pos {
            return None;
        }
        let pos = self.pos;
        let mut r#type = None;
        while let Some((header, next)) = chop_header(self.buf, &mut self.pos) {
            r#type = Some(header.r#type);
            let res = match header.r#type {
                1u16 => OpGetInterfaceDumpReply::Wiphy({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                3u16 => OpGetInterfaceDumpReply::Ifindex({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                4u16 => OpGetInterfaceDumpReply::Ifname({
                    let res = CStr::from_bytes_with_nul(next).ok();
                    let Some(val) = res else { break };
                    val
                }),
                5u16 => OpGetInterfaceDumpReply::Iftype({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                6u16 => OpGetInterfaceDumpReply::Mac({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                46u16 => OpGetInterfaceDumpReply::Generation({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                82u16 => OpGetInterfaceDumpReply::_4addr({
                    let res = parse_u8(next);
                    let Some(val) = res else { break };
                    val
                }),
                152u16 => OpGetInterfaceDumpReply::Wdev({
                    let res = parse_u64(next);
                    let Some(val) = res else { break };
                    val
                }),
                264u16 => OpGetInterfaceDumpReply::TxqStats({
                    let res = Some(IterableTxqStatsAttrs::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                n => {
                    if cfg!(any(test, feature = "deny-unknown-attrs")) {
                        break;
                    } else {
                        continue;
                    }
                }
            };
            return Some(Ok(res));
        }
        Some(Err(ErrorContext::new(
            "OpGetInterfaceDumpReply",
            r#type.and_then(|t| OpGetInterfaceDumpReply::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl<'a> std::fmt::Debug for IterableOpGetInterfaceDumpReply<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("OpGetInterfaceDumpReply");
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
                OpGetInterfaceDumpReply::Wiphy(val) => fmt.field("Wiphy", &val),
                OpGetInterfaceDumpReply::Ifindex(val) => fmt.field("Ifindex", &val),
                OpGetInterfaceDumpReply::Ifname(val) => fmt.field("Ifname", &val),
                OpGetInterfaceDumpReply::Iftype(val) => fmt.field("Iftype", &val),
                OpGetInterfaceDumpReply::Mac(val) => fmt.field("Mac", &val),
                OpGetInterfaceDumpReply::Generation(val) => fmt.field("Generation", &val),
                OpGetInterfaceDumpReply::_4addr(val) => fmt.field("_4addr", &val),
                OpGetInterfaceDumpReply::Wdev(val) => fmt.field("Wdev", &val),
                OpGetInterfaceDumpReply::TxqStats(val) => fmt.field("TxqStats", &val),
            };
        }
        fmt.finish()
    }
}
impl IterableOpGetInterfaceDumpReply<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if cur == offset + PushBuiltinNfgenmsg::len() {
            stack.push(("OpGetInterfaceDumpReply", offset));
            return (
                stack,
                missing_type.and_then(|t| OpGetInterfaceDumpReply::attr_from_type(t)),
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
                OpGetInterfaceDumpReply::Wiphy(val) => {
                    if last_off == offset {
                        stack.push(("Wiphy", last_off));
                        break;
                    }
                }
                OpGetInterfaceDumpReply::Ifindex(val) => {
                    if last_off == offset {
                        stack.push(("Ifindex", last_off));
                        break;
                    }
                }
                OpGetInterfaceDumpReply::Ifname(val) => {
                    if last_off == offset {
                        stack.push(("Ifname", last_off));
                        break;
                    }
                }
                OpGetInterfaceDumpReply::Iftype(val) => {
                    if last_off == offset {
                        stack.push(("Iftype", last_off));
                        break;
                    }
                }
                OpGetInterfaceDumpReply::Mac(val) => {
                    if last_off == offset {
                        stack.push(("Mac", last_off));
                        break;
                    }
                }
                OpGetInterfaceDumpReply::Generation(val) => {
                    if last_off == offset {
                        stack.push(("Generation", last_off));
                        break;
                    }
                }
                OpGetInterfaceDumpReply::_4addr(val) => {
                    if last_off == offset {
                        stack.push(("4addr", last_off));
                        break;
                    }
                }
                OpGetInterfaceDumpReply::Wdev(val) => {
                    if last_off == offset {
                        stack.push(("Wdev", last_off));
                        break;
                    }
                }
                OpGetInterfaceDumpReply::TxqStats(val) => {
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
            stack.push(("OpGetInterfaceDumpReply", cur));
        }
        (stack, missing)
    }
}
#[derive(Debug)]
pub struct RequestOpGetInterfaceDumpRequest<'r> {
    request: Request<'r>,
}
impl<'r> RequestOpGetInterfaceDumpRequest<'r> {
    pub fn new(mut request: Request<'r>) -> Self {
        PushOpGetInterfaceDumpRequest::write_header(&mut request.buf_mut());
        Self {
            request: request.set_dump(),
        }
    }
    pub fn encode(&mut self) -> PushOpGetInterfaceDumpRequest<&mut Vec<u8>> {
        PushOpGetInterfaceDumpRequest::new_without_header(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushOpGetInterfaceDumpRequest<RequestBuf<'r>> {
        PushOpGetInterfaceDumpRequest::new_without_header(self.request.buf)
    }
}
impl NetlinkRequest for RequestOpGetInterfaceDumpRequest<'_> {
    type ReplyType<'buf> = IterableOpGetInterfaceDumpReply<'buf>;
    fn protocol(&self) -> Protocol {
        Protocol::Generic("nl80211".as_bytes())
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    fn decode_reply<'buf>(buf: &'buf [u8]) -> Self::ReplyType<'buf> {
        OpGetInterfaceDumpReply::new(buf)
    }
    fn lookup(
        buf: &[u8],
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        OpGetInterfaceDumpRequest::new(buf).lookup_attr(offset, missing_type)
    }
}
#[doc = "Get information about an interface or dump a list of all interfaces"]
pub struct PushOpGetInterfaceDoRequest<Prev: Rec> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Rec> Rec for PushOpGetInterfaceDoRequest<Prev> {
    fn as_rec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_rec_mut()
    }
}
impl<Prev: Rec> PushOpGetInterfaceDoRequest<Prev> {
    pub fn new(mut prev: Prev) -> Self {
        Self::write_header(&mut prev);
        Self::new_without_header(prev)
    }
    fn new_without_header(prev: Prev) -> Self {
        Self {
            prev: Some(prev),
            header_offset: None,
        }
    }
    fn write_header(prev: &mut Prev) {
        let mut header = PushBuiltinNfgenmsg::new();
        header.set_cmd(5u8);
        header.set_version(1u8);
        prev.as_rec_mut().extend(header.as_slice());
    }
    pub fn end_nested(mut self) -> Prev {
        let mut prev = self.prev.take().unwrap();
        if let Some(header_offset) = &self.header_offset {
            finalize_nested_header(prev.as_rec_mut(), *header_offset);
        }
        prev
    }
    pub fn push_ifname(mut self, value: &CStr) -> Self {
        push_header(
            self.as_rec_mut(),
            4u16,
            value.to_bytes_with_nul().len() as u16,
        );
        self.as_rec_mut().extend(value.to_bytes_with_nul());
        self
    }
    pub fn push_ifname_bytes(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 4u16, (value.len() + 1) as u16);
        self.as_rec_mut().extend(value);
        self.as_rec_mut().push(0);
        self
    }
}
impl<Prev: Rec> Drop for PushOpGetInterfaceDoRequest<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_rec_mut(), *header_offset);
            }
        }
    }
}
#[doc = "Get information about an interface or dump a list of all interfaces"]
#[derive(Clone)]
pub enum OpGetInterfaceDoRequest<'a> {
    Ifname(&'a CStr),
}
impl<'a> IterableOpGetInterfaceDoRequest<'a> {
    pub fn get_ifname(&self) -> Result<&'a CStr, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetInterfaceDoRequest::Ifname(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpGetInterfaceDoRequest",
            "Ifname",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl<'a> OpGetInterfaceDoRequest<'a> {
    pub fn new(buf: &'a [u8]) -> IterableOpGetInterfaceDoRequest<'a> {
        let (_header, attrs) = buf.split_at(buf.len().min(PushBuiltinNfgenmsg::len()));
        IterableOpGetInterfaceDoRequest::with_loc(attrs, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        Nl80211Attrs::attr_from_type(r#type)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableOpGetInterfaceDoRequest<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableOpGetInterfaceDoRequest<'a> {
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
impl<'a> Iterator for IterableOpGetInterfaceDoRequest<'a> {
    type Item = Result<OpGetInterfaceDoRequest<'a>, ErrorContext>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.buf.len() == self.pos {
            return None;
        }
        let pos = self.pos;
        let mut r#type = None;
        while let Some((header, next)) = chop_header(self.buf, &mut self.pos) {
            r#type = Some(header.r#type);
            let res = match header.r#type {
                4u16 => OpGetInterfaceDoRequest::Ifname({
                    let res = CStr::from_bytes_with_nul(next).ok();
                    let Some(val) = res else { break };
                    val
                }),
                n => {
                    if cfg!(any(test, feature = "deny-unknown-attrs")) {
                        break;
                    } else {
                        continue;
                    }
                }
            };
            return Some(Ok(res));
        }
        Some(Err(ErrorContext::new(
            "OpGetInterfaceDoRequest",
            r#type.and_then(|t| OpGetInterfaceDoRequest::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl<'a> std::fmt::Debug for IterableOpGetInterfaceDoRequest<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("OpGetInterfaceDoRequest");
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
                OpGetInterfaceDoRequest::Ifname(val) => fmt.field("Ifname", &val),
            };
        }
        fmt.finish()
    }
}
impl IterableOpGetInterfaceDoRequest<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if cur == offset + PushBuiltinNfgenmsg::len() {
            stack.push(("OpGetInterfaceDoRequest", offset));
            return (
                stack,
                missing_type.and_then(|t| OpGetInterfaceDoRequest::attr_from_type(t)),
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
                OpGetInterfaceDoRequest::Ifname(val) => {
                    if last_off == offset {
                        stack.push(("Ifname", last_off));
                        break;
                    }
                }
                _ => {}
            };
            last_off = cur + attrs.pos;
        }
        if !stack.is_empty() {
            stack.push(("OpGetInterfaceDoRequest", cur));
        }
        (stack, None)
    }
}
#[doc = "Get information about an interface or dump a list of all interfaces"]
pub struct PushOpGetInterfaceDoReply<Prev: Rec> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Rec> Rec for PushOpGetInterfaceDoReply<Prev> {
    fn as_rec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_rec_mut()
    }
}
impl<Prev: Rec> PushOpGetInterfaceDoReply<Prev> {
    pub fn new(mut prev: Prev) -> Self {
        Self::write_header(&mut prev);
        Self::new_without_header(prev)
    }
    fn new_without_header(prev: Prev) -> Self {
        Self {
            prev: Some(prev),
            header_offset: None,
        }
    }
    fn write_header(prev: &mut Prev) {
        let mut header = PushBuiltinNfgenmsg::new();
        header.set_cmd(7u8);
        header.set_version(1u8);
        prev.as_rec_mut().extend(header.as_slice());
    }
    pub fn end_nested(mut self) -> Prev {
        let mut prev = self.prev.take().unwrap();
        if let Some(header_offset) = &self.header_offset {
            finalize_nested_header(prev.as_rec_mut(), *header_offset);
        }
        prev
    }
    pub fn push_wiphy(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 1u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_ifindex(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 3u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_ifname(mut self, value: &CStr) -> Self {
        push_header(
            self.as_rec_mut(),
            4u16,
            value.to_bytes_with_nul().len() as u16,
        );
        self.as_rec_mut().extend(value.to_bytes_with_nul());
        self
    }
    pub fn push_ifname_bytes(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 4u16, (value.len() + 1) as u16);
        self.as_rec_mut().extend(value);
        self.as_rec_mut().push(0);
        self
    }
    pub fn push_iftype(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 5u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_mac(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 6u16, value.len() as u16);
        self.as_rec_mut().extend(value);
        self
    }
    pub fn push_generation(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 46u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_4addr(mut self, value: u8) -> Self {
        push_header(self.as_rec_mut(), 82u16, 1 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_wdev(mut self, value: u64) -> Self {
        push_header(self.as_rec_mut(), 152u16, 8 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn nested_txq_stats(mut self) -> PushTxqStatsAttrs<Self> {
        let header_offset = push_nested_header(self.as_rec_mut(), 264u16);
        PushTxqStatsAttrs {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
}
impl<Prev: Rec> Drop for PushOpGetInterfaceDoReply<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_rec_mut(), *header_offset);
            }
        }
    }
}
#[doc = "Get information about an interface or dump a list of all interfaces"]
#[derive(Clone)]
pub enum OpGetInterfaceDoReply<'a> {
    Wiphy(u32),
    Ifindex(u32),
    Ifname(&'a CStr),
    Iftype(u32),
    Mac(&'a [u8]),
    Generation(u32),
    _4addr(u8),
    Wdev(u64),
    TxqStats(IterableTxqStatsAttrs<'a>),
}
impl<'a> IterableOpGetInterfaceDoReply<'a> {
    pub fn get_wiphy(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetInterfaceDoReply::Wiphy(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpGetInterfaceDoReply",
            "Wiphy",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_ifindex(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetInterfaceDoReply::Ifindex(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpGetInterfaceDoReply",
            "Ifindex",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_ifname(&self) -> Result<&'a CStr, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetInterfaceDoReply::Ifname(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpGetInterfaceDoReply",
            "Ifname",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_iftype(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetInterfaceDoReply::Iftype(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpGetInterfaceDoReply",
            "Iftype",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_mac(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetInterfaceDoReply::Mac(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpGetInterfaceDoReply",
            "Mac",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_generation(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetInterfaceDoReply::Generation(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpGetInterfaceDoReply",
            "Generation",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_4addr(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetInterfaceDoReply::_4addr(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpGetInterfaceDoReply",
            "4addr",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_wdev(&self) -> Result<u64, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetInterfaceDoReply::Wdev(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpGetInterfaceDoReply",
            "Wdev",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_txq_stats(&self) -> Result<IterableTxqStatsAttrs<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetInterfaceDoReply::TxqStats(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpGetInterfaceDoReply",
            "TxqStats",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl<'a> OpGetInterfaceDoReply<'a> {
    pub fn new(buf: &'a [u8]) -> IterableOpGetInterfaceDoReply<'a> {
        let (_header, attrs) = buf.split_at(buf.len().min(PushBuiltinNfgenmsg::len()));
        IterableOpGetInterfaceDoReply::with_loc(attrs, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        Nl80211Attrs::attr_from_type(r#type)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableOpGetInterfaceDoReply<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableOpGetInterfaceDoReply<'a> {
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
impl<'a> Iterator for IterableOpGetInterfaceDoReply<'a> {
    type Item = Result<OpGetInterfaceDoReply<'a>, ErrorContext>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.buf.len() == self.pos {
            return None;
        }
        let pos = self.pos;
        let mut r#type = None;
        while let Some((header, next)) = chop_header(self.buf, &mut self.pos) {
            r#type = Some(header.r#type);
            let res = match header.r#type {
                1u16 => OpGetInterfaceDoReply::Wiphy({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                3u16 => OpGetInterfaceDoReply::Ifindex({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                4u16 => OpGetInterfaceDoReply::Ifname({
                    let res = CStr::from_bytes_with_nul(next).ok();
                    let Some(val) = res else { break };
                    val
                }),
                5u16 => OpGetInterfaceDoReply::Iftype({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                6u16 => OpGetInterfaceDoReply::Mac({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                46u16 => OpGetInterfaceDoReply::Generation({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                82u16 => OpGetInterfaceDoReply::_4addr({
                    let res = parse_u8(next);
                    let Some(val) = res else { break };
                    val
                }),
                152u16 => OpGetInterfaceDoReply::Wdev({
                    let res = parse_u64(next);
                    let Some(val) = res else { break };
                    val
                }),
                264u16 => OpGetInterfaceDoReply::TxqStats({
                    let res = Some(IterableTxqStatsAttrs::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                n => {
                    if cfg!(any(test, feature = "deny-unknown-attrs")) {
                        break;
                    } else {
                        continue;
                    }
                }
            };
            return Some(Ok(res));
        }
        Some(Err(ErrorContext::new(
            "OpGetInterfaceDoReply",
            r#type.and_then(|t| OpGetInterfaceDoReply::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl<'a> std::fmt::Debug for IterableOpGetInterfaceDoReply<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("OpGetInterfaceDoReply");
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
                OpGetInterfaceDoReply::Wiphy(val) => fmt.field("Wiphy", &val),
                OpGetInterfaceDoReply::Ifindex(val) => fmt.field("Ifindex", &val),
                OpGetInterfaceDoReply::Ifname(val) => fmt.field("Ifname", &val),
                OpGetInterfaceDoReply::Iftype(val) => fmt.field("Iftype", &val),
                OpGetInterfaceDoReply::Mac(val) => fmt.field("Mac", &val),
                OpGetInterfaceDoReply::Generation(val) => fmt.field("Generation", &val),
                OpGetInterfaceDoReply::_4addr(val) => fmt.field("_4addr", &val),
                OpGetInterfaceDoReply::Wdev(val) => fmt.field("Wdev", &val),
                OpGetInterfaceDoReply::TxqStats(val) => fmt.field("TxqStats", &val),
            };
        }
        fmt.finish()
    }
}
impl IterableOpGetInterfaceDoReply<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if cur == offset + PushBuiltinNfgenmsg::len() {
            stack.push(("OpGetInterfaceDoReply", offset));
            return (
                stack,
                missing_type.and_then(|t| OpGetInterfaceDoReply::attr_from_type(t)),
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
                OpGetInterfaceDoReply::Wiphy(val) => {
                    if last_off == offset {
                        stack.push(("Wiphy", last_off));
                        break;
                    }
                }
                OpGetInterfaceDoReply::Ifindex(val) => {
                    if last_off == offset {
                        stack.push(("Ifindex", last_off));
                        break;
                    }
                }
                OpGetInterfaceDoReply::Ifname(val) => {
                    if last_off == offset {
                        stack.push(("Ifname", last_off));
                        break;
                    }
                }
                OpGetInterfaceDoReply::Iftype(val) => {
                    if last_off == offset {
                        stack.push(("Iftype", last_off));
                        break;
                    }
                }
                OpGetInterfaceDoReply::Mac(val) => {
                    if last_off == offset {
                        stack.push(("Mac", last_off));
                        break;
                    }
                }
                OpGetInterfaceDoReply::Generation(val) => {
                    if last_off == offset {
                        stack.push(("Generation", last_off));
                        break;
                    }
                }
                OpGetInterfaceDoReply::_4addr(val) => {
                    if last_off == offset {
                        stack.push(("4addr", last_off));
                        break;
                    }
                }
                OpGetInterfaceDoReply::Wdev(val) => {
                    if last_off == offset {
                        stack.push(("Wdev", last_off));
                        break;
                    }
                }
                OpGetInterfaceDoReply::TxqStats(val) => {
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
            stack.push(("OpGetInterfaceDoReply", cur));
        }
        (stack, missing)
    }
}
#[derive(Debug)]
pub struct RequestOpGetInterfaceDoRequest<'r> {
    request: Request<'r>,
}
impl<'r> RequestOpGetInterfaceDoRequest<'r> {
    pub fn new(mut request: Request<'r>) -> Self {
        PushOpGetInterfaceDoRequest::write_header(&mut request.buf_mut());
        Self { request: request }
    }
    pub fn encode(&mut self) -> PushOpGetInterfaceDoRequest<&mut Vec<u8>> {
        PushOpGetInterfaceDoRequest::new_without_header(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushOpGetInterfaceDoRequest<RequestBuf<'r>> {
        PushOpGetInterfaceDoRequest::new_without_header(self.request.buf)
    }
}
impl NetlinkRequest for RequestOpGetInterfaceDoRequest<'_> {
    type ReplyType<'buf> = IterableOpGetInterfaceDoReply<'buf>;
    fn protocol(&self) -> Protocol {
        Protocol::Generic("nl80211".as_bytes())
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    fn decode_reply<'buf>(buf: &'buf [u8]) -> Self::ReplyType<'buf> {
        OpGetInterfaceDoReply::new(buf)
    }
    fn lookup(
        buf: &[u8],
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        OpGetInterfaceDoRequest::new(buf).lookup_attr(offset, missing_type)
    }
}
#[doc = "Get information about supported protocol features"]
pub struct PushOpGetProtocolFeaturesDoRequest<Prev: Rec> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Rec> Rec for PushOpGetProtocolFeaturesDoRequest<Prev> {
    fn as_rec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_rec_mut()
    }
}
impl<Prev: Rec> PushOpGetProtocolFeaturesDoRequest<Prev> {
    pub fn new(mut prev: Prev) -> Self {
        Self::write_header(&mut prev);
        Self::new_without_header(prev)
    }
    fn new_without_header(prev: Prev) -> Self {
        Self {
            prev: Some(prev),
            header_offset: None,
        }
    }
    fn write_header(prev: &mut Prev) {
        let mut header = PushBuiltinNfgenmsg::new();
        header.set_cmd(95u8);
        header.set_version(1u8);
        prev.as_rec_mut().extend(header.as_slice());
    }
    pub fn end_nested(mut self) -> Prev {
        let mut prev = self.prev.take().unwrap();
        if let Some(header_offset) = &self.header_offset {
            finalize_nested_header(prev.as_rec_mut(), *header_offset);
        }
        prev
    }
    #[doc = "Associated type: \"ProtocolFeatures\" (enum)"]
    pub fn push_protocol_features(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 172u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
}
impl<Prev: Rec> Drop for PushOpGetProtocolFeaturesDoRequest<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_rec_mut(), *header_offset);
            }
        }
    }
}
#[doc = "Get information about supported protocol features"]
#[derive(Clone)]
pub enum OpGetProtocolFeaturesDoRequest {
    #[doc = "Associated type: \"ProtocolFeatures\" (enum)"]
    ProtocolFeatures(u32),
}
impl<'a> IterableOpGetProtocolFeaturesDoRequest<'a> {
    #[doc = "Associated type: \"ProtocolFeatures\" (enum)"]
    pub fn get_protocol_features(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetProtocolFeaturesDoRequest::ProtocolFeatures(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpGetProtocolFeaturesDoRequest",
            "ProtocolFeatures",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl OpGetProtocolFeaturesDoRequest {
    pub fn new(buf: &'_ [u8]) -> IterableOpGetProtocolFeaturesDoRequest<'_> {
        let (_header, attrs) = buf.split_at(buf.len().min(PushBuiltinNfgenmsg::len()));
        IterableOpGetProtocolFeaturesDoRequest::with_loc(attrs, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        Nl80211Attrs::attr_from_type(r#type)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableOpGetProtocolFeaturesDoRequest<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableOpGetProtocolFeaturesDoRequest<'a> {
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
impl<'a> Iterator for IterableOpGetProtocolFeaturesDoRequest<'a> {
    type Item = Result<OpGetProtocolFeaturesDoRequest, ErrorContext>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.buf.len() == self.pos {
            return None;
        }
        let pos = self.pos;
        let mut r#type = None;
        while let Some((header, next)) = chop_header(self.buf, &mut self.pos) {
            r#type = Some(header.r#type);
            let res = match header.r#type {
                172u16 => OpGetProtocolFeaturesDoRequest::ProtocolFeatures({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                n => {
                    if cfg!(any(test, feature = "deny-unknown-attrs")) {
                        break;
                    } else {
                        continue;
                    }
                }
            };
            return Some(Ok(res));
        }
        Some(Err(ErrorContext::new(
            "OpGetProtocolFeaturesDoRequest",
            r#type.and_then(|t| OpGetProtocolFeaturesDoRequest::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl std::fmt::Debug for IterableOpGetProtocolFeaturesDoRequest<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("OpGetProtocolFeaturesDoRequest");
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
                OpGetProtocolFeaturesDoRequest::ProtocolFeatures(val) => fmt.field(
                    "ProtocolFeatures",
                    &FormatFlags(val.into(), ProtocolFeatures::from_value),
                ),
            };
        }
        fmt.finish()
    }
}
impl IterableOpGetProtocolFeaturesDoRequest<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if cur == offset + PushBuiltinNfgenmsg::len() {
            stack.push(("OpGetProtocolFeaturesDoRequest", offset));
            return (
                stack,
                missing_type.and_then(|t| OpGetProtocolFeaturesDoRequest::attr_from_type(t)),
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
                OpGetProtocolFeaturesDoRequest::ProtocolFeatures(val) => {
                    if last_off == offset {
                        stack.push(("ProtocolFeatures", last_off));
                        break;
                    }
                }
                _ => {}
            };
            last_off = cur + attrs.pos;
        }
        if !stack.is_empty() {
            stack.push(("OpGetProtocolFeaturesDoRequest", cur));
        }
        (stack, None)
    }
}
#[doc = "Get information about supported protocol features"]
pub struct PushOpGetProtocolFeaturesDoReply<Prev: Rec> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Rec> Rec for PushOpGetProtocolFeaturesDoReply<Prev> {
    fn as_rec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_rec_mut()
    }
}
impl<Prev: Rec> PushOpGetProtocolFeaturesDoReply<Prev> {
    pub fn new(mut prev: Prev) -> Self {
        Self::write_header(&mut prev);
        Self::new_without_header(prev)
    }
    fn new_without_header(prev: Prev) -> Self {
        Self {
            prev: Some(prev),
            header_offset: None,
        }
    }
    fn write_header(prev: &mut Prev) {
        let mut header = PushBuiltinNfgenmsg::new();
        header.set_cmd(95u8);
        header.set_version(1u8);
        prev.as_rec_mut().extend(header.as_slice());
    }
    pub fn end_nested(mut self) -> Prev {
        let mut prev = self.prev.take().unwrap();
        if let Some(header_offset) = &self.header_offset {
            finalize_nested_header(prev.as_rec_mut(), *header_offset);
        }
        prev
    }
    #[doc = "Associated type: \"ProtocolFeatures\" (enum)"]
    pub fn push_protocol_features(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 172u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
}
impl<Prev: Rec> Drop for PushOpGetProtocolFeaturesDoReply<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_rec_mut(), *header_offset);
            }
        }
    }
}
#[doc = "Get information about supported protocol features"]
#[derive(Clone)]
pub enum OpGetProtocolFeaturesDoReply {
    #[doc = "Associated type: \"ProtocolFeatures\" (enum)"]
    ProtocolFeatures(u32),
}
impl<'a> IterableOpGetProtocolFeaturesDoReply<'a> {
    #[doc = "Associated type: \"ProtocolFeatures\" (enum)"]
    pub fn get_protocol_features(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetProtocolFeaturesDoReply::ProtocolFeatures(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpGetProtocolFeaturesDoReply",
            "ProtocolFeatures",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl OpGetProtocolFeaturesDoReply {
    pub fn new(buf: &'_ [u8]) -> IterableOpGetProtocolFeaturesDoReply<'_> {
        let (_header, attrs) = buf.split_at(buf.len().min(PushBuiltinNfgenmsg::len()));
        IterableOpGetProtocolFeaturesDoReply::with_loc(attrs, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        Nl80211Attrs::attr_from_type(r#type)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableOpGetProtocolFeaturesDoReply<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableOpGetProtocolFeaturesDoReply<'a> {
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
impl<'a> Iterator for IterableOpGetProtocolFeaturesDoReply<'a> {
    type Item = Result<OpGetProtocolFeaturesDoReply, ErrorContext>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.buf.len() == self.pos {
            return None;
        }
        let pos = self.pos;
        let mut r#type = None;
        while let Some((header, next)) = chop_header(self.buf, &mut self.pos) {
            r#type = Some(header.r#type);
            let res = match header.r#type {
                172u16 => OpGetProtocolFeaturesDoReply::ProtocolFeatures({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                n => {
                    if cfg!(any(test, feature = "deny-unknown-attrs")) {
                        break;
                    } else {
                        continue;
                    }
                }
            };
            return Some(Ok(res));
        }
        Some(Err(ErrorContext::new(
            "OpGetProtocolFeaturesDoReply",
            r#type.and_then(|t| OpGetProtocolFeaturesDoReply::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl std::fmt::Debug for IterableOpGetProtocolFeaturesDoReply<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("OpGetProtocolFeaturesDoReply");
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
                OpGetProtocolFeaturesDoReply::ProtocolFeatures(val) => fmt.field(
                    "ProtocolFeatures",
                    &FormatFlags(val.into(), ProtocolFeatures::from_value),
                ),
            };
        }
        fmt.finish()
    }
}
impl IterableOpGetProtocolFeaturesDoReply<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if cur == offset + PushBuiltinNfgenmsg::len() {
            stack.push(("OpGetProtocolFeaturesDoReply", offset));
            return (
                stack,
                missing_type.and_then(|t| OpGetProtocolFeaturesDoReply::attr_from_type(t)),
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
                OpGetProtocolFeaturesDoReply::ProtocolFeatures(val) => {
                    if last_off == offset {
                        stack.push(("ProtocolFeatures", last_off));
                        break;
                    }
                }
                _ => {}
            };
            last_off = cur + attrs.pos;
        }
        if !stack.is_empty() {
            stack.push(("OpGetProtocolFeaturesDoReply", cur));
        }
        (stack, None)
    }
}
#[derive(Debug)]
pub struct RequestOpGetProtocolFeaturesDoRequest<'r> {
    request: Request<'r>,
}
impl<'r> RequestOpGetProtocolFeaturesDoRequest<'r> {
    pub fn new(mut request: Request<'r>) -> Self {
        PushOpGetProtocolFeaturesDoRequest::write_header(&mut request.buf_mut());
        Self { request: request }
    }
    pub fn encode(&mut self) -> PushOpGetProtocolFeaturesDoRequest<&mut Vec<u8>> {
        PushOpGetProtocolFeaturesDoRequest::new_without_header(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushOpGetProtocolFeaturesDoRequest<RequestBuf<'r>> {
        PushOpGetProtocolFeaturesDoRequest::new_without_header(self.request.buf)
    }
}
impl NetlinkRequest for RequestOpGetProtocolFeaturesDoRequest<'_> {
    type ReplyType<'buf> = IterableOpGetProtocolFeaturesDoReply<'buf>;
    fn protocol(&self) -> Protocol {
        Protocol::Generic("nl80211".as_bytes())
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    fn decode_reply<'buf>(buf: &'buf [u8]) -> Self::ReplyType<'buf> {
        OpGetProtocolFeaturesDoReply::new(buf)
    }
    fn lookup(
        buf: &[u8],
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        OpGetProtocolFeaturesDoRequest::new(buf).lookup_attr(offset, missing_type)
    }
}
use crate::traits::LookupFn;
use crate::utils::RequestBuf;
#[derive(Debug)]
pub struct Request<'buf> {
    buf: RequestBuf<'buf>,
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
    pub fn new_from_buf(buf: Vec<u8>) -> Self {
        Self {
            flags: 0,
            buf: RequestBuf::Own(buf),
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
        Self {
            flags: 0,
            buf: RequestBuf::Ref(buf),
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
    #[doc = "Set `NLM_F_DUMP` flag"]
    fn set_dump(mut self) -> Self {
        self.flags |= consts::NLM_F_DUMP as u16;
        self
    }
    pub fn op_get_wiphy_dump_request(self) -> RequestOpGetWiphyDumpRequest<'buf> {
        let mut res = RequestOpGetWiphyDumpRequest::new(self);
        res.request.do_writeback(
            res.protocol(),
            "op-get-wiphy-dump-request",
            RequestOpGetWiphyDumpRequest::lookup,
        );
        res
    }
    pub fn op_get_wiphy_do_request(self) -> RequestOpGetWiphyDoRequest<'buf> {
        let mut res = RequestOpGetWiphyDoRequest::new(self);
        res.request.do_writeback(
            res.protocol(),
            "op-get-wiphy-do-request",
            RequestOpGetWiphyDoRequest::lookup,
        );
        res
    }
    pub fn op_get_interface_dump_request(self) -> RequestOpGetInterfaceDumpRequest<'buf> {
        let mut res = RequestOpGetInterfaceDumpRequest::new(self);
        res.request.do_writeback(
            res.protocol(),
            "op-get-interface-dump-request",
            RequestOpGetInterfaceDumpRequest::lookup,
        );
        res
    }
    pub fn op_get_interface_do_request(self) -> RequestOpGetInterfaceDoRequest<'buf> {
        let mut res = RequestOpGetInterfaceDoRequest::new(self);
        res.request.do_writeback(
            res.protocol(),
            "op-get-interface-do-request",
            RequestOpGetInterfaceDoRequest::lookup,
        );
        res
    }
    pub fn op_get_protocol_features_do_request(
        self,
    ) -> RequestOpGetProtocolFeaturesDoRequest<'buf> {
        let mut res = RequestOpGetProtocolFeaturesDoRequest::new(self);
        res.request.do_writeback(
            res.protocol(),
            "op-get-protocol-features-do-request",
            RequestOpGetProtocolFeaturesDoRequest::lookup,
        );
        res
    }
}
