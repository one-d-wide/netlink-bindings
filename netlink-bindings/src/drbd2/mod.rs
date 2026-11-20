#![doc = "DRBD 9 management interface: configure and monitor replicated block\ndevice resources, their devices, connections to peers, peer devices and\nnetwork paths.\n"]
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
    traits::{self, NetlinkRequest, Protocol},
    utils::*,
};
pub const PROTONAME: &str = "drbd2";
pub const PROTONAME_CSTR: &CStr = c"drbd2";
#[doc = "Maximum length of a resource name including the terminating NUL.\n"]
pub const RESOURCE_NAME_MAX: u64 = 128u64;
#[doc = "Maximum length of a connection name including the terminating NUL.\n"]
pub const CONNECTION_NAME_MAX: u64 = 64u64;
#[doc = "Maximum length of the shared secret including the terminating NUL.\n"]
pub const SHARED_SECRET_MAX: u64 = 64u64;
#[doc = "Maximum length of a crypto algorithm name including the terminating NUL.\n"]
pub const ALG_NAME_MAX: u64 = 64u64;
#[doc = "Maximum length of a network transport name including the terminating\nNUL.\n"]
pub const TRANSPORT_NAME_MAX: u64 = 64u64;
#[doc = "Maximum length of a backing/meta device path including the terminating\nNUL.\n"]
pub const DEVICE_PATH_MAX: u64 = 128u64;
#[doc = "Maximum length of the cpu-mask string.\n"]
pub const CPU_MASK_SIZE: u64 = 256u64;
#[doc = "Maximum length of a helper name.\n"]
pub const HELPER_NAME_MAX: u64 = 32u64;
#[doc = "Maximum size in bytes of the history UUID array (32 peer slots of 8\nbytes).\n"]
pub const HISTORY_UUIDS_SIZE: u64 = 256u64;
#[doc = "What to do when the backing device reports an I/O error (on-io-error).\n"]
#[doc = "Enum - defines an integer enumeration, with values for each entry incrementing by 1, (e.g. 0, 1, 2, 3)"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum IoErrorPolicy {
    #[doc = "Mark the failed block out of sync, change the disk state to inconsistent\nand retry the request on a peer.\n"]
    PassOn = 0,
    #[doc = "Call the local-io-error handler.\n"]
    CallLocalIoError = 1,
    #[doc = "Detach the backing device and continue in diskless mode.\n"]
    Detach = 2,
}
impl IoErrorPolicy {
    pub fn from_value(value: u64) -> Option<Self> {
        Some(match value {
            0 => Self::PassOn,
            1 => Self::CallLocalIoError,
            2 => Self::Detach,
            _ => return None,
        })
    }
}
#[doc = "Fencing policy (fencing).\n"]
#[doc = "Enum - defines an integer enumeration, with values for each entry incrementing by 1, (e.g. 0, 1, 2, 3)"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum FencingPolicy {
    #[doc = "Take no fencing action at all.\n"]
    DontCare = 0,
    #[doc = "A node that becomes a disconnected primary calls the fence-peer handler\nto outdate the peer.\n"]
    Resource = 1,
    #[doc = "A node that becomes a disconnected primary freezes its I/O and calls the\nfence-peer handler, which may have to shoot the peer.\n"]
    Stonith = 2,
}
impl FencingPolicy {
    pub fn from_value(value: u64) -> Option<Self> {
        Some(match value {
            0 => Self::DontCare,
            1 => Self::Resource,
            2 => Self::Stonith,
            _ => return None,
        })
    }
}
#[doc = "Split-brain recovery policy (after-sb-0pri, after-sb-1pri,\nafter-sb-2pri, rr-conflict).\n"]
#[doc = "Enum - defines an integer enumeration, with values for each entry incrementing by 1, (e.g. 0, 1, 2, 3)"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum AfterSbPolicy {
    #[doc = "Do not resynchronize automatically, simply disconnect.\n"]
    Disconnect = 0,
    #[doc = "Resynchronize from the node that became primary first.\n"]
    DiscardYoungerPrimary = 1,
    #[doc = "Resynchronize from the node that became primary last.\n"]
    DiscardOlderPrimary = 2,
    #[doc = "Resynchronize from the node that wrote data since the split brain was\ndetected, or disconnect if both nodes wrote.\n"]
    DiscardZeroChanges = 3,
    #[doc = "Resynchronize from the node with more modified blocks.\n"]
    DiscardLeastChanges = 4,
    #[doc = "Discard the data on this node and resynchronize it from the peer.\n"]
    DiscardLocal = 5,
    #[doc = "Discard the data on the peer and resynchronize it from this node.\n"]
    DiscardRemote = 6,
    #[doc = "Discard the data on the secondary if the after-sb-0pri policy would\ndiscard it as well, otherwise disconnect.\n"]
    Consensus = 7,
    #[doc = "Discard the data on the secondary node.\n"]
    DiscardSecondary = 8,
    #[doc = "Take the after-sb-0pri decision and call the pri-lost-after-sb handler\non the node that has to give up its data.\n"]
    CallPriLostAfterSb = 9,
    #[doc = "Always take the after-sb-0pri decision, even if that changes the\nprimary\\'s view of the data; this can crash the primary.\n"]
    ViolentlyAs0p = 10,
    #[doc = "Disconnect now and immediately retry to connect (rr-conflict only).\n"]
    RetryConnect = 11,
    #[doc = "Reverse the resync direction so that the current primary is\nresynchronized from the current secondary (rr-conflict with protocol A\nonly).\n"]
    AutoDiscard = 12,
}
impl AfterSbPolicy {
    pub fn from_value(value: u64) -> Option<Self> {
        Some(match value {
            0 => Self::Disconnect,
            1 => Self::DiscardYoungerPrimary,
            2 => Self::DiscardOlderPrimary,
            3 => Self::DiscardZeroChanges,
            4 => Self::DiscardLeastChanges,
            5 => Self::DiscardLocal,
            6 => Self::DiscardRemote,
            7 => Self::Consensus,
            8 => Self::DiscardSecondary,
            9 => Self::CallPriLostAfterSb,
            10 => Self::ViolentlyAs0p,
            11 => Self::RetryConnect,
            12 => Self::AutoDiscard,
            _ => return None,
        })
    }
}
#[doc = "What to do when no data is accessible (on-no-data-accessible).\n"]
#[doc = "Enum - defines an integer enumeration, with values for each entry incrementing by 1, (e.g. 0, 1, 2, 3)"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum OnNoDataPolicy {
    #[doc = "Complete the requests with an I/O error, so that system calls fail with\nEIO.\n"]
    IoError = 0,
    #[doc = "Suspend I/O until the data becomes accessible again or the administrator\nforces I/O to resume.\n"]
    SuspendIo = 1,
}
impl OnNoDataPolicy {
    pub fn from_value(value: u64) -> Option<Self> {
        Some(match value {
            0 => Self::IoError,
            1 => Self::SuspendIo,
            _ => return None,
        })
    }
}
#[doc = "What to do when quorum is lost (on-no-quorum).\n"]
#[doc = "Enum - defines an integer enumeration, with values for each entry incrementing by 1, (e.g. 0, 1, 2, 3)"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum OnNoQuorumPolicy {
    #[doc = "Complete the requests with an I/O error while the resource has no\nquorum.\n"]
    IoError = 0,
    #[doc = "Freeze I/O while the resource has no quorum.\n"]
    SuspendIo = 1,
}
impl OnNoQuorumPolicy {
    pub fn from_value(value: u64) -> Option<Self> {
        Some(match value {
            0 => Self::IoError,
            1 => Self::SuspendIo,
            _ => return None,
        })
    }
}
#[doc = "What to do with a suspended primary that became outdated.\n"]
#[doc = "Enum - defines an integer enumeration, with values for each entry incrementing by 1, (e.g. 0, 1, 2, 3)"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum OnSuspendedPrimaryOutdatedPolicy {
    #[doc = "Reject connection attempts and stay isolated.\n"]
    Disconnect = 0,
    #[doc = "Demote to secondary immediately and fail all pending and new I/O\nrequests.\n"]
    ForceSecondary = 1,
}
impl OnSuspendedPrimaryOutdatedPolicy {
    pub fn from_value(value: u64) -> Option<Self> {
        Some(match value {
            0 => Self::Disconnect,
            1 => Self::ForceSecondary,
            _ => return None,
        })
    }
}
#[doc = "What to do when the replication link is congested (on-congestion).\n"]
#[doc = "Enum - defines an integer enumeration, with values for each entry incrementing by 1, (e.g. 0, 1, 2, 3)"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum OnCongestionPolicy {
    #[doc = "Block application writes until the transport\\'s send queue drains.\n"]
    Block = 0,
    #[doc = "Switch into ahead/behind mode and record further changes in the bitmap\ninstead of replicating them.\n"]
    PullAhead = 1,
    #[doc = "Disconnect from the peer instead of blocking application writes.\n"]
    Disconnect = 2,
}
impl OnCongestionPolicy {
    pub fn from_value(value: u64) -> Option<Self> {
        Some(match value {
            0 => Self::Block,
            1 => Self::PullAhead,
            2 => Self::Disconnect,
            _ => return None,
        })
    }
}
#[doc = "Read balancing policy (read-balancing).\n"]
#[doc = "Enum - defines an integer enumeration, with values for each entry incrementing by 1, (e.g. 0, 1, 2, 3)"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ReadBalancingPolicy {
    #[doc = "Always read from the local disk.\n"]
    PreferLocal = 0,
    #[doc = "Always read from a peer\\'s disk.\n"]
    PreferRemote = 1,
    #[doc = "Alternate between the local disk and a peer for consecutive reads.\n"]
    RoundRobin = 2,
    #[doc = "Read from the disk with the fewest pending requests.\n"]
    LeastPending = 3,
    #[doc = "Read from a peer only while the local disk is congested; deprecated\nsince DRBD 9.1.12.\n"]
    WhenCongestedRemote = 4,
    #[doc = "Read alternating 32 KiB stripes from the local disk and from a peer.\n"]
    Striping32k = 5,
    #[doc = "Read alternating 64 KiB stripes from the local disk and from a peer.\n"]
    Striping64k = 6,
    #[doc = "Read alternating 128 KiB stripes from the local disk and from a peer.\n"]
    Striping128k = 7,
    #[doc = "Read alternating 256 KiB stripes from the local disk and from a peer.\n"]
    Striping256k = 8,
    #[doc = "Read alternating 512 KiB stripes from the local disk and from a peer.\n"]
    Striping512k = 9,
    #[doc = "Read alternating 1 MiB stripes from the local disk and from a peer.\n"]
    Striping1m = 10,
}
impl ReadBalancingPolicy {
    pub fn from_value(value: u64) -> Option<Self> {
        Some(match value {
            0 => Self::PreferLocal,
            1 => Self::PreferRemote,
            2 => Self::RoundRobin,
            3 => Self::LeastPending,
            4 => Self::WhenCongestedRemote,
            5 => Self::Striping32k,
            6 => Self::Striping64k,
            7 => Self::Striping128k,
            8 => Self::Striping256k,
            9 => Self::Striping512k,
            10 => Self::Striping1m,
            _ => return None,
        })
    }
}
#[doc = "Replication protocol (protocol).\n"]
#[doc = "Enum - defines an integer enumeration, with values for each entry incrementing by 1, (e.g. 0, 1, 2, 3)"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum WireProtocol {
    #[doc = "Writes complete as soon as they reached the local disk and the local\nsend buffer.\n"]
    A = 1,
    #[doc = "Writes complete as soon as they reached the local disk and all peers\nacknowledged their receipt.\n"]
    B = 2,
    #[doc = "Writes complete as soon as they reached the local disk and all remote\ndisks.\n"]
    C = 3,
}
impl WireProtocol {
    pub fn from_value(value: u64) -> Option<Self> {
        Some(match value {
            1 => Self::A,
            2 => Self::B,
            3 => Self::C,
            _ => return None,
        })
    }
}
#[doc = "Role of a resource on a node. Values mirror enum drbd_role in\nlinux/drbd.h so both netlink dialects share one numbering.\n"]
#[doc = "Enum - defines an integer enumeration, with values for each entry incrementing by 1, (e.g. 0, 1, 2, 3)"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Role {
    #[doc = "The role is not known, for example because the peer is not connected.\n"]
    Unknown = 0,
    #[doc = "The resource is primary; its devices may be opened for writing.\n"]
    Primary = 1,
    #[doc = "The resource is secondary; its devices may not be opened for writing.\n"]
    Secondary = 2,
}
impl Role {
    pub fn from_value(value: u64) -> Option<Self> {
        Some(match value {
            0 => Self::Unknown,
            1 => Self::Primary,
            2 => Self::Secondary,
            _ => return None,
        })
    }
}
#[doc = "State of a connection to a peer. Values mirror enum drbd_conn_state in\nlinux/drbd.h so both netlink dialects share one numbering.\n"]
#[doc = "Enum - defines an integer enumeration, with values for each entry incrementing by 1, (e.g. 0, 1, 2, 3)"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ConnectionState {
    #[doc = "No network configuration is in effect and no connection is attempted.\n"]
    Standalone = 0,
    #[doc = "Temporary state on the way to standalone while the connection is torn\ndown.\n"]
    Disconnecting = 1,
    #[doc = "Temporary state before the next connection attempt starts.\n"]
    Unconnected = 2,
    #[doc = "The connection was dropped because the peer did not answer in time.\n"]
    Timeout = 3,
    #[doc = "The connection was dropped because the transport reported a broken\nconnection.\n"]
    BrokenPipe = 4,
    #[doc = "The connection was dropped because of a network failure.\n"]
    NetworkFailure = 5,
    #[doc = "The connection was dropped because the peer violated the replication\nprotocol.\n"]
    ProtocolError = 6,
    #[doc = "Temporary state while the peer closes the connection.\n"]
    TearDown = 7,
    #[doc = "Listening for the peer and trying to establish a connection.\n"]
    Connecting = 8,
    #[doc = "A connection to the peer is established.\n"]
    Connected = 9,
}
impl ConnectionState {
    pub fn from_value(value: u64) -> Option<Self> {
        Some(match value {
            0 => Self::Standalone,
            1 => Self::Disconnecting,
            2 => Self::Unconnected,
            3 => Self::Timeout,
            4 => Self::BrokenPipe,
            5 => Self::NetworkFailure,
            6 => Self::ProtocolError,
            7 => Self::TearDown,
            8 => Self::Connecting,
            9 => Self::Connected,
            _ => return None,
        })
    }
}
#[doc = "Replication state of a peer device. Values mirror enum drbd_repl_state\nin linux/drbd.h so both netlink dialects share one numbering.\n"]
#[doc = "Enum - defines an integer enumeration, with values for each entry incrementing by 1, (e.g. 0, 1, 2, 3)"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ReplState {
    #[doc = "The peer device does not replicate because the connection is not\nestablished.\n"]
    Off = 9,
    #[doc = "The peer device is connected and replicating normally.\n"]
    Established = 10,
    #[doc = "A full resynchronization was requested by the administrator and starts\nwith this node as the source.\n"]
    StartingSyncS = 11,
    #[doc = "A full resynchronization was requested by the administrator and starts\nwith this node as the target.\n"]
    StartingSyncT = 12,
    #[doc = "Waiting for the bitmap exchange to finish, with this node as the\nprospective resync source.\n"]
    WfBitmapS = 13,
    #[doc = "Waiting for the bitmap exchange to finish, with this node as the\nprospective resync target.\n"]
    WfBitmapT = 14,
    #[doc = "Waiting for the resync source to create and send the new resync UUID.\n"]
    WfSyncUuid = 15,
    #[doc = "Resynchronization is running with this node as the source.\n"]
    SyncSource = 16,
    #[doc = "Resynchronization is running with this node as the target.\n"]
    SyncTarget = 17,
    #[doc = "Online verification is running with this node as the initiator.\n"]
    VerifyS = 18,
    #[doc = "Online verification is running with this node as the target.\n"]
    VerifyT = 19,
    #[doc = "Resynchronization with this node as the source is paused.\n"]
    PausedSyncS = 20,
    #[doc = "Resynchronization with this node as the target is paused.\n"]
    PausedSyncT = 21,
    #[doc = "Replication is suspended because the link is congested; changes are only\nrecorded in the bitmap.\n"]
    Ahead = 22,
    #[doc = "The peer is ahead of this node, which will be resynchronized once the\ncongestion is over.\n"]
    Behind = 23,
}
impl ReplState {
    pub fn from_value(value: u64) -> Option<Self> {
        Some(match value {
            9 => Self::Off,
            10 => Self::Established,
            11 => Self::StartingSyncS,
            12 => Self::StartingSyncT,
            13 => Self::WfBitmapS,
            14 => Self::WfBitmapT,
            15 => Self::WfSyncUuid,
            16 => Self::SyncSource,
            17 => Self::SyncTarget,
            18 => Self::VerifyS,
            19 => Self::VerifyT,
            20 => Self::PausedSyncS,
            21 => Self::PausedSyncT,
            22 => Self::Ahead,
            23 => Self::Behind,
            _ => return None,
        })
    }
}
#[doc = "State of a backing disk. Values mirror enum drbd_disk_state in\nlinux/drbd.h so both netlink dialects share one numbering.\n"]
#[doc = "Enum - defines an integer enumeration, with values for each entry incrementing by 1, (e.g. 0, 1, 2, 3)"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum DiskState {
    #[doc = "No backing device is attached.\n"]
    Diskless = 0,
    #[doc = "The backing device\\'s metadata is being read.\n"]
    Attaching = 1,
    #[doc = "The backing device is being detached while pending I/O drains.\n"]
    Detaching = 2,
    #[doc = "The backing device failed; the disk becomes diskless once the peers have\nbeen told.\n"]
    Failed = 3,
    #[doc = "Attaching while connected, with the data generation still being\nnegotiated with the peers.\n"]
    Negotiating = 4,
    #[doc = "The data is inconsistent, for example because a resynchronization is in\nprogress.\n"]
    Inconsistent = 5,
    #[doc = "The data is consistent but known to be stale relative to a peer.\n"]
    Outdated = 6,
    #[doc = "The disk state is not known; used only for a peer\\'s disk.\n"]
    Unknown = 7,
    #[doc = "The data is consistent but it is not yet known whether it is up to date.\n"]
    Consistent = 8,
    #[doc = "The data is consistent and up to date; only this state allows\napplication I/O.\n"]
    UpToDate = 9,
}
impl DiskState {
    pub fn from_value(value: u64) -> Option<Self> {
        Some(match value {
            0 => Self::Diskless,
            1 => Self::Attaching,
            2 => Self::Detaching,
            3 => Self::Failed,
            4 => Self::Negotiating,
            5 => Self::Inconsistent,
            6 => Self::Outdated,
            7 => Self::Unknown,
            8 => Self::Consistent,
            9 => Self::UpToDate,
            _ => return None,
        })
    }
}
#[doc = "Write ordering method in use for the backing device.\n"]
#[doc = "Enum - defines an integer enumeration, with values for each entry incrementing by 1, (e.g. 0, 1, 2, 3)"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum WriteOrdering {
    #[doc = "No ordering between dependent writes is enforced on the backing device.\n"]
    None = 0,
    #[doc = "Dependent writes are ordered by draining the backing device\\'s request\nqueue.\n"]
    DrainIo = 1,
    #[doc = "Dependent writes are ordered by issuing flushes to the backing device.\n"]
    BdevFlush = 2,
    #[doc = "Dependent writes are ordered with block device barriers, which the Linux\nblock layer no longer provides.\n"]
    BioBarrier = 3,
}
impl WriteOrdering {
    pub fn from_value(value: u64) -> Option<Self> {
        Some(match value {
            0 => Self::None,
            1 => Self::DrainIo,
            2 => Self::BdevFlush,
            3 => Self::BioBarrier,
            _ => return None,
        })
    }
}
#[doc = "Which wait-for-connection timeout applies (timeout-type-get reply).\n"]
#[doc = "Enum - defines an integer enumeration, with values for each entry incrementing by 1, (e.g. 0, 1, 2, 3)"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum TimeoutType {
    #[doc = "The regular wait-for-connection timeout applies.\n"]
    Default = 0,
    #[doc = "The degraded-cluster timeout applies because the local data is not up to\ndate.\n"]
    Degraded = 1,
    #[doc = "The timeout for an outdated peer applies.\n"]
    PeerOutdated = 2,
}
impl TimeoutType {
    pub fn from_value(value: u64) -> Option<Self> {
        Some(match value {
            0 => Self::Default,
            1 => Self::Degraded,
            2 => Self::PeerOutdated,
            _ => return None,
        })
    }
}
#[doc = "Whether a helper notification announces the call or reports its result.\n"]
#[doc = "Enum - defines an integer enumeration, with values for each entry incrementing by 1, (e.g. 0, 1, 2, 3)"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum HelperPhase {
    #[doc = "The helper script is about to be called.\n"]
    Call = 0,
    #[doc = "The helper script has finished and its exit status is reported.\n"]
    Response = 1,
}
impl HelperPhase {
    pub fn from_value(value: u64) -> Option<Self> {
        Some(match value {
            0 => Self::Call,
            1 => Self::Response,
            _ => return None,
        })
    }
}
#[doc = "What a state-change message reports about the object it carries. Values\nare assigned independently of the legacy drbd_notification_type.\n"]
#[doc = "Enum - defines an integer enumeration, with values for each entry incrementing by 1, (e.g. 0, 1, 2, 3)"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum StateChangeAction {
    #[doc = "The object already existed when the notification stream was opened.\n"]
    Exists = 0,
    #[doc = "The object was just created.\n"]
    Create = 1,
    #[doc = "The state or statistics of the object changed.\n"]
    Change = 2,
    #[doc = "The object was destroyed.\n"]
    Destroy = 3,
    #[doc = "The resource was renamed; the new name is carried in resource.new-name.\n"]
    Rename = 4,
}
impl StateChangeAction {
    pub fn from_value(value: u64) -> Option<Self> {
        Some(match value {
            0 => Self::Exists,
            1 => Self::Create,
            2 => Self::Change,
            3 => Self::Destroy,
            4 => Self::Rename,
            _ => return None,
        })
    }
}
#[doc = "Outcome of a state change request. Error values equal the negated\nkernel-internal [SS]()\\* codes (value 3 is unused); success and the\nother non-error outcomes follow at 29 and above.\n"]
#[doc = "Enum - defines an integer enumeration, with values for each entry incrementing by 1, (e.g. 0, 1, 2, 3)"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum StateResult {
    #[doc = "The state change failed for an unspecified reason.\n"]
    UnknownError = 0,
    #[doc = "Multiple primaries are not allowed by the configuration.\n"]
    TwoPrimaries = 1,
    #[doc = "The request needs access to up-to-date data.\n"]
    NoUpToDateDisk = 2,
    #[doc = "Cannot resynchronize without a local disk.\n"]
    NoLocalDisk = 4,
    #[doc = "Cannot resynchronize without a remote disk.\n"]
    NoRemoteDisk = 5,
    #[doc = "Refusing to become outdated while connected.\n"]
    ConnectedOutdates = 6,
    #[doc = "Refusing to become primary while the peer is not outdated.\n"]
    PrimaryNop = 7,
    #[doc = "Cannot start online verify or resynchronization because one is already\nactive.\n"]
    ResyncRunning = 8,
    #[doc = "Cannot disconnect a connection that is already standalone.\n"]
    AlreadyStandalone = 9,
    #[doc = "The cluster-wide state change was refused by a peer node.\n"]
    CwFailedByPeer = 10,
    #[doc = "The device is diskless but the requested operation requires a disk.\n"]
    IsDiskless = 11,
    #[doc = "The device is held open by someone.\n"]
    DeviceInUse = 12,
    #[doc = "There is no network configuration for this connection.\n"]
    NoNetConfig = 13,
    #[doc = "Starting online verify requires a verify algorithm to be configured.\n"]
    NoVerifyAlg = 14,
    #[doc = "The state change requires an established connection.\n"]
    NeedConnection = 15,
    #[doc = "The disk state is lower than outdated.\n"]
    LowerThanOutdated = 16,
    #[doc = "The peer does not support the requested protocol feature.\n"]
    NotSupported = 17,
    #[doc = "The object is in a transient state; retry after the next state change.\n"]
    InTransientState = 18,
    #[doc = "Concurrent state changes were detected and this one was aborted.\n"]
    ConcurrentStateChange = 19,
    #[doc = "Another volume is primary on the peer, which the configuration does not\nallow.\n"]
    OtherVolumePeerPrimary = 20,
    #[doc = "The state change was interrupted.\n"]
    Interrupted = 21,
    #[doc = "The peer may not become primary while the device is open for reading\nhere.\n"]
    PrimaryReader = 22,
    #[doc = "The state change timed out.\n"]
    Timeout = 23,
    #[doc = "Primary nodes must be strongly connected among each other.\n"]
    WeaklyConnected = 24,
    #[doc = "The resource does not have quorum.\n"]
    NoQuorum = 25,
    #[doc = "An intentionally diskless peer may not attach a disk.\n"]
    AttachNoBitmap = 26,
    #[doc = "Disconnect was chosen during the connection handshake.\n"]
    HandshakeDisconnect = 27,
    #[doc = "Retry was chosen during the connection handshake.\n"]
    HandshakeRetry = 28,
    #[doc = "The state change succeeded.\n"]
    Success = 29,
    #[doc = "The requested state was already in effect, so nothing had to be done.\n"]
    NothingToDo = 30,
    #[doc = "The cluster-wide state change succeeded.\n"]
    CwSuccess = 31,
    #[doc = "No cluster-wide state change was necessary.\n"]
    CwNoNeed = 32,
}
impl StateResult {
    pub fn from_value(value: u64) -> Option<Self> {
        Some(match value {
            0 => Self::UnknownError,
            1 => Self::TwoPrimaries,
            2 => Self::NoUpToDateDisk,
            4 => Self::NoLocalDisk,
            5 => Self::NoRemoteDisk,
            6 => Self::ConnectedOutdates,
            7 => Self::PrimaryNop,
            8 => Self::ResyncRunning,
            9 => Self::AlreadyStandalone,
            10 => Self::CwFailedByPeer,
            11 => Self::IsDiskless,
            12 => Self::DeviceInUse,
            13 => Self::NoNetConfig,
            14 => Self::NoVerifyAlg,
            15 => Self::NeedConnection,
            16 => Self::LowerThanOutdated,
            17 => Self::NotSupported,
            18 => Self::InTransientState,
            19 => Self::ConcurrentStateChange,
            20 => Self::OtherVolumePeerPrimary,
            21 => Self::Interrupted,
            22 => Self::PrimaryReader,
            23 => Self::Timeout,
            24 => Self::WeaklyConnected,
            25 => Self::NoQuorum,
            26 => Self::AttachNoBitmap,
            27 => Self::HandshakeDisconnect,
            28 => Self::HandshakeRetry,
            29 => Self::Success,
            30 => Self::NothingToDo,
            31 => Self::CwSuccess,
            32 => Self::CwNoNeed,
            _ => return None,
        })
    }
}
#[derive(Clone)]
pub enum Address {
    #[doc = "Address family (AF_INET or AF_INET6).\n"]
    Family(u16),
    #[doc = "Port number.\n"]
    Port(u16),
    #[doc = "IPv4 address, present when family is AF_INET.\n"]
    Ipv4(std::net::Ipv4Addr),
    #[doc = "IPv6 address, present when family is AF_INET6.\n"]
    Ipv6(std::net::Ipv6Addr),
}
impl<'a> IterableAddress<'a> {
    #[doc = "Address family (AF_INET or AF_INET6).\n"]
    pub fn get_family(&self) -> Result<u16, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Address::Family(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Address",
            "Family",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Port number.\n"]
    pub fn get_port(&self) -> Result<u16, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Address::Port(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Address",
            "Port",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "IPv4 address, present when family is AF_INET.\n"]
    pub fn get_ipv4(&self) -> Result<std::net::Ipv4Addr, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Address::Ipv4(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Address",
            "Ipv4",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "IPv6 address, present when family is AF_INET6.\n"]
    pub fn get_ipv6(&self) -> Result<std::net::Ipv6Addr, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Address::Ipv6(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Address",
            "Ipv6",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl Address {
    pub fn new<'a>(buf: &'a [u8]) -> IterableAddress<'a> {
        IterableAddress::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        let res = match r#type {
            1u16 => "Family",
            2u16 => "Port",
            3u16 => "Ipv4",
            4u16 => "Ipv6",
            _ => return None,
        };
        Some(res)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableAddress<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableAddress<'a> {
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
impl<'a> Iterator for IterableAddress<'a> {
    type Item = Result<Address, ErrorContext>;
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
                1u16 => Address::Family({
                    let res = parse_u16(next);
                    let Some(val) = res else { break };
                    val
                }),
                2u16 => Address::Port({
                    let res = parse_be_u16(next);
                    let Some(val) = res else { break };
                    val
                }),
                3u16 => Address::Ipv4({
                    let res = parse_be_u32(next).map(Ipv4Addr::from_bits);
                    let Some(val) = res else { break };
                    val
                }),
                4u16 => Address::Ipv6({
                    let res = parse_u128(next).map(Ipv6Addr::from_bits);
                    let Some(val) = res else { break };
                    val
                }),
                n if cfg!(any(test, feature = "deny-unknown-attrs")) => break,
                n => continue,
            };
            return Some(Ok(res));
        }
        Some(Err(ErrorContext::new(
            "Address",
            r#type.and_then(|t| Address::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl std::fmt::Debug for IterableAddress<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("Address");
        let mut iter = IterableAddress::with_loc(&[], self.orig_loc);
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
                Address::Family(val) => fmt.field("Family", &val),
                Address::Port(val) => fmt.field("Port", &val),
                Address::Ipv4(val) => fmt.field("Ipv4", &val),
                Address::Ipv6(val) => fmt.field("Ipv6", &val),
            };
        }
        fmt.finish()
    }
}
impl IterableAddress<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("Address", offset));
            return (stack, missing_type.and_then(|t| Address::attr_from_type(t)));
        }
        if cur > offset || cur + self.buf.len() < offset {
            return (stack, None);
        }
        let mut attrs = self.clone();
        let mut last_off = cur + attrs.pos;
        while let Some(attr) = attrs.next() {
            let Ok(attr) = attr else { break };
            match attr {
                Address::Family(val) => {
                    if last_off == offset {
                        stack.push(("Family", last_off));
                        break;
                    }
                }
                Address::Port(val) => {
                    if last_off == offset {
                        stack.push(("Port", last_off));
                        break;
                    }
                }
                Address::Ipv4(val) => {
                    if last_off == offset {
                        stack.push(("Ipv4", last_off));
                        break;
                    }
                }
                Address::Ipv6(val) => {
                    if last_off == offset {
                        stack.push(("Ipv6", last_off));
                        break;
                    }
                }
                _ => {}
            };
            last_off = cur + attrs.pos;
        }
        if !stack.is_empty() {
            stack.push(("Address", cur));
        }
        (stack, None)
    }
}
#[derive(Clone)]
pub enum Context<'a> {
    #[doc = "Resource name.\n"]
    ResourceName(&'a CStr),
    #[doc = "Volume number within the resource.\n"]
    Volume(u32),
    #[doc = "Minor number of the DRBD device.\n"]
    Minor(u32),
    #[doc = "Node ID of the peer.\n"]
    PeerNodeId(u32),
    #[doc = "Name of the connection to the peer.\n"]
    ConnectionName(&'a CStr),
    #[doc = "Local address of a path.\n"]
    MyAddress(IterableAddress<'a>),
    #[doc = "Peer address of a path.\n"]
    PeerAddress(IterableAddress<'a>),
}
impl<'a> IterableContext<'a> {
    #[doc = "Resource name.\n"]
    pub fn get_resource_name(&self) -> Result<&'a CStr, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Context::ResourceName(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Context",
            "ResourceName",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Volume number within the resource.\n"]
    pub fn get_volume(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Context::Volume(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Context",
            "Volume",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Minor number of the DRBD device.\n"]
    pub fn get_minor(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Context::Minor(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Context",
            "Minor",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Node ID of the peer.\n"]
    pub fn get_peer_node_id(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Context::PeerNodeId(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Context",
            "PeerNodeId",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Name of the connection to the peer.\n"]
    pub fn get_connection_name(&self) -> Result<&'a CStr, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Context::ConnectionName(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Context",
            "ConnectionName",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Local address of a path.\n"]
    pub fn get_my_address(&self) -> Result<IterableAddress<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Context::MyAddress(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Context",
            "MyAddress",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Peer address of a path.\n"]
    pub fn get_peer_address(&self) -> Result<IterableAddress<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Context::PeerAddress(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Context",
            "PeerAddress",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl Context<'_> {
    pub fn new<'a>(buf: &'a [u8]) -> IterableContext<'a> {
        IterableContext::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        let res = match r#type {
            1u16 => "ResourceName",
            2u16 => "Volume",
            3u16 => "Minor",
            4u16 => "PeerNodeId",
            5u16 => "ConnectionName",
            6u16 => "MyAddress",
            7u16 => "PeerAddress",
            _ => return None,
        };
        Some(res)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableContext<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableContext<'a> {
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
impl<'a> Iterator for IterableContext<'a> {
    type Item = Result<Context<'a>, ErrorContext>;
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
                1u16 => Context::ResourceName({
                    let res = CStr::from_bytes_with_nul(next).ok();
                    let Some(val) = res else { break };
                    val
                }),
                2u16 => Context::Volume({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                3u16 => Context::Minor({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                4u16 => Context::PeerNodeId({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                5u16 => Context::ConnectionName({
                    let res = CStr::from_bytes_with_nul(next).ok();
                    let Some(val) = res else { break };
                    val
                }),
                6u16 => Context::MyAddress({
                    let res = Some(IterableAddress::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                7u16 => Context::PeerAddress({
                    let res = Some(IterableAddress::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                n if cfg!(any(test, feature = "deny-unknown-attrs")) => break,
                n => continue,
            };
            return Some(Ok(res));
        }
        Some(Err(ErrorContext::new(
            "Context",
            r#type.and_then(|t| Context::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl<'a> std::fmt::Debug for IterableContext<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("Context");
        let mut iter = IterableContext::with_loc(&[], self.orig_loc);
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
                Context::ResourceName(val) => fmt.field("ResourceName", &val),
                Context::Volume(val) => fmt.field("Volume", &val),
                Context::Minor(val) => fmt.field("Minor", &val),
                Context::PeerNodeId(val) => fmt.field("PeerNodeId", &val),
                Context::ConnectionName(val) => fmt.field("ConnectionName", &val),
                Context::MyAddress(val) => fmt.field("MyAddress", &val),
                Context::PeerAddress(val) => fmt.field("PeerAddress", &val),
            };
        }
        fmt.finish()
    }
}
impl IterableContext<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("Context", offset));
            return (stack, missing_type.and_then(|t| Context::attr_from_type(t)));
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
                Context::ResourceName(val) => {
                    if last_off == offset {
                        stack.push(("ResourceName", last_off));
                        break;
                    }
                }
                Context::Volume(val) => {
                    if last_off == offset {
                        stack.push(("Volume", last_off));
                        break;
                    }
                }
                Context::Minor(val) => {
                    if last_off == offset {
                        stack.push(("Minor", last_off));
                        break;
                    }
                }
                Context::PeerNodeId(val) => {
                    if last_off == offset {
                        stack.push(("PeerNodeId", last_off));
                        break;
                    }
                }
                Context::ConnectionName(val) => {
                    if last_off == offset {
                        stack.push(("ConnectionName", last_off));
                        break;
                    }
                }
                Context::MyAddress(val) => {
                    (stack, missing) = val.lookup_attr(offset, missing_type);
                    if !stack.is_empty() {
                        break;
                    }
                }
                Context::PeerAddress(val) => {
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
            stack.push(("Context", cur));
        }
        (stack, missing)
    }
}
#[derive(Clone)]
pub enum DiskConf<'a> {
    #[doc = "Path of the lower-level block device used to store the actual data\n(drbd.conf option disk).\n"]
    BackingDev(&'a CStr),
    #[doc = "Path of the device holding the metadata, or \\\"internal\\\" to store it on\nthe backing device (drbd.conf option meta-disk).\n"]
    MetaDev(&'a CStr),
    #[doc = "Index selecting which slot to use on a shared external metadata device\n(drbd.conf option meta-disk index).\n"]
    MetaDevIdx(i32),
    #[doc = "Size of the backing device to use, in sectors, overriding automatic\ndetection.\n"]
    Size(u64),
    #[doc = "Action to take when the backing disk reports an I/O error.\n\nAssociated type: [`IoErrorPolicy`] (enum)"]
    OnIoError(u32),
    #[doc = "Volume number of another volume that must finish resynchronizing before\nthis one starts (drbd.conf option resync-after).\n"]
    ResyncAfter(i32),
    #[doc = "Number of 4 MiB activity log extents that can be active at the same\ntime.\n"]
    AlExtents(u32),
    #[doc = "Whether to use disk barriers to enforce the ordering of dependent write\nrequests on the backing device.\n"]
    DiskBarrier(u8),
    #[doc = "Whether to issue disk flushes to the backing device to enforce the\nordering of dependent write requests.\n"]
    DiskFlushes(u8),
    #[doc = "Whether to wait for the local I/O request queue to drain before\ncompleting a write barrier.\n"]
    DiskDrain(u8),
    #[doc = "Whether to issue disk flushes to the metadata device.\n"]
    MdFlushes(u8),
    #[doc = "Time to wait for the backing device to complete an I/O request before\ntreating it as failed, in units of 0.1 seconds, or 0 to disable\n(drbd.conf option disk-timeout).\n"]
    DiskTimeout(u32),
    #[doc = "Policy for balancing reads across the local disk and peers (drbd.conf\noption read-balancing).\n\nAssociated type: [`ReadBalancingPolicy`] (enum)"]
    ReadBalancing(u32),
    #[doc = "Legacy watermark, in pages, that used to trigger an early unplug of the\nbacking device\\'s request queue (drbd.conf option unplug-watermark).\n"]
    UnplugWatermark(u32),
    #[doc = "Size, in bytes, of the resync requests DRBD tries to discard on the sync\ntarget when the source\\'s data is all zeroes (drbd.conf option\nrs-discard-granularity).\n"]
    RsDiscardGranularity(u32),
    #[doc = "Whether to update the activity log on disk, at the cost of a full resync\nafter a primary crash if disabled (drbd.conf option al-updates).\n"]
    AlUpdates(u8),
    #[doc = "Whether to treat discarded, aligned blocks on the backing device as\nreading back as zero (drbd.conf option discard-zeroes-if-aligned).\n"]
    DiscardZeroesIfAligned(u8),
    #[doc = "Whether to avoid using the WRITE SAME command on the backing device, for\ndevices that advertise support but fail such requests (drbd.conf option\ndisable-write-same).\n"]
    DisableWriteSame(u8),
    #[doc = "Whether to allocate the on-disk change-tracking bitmap when the device\nis attached (drbd.conf option bitmap).\n"]
    Bitmap(u8),
}
impl<'a> IterableDiskConf<'a> {
    #[doc = "Path of the lower-level block device used to store the actual data\n(drbd.conf option disk).\n"]
    pub fn get_backing_dev(&self) -> Result<&'a CStr, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(DiskConf::BackingDev(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "DiskConf",
            "BackingDev",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Path of the device holding the metadata, or \\\"internal\\\" to store it on\nthe backing device (drbd.conf option meta-disk).\n"]
    pub fn get_meta_dev(&self) -> Result<&'a CStr, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(DiskConf::MetaDev(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "DiskConf",
            "MetaDev",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Index selecting which slot to use on a shared external metadata device\n(drbd.conf option meta-disk index).\n"]
    pub fn get_meta_dev_idx(&self) -> Result<i32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(DiskConf::MetaDevIdx(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "DiskConf",
            "MetaDevIdx",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Size of the backing device to use, in sectors, overriding automatic\ndetection.\n"]
    pub fn get_size(&self) -> Result<u64, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(DiskConf::Size(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "DiskConf",
            "Size",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Action to take when the backing disk reports an I/O error.\n\nAssociated type: [`IoErrorPolicy`] (enum)"]
    pub fn get_on_io_error(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(DiskConf::OnIoError(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "DiskConf",
            "OnIoError",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Volume number of another volume that must finish resynchronizing before\nthis one starts (drbd.conf option resync-after).\n"]
    pub fn get_resync_after(&self) -> Result<i32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(DiskConf::ResyncAfter(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "DiskConf",
            "ResyncAfter",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Number of 4 MiB activity log extents that can be active at the same\ntime.\n"]
    pub fn get_al_extents(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(DiskConf::AlExtents(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "DiskConf",
            "AlExtents",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Whether to use disk barriers to enforce the ordering of dependent write\nrequests on the backing device.\n"]
    pub fn get_disk_barrier(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(DiskConf::DiskBarrier(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "DiskConf",
            "DiskBarrier",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Whether to issue disk flushes to the backing device to enforce the\nordering of dependent write requests.\n"]
    pub fn get_disk_flushes(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(DiskConf::DiskFlushes(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "DiskConf",
            "DiskFlushes",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Whether to wait for the local I/O request queue to drain before\ncompleting a write barrier.\n"]
    pub fn get_disk_drain(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(DiskConf::DiskDrain(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "DiskConf",
            "DiskDrain",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Whether to issue disk flushes to the metadata device.\n"]
    pub fn get_md_flushes(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(DiskConf::MdFlushes(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "DiskConf",
            "MdFlushes",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Time to wait for the backing device to complete an I/O request before\ntreating it as failed, in units of 0.1 seconds, or 0 to disable\n(drbd.conf option disk-timeout).\n"]
    pub fn get_disk_timeout(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(DiskConf::DiskTimeout(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "DiskConf",
            "DiskTimeout",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Policy for balancing reads across the local disk and peers (drbd.conf\noption read-balancing).\n\nAssociated type: [`ReadBalancingPolicy`] (enum)"]
    pub fn get_read_balancing(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(DiskConf::ReadBalancing(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "DiskConf",
            "ReadBalancing",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Legacy watermark, in pages, that used to trigger an early unplug of the\nbacking device\\'s request queue (drbd.conf option unplug-watermark).\n"]
    pub fn get_unplug_watermark(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(DiskConf::UnplugWatermark(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "DiskConf",
            "UnplugWatermark",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Size, in bytes, of the resync requests DRBD tries to discard on the sync\ntarget when the source\\'s data is all zeroes (drbd.conf option\nrs-discard-granularity).\n"]
    pub fn get_rs_discard_granularity(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(DiskConf::RsDiscardGranularity(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "DiskConf",
            "RsDiscardGranularity",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Whether to update the activity log on disk, at the cost of a full resync\nafter a primary crash if disabled (drbd.conf option al-updates).\n"]
    pub fn get_al_updates(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(DiskConf::AlUpdates(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "DiskConf",
            "AlUpdates",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Whether to treat discarded, aligned blocks on the backing device as\nreading back as zero (drbd.conf option discard-zeroes-if-aligned).\n"]
    pub fn get_discard_zeroes_if_aligned(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(DiskConf::DiscardZeroesIfAligned(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "DiskConf",
            "DiscardZeroesIfAligned",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Whether to avoid using the WRITE SAME command on the backing device, for\ndevices that advertise support but fail such requests (drbd.conf option\ndisable-write-same).\n"]
    pub fn get_disable_write_same(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(DiskConf::DisableWriteSame(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "DiskConf",
            "DisableWriteSame",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Whether to allocate the on-disk change-tracking bitmap when the device\nis attached (drbd.conf option bitmap).\n"]
    pub fn get_bitmap(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(DiskConf::Bitmap(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "DiskConf",
            "Bitmap",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl DiskConf<'_> {
    pub fn new<'a>(buf: &'a [u8]) -> IterableDiskConf<'a> {
        IterableDiskConf::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        let res = match r#type {
            1u16 => "BackingDev",
            2u16 => "MetaDev",
            3u16 => "MetaDevIdx",
            4u16 => "Size",
            5u16 => "OnIoError",
            6u16 => "ResyncAfter",
            7u16 => "AlExtents",
            8u16 => "DiskBarrier",
            9u16 => "DiskFlushes",
            10u16 => "DiskDrain",
            11u16 => "MdFlushes",
            12u16 => "DiskTimeout",
            13u16 => "ReadBalancing",
            14u16 => "UnplugWatermark",
            15u16 => "RsDiscardGranularity",
            16u16 => "AlUpdates",
            17u16 => "DiscardZeroesIfAligned",
            18u16 => "DisableWriteSame",
            19u16 => "Bitmap",
            _ => return None,
        };
        Some(res)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableDiskConf<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableDiskConf<'a> {
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
impl<'a> Iterator for IterableDiskConf<'a> {
    type Item = Result<DiskConf<'a>, ErrorContext>;
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
                1u16 => DiskConf::BackingDev({
                    let res = CStr::from_bytes_with_nul(next).ok();
                    let Some(val) = res else { break };
                    val
                }),
                2u16 => DiskConf::MetaDev({
                    let res = CStr::from_bytes_with_nul(next).ok();
                    let Some(val) = res else { break };
                    val
                }),
                3u16 => DiskConf::MetaDevIdx({
                    let res = parse_i32(next);
                    let Some(val) = res else { break };
                    val
                }),
                4u16 => DiskConf::Size({
                    let res = parse_u64(next);
                    let Some(val) = res else { break };
                    val
                }),
                5u16 => DiskConf::OnIoError({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                6u16 => DiskConf::ResyncAfter({
                    let res = parse_i32(next);
                    let Some(val) = res else { break };
                    val
                }),
                7u16 => DiskConf::AlExtents({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                8u16 => DiskConf::DiskBarrier({
                    let res = parse_u8(next);
                    let Some(val) = res else { break };
                    val
                }),
                9u16 => DiskConf::DiskFlushes({
                    let res = parse_u8(next);
                    let Some(val) = res else { break };
                    val
                }),
                10u16 => DiskConf::DiskDrain({
                    let res = parse_u8(next);
                    let Some(val) = res else { break };
                    val
                }),
                11u16 => DiskConf::MdFlushes({
                    let res = parse_u8(next);
                    let Some(val) = res else { break };
                    val
                }),
                12u16 => DiskConf::DiskTimeout({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                13u16 => DiskConf::ReadBalancing({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                14u16 => DiskConf::UnplugWatermark({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                15u16 => DiskConf::RsDiscardGranularity({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                16u16 => DiskConf::AlUpdates({
                    let res = parse_u8(next);
                    let Some(val) = res else { break };
                    val
                }),
                17u16 => DiskConf::DiscardZeroesIfAligned({
                    let res = parse_u8(next);
                    let Some(val) = res else { break };
                    val
                }),
                18u16 => DiskConf::DisableWriteSame({
                    let res = parse_u8(next);
                    let Some(val) = res else { break };
                    val
                }),
                19u16 => DiskConf::Bitmap({
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
            "DiskConf",
            r#type.and_then(|t| DiskConf::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl<'a> std::fmt::Debug for IterableDiskConf<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("DiskConf");
        let mut iter = IterableDiskConf::with_loc(&[], self.orig_loc);
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
                DiskConf::BackingDev(val) => fmt.field("BackingDev", &val),
                DiskConf::MetaDev(val) => fmt.field("MetaDev", &val),
                DiskConf::MetaDevIdx(val) => fmt.field("MetaDevIdx", &val),
                DiskConf::Size(val) => fmt.field("Size", &val),
                DiskConf::OnIoError(val) => fmt.field(
                    "OnIoError",
                    &FormatEnum(val.into(), IoErrorPolicy::from_value),
                ),
                DiskConf::ResyncAfter(val) => fmt.field("ResyncAfter", &val),
                DiskConf::AlExtents(val) => fmt.field("AlExtents", &val),
                DiskConf::DiskBarrier(val) => fmt.field("DiskBarrier", &val),
                DiskConf::DiskFlushes(val) => fmt.field("DiskFlushes", &val),
                DiskConf::DiskDrain(val) => fmt.field("DiskDrain", &val),
                DiskConf::MdFlushes(val) => fmt.field("MdFlushes", &val),
                DiskConf::DiskTimeout(val) => fmt.field("DiskTimeout", &val),
                DiskConf::ReadBalancing(val) => fmt.field(
                    "ReadBalancing",
                    &FormatEnum(val.into(), ReadBalancingPolicy::from_value),
                ),
                DiskConf::UnplugWatermark(val) => fmt.field("UnplugWatermark", &val),
                DiskConf::RsDiscardGranularity(val) => fmt.field("RsDiscardGranularity", &val),
                DiskConf::AlUpdates(val) => fmt.field("AlUpdates", &val),
                DiskConf::DiscardZeroesIfAligned(val) => fmt.field("DiscardZeroesIfAligned", &val),
                DiskConf::DisableWriteSame(val) => fmt.field("DisableWriteSame", &val),
                DiskConf::Bitmap(val) => fmt.field("Bitmap", &val),
            };
        }
        fmt.finish()
    }
}
impl IterableDiskConf<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("DiskConf", offset));
            return (
                stack,
                missing_type.and_then(|t| DiskConf::attr_from_type(t)),
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
                DiskConf::BackingDev(val) => {
                    if last_off == offset {
                        stack.push(("BackingDev", last_off));
                        break;
                    }
                }
                DiskConf::MetaDev(val) => {
                    if last_off == offset {
                        stack.push(("MetaDev", last_off));
                        break;
                    }
                }
                DiskConf::MetaDevIdx(val) => {
                    if last_off == offset {
                        stack.push(("MetaDevIdx", last_off));
                        break;
                    }
                }
                DiskConf::Size(val) => {
                    if last_off == offset {
                        stack.push(("Size", last_off));
                        break;
                    }
                }
                DiskConf::OnIoError(val) => {
                    if last_off == offset {
                        stack.push(("OnIoError", last_off));
                        break;
                    }
                }
                DiskConf::ResyncAfter(val) => {
                    if last_off == offset {
                        stack.push(("ResyncAfter", last_off));
                        break;
                    }
                }
                DiskConf::AlExtents(val) => {
                    if last_off == offset {
                        stack.push(("AlExtents", last_off));
                        break;
                    }
                }
                DiskConf::DiskBarrier(val) => {
                    if last_off == offset {
                        stack.push(("DiskBarrier", last_off));
                        break;
                    }
                }
                DiskConf::DiskFlushes(val) => {
                    if last_off == offset {
                        stack.push(("DiskFlushes", last_off));
                        break;
                    }
                }
                DiskConf::DiskDrain(val) => {
                    if last_off == offset {
                        stack.push(("DiskDrain", last_off));
                        break;
                    }
                }
                DiskConf::MdFlushes(val) => {
                    if last_off == offset {
                        stack.push(("MdFlushes", last_off));
                        break;
                    }
                }
                DiskConf::DiskTimeout(val) => {
                    if last_off == offset {
                        stack.push(("DiskTimeout", last_off));
                        break;
                    }
                }
                DiskConf::ReadBalancing(val) => {
                    if last_off == offset {
                        stack.push(("ReadBalancing", last_off));
                        break;
                    }
                }
                DiskConf::UnplugWatermark(val) => {
                    if last_off == offset {
                        stack.push(("UnplugWatermark", last_off));
                        break;
                    }
                }
                DiskConf::RsDiscardGranularity(val) => {
                    if last_off == offset {
                        stack.push(("RsDiscardGranularity", last_off));
                        break;
                    }
                }
                DiskConf::AlUpdates(val) => {
                    if last_off == offset {
                        stack.push(("AlUpdates", last_off));
                        break;
                    }
                }
                DiskConf::DiscardZeroesIfAligned(val) => {
                    if last_off == offset {
                        stack.push(("DiscardZeroesIfAligned", last_off));
                        break;
                    }
                }
                DiskConf::DisableWriteSame(val) => {
                    if last_off == offset {
                        stack.push(("DisableWriteSame", last_off));
                        break;
                    }
                }
                DiskConf::Bitmap(val) => {
                    if last_off == offset {
                        stack.push(("Bitmap", last_off));
                        break;
                    }
                }
                _ => {}
            };
            last_off = cur + attrs.pos;
        }
        if !stack.is_empty() {
            stack.push(("DiskConf", cur));
        }
        (stack, None)
    }
}
#[derive(Clone)]
pub enum ResourceOpts<'a> {
    #[doc = "CPU affinity mask for DRBD kernel threads, as a hexadecimal number\n(drbd.conf option cpu-mask).\n"]
    CpuMask(&'a CStr),
    #[doc = "Action to take for I/O requests when the requested data is not available\nlocally or remotely (drbd.conf option on-no-data-accessible).\n\nAssociated type: [`OnNoDataPolicy`] (enum)"]
    OnNoDataAccessible(u32),
    #[doc = "Whether to automatically promote the resource to primary when one of its\ndevices is mounted or opened for writing (drbd.conf option\nauto-promote).\n"]
    AutoPromote(u8),
    #[doc = "Unique node identifier of this node in the cluster (drbd.conf option\nnode-id).\n"]
    NodeId(u32),
    #[doc = "Amount of data, in sectors, DRBD replicates to a peer before issuing a\npeer-ack for the writes in between (drbd.conf option peer-ack-window).\n"]
    PeerAckWindow(u32),
    #[doc = "Timeout for a two-phase-commit cluster-wide state transition, in units\nof 0.1 seconds (drbd.conf option twopc-timeout).\n"]
    TwopcTimeout(u32),
    #[doc = "Base multiplier for the randomized backoff before retrying a conflicting\ntwo-phase commit, in units of 0.1 seconds (drbd.conf option\ntwopc-retry-timeout).\n"]
    TwopcRetryTimeout(u32),
    #[doc = "How long to wait after the last write request before sending a peer-ack\nif no new write request arrives, in milliseconds (drbd.conf option\npeer-ack-delay).\n"]
    PeerAckDelay(u32),
    #[doc = "How long to wait for a device to become promotable when auto-promote\ntriggers a promotion, in units of 0.1 seconds (drbd.conf option\nauto-promote-timeout).\n"]
    AutoPromoteTimeout(u32),
    #[doc = "Maximum number of outstanding application I/O requests on a DRBD device.\n"]
    MaxIoDepth(u32),
    #[doc = "Number of nodes with an accessible disk required for a partition to have\nquorum, or a symbolic value (off, majority, all) (drbd.conf option\nquorum).\n"]
    Quorum(i32),
    #[doc = "Action to take for I/O requests when the resource loses quorum\n(drbd.conf option on-no-quorum).\n\nAssociated type: [`OnNoQuorumPolicy`] (enum)"]
    OnNoQuorum(u32),
    #[doc = "Minimum number of nodes with an UpToDate disk required for a partition\nto gain quorum, or a symbolic value (off, majority, all) (drbd.conf\noption quorum-minimum-redundancy).\n"]
    QuorumMinRedundancy(i32),
    #[doc = "Action to take for a primary that has I/O suspended due to lost quorum\nand learns its data is outdated (drbd.conf option\non-suspended-primary-outdated).\n\nAssociated type: [`OnSuspendedPrimaryOutdatedPolicy`] (enum)"]
    OnSuspendedPrimaryOutdated(u32),
    #[doc = "Whether the resource operates in DRBD 8 compatible, single-peer mode.\n"]
    Drbd8CompatMode(u8),
    #[doc = "Whether to explicitly (re)configure DRBD 8 compatible mode whenever a\nnode-id is configured, instead of leaving it unchanged.\n"]
    ExplicitDrbd8Compat(u8),
}
impl<'a> IterableResourceOpts<'a> {
    #[doc = "CPU affinity mask for DRBD kernel threads, as a hexadecimal number\n(drbd.conf option cpu-mask).\n"]
    pub fn get_cpu_mask(&self) -> Result<&'a CStr, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(ResourceOpts::CpuMask(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "ResourceOpts",
            "CpuMask",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Action to take for I/O requests when the requested data is not available\nlocally or remotely (drbd.conf option on-no-data-accessible).\n\nAssociated type: [`OnNoDataPolicy`] (enum)"]
    pub fn get_on_no_data_accessible(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(ResourceOpts::OnNoDataAccessible(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "ResourceOpts",
            "OnNoDataAccessible",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Whether to automatically promote the resource to primary when one of its\ndevices is mounted or opened for writing (drbd.conf option\nauto-promote).\n"]
    pub fn get_auto_promote(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(ResourceOpts::AutoPromote(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "ResourceOpts",
            "AutoPromote",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Unique node identifier of this node in the cluster (drbd.conf option\nnode-id).\n"]
    pub fn get_node_id(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(ResourceOpts::NodeId(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "ResourceOpts",
            "NodeId",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Amount of data, in sectors, DRBD replicates to a peer before issuing a\npeer-ack for the writes in between (drbd.conf option peer-ack-window).\n"]
    pub fn get_peer_ack_window(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(ResourceOpts::PeerAckWindow(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "ResourceOpts",
            "PeerAckWindow",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Timeout for a two-phase-commit cluster-wide state transition, in units\nof 0.1 seconds (drbd.conf option twopc-timeout).\n"]
    pub fn get_twopc_timeout(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(ResourceOpts::TwopcTimeout(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "ResourceOpts",
            "TwopcTimeout",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Base multiplier for the randomized backoff before retrying a conflicting\ntwo-phase commit, in units of 0.1 seconds (drbd.conf option\ntwopc-retry-timeout).\n"]
    pub fn get_twopc_retry_timeout(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(ResourceOpts::TwopcRetryTimeout(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "ResourceOpts",
            "TwopcRetryTimeout",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "How long to wait after the last write request before sending a peer-ack\nif no new write request arrives, in milliseconds (drbd.conf option\npeer-ack-delay).\n"]
    pub fn get_peer_ack_delay(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(ResourceOpts::PeerAckDelay(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "ResourceOpts",
            "PeerAckDelay",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "How long to wait for a device to become promotable when auto-promote\ntriggers a promotion, in units of 0.1 seconds (drbd.conf option\nauto-promote-timeout).\n"]
    pub fn get_auto_promote_timeout(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(ResourceOpts::AutoPromoteTimeout(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "ResourceOpts",
            "AutoPromoteTimeout",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Maximum number of outstanding application I/O requests on a DRBD device.\n"]
    pub fn get_max_io_depth(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(ResourceOpts::MaxIoDepth(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "ResourceOpts",
            "MaxIoDepth",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Number of nodes with an accessible disk required for a partition to have\nquorum, or a symbolic value (off, majority, all) (drbd.conf option\nquorum).\n"]
    pub fn get_quorum(&self) -> Result<i32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(ResourceOpts::Quorum(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "ResourceOpts",
            "Quorum",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Action to take for I/O requests when the resource loses quorum\n(drbd.conf option on-no-quorum).\n\nAssociated type: [`OnNoQuorumPolicy`] (enum)"]
    pub fn get_on_no_quorum(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(ResourceOpts::OnNoQuorum(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "ResourceOpts",
            "OnNoQuorum",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Minimum number of nodes with an UpToDate disk required for a partition\nto gain quorum, or a symbolic value (off, majority, all) (drbd.conf\noption quorum-minimum-redundancy).\n"]
    pub fn get_quorum_min_redundancy(&self) -> Result<i32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(ResourceOpts::QuorumMinRedundancy(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "ResourceOpts",
            "QuorumMinRedundancy",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Action to take for a primary that has I/O suspended due to lost quorum\nand learns its data is outdated (drbd.conf option\non-suspended-primary-outdated).\n\nAssociated type: [`OnSuspendedPrimaryOutdatedPolicy`] (enum)"]
    pub fn get_on_suspended_primary_outdated(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(ResourceOpts::OnSuspendedPrimaryOutdated(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "ResourceOpts",
            "OnSuspendedPrimaryOutdated",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Whether the resource operates in DRBD 8 compatible, single-peer mode.\n"]
    pub fn get_drbd8_compat_mode(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(ResourceOpts::Drbd8CompatMode(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "ResourceOpts",
            "Drbd8CompatMode",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Whether to explicitly (re)configure DRBD 8 compatible mode whenever a\nnode-id is configured, instead of leaving it unchanged.\n"]
    pub fn get_explicit_drbd8_compat(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(ResourceOpts::ExplicitDrbd8Compat(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "ResourceOpts",
            "ExplicitDrbd8Compat",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl ResourceOpts<'_> {
    pub fn new<'a>(buf: &'a [u8]) -> IterableResourceOpts<'a> {
        IterableResourceOpts::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        let res = match r#type {
            1u16 => "CpuMask",
            2u16 => "OnNoDataAccessible",
            3u16 => "AutoPromote",
            4u16 => "NodeId",
            5u16 => "PeerAckWindow",
            6u16 => "TwopcTimeout",
            7u16 => "TwopcRetryTimeout",
            8u16 => "PeerAckDelay",
            9u16 => "AutoPromoteTimeout",
            10u16 => "MaxIoDepth",
            11u16 => "Quorum",
            12u16 => "OnNoQuorum",
            13u16 => "QuorumMinRedundancy",
            14u16 => "OnSuspendedPrimaryOutdated",
            15u16 => "Drbd8CompatMode",
            16u16 => "ExplicitDrbd8Compat",
            _ => return None,
        };
        Some(res)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableResourceOpts<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableResourceOpts<'a> {
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
impl<'a> Iterator for IterableResourceOpts<'a> {
    type Item = Result<ResourceOpts<'a>, ErrorContext>;
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
                1u16 => ResourceOpts::CpuMask({
                    let res = CStr::from_bytes_with_nul(next).ok();
                    let Some(val) = res else { break };
                    val
                }),
                2u16 => ResourceOpts::OnNoDataAccessible({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                3u16 => ResourceOpts::AutoPromote({
                    let res = parse_u8(next);
                    let Some(val) = res else { break };
                    val
                }),
                4u16 => ResourceOpts::NodeId({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                5u16 => ResourceOpts::PeerAckWindow({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                6u16 => ResourceOpts::TwopcTimeout({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                7u16 => ResourceOpts::TwopcRetryTimeout({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                8u16 => ResourceOpts::PeerAckDelay({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                9u16 => ResourceOpts::AutoPromoteTimeout({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                10u16 => ResourceOpts::MaxIoDepth({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                11u16 => ResourceOpts::Quorum({
                    let res = parse_i32(next);
                    let Some(val) = res else { break };
                    val
                }),
                12u16 => ResourceOpts::OnNoQuorum({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                13u16 => ResourceOpts::QuorumMinRedundancy({
                    let res = parse_i32(next);
                    let Some(val) = res else { break };
                    val
                }),
                14u16 => ResourceOpts::OnSuspendedPrimaryOutdated({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                15u16 => ResourceOpts::Drbd8CompatMode({
                    let res = parse_u8(next);
                    let Some(val) = res else { break };
                    val
                }),
                16u16 => ResourceOpts::ExplicitDrbd8Compat({
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
            "ResourceOpts",
            r#type.and_then(|t| ResourceOpts::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl<'a> std::fmt::Debug for IterableResourceOpts<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("ResourceOpts");
        let mut iter = IterableResourceOpts::with_loc(&[], self.orig_loc);
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
                ResourceOpts::CpuMask(val) => fmt.field("CpuMask", &val),
                ResourceOpts::OnNoDataAccessible(val) => fmt.field(
                    "OnNoDataAccessible",
                    &FormatEnum(val.into(), OnNoDataPolicy::from_value),
                ),
                ResourceOpts::AutoPromote(val) => fmt.field("AutoPromote", &val),
                ResourceOpts::NodeId(val) => fmt.field("NodeId", &val),
                ResourceOpts::PeerAckWindow(val) => fmt.field("PeerAckWindow", &val),
                ResourceOpts::TwopcTimeout(val) => fmt.field("TwopcTimeout", &val),
                ResourceOpts::TwopcRetryTimeout(val) => fmt.field("TwopcRetryTimeout", &val),
                ResourceOpts::PeerAckDelay(val) => fmt.field("PeerAckDelay", &val),
                ResourceOpts::AutoPromoteTimeout(val) => fmt.field("AutoPromoteTimeout", &val),
                ResourceOpts::MaxIoDepth(val) => fmt.field("MaxIoDepth", &val),
                ResourceOpts::Quorum(val) => fmt.field("Quorum", &val),
                ResourceOpts::OnNoQuorum(val) => fmt.field(
                    "OnNoQuorum",
                    &FormatEnum(val.into(), OnNoQuorumPolicy::from_value),
                ),
                ResourceOpts::QuorumMinRedundancy(val) => fmt.field("QuorumMinRedundancy", &val),
                ResourceOpts::OnSuspendedPrimaryOutdated(val) => fmt.field(
                    "OnSuspendedPrimaryOutdated",
                    &FormatEnum(val.into(), OnSuspendedPrimaryOutdatedPolicy::from_value),
                ),
                ResourceOpts::Drbd8CompatMode(val) => fmt.field("Drbd8CompatMode", &val),
                ResourceOpts::ExplicitDrbd8Compat(val) => fmt.field("ExplicitDrbd8Compat", &val),
            };
        }
        fmt.finish()
    }
}
impl IterableResourceOpts<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("ResourceOpts", offset));
            return (
                stack,
                missing_type.and_then(|t| ResourceOpts::attr_from_type(t)),
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
                ResourceOpts::CpuMask(val) => {
                    if last_off == offset {
                        stack.push(("CpuMask", last_off));
                        break;
                    }
                }
                ResourceOpts::OnNoDataAccessible(val) => {
                    if last_off == offset {
                        stack.push(("OnNoDataAccessible", last_off));
                        break;
                    }
                }
                ResourceOpts::AutoPromote(val) => {
                    if last_off == offset {
                        stack.push(("AutoPromote", last_off));
                        break;
                    }
                }
                ResourceOpts::NodeId(val) => {
                    if last_off == offset {
                        stack.push(("NodeId", last_off));
                        break;
                    }
                }
                ResourceOpts::PeerAckWindow(val) => {
                    if last_off == offset {
                        stack.push(("PeerAckWindow", last_off));
                        break;
                    }
                }
                ResourceOpts::TwopcTimeout(val) => {
                    if last_off == offset {
                        stack.push(("TwopcTimeout", last_off));
                        break;
                    }
                }
                ResourceOpts::TwopcRetryTimeout(val) => {
                    if last_off == offset {
                        stack.push(("TwopcRetryTimeout", last_off));
                        break;
                    }
                }
                ResourceOpts::PeerAckDelay(val) => {
                    if last_off == offset {
                        stack.push(("PeerAckDelay", last_off));
                        break;
                    }
                }
                ResourceOpts::AutoPromoteTimeout(val) => {
                    if last_off == offset {
                        stack.push(("AutoPromoteTimeout", last_off));
                        break;
                    }
                }
                ResourceOpts::MaxIoDepth(val) => {
                    if last_off == offset {
                        stack.push(("MaxIoDepth", last_off));
                        break;
                    }
                }
                ResourceOpts::Quorum(val) => {
                    if last_off == offset {
                        stack.push(("Quorum", last_off));
                        break;
                    }
                }
                ResourceOpts::OnNoQuorum(val) => {
                    if last_off == offset {
                        stack.push(("OnNoQuorum", last_off));
                        break;
                    }
                }
                ResourceOpts::QuorumMinRedundancy(val) => {
                    if last_off == offset {
                        stack.push(("QuorumMinRedundancy", last_off));
                        break;
                    }
                }
                ResourceOpts::OnSuspendedPrimaryOutdated(val) => {
                    if last_off == offset {
                        stack.push(("OnSuspendedPrimaryOutdated", last_off));
                        break;
                    }
                }
                ResourceOpts::Drbd8CompatMode(val) => {
                    if last_off == offset {
                        stack.push(("Drbd8CompatMode", last_off));
                        break;
                    }
                }
                ResourceOpts::ExplicitDrbd8Compat(val) => {
                    if last_off == offset {
                        stack.push(("ExplicitDrbd8Compat", last_off));
                        break;
                    }
                }
                _ => {}
            };
            last_off = cur + attrs.pos;
        }
        if !stack.is_empty() {
            stack.push(("ResourceOpts", cur));
        }
        (stack, None)
    }
}
#[derive(Clone)]
pub enum NetConf<'a> {
    #[doc = "Shared secret used for peer authentication (drbd.conf option\nshared-secret).\n"]
    SharedSecret(&'a CStr),
    #[doc = "Hash-based message authentication code (HMAC) algorithm used for peer\nauthentication (drbd.conf option cram-hmac-alg).\n"]
    CramHmacAlg(&'a CStr),
    #[doc = "Data integrity algorithm used, in addition to the TCP/IP checksum, to\ndetect corruption of data received over the network (drbd.conf option\ndata-integrity-alg).\n"]
    IntegrityAlg(&'a CStr),
    #[doc = "Hash algorithm used to compute and compare block checksums during online\nverification (drbd.conf option verify-alg).\n"]
    VerifyAlg(&'a CStr),
    #[doc = "Hash algorithm used to compare blocks before resending identical data\nduring a resync (drbd.conf option csums-alg).\n"]
    CsumsAlg(&'a CStr),
    #[doc = "Replication protocol used on this connection.\n\nAssociated type: [`WireProtocol`] (enum)"]
    Protocol(u32),
    #[doc = "How long to wait before retrying to establish a connection to the peer,\nin seconds (drbd.conf option connect-int).\n"]
    ConnectInt(u32),
    #[doc = "Timeout for replies over the network before considering a peer dead, in\nunits of 0.1 seconds (drbd.conf option timeout).\n"]
    Timeout(u32),
    #[doc = "Interval of connection idle time after which DRBD sends a keep-alive\npacket, in seconds (drbd.conf option ping-int).\n"]
    PingInt(u32),
    #[doc = "Timeout for replies to keep-alive packets, in units of 0.1 seconds\n(drbd.conf option ping-timeout).\n"]
    PingTimeo(u32),
    #[doc = "Size of the TCP/IP send buffer, in bytes, or 0 to size it automatically\n(drbd.conf option sndbuf-size).\n"]
    SndbufSize(u32),
    #[doc = "Size of the TCP/IP receive buffer, in bytes, or 0 to size it\nautomatically (drbd.conf option rcvbuf-size).\n"]
    RcvbufSize(u32),
    #[doc = "Number of times the timeout may be exceeded before a non-responding peer\nis excluded from the cluster (drbd.conf option ko-count).\n"]
    KoCount(u32),
    #[doc = "Maximum number of write requests DRBD may issue before issuing a write\nbarrier (drbd.conf option max-epoch-size).\n"]
    MaxEpochSize(u32),
    #[doc = "Split-brain recovery policy when neither node is primary.\n\nAssociated type: [`AfterSbPolicy`] (enum)"]
    AfterSb0pri(u32),
    #[doc = "Split-brain recovery policy when exactly one node is primary.\n\nAssociated type: [`AfterSbPolicy`] (enum)"]
    AfterSb1pri(u32),
    #[doc = "Split-brain recovery policy when both nodes are primary.\n\nAssociated type: [`AfterSbPolicy`] (enum)"]
    AfterSb2pri(u32),
    #[doc = "Policy for resolving a role/resync conflict left over from a split-brain\nrecovery (drbd.conf option rr-conflict).\n\nAssociated type: [`AfterSbPolicy`] (enum)"]
    RrConflict(u32),
    #[doc = "Action to take when the replication link\\'s TCP send queue is congested\n(drbd.conf option on-congestion).\n\nAssociated type: [`OnCongestionPolicy`] (enum)"]
    OnCongestion(u32),
    #[doc = "Amount of in-flight data, in sectors, allowed on this connection before\nswitching into ahead/behind mode, or 0 to disable (drbd.conf option\ncongestion-fill).\n"]
    CongFill(u32),
    #[doc = "Number of active bitmap extents allowed before switching into\nahead/behind mode (drbd.conf option congestion-extents).\n"]
    CongExtents(u32),
    #[doc = "Whether to allow both nodes of this connection to be primary at the same\ntime (drbd.conf option allow-two-primaries).\n"]
    TwoPrimaries(u8),
    #[doc = "Whether to use the TCP_CORK socket option to coalesce partial messages\ninto fewer, bigger packets (drbd.conf option tcp-cork).\n"]
    TcpCork(u8),
    #[doc = "Whether to apply the automatic split-brain recovery policies even when\nthe UUIDs indicate a third node may be involved (drbd.conf option\nalways-asbp).\n"]
    AlwaysAsbp(u8),
    #[doc = "Whether to run-length encode the bitmap exchanged with the peer when a\nconnection is established (drbd.conf option use-rle).\n"]
    UseRle(u8),
    #[doc = "Fencing policy used to avoid a split-brain when both nodes are primary\nand disconnected.\n\nAssociated type: [`FencingPolicy`] (enum)"]
    Fencing(u32),
    #[doc = "Name of the connection this net-conf applies to.\n"]
    ConnectionName(&'a CStr),
    #[doc = "Whether to compare block checksums only for the resync after a primary\ncrash, rather than for every resync (drbd.conf option\ncsums-after-crash-only).\n"]
    CsumsAfterCrashOnly(u8),
    #[doc = "Timeout used to poll the send/receive socket for a broken TCP connection\nwhen DRBD-proxy is in use, in units of 0.1 seconds (drbd.conf option\nsocket-check-timeout).\n"]
    SockCheckTimeo(u32),
    #[doc = "Name of the network transport module used for this connection, e.g. tcp\nor rdma (drbd.conf option transport).\n"]
    TransportName(&'a CStr),
    #[doc = "Maximum number of pages used for receive buffers and internal\nresync/verify buffers per device (drbd.conf option max-buffers).\n"]
    MaxBuffers(u32),
    #[doc = "Whether to allow reads to be served from a peer\\'s disk when the local\ndisk is not up to date (drbd.conf option allow-remote-read).\n"]
    AllowRemoteRead(u8),
    #[doc = "Whether to encrypt this connection with TLS (drbd.conf option tls).\n"]
    Tls(u8),
    #[doc = "Serial number of the kernel keyring key holding the private key used for\nTLS encryption (drbd.conf option tls-privkey).\n"]
    TlsPrivkey(i32),
    #[doc = "Serial number of the kernel keyring key holding the certificate used for\nTLS encryption (drbd.conf option tls-certificate).\n"]
    TlsCertificate(i32),
    #[doc = "Serial number of the kernel keyring used to look up the TLS key material\n(drbd.conf option tls-keyring).\n"]
    TlsKeyring(i32),
    #[doc = "Whether to load-balance traffic for this connection across all\nconfigured paths instead of using only one at a time (drbd.conf option\nload-balance-paths).\n"]
    LoadBalancePaths(u8),
    #[doc = "Size, in bytes, of the RDMA transport\\'s control-stream receive buffer,\nor 0 to derive it from rcvbuf-size (drbd.conf option\nrdma-ctrl-rcvbuf-size).\n"]
    RdmaCtrlRcvbufSize(u32),
    #[doc = "Size, in bytes, of the RDMA transport\\'s control-stream send buffer, or\n0 to derive it from sndbuf-size (drbd.conf option\nrdma-ctrl-sndbuf-size).\n"]
    RdmaCtrlSndbufSize(u32),
}
impl<'a> IterableNetConf<'a> {
    #[doc = "Shared secret used for peer authentication (drbd.conf option\nshared-secret).\n"]
    pub fn get_shared_secret(&self) -> Result<&'a CStr, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(NetConf::SharedSecret(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "NetConf",
            "SharedSecret",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Hash-based message authentication code (HMAC) algorithm used for peer\nauthentication (drbd.conf option cram-hmac-alg).\n"]
    pub fn get_cram_hmac_alg(&self) -> Result<&'a CStr, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(NetConf::CramHmacAlg(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "NetConf",
            "CramHmacAlg",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Data integrity algorithm used, in addition to the TCP/IP checksum, to\ndetect corruption of data received over the network (drbd.conf option\ndata-integrity-alg).\n"]
    pub fn get_integrity_alg(&self) -> Result<&'a CStr, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(NetConf::IntegrityAlg(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "NetConf",
            "IntegrityAlg",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Hash algorithm used to compute and compare block checksums during online\nverification (drbd.conf option verify-alg).\n"]
    pub fn get_verify_alg(&self) -> Result<&'a CStr, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(NetConf::VerifyAlg(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "NetConf",
            "VerifyAlg",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Hash algorithm used to compare blocks before resending identical data\nduring a resync (drbd.conf option csums-alg).\n"]
    pub fn get_csums_alg(&self) -> Result<&'a CStr, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(NetConf::CsumsAlg(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "NetConf",
            "CsumsAlg",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Replication protocol used on this connection.\n\nAssociated type: [`WireProtocol`] (enum)"]
    pub fn get_protocol(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(NetConf::Protocol(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "NetConf",
            "Protocol",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "How long to wait before retrying to establish a connection to the peer,\nin seconds (drbd.conf option connect-int).\n"]
    pub fn get_connect_int(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(NetConf::ConnectInt(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "NetConf",
            "ConnectInt",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Timeout for replies over the network before considering a peer dead, in\nunits of 0.1 seconds (drbd.conf option timeout).\n"]
    pub fn get_timeout(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(NetConf::Timeout(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "NetConf",
            "Timeout",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Interval of connection idle time after which DRBD sends a keep-alive\npacket, in seconds (drbd.conf option ping-int).\n"]
    pub fn get_ping_int(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(NetConf::PingInt(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "NetConf",
            "PingInt",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Timeout for replies to keep-alive packets, in units of 0.1 seconds\n(drbd.conf option ping-timeout).\n"]
    pub fn get_ping_timeo(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(NetConf::PingTimeo(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "NetConf",
            "PingTimeo",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Size of the TCP/IP send buffer, in bytes, or 0 to size it automatically\n(drbd.conf option sndbuf-size).\n"]
    pub fn get_sndbuf_size(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(NetConf::SndbufSize(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "NetConf",
            "SndbufSize",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Size of the TCP/IP receive buffer, in bytes, or 0 to size it\nautomatically (drbd.conf option rcvbuf-size).\n"]
    pub fn get_rcvbuf_size(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(NetConf::RcvbufSize(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "NetConf",
            "RcvbufSize",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Number of times the timeout may be exceeded before a non-responding peer\nis excluded from the cluster (drbd.conf option ko-count).\n"]
    pub fn get_ko_count(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(NetConf::KoCount(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "NetConf",
            "KoCount",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Maximum number of write requests DRBD may issue before issuing a write\nbarrier (drbd.conf option max-epoch-size).\n"]
    pub fn get_max_epoch_size(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(NetConf::MaxEpochSize(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "NetConf",
            "MaxEpochSize",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Split-brain recovery policy when neither node is primary.\n\nAssociated type: [`AfterSbPolicy`] (enum)"]
    pub fn get_after_sb_0pri(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(NetConf::AfterSb0pri(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "NetConf",
            "AfterSb0pri",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Split-brain recovery policy when exactly one node is primary.\n\nAssociated type: [`AfterSbPolicy`] (enum)"]
    pub fn get_after_sb_1pri(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(NetConf::AfterSb1pri(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "NetConf",
            "AfterSb1pri",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Split-brain recovery policy when both nodes are primary.\n\nAssociated type: [`AfterSbPolicy`] (enum)"]
    pub fn get_after_sb_2pri(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(NetConf::AfterSb2pri(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "NetConf",
            "AfterSb2pri",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Policy for resolving a role/resync conflict left over from a split-brain\nrecovery (drbd.conf option rr-conflict).\n\nAssociated type: [`AfterSbPolicy`] (enum)"]
    pub fn get_rr_conflict(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(NetConf::RrConflict(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "NetConf",
            "RrConflict",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Action to take when the replication link\\'s TCP send queue is congested\n(drbd.conf option on-congestion).\n\nAssociated type: [`OnCongestionPolicy`] (enum)"]
    pub fn get_on_congestion(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(NetConf::OnCongestion(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "NetConf",
            "OnCongestion",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Amount of in-flight data, in sectors, allowed on this connection before\nswitching into ahead/behind mode, or 0 to disable (drbd.conf option\ncongestion-fill).\n"]
    pub fn get_cong_fill(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(NetConf::CongFill(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "NetConf",
            "CongFill",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Number of active bitmap extents allowed before switching into\nahead/behind mode (drbd.conf option congestion-extents).\n"]
    pub fn get_cong_extents(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(NetConf::CongExtents(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "NetConf",
            "CongExtents",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Whether to allow both nodes of this connection to be primary at the same\ntime (drbd.conf option allow-two-primaries).\n"]
    pub fn get_two_primaries(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(NetConf::TwoPrimaries(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "NetConf",
            "TwoPrimaries",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Whether to use the TCP_CORK socket option to coalesce partial messages\ninto fewer, bigger packets (drbd.conf option tcp-cork).\n"]
    pub fn get_tcp_cork(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(NetConf::TcpCork(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "NetConf",
            "TcpCork",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Whether to apply the automatic split-brain recovery policies even when\nthe UUIDs indicate a third node may be involved (drbd.conf option\nalways-asbp).\n"]
    pub fn get_always_asbp(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(NetConf::AlwaysAsbp(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "NetConf",
            "AlwaysAsbp",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Whether to run-length encode the bitmap exchanged with the peer when a\nconnection is established (drbd.conf option use-rle).\n"]
    pub fn get_use_rle(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(NetConf::UseRle(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "NetConf",
            "UseRle",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Fencing policy used to avoid a split-brain when both nodes are primary\nand disconnected.\n\nAssociated type: [`FencingPolicy`] (enum)"]
    pub fn get_fencing(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(NetConf::Fencing(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "NetConf",
            "Fencing",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Name of the connection this net-conf applies to.\n"]
    pub fn get_connection_name(&self) -> Result<&'a CStr, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(NetConf::ConnectionName(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "NetConf",
            "ConnectionName",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Whether to compare block checksums only for the resync after a primary\ncrash, rather than for every resync (drbd.conf option\ncsums-after-crash-only).\n"]
    pub fn get_csums_after_crash_only(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(NetConf::CsumsAfterCrashOnly(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "NetConf",
            "CsumsAfterCrashOnly",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Timeout used to poll the send/receive socket for a broken TCP connection\nwhen DRBD-proxy is in use, in units of 0.1 seconds (drbd.conf option\nsocket-check-timeout).\n"]
    pub fn get_sock_check_timeo(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(NetConf::SockCheckTimeo(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "NetConf",
            "SockCheckTimeo",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Name of the network transport module used for this connection, e.g. tcp\nor rdma (drbd.conf option transport).\n"]
    pub fn get_transport_name(&self) -> Result<&'a CStr, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(NetConf::TransportName(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "NetConf",
            "TransportName",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Maximum number of pages used for receive buffers and internal\nresync/verify buffers per device (drbd.conf option max-buffers).\n"]
    pub fn get_max_buffers(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(NetConf::MaxBuffers(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "NetConf",
            "MaxBuffers",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Whether to allow reads to be served from a peer\\'s disk when the local\ndisk is not up to date (drbd.conf option allow-remote-read).\n"]
    pub fn get_allow_remote_read(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(NetConf::AllowRemoteRead(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "NetConf",
            "AllowRemoteRead",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Whether to encrypt this connection with TLS (drbd.conf option tls).\n"]
    pub fn get_tls(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(NetConf::Tls(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "NetConf",
            "Tls",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Serial number of the kernel keyring key holding the private key used for\nTLS encryption (drbd.conf option tls-privkey).\n"]
    pub fn get_tls_privkey(&self) -> Result<i32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(NetConf::TlsPrivkey(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "NetConf",
            "TlsPrivkey",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Serial number of the kernel keyring key holding the certificate used for\nTLS encryption (drbd.conf option tls-certificate).\n"]
    pub fn get_tls_certificate(&self) -> Result<i32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(NetConf::TlsCertificate(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "NetConf",
            "TlsCertificate",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Serial number of the kernel keyring used to look up the TLS key material\n(drbd.conf option tls-keyring).\n"]
    pub fn get_tls_keyring(&self) -> Result<i32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(NetConf::TlsKeyring(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "NetConf",
            "TlsKeyring",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Whether to load-balance traffic for this connection across all\nconfigured paths instead of using only one at a time (drbd.conf option\nload-balance-paths).\n"]
    pub fn get_load_balance_paths(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(NetConf::LoadBalancePaths(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "NetConf",
            "LoadBalancePaths",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Size, in bytes, of the RDMA transport\\'s control-stream receive buffer,\nor 0 to derive it from rcvbuf-size (drbd.conf option\nrdma-ctrl-rcvbuf-size).\n"]
    pub fn get_rdma_ctrl_rcvbuf_size(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(NetConf::RdmaCtrlRcvbufSize(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "NetConf",
            "RdmaCtrlRcvbufSize",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Size, in bytes, of the RDMA transport\\'s control-stream send buffer, or\n0 to derive it from sndbuf-size (drbd.conf option\nrdma-ctrl-sndbuf-size).\n"]
    pub fn get_rdma_ctrl_sndbuf_size(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(NetConf::RdmaCtrlSndbufSize(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "NetConf",
            "RdmaCtrlSndbufSize",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl NetConf<'_> {
    pub fn new<'a>(buf: &'a [u8]) -> IterableNetConf<'a> {
        IterableNetConf::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        let res = match r#type {
            1u16 => "SharedSecret",
            2u16 => "CramHmacAlg",
            3u16 => "IntegrityAlg",
            4u16 => "VerifyAlg",
            5u16 => "CsumsAlg",
            6u16 => "Protocol",
            7u16 => "ConnectInt",
            8u16 => "Timeout",
            9u16 => "PingInt",
            10u16 => "PingTimeo",
            11u16 => "SndbufSize",
            12u16 => "RcvbufSize",
            13u16 => "KoCount",
            14u16 => "MaxEpochSize",
            15u16 => "AfterSb0pri",
            16u16 => "AfterSb1pri",
            17u16 => "AfterSb2pri",
            18u16 => "RrConflict",
            19u16 => "OnCongestion",
            20u16 => "CongFill",
            21u16 => "CongExtents",
            22u16 => "TwoPrimaries",
            23u16 => "TcpCork",
            24u16 => "AlwaysAsbp",
            25u16 => "UseRle",
            26u16 => "Fencing",
            27u16 => "ConnectionName",
            28u16 => "CsumsAfterCrashOnly",
            29u16 => "SockCheckTimeo",
            30u16 => "TransportName",
            31u16 => "MaxBuffers",
            32u16 => "AllowRemoteRead",
            33u16 => "Tls",
            34u16 => "TlsPrivkey",
            35u16 => "TlsCertificate",
            36u16 => "TlsKeyring",
            37u16 => "LoadBalancePaths",
            38u16 => "RdmaCtrlRcvbufSize",
            39u16 => "RdmaCtrlSndbufSize",
            _ => return None,
        };
        Some(res)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableNetConf<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableNetConf<'a> {
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
impl<'a> Iterator for IterableNetConf<'a> {
    type Item = Result<NetConf<'a>, ErrorContext>;
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
                1u16 => NetConf::SharedSecret({
                    let res = CStr::from_bytes_with_nul(next).ok();
                    let Some(val) = res else { break };
                    val
                }),
                2u16 => NetConf::CramHmacAlg({
                    let res = CStr::from_bytes_with_nul(next).ok();
                    let Some(val) = res else { break };
                    val
                }),
                3u16 => NetConf::IntegrityAlg({
                    let res = CStr::from_bytes_with_nul(next).ok();
                    let Some(val) = res else { break };
                    val
                }),
                4u16 => NetConf::VerifyAlg({
                    let res = CStr::from_bytes_with_nul(next).ok();
                    let Some(val) = res else { break };
                    val
                }),
                5u16 => NetConf::CsumsAlg({
                    let res = CStr::from_bytes_with_nul(next).ok();
                    let Some(val) = res else { break };
                    val
                }),
                6u16 => NetConf::Protocol({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                7u16 => NetConf::ConnectInt({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                8u16 => NetConf::Timeout({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                9u16 => NetConf::PingInt({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                10u16 => NetConf::PingTimeo({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                11u16 => NetConf::SndbufSize({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                12u16 => NetConf::RcvbufSize({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                13u16 => NetConf::KoCount({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                14u16 => NetConf::MaxEpochSize({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                15u16 => NetConf::AfterSb0pri({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                16u16 => NetConf::AfterSb1pri({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                17u16 => NetConf::AfterSb2pri({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                18u16 => NetConf::RrConflict({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                19u16 => NetConf::OnCongestion({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                20u16 => NetConf::CongFill({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                21u16 => NetConf::CongExtents({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                22u16 => NetConf::TwoPrimaries({
                    let res = parse_u8(next);
                    let Some(val) = res else { break };
                    val
                }),
                23u16 => NetConf::TcpCork({
                    let res = parse_u8(next);
                    let Some(val) = res else { break };
                    val
                }),
                24u16 => NetConf::AlwaysAsbp({
                    let res = parse_u8(next);
                    let Some(val) = res else { break };
                    val
                }),
                25u16 => NetConf::UseRle({
                    let res = parse_u8(next);
                    let Some(val) = res else { break };
                    val
                }),
                26u16 => NetConf::Fencing({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                27u16 => NetConf::ConnectionName({
                    let res = CStr::from_bytes_with_nul(next).ok();
                    let Some(val) = res else { break };
                    val
                }),
                28u16 => NetConf::CsumsAfterCrashOnly({
                    let res = parse_u8(next);
                    let Some(val) = res else { break };
                    val
                }),
                29u16 => NetConf::SockCheckTimeo({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                30u16 => NetConf::TransportName({
                    let res = CStr::from_bytes_with_nul(next).ok();
                    let Some(val) = res else { break };
                    val
                }),
                31u16 => NetConf::MaxBuffers({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                32u16 => NetConf::AllowRemoteRead({
                    let res = parse_u8(next);
                    let Some(val) = res else { break };
                    val
                }),
                33u16 => NetConf::Tls({
                    let res = parse_u8(next);
                    let Some(val) = res else { break };
                    val
                }),
                34u16 => NetConf::TlsPrivkey({
                    let res = parse_i32(next);
                    let Some(val) = res else { break };
                    val
                }),
                35u16 => NetConf::TlsCertificate({
                    let res = parse_i32(next);
                    let Some(val) = res else { break };
                    val
                }),
                36u16 => NetConf::TlsKeyring({
                    let res = parse_i32(next);
                    let Some(val) = res else { break };
                    val
                }),
                37u16 => NetConf::LoadBalancePaths({
                    let res = parse_u8(next);
                    let Some(val) = res else { break };
                    val
                }),
                38u16 => NetConf::RdmaCtrlRcvbufSize({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                39u16 => NetConf::RdmaCtrlSndbufSize({
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
            "NetConf",
            r#type.and_then(|t| NetConf::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl<'a> std::fmt::Debug for IterableNetConf<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("NetConf");
        let mut iter = IterableNetConf::with_loc(&[], self.orig_loc);
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
                NetConf::SharedSecret(val) => fmt.field("SharedSecret", &val),
                NetConf::CramHmacAlg(val) => fmt.field("CramHmacAlg", &val),
                NetConf::IntegrityAlg(val) => fmt.field("IntegrityAlg", &val),
                NetConf::VerifyAlg(val) => fmt.field("VerifyAlg", &val),
                NetConf::CsumsAlg(val) => fmt.field("CsumsAlg", &val),
                NetConf::Protocol(val) => fmt.field(
                    "Protocol",
                    &FormatEnum(val.into(), WireProtocol::from_value),
                ),
                NetConf::ConnectInt(val) => fmt.field("ConnectInt", &val),
                NetConf::Timeout(val) => fmt.field("Timeout", &val),
                NetConf::PingInt(val) => fmt.field("PingInt", &val),
                NetConf::PingTimeo(val) => fmt.field("PingTimeo", &val),
                NetConf::SndbufSize(val) => fmt.field("SndbufSize", &val),
                NetConf::RcvbufSize(val) => fmt.field("RcvbufSize", &val),
                NetConf::KoCount(val) => fmt.field("KoCount", &val),
                NetConf::MaxEpochSize(val) => fmt.field("MaxEpochSize", &val),
                NetConf::AfterSb0pri(val) => fmt.field(
                    "AfterSb0pri",
                    &FormatEnum(val.into(), AfterSbPolicy::from_value),
                ),
                NetConf::AfterSb1pri(val) => fmt.field(
                    "AfterSb1pri",
                    &FormatEnum(val.into(), AfterSbPolicy::from_value),
                ),
                NetConf::AfterSb2pri(val) => fmt.field(
                    "AfterSb2pri",
                    &FormatEnum(val.into(), AfterSbPolicy::from_value),
                ),
                NetConf::RrConflict(val) => fmt.field(
                    "RrConflict",
                    &FormatEnum(val.into(), AfterSbPolicy::from_value),
                ),
                NetConf::OnCongestion(val) => fmt.field(
                    "OnCongestion",
                    &FormatEnum(val.into(), OnCongestionPolicy::from_value),
                ),
                NetConf::CongFill(val) => fmt.field("CongFill", &val),
                NetConf::CongExtents(val) => fmt.field("CongExtents", &val),
                NetConf::TwoPrimaries(val) => fmt.field("TwoPrimaries", &val),
                NetConf::TcpCork(val) => fmt.field("TcpCork", &val),
                NetConf::AlwaysAsbp(val) => fmt.field("AlwaysAsbp", &val),
                NetConf::UseRle(val) => fmt.field("UseRle", &val),
                NetConf::Fencing(val) => fmt.field(
                    "Fencing",
                    &FormatEnum(val.into(), FencingPolicy::from_value),
                ),
                NetConf::ConnectionName(val) => fmt.field("ConnectionName", &val),
                NetConf::CsumsAfterCrashOnly(val) => fmt.field("CsumsAfterCrashOnly", &val),
                NetConf::SockCheckTimeo(val) => fmt.field("SockCheckTimeo", &val),
                NetConf::TransportName(val) => fmt.field("TransportName", &val),
                NetConf::MaxBuffers(val) => fmt.field("MaxBuffers", &val),
                NetConf::AllowRemoteRead(val) => fmt.field("AllowRemoteRead", &val),
                NetConf::Tls(val) => fmt.field("Tls", &val),
                NetConf::TlsPrivkey(val) => fmt.field("TlsPrivkey", &val),
                NetConf::TlsCertificate(val) => fmt.field("TlsCertificate", &val),
                NetConf::TlsKeyring(val) => fmt.field("TlsKeyring", &val),
                NetConf::LoadBalancePaths(val) => fmt.field("LoadBalancePaths", &val),
                NetConf::RdmaCtrlRcvbufSize(val) => fmt.field("RdmaCtrlRcvbufSize", &val),
                NetConf::RdmaCtrlSndbufSize(val) => fmt.field("RdmaCtrlSndbufSize", &val),
            };
        }
        fmt.finish()
    }
}
impl IterableNetConf<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("NetConf", offset));
            return (stack, missing_type.and_then(|t| NetConf::attr_from_type(t)));
        }
        if cur > offset || cur + self.buf.len() < offset {
            return (stack, None);
        }
        let mut attrs = self.clone();
        let mut last_off = cur + attrs.pos;
        while let Some(attr) = attrs.next() {
            let Ok(attr) = attr else { break };
            match attr {
                NetConf::SharedSecret(val) => {
                    if last_off == offset {
                        stack.push(("SharedSecret", last_off));
                        break;
                    }
                }
                NetConf::CramHmacAlg(val) => {
                    if last_off == offset {
                        stack.push(("CramHmacAlg", last_off));
                        break;
                    }
                }
                NetConf::IntegrityAlg(val) => {
                    if last_off == offset {
                        stack.push(("IntegrityAlg", last_off));
                        break;
                    }
                }
                NetConf::VerifyAlg(val) => {
                    if last_off == offset {
                        stack.push(("VerifyAlg", last_off));
                        break;
                    }
                }
                NetConf::CsumsAlg(val) => {
                    if last_off == offset {
                        stack.push(("CsumsAlg", last_off));
                        break;
                    }
                }
                NetConf::Protocol(val) => {
                    if last_off == offset {
                        stack.push(("Protocol", last_off));
                        break;
                    }
                }
                NetConf::ConnectInt(val) => {
                    if last_off == offset {
                        stack.push(("ConnectInt", last_off));
                        break;
                    }
                }
                NetConf::Timeout(val) => {
                    if last_off == offset {
                        stack.push(("Timeout", last_off));
                        break;
                    }
                }
                NetConf::PingInt(val) => {
                    if last_off == offset {
                        stack.push(("PingInt", last_off));
                        break;
                    }
                }
                NetConf::PingTimeo(val) => {
                    if last_off == offset {
                        stack.push(("PingTimeo", last_off));
                        break;
                    }
                }
                NetConf::SndbufSize(val) => {
                    if last_off == offset {
                        stack.push(("SndbufSize", last_off));
                        break;
                    }
                }
                NetConf::RcvbufSize(val) => {
                    if last_off == offset {
                        stack.push(("RcvbufSize", last_off));
                        break;
                    }
                }
                NetConf::KoCount(val) => {
                    if last_off == offset {
                        stack.push(("KoCount", last_off));
                        break;
                    }
                }
                NetConf::MaxEpochSize(val) => {
                    if last_off == offset {
                        stack.push(("MaxEpochSize", last_off));
                        break;
                    }
                }
                NetConf::AfterSb0pri(val) => {
                    if last_off == offset {
                        stack.push(("AfterSb0pri", last_off));
                        break;
                    }
                }
                NetConf::AfterSb1pri(val) => {
                    if last_off == offset {
                        stack.push(("AfterSb1pri", last_off));
                        break;
                    }
                }
                NetConf::AfterSb2pri(val) => {
                    if last_off == offset {
                        stack.push(("AfterSb2pri", last_off));
                        break;
                    }
                }
                NetConf::RrConflict(val) => {
                    if last_off == offset {
                        stack.push(("RrConflict", last_off));
                        break;
                    }
                }
                NetConf::OnCongestion(val) => {
                    if last_off == offset {
                        stack.push(("OnCongestion", last_off));
                        break;
                    }
                }
                NetConf::CongFill(val) => {
                    if last_off == offset {
                        stack.push(("CongFill", last_off));
                        break;
                    }
                }
                NetConf::CongExtents(val) => {
                    if last_off == offset {
                        stack.push(("CongExtents", last_off));
                        break;
                    }
                }
                NetConf::TwoPrimaries(val) => {
                    if last_off == offset {
                        stack.push(("TwoPrimaries", last_off));
                        break;
                    }
                }
                NetConf::TcpCork(val) => {
                    if last_off == offset {
                        stack.push(("TcpCork", last_off));
                        break;
                    }
                }
                NetConf::AlwaysAsbp(val) => {
                    if last_off == offset {
                        stack.push(("AlwaysAsbp", last_off));
                        break;
                    }
                }
                NetConf::UseRle(val) => {
                    if last_off == offset {
                        stack.push(("UseRle", last_off));
                        break;
                    }
                }
                NetConf::Fencing(val) => {
                    if last_off == offset {
                        stack.push(("Fencing", last_off));
                        break;
                    }
                }
                NetConf::ConnectionName(val) => {
                    if last_off == offset {
                        stack.push(("ConnectionName", last_off));
                        break;
                    }
                }
                NetConf::CsumsAfterCrashOnly(val) => {
                    if last_off == offset {
                        stack.push(("CsumsAfterCrashOnly", last_off));
                        break;
                    }
                }
                NetConf::SockCheckTimeo(val) => {
                    if last_off == offset {
                        stack.push(("SockCheckTimeo", last_off));
                        break;
                    }
                }
                NetConf::TransportName(val) => {
                    if last_off == offset {
                        stack.push(("TransportName", last_off));
                        break;
                    }
                }
                NetConf::MaxBuffers(val) => {
                    if last_off == offset {
                        stack.push(("MaxBuffers", last_off));
                        break;
                    }
                }
                NetConf::AllowRemoteRead(val) => {
                    if last_off == offset {
                        stack.push(("AllowRemoteRead", last_off));
                        break;
                    }
                }
                NetConf::Tls(val) => {
                    if last_off == offset {
                        stack.push(("Tls", last_off));
                        break;
                    }
                }
                NetConf::TlsPrivkey(val) => {
                    if last_off == offset {
                        stack.push(("TlsPrivkey", last_off));
                        break;
                    }
                }
                NetConf::TlsCertificate(val) => {
                    if last_off == offset {
                        stack.push(("TlsCertificate", last_off));
                        break;
                    }
                }
                NetConf::TlsKeyring(val) => {
                    if last_off == offset {
                        stack.push(("TlsKeyring", last_off));
                        break;
                    }
                }
                NetConf::LoadBalancePaths(val) => {
                    if last_off == offset {
                        stack.push(("LoadBalancePaths", last_off));
                        break;
                    }
                }
                NetConf::RdmaCtrlRcvbufSize(val) => {
                    if last_off == offset {
                        stack.push(("RdmaCtrlRcvbufSize", last_off));
                        break;
                    }
                }
                NetConf::RdmaCtrlSndbufSize(val) => {
                    if last_off == offset {
                        stack.push(("RdmaCtrlSndbufSize", last_off));
                        break;
                    }
                }
                _ => {}
            };
            last_off = cur + attrs.pos;
        }
        if !stack.is_empty() {
            stack.push(("NetConf", cur));
        }
        (stack, None)
    }
}
#[derive(Clone)]
pub enum SetRoleParms {
    #[doc = "Request that the promotion to primary succeed even if some devices are\nnot guaranteed to have up-to-date data (drbdsetup primary option force).\n"]
    Force(()),
}
impl<'a> IterableSetRoleParms<'a> {
    #[doc = "Request that the promotion to primary succeed even if some devices are\nnot guaranteed to have up-to-date data (drbdsetup primary option force).\n"]
    pub fn get_force(&self) -> Result<(), ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(SetRoleParms::Force(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "SetRoleParms",
            "Force",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl SetRoleParms {
    pub fn new<'a>(buf: &'a [u8]) -> IterableSetRoleParms<'a> {
        IterableSetRoleParms::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        let res = match r#type {
            1u16 => "Force",
            _ => return None,
        };
        Some(res)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableSetRoleParms<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableSetRoleParms<'a> {
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
impl<'a> Iterator for IterableSetRoleParms<'a> {
    type Item = Result<SetRoleParms, ErrorContext>;
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
                1u16 => SetRoleParms::Force(()),
                n if cfg!(any(test, feature = "deny-unknown-attrs")) => break,
                n => continue,
            };
            return Some(Ok(res));
        }
        Some(Err(ErrorContext::new(
            "SetRoleParms",
            r#type.and_then(|t| SetRoleParms::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl std::fmt::Debug for IterableSetRoleParms<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("SetRoleParms");
        let mut iter = IterableSetRoleParms::with_loc(&[], self.orig_loc);
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
                SetRoleParms::Force(val) => fmt.field("Force", &val),
            };
        }
        fmt.finish()
    }
}
impl IterableSetRoleParms<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("SetRoleParms", offset));
            return (
                stack,
                missing_type.and_then(|t| SetRoleParms::attr_from_type(t)),
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
                SetRoleParms::Force(val) => {
                    if last_off == offset {
                        stack.push(("Force", last_off));
                        break;
                    }
                }
                _ => {}
            };
            last_off = cur + attrs.pos;
        }
        if !stack.is_empty() {
            stack.push(("SetRoleParms", cur));
        }
        (stack, None)
    }
}
#[derive(Clone)]
pub enum ResizeParms {
    #[doc = "New size of the device, in sectors, or 0 to use the full size of the\nbacking devices (drbdsetup resize option size).\n"]
    Size(u64),
    #[doc = "Request that the resize proceed even though not all peer devices are\ncurrently connected, assuming they already have enough space.\n"]
    AssumePeerHasSpace(()),
    #[doc = "Request that the additional space not be resynchronized, assuming it is\nalready identical on all nodes.\n"]
    AssumeClean(()),
    #[doc = "Number of activity log stripes to use after the resize (drbdsetup resize\noption al-stripes).\n"]
    AlStripes(u32),
    #[doc = "Size of each activity log stripe, in KiB, after the resize (drbdsetup\nresize option al-stripe-size).\n"]
    AlStripeSize(u32),
}
impl<'a> IterableResizeParms<'a> {
    #[doc = "New size of the device, in sectors, or 0 to use the full size of the\nbacking devices (drbdsetup resize option size).\n"]
    pub fn get_size(&self) -> Result<u64, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(ResizeParms::Size(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "ResizeParms",
            "Size",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Request that the resize proceed even though not all peer devices are\ncurrently connected, assuming they already have enough space.\n"]
    pub fn get_assume_peer_has_space(&self) -> Result<(), ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(ResizeParms::AssumePeerHasSpace(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "ResizeParms",
            "AssumePeerHasSpace",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Request that the additional space not be resynchronized, assuming it is\nalready identical on all nodes.\n"]
    pub fn get_assume_clean(&self) -> Result<(), ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(ResizeParms::AssumeClean(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "ResizeParms",
            "AssumeClean",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Number of activity log stripes to use after the resize (drbdsetup resize\noption al-stripes).\n"]
    pub fn get_al_stripes(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(ResizeParms::AlStripes(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "ResizeParms",
            "AlStripes",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Size of each activity log stripe, in KiB, after the resize (drbdsetup\nresize option al-stripe-size).\n"]
    pub fn get_al_stripe_size(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(ResizeParms::AlStripeSize(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "ResizeParms",
            "AlStripeSize",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl ResizeParms {
    pub fn new<'a>(buf: &'a [u8]) -> IterableResizeParms<'a> {
        IterableResizeParms::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        let res = match r#type {
            1u16 => "Size",
            2u16 => "AssumePeerHasSpace",
            3u16 => "AssumeClean",
            4u16 => "AlStripes",
            5u16 => "AlStripeSize",
            _ => return None,
        };
        Some(res)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableResizeParms<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableResizeParms<'a> {
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
impl<'a> Iterator for IterableResizeParms<'a> {
    type Item = Result<ResizeParms, ErrorContext>;
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
                1u16 => ResizeParms::Size({
                    let res = parse_u64(next);
                    let Some(val) = res else { break };
                    val
                }),
                2u16 => ResizeParms::AssumePeerHasSpace(()),
                3u16 => ResizeParms::AssumeClean(()),
                4u16 => ResizeParms::AlStripes({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                5u16 => ResizeParms::AlStripeSize({
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
            "ResizeParms",
            r#type.and_then(|t| ResizeParms::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl std::fmt::Debug for IterableResizeParms<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("ResizeParms");
        let mut iter = IterableResizeParms::with_loc(&[], self.orig_loc);
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
                ResizeParms::Size(val) => fmt.field("Size", &val),
                ResizeParms::AssumePeerHasSpace(val) => fmt.field("AssumePeerHasSpace", &val),
                ResizeParms::AssumeClean(val) => fmt.field("AssumeClean", &val),
                ResizeParms::AlStripes(val) => fmt.field("AlStripes", &val),
                ResizeParms::AlStripeSize(val) => fmt.field("AlStripeSize", &val),
            };
        }
        fmt.finish()
    }
}
impl IterableResizeParms<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("ResizeParms", offset));
            return (
                stack,
                missing_type.and_then(|t| ResizeParms::attr_from_type(t)),
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
                ResizeParms::Size(val) => {
                    if last_off == offset {
                        stack.push(("Size", last_off));
                        break;
                    }
                }
                ResizeParms::AssumePeerHasSpace(val) => {
                    if last_off == offset {
                        stack.push(("AssumePeerHasSpace", last_off));
                        break;
                    }
                }
                ResizeParms::AssumeClean(val) => {
                    if last_off == offset {
                        stack.push(("AssumeClean", last_off));
                        break;
                    }
                }
                ResizeParms::AlStripes(val) => {
                    if last_off == offset {
                        stack.push(("AlStripes", last_off));
                        break;
                    }
                }
                ResizeParms::AlStripeSize(val) => {
                    if last_off == offset {
                        stack.push(("AlStripeSize", last_off));
                        break;
                    }
                }
                _ => {}
            };
            last_off = cur + attrs.pos;
        }
        if !stack.is_empty() {
            stack.push(("ResizeParms", cur));
        }
        (stack, None)
    }
}
#[derive(Clone)]
pub enum StartOvParms {
    #[doc = "Sector at which to start online verification, to resume a previously\nstopped run.\n"]
    StartSector(u64),
    #[doc = "Sector at which to stop online verification.\n"]
    StopSector(u64),
}
impl<'a> IterableStartOvParms<'a> {
    #[doc = "Sector at which to start online verification, to resume a previously\nstopped run.\n"]
    pub fn get_start_sector(&self) -> Result<u64, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(StartOvParms::StartSector(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "StartOvParms",
            "StartSector",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Sector at which to stop online verification.\n"]
    pub fn get_stop_sector(&self) -> Result<u64, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(StartOvParms::StopSector(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "StartOvParms",
            "StopSector",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl StartOvParms {
    pub fn new<'a>(buf: &'a [u8]) -> IterableStartOvParms<'a> {
        IterableStartOvParms::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        let res = match r#type {
            1u16 => "StartSector",
            2u16 => "StopSector",
            _ => return None,
        };
        Some(res)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableStartOvParms<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableStartOvParms<'a> {
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
impl<'a> Iterator for IterableStartOvParms<'a> {
    type Item = Result<StartOvParms, ErrorContext>;
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
                1u16 => StartOvParms::StartSector({
                    let res = parse_u64(next);
                    let Some(val) = res else { break };
                    val
                }),
                2u16 => StartOvParms::StopSector({
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
            "StartOvParms",
            r#type.and_then(|t| StartOvParms::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl std::fmt::Debug for IterableStartOvParms<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("StartOvParms");
        let mut iter = IterableStartOvParms::with_loc(&[], self.orig_loc);
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
                StartOvParms::StartSector(val) => fmt.field("StartSector", &val),
                StartOvParms::StopSector(val) => fmt.field("StopSector", &val),
            };
        }
        fmt.finish()
    }
}
impl IterableStartOvParms<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("StartOvParms", offset));
            return (
                stack,
                missing_type.and_then(|t| StartOvParms::attr_from_type(t)),
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
                StartOvParms::StartSector(val) => {
                    if last_off == offset {
                        stack.push(("StartSector", last_off));
                        break;
                    }
                }
                StartOvParms::StopSector(val) => {
                    if last_off == offset {
                        stack.push(("StopSector", last_off));
                        break;
                    }
                }
                _ => {}
            };
            last_off = cur + attrs.pos;
        }
        if !stack.is_empty() {
            stack.push(("StartOvParms", cur));
        }
        (stack, None)
    }
}
#[derive(Clone)]
pub enum NewCurrentUuidParms {
    #[doc = "Request that the peer\\'s bitmap also be cleared, marking the peer as\nfully in sync without a resync (drbdsetup new-current-uuid option\nclear-bitmap).\n"]
    ClearBm(()),
    #[doc = "Request that the peer be marked out of sync so that a resync starts,\ninstead of assuming it already has the same data (drbdsetup\nnew-current-uuid option force-resync).\n"]
    ForceResync(()),
}
impl<'a> IterableNewCurrentUuidParms<'a> {
    #[doc = "Request that the peer\\'s bitmap also be cleared, marking the peer as\nfully in sync without a resync (drbdsetup new-current-uuid option\nclear-bitmap).\n"]
    pub fn get_clear_bm(&self) -> Result<(), ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(NewCurrentUuidParms::ClearBm(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "NewCurrentUuidParms",
            "ClearBm",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Request that the peer be marked out of sync so that a resync starts,\ninstead of assuming it already has the same data (drbdsetup\nnew-current-uuid option force-resync).\n"]
    pub fn get_force_resync(&self) -> Result<(), ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(NewCurrentUuidParms::ForceResync(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "NewCurrentUuidParms",
            "ForceResync",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl NewCurrentUuidParms {
    pub fn new<'a>(buf: &'a [u8]) -> IterableNewCurrentUuidParms<'a> {
        IterableNewCurrentUuidParms::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        let res = match r#type {
            1u16 => "ClearBm",
            2u16 => "ForceResync",
            _ => return None,
        };
        Some(res)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableNewCurrentUuidParms<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableNewCurrentUuidParms<'a> {
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
impl<'a> Iterator for IterableNewCurrentUuidParms<'a> {
    type Item = Result<NewCurrentUuidParms, ErrorContext>;
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
                1u16 => NewCurrentUuidParms::ClearBm(()),
                2u16 => NewCurrentUuidParms::ForceResync(()),
                n if cfg!(any(test, feature = "deny-unknown-attrs")) => break,
                n => continue,
            };
            return Some(Ok(res));
        }
        Some(Err(ErrorContext::new(
            "NewCurrentUuidParms",
            r#type.and_then(|t| NewCurrentUuidParms::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl std::fmt::Debug for IterableNewCurrentUuidParms<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("NewCurrentUuidParms");
        let mut iter = IterableNewCurrentUuidParms::with_loc(&[], self.orig_loc);
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
                NewCurrentUuidParms::ClearBm(val) => fmt.field("ClearBm", &val),
                NewCurrentUuidParms::ForceResync(val) => fmt.field("ForceResync", &val),
            };
        }
        fmt.finish()
    }
}
impl IterableNewCurrentUuidParms<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("NewCurrentUuidParms", offset));
            return (
                stack,
                missing_type.and_then(|t| NewCurrentUuidParms::attr_from_type(t)),
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
                NewCurrentUuidParms::ClearBm(val) => {
                    if last_off == offset {
                        stack.push(("ClearBm", last_off));
                        break;
                    }
                }
                NewCurrentUuidParms::ForceResync(val) => {
                    if last_off == offset {
                        stack.push(("ForceResync", last_off));
                        break;
                    }
                }
                _ => {}
            };
            last_off = cur + attrs.pos;
        }
        if !stack.is_empty() {
            stack.push(("NewCurrentUuidParms", cur));
        }
        (stack, None)
    }
}
#[derive(Clone)]
pub enum DisconnectParms {
    #[doc = "Request that the connection be torn down immediately, discarding any\npending, not yet acknowledged data instead of waiting for it (drbdsetup\ndisconnect option force).\n"]
    Force(()),
}
impl<'a> IterableDisconnectParms<'a> {
    #[doc = "Request that the connection be torn down immediately, discarding any\npending, not yet acknowledged data instead of waiting for it (drbdsetup\ndisconnect option force).\n"]
    pub fn get_force(&self) -> Result<(), ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(DisconnectParms::Force(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "DisconnectParms",
            "Force",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl DisconnectParms {
    pub fn new<'a>(buf: &'a [u8]) -> IterableDisconnectParms<'a> {
        IterableDisconnectParms::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        let res = match r#type {
            1u16 => "Force",
            _ => return None,
        };
        Some(res)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableDisconnectParms<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableDisconnectParms<'a> {
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
impl<'a> Iterator for IterableDisconnectParms<'a> {
    type Item = Result<DisconnectParms, ErrorContext>;
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
                1u16 => DisconnectParms::Force(()),
                n if cfg!(any(test, feature = "deny-unknown-attrs")) => break,
                n => continue,
            };
            return Some(Ok(res));
        }
        Some(Err(ErrorContext::new(
            "DisconnectParms",
            r#type.and_then(|t| DisconnectParms::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl std::fmt::Debug for IterableDisconnectParms<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("DisconnectParms");
        let mut iter = IterableDisconnectParms::with_loc(&[], self.orig_loc);
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
                DisconnectParms::Force(val) => fmt.field("Force", &val),
            };
        }
        fmt.finish()
    }
}
impl IterableDisconnectParms<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("DisconnectParms", offset));
            return (
                stack,
                missing_type.and_then(|t| DisconnectParms::attr_from_type(t)),
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
                DisconnectParms::Force(val) => {
                    if last_off == offset {
                        stack.push(("Force", last_off));
                        break;
                    }
                }
                _ => {}
            };
            last_off = cur + attrs.pos;
        }
        if !stack.is_empty() {
            stack.push(("DisconnectParms", cur));
        }
        (stack, None)
    }
}
#[derive(Clone)]
pub enum DetachParms {
    #[doc = "Request that the detach proceed immediately, marking the lower-level\ndevice failed until pending I/O completes (drbdsetup detach option\nforce).\n"]
    Force(()),
    #[doc = "Request that the device be marked permanently, intentionally diskless\ninstead of temporarily diskless (drbdsetup detach option diskless).\n"]
    IntentionalDisklessDetach(()),
}
impl<'a> IterableDetachParms<'a> {
    #[doc = "Request that the detach proceed immediately, marking the lower-level\ndevice failed until pending I/O completes (drbdsetup detach option\nforce).\n"]
    pub fn get_force(&self) -> Result<(), ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(DetachParms::Force(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "DetachParms",
            "Force",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Request that the device be marked permanently, intentionally diskless\ninstead of temporarily diskless (drbdsetup detach option diskless).\n"]
    pub fn get_intentional_diskless_detach(&self) -> Result<(), ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(DetachParms::IntentionalDisklessDetach(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "DetachParms",
            "IntentionalDisklessDetach",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl DetachParms {
    pub fn new<'a>(buf: &'a [u8]) -> IterableDetachParms<'a> {
        IterableDetachParms::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        let res = match r#type {
            1u16 => "Force",
            2u16 => "IntentionalDisklessDetach",
            _ => return None,
        };
        Some(res)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableDetachParms<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableDetachParms<'a> {
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
impl<'a> Iterator for IterableDetachParms<'a> {
    type Item = Result<DetachParms, ErrorContext>;
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
                1u16 => DetachParms::Force(()),
                2u16 => DetachParms::IntentionalDisklessDetach(()),
                n if cfg!(any(test, feature = "deny-unknown-attrs")) => break,
                n => continue,
            };
            return Some(Ok(res));
        }
        Some(Err(ErrorContext::new(
            "DetachParms",
            r#type.and_then(|t| DetachParms::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl std::fmt::Debug for IterableDetachParms<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("DetachParms");
        let mut iter = IterableDetachParms::with_loc(&[], self.orig_loc);
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
                DetachParms::Force(val) => fmt.field("Force", &val),
                DetachParms::IntentionalDisklessDetach(val) => {
                    fmt.field("IntentionalDisklessDetach", &val)
                }
            };
        }
        fmt.finish()
    }
}
impl IterableDetachParms<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("DetachParms", offset));
            return (
                stack,
                missing_type.and_then(|t| DetachParms::attr_from_type(t)),
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
                DetachParms::Force(val) => {
                    if last_off == offset {
                        stack.push(("Force", last_off));
                        break;
                    }
                }
                DetachParms::IntentionalDisklessDetach(val) => {
                    if last_off == offset {
                        stack.push(("IntentionalDisklessDetach", last_off));
                        break;
                    }
                }
                _ => {}
            };
            last_off = cur + attrs.pos;
        }
        if !stack.is_empty() {
            stack.push(("DetachParms", cur));
        }
        (stack, None)
    }
}
#[derive(Clone)]
pub enum AttachParms {
    #[doc = "Request that, should the handshake with a peer find a split brain, the\nlocal data be discarded and resynchronized from that peer (drbdsetup\nattach option discard-my-data).\n"]
    DiscardMyData(()),
}
impl<'a> IterableAttachParms<'a> {
    #[doc = "Request that, should the handshake with a peer find a split brain, the\nlocal data be discarded and resynchronized from that peer (drbdsetup\nattach option discard-my-data).\n"]
    pub fn get_discard_my_data(&self) -> Result<(), ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(AttachParms::DiscardMyData(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "AttachParms",
            "DiscardMyData",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl AttachParms {
    pub fn new<'a>(buf: &'a [u8]) -> IterableAttachParms<'a> {
        IterableAttachParms::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        let res = match r#type {
            1u16 => "DiscardMyData",
            _ => return None,
        };
        Some(res)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableAttachParms<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableAttachParms<'a> {
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
impl<'a> Iterator for IterableAttachParms<'a> {
    type Item = Result<AttachParms, ErrorContext>;
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
                1u16 => AttachParms::DiscardMyData(()),
                n if cfg!(any(test, feature = "deny-unknown-attrs")) => break,
                n => continue,
            };
            return Some(Ok(res));
        }
        Some(Err(ErrorContext::new(
            "AttachParms",
            r#type.and_then(|t| AttachParms::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl std::fmt::Debug for IterableAttachParms<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("AttachParms");
        let mut iter = IterableAttachParms::with_loc(&[], self.orig_loc);
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
                AttachParms::DiscardMyData(val) => fmt.field("DiscardMyData", &val),
            };
        }
        fmt.finish()
    }
}
impl IterableAttachParms<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("AttachParms", offset));
            return (
                stack,
                missing_type.and_then(|t| AttachParms::attr_from_type(t)),
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
                AttachParms::DiscardMyData(val) => {
                    if last_off == offset {
                        stack.push(("DiscardMyData", last_off));
                        break;
                    }
                }
                _ => {}
            };
            last_off = cur + attrs.pos;
        }
        if !stack.is_empty() {
            stack.push(("AttachParms", cur));
        }
        (stack, None)
    }
}
#[derive(Clone)]
pub enum DeviceConf {
    #[doc = "Maximum size of a single I/O request the device accepts, in bytes\n(drbdsetup new-minor option max-bio-size).\n"]
    MaxBioSize(u32),
    #[doc = "Whether the device is deliberately created without a local disk, e.g. as\na diskless client (drbdsetup new-minor option diskless).\n"]
    IntentionalDiskless(u8),
    #[doc = "Logical block size the device reports, needed to combine differing\nbacking disk block sizes across nodes (drbd.conf option block-size).\n"]
    BlockSize(u32),
    #[doc = "Discard granularity, in bytes, advertised by the device; discard\nrequests must be aligned to and a multiple of this value (drbd.conf\noption discard-granularity).\n"]
    DiscardGranularity(u32),
}
impl<'a> IterableDeviceConf<'a> {
    #[doc = "Maximum size of a single I/O request the device accepts, in bytes\n(drbdsetup new-minor option max-bio-size).\n"]
    pub fn get_max_bio_size(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(DeviceConf::MaxBioSize(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "DeviceConf",
            "MaxBioSize",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Whether the device is deliberately created without a local disk, e.g. as\na diskless client (drbdsetup new-minor option diskless).\n"]
    pub fn get_intentional_diskless(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(DeviceConf::IntentionalDiskless(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "DeviceConf",
            "IntentionalDiskless",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Logical block size the device reports, needed to combine differing\nbacking disk block sizes across nodes (drbd.conf option block-size).\n"]
    pub fn get_block_size(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(DeviceConf::BlockSize(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "DeviceConf",
            "BlockSize",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Discard granularity, in bytes, advertised by the device; discard\nrequests must be aligned to and a multiple of this value (drbd.conf\noption discard-granularity).\n"]
    pub fn get_discard_granularity(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(DeviceConf::DiscardGranularity(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "DeviceConf",
            "DiscardGranularity",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl DeviceConf {
    pub fn new<'a>(buf: &'a [u8]) -> IterableDeviceConf<'a> {
        IterableDeviceConf::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        let res = match r#type {
            1u16 => "MaxBioSize",
            2u16 => "IntentionalDiskless",
            3u16 => "BlockSize",
            4u16 => "DiscardGranularity",
            _ => return None,
        };
        Some(res)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableDeviceConf<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableDeviceConf<'a> {
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
impl<'a> Iterator for IterableDeviceConf<'a> {
    type Item = Result<DeviceConf, ErrorContext>;
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
                1u16 => DeviceConf::MaxBioSize({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                2u16 => DeviceConf::IntentionalDiskless({
                    let res = parse_u8(next);
                    let Some(val) = res else { break };
                    val
                }),
                3u16 => DeviceConf::BlockSize({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                4u16 => DeviceConf::DiscardGranularity({
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
            "DeviceConf",
            r#type.and_then(|t| DeviceConf::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl std::fmt::Debug for IterableDeviceConf<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("DeviceConf");
        let mut iter = IterableDeviceConf::with_loc(&[], self.orig_loc);
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
                DeviceConf::MaxBioSize(val) => fmt.field("MaxBioSize", &val),
                DeviceConf::IntentionalDiskless(val) => fmt.field("IntentionalDiskless", &val),
                DeviceConf::BlockSize(val) => fmt.field("BlockSize", &val),
                DeviceConf::DiscardGranularity(val) => fmt.field("DiscardGranularity", &val),
            };
        }
        fmt.finish()
    }
}
impl IterableDeviceConf<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("DeviceConf", offset));
            return (
                stack,
                missing_type.and_then(|t| DeviceConf::attr_from_type(t)),
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
                DeviceConf::MaxBioSize(val) => {
                    if last_off == offset {
                        stack.push(("MaxBioSize", last_off));
                        break;
                    }
                }
                DeviceConf::IntentionalDiskless(val) => {
                    if last_off == offset {
                        stack.push(("IntentionalDiskless", last_off));
                        break;
                    }
                }
                DeviceConf::BlockSize(val) => {
                    if last_off == offset {
                        stack.push(("BlockSize", last_off));
                        break;
                    }
                }
                DeviceConf::DiscardGranularity(val) => {
                    if last_off == offset {
                        stack.push(("DiscardGranularity", last_off));
                        break;
                    }
                }
                _ => {}
            };
            last_off = cur + attrs.pos;
        }
        if !stack.is_empty() {
            stack.push(("DeviceConf", cur));
        }
        (stack, None)
    }
}
#[derive(Clone)]
pub enum ResourceInfo {
    #[doc = "Role of the resource on this node.\n\nAssociated type: [`Role`] (enum)"]
    Role(u32),
    #[doc = "Whether I/O on the resource is suspended by the user.\n"]
    Susp(u8),
    #[doc = "Whether I/O on the resource is suspended because no data is accessible.\n"]
    SuspNod(u8),
    #[doc = "Whether I/O on the resource is suspended because a fence-peer handler is\nrunning.\n"]
    SuspFen(u8),
    #[doc = "Whether I/O on the resource is suspended because it has lost quorum.\n"]
    SuspQuorum(u8),
    #[doc = "Whether all I/O on the resource fails because it was forced to demote\nfrom primary.\n"]
    FailIo(u8),
}
impl<'a> IterableResourceInfo<'a> {
    #[doc = "Role of the resource on this node.\n\nAssociated type: [`Role`] (enum)"]
    pub fn get_role(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(ResourceInfo::Role(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "ResourceInfo",
            "Role",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Whether I/O on the resource is suspended by the user.\n"]
    pub fn get_susp(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(ResourceInfo::Susp(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "ResourceInfo",
            "Susp",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Whether I/O on the resource is suspended because no data is accessible.\n"]
    pub fn get_susp_nod(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(ResourceInfo::SuspNod(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "ResourceInfo",
            "SuspNod",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Whether I/O on the resource is suspended because a fence-peer handler is\nrunning.\n"]
    pub fn get_susp_fen(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(ResourceInfo::SuspFen(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "ResourceInfo",
            "SuspFen",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Whether I/O on the resource is suspended because it has lost quorum.\n"]
    pub fn get_susp_quorum(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(ResourceInfo::SuspQuorum(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "ResourceInfo",
            "SuspQuorum",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Whether all I/O on the resource fails because it was forced to demote\nfrom primary.\n"]
    pub fn get_fail_io(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(ResourceInfo::FailIo(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "ResourceInfo",
            "FailIo",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl ResourceInfo {
    pub fn new<'a>(buf: &'a [u8]) -> IterableResourceInfo<'a> {
        IterableResourceInfo::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        let res = match r#type {
            1u16 => "Role",
            2u16 => "Susp",
            3u16 => "SuspNod",
            4u16 => "SuspFen",
            5u16 => "SuspQuorum",
            6u16 => "FailIo",
            _ => return None,
        };
        Some(res)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableResourceInfo<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableResourceInfo<'a> {
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
impl<'a> Iterator for IterableResourceInfo<'a> {
    type Item = Result<ResourceInfo, ErrorContext>;
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
                1u16 => ResourceInfo::Role({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                2u16 => ResourceInfo::Susp({
                    let res = parse_u8(next);
                    let Some(val) = res else { break };
                    val
                }),
                3u16 => ResourceInfo::SuspNod({
                    let res = parse_u8(next);
                    let Some(val) = res else { break };
                    val
                }),
                4u16 => ResourceInfo::SuspFen({
                    let res = parse_u8(next);
                    let Some(val) = res else { break };
                    val
                }),
                5u16 => ResourceInfo::SuspQuorum({
                    let res = parse_u8(next);
                    let Some(val) = res else { break };
                    val
                }),
                6u16 => ResourceInfo::FailIo({
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
            "ResourceInfo",
            r#type.and_then(|t| ResourceInfo::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl std::fmt::Debug for IterableResourceInfo<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("ResourceInfo");
        let mut iter = IterableResourceInfo::with_loc(&[], self.orig_loc);
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
                ResourceInfo::Role(val) => {
                    fmt.field("Role", &FormatEnum(val.into(), Role::from_value))
                }
                ResourceInfo::Susp(val) => fmt.field("Susp", &val),
                ResourceInfo::SuspNod(val) => fmt.field("SuspNod", &val),
                ResourceInfo::SuspFen(val) => fmt.field("SuspFen", &val),
                ResourceInfo::SuspQuorum(val) => fmt.field("SuspQuorum", &val),
                ResourceInfo::FailIo(val) => fmt.field("FailIo", &val),
            };
        }
        fmt.finish()
    }
}
impl IterableResourceInfo<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("ResourceInfo", offset));
            return (
                stack,
                missing_type.and_then(|t| ResourceInfo::attr_from_type(t)),
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
                ResourceInfo::Role(val) => {
                    if last_off == offset {
                        stack.push(("Role", last_off));
                        break;
                    }
                }
                ResourceInfo::Susp(val) => {
                    if last_off == offset {
                        stack.push(("Susp", last_off));
                        break;
                    }
                }
                ResourceInfo::SuspNod(val) => {
                    if last_off == offset {
                        stack.push(("SuspNod", last_off));
                        break;
                    }
                }
                ResourceInfo::SuspFen(val) => {
                    if last_off == offset {
                        stack.push(("SuspFen", last_off));
                        break;
                    }
                }
                ResourceInfo::SuspQuorum(val) => {
                    if last_off == offset {
                        stack.push(("SuspQuorum", last_off));
                        break;
                    }
                }
                ResourceInfo::FailIo(val) => {
                    if last_off == offset {
                        stack.push(("FailIo", last_off));
                        break;
                    }
                }
                _ => {}
            };
            last_off = cur + attrs.pos;
        }
        if !stack.is_empty() {
            stack.push(("ResourceInfo", cur));
        }
        (stack, None)
    }
}
#[derive(Clone)]
pub enum DeviceInfo<'a> {
    #[doc = "Current state of the backing disk.\n\nAssociated type: [`DiskState`] (enum)"]
    DiskState(u32),
    #[doc = "Whether the device is deliberately configured without a local disk.\n"]
    IsIntentionalDiskless(u8),
    #[doc = "Whether the device currently has quorum.\n"]
    HasQuorum(u8),
    #[doc = "Whether the device is currently opened by an application.\n"]
    IsOpen(u8),
    #[doc = "Path of the backing device, present while it is still accessible after a\ndisk failure.\n"]
    BackingDevPath(&'a CStr),
}
impl<'a> IterableDeviceInfo<'a> {
    #[doc = "Current state of the backing disk.\n\nAssociated type: [`DiskState`] (enum)"]
    pub fn get_disk_state(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(DeviceInfo::DiskState(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "DeviceInfo",
            "DiskState",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Whether the device is deliberately configured without a local disk.\n"]
    pub fn get_is_intentional_diskless(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(DeviceInfo::IsIntentionalDiskless(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "DeviceInfo",
            "IsIntentionalDiskless",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Whether the device currently has quorum.\n"]
    pub fn get_has_quorum(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(DeviceInfo::HasQuorum(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "DeviceInfo",
            "HasQuorum",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Whether the device is currently opened by an application.\n"]
    pub fn get_is_open(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(DeviceInfo::IsOpen(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "DeviceInfo",
            "IsOpen",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Path of the backing device, present while it is still accessible after a\ndisk failure.\n"]
    pub fn get_backing_dev_path(&self) -> Result<&'a CStr, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(DeviceInfo::BackingDevPath(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "DeviceInfo",
            "BackingDevPath",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl DeviceInfo<'_> {
    pub fn new<'a>(buf: &'a [u8]) -> IterableDeviceInfo<'a> {
        IterableDeviceInfo::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        let res = match r#type {
            1u16 => "DiskState",
            2u16 => "IsIntentionalDiskless",
            3u16 => "HasQuorum",
            4u16 => "IsOpen",
            5u16 => "BackingDevPath",
            _ => return None,
        };
        Some(res)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableDeviceInfo<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableDeviceInfo<'a> {
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
impl<'a> Iterator for IterableDeviceInfo<'a> {
    type Item = Result<DeviceInfo<'a>, ErrorContext>;
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
                1u16 => DeviceInfo::DiskState({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                2u16 => DeviceInfo::IsIntentionalDiskless({
                    let res = parse_u8(next);
                    let Some(val) = res else { break };
                    val
                }),
                3u16 => DeviceInfo::HasQuorum({
                    let res = parse_u8(next);
                    let Some(val) = res else { break };
                    val
                }),
                4u16 => DeviceInfo::IsOpen({
                    let res = parse_u8(next);
                    let Some(val) = res else { break };
                    val
                }),
                5u16 => DeviceInfo::BackingDevPath({
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
            "DeviceInfo",
            r#type.and_then(|t| DeviceInfo::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl<'a> std::fmt::Debug for IterableDeviceInfo<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("DeviceInfo");
        let mut iter = IterableDeviceInfo::with_loc(&[], self.orig_loc);
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
                DeviceInfo::DiskState(val) => {
                    fmt.field("DiskState", &FormatEnum(val.into(), DiskState::from_value))
                }
                DeviceInfo::IsIntentionalDiskless(val) => fmt.field("IsIntentionalDiskless", &val),
                DeviceInfo::HasQuorum(val) => fmt.field("HasQuorum", &val),
                DeviceInfo::IsOpen(val) => fmt.field("IsOpen", &val),
                DeviceInfo::BackingDevPath(val) => fmt.field("BackingDevPath", &val),
            };
        }
        fmt.finish()
    }
}
impl IterableDeviceInfo<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("DeviceInfo", offset));
            return (
                stack,
                missing_type.and_then(|t| DeviceInfo::attr_from_type(t)),
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
                DeviceInfo::DiskState(val) => {
                    if last_off == offset {
                        stack.push(("DiskState", last_off));
                        break;
                    }
                }
                DeviceInfo::IsIntentionalDiskless(val) => {
                    if last_off == offset {
                        stack.push(("IsIntentionalDiskless", last_off));
                        break;
                    }
                }
                DeviceInfo::HasQuorum(val) => {
                    if last_off == offset {
                        stack.push(("HasQuorum", last_off));
                        break;
                    }
                }
                DeviceInfo::IsOpen(val) => {
                    if last_off == offset {
                        stack.push(("IsOpen", last_off));
                        break;
                    }
                }
                DeviceInfo::BackingDevPath(val) => {
                    if last_off == offset {
                        stack.push(("BackingDevPath", last_off));
                        break;
                    }
                }
                _ => {}
            };
            last_off = cur + attrs.pos;
        }
        if !stack.is_empty() {
            stack.push(("DeviceInfo", cur));
        }
        (stack, None)
    }
}
#[derive(Clone)]
pub enum ConnectionInfo {
    #[doc = "Current state of the connection to the peer.\n\nAssociated type: [`ConnectionState`] (enum)"]
    ConnectionState(u32),
    #[doc = "Current role of the peer.\n\nAssociated type: [`Role`] (enum)"]
    Role(u32),
}
impl<'a> IterableConnectionInfo<'a> {
    #[doc = "Current state of the connection to the peer.\n\nAssociated type: [`ConnectionState`] (enum)"]
    pub fn get_connection_state(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(ConnectionInfo::ConnectionState(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "ConnectionInfo",
            "ConnectionState",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Current role of the peer.\n\nAssociated type: [`Role`] (enum)"]
    pub fn get_role(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(ConnectionInfo::Role(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "ConnectionInfo",
            "Role",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl ConnectionInfo {
    pub fn new<'a>(buf: &'a [u8]) -> IterableConnectionInfo<'a> {
        IterableConnectionInfo::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        let res = match r#type {
            1u16 => "ConnectionState",
            2u16 => "Role",
            _ => return None,
        };
        Some(res)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableConnectionInfo<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableConnectionInfo<'a> {
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
impl<'a> Iterator for IterableConnectionInfo<'a> {
    type Item = Result<ConnectionInfo, ErrorContext>;
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
                1u16 => ConnectionInfo::ConnectionState({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                2u16 => ConnectionInfo::Role({
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
            "ConnectionInfo",
            r#type.and_then(|t| ConnectionInfo::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl std::fmt::Debug for IterableConnectionInfo<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("ConnectionInfo");
        let mut iter = IterableConnectionInfo::with_loc(&[], self.orig_loc);
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
                ConnectionInfo::ConnectionState(val) => fmt.field(
                    "ConnectionState",
                    &FormatEnum(val.into(), ConnectionState::from_value),
                ),
                ConnectionInfo::Role(val) => {
                    fmt.field("Role", &FormatEnum(val.into(), Role::from_value))
                }
            };
        }
        fmt.finish()
    }
}
impl IterableConnectionInfo<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("ConnectionInfo", offset));
            return (
                stack,
                missing_type.and_then(|t| ConnectionInfo::attr_from_type(t)),
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
                ConnectionInfo::ConnectionState(val) => {
                    if last_off == offset {
                        stack.push(("ConnectionState", last_off));
                        break;
                    }
                }
                ConnectionInfo::Role(val) => {
                    if last_off == offset {
                        stack.push(("Role", last_off));
                        break;
                    }
                }
                _ => {}
            };
            last_off = cur + attrs.pos;
        }
        if !stack.is_empty() {
            stack.push(("ConnectionInfo", cur));
        }
        (stack, None)
    }
}
#[derive(Clone)]
pub enum PeerDeviceInfo {
    #[doc = "Current replication state with the peer.\n\nAssociated type: [`ReplState`] (enum)"]
    ReplState(u32),
    #[doc = "Current state of the peer\\'s disk.\n\nAssociated type: [`DiskState`] (enum)"]
    DiskState(u32),
    #[doc = "Whether resynchronization with this peer is suspended by the user.\n"]
    ResyncSuspUser(u8),
    #[doc = "Whether resynchronization with this peer is suspended by the peer.\n"]
    ResyncSuspPeer(u8),
    #[doc = "Whether resynchronization with this peer is suspended because it depends\non another volume or connection resynchronizing first.\n"]
    ResyncSuspDependency(u8),
    #[doc = "Whether the peer is deliberately configured without a local disk.\n"]
    IsIntentionalDiskless(u8),
    #[doc = "Whether resynchronization with this peer is suspended to enforce the\nmaximum number of parallel resyncs.\n"]
    ResyncSuspMaxParallel(u8),
}
impl<'a> IterablePeerDeviceInfo<'a> {
    #[doc = "Current replication state with the peer.\n\nAssociated type: [`ReplState`] (enum)"]
    pub fn get_repl_state(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(PeerDeviceInfo::ReplState(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "PeerDeviceInfo",
            "ReplState",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Current state of the peer\\'s disk.\n\nAssociated type: [`DiskState`] (enum)"]
    pub fn get_disk_state(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(PeerDeviceInfo::DiskState(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "PeerDeviceInfo",
            "DiskState",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Whether resynchronization with this peer is suspended by the user.\n"]
    pub fn get_resync_susp_user(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(PeerDeviceInfo::ResyncSuspUser(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "PeerDeviceInfo",
            "ResyncSuspUser",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Whether resynchronization with this peer is suspended by the peer.\n"]
    pub fn get_resync_susp_peer(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(PeerDeviceInfo::ResyncSuspPeer(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "PeerDeviceInfo",
            "ResyncSuspPeer",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Whether resynchronization with this peer is suspended because it depends\non another volume or connection resynchronizing first.\n"]
    pub fn get_resync_susp_dependency(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(PeerDeviceInfo::ResyncSuspDependency(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "PeerDeviceInfo",
            "ResyncSuspDependency",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Whether the peer is deliberately configured without a local disk.\n"]
    pub fn get_is_intentional_diskless(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(PeerDeviceInfo::IsIntentionalDiskless(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "PeerDeviceInfo",
            "IsIntentionalDiskless",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Whether resynchronization with this peer is suspended to enforce the\nmaximum number of parallel resyncs.\n"]
    pub fn get_resync_susp_max_parallel(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(PeerDeviceInfo::ResyncSuspMaxParallel(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "PeerDeviceInfo",
            "ResyncSuspMaxParallel",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl PeerDeviceInfo {
    pub fn new<'a>(buf: &'a [u8]) -> IterablePeerDeviceInfo<'a> {
        IterablePeerDeviceInfo::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        let res = match r#type {
            1u16 => "ReplState",
            2u16 => "DiskState",
            3u16 => "ResyncSuspUser",
            4u16 => "ResyncSuspPeer",
            5u16 => "ResyncSuspDependency",
            6u16 => "IsIntentionalDiskless",
            7u16 => "ResyncSuspMaxParallel",
            _ => return None,
        };
        Some(res)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterablePeerDeviceInfo<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterablePeerDeviceInfo<'a> {
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
impl<'a> Iterator for IterablePeerDeviceInfo<'a> {
    type Item = Result<PeerDeviceInfo, ErrorContext>;
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
                1u16 => PeerDeviceInfo::ReplState({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                2u16 => PeerDeviceInfo::DiskState({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                3u16 => PeerDeviceInfo::ResyncSuspUser({
                    let res = parse_u8(next);
                    let Some(val) = res else { break };
                    val
                }),
                4u16 => PeerDeviceInfo::ResyncSuspPeer({
                    let res = parse_u8(next);
                    let Some(val) = res else { break };
                    val
                }),
                5u16 => PeerDeviceInfo::ResyncSuspDependency({
                    let res = parse_u8(next);
                    let Some(val) = res else { break };
                    val
                }),
                6u16 => PeerDeviceInfo::IsIntentionalDiskless({
                    let res = parse_u8(next);
                    let Some(val) = res else { break };
                    val
                }),
                7u16 => PeerDeviceInfo::ResyncSuspMaxParallel({
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
            "PeerDeviceInfo",
            r#type.and_then(|t| PeerDeviceInfo::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl std::fmt::Debug for IterablePeerDeviceInfo<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("PeerDeviceInfo");
        let mut iter = IterablePeerDeviceInfo::with_loc(&[], self.orig_loc);
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
                PeerDeviceInfo::ReplState(val) => {
                    fmt.field("ReplState", &FormatEnum(val.into(), ReplState::from_value))
                }
                PeerDeviceInfo::DiskState(val) => {
                    fmt.field("DiskState", &FormatEnum(val.into(), DiskState::from_value))
                }
                PeerDeviceInfo::ResyncSuspUser(val) => fmt.field("ResyncSuspUser", &val),
                PeerDeviceInfo::ResyncSuspPeer(val) => fmt.field("ResyncSuspPeer", &val),
                PeerDeviceInfo::ResyncSuspDependency(val) => {
                    fmt.field("ResyncSuspDependency", &val)
                }
                PeerDeviceInfo::IsIntentionalDiskless(val) => {
                    fmt.field("IsIntentionalDiskless", &val)
                }
                PeerDeviceInfo::ResyncSuspMaxParallel(val) => {
                    fmt.field("ResyncSuspMaxParallel", &val)
                }
            };
        }
        fmt.finish()
    }
}
impl IterablePeerDeviceInfo<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("PeerDeviceInfo", offset));
            return (
                stack,
                missing_type.and_then(|t| PeerDeviceInfo::attr_from_type(t)),
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
                PeerDeviceInfo::ReplState(val) => {
                    if last_off == offset {
                        stack.push(("ReplState", last_off));
                        break;
                    }
                }
                PeerDeviceInfo::DiskState(val) => {
                    if last_off == offset {
                        stack.push(("DiskState", last_off));
                        break;
                    }
                }
                PeerDeviceInfo::ResyncSuspUser(val) => {
                    if last_off == offset {
                        stack.push(("ResyncSuspUser", last_off));
                        break;
                    }
                }
                PeerDeviceInfo::ResyncSuspPeer(val) => {
                    if last_off == offset {
                        stack.push(("ResyncSuspPeer", last_off));
                        break;
                    }
                }
                PeerDeviceInfo::ResyncSuspDependency(val) => {
                    if last_off == offset {
                        stack.push(("ResyncSuspDependency", last_off));
                        break;
                    }
                }
                PeerDeviceInfo::IsIntentionalDiskless(val) => {
                    if last_off == offset {
                        stack.push(("IsIntentionalDiskless", last_off));
                        break;
                    }
                }
                PeerDeviceInfo::ResyncSuspMaxParallel(val) => {
                    if last_off == offset {
                        stack.push(("ResyncSuspMaxParallel", last_off));
                        break;
                    }
                }
                _ => {}
            };
            last_off = cur + attrs.pos;
        }
        if !stack.is_empty() {
            stack.push(("PeerDeviceInfo", cur));
        }
        (stack, None)
    }
}
#[derive(Clone)]
pub enum ResourceStatistics {
    #[doc = "Write ordering method currently in use for the resource\\'s backing\ndevices.\n\nAssociated type: [`WriteOrdering`] (enum)"]
    WriteOrdering(u32),
}
impl<'a> IterableResourceStatistics<'a> {
    #[doc = "Write ordering method currently in use for the resource\\'s backing\ndevices.\n\nAssociated type: [`WriteOrdering`] (enum)"]
    pub fn get_write_ordering(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(ResourceStatistics::WriteOrdering(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "ResourceStatistics",
            "WriteOrdering",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl ResourceStatistics {
    pub fn new<'a>(buf: &'a [u8]) -> IterableResourceStatistics<'a> {
        IterableResourceStatistics::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        let res = match r#type {
            1u16 => "WriteOrdering",
            _ => return None,
        };
        Some(res)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableResourceStatistics<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableResourceStatistics<'a> {
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
impl<'a> Iterator for IterableResourceStatistics<'a> {
    type Item = Result<ResourceStatistics, ErrorContext>;
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
                1u16 => ResourceStatistics::WriteOrdering({
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
            "ResourceStatistics",
            r#type.and_then(|t| ResourceStatistics::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl std::fmt::Debug for IterableResourceStatistics<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("ResourceStatistics");
        let mut iter = IterableResourceStatistics::with_loc(&[], self.orig_loc);
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
                ResourceStatistics::WriteOrdering(val) => fmt.field(
                    "WriteOrdering",
                    &FormatEnum(val.into(), WriteOrdering::from_value),
                ),
            };
        }
        fmt.finish()
    }
}
impl IterableResourceStatistics<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("ResourceStatistics", offset));
            return (
                stack,
                missing_type.and_then(|t| ResourceStatistics::attr_from_type(t)),
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
                ResourceStatistics::WriteOrdering(val) => {
                    if last_off == offset {
                        stack.push(("WriteOrdering", last_off));
                        break;
                    }
                }
                _ => {}
            };
            last_off = cur + attrs.pos;
        }
        if !stack.is_empty() {
            stack.push(("ResourceStatistics", cur));
        }
        (stack, None)
    }
}
#[derive(Clone)]
pub enum DeviceStatistics<'a> {
    #[doc = "Capacity of the device, in sectors.\n"]
    Size(u64),
    #[doc = "Total data read from the device, in sectors, since it was attached.\n"]
    Read(u64),
    #[doc = "Total data written to the device, in sectors, since it was attached.\n"]
    Write(u64),
    #[doc = "Number of activity log metadata writes since the device was attached.\n"]
    AlWrites(u64),
    #[doc = "Number of bitmap metadata writes since the device was attached.\n"]
    BmWrites(u64),
    #[doc = "Number of application I/O requests currently in flight on the device.\n"]
    UpperPending(u32),
    #[doc = "Number of I/O requests currently in flight on the backing device.\n"]
    LowerPending(u32),
    #[doc = "Whether application I/O to the device is currently throttled.\n"]
    UpperBlocked(u8),
    #[doc = "Whether I/O to the backing device is currently blocked.\n"]
    LowerBlocked(u8),
    #[doc = "Whether updates to the activity log are currently suspended.\n"]
    AlSuspended(u8),
    #[doc = "Data generation UUID currently exposed to peers, identifying the\ngeneration of data this node has.\n"]
    ExposedDataUuid(u64),
    #[doc = "Current data generation UUID of the device\\'s metadata.\n"]
    CurrentUuid(u64),
    #[doc = "Bitmask of the backing disk\\'s metadata flags.\n"]
    DiskFlags(u32),
    #[doc = "Historical bitmap UUIDs of prior data generations, one slot per peer.\n"]
    HistoryUuids(&'a [u8]),
}
impl<'a> IterableDeviceStatistics<'a> {
    #[doc = "Capacity of the device, in sectors.\n"]
    pub fn get_size(&self) -> Result<u64, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(DeviceStatistics::Size(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "DeviceStatistics",
            "Size",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Total data read from the device, in sectors, since it was attached.\n"]
    pub fn get_read(&self) -> Result<u64, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(DeviceStatistics::Read(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "DeviceStatistics",
            "Read",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Total data written to the device, in sectors, since it was attached.\n"]
    pub fn get_write(&self) -> Result<u64, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(DeviceStatistics::Write(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "DeviceStatistics",
            "Write",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Number of activity log metadata writes since the device was attached.\n"]
    pub fn get_al_writes(&self) -> Result<u64, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(DeviceStatistics::AlWrites(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "DeviceStatistics",
            "AlWrites",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Number of bitmap metadata writes since the device was attached.\n"]
    pub fn get_bm_writes(&self) -> Result<u64, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(DeviceStatistics::BmWrites(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "DeviceStatistics",
            "BmWrites",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Number of application I/O requests currently in flight on the device.\n"]
    pub fn get_upper_pending(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(DeviceStatistics::UpperPending(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "DeviceStatistics",
            "UpperPending",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Number of I/O requests currently in flight on the backing device.\n"]
    pub fn get_lower_pending(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(DeviceStatistics::LowerPending(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "DeviceStatistics",
            "LowerPending",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Whether application I/O to the device is currently throttled.\n"]
    pub fn get_upper_blocked(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(DeviceStatistics::UpperBlocked(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "DeviceStatistics",
            "UpperBlocked",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Whether I/O to the backing device is currently blocked.\n"]
    pub fn get_lower_blocked(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(DeviceStatistics::LowerBlocked(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "DeviceStatistics",
            "LowerBlocked",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Whether updates to the activity log are currently suspended.\n"]
    pub fn get_al_suspended(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(DeviceStatistics::AlSuspended(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "DeviceStatistics",
            "AlSuspended",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Data generation UUID currently exposed to peers, identifying the\ngeneration of data this node has.\n"]
    pub fn get_exposed_data_uuid(&self) -> Result<u64, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(DeviceStatistics::ExposedDataUuid(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "DeviceStatistics",
            "ExposedDataUuid",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Current data generation UUID of the device\\'s metadata.\n"]
    pub fn get_current_uuid(&self) -> Result<u64, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(DeviceStatistics::CurrentUuid(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "DeviceStatistics",
            "CurrentUuid",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Bitmask of the backing disk\\'s metadata flags.\n"]
    pub fn get_disk_flags(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(DeviceStatistics::DiskFlags(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "DeviceStatistics",
            "DiskFlags",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Historical bitmap UUIDs of prior data generations, one slot per peer.\n"]
    pub fn get_history_uuids(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(DeviceStatistics::HistoryUuids(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "DeviceStatistics",
            "HistoryUuids",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl DeviceStatistics<'_> {
    pub fn new<'a>(buf: &'a [u8]) -> IterableDeviceStatistics<'a> {
        IterableDeviceStatistics::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        let res = match r#type {
            1u16 => "Size",
            2u16 => "Read",
            3u16 => "Write",
            4u16 => "AlWrites",
            5u16 => "BmWrites",
            6u16 => "UpperPending",
            7u16 => "LowerPending",
            8u16 => "UpperBlocked",
            9u16 => "LowerBlocked",
            10u16 => "AlSuspended",
            11u16 => "ExposedDataUuid",
            12u16 => "CurrentUuid",
            13u16 => "DiskFlags",
            14u16 => "HistoryUuids",
            _ => return None,
        };
        Some(res)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableDeviceStatistics<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableDeviceStatistics<'a> {
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
impl<'a> Iterator for IterableDeviceStatistics<'a> {
    type Item = Result<DeviceStatistics<'a>, ErrorContext>;
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
                1u16 => DeviceStatistics::Size({
                    let res = parse_u64(next);
                    let Some(val) = res else { break };
                    val
                }),
                2u16 => DeviceStatistics::Read({
                    let res = parse_u64(next);
                    let Some(val) = res else { break };
                    val
                }),
                3u16 => DeviceStatistics::Write({
                    let res = parse_u64(next);
                    let Some(val) = res else { break };
                    val
                }),
                4u16 => DeviceStatistics::AlWrites({
                    let res = parse_u64(next);
                    let Some(val) = res else { break };
                    val
                }),
                5u16 => DeviceStatistics::BmWrites({
                    let res = parse_u64(next);
                    let Some(val) = res else { break };
                    val
                }),
                6u16 => DeviceStatistics::UpperPending({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                7u16 => DeviceStatistics::LowerPending({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                8u16 => DeviceStatistics::UpperBlocked({
                    let res = parse_u8(next);
                    let Some(val) = res else { break };
                    val
                }),
                9u16 => DeviceStatistics::LowerBlocked({
                    let res = parse_u8(next);
                    let Some(val) = res else { break };
                    val
                }),
                10u16 => DeviceStatistics::AlSuspended({
                    let res = parse_u8(next);
                    let Some(val) = res else { break };
                    val
                }),
                11u16 => DeviceStatistics::ExposedDataUuid({
                    let res = parse_u64(next);
                    let Some(val) = res else { break };
                    val
                }),
                12u16 => DeviceStatistics::CurrentUuid({
                    let res = parse_u64(next);
                    let Some(val) = res else { break };
                    val
                }),
                13u16 => DeviceStatistics::DiskFlags({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                14u16 => DeviceStatistics::HistoryUuids({
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
            "DeviceStatistics",
            r#type.and_then(|t| DeviceStatistics::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl<'a> std::fmt::Debug for IterableDeviceStatistics<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("DeviceStatistics");
        let mut iter = IterableDeviceStatistics::with_loc(&[], self.orig_loc);
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
                DeviceStatistics::Size(val) => fmt.field("Size", &val),
                DeviceStatistics::Read(val) => fmt.field("Read", &val),
                DeviceStatistics::Write(val) => fmt.field("Write", &val),
                DeviceStatistics::AlWrites(val) => fmt.field("AlWrites", &val),
                DeviceStatistics::BmWrites(val) => fmt.field("BmWrites", &val),
                DeviceStatistics::UpperPending(val) => fmt.field("UpperPending", &val),
                DeviceStatistics::LowerPending(val) => fmt.field("LowerPending", &val),
                DeviceStatistics::UpperBlocked(val) => fmt.field("UpperBlocked", &val),
                DeviceStatistics::LowerBlocked(val) => fmt.field("LowerBlocked", &val),
                DeviceStatistics::AlSuspended(val) => fmt.field("AlSuspended", &val),
                DeviceStatistics::ExposedDataUuid(val) => fmt.field("ExposedDataUuid", &val),
                DeviceStatistics::CurrentUuid(val) => fmt.field("CurrentUuid", &val),
                DeviceStatistics::DiskFlags(val) => fmt.field("DiskFlags", &val),
                DeviceStatistics::HistoryUuids(val) => {
                    fmt.field("HistoryUuids", &FormatHexdump(val))
                }
            };
        }
        fmt.finish()
    }
}
impl IterableDeviceStatistics<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("DeviceStatistics", offset));
            return (
                stack,
                missing_type.and_then(|t| DeviceStatistics::attr_from_type(t)),
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
                DeviceStatistics::Size(val) => {
                    if last_off == offset {
                        stack.push(("Size", last_off));
                        break;
                    }
                }
                DeviceStatistics::Read(val) => {
                    if last_off == offset {
                        stack.push(("Read", last_off));
                        break;
                    }
                }
                DeviceStatistics::Write(val) => {
                    if last_off == offset {
                        stack.push(("Write", last_off));
                        break;
                    }
                }
                DeviceStatistics::AlWrites(val) => {
                    if last_off == offset {
                        stack.push(("AlWrites", last_off));
                        break;
                    }
                }
                DeviceStatistics::BmWrites(val) => {
                    if last_off == offset {
                        stack.push(("BmWrites", last_off));
                        break;
                    }
                }
                DeviceStatistics::UpperPending(val) => {
                    if last_off == offset {
                        stack.push(("UpperPending", last_off));
                        break;
                    }
                }
                DeviceStatistics::LowerPending(val) => {
                    if last_off == offset {
                        stack.push(("LowerPending", last_off));
                        break;
                    }
                }
                DeviceStatistics::UpperBlocked(val) => {
                    if last_off == offset {
                        stack.push(("UpperBlocked", last_off));
                        break;
                    }
                }
                DeviceStatistics::LowerBlocked(val) => {
                    if last_off == offset {
                        stack.push(("LowerBlocked", last_off));
                        break;
                    }
                }
                DeviceStatistics::AlSuspended(val) => {
                    if last_off == offset {
                        stack.push(("AlSuspended", last_off));
                        break;
                    }
                }
                DeviceStatistics::ExposedDataUuid(val) => {
                    if last_off == offset {
                        stack.push(("ExposedDataUuid", last_off));
                        break;
                    }
                }
                DeviceStatistics::CurrentUuid(val) => {
                    if last_off == offset {
                        stack.push(("CurrentUuid", last_off));
                        break;
                    }
                }
                DeviceStatistics::DiskFlags(val) => {
                    if last_off == offset {
                        stack.push(("DiskFlags", last_off));
                        break;
                    }
                }
                DeviceStatistics::HistoryUuids(val) => {
                    if last_off == offset {
                        stack.push(("HistoryUuids", last_off));
                        break;
                    }
                }
                _ => {}
            };
            last_off = cur + attrs.pos;
        }
        if !stack.is_empty() {
            stack.push(("DeviceStatistics", cur));
        }
        (stack, None)
    }
}
#[derive(Clone)]
pub enum ConnectionStatistics {
    #[doc = "Whether the network transport to the peer is currently congested.\n"]
    Congested(u8),
    #[doc = "Application data sent to the peer but not yet acknowledged, in sectors.\n"]
    ApInFlight(u64),
    #[doc = "Resync data sent to the peer but not yet acknowledged, in sectors.\n"]
    RsInFlight(u64),
}
impl<'a> IterableConnectionStatistics<'a> {
    #[doc = "Whether the network transport to the peer is currently congested.\n"]
    pub fn get_congested(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(ConnectionStatistics::Congested(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "ConnectionStatistics",
            "Congested",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Application data sent to the peer but not yet acknowledged, in sectors.\n"]
    pub fn get_ap_in_flight(&self) -> Result<u64, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(ConnectionStatistics::ApInFlight(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "ConnectionStatistics",
            "ApInFlight",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Resync data sent to the peer but not yet acknowledged, in sectors.\n"]
    pub fn get_rs_in_flight(&self) -> Result<u64, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(ConnectionStatistics::RsInFlight(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "ConnectionStatistics",
            "RsInFlight",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl ConnectionStatistics {
    pub fn new<'a>(buf: &'a [u8]) -> IterableConnectionStatistics<'a> {
        IterableConnectionStatistics::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        let res = match r#type {
            1u16 => "Congested",
            2u16 => "ApInFlight",
            3u16 => "RsInFlight",
            _ => return None,
        };
        Some(res)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableConnectionStatistics<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableConnectionStatistics<'a> {
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
impl<'a> Iterator for IterableConnectionStatistics<'a> {
    type Item = Result<ConnectionStatistics, ErrorContext>;
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
                1u16 => ConnectionStatistics::Congested({
                    let res = parse_u8(next);
                    let Some(val) = res else { break };
                    val
                }),
                2u16 => ConnectionStatistics::ApInFlight({
                    let res = parse_u64(next);
                    let Some(val) = res else { break };
                    val
                }),
                3u16 => ConnectionStatistics::RsInFlight({
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
            "ConnectionStatistics",
            r#type.and_then(|t| ConnectionStatistics::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl std::fmt::Debug for IterableConnectionStatistics<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("ConnectionStatistics");
        let mut iter = IterableConnectionStatistics::with_loc(&[], self.orig_loc);
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
                ConnectionStatistics::Congested(val) => fmt.field("Congested", &val),
                ConnectionStatistics::ApInFlight(val) => fmt.field("ApInFlight", &val),
                ConnectionStatistics::RsInFlight(val) => fmt.field("RsInFlight", &val),
            };
        }
        fmt.finish()
    }
}
impl IterableConnectionStatistics<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("ConnectionStatistics", offset));
            return (
                stack,
                missing_type.and_then(|t| ConnectionStatistics::attr_from_type(t)),
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
                ConnectionStatistics::Congested(val) => {
                    if last_off == offset {
                        stack.push(("Congested", last_off));
                        break;
                    }
                }
                ConnectionStatistics::ApInFlight(val) => {
                    if last_off == offset {
                        stack.push(("ApInFlight", last_off));
                        break;
                    }
                }
                ConnectionStatistics::RsInFlight(val) => {
                    if last_off == offset {
                        stack.push(("RsInFlight", last_off));
                        break;
                    }
                }
                _ => {}
            };
            last_off = cur + attrs.pos;
        }
        if !stack.is_empty() {
            stack.push(("ConnectionStatistics", cur));
        }
        (stack, None)
    }
}
#[derive(Clone)]
pub enum PeerDeviceStatistics {
    #[doc = "Total data received from the peer for this device, in sectors.\n"]
    Received(u64),
    #[doc = "Total data sent to the peer for this device, in sectors.\n"]
    Sent(u64),
    #[doc = "Number of requests to this peer waiting for an acknowledgement.\n"]
    Pending(u32),
    #[doc = "Number of requests from this peer that have not yet been acknowledged.\n"]
    Unacked(u32),
    #[doc = "Amount of data known to differ from this peer, in sectors.\n"]
    OutOfSync(u64),
    #[doc = "Amount of data that failed to resynchronize with this peer, in sectors.\n"]
    ResyncFailed(u64),
    #[doc = "Bitmap UUID recorded for this peer.\n"]
    BitmapUuid(u64),
    #[doc = "Bitmask of this peer device\\'s metadata flags.\n"]
    Flags(u32),
    #[doc = "Total amount of data to resynchronize with this peer for the current\nresync, in sectors.\n"]
    RsTotal(u64),
    #[doc = "Sector at which the current online verify run started.\n"]
    OvStartSector(u64),
    #[doc = "Sector at which the current online verify run will stop.\n"]
    OvStopSector(u64),
    #[doc = "Current position of an ongoing online verify run, in sectors.\n"]
    OvPosition(u64),
    #[doc = "Amount of data left to verify in an ongoing online verify run, in\nsectors.\n"]
    OvLeft(u64),
    #[doc = "Amount of data skipped during an ongoing online verify run, in sectors.\n"]
    OvSkipped(u64),
    #[doc = "Amount of resync data found identical by checksum comparison and not\nretransmitted, in sectors.\n"]
    RsSameCsum(u64),
    #[doc = "Time elapsed since the current resync with this peer started, in\nmilliseconds.\n"]
    RsDtStartMs(u64),
    #[doc = "Total time the current resync with this peer has been paused, in\nmilliseconds.\n"]
    RsPausedMs(u64),
    #[doc = "Time since the most recent resync progress mark, in milliseconds.\n"]
    RsDt0Ms(u64),
    #[doc = "Data resynchronized since the most recent resync progress mark, in\nsectors.\n"]
    RsDb0Sectors(u64),
    #[doc = "Time since the resync progress mark before the most recent one, in\nmilliseconds.\n"]
    RsDt1Ms(u64),
    #[doc = "Data resynchronized since the resync progress mark before the most\nrecent one, in sectors.\n"]
    RsDb1Sectors(u64),
    #[doc = "Current dynamically controlled resync rate for this peer, in KiB/s.\n"]
    RsCSyncRate(u32),
    #[doc = "Bitmask of UUID-related flags negotiated with this peer.\n"]
    UuidFlags(u64),
}
impl<'a> IterablePeerDeviceStatistics<'a> {
    #[doc = "Total data received from the peer for this device, in sectors.\n"]
    pub fn get_received(&self) -> Result<u64, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(PeerDeviceStatistics::Received(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "PeerDeviceStatistics",
            "Received",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Total data sent to the peer for this device, in sectors.\n"]
    pub fn get_sent(&self) -> Result<u64, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(PeerDeviceStatistics::Sent(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "PeerDeviceStatistics",
            "Sent",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Number of requests to this peer waiting for an acknowledgement.\n"]
    pub fn get_pending(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(PeerDeviceStatistics::Pending(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "PeerDeviceStatistics",
            "Pending",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Number of requests from this peer that have not yet been acknowledged.\n"]
    pub fn get_unacked(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(PeerDeviceStatistics::Unacked(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "PeerDeviceStatistics",
            "Unacked",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Amount of data known to differ from this peer, in sectors.\n"]
    pub fn get_out_of_sync(&self) -> Result<u64, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(PeerDeviceStatistics::OutOfSync(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "PeerDeviceStatistics",
            "OutOfSync",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Amount of data that failed to resynchronize with this peer, in sectors.\n"]
    pub fn get_resync_failed(&self) -> Result<u64, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(PeerDeviceStatistics::ResyncFailed(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "PeerDeviceStatistics",
            "ResyncFailed",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Bitmap UUID recorded for this peer.\n"]
    pub fn get_bitmap_uuid(&self) -> Result<u64, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(PeerDeviceStatistics::BitmapUuid(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "PeerDeviceStatistics",
            "BitmapUuid",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Bitmask of this peer device\\'s metadata flags.\n"]
    pub fn get_flags(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(PeerDeviceStatistics::Flags(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "PeerDeviceStatistics",
            "Flags",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Total amount of data to resynchronize with this peer for the current\nresync, in sectors.\n"]
    pub fn get_rs_total(&self) -> Result<u64, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(PeerDeviceStatistics::RsTotal(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "PeerDeviceStatistics",
            "RsTotal",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Sector at which the current online verify run started.\n"]
    pub fn get_ov_start_sector(&self) -> Result<u64, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(PeerDeviceStatistics::OvStartSector(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "PeerDeviceStatistics",
            "OvStartSector",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Sector at which the current online verify run will stop.\n"]
    pub fn get_ov_stop_sector(&self) -> Result<u64, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(PeerDeviceStatistics::OvStopSector(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "PeerDeviceStatistics",
            "OvStopSector",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Current position of an ongoing online verify run, in sectors.\n"]
    pub fn get_ov_position(&self) -> Result<u64, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(PeerDeviceStatistics::OvPosition(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "PeerDeviceStatistics",
            "OvPosition",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Amount of data left to verify in an ongoing online verify run, in\nsectors.\n"]
    pub fn get_ov_left(&self) -> Result<u64, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(PeerDeviceStatistics::OvLeft(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "PeerDeviceStatistics",
            "OvLeft",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Amount of data skipped during an ongoing online verify run, in sectors.\n"]
    pub fn get_ov_skipped(&self) -> Result<u64, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(PeerDeviceStatistics::OvSkipped(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "PeerDeviceStatistics",
            "OvSkipped",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Amount of resync data found identical by checksum comparison and not\nretransmitted, in sectors.\n"]
    pub fn get_rs_same_csum(&self) -> Result<u64, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(PeerDeviceStatistics::RsSameCsum(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "PeerDeviceStatistics",
            "RsSameCsum",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Time elapsed since the current resync with this peer started, in\nmilliseconds.\n"]
    pub fn get_rs_dt_start_ms(&self) -> Result<u64, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(PeerDeviceStatistics::RsDtStartMs(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "PeerDeviceStatistics",
            "RsDtStartMs",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Total time the current resync with this peer has been paused, in\nmilliseconds.\n"]
    pub fn get_rs_paused_ms(&self) -> Result<u64, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(PeerDeviceStatistics::RsPausedMs(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "PeerDeviceStatistics",
            "RsPausedMs",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Time since the most recent resync progress mark, in milliseconds.\n"]
    pub fn get_rs_dt0_ms(&self) -> Result<u64, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(PeerDeviceStatistics::RsDt0Ms(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "PeerDeviceStatistics",
            "RsDt0Ms",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Data resynchronized since the most recent resync progress mark, in\nsectors.\n"]
    pub fn get_rs_db0_sectors(&self) -> Result<u64, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(PeerDeviceStatistics::RsDb0Sectors(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "PeerDeviceStatistics",
            "RsDb0Sectors",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Time since the resync progress mark before the most recent one, in\nmilliseconds.\n"]
    pub fn get_rs_dt1_ms(&self) -> Result<u64, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(PeerDeviceStatistics::RsDt1Ms(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "PeerDeviceStatistics",
            "RsDt1Ms",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Data resynchronized since the resync progress mark before the most\nrecent one, in sectors.\n"]
    pub fn get_rs_db1_sectors(&self) -> Result<u64, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(PeerDeviceStatistics::RsDb1Sectors(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "PeerDeviceStatistics",
            "RsDb1Sectors",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Current dynamically controlled resync rate for this peer, in KiB/s.\n"]
    pub fn get_rs_c_sync_rate(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(PeerDeviceStatistics::RsCSyncRate(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "PeerDeviceStatistics",
            "RsCSyncRate",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Bitmask of UUID-related flags negotiated with this peer.\n"]
    pub fn get_uuid_flags(&self) -> Result<u64, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(PeerDeviceStatistics::UuidFlags(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "PeerDeviceStatistics",
            "UuidFlags",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl PeerDeviceStatistics {
    pub fn new<'a>(buf: &'a [u8]) -> IterablePeerDeviceStatistics<'a> {
        IterablePeerDeviceStatistics::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        let res = match r#type {
            1u16 => "Received",
            2u16 => "Sent",
            3u16 => "Pending",
            4u16 => "Unacked",
            5u16 => "OutOfSync",
            6u16 => "ResyncFailed",
            7u16 => "BitmapUuid",
            8u16 => "Flags",
            9u16 => "RsTotal",
            10u16 => "OvStartSector",
            11u16 => "OvStopSector",
            12u16 => "OvPosition",
            13u16 => "OvLeft",
            14u16 => "OvSkipped",
            15u16 => "RsSameCsum",
            16u16 => "RsDtStartMs",
            17u16 => "RsPausedMs",
            18u16 => "RsDt0Ms",
            19u16 => "RsDb0Sectors",
            20u16 => "RsDt1Ms",
            21u16 => "RsDb1Sectors",
            22u16 => "RsCSyncRate",
            23u16 => "UuidFlags",
            _ => return None,
        };
        Some(res)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterablePeerDeviceStatistics<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterablePeerDeviceStatistics<'a> {
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
impl<'a> Iterator for IterablePeerDeviceStatistics<'a> {
    type Item = Result<PeerDeviceStatistics, ErrorContext>;
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
                1u16 => PeerDeviceStatistics::Received({
                    let res = parse_u64(next);
                    let Some(val) = res else { break };
                    val
                }),
                2u16 => PeerDeviceStatistics::Sent({
                    let res = parse_u64(next);
                    let Some(val) = res else { break };
                    val
                }),
                3u16 => PeerDeviceStatistics::Pending({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                4u16 => PeerDeviceStatistics::Unacked({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                5u16 => PeerDeviceStatistics::OutOfSync({
                    let res = parse_u64(next);
                    let Some(val) = res else { break };
                    val
                }),
                6u16 => PeerDeviceStatistics::ResyncFailed({
                    let res = parse_u64(next);
                    let Some(val) = res else { break };
                    val
                }),
                7u16 => PeerDeviceStatistics::BitmapUuid({
                    let res = parse_u64(next);
                    let Some(val) = res else { break };
                    val
                }),
                8u16 => PeerDeviceStatistics::Flags({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                9u16 => PeerDeviceStatistics::RsTotal({
                    let res = parse_u64(next);
                    let Some(val) = res else { break };
                    val
                }),
                10u16 => PeerDeviceStatistics::OvStartSector({
                    let res = parse_u64(next);
                    let Some(val) = res else { break };
                    val
                }),
                11u16 => PeerDeviceStatistics::OvStopSector({
                    let res = parse_u64(next);
                    let Some(val) = res else { break };
                    val
                }),
                12u16 => PeerDeviceStatistics::OvPosition({
                    let res = parse_u64(next);
                    let Some(val) = res else { break };
                    val
                }),
                13u16 => PeerDeviceStatistics::OvLeft({
                    let res = parse_u64(next);
                    let Some(val) = res else { break };
                    val
                }),
                14u16 => PeerDeviceStatistics::OvSkipped({
                    let res = parse_u64(next);
                    let Some(val) = res else { break };
                    val
                }),
                15u16 => PeerDeviceStatistics::RsSameCsum({
                    let res = parse_u64(next);
                    let Some(val) = res else { break };
                    val
                }),
                16u16 => PeerDeviceStatistics::RsDtStartMs({
                    let res = parse_u64(next);
                    let Some(val) = res else { break };
                    val
                }),
                17u16 => PeerDeviceStatistics::RsPausedMs({
                    let res = parse_u64(next);
                    let Some(val) = res else { break };
                    val
                }),
                18u16 => PeerDeviceStatistics::RsDt0Ms({
                    let res = parse_u64(next);
                    let Some(val) = res else { break };
                    val
                }),
                19u16 => PeerDeviceStatistics::RsDb0Sectors({
                    let res = parse_u64(next);
                    let Some(val) = res else { break };
                    val
                }),
                20u16 => PeerDeviceStatistics::RsDt1Ms({
                    let res = parse_u64(next);
                    let Some(val) = res else { break };
                    val
                }),
                21u16 => PeerDeviceStatistics::RsDb1Sectors({
                    let res = parse_u64(next);
                    let Some(val) = res else { break };
                    val
                }),
                22u16 => PeerDeviceStatistics::RsCSyncRate({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                23u16 => PeerDeviceStatistics::UuidFlags({
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
            "PeerDeviceStatistics",
            r#type.and_then(|t| PeerDeviceStatistics::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl std::fmt::Debug for IterablePeerDeviceStatistics<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("PeerDeviceStatistics");
        let mut iter = IterablePeerDeviceStatistics::with_loc(&[], self.orig_loc);
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
                PeerDeviceStatistics::Received(val) => fmt.field("Received", &val),
                PeerDeviceStatistics::Sent(val) => fmt.field("Sent", &val),
                PeerDeviceStatistics::Pending(val) => fmt.field("Pending", &val),
                PeerDeviceStatistics::Unacked(val) => fmt.field("Unacked", &val),
                PeerDeviceStatistics::OutOfSync(val) => fmt.field("OutOfSync", &val),
                PeerDeviceStatistics::ResyncFailed(val) => fmt.field("ResyncFailed", &val),
                PeerDeviceStatistics::BitmapUuid(val) => fmt.field("BitmapUuid", &val),
                PeerDeviceStatistics::Flags(val) => fmt.field("Flags", &val),
                PeerDeviceStatistics::RsTotal(val) => fmt.field("RsTotal", &val),
                PeerDeviceStatistics::OvStartSector(val) => fmt.field("OvStartSector", &val),
                PeerDeviceStatistics::OvStopSector(val) => fmt.field("OvStopSector", &val),
                PeerDeviceStatistics::OvPosition(val) => fmt.field("OvPosition", &val),
                PeerDeviceStatistics::OvLeft(val) => fmt.field("OvLeft", &val),
                PeerDeviceStatistics::OvSkipped(val) => fmt.field("OvSkipped", &val),
                PeerDeviceStatistics::RsSameCsum(val) => fmt.field("RsSameCsum", &val),
                PeerDeviceStatistics::RsDtStartMs(val) => fmt.field("RsDtStartMs", &val),
                PeerDeviceStatistics::RsPausedMs(val) => fmt.field("RsPausedMs", &val),
                PeerDeviceStatistics::RsDt0Ms(val) => fmt.field("RsDt0Ms", &val),
                PeerDeviceStatistics::RsDb0Sectors(val) => fmt.field("RsDb0Sectors", &val),
                PeerDeviceStatistics::RsDt1Ms(val) => fmt.field("RsDt1Ms", &val),
                PeerDeviceStatistics::RsDb1Sectors(val) => fmt.field("RsDb1Sectors", &val),
                PeerDeviceStatistics::RsCSyncRate(val) => fmt.field("RsCSyncRate", &val),
                PeerDeviceStatistics::UuidFlags(val) => fmt.field("UuidFlags", &val),
            };
        }
        fmt.finish()
    }
}
impl IterablePeerDeviceStatistics<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("PeerDeviceStatistics", offset));
            return (
                stack,
                missing_type.and_then(|t| PeerDeviceStatistics::attr_from_type(t)),
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
                PeerDeviceStatistics::Received(val) => {
                    if last_off == offset {
                        stack.push(("Received", last_off));
                        break;
                    }
                }
                PeerDeviceStatistics::Sent(val) => {
                    if last_off == offset {
                        stack.push(("Sent", last_off));
                        break;
                    }
                }
                PeerDeviceStatistics::Pending(val) => {
                    if last_off == offset {
                        stack.push(("Pending", last_off));
                        break;
                    }
                }
                PeerDeviceStatistics::Unacked(val) => {
                    if last_off == offset {
                        stack.push(("Unacked", last_off));
                        break;
                    }
                }
                PeerDeviceStatistics::OutOfSync(val) => {
                    if last_off == offset {
                        stack.push(("OutOfSync", last_off));
                        break;
                    }
                }
                PeerDeviceStatistics::ResyncFailed(val) => {
                    if last_off == offset {
                        stack.push(("ResyncFailed", last_off));
                        break;
                    }
                }
                PeerDeviceStatistics::BitmapUuid(val) => {
                    if last_off == offset {
                        stack.push(("BitmapUuid", last_off));
                        break;
                    }
                }
                PeerDeviceStatistics::Flags(val) => {
                    if last_off == offset {
                        stack.push(("Flags", last_off));
                        break;
                    }
                }
                PeerDeviceStatistics::RsTotal(val) => {
                    if last_off == offset {
                        stack.push(("RsTotal", last_off));
                        break;
                    }
                }
                PeerDeviceStatistics::OvStartSector(val) => {
                    if last_off == offset {
                        stack.push(("OvStartSector", last_off));
                        break;
                    }
                }
                PeerDeviceStatistics::OvStopSector(val) => {
                    if last_off == offset {
                        stack.push(("OvStopSector", last_off));
                        break;
                    }
                }
                PeerDeviceStatistics::OvPosition(val) => {
                    if last_off == offset {
                        stack.push(("OvPosition", last_off));
                        break;
                    }
                }
                PeerDeviceStatistics::OvLeft(val) => {
                    if last_off == offset {
                        stack.push(("OvLeft", last_off));
                        break;
                    }
                }
                PeerDeviceStatistics::OvSkipped(val) => {
                    if last_off == offset {
                        stack.push(("OvSkipped", last_off));
                        break;
                    }
                }
                PeerDeviceStatistics::RsSameCsum(val) => {
                    if last_off == offset {
                        stack.push(("RsSameCsum", last_off));
                        break;
                    }
                }
                PeerDeviceStatistics::RsDtStartMs(val) => {
                    if last_off == offset {
                        stack.push(("RsDtStartMs", last_off));
                        break;
                    }
                }
                PeerDeviceStatistics::RsPausedMs(val) => {
                    if last_off == offset {
                        stack.push(("RsPausedMs", last_off));
                        break;
                    }
                }
                PeerDeviceStatistics::RsDt0Ms(val) => {
                    if last_off == offset {
                        stack.push(("RsDt0Ms", last_off));
                        break;
                    }
                }
                PeerDeviceStatistics::RsDb0Sectors(val) => {
                    if last_off == offset {
                        stack.push(("RsDb0Sectors", last_off));
                        break;
                    }
                }
                PeerDeviceStatistics::RsDt1Ms(val) => {
                    if last_off == offset {
                        stack.push(("RsDt1Ms", last_off));
                        break;
                    }
                }
                PeerDeviceStatistics::RsDb1Sectors(val) => {
                    if last_off == offset {
                        stack.push(("RsDb1Sectors", last_off));
                        break;
                    }
                }
                PeerDeviceStatistics::RsCSyncRate(val) => {
                    if last_off == offset {
                        stack.push(("RsCSyncRate", last_off));
                        break;
                    }
                }
                PeerDeviceStatistics::UuidFlags(val) => {
                    if last_off == offset {
                        stack.push(("UuidFlags", last_off));
                        break;
                    }
                }
                _ => {}
            };
            last_off = cur + attrs.pos;
        }
        if !stack.is_empty() {
            stack.push(("PeerDeviceStatistics", cur));
        }
        (stack, None)
    }
}
#[derive(Clone)]
pub enum HelperInfo<'a> {
    #[doc = "Name of the handler script being invoked.\n"]
    Name(&'a CStr),
    #[doc = "Exit status of the handler script, present when phase is response.\n"]
    Status(u32),
    #[doc = "Whether this notification announces the call or reports its result.\n\nAssociated type: [`HelperPhase`] (enum)"]
    Phase(u32),
}
impl<'a> IterableHelperInfo<'a> {
    #[doc = "Name of the handler script being invoked.\n"]
    pub fn get_name(&self) -> Result<&'a CStr, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(HelperInfo::Name(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "HelperInfo",
            "Name",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Exit status of the handler script, present when phase is response.\n"]
    pub fn get_status(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(HelperInfo::Status(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "HelperInfo",
            "Status",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Whether this notification announces the call or reports its result.\n\nAssociated type: [`HelperPhase`] (enum)"]
    pub fn get_phase(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(HelperInfo::Phase(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "HelperInfo",
            "Phase",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl HelperInfo<'_> {
    pub fn new<'a>(buf: &'a [u8]) -> IterableHelperInfo<'a> {
        IterableHelperInfo::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        let res = match r#type {
            1u16 => "Name",
            2u16 => "Status",
            3u16 => "Phase",
            _ => return None,
        };
        Some(res)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableHelperInfo<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableHelperInfo<'a> {
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
impl<'a> Iterator for IterableHelperInfo<'a> {
    type Item = Result<HelperInfo<'a>, ErrorContext>;
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
                1u16 => HelperInfo::Name({
                    let res = CStr::from_bytes_with_nul(next).ok();
                    let Some(val) = res else { break };
                    val
                }),
                2u16 => HelperInfo::Status({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                3u16 => HelperInfo::Phase({
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
            "HelperInfo",
            r#type.and_then(|t| HelperInfo::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl<'a> std::fmt::Debug for IterableHelperInfo<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("HelperInfo");
        let mut iter = IterableHelperInfo::with_loc(&[], self.orig_loc);
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
                HelperInfo::Name(val) => fmt.field("Name", &val),
                HelperInfo::Status(val) => fmt.field("Status", &val),
                HelperInfo::Phase(val) => {
                    fmt.field("Phase", &FormatEnum(val.into(), HelperPhase::from_value))
                }
            };
        }
        fmt.finish()
    }
}
impl IterableHelperInfo<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("HelperInfo", offset));
            return (
                stack,
                missing_type.and_then(|t| HelperInfo::attr_from_type(t)),
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
                HelperInfo::Name(val) => {
                    if last_off == offset {
                        stack.push(("Name", last_off));
                        break;
                    }
                }
                HelperInfo::Status(val) => {
                    if last_off == offset {
                        stack.push(("Status", last_off));
                        break;
                    }
                }
                HelperInfo::Phase(val) => {
                    if last_off == offset {
                        stack.push(("Phase", last_off));
                        break;
                    }
                }
                _ => {}
            };
            last_off = cur + attrs.pos;
        }
        if !stack.is_empty() {
            stack.push(("HelperInfo", cur));
        }
        (stack, None)
    }
}
#[derive(Clone)]
pub enum InvalidateParms {
    #[doc = "Node ID of the peer to resynchronize from; if absent, the kernel chooses\na peer.\n"]
    SyncFromPeerNodeId(u32),
    #[doc = "Whether to mark the whole bitmap out of sync before starting the resync,\ninstead of resyncing only the blocks already marked out of sync\n(drbdsetup invalidate option reset-bitmap).\n"]
    ResetBitmap(u8),
}
impl<'a> IterableInvalidateParms<'a> {
    #[doc = "Node ID of the peer to resynchronize from; if absent, the kernel chooses\na peer.\n"]
    pub fn get_sync_from_peer_node_id(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(InvalidateParms::SyncFromPeerNodeId(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "InvalidateParms",
            "SyncFromPeerNodeId",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Whether to mark the whole bitmap out of sync before starting the resync,\ninstead of resyncing only the blocks already marked out of sync\n(drbdsetup invalidate option reset-bitmap).\n"]
    pub fn get_reset_bitmap(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(InvalidateParms::ResetBitmap(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "InvalidateParms",
            "ResetBitmap",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl InvalidateParms {
    pub fn new<'a>(buf: &'a [u8]) -> IterableInvalidateParms<'a> {
        IterableInvalidateParms::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        let res = match r#type {
            1u16 => "SyncFromPeerNodeId",
            2u16 => "ResetBitmap",
            _ => return None,
        };
        Some(res)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableInvalidateParms<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableInvalidateParms<'a> {
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
impl<'a> Iterator for IterableInvalidateParms<'a> {
    type Item = Result<InvalidateParms, ErrorContext>;
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
                1u16 => InvalidateParms::SyncFromPeerNodeId({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                2u16 => InvalidateParms::ResetBitmap({
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
            "InvalidateParms",
            r#type.and_then(|t| InvalidateParms::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl std::fmt::Debug for IterableInvalidateParms<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("InvalidateParms");
        let mut iter = IterableInvalidateParms::with_loc(&[], self.orig_loc);
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
                InvalidateParms::SyncFromPeerNodeId(val) => fmt.field("SyncFromPeerNodeId", &val),
                InvalidateParms::ResetBitmap(val) => fmt.field("ResetBitmap", &val),
            };
        }
        fmt.finish()
    }
}
impl IterableInvalidateParms<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("InvalidateParms", offset));
            return (
                stack,
                missing_type.and_then(|t| InvalidateParms::attr_from_type(t)),
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
                InvalidateParms::SyncFromPeerNodeId(val) => {
                    if last_off == offset {
                        stack.push(("SyncFromPeerNodeId", last_off));
                        break;
                    }
                }
                InvalidateParms::ResetBitmap(val) => {
                    if last_off == offset {
                        stack.push(("ResetBitmap", last_off));
                        break;
                    }
                }
                _ => {}
            };
            last_off = cur + attrs.pos;
        }
        if !stack.is_empty() {
            stack.push(("InvalidateParms", cur));
        }
        (stack, None)
    }
}
#[derive(Clone)]
pub enum PeerDeviceConf {
    #[doc = "Bandwidth DRBD may use for resynchronizing with this peer when the\ndynamic resync controller is disabled, in KiB/s (drbd.conf option\nresync-rate).\n"]
    ResyncRate(u32),
    #[doc = "How fast the dynamic resync-rate controller adapts to changes in resync\nspeed, in units of 0.1 seconds, or 0 to use a fixed resync-rate instead\n(drbd.conf option c-plan-ahead).\n"]
    CPlanAhead(u32),
    #[doc = "Delay the dynamic resync-rate controller aims for along the resync path,\nin units of 0.1 seconds (drbd.conf option c-delay-target).\n"]
    CDelayTarget(u32),
    #[doc = "Amount of resync data the dynamic resync-rate controller aims to keep in\nflight, in sectors (drbd.conf option c-fill-target).\n"]
    CFillTarget(u32),
    #[doc = "Maximum bandwidth used by the dynamic resync-rate controller, in KiB/s,\nor 0 for no limit (drbd.conf option c-max-rate).\n"]
    CMaxRate(u32),
    #[doc = "Bandwidth reserved for resync I/O against this peer when this node is\nthe sync source and primary, in KiB/s (drbd.conf option c-min-rate).\n"]
    CMinRate(u32),
    #[doc = "Whether to allocate a bitmap slot for this peer, overriding the\nautomatic default for a diskless peer (drbd.conf option bitmap).\n"]
    Bitmap(u8),
    #[doc = "Whether to suspend replication of application writes to this peer while\nit is being resynchronized (drbd.conf option\nresync-without-replication).\n"]
    ResyncWithoutReplication(u8),
    #[doc = "Whether this peer participates as a tiebreaker in quorum decisions even\nthough it is intentionally diskless (drbd.conf option peer-tiebreaker).\n"]
    PeerTiebreaker(u8),
}
impl<'a> IterablePeerDeviceConf<'a> {
    #[doc = "Bandwidth DRBD may use for resynchronizing with this peer when the\ndynamic resync controller is disabled, in KiB/s (drbd.conf option\nresync-rate).\n"]
    pub fn get_resync_rate(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(PeerDeviceConf::ResyncRate(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "PeerDeviceConf",
            "ResyncRate",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "How fast the dynamic resync-rate controller adapts to changes in resync\nspeed, in units of 0.1 seconds, or 0 to use a fixed resync-rate instead\n(drbd.conf option c-plan-ahead).\n"]
    pub fn get_c_plan_ahead(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(PeerDeviceConf::CPlanAhead(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "PeerDeviceConf",
            "CPlanAhead",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Delay the dynamic resync-rate controller aims for along the resync path,\nin units of 0.1 seconds (drbd.conf option c-delay-target).\n"]
    pub fn get_c_delay_target(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(PeerDeviceConf::CDelayTarget(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "PeerDeviceConf",
            "CDelayTarget",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Amount of resync data the dynamic resync-rate controller aims to keep in\nflight, in sectors (drbd.conf option c-fill-target).\n"]
    pub fn get_c_fill_target(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(PeerDeviceConf::CFillTarget(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "PeerDeviceConf",
            "CFillTarget",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Maximum bandwidth used by the dynamic resync-rate controller, in KiB/s,\nor 0 for no limit (drbd.conf option c-max-rate).\n"]
    pub fn get_c_max_rate(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(PeerDeviceConf::CMaxRate(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "PeerDeviceConf",
            "CMaxRate",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Bandwidth reserved for resync I/O against this peer when this node is\nthe sync source and primary, in KiB/s (drbd.conf option c-min-rate).\n"]
    pub fn get_c_min_rate(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(PeerDeviceConf::CMinRate(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "PeerDeviceConf",
            "CMinRate",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Whether to allocate a bitmap slot for this peer, overriding the\nautomatic default for a diskless peer (drbd.conf option bitmap).\n"]
    pub fn get_bitmap(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(PeerDeviceConf::Bitmap(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "PeerDeviceConf",
            "Bitmap",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Whether to suspend replication of application writes to this peer while\nit is being resynchronized (drbd.conf option\nresync-without-replication).\n"]
    pub fn get_resync_without_replication(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(PeerDeviceConf::ResyncWithoutReplication(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "PeerDeviceConf",
            "ResyncWithoutReplication",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Whether this peer participates as a tiebreaker in quorum decisions even\nthough it is intentionally diskless (drbd.conf option peer-tiebreaker).\n"]
    pub fn get_peer_tiebreaker(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(PeerDeviceConf::PeerTiebreaker(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "PeerDeviceConf",
            "PeerTiebreaker",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl PeerDeviceConf {
    pub fn new<'a>(buf: &'a [u8]) -> IterablePeerDeviceConf<'a> {
        IterablePeerDeviceConf::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        let res = match r#type {
            1u16 => "ResyncRate",
            2u16 => "CPlanAhead",
            3u16 => "CDelayTarget",
            4u16 => "CFillTarget",
            5u16 => "CMaxRate",
            6u16 => "CMinRate",
            7u16 => "Bitmap",
            8u16 => "ResyncWithoutReplication",
            9u16 => "PeerTiebreaker",
            _ => return None,
        };
        Some(res)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterablePeerDeviceConf<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterablePeerDeviceConf<'a> {
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
impl<'a> Iterator for IterablePeerDeviceConf<'a> {
    type Item = Result<PeerDeviceConf, ErrorContext>;
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
                1u16 => PeerDeviceConf::ResyncRate({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                2u16 => PeerDeviceConf::CPlanAhead({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                3u16 => PeerDeviceConf::CDelayTarget({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                4u16 => PeerDeviceConf::CFillTarget({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                5u16 => PeerDeviceConf::CMaxRate({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                6u16 => PeerDeviceConf::CMinRate({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                7u16 => PeerDeviceConf::Bitmap({
                    let res = parse_u8(next);
                    let Some(val) = res else { break };
                    val
                }),
                8u16 => PeerDeviceConf::ResyncWithoutReplication({
                    let res = parse_u8(next);
                    let Some(val) = res else { break };
                    val
                }),
                9u16 => PeerDeviceConf::PeerTiebreaker({
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
            "PeerDeviceConf",
            r#type.and_then(|t| PeerDeviceConf::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl std::fmt::Debug for IterablePeerDeviceConf<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("PeerDeviceConf");
        let mut iter = IterablePeerDeviceConf::with_loc(&[], self.orig_loc);
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
                PeerDeviceConf::ResyncRate(val) => fmt.field("ResyncRate", &val),
                PeerDeviceConf::CPlanAhead(val) => fmt.field("CPlanAhead", &val),
                PeerDeviceConf::CDelayTarget(val) => fmt.field("CDelayTarget", &val),
                PeerDeviceConf::CFillTarget(val) => fmt.field("CFillTarget", &val),
                PeerDeviceConf::CMaxRate(val) => fmt.field("CMaxRate", &val),
                PeerDeviceConf::CMinRate(val) => fmt.field("CMinRate", &val),
                PeerDeviceConf::Bitmap(val) => fmt.field("Bitmap", &val),
                PeerDeviceConf::ResyncWithoutReplication(val) => {
                    fmt.field("ResyncWithoutReplication", &val)
                }
                PeerDeviceConf::PeerTiebreaker(val) => fmt.field("PeerTiebreaker", &val),
            };
        }
        fmt.finish()
    }
}
impl IterablePeerDeviceConf<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("PeerDeviceConf", offset));
            return (
                stack,
                missing_type.and_then(|t| PeerDeviceConf::attr_from_type(t)),
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
                PeerDeviceConf::ResyncRate(val) => {
                    if last_off == offset {
                        stack.push(("ResyncRate", last_off));
                        break;
                    }
                }
                PeerDeviceConf::CPlanAhead(val) => {
                    if last_off == offset {
                        stack.push(("CPlanAhead", last_off));
                        break;
                    }
                }
                PeerDeviceConf::CDelayTarget(val) => {
                    if last_off == offset {
                        stack.push(("CDelayTarget", last_off));
                        break;
                    }
                }
                PeerDeviceConf::CFillTarget(val) => {
                    if last_off == offset {
                        stack.push(("CFillTarget", last_off));
                        break;
                    }
                }
                PeerDeviceConf::CMaxRate(val) => {
                    if last_off == offset {
                        stack.push(("CMaxRate", last_off));
                        break;
                    }
                }
                PeerDeviceConf::CMinRate(val) => {
                    if last_off == offset {
                        stack.push(("CMinRate", last_off));
                        break;
                    }
                }
                PeerDeviceConf::Bitmap(val) => {
                    if last_off == offset {
                        stack.push(("Bitmap", last_off));
                        break;
                    }
                }
                PeerDeviceConf::ResyncWithoutReplication(val) => {
                    if last_off == offset {
                        stack.push(("ResyncWithoutReplication", last_off));
                        break;
                    }
                }
                PeerDeviceConf::PeerTiebreaker(val) => {
                    if last_off == offset {
                        stack.push(("PeerTiebreaker", last_off));
                        break;
                    }
                }
                _ => {}
            };
            last_off = cur + attrs.pos;
        }
        if !stack.is_empty() {
            stack.push(("PeerDeviceConf", cur));
        }
        (stack, None)
    }
}
#[derive(Clone)]
pub enum ConnectParms {
    #[doc = "Request that DRBD only determine whether a connection could be\nestablished and which resync direction would be used, without actually\nconnecting or resyncing (drbdsetup connect option tentative).\n"]
    Tentative(()),
    #[doc = "Request that local data be discarded and resynchronized from the peer\nthat has the most up-to-date data (drbdsetup connect option\ndiscard-my-data).\n"]
    DiscardMyData(()),
}
impl<'a> IterableConnectParms<'a> {
    #[doc = "Request that DRBD only determine whether a connection could be\nestablished and which resync direction would be used, without actually\nconnecting or resyncing (drbdsetup connect option tentative).\n"]
    pub fn get_tentative(&self) -> Result<(), ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(ConnectParms::Tentative(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "ConnectParms",
            "Tentative",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Request that local data be discarded and resynchronized from the peer\nthat has the most up-to-date data (drbdsetup connect option\ndiscard-my-data).\n"]
    pub fn get_discard_my_data(&self) -> Result<(), ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(ConnectParms::DiscardMyData(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "ConnectParms",
            "DiscardMyData",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl ConnectParms {
    pub fn new<'a>(buf: &'a [u8]) -> IterableConnectParms<'a> {
        IterableConnectParms::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        let res = match r#type {
            1u16 => "Tentative",
            2u16 => "DiscardMyData",
            _ => return None,
        };
        Some(res)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableConnectParms<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableConnectParms<'a> {
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
impl<'a> Iterator for IterableConnectParms<'a> {
    type Item = Result<ConnectParms, ErrorContext>;
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
                1u16 => ConnectParms::Tentative(()),
                2u16 => ConnectParms::DiscardMyData(()),
                n if cfg!(any(test, feature = "deny-unknown-attrs")) => break,
                n => continue,
            };
            return Some(Ok(res));
        }
        Some(Err(ErrorContext::new(
            "ConnectParms",
            r#type.and_then(|t| ConnectParms::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl std::fmt::Debug for IterableConnectParms<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("ConnectParms");
        let mut iter = IterableConnectParms::with_loc(&[], self.orig_loc);
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
                ConnectParms::Tentative(val) => fmt.field("Tentative", &val),
                ConnectParms::DiscardMyData(val) => fmt.field("DiscardMyData", &val),
            };
        }
        fmt.finish()
    }
}
impl IterableConnectParms<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("ConnectParms", offset));
            return (
                stack,
                missing_type.and_then(|t| ConnectParms::attr_from_type(t)),
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
                ConnectParms::Tentative(val) => {
                    if last_off == offset {
                        stack.push(("Tentative", last_off));
                        break;
                    }
                }
                ConnectParms::DiscardMyData(val) => {
                    if last_off == offset {
                        stack.push(("DiscardMyData", last_off));
                        break;
                    }
                }
                _ => {}
            };
            last_off = cur + attrs.pos;
        }
        if !stack.is_empty() {
            stack.push(("ConnectParms", cur));
        }
        (stack, None)
    }
}
#[derive(Clone)]
pub enum PathInfo {
    #[doc = "Whether the path\\'s transport connection is currently established.\n"]
    Established(u8),
}
impl<'a> IterablePathInfo<'a> {
    #[doc = "Whether the path\\'s transport connection is currently established.\n"]
    pub fn get_established(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(PathInfo::Established(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "PathInfo",
            "Established",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl PathInfo {
    pub fn new<'a>(buf: &'a [u8]) -> IterablePathInfo<'a> {
        IterablePathInfo::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        let res = match r#type {
            1u16 => "Established",
            _ => return None,
        };
        Some(res)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterablePathInfo<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterablePathInfo<'a> {
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
impl<'a> Iterator for IterablePathInfo<'a> {
    type Item = Result<PathInfo, ErrorContext>;
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
                1u16 => PathInfo::Established({
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
            "PathInfo",
            r#type.and_then(|t| PathInfo::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl std::fmt::Debug for IterablePathInfo<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("PathInfo");
        let mut iter = IterablePathInfo::with_loc(&[], self.orig_loc);
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
                PathInfo::Established(val) => fmt.field("Established", &val),
            };
        }
        fmt.finish()
    }
}
impl IterablePathInfo<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("PathInfo", offset));
            return (
                stack,
                missing_type.and_then(|t| PathInfo::attr_from_type(t)),
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
                PathInfo::Established(val) => {
                    if last_off == offset {
                        stack.push(("Established", last_off));
                        break;
                    }
                }
                _ => {}
            };
            last_off = cur + attrs.pos;
        }
        if !stack.is_empty() {
            stack.push(("PathInfo", cur));
        }
        (stack, None)
    }
}
#[derive(Clone)]
pub enum RenameParms<'a> {
    #[doc = "New name for the resource.\n"]
    NewName(&'a CStr),
}
impl<'a> IterableRenameParms<'a> {
    #[doc = "New name for the resource.\n"]
    pub fn get_new_name(&self) -> Result<&'a CStr, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(RenameParms::NewName(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "RenameParms",
            "NewName",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl RenameParms<'_> {
    pub fn new<'a>(buf: &'a [u8]) -> IterableRenameParms<'a> {
        IterableRenameParms::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        let res = match r#type {
            1u16 => "NewName",
            _ => return None,
        };
        Some(res)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableRenameParms<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableRenameParms<'a> {
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
impl<'a> Iterator for IterableRenameParms<'a> {
    type Item = Result<RenameParms<'a>, ErrorContext>;
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
                1u16 => RenameParms::NewName({
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
            "RenameParms",
            r#type.and_then(|t| RenameParms::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl<'a> std::fmt::Debug for IterableRenameParms<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("RenameParms");
        let mut iter = IterableRenameParms::with_loc(&[], self.orig_loc);
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
                RenameParms::NewName(val) => fmt.field("NewName", &val),
            };
        }
        fmt.finish()
    }
}
impl IterableRenameParms<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("RenameParms", offset));
            return (
                stack,
                missing_type.and_then(|t| RenameParms::attr_from_type(t)),
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
                RenameParms::NewName(val) => {
                    if last_off == offset {
                        stack.push(("NewName", last_off));
                        break;
                    }
                }
                _ => {}
            };
            last_off = cur + attrs.pos;
        }
        if !stack.is_empty() {
            stack.push(("RenameParms", cur));
        }
        (stack, None)
    }
}
#[derive(Clone)]
pub enum InvalidatePeerParms {
    #[doc = "Whether to mark the whole bitmap out of sync before starting the resync,\ninstead of resyncing only the blocks already marked out of sync\n(drbdsetup invalidate-remote option reset-bitmap).\n"]
    ResetBitmap(u8),
}
impl<'a> IterableInvalidatePeerParms<'a> {
    #[doc = "Whether to mark the whole bitmap out of sync before starting the resync,\ninstead of resyncing only the blocks already marked out of sync\n(drbdsetup invalidate-remote option reset-bitmap).\n"]
    pub fn get_reset_bitmap(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(InvalidatePeerParms::ResetBitmap(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "InvalidatePeerParms",
            "ResetBitmap",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl InvalidatePeerParms {
    pub fn new<'a>(buf: &'a [u8]) -> IterableInvalidatePeerParms<'a> {
        IterableInvalidatePeerParms::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        let res = match r#type {
            1u16 => "ResetBitmap",
            _ => return None,
        };
        Some(res)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableInvalidatePeerParms<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableInvalidatePeerParms<'a> {
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
impl<'a> Iterator for IterableInvalidatePeerParms<'a> {
    type Item = Result<InvalidatePeerParms, ErrorContext>;
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
                1u16 => InvalidatePeerParms::ResetBitmap({
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
            "InvalidatePeerParms",
            r#type.and_then(|t| InvalidatePeerParms::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl std::fmt::Debug for IterableInvalidatePeerParms<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("InvalidatePeerParms");
        let mut iter = IterableInvalidatePeerParms::with_loc(&[], self.orig_loc);
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
                InvalidatePeerParms::ResetBitmap(val) => fmt.field("ResetBitmap", &val),
            };
        }
        fmt.finish()
    }
}
impl IterableInvalidatePeerParms<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("InvalidatePeerParms", offset));
            return (
                stack,
                missing_type.and_then(|t| InvalidatePeerParms::attr_from_type(t)),
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
                InvalidatePeerParms::ResetBitmap(val) => {
                    if last_off == offset {
                        stack.push(("ResetBitmap", last_off));
                        break;
                    }
                }
                _ => {}
            };
            last_off = cur + attrs.pos;
        }
        if !stack.is_empty() {
            stack.push(("InvalidatePeerParms", cur));
        }
        (stack, None)
    }
}
#[derive(Clone)]
pub enum SuspendIoParms {
    #[doc = "Whether to freeze the filesystem mounted on the device to bring it into\na consistent state before suspending I/O (drbdsetup suspend-io option\nbdev-freeze).\n"]
    BdevFreeze(u8),
    #[doc = "Whether to wait until the peers have acknowledged the barrier that\ncloses the last epoch, so that the writes before the suspension are on\ntheir stable storage also with protocols A and B (drbdsetup suspend-io\noption flush-to-peers).\n"]
    FlushToPeers(u8),
}
impl<'a> IterableSuspendIoParms<'a> {
    #[doc = "Whether to freeze the filesystem mounted on the device to bring it into\na consistent state before suspending I/O (drbdsetup suspend-io option\nbdev-freeze).\n"]
    pub fn get_bdev_freeze(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(SuspendIoParms::BdevFreeze(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "SuspendIoParms",
            "BdevFreeze",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Whether to wait until the peers have acknowledged the barrier that\ncloses the last epoch, so that the writes before the suspension are on\ntheir stable storage also with protocols A and B (drbdsetup suspend-io\noption flush-to-peers).\n"]
    pub fn get_flush_to_peers(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(SuspendIoParms::FlushToPeers(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "SuspendIoParms",
            "FlushToPeers",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl SuspendIoParms {
    pub fn new<'a>(buf: &'a [u8]) -> IterableSuspendIoParms<'a> {
        IterableSuspendIoParms::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        let res = match r#type {
            1u16 => "BdevFreeze",
            2u16 => "FlushToPeers",
            _ => return None,
        };
        Some(res)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableSuspendIoParms<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableSuspendIoParms<'a> {
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
impl<'a> Iterator for IterableSuspendIoParms<'a> {
    type Item = Result<SuspendIoParms, ErrorContext>;
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
                1u16 => SuspendIoParms::BdevFreeze({
                    let res = parse_u8(next);
                    let Some(val) = res else { break };
                    val
                }),
                2u16 => SuspendIoParms::FlushToPeers({
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
            "SuspendIoParms",
            r#type.and_then(|t| SuspendIoParms::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl std::fmt::Debug for IterableSuspendIoParms<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("SuspendIoParms");
        let mut iter = IterableSuspendIoParms::with_loc(&[], self.orig_loc);
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
                SuspendIoParms::BdevFreeze(val) => fmt.field("BdevFreeze", &val),
                SuspendIoParms::FlushToPeers(val) => fmt.field("FlushToPeers", &val),
            };
        }
        fmt.finish()
    }
}
impl IterableSuspendIoParms<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("SuspendIoParms", offset));
            return (
                stack,
                missing_type.and_then(|t| SuspendIoParms::attr_from_type(t)),
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
                SuspendIoParms::BdevFreeze(val) => {
                    if last_off == offset {
                        stack.push(("BdevFreeze", last_off));
                        break;
                    }
                }
                SuspendIoParms::FlushToPeers(val) => {
                    if last_off == offset {
                        stack.push(("FlushToPeers", last_off));
                        break;
                    }
                }
                _ => {}
            };
            last_off = cur + attrs.pos;
        }
        if !stack.is_empty() {
            stack.push(("SuspendIoParms", cur));
        }
        (stack, None)
    }
}
#[derive(Clone)]
pub enum CbtStartParms {
    #[doc = "Caller-chosen identifier of the backup snapshot the new tracking\ninterval starts at; required.\n"]
    BookmarkId(u64),
}
impl<'a> IterableCbtStartParms<'a> {
    #[doc = "Caller-chosen identifier of the backup snapshot the new tracking\ninterval starts at; required.\n"]
    pub fn get_bookmark_id(&self) -> Result<u64, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(CbtStartParms::BookmarkId(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "CbtStartParms",
            "BookmarkId",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl CbtStartParms {
    pub fn new<'a>(buf: &'a [u8]) -> IterableCbtStartParms<'a> {
        IterableCbtStartParms::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        let res = match r#type {
            1u16 => "BookmarkId",
            _ => return None,
        };
        Some(res)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableCbtStartParms<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableCbtStartParms<'a> {
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
impl<'a> Iterator for IterableCbtStartParms<'a> {
    type Item = Result<CbtStartParms, ErrorContext>;
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
                1u16 => CbtStartParms::BookmarkId({
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
            "CbtStartParms",
            r#type.and_then(|t| CbtStartParms::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl std::fmt::Debug for IterableCbtStartParms<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("CbtStartParms");
        let mut iter = IterableCbtStartParms::with_loc(&[], self.orig_loc);
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
                CbtStartParms::BookmarkId(val) => fmt.field("BookmarkId", &val),
            };
        }
        fmt.finish()
    }
}
impl IterableCbtStartParms<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("CbtStartParms", offset));
            return (
                stack,
                missing_type.and_then(|t| CbtStartParms::attr_from_type(t)),
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
                CbtStartParms::BookmarkId(val) => {
                    if last_off == offset {
                        stack.push(("BookmarkId", last_off));
                        break;
                    }
                }
                _ => {}
            };
            last_off = cur + attrs.pos;
        }
        if !stack.is_empty() {
            stack.push(("CbtStartParms", cur));
        }
        (stack, None)
    }
}
#[derive(Clone)]
pub enum CbtInfo {
    #[doc = "Whether a bitmap slot tracks the blocks changed since a bookmark.\n"]
    Active(u8),
    #[doc = "Whether the tracking bitmap holds every change since the bookmark;\ncleared when changes may have been missed, which requires a full backup.\n"]
    Valid(u8),
    #[doc = "Identifier given to the device-cbt-start that started the interval.\n"]
    BookmarkId(u64),
    #[doc = "Time of that device-cbt-start, in seconds since the epoch.\n"]
    Since(u64),
    #[doc = "Current UUID of the device at that device-cbt-start.\n"]
    BitmapUuid(u64),
    #[doc = "Data generation tag of the Primary at that device-cbt-start; on a\nSecondary the last one received from the Primary.\n"]
    BitmapDagtag(u64),
}
impl<'a> IterableCbtInfo<'a> {
    #[doc = "Whether a bitmap slot tracks the blocks changed since a bookmark.\n"]
    pub fn get_active(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(CbtInfo::Active(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "CbtInfo",
            "Active",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Whether the tracking bitmap holds every change since the bookmark;\ncleared when changes may have been missed, which requires a full backup.\n"]
    pub fn get_valid(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(CbtInfo::Valid(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "CbtInfo",
            "Valid",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Identifier given to the device-cbt-start that started the interval.\n"]
    pub fn get_bookmark_id(&self) -> Result<u64, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(CbtInfo::BookmarkId(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "CbtInfo",
            "BookmarkId",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Time of that device-cbt-start, in seconds since the epoch.\n"]
    pub fn get_since(&self) -> Result<u64, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(CbtInfo::Since(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "CbtInfo",
            "Since",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Current UUID of the device at that device-cbt-start.\n"]
    pub fn get_bitmap_uuid(&self) -> Result<u64, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(CbtInfo::BitmapUuid(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "CbtInfo",
            "BitmapUuid",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Data generation tag of the Primary at that device-cbt-start; on a\nSecondary the last one received from the Primary.\n"]
    pub fn get_bitmap_dagtag(&self) -> Result<u64, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(CbtInfo::BitmapDagtag(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "CbtInfo",
            "BitmapDagtag",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl CbtInfo {
    pub fn new<'a>(buf: &'a [u8]) -> IterableCbtInfo<'a> {
        IterableCbtInfo::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        let res = match r#type {
            1u16 => "Active",
            2u16 => "Valid",
            3u16 => "BookmarkId",
            4u16 => "Since",
            5u16 => "BitmapUuid",
            6u16 => "BitmapDagtag",
            _ => return None,
        };
        Some(res)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableCbtInfo<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableCbtInfo<'a> {
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
impl<'a> Iterator for IterableCbtInfo<'a> {
    type Item = Result<CbtInfo, ErrorContext>;
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
                1u16 => CbtInfo::Active({
                    let res = parse_u8(next);
                    let Some(val) = res else { break };
                    val
                }),
                2u16 => CbtInfo::Valid({
                    let res = parse_u8(next);
                    let Some(val) = res else { break };
                    val
                }),
                3u16 => CbtInfo::BookmarkId({
                    let res = parse_u64(next);
                    let Some(val) = res else { break };
                    val
                }),
                4u16 => CbtInfo::Since({
                    let res = parse_u64(next);
                    let Some(val) = res else { break };
                    val
                }),
                5u16 => CbtInfo::BitmapUuid({
                    let res = parse_u64(next);
                    let Some(val) = res else { break };
                    val
                }),
                6u16 => CbtInfo::BitmapDagtag({
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
            "CbtInfo",
            r#type.and_then(|t| CbtInfo::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl std::fmt::Debug for IterableCbtInfo<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("CbtInfo");
        let mut iter = IterableCbtInfo::with_loc(&[], self.orig_loc);
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
                CbtInfo::Active(val) => fmt.field("Active", &val),
                CbtInfo::Valid(val) => fmt.field("Valid", &val),
                CbtInfo::BookmarkId(val) => fmt.field("BookmarkId", &val),
                CbtInfo::Since(val) => fmt.field("Since", &val),
                CbtInfo::BitmapUuid(val) => fmt.field("BitmapUuid", &val),
                CbtInfo::BitmapDagtag(val) => fmt.field("BitmapDagtag", &val),
            };
        }
        fmt.finish()
    }
}
impl IterableCbtInfo<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("CbtInfo", offset));
            return (stack, missing_type.and_then(|t| CbtInfo::attr_from_type(t)));
        }
        if cur > offset || cur + self.buf.len() < offset {
            return (stack, None);
        }
        let mut attrs = self.clone();
        let mut last_off = cur + attrs.pos;
        while let Some(attr) = attrs.next() {
            let Ok(attr) = attr else { break };
            match attr {
                CbtInfo::Active(val) => {
                    if last_off == offset {
                        stack.push(("Active", last_off));
                        break;
                    }
                }
                CbtInfo::Valid(val) => {
                    if last_off == offset {
                        stack.push(("Valid", last_off));
                        break;
                    }
                }
                CbtInfo::BookmarkId(val) => {
                    if last_off == offset {
                        stack.push(("BookmarkId", last_off));
                        break;
                    }
                }
                CbtInfo::Since(val) => {
                    if last_off == offset {
                        stack.push(("Since", last_off));
                        break;
                    }
                }
                CbtInfo::BitmapUuid(val) => {
                    if last_off == offset {
                        stack.push(("BitmapUuid", last_off));
                        break;
                    }
                }
                CbtInfo::BitmapDagtag(val) => {
                    if last_off == offset {
                        stack.push(("BitmapDagtag", last_off));
                        break;
                    }
                }
                _ => {}
            };
            last_off = cur + attrs.pos;
        }
        if !stack.is_empty() {
            stack.push(("CbtInfo", cur));
        }
        (stack, None)
    }
}
#[derive(Clone)]
pub enum Resource<'a> {
    #[doc = "Identity of the object.\n"]
    Context(IterableContext<'a>),
    #[doc = "State of the object.\n"]
    Info(IterableResourceInfo<'a>),
    #[doc = "Statistics of the object.\n"]
    Statistics(IterableResourceStatistics<'a>),
    #[doc = "New name of the resource; present in state-change messages with action\nrename.\n"]
    NewName(&'a CStr),
    #[doc = "Resource-wide options in effect.\n"]
    ResourceOpts(IterableResourceOpts<'a>),
}
impl<'a> IterableResource<'a> {
    #[doc = "Identity of the object.\n"]
    pub fn get_context(&self) -> Result<IterableContext<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Resource::Context(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Resource",
            "Context",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "State of the object.\n"]
    pub fn get_info(&self) -> Result<IterableResourceInfo<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Resource::Info(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Resource",
            "Info",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Statistics of the object.\n"]
    pub fn get_statistics(&self) -> Result<IterableResourceStatistics<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Resource::Statistics(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Resource",
            "Statistics",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "New name of the resource; present in state-change messages with action\nrename.\n"]
    pub fn get_new_name(&self) -> Result<&'a CStr, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Resource::NewName(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Resource",
            "NewName",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Resource-wide options in effect.\n"]
    pub fn get_resource_opts(&self) -> Result<IterableResourceOpts<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Resource::ResourceOpts(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Resource",
            "ResourceOpts",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl Resource<'_> {
    pub fn new<'a>(buf: &'a [u8]) -> IterableResource<'a> {
        IterableResource::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        let res = match r#type {
            1u16 => "Context",
            2u16 => "Info",
            3u16 => "Statistics",
            4u16 => "NewName",
            5u16 => "ResourceOpts",
            _ => return None,
        };
        Some(res)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableResource<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableResource<'a> {
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
impl<'a> Iterator for IterableResource<'a> {
    type Item = Result<Resource<'a>, ErrorContext>;
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
                1u16 => Resource::Context({
                    let res = Some(IterableContext::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                2u16 => Resource::Info({
                    let res = Some(IterableResourceInfo::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                3u16 => Resource::Statistics({
                    let res = Some(IterableResourceStatistics::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                4u16 => Resource::NewName({
                    let res = CStr::from_bytes_with_nul(next).ok();
                    let Some(val) = res else { break };
                    val
                }),
                5u16 => Resource::ResourceOpts({
                    let res = Some(IterableResourceOpts::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                n if cfg!(any(test, feature = "deny-unknown-attrs")) => break,
                n => continue,
            };
            return Some(Ok(res));
        }
        Some(Err(ErrorContext::new(
            "Resource",
            r#type.and_then(|t| Resource::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl<'a> std::fmt::Debug for IterableResource<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("Resource");
        let mut iter = IterableResource::with_loc(&[], self.orig_loc);
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
                Resource::Context(val) => fmt.field("Context", &val),
                Resource::Info(val) => fmt.field("Info", &val),
                Resource::Statistics(val) => fmt.field("Statistics", &val),
                Resource::NewName(val) => fmt.field("NewName", &val),
                Resource::ResourceOpts(val) => fmt.field("ResourceOpts", &val),
            };
        }
        fmt.finish()
    }
}
impl IterableResource<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("Resource", offset));
            return (
                stack,
                missing_type.and_then(|t| Resource::attr_from_type(t)),
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
                Resource::Context(val) => {
                    (stack, missing) = val.lookup_attr(offset, missing_type);
                    if !stack.is_empty() {
                        break;
                    }
                }
                Resource::Info(val) => {
                    (stack, missing) = val.lookup_attr(offset, missing_type);
                    if !stack.is_empty() {
                        break;
                    }
                }
                Resource::Statistics(val) => {
                    (stack, missing) = val.lookup_attr(offset, missing_type);
                    if !stack.is_empty() {
                        break;
                    }
                }
                Resource::NewName(val) => {
                    if last_off == offset {
                        stack.push(("NewName", last_off));
                        break;
                    }
                }
                Resource::ResourceOpts(val) => {
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
            stack.push(("Resource", cur));
        }
        (stack, missing)
    }
}
#[derive(Clone)]
pub enum Device<'a> {
    #[doc = "Identity of the object.\n"]
    Context(IterableContext<'a>),
    #[doc = "State of the object.\n"]
    Info(IterableDeviceInfo<'a>),
    #[doc = "Statistics of the object.\n"]
    Statistics(IterableDeviceStatistics<'a>),
    #[doc = "Backing disk options in effect; absent while the device is diskless.\n"]
    DiskConf(IterableDiskConf<'a>),
    #[doc = "Device options in effect.\n"]
    DeviceConf(IterableDeviceConf<'a>),
}
impl<'a> IterableDevice<'a> {
    #[doc = "Identity of the object.\n"]
    pub fn get_context(&self) -> Result<IterableContext<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Device::Context(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Device",
            "Context",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "State of the object.\n"]
    pub fn get_info(&self) -> Result<IterableDeviceInfo<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Device::Info(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Device",
            "Info",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Statistics of the object.\n"]
    pub fn get_statistics(&self) -> Result<IterableDeviceStatistics<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Device::Statistics(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Device",
            "Statistics",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Backing disk options in effect; absent while the device is diskless.\n"]
    pub fn get_disk_conf(&self) -> Result<IterableDiskConf<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Device::DiskConf(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Device",
            "DiskConf",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Device options in effect.\n"]
    pub fn get_device_conf(&self) -> Result<IterableDeviceConf<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Device::DeviceConf(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Device",
            "DeviceConf",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl Device<'_> {
    pub fn new<'a>(buf: &'a [u8]) -> IterableDevice<'a> {
        IterableDevice::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        let res = match r#type {
            1u16 => "Context",
            2u16 => "Info",
            3u16 => "Statistics",
            4u16 => "DiskConf",
            5u16 => "DeviceConf",
            _ => return None,
        };
        Some(res)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableDevice<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableDevice<'a> {
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
impl<'a> Iterator for IterableDevice<'a> {
    type Item = Result<Device<'a>, ErrorContext>;
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
                1u16 => Device::Context({
                    let res = Some(IterableContext::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                2u16 => Device::Info({
                    let res = Some(IterableDeviceInfo::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                3u16 => Device::Statistics({
                    let res = Some(IterableDeviceStatistics::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                4u16 => Device::DiskConf({
                    let res = Some(IterableDiskConf::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                5u16 => Device::DeviceConf({
                    let res = Some(IterableDeviceConf::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                n if cfg!(any(test, feature = "deny-unknown-attrs")) => break,
                n => continue,
            };
            return Some(Ok(res));
        }
        Some(Err(ErrorContext::new(
            "Device",
            r#type.and_then(|t| Device::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl<'a> std::fmt::Debug for IterableDevice<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("Device");
        let mut iter = IterableDevice::with_loc(&[], self.orig_loc);
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
                Device::Context(val) => fmt.field("Context", &val),
                Device::Info(val) => fmt.field("Info", &val),
                Device::Statistics(val) => fmt.field("Statistics", &val),
                Device::DiskConf(val) => fmt.field("DiskConf", &val),
                Device::DeviceConf(val) => fmt.field("DeviceConf", &val),
            };
        }
        fmt.finish()
    }
}
impl IterableDevice<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("Device", offset));
            return (stack, missing_type.and_then(|t| Device::attr_from_type(t)));
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
                Device::Context(val) => {
                    (stack, missing) = val.lookup_attr(offset, missing_type);
                    if !stack.is_empty() {
                        break;
                    }
                }
                Device::Info(val) => {
                    (stack, missing) = val.lookup_attr(offset, missing_type);
                    if !stack.is_empty() {
                        break;
                    }
                }
                Device::Statistics(val) => {
                    (stack, missing) = val.lookup_attr(offset, missing_type);
                    if !stack.is_empty() {
                        break;
                    }
                }
                Device::DiskConf(val) => {
                    (stack, missing) = val.lookup_attr(offset, missing_type);
                    if !stack.is_empty() {
                        break;
                    }
                }
                Device::DeviceConf(val) => {
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
            stack.push(("Device", cur));
        }
        (stack, missing)
    }
}
#[derive(Clone)]
pub enum Connection<'a> {
    #[doc = "Identity of the object.\n"]
    Context(IterableContext<'a>),
    #[doc = "State of the object.\n"]
    Info(IterableConnectionInfo<'a>),
    #[doc = "Statistics of the object.\n"]
    Statistics(IterableConnectionStatistics<'a>),
    #[doc = "Connection options in effect; absent when the connection has none.\n"]
    NetConf(IterableNetConf<'a>),
    #[doc = "The paths of this connection, one attribute each. Also available on\ntheir own through path-get.\n\nAttribute may repeat multiple times (treat it as array)"]
    Path(IterablePath<'a>),
}
impl<'a> IterableConnection<'a> {
    #[doc = "Identity of the object.\n"]
    pub fn get_context(&self) -> Result<IterableContext<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Connection::Context(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Connection",
            "Context",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "State of the object.\n"]
    pub fn get_info(&self) -> Result<IterableConnectionInfo<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Connection::Info(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Connection",
            "Info",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Statistics of the object.\n"]
    pub fn get_statistics(&self) -> Result<IterableConnectionStatistics<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Connection::Statistics(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Connection",
            "Statistics",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Connection options in effect; absent when the connection has none.\n"]
    pub fn get_net_conf(&self) -> Result<IterableNetConf<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Connection::NetConf(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Connection",
            "NetConf",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "The paths of this connection, one attribute each. Also available on\ntheir own through path-get.\n\nAttribute may repeat multiple times (treat it as array)"]
    pub fn get_path(&self) -> MultiAttrIterable<Self, Connection<'a>, IterablePath<'a>> {
        MultiAttrIterable::new(self.clone(), |variant| {
            if let Connection::Path(val) = variant {
                Some(val)
            } else {
                None
            }
        })
    }
}
impl Connection<'_> {
    pub fn new<'a>(buf: &'a [u8]) -> IterableConnection<'a> {
        IterableConnection::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        let res = match r#type {
            1u16 => "Context",
            2u16 => "Info",
            3u16 => "Statistics",
            4u16 => "NetConf",
            5u16 => "Path",
            _ => return None,
        };
        Some(res)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableConnection<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableConnection<'a> {
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
impl<'a> Iterator for IterableConnection<'a> {
    type Item = Result<Connection<'a>, ErrorContext>;
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
                1u16 => Connection::Context({
                    let res = Some(IterableContext::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                2u16 => Connection::Info({
                    let res = Some(IterableConnectionInfo::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                3u16 => Connection::Statistics({
                    let res = Some(IterableConnectionStatistics::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                4u16 => Connection::NetConf({
                    let res = Some(IterableNetConf::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                5u16 => Connection::Path({
                    let res = Some(IterablePath::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                n if cfg!(any(test, feature = "deny-unknown-attrs")) => break,
                n => continue,
            };
            return Some(Ok(res));
        }
        Some(Err(ErrorContext::new(
            "Connection",
            r#type.and_then(|t| Connection::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl<'a> std::fmt::Debug for IterableConnection<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("Connection");
        let mut iter = IterableConnection::with_loc(&[], self.orig_loc);
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
                Connection::Context(val) => fmt.field("Context", &val),
                Connection::Info(val) => fmt.field("Info", &val),
                Connection::Statistics(val) => fmt.field("Statistics", &val),
                Connection::NetConf(val) => fmt.field("NetConf", &val),
                Connection::Path(val) => fmt.field("Path", &val),
            };
        }
        fmt.finish()
    }
}
impl IterableConnection<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("Connection", offset));
            return (
                stack,
                missing_type.and_then(|t| Connection::attr_from_type(t)),
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
                Connection::Context(val) => {
                    (stack, missing) = val.lookup_attr(offset, missing_type);
                    if !stack.is_empty() {
                        break;
                    }
                }
                Connection::Info(val) => {
                    (stack, missing) = val.lookup_attr(offset, missing_type);
                    if !stack.is_empty() {
                        break;
                    }
                }
                Connection::Statistics(val) => {
                    (stack, missing) = val.lookup_attr(offset, missing_type);
                    if !stack.is_empty() {
                        break;
                    }
                }
                Connection::NetConf(val) => {
                    (stack, missing) = val.lookup_attr(offset, missing_type);
                    if !stack.is_empty() {
                        break;
                    }
                }
                Connection::Path(val) => {
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
            stack.push(("Connection", cur));
        }
        (stack, missing)
    }
}
#[derive(Clone)]
pub enum PeerDevice<'a> {
    #[doc = "Identity of the object.\n"]
    Context(IterableContext<'a>),
    #[doc = "State of the object.\n"]
    Info(IterablePeerDeviceInfo<'a>),
    #[doc = "Statistics of the object.\n"]
    Statistics(IterablePeerDeviceStatistics<'a>),
    #[doc = "Peer device options in effect; absent when the peer device has none.\n"]
    PeerDeviceConf(IterablePeerDeviceConf<'a>),
}
impl<'a> IterablePeerDevice<'a> {
    #[doc = "Identity of the object.\n"]
    pub fn get_context(&self) -> Result<IterableContext<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(PeerDevice::Context(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "PeerDevice",
            "Context",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "State of the object.\n"]
    pub fn get_info(&self) -> Result<IterablePeerDeviceInfo<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(PeerDevice::Info(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "PeerDevice",
            "Info",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Statistics of the object.\n"]
    pub fn get_statistics(&self) -> Result<IterablePeerDeviceStatistics<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(PeerDevice::Statistics(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "PeerDevice",
            "Statistics",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Peer device options in effect; absent when the peer device has none.\n"]
    pub fn get_peer_device_conf(&self) -> Result<IterablePeerDeviceConf<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(PeerDevice::PeerDeviceConf(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "PeerDevice",
            "PeerDeviceConf",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl PeerDevice<'_> {
    pub fn new<'a>(buf: &'a [u8]) -> IterablePeerDevice<'a> {
        IterablePeerDevice::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        let res = match r#type {
            1u16 => "Context",
            2u16 => "Info",
            3u16 => "Statistics",
            4u16 => "PeerDeviceConf",
            _ => return None,
        };
        Some(res)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterablePeerDevice<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterablePeerDevice<'a> {
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
impl<'a> Iterator for IterablePeerDevice<'a> {
    type Item = Result<PeerDevice<'a>, ErrorContext>;
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
                1u16 => PeerDevice::Context({
                    let res = Some(IterableContext::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                2u16 => PeerDevice::Info({
                    let res = Some(IterablePeerDeviceInfo::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                3u16 => PeerDevice::Statistics({
                    let res = Some(IterablePeerDeviceStatistics::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                4u16 => PeerDevice::PeerDeviceConf({
                    let res = Some(IterablePeerDeviceConf::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                n if cfg!(any(test, feature = "deny-unknown-attrs")) => break,
                n => continue,
            };
            return Some(Ok(res));
        }
        Some(Err(ErrorContext::new(
            "PeerDevice",
            r#type.and_then(|t| PeerDevice::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl<'a> std::fmt::Debug for IterablePeerDevice<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("PeerDevice");
        let mut iter = IterablePeerDevice::with_loc(&[], self.orig_loc);
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
                PeerDevice::Context(val) => fmt.field("Context", &val),
                PeerDevice::Info(val) => fmt.field("Info", &val),
                PeerDevice::Statistics(val) => fmt.field("Statistics", &val),
                PeerDevice::PeerDeviceConf(val) => fmt.field("PeerDeviceConf", &val),
            };
        }
        fmt.finish()
    }
}
impl IterablePeerDevice<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("PeerDevice", offset));
            return (
                stack,
                missing_type.and_then(|t| PeerDevice::attr_from_type(t)),
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
                PeerDevice::Context(val) => {
                    (stack, missing) = val.lookup_attr(offset, missing_type);
                    if !stack.is_empty() {
                        break;
                    }
                }
                PeerDevice::Info(val) => {
                    (stack, missing) = val.lookup_attr(offset, missing_type);
                    if !stack.is_empty() {
                        break;
                    }
                }
                PeerDevice::Statistics(val) => {
                    (stack, missing) = val.lookup_attr(offset, missing_type);
                    if !stack.is_empty() {
                        break;
                    }
                }
                PeerDevice::PeerDeviceConf(val) => {
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
            stack.push(("PeerDevice", cur));
        }
        (stack, missing)
    }
}
#[derive(Clone)]
pub enum Path<'a> {
    #[doc = "Identity of the object.\n"]
    Context(IterableContext<'a>),
    #[doc = "State of the object.\n"]
    Info(IterablePathInfo<'a>),
}
impl<'a> IterablePath<'a> {
    #[doc = "Identity of the object.\n"]
    pub fn get_context(&self) -> Result<IterableContext<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Path::Context(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Path",
            "Context",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "State of the object.\n"]
    pub fn get_info(&self) -> Result<IterablePathInfo<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Path::Info(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Path",
            "Info",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl Path<'_> {
    pub fn new<'a>(buf: &'a [u8]) -> IterablePath<'a> {
        IterablePath::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        let res = match r#type {
            1u16 => "Context",
            2u16 => "Info",
            _ => return None,
        };
        Some(res)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterablePath<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterablePath<'a> {
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
impl<'a> Iterator for IterablePath<'a> {
    type Item = Result<Path<'a>, ErrorContext>;
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
                1u16 => Path::Context({
                    let res = Some(IterableContext::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                2u16 => Path::Info({
                    let res = Some(IterablePathInfo::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                n if cfg!(any(test, feature = "deny-unknown-attrs")) => break,
                n => continue,
            };
            return Some(Ok(res));
        }
        Some(Err(ErrorContext::new(
            "Path",
            r#type.and_then(|t| Path::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl<'a> std::fmt::Debug for IterablePath<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("Path");
        let mut iter = IterablePath::with_loc(&[], self.orig_loc);
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
                Path::Context(val) => fmt.field("Context", &val),
                Path::Info(val) => fmt.field("Info", &val),
            };
        }
        fmt.finish()
    }
}
impl IterablePath<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("Path", offset));
            return (stack, missing_type.and_then(|t| Path::attr_from_type(t)));
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
                Path::Context(val) => {
                    (stack, missing) = val.lookup_attr(offset, missing_type);
                    if !stack.is_empty() {
                        break;
                    }
                }
                Path::Info(val) => {
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
            stack.push(("Path", cur));
        }
        (stack, missing)
    }
}
#[derive(Clone)]
pub enum Drbd2<'a> {
    #[doc = "Object the message refers to.\n"]
    Context(IterableContext<'a>),
    #[doc = "Reset all options not given in this request to their defaults before\napplying it.\n"]
    SetDefaults(()),
    #[doc = "Outcome of the state change performed by the request.\n\nAssociated type: [`StateResult`] (enum)"]
    StateResult(u32),
    #[doc = "Human-readable explanation accompanying state-result.\n"]
    Message(&'a CStr),
    #[doc = "Reply of timeout-type-get.\n\nAssociated type: [`TimeoutType`] (enum)"]
    TimeoutType(u32),
    #[doc = "Backing disk options for device-attach and disk-set.\n"]
    DiskConf(IterableDiskConf<'a>),
    #[doc = "Resource-wide options for resource-new and resource-set.\n"]
    ResourceOpts(IterableResourceOpts<'a>),
    #[doc = "Connection options for connection-new and connection-set.\n"]
    NetConf(IterableNetConf<'a>),
    #[doc = "Parameters of resource-primary and resource-secondary.\n"]
    SetRoleParms(IterableSetRoleParms<'a>),
    #[doc = "Parameters of device-resize.\n"]
    ResizeParms(IterableResizeParms<'a>),
    #[doc = "Parameters of peer-device-start-ov.\n"]
    StartOvParms(IterableStartOvParms<'a>),
    #[doc = "Parameters of device-new-current-uuid.\n"]
    NewCurrentUuidParms(IterableNewCurrentUuidParms<'a>),
    #[doc = "Parameters of connection-disconnect and connection-del.\n"]
    DisconnectParms(IterableDisconnectParms<'a>),
    #[doc = "Parameters of device-detach.\n"]
    DetachParms(IterableDetachParms<'a>),
    #[doc = "Device options set when the device is created by device-new.\n"]
    DeviceConf(IterableDeviceConf<'a>),
    #[doc = "Identity and outcome of the handler script reported by helper-ntf.\n"]
    Helper(IterableHelperInfo<'a>),
    #[doc = "Parameters of device-invalidate.\n"]
    InvalidateParms(IterableInvalidateParms<'a>),
    #[doc = "Peer device options for peer-device-set.\n"]
    PeerDeviceConf(IterablePeerDeviceConf<'a>),
    #[doc = "Parameters of connection-connect.\n"]
    ConnectParms(IterableConnectParms<'a>),
    #[doc = "Parameters of resource-rename.\n"]
    RenameParms(IterableRenameParms<'a>),
    #[doc = "Parameters of peer-device-invalidate.\n"]
    InvalidatePeerParms(IterableInvalidatePeerParms<'a>),
    #[doc = "Parameters of resource-suspend-io.\n"]
    SuspendIoParms(IterableSuspendIoParms<'a>),
    #[doc = "A resource (dump reply).\n"]
    Resource(IterableResource<'a>),
    #[doc = "A device (dump reply).\n"]
    Device(IterableDevice<'a>),
    #[doc = "A connection (dump reply).\n"]
    Connection(IterableConnection<'a>),
    #[doc = "A peer device (dump reply).\n"]
    PeerDevice(IterablePeerDevice<'a>),
    #[doc = "A path (dump reply).\n"]
    Path(IterablePath<'a>),
    #[doc = "Parameters of device-cbt-start.\n"]
    CbtStartParms(IterableCbtStartParms<'a>),
    #[doc = "Reply of device-cbt-status.\n"]
    CbtInfo(IterableCbtInfo<'a>),
    #[doc = "Parameters of device-attach.\n"]
    AttachParms(IterableAttachParms<'a>),
}
impl<'a> IterableDrbd2<'a> {
    #[doc = "Object the message refers to.\n"]
    pub fn get_context(&self) -> Result<IterableContext<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Drbd2::Context(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Drbd2",
            "Context",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Reset all options not given in this request to their defaults before\napplying it.\n"]
    pub fn get_set_defaults(&self) -> Result<(), ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Drbd2::SetDefaults(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Drbd2",
            "SetDefaults",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Outcome of the state change performed by the request.\n\nAssociated type: [`StateResult`] (enum)"]
    pub fn get_state_result(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Drbd2::StateResult(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Drbd2",
            "StateResult",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Human-readable explanation accompanying state-result.\n"]
    pub fn get_message(&self) -> Result<&'a CStr, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Drbd2::Message(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Drbd2",
            "Message",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Reply of timeout-type-get.\n\nAssociated type: [`TimeoutType`] (enum)"]
    pub fn get_timeout_type(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Drbd2::TimeoutType(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Drbd2",
            "TimeoutType",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Backing disk options for device-attach and disk-set.\n"]
    pub fn get_disk_conf(&self) -> Result<IterableDiskConf<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Drbd2::DiskConf(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Drbd2",
            "DiskConf",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Resource-wide options for resource-new and resource-set.\n"]
    pub fn get_resource_opts(&self) -> Result<IterableResourceOpts<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Drbd2::ResourceOpts(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Drbd2",
            "ResourceOpts",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Connection options for connection-new and connection-set.\n"]
    pub fn get_net_conf(&self) -> Result<IterableNetConf<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Drbd2::NetConf(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Drbd2",
            "NetConf",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Parameters of resource-primary and resource-secondary.\n"]
    pub fn get_set_role_parms(&self) -> Result<IterableSetRoleParms<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Drbd2::SetRoleParms(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Drbd2",
            "SetRoleParms",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Parameters of device-resize.\n"]
    pub fn get_resize_parms(&self) -> Result<IterableResizeParms<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Drbd2::ResizeParms(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Drbd2",
            "ResizeParms",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Parameters of peer-device-start-ov.\n"]
    pub fn get_start_ov_parms(&self) -> Result<IterableStartOvParms<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Drbd2::StartOvParms(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Drbd2",
            "StartOvParms",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Parameters of device-new-current-uuid.\n"]
    pub fn get_new_current_uuid_parms(
        &self,
    ) -> Result<IterableNewCurrentUuidParms<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Drbd2::NewCurrentUuidParms(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Drbd2",
            "NewCurrentUuidParms",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Parameters of connection-disconnect and connection-del.\n"]
    pub fn get_disconnect_parms(&self) -> Result<IterableDisconnectParms<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Drbd2::DisconnectParms(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Drbd2",
            "DisconnectParms",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Parameters of device-detach.\n"]
    pub fn get_detach_parms(&self) -> Result<IterableDetachParms<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Drbd2::DetachParms(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Drbd2",
            "DetachParms",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Device options set when the device is created by device-new.\n"]
    pub fn get_device_conf(&self) -> Result<IterableDeviceConf<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Drbd2::DeviceConf(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Drbd2",
            "DeviceConf",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Identity and outcome of the handler script reported by helper-ntf.\n"]
    pub fn get_helper(&self) -> Result<IterableHelperInfo<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Drbd2::Helper(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Drbd2",
            "Helper",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Parameters of device-invalidate.\n"]
    pub fn get_invalidate_parms(&self) -> Result<IterableInvalidateParms<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Drbd2::InvalidateParms(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Drbd2",
            "InvalidateParms",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Peer device options for peer-device-set.\n"]
    pub fn get_peer_device_conf(&self) -> Result<IterablePeerDeviceConf<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Drbd2::PeerDeviceConf(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Drbd2",
            "PeerDeviceConf",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Parameters of connection-connect.\n"]
    pub fn get_connect_parms(&self) -> Result<IterableConnectParms<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Drbd2::ConnectParms(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Drbd2",
            "ConnectParms",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Parameters of resource-rename.\n"]
    pub fn get_rename_parms(&self) -> Result<IterableRenameParms<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Drbd2::RenameParms(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Drbd2",
            "RenameParms",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Parameters of peer-device-invalidate.\n"]
    pub fn get_invalidate_peer_parms(
        &self,
    ) -> Result<IterableInvalidatePeerParms<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Drbd2::InvalidatePeerParms(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Drbd2",
            "InvalidatePeerParms",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Parameters of resource-suspend-io.\n"]
    pub fn get_suspend_io_parms(&self) -> Result<IterableSuspendIoParms<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Drbd2::SuspendIoParms(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Drbd2",
            "SuspendIoParms",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "A resource (dump reply).\n"]
    pub fn get_resource(&self) -> Result<IterableResource<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Drbd2::Resource(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Drbd2",
            "Resource",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "A device (dump reply).\n"]
    pub fn get_device(&self) -> Result<IterableDevice<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Drbd2::Device(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Drbd2",
            "Device",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "A connection (dump reply).\n"]
    pub fn get_connection(&self) -> Result<IterableConnection<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Drbd2::Connection(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Drbd2",
            "Connection",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "A peer device (dump reply).\n"]
    pub fn get_peer_device(&self) -> Result<IterablePeerDevice<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Drbd2::PeerDevice(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Drbd2",
            "PeerDevice",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "A path (dump reply).\n"]
    pub fn get_path(&self) -> Result<IterablePath<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Drbd2::Path(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Drbd2",
            "Path",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Parameters of device-cbt-start.\n"]
    pub fn get_cbt_start_parms(&self) -> Result<IterableCbtStartParms<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Drbd2::CbtStartParms(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Drbd2",
            "CbtStartParms",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Reply of device-cbt-status.\n"]
    pub fn get_cbt_info(&self) -> Result<IterableCbtInfo<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Drbd2::CbtInfo(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Drbd2",
            "CbtInfo",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Parameters of device-attach.\n"]
    pub fn get_attach_parms(&self) -> Result<IterableAttachParms<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Drbd2::AttachParms(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Drbd2",
            "AttachParms",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl Drbd2<'_> {
    pub fn new<'a>(buf: &'a [u8]) -> IterableDrbd2<'a> {
        IterableDrbd2::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        let res = match r#type {
            1u16 => "Context",
            2u16 => "SetDefaults",
            3u16 => "StateResult",
            4u16 => "Message",
            5u16 => "TimeoutType",
            6u16 => "DiskConf",
            7u16 => "ResourceOpts",
            8u16 => "NetConf",
            9u16 => "SetRoleParms",
            10u16 => "ResizeParms",
            11u16 => "StartOvParms",
            12u16 => "NewCurrentUuidParms",
            13u16 => "DisconnectParms",
            14u16 => "DetachParms",
            15u16 => "DeviceConf",
            16u16 => "Helper",
            17u16 => "InvalidateParms",
            18u16 => "PeerDeviceConf",
            19u16 => "ConnectParms",
            20u16 => "RenameParms",
            21u16 => "InvalidatePeerParms",
            22u16 => "SuspendIoParms",
            23u16 => "Resource",
            24u16 => "Device",
            25u16 => "Connection",
            26u16 => "PeerDevice",
            27u16 => "Path",
            28u16 => "CbtStartParms",
            29u16 => "CbtInfo",
            30u16 => "AttachParms",
            _ => return None,
        };
        Some(res)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableDrbd2<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableDrbd2<'a> {
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
impl<'a> Iterator for IterableDrbd2<'a> {
    type Item = Result<Drbd2<'a>, ErrorContext>;
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
                1u16 => Drbd2::Context({
                    let res = Some(IterableContext::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                2u16 => Drbd2::SetDefaults(()),
                3u16 => Drbd2::StateResult({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                4u16 => Drbd2::Message({
                    let res = CStr::from_bytes_with_nul(next).ok();
                    let Some(val) = res else { break };
                    val
                }),
                5u16 => Drbd2::TimeoutType({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                6u16 => Drbd2::DiskConf({
                    let res = Some(IterableDiskConf::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                7u16 => Drbd2::ResourceOpts({
                    let res = Some(IterableResourceOpts::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                8u16 => Drbd2::NetConf({
                    let res = Some(IterableNetConf::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                9u16 => Drbd2::SetRoleParms({
                    let res = Some(IterableSetRoleParms::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                10u16 => Drbd2::ResizeParms({
                    let res = Some(IterableResizeParms::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                11u16 => Drbd2::StartOvParms({
                    let res = Some(IterableStartOvParms::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                12u16 => Drbd2::NewCurrentUuidParms({
                    let res = Some(IterableNewCurrentUuidParms::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                13u16 => Drbd2::DisconnectParms({
                    let res = Some(IterableDisconnectParms::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                14u16 => Drbd2::DetachParms({
                    let res = Some(IterableDetachParms::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                15u16 => Drbd2::DeviceConf({
                    let res = Some(IterableDeviceConf::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                16u16 => Drbd2::Helper({
                    let res = Some(IterableHelperInfo::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                17u16 => Drbd2::InvalidateParms({
                    let res = Some(IterableInvalidateParms::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                18u16 => Drbd2::PeerDeviceConf({
                    let res = Some(IterablePeerDeviceConf::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                19u16 => Drbd2::ConnectParms({
                    let res = Some(IterableConnectParms::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                20u16 => Drbd2::RenameParms({
                    let res = Some(IterableRenameParms::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                21u16 => Drbd2::InvalidatePeerParms({
                    let res = Some(IterableInvalidatePeerParms::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                22u16 => Drbd2::SuspendIoParms({
                    let res = Some(IterableSuspendIoParms::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                23u16 => Drbd2::Resource({
                    let res = Some(IterableResource::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                24u16 => Drbd2::Device({
                    let res = Some(IterableDevice::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                25u16 => Drbd2::Connection({
                    let res = Some(IterableConnection::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                26u16 => Drbd2::PeerDevice({
                    let res = Some(IterablePeerDevice::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                27u16 => Drbd2::Path({
                    let res = Some(IterablePath::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                28u16 => Drbd2::CbtStartParms({
                    let res = Some(IterableCbtStartParms::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                29u16 => Drbd2::CbtInfo({
                    let res = Some(IterableCbtInfo::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                30u16 => Drbd2::AttachParms({
                    let res = Some(IterableAttachParms::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                n if cfg!(any(test, feature = "deny-unknown-attrs")) => break,
                n => continue,
            };
            return Some(Ok(res));
        }
        Some(Err(ErrorContext::new(
            "Drbd2",
            r#type.and_then(|t| Drbd2::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl<'a> std::fmt::Debug for IterableDrbd2<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("Drbd2");
        let mut iter = IterableDrbd2::with_loc(&[], self.orig_loc);
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
                Drbd2::Context(val) => fmt.field("Context", &val),
                Drbd2::SetDefaults(val) => fmt.field("SetDefaults", &val),
                Drbd2::StateResult(val) => fmt.field(
                    "StateResult",
                    &FormatEnum(val.into(), StateResult::from_value),
                ),
                Drbd2::Message(val) => fmt.field("Message", &val),
                Drbd2::TimeoutType(val) => fmt.field(
                    "TimeoutType",
                    &FormatEnum(val.into(), TimeoutType::from_value),
                ),
                Drbd2::DiskConf(val) => fmt.field("DiskConf", &val),
                Drbd2::ResourceOpts(val) => fmt.field("ResourceOpts", &val),
                Drbd2::NetConf(val) => fmt.field("NetConf", &val),
                Drbd2::SetRoleParms(val) => fmt.field("SetRoleParms", &val),
                Drbd2::ResizeParms(val) => fmt.field("ResizeParms", &val),
                Drbd2::StartOvParms(val) => fmt.field("StartOvParms", &val),
                Drbd2::NewCurrentUuidParms(val) => fmt.field("NewCurrentUuidParms", &val),
                Drbd2::DisconnectParms(val) => fmt.field("DisconnectParms", &val),
                Drbd2::DetachParms(val) => fmt.field("DetachParms", &val),
                Drbd2::DeviceConf(val) => fmt.field("DeviceConf", &val),
                Drbd2::Helper(val) => fmt.field("Helper", &val),
                Drbd2::InvalidateParms(val) => fmt.field("InvalidateParms", &val),
                Drbd2::PeerDeviceConf(val) => fmt.field("PeerDeviceConf", &val),
                Drbd2::ConnectParms(val) => fmt.field("ConnectParms", &val),
                Drbd2::RenameParms(val) => fmt.field("RenameParms", &val),
                Drbd2::InvalidatePeerParms(val) => fmt.field("InvalidatePeerParms", &val),
                Drbd2::SuspendIoParms(val) => fmt.field("SuspendIoParms", &val),
                Drbd2::Resource(val) => fmt.field("Resource", &val),
                Drbd2::Device(val) => fmt.field("Device", &val),
                Drbd2::Connection(val) => fmt.field("Connection", &val),
                Drbd2::PeerDevice(val) => fmt.field("PeerDevice", &val),
                Drbd2::Path(val) => fmt.field("Path", &val),
                Drbd2::CbtStartParms(val) => fmt.field("CbtStartParms", &val),
                Drbd2::CbtInfo(val) => fmt.field("CbtInfo", &val),
                Drbd2::AttachParms(val) => fmt.field("AttachParms", &val),
            };
        }
        fmt.finish()
    }
}
impl IterableDrbd2<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("Drbd2", offset));
            return (stack, missing_type.and_then(|t| Drbd2::attr_from_type(t)));
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
                Drbd2::Context(val) => {
                    (stack, missing) = val.lookup_attr(offset, missing_type);
                    if !stack.is_empty() {
                        break;
                    }
                }
                Drbd2::SetDefaults(val) => {
                    if last_off == offset {
                        stack.push(("SetDefaults", last_off));
                        break;
                    }
                }
                Drbd2::StateResult(val) => {
                    if last_off == offset {
                        stack.push(("StateResult", last_off));
                        break;
                    }
                }
                Drbd2::Message(val) => {
                    if last_off == offset {
                        stack.push(("Message", last_off));
                        break;
                    }
                }
                Drbd2::TimeoutType(val) => {
                    if last_off == offset {
                        stack.push(("TimeoutType", last_off));
                        break;
                    }
                }
                Drbd2::DiskConf(val) => {
                    (stack, missing) = val.lookup_attr(offset, missing_type);
                    if !stack.is_empty() {
                        break;
                    }
                }
                Drbd2::ResourceOpts(val) => {
                    (stack, missing) = val.lookup_attr(offset, missing_type);
                    if !stack.is_empty() {
                        break;
                    }
                }
                Drbd2::NetConf(val) => {
                    (stack, missing) = val.lookup_attr(offset, missing_type);
                    if !stack.is_empty() {
                        break;
                    }
                }
                Drbd2::SetRoleParms(val) => {
                    (stack, missing) = val.lookup_attr(offset, missing_type);
                    if !stack.is_empty() {
                        break;
                    }
                }
                Drbd2::ResizeParms(val) => {
                    (stack, missing) = val.lookup_attr(offset, missing_type);
                    if !stack.is_empty() {
                        break;
                    }
                }
                Drbd2::StartOvParms(val) => {
                    (stack, missing) = val.lookup_attr(offset, missing_type);
                    if !stack.is_empty() {
                        break;
                    }
                }
                Drbd2::NewCurrentUuidParms(val) => {
                    (stack, missing) = val.lookup_attr(offset, missing_type);
                    if !stack.is_empty() {
                        break;
                    }
                }
                Drbd2::DisconnectParms(val) => {
                    (stack, missing) = val.lookup_attr(offset, missing_type);
                    if !stack.is_empty() {
                        break;
                    }
                }
                Drbd2::DetachParms(val) => {
                    (stack, missing) = val.lookup_attr(offset, missing_type);
                    if !stack.is_empty() {
                        break;
                    }
                }
                Drbd2::DeviceConf(val) => {
                    (stack, missing) = val.lookup_attr(offset, missing_type);
                    if !stack.is_empty() {
                        break;
                    }
                }
                Drbd2::Helper(val) => {
                    (stack, missing) = val.lookup_attr(offset, missing_type);
                    if !stack.is_empty() {
                        break;
                    }
                }
                Drbd2::InvalidateParms(val) => {
                    (stack, missing) = val.lookup_attr(offset, missing_type);
                    if !stack.is_empty() {
                        break;
                    }
                }
                Drbd2::PeerDeviceConf(val) => {
                    (stack, missing) = val.lookup_attr(offset, missing_type);
                    if !stack.is_empty() {
                        break;
                    }
                }
                Drbd2::ConnectParms(val) => {
                    (stack, missing) = val.lookup_attr(offset, missing_type);
                    if !stack.is_empty() {
                        break;
                    }
                }
                Drbd2::RenameParms(val) => {
                    (stack, missing) = val.lookup_attr(offset, missing_type);
                    if !stack.is_empty() {
                        break;
                    }
                }
                Drbd2::InvalidatePeerParms(val) => {
                    (stack, missing) = val.lookup_attr(offset, missing_type);
                    if !stack.is_empty() {
                        break;
                    }
                }
                Drbd2::SuspendIoParms(val) => {
                    (stack, missing) = val.lookup_attr(offset, missing_type);
                    if !stack.is_empty() {
                        break;
                    }
                }
                Drbd2::Resource(val) => {
                    (stack, missing) = val.lookup_attr(offset, missing_type);
                    if !stack.is_empty() {
                        break;
                    }
                }
                Drbd2::Device(val) => {
                    (stack, missing) = val.lookup_attr(offset, missing_type);
                    if !stack.is_empty() {
                        break;
                    }
                }
                Drbd2::Connection(val) => {
                    (stack, missing) = val.lookup_attr(offset, missing_type);
                    if !stack.is_empty() {
                        break;
                    }
                }
                Drbd2::PeerDevice(val) => {
                    (stack, missing) = val.lookup_attr(offset, missing_type);
                    if !stack.is_empty() {
                        break;
                    }
                }
                Drbd2::Path(val) => {
                    (stack, missing) = val.lookup_attr(offset, missing_type);
                    if !stack.is_empty() {
                        break;
                    }
                }
                Drbd2::CbtStartParms(val) => {
                    (stack, missing) = val.lookup_attr(offset, missing_type);
                    if !stack.is_empty() {
                        break;
                    }
                }
                Drbd2::CbtInfo(val) => {
                    (stack, missing) = val.lookup_attr(offset, missing_type);
                    if !stack.is_empty() {
                        break;
                    }
                }
                Drbd2::AttachParms(val) => {
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
            stack.push(("Drbd2", cur));
        }
        (stack, missing)
    }
}
#[derive(Clone)]
pub enum StateChange<'a> {
    #[doc = "Whether the object exists, was created, changed or destroyed.\n\nAssociated type: [`StateChangeAction`] (enum)"]
    Action(u32),
    #[doc = "Further messages belong to the same state change.\n"]
    More(()),
    #[doc = "Present when the message describes a resource.\n"]
    Resource(IterableResource<'a>),
    #[doc = "Present when the message describes a device.\n"]
    Device(IterableDevice<'a>),
    #[doc = "Present when the message describes a connection.\n"]
    Connection(IterableConnection<'a>),
    #[doc = "Present when the message describes a peer device.\n"]
    PeerDevice(IterablePeerDevice<'a>),
    #[doc = "Present when the message describes a path.\n"]
    Path(IterablePath<'a>),
}
impl<'a> IterableStateChange<'a> {
    #[doc = "Whether the object exists, was created, changed or destroyed.\n\nAssociated type: [`StateChangeAction`] (enum)"]
    pub fn get_action(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(StateChange::Action(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "StateChange",
            "Action",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Further messages belong to the same state change.\n"]
    pub fn get_more(&self) -> Result<(), ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(StateChange::More(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "StateChange",
            "More",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Present when the message describes a resource.\n"]
    pub fn get_resource(&self) -> Result<IterableResource<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(StateChange::Resource(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "StateChange",
            "Resource",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Present when the message describes a device.\n"]
    pub fn get_device(&self) -> Result<IterableDevice<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(StateChange::Device(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "StateChange",
            "Device",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Present when the message describes a connection.\n"]
    pub fn get_connection(&self) -> Result<IterableConnection<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(StateChange::Connection(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "StateChange",
            "Connection",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Present when the message describes a peer device.\n"]
    pub fn get_peer_device(&self) -> Result<IterablePeerDevice<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(StateChange::PeerDevice(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "StateChange",
            "PeerDevice",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Present when the message describes a path.\n"]
    pub fn get_path(&self) -> Result<IterablePath<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(StateChange::Path(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "StateChange",
            "Path",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl StateChange<'_> {
    pub fn new<'a>(buf: &'a [u8]) -> IterableStateChange<'a> {
        IterableStateChange::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        let res = match r#type {
            1u16 => "Action",
            2u16 => "More",
            3u16 => "Resource",
            4u16 => "Device",
            5u16 => "Connection",
            6u16 => "PeerDevice",
            7u16 => "Path",
            _ => return None,
        };
        Some(res)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableStateChange<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableStateChange<'a> {
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
impl<'a> Iterator for IterableStateChange<'a> {
    type Item = Result<StateChange<'a>, ErrorContext>;
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
                1u16 => StateChange::Action({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                2u16 => StateChange::More(()),
                3u16 => StateChange::Resource({
                    let res = Some(IterableResource::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                4u16 => StateChange::Device({
                    let res = Some(IterableDevice::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                5u16 => StateChange::Connection({
                    let res = Some(IterableConnection::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                6u16 => StateChange::PeerDevice({
                    let res = Some(IterablePeerDevice::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                7u16 => StateChange::Path({
                    let res = Some(IterablePath::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                n if cfg!(any(test, feature = "deny-unknown-attrs")) => break,
                n => continue,
            };
            return Some(Ok(res));
        }
        Some(Err(ErrorContext::new(
            "StateChange",
            r#type.and_then(|t| StateChange::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl<'a> std::fmt::Debug for IterableStateChange<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("StateChange");
        let mut iter = IterableStateChange::with_loc(&[], self.orig_loc);
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
                StateChange::Action(val) => fmt.field(
                    "Action",
                    &FormatEnum(val.into(), StateChangeAction::from_value),
                ),
                StateChange::More(val) => fmt.field("More", &val),
                StateChange::Resource(val) => fmt.field("Resource", &val),
                StateChange::Device(val) => fmt.field("Device", &val),
                StateChange::Connection(val) => fmt.field("Connection", &val),
                StateChange::PeerDevice(val) => fmt.field("PeerDevice", &val),
                StateChange::Path(val) => fmt.field("Path", &val),
            };
        }
        fmt.finish()
    }
}
impl IterableStateChange<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("StateChange", offset));
            return (
                stack,
                missing_type.and_then(|t| StateChange::attr_from_type(t)),
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
                StateChange::Action(val) => {
                    if last_off == offset {
                        stack.push(("Action", last_off));
                        break;
                    }
                }
                StateChange::More(val) => {
                    if last_off == offset {
                        stack.push(("More", last_off));
                        break;
                    }
                }
                StateChange::Resource(val) => {
                    (stack, missing) = val.lookup_attr(offset, missing_type);
                    if !stack.is_empty() {
                        break;
                    }
                }
                StateChange::Device(val) => {
                    (stack, missing) = val.lookup_attr(offset, missing_type);
                    if !stack.is_empty() {
                        break;
                    }
                }
                StateChange::Connection(val) => {
                    (stack, missing) = val.lookup_attr(offset, missing_type);
                    if !stack.is_empty() {
                        break;
                    }
                }
                StateChange::PeerDevice(val) => {
                    (stack, missing) = val.lookup_attr(offset, missing_type);
                    if !stack.is_empty() {
                        break;
                    }
                }
                StateChange::Path(val) => {
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
            stack.push(("StateChange", cur));
        }
        (stack, missing)
    }
}
pub struct PushAddress<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushAddress<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushAddress<Prev> {
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
    #[doc = "Address family (AF_INET or AF_INET6).\n"]
    pub fn push_family(mut self, value: u16) -> Self {
        push_header(self.as_vec_mut(), 1u16, 2 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Port number.\n"]
    pub fn push_port(mut self, value: u16) -> Self {
        push_header(self.as_vec_mut(), 2u16, 2 as u16);
        self.as_vec_mut().extend(value.to_be_bytes());
        self
    }
    #[doc = "IPv4 address, present when family is AF_INET.\n"]
    pub fn push_ipv4(mut self, value: std::net::Ipv4Addr) -> Self {
        push_header(self.as_vec_mut(), 3u16, 4 as u16);
        self.as_vec_mut().extend(&value.to_bits().to_be_bytes());
        self
    }
    #[doc = "IPv6 address, present when family is AF_INET6.\n"]
    pub fn push_ipv6(mut self, value: std::net::Ipv6Addr) -> Self {
        push_header(self.as_vec_mut(), 4u16, 16 as u16);
        self.as_vec_mut().extend(&value.to_bits().to_ne_bytes());
        self
    }
}
impl<Prev: Pusher> Drop for PushAddress<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushContext<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushContext<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushContext<Prev> {
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
    #[doc = "Resource name.\n"]
    pub fn push_resource_name(mut self, value: &CStr) -> Self {
        push_header(
            self.as_vec_mut(),
            1u16,
            value.to_bytes_with_nul().len() as u16,
        );
        self.as_vec_mut().extend(value.to_bytes_with_nul());
        self
    }
    #[doc = "Resource name.\n"]
    pub fn push_resource_name_bytes(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 1u16, (value.len() + 1) as u16);
        self.as_vec_mut().extend(value);
        self.as_vec_mut().push(0);
        self
    }
    #[doc = "Volume number within the resource.\n"]
    pub fn push_volume(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 2u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Minor number of the DRBD device.\n"]
    pub fn push_minor(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 3u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Node ID of the peer.\n"]
    pub fn push_peer_node_id(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 4u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Name of the connection to the peer.\n"]
    pub fn push_connection_name(mut self, value: &CStr) -> Self {
        push_header(
            self.as_vec_mut(),
            5u16,
            value.to_bytes_with_nul().len() as u16,
        );
        self.as_vec_mut().extend(value.to_bytes_with_nul());
        self
    }
    #[doc = "Name of the connection to the peer.\n"]
    pub fn push_connection_name_bytes(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 5u16, (value.len() + 1) as u16);
        self.as_vec_mut().extend(value);
        self.as_vec_mut().push(0);
        self
    }
    #[doc = "Local address of a path.\n"]
    pub fn nested_my_address(mut self) -> PushAddress<Self> {
        let header_offset = push_nested_header(self.as_vec_mut(), 6u16);
        PushAddress {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
    #[doc = "Peer address of a path.\n"]
    pub fn nested_peer_address(mut self) -> PushAddress<Self> {
        let header_offset = push_nested_header(self.as_vec_mut(), 7u16);
        PushAddress {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
}
impl<Prev: Pusher> Drop for PushContext<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushDiskConf<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushDiskConf<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushDiskConf<Prev> {
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
    #[doc = "Path of the lower-level block device used to store the actual data\n(drbd.conf option disk).\n"]
    pub fn push_backing_dev(mut self, value: &CStr) -> Self {
        push_header(
            self.as_vec_mut(),
            1u16,
            value.to_bytes_with_nul().len() as u16,
        );
        self.as_vec_mut().extend(value.to_bytes_with_nul());
        self
    }
    #[doc = "Path of the lower-level block device used to store the actual data\n(drbd.conf option disk).\n"]
    pub fn push_backing_dev_bytes(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 1u16, (value.len() + 1) as u16);
        self.as_vec_mut().extend(value);
        self.as_vec_mut().push(0);
        self
    }
    #[doc = "Path of the device holding the metadata, or \\\"internal\\\" to store it on\nthe backing device (drbd.conf option meta-disk).\n"]
    pub fn push_meta_dev(mut self, value: &CStr) -> Self {
        push_header(
            self.as_vec_mut(),
            2u16,
            value.to_bytes_with_nul().len() as u16,
        );
        self.as_vec_mut().extend(value.to_bytes_with_nul());
        self
    }
    #[doc = "Path of the device holding the metadata, or \\\"internal\\\" to store it on\nthe backing device (drbd.conf option meta-disk).\n"]
    pub fn push_meta_dev_bytes(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 2u16, (value.len() + 1) as u16);
        self.as_vec_mut().extend(value);
        self.as_vec_mut().push(0);
        self
    }
    #[doc = "Index selecting which slot to use on a shared external metadata device\n(drbd.conf option meta-disk index).\n"]
    pub fn push_meta_dev_idx(mut self, value: i32) -> Self {
        push_header(self.as_vec_mut(), 3u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Size of the backing device to use, in sectors, overriding automatic\ndetection.\n"]
    pub fn push_size(mut self, value: u64) -> Self {
        push_header(self.as_vec_mut(), 4u16, 8 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Action to take when the backing disk reports an I/O error.\n\nAssociated type: [`IoErrorPolicy`] (enum)"]
    pub fn push_on_io_error(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 5u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Volume number of another volume that must finish resynchronizing before\nthis one starts (drbd.conf option resync-after).\n"]
    pub fn push_resync_after(mut self, value: i32) -> Self {
        push_header(self.as_vec_mut(), 6u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Number of 4 MiB activity log extents that can be active at the same\ntime.\n"]
    pub fn push_al_extents(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 7u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Whether to use disk barriers to enforce the ordering of dependent write\nrequests on the backing device.\n"]
    pub fn push_disk_barrier(mut self, value: u8) -> Self {
        push_header(self.as_vec_mut(), 8u16, 1 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Whether to issue disk flushes to the backing device to enforce the\nordering of dependent write requests.\n"]
    pub fn push_disk_flushes(mut self, value: u8) -> Self {
        push_header(self.as_vec_mut(), 9u16, 1 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Whether to wait for the local I/O request queue to drain before\ncompleting a write barrier.\n"]
    pub fn push_disk_drain(mut self, value: u8) -> Self {
        push_header(self.as_vec_mut(), 10u16, 1 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Whether to issue disk flushes to the metadata device.\n"]
    pub fn push_md_flushes(mut self, value: u8) -> Self {
        push_header(self.as_vec_mut(), 11u16, 1 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Time to wait for the backing device to complete an I/O request before\ntreating it as failed, in units of 0.1 seconds, or 0 to disable\n(drbd.conf option disk-timeout).\n"]
    pub fn push_disk_timeout(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 12u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Policy for balancing reads across the local disk and peers (drbd.conf\noption read-balancing).\n\nAssociated type: [`ReadBalancingPolicy`] (enum)"]
    pub fn push_read_balancing(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 13u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Legacy watermark, in pages, that used to trigger an early unplug of the\nbacking device\\'s request queue (drbd.conf option unplug-watermark).\n"]
    pub fn push_unplug_watermark(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 14u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Size, in bytes, of the resync requests DRBD tries to discard on the sync\ntarget when the source\\'s data is all zeroes (drbd.conf option\nrs-discard-granularity).\n"]
    pub fn push_rs_discard_granularity(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 15u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Whether to update the activity log on disk, at the cost of a full resync\nafter a primary crash if disabled (drbd.conf option al-updates).\n"]
    pub fn push_al_updates(mut self, value: u8) -> Self {
        push_header(self.as_vec_mut(), 16u16, 1 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Whether to treat discarded, aligned blocks on the backing device as\nreading back as zero (drbd.conf option discard-zeroes-if-aligned).\n"]
    pub fn push_discard_zeroes_if_aligned(mut self, value: u8) -> Self {
        push_header(self.as_vec_mut(), 17u16, 1 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Whether to avoid using the WRITE SAME command on the backing device, for\ndevices that advertise support but fail such requests (drbd.conf option\ndisable-write-same).\n"]
    pub fn push_disable_write_same(mut self, value: u8) -> Self {
        push_header(self.as_vec_mut(), 18u16, 1 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Whether to allocate the on-disk change-tracking bitmap when the device\nis attached (drbd.conf option bitmap).\n"]
    pub fn push_bitmap(mut self, value: u8) -> Self {
        push_header(self.as_vec_mut(), 19u16, 1 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
}
impl<Prev: Pusher> Drop for PushDiskConf<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushResourceOpts<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushResourceOpts<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushResourceOpts<Prev> {
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
    #[doc = "CPU affinity mask for DRBD kernel threads, as a hexadecimal number\n(drbd.conf option cpu-mask).\n"]
    pub fn push_cpu_mask(mut self, value: &CStr) -> Self {
        push_header(
            self.as_vec_mut(),
            1u16,
            value.to_bytes_with_nul().len() as u16,
        );
        self.as_vec_mut().extend(value.to_bytes_with_nul());
        self
    }
    #[doc = "CPU affinity mask for DRBD kernel threads, as a hexadecimal number\n(drbd.conf option cpu-mask).\n"]
    pub fn push_cpu_mask_bytes(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 1u16, (value.len() + 1) as u16);
        self.as_vec_mut().extend(value);
        self.as_vec_mut().push(0);
        self
    }
    #[doc = "Action to take for I/O requests when the requested data is not available\nlocally or remotely (drbd.conf option on-no-data-accessible).\n\nAssociated type: [`OnNoDataPolicy`] (enum)"]
    pub fn push_on_no_data_accessible(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 2u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Whether to automatically promote the resource to primary when one of its\ndevices is mounted or opened for writing (drbd.conf option\nauto-promote).\n"]
    pub fn push_auto_promote(mut self, value: u8) -> Self {
        push_header(self.as_vec_mut(), 3u16, 1 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Unique node identifier of this node in the cluster (drbd.conf option\nnode-id).\n"]
    pub fn push_node_id(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 4u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Amount of data, in sectors, DRBD replicates to a peer before issuing a\npeer-ack for the writes in between (drbd.conf option peer-ack-window).\n"]
    pub fn push_peer_ack_window(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 5u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Timeout for a two-phase-commit cluster-wide state transition, in units\nof 0.1 seconds (drbd.conf option twopc-timeout).\n"]
    pub fn push_twopc_timeout(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 6u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Base multiplier for the randomized backoff before retrying a conflicting\ntwo-phase commit, in units of 0.1 seconds (drbd.conf option\ntwopc-retry-timeout).\n"]
    pub fn push_twopc_retry_timeout(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 7u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "How long to wait after the last write request before sending a peer-ack\nif no new write request arrives, in milliseconds (drbd.conf option\npeer-ack-delay).\n"]
    pub fn push_peer_ack_delay(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 8u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "How long to wait for a device to become promotable when auto-promote\ntriggers a promotion, in units of 0.1 seconds (drbd.conf option\nauto-promote-timeout).\n"]
    pub fn push_auto_promote_timeout(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 9u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Maximum number of outstanding application I/O requests on a DRBD device.\n"]
    pub fn push_max_io_depth(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 10u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Number of nodes with an accessible disk required for a partition to have\nquorum, or a symbolic value (off, majority, all) (drbd.conf option\nquorum).\n"]
    pub fn push_quorum(mut self, value: i32) -> Self {
        push_header(self.as_vec_mut(), 11u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Action to take for I/O requests when the resource loses quorum\n(drbd.conf option on-no-quorum).\n\nAssociated type: [`OnNoQuorumPolicy`] (enum)"]
    pub fn push_on_no_quorum(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 12u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Minimum number of nodes with an UpToDate disk required for a partition\nto gain quorum, or a symbolic value (off, majority, all) (drbd.conf\noption quorum-minimum-redundancy).\n"]
    pub fn push_quorum_min_redundancy(mut self, value: i32) -> Self {
        push_header(self.as_vec_mut(), 13u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Action to take for a primary that has I/O suspended due to lost quorum\nand learns its data is outdated (drbd.conf option\non-suspended-primary-outdated).\n\nAssociated type: [`OnSuspendedPrimaryOutdatedPolicy`] (enum)"]
    pub fn push_on_suspended_primary_outdated(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 14u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Whether the resource operates in DRBD 8 compatible, single-peer mode.\n"]
    pub fn push_drbd8_compat_mode(mut self, value: u8) -> Self {
        push_header(self.as_vec_mut(), 15u16, 1 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Whether to explicitly (re)configure DRBD 8 compatible mode whenever a\nnode-id is configured, instead of leaving it unchanged.\n"]
    pub fn push_explicit_drbd8_compat(mut self, value: u8) -> Self {
        push_header(self.as_vec_mut(), 16u16, 1 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
}
impl<Prev: Pusher> Drop for PushResourceOpts<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushNetConf<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushNetConf<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushNetConf<Prev> {
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
    #[doc = "Shared secret used for peer authentication (drbd.conf option\nshared-secret).\n"]
    pub fn push_shared_secret(mut self, value: &CStr) -> Self {
        push_header(
            self.as_vec_mut(),
            1u16,
            value.to_bytes_with_nul().len() as u16,
        );
        self.as_vec_mut().extend(value.to_bytes_with_nul());
        self
    }
    #[doc = "Shared secret used for peer authentication (drbd.conf option\nshared-secret).\n"]
    pub fn push_shared_secret_bytes(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 1u16, (value.len() + 1) as u16);
        self.as_vec_mut().extend(value);
        self.as_vec_mut().push(0);
        self
    }
    #[doc = "Hash-based message authentication code (HMAC) algorithm used for peer\nauthentication (drbd.conf option cram-hmac-alg).\n"]
    pub fn push_cram_hmac_alg(mut self, value: &CStr) -> Self {
        push_header(
            self.as_vec_mut(),
            2u16,
            value.to_bytes_with_nul().len() as u16,
        );
        self.as_vec_mut().extend(value.to_bytes_with_nul());
        self
    }
    #[doc = "Hash-based message authentication code (HMAC) algorithm used for peer\nauthentication (drbd.conf option cram-hmac-alg).\n"]
    pub fn push_cram_hmac_alg_bytes(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 2u16, (value.len() + 1) as u16);
        self.as_vec_mut().extend(value);
        self.as_vec_mut().push(0);
        self
    }
    #[doc = "Data integrity algorithm used, in addition to the TCP/IP checksum, to\ndetect corruption of data received over the network (drbd.conf option\ndata-integrity-alg).\n"]
    pub fn push_integrity_alg(mut self, value: &CStr) -> Self {
        push_header(
            self.as_vec_mut(),
            3u16,
            value.to_bytes_with_nul().len() as u16,
        );
        self.as_vec_mut().extend(value.to_bytes_with_nul());
        self
    }
    #[doc = "Data integrity algorithm used, in addition to the TCP/IP checksum, to\ndetect corruption of data received over the network (drbd.conf option\ndata-integrity-alg).\n"]
    pub fn push_integrity_alg_bytes(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 3u16, (value.len() + 1) as u16);
        self.as_vec_mut().extend(value);
        self.as_vec_mut().push(0);
        self
    }
    #[doc = "Hash algorithm used to compute and compare block checksums during online\nverification (drbd.conf option verify-alg).\n"]
    pub fn push_verify_alg(mut self, value: &CStr) -> Self {
        push_header(
            self.as_vec_mut(),
            4u16,
            value.to_bytes_with_nul().len() as u16,
        );
        self.as_vec_mut().extend(value.to_bytes_with_nul());
        self
    }
    #[doc = "Hash algorithm used to compute and compare block checksums during online\nverification (drbd.conf option verify-alg).\n"]
    pub fn push_verify_alg_bytes(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 4u16, (value.len() + 1) as u16);
        self.as_vec_mut().extend(value);
        self.as_vec_mut().push(0);
        self
    }
    #[doc = "Hash algorithm used to compare blocks before resending identical data\nduring a resync (drbd.conf option csums-alg).\n"]
    pub fn push_csums_alg(mut self, value: &CStr) -> Self {
        push_header(
            self.as_vec_mut(),
            5u16,
            value.to_bytes_with_nul().len() as u16,
        );
        self.as_vec_mut().extend(value.to_bytes_with_nul());
        self
    }
    #[doc = "Hash algorithm used to compare blocks before resending identical data\nduring a resync (drbd.conf option csums-alg).\n"]
    pub fn push_csums_alg_bytes(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 5u16, (value.len() + 1) as u16);
        self.as_vec_mut().extend(value);
        self.as_vec_mut().push(0);
        self
    }
    #[doc = "Replication protocol used on this connection.\n\nAssociated type: [`WireProtocol`] (enum)"]
    pub fn push_protocol(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 6u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "How long to wait before retrying to establish a connection to the peer,\nin seconds (drbd.conf option connect-int).\n"]
    pub fn push_connect_int(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 7u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Timeout for replies over the network before considering a peer dead, in\nunits of 0.1 seconds (drbd.conf option timeout).\n"]
    pub fn push_timeout(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 8u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Interval of connection idle time after which DRBD sends a keep-alive\npacket, in seconds (drbd.conf option ping-int).\n"]
    pub fn push_ping_int(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 9u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Timeout for replies to keep-alive packets, in units of 0.1 seconds\n(drbd.conf option ping-timeout).\n"]
    pub fn push_ping_timeo(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 10u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Size of the TCP/IP send buffer, in bytes, or 0 to size it automatically\n(drbd.conf option sndbuf-size).\n"]
    pub fn push_sndbuf_size(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 11u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Size of the TCP/IP receive buffer, in bytes, or 0 to size it\nautomatically (drbd.conf option rcvbuf-size).\n"]
    pub fn push_rcvbuf_size(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 12u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Number of times the timeout may be exceeded before a non-responding peer\nis excluded from the cluster (drbd.conf option ko-count).\n"]
    pub fn push_ko_count(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 13u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Maximum number of write requests DRBD may issue before issuing a write\nbarrier (drbd.conf option max-epoch-size).\n"]
    pub fn push_max_epoch_size(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 14u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Split-brain recovery policy when neither node is primary.\n\nAssociated type: [`AfterSbPolicy`] (enum)"]
    pub fn push_after_sb_0pri(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 15u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Split-brain recovery policy when exactly one node is primary.\n\nAssociated type: [`AfterSbPolicy`] (enum)"]
    pub fn push_after_sb_1pri(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 16u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Split-brain recovery policy when both nodes are primary.\n\nAssociated type: [`AfterSbPolicy`] (enum)"]
    pub fn push_after_sb_2pri(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 17u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Policy for resolving a role/resync conflict left over from a split-brain\nrecovery (drbd.conf option rr-conflict).\n\nAssociated type: [`AfterSbPolicy`] (enum)"]
    pub fn push_rr_conflict(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 18u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Action to take when the replication link\\'s TCP send queue is congested\n(drbd.conf option on-congestion).\n\nAssociated type: [`OnCongestionPolicy`] (enum)"]
    pub fn push_on_congestion(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 19u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Amount of in-flight data, in sectors, allowed on this connection before\nswitching into ahead/behind mode, or 0 to disable (drbd.conf option\ncongestion-fill).\n"]
    pub fn push_cong_fill(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 20u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Number of active bitmap extents allowed before switching into\nahead/behind mode (drbd.conf option congestion-extents).\n"]
    pub fn push_cong_extents(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 21u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Whether to allow both nodes of this connection to be primary at the same\ntime (drbd.conf option allow-two-primaries).\n"]
    pub fn push_two_primaries(mut self, value: u8) -> Self {
        push_header(self.as_vec_mut(), 22u16, 1 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Whether to use the TCP_CORK socket option to coalesce partial messages\ninto fewer, bigger packets (drbd.conf option tcp-cork).\n"]
    pub fn push_tcp_cork(mut self, value: u8) -> Self {
        push_header(self.as_vec_mut(), 23u16, 1 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Whether to apply the automatic split-brain recovery policies even when\nthe UUIDs indicate a third node may be involved (drbd.conf option\nalways-asbp).\n"]
    pub fn push_always_asbp(mut self, value: u8) -> Self {
        push_header(self.as_vec_mut(), 24u16, 1 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Whether to run-length encode the bitmap exchanged with the peer when a\nconnection is established (drbd.conf option use-rle).\n"]
    pub fn push_use_rle(mut self, value: u8) -> Self {
        push_header(self.as_vec_mut(), 25u16, 1 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Fencing policy used to avoid a split-brain when both nodes are primary\nand disconnected.\n\nAssociated type: [`FencingPolicy`] (enum)"]
    pub fn push_fencing(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 26u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Name of the connection this net-conf applies to.\n"]
    pub fn push_connection_name(mut self, value: &CStr) -> Self {
        push_header(
            self.as_vec_mut(),
            27u16,
            value.to_bytes_with_nul().len() as u16,
        );
        self.as_vec_mut().extend(value.to_bytes_with_nul());
        self
    }
    #[doc = "Name of the connection this net-conf applies to.\n"]
    pub fn push_connection_name_bytes(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 27u16, (value.len() + 1) as u16);
        self.as_vec_mut().extend(value);
        self.as_vec_mut().push(0);
        self
    }
    #[doc = "Whether to compare block checksums only for the resync after a primary\ncrash, rather than for every resync (drbd.conf option\ncsums-after-crash-only).\n"]
    pub fn push_csums_after_crash_only(mut self, value: u8) -> Self {
        push_header(self.as_vec_mut(), 28u16, 1 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Timeout used to poll the send/receive socket for a broken TCP connection\nwhen DRBD-proxy is in use, in units of 0.1 seconds (drbd.conf option\nsocket-check-timeout).\n"]
    pub fn push_sock_check_timeo(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 29u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Name of the network transport module used for this connection, e.g. tcp\nor rdma (drbd.conf option transport).\n"]
    pub fn push_transport_name(mut self, value: &CStr) -> Self {
        push_header(
            self.as_vec_mut(),
            30u16,
            value.to_bytes_with_nul().len() as u16,
        );
        self.as_vec_mut().extend(value.to_bytes_with_nul());
        self
    }
    #[doc = "Name of the network transport module used for this connection, e.g. tcp\nor rdma (drbd.conf option transport).\n"]
    pub fn push_transport_name_bytes(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 30u16, (value.len() + 1) as u16);
        self.as_vec_mut().extend(value);
        self.as_vec_mut().push(0);
        self
    }
    #[doc = "Maximum number of pages used for receive buffers and internal\nresync/verify buffers per device (drbd.conf option max-buffers).\n"]
    pub fn push_max_buffers(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 31u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Whether to allow reads to be served from a peer\\'s disk when the local\ndisk is not up to date (drbd.conf option allow-remote-read).\n"]
    pub fn push_allow_remote_read(mut self, value: u8) -> Self {
        push_header(self.as_vec_mut(), 32u16, 1 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Whether to encrypt this connection with TLS (drbd.conf option tls).\n"]
    pub fn push_tls(mut self, value: u8) -> Self {
        push_header(self.as_vec_mut(), 33u16, 1 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Serial number of the kernel keyring key holding the private key used for\nTLS encryption (drbd.conf option tls-privkey).\n"]
    pub fn push_tls_privkey(mut self, value: i32) -> Self {
        push_header(self.as_vec_mut(), 34u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Serial number of the kernel keyring key holding the certificate used for\nTLS encryption (drbd.conf option tls-certificate).\n"]
    pub fn push_tls_certificate(mut self, value: i32) -> Self {
        push_header(self.as_vec_mut(), 35u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Serial number of the kernel keyring used to look up the TLS key material\n(drbd.conf option tls-keyring).\n"]
    pub fn push_tls_keyring(mut self, value: i32) -> Self {
        push_header(self.as_vec_mut(), 36u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Whether to load-balance traffic for this connection across all\nconfigured paths instead of using only one at a time (drbd.conf option\nload-balance-paths).\n"]
    pub fn push_load_balance_paths(mut self, value: u8) -> Self {
        push_header(self.as_vec_mut(), 37u16, 1 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Size, in bytes, of the RDMA transport\\'s control-stream receive buffer,\nor 0 to derive it from rcvbuf-size (drbd.conf option\nrdma-ctrl-rcvbuf-size).\n"]
    pub fn push_rdma_ctrl_rcvbuf_size(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 38u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Size, in bytes, of the RDMA transport\\'s control-stream send buffer, or\n0 to derive it from sndbuf-size (drbd.conf option\nrdma-ctrl-sndbuf-size).\n"]
    pub fn push_rdma_ctrl_sndbuf_size(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 39u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
}
impl<Prev: Pusher> Drop for PushNetConf<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushSetRoleParms<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushSetRoleParms<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushSetRoleParms<Prev> {
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
    #[doc = "Request that the promotion to primary succeed even if some devices are\nnot guaranteed to have up-to-date data (drbdsetup primary option force).\n"]
    pub fn push_force(mut self, value: ()) -> Self {
        push_header(self.as_vec_mut(), 1u16, 0 as u16);
        self
    }
}
impl<Prev: Pusher> Drop for PushSetRoleParms<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushResizeParms<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushResizeParms<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushResizeParms<Prev> {
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
    #[doc = "New size of the device, in sectors, or 0 to use the full size of the\nbacking devices (drbdsetup resize option size).\n"]
    pub fn push_size(mut self, value: u64) -> Self {
        push_header(self.as_vec_mut(), 1u16, 8 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Request that the resize proceed even though not all peer devices are\ncurrently connected, assuming they already have enough space.\n"]
    pub fn push_assume_peer_has_space(mut self, value: ()) -> Self {
        push_header(self.as_vec_mut(), 2u16, 0 as u16);
        self
    }
    #[doc = "Request that the additional space not be resynchronized, assuming it is\nalready identical on all nodes.\n"]
    pub fn push_assume_clean(mut self, value: ()) -> Self {
        push_header(self.as_vec_mut(), 3u16, 0 as u16);
        self
    }
    #[doc = "Number of activity log stripes to use after the resize (drbdsetup resize\noption al-stripes).\n"]
    pub fn push_al_stripes(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 4u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Size of each activity log stripe, in KiB, after the resize (drbdsetup\nresize option al-stripe-size).\n"]
    pub fn push_al_stripe_size(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 5u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
}
impl<Prev: Pusher> Drop for PushResizeParms<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushStartOvParms<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushStartOvParms<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushStartOvParms<Prev> {
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
    #[doc = "Sector at which to start online verification, to resume a previously\nstopped run.\n"]
    pub fn push_start_sector(mut self, value: u64) -> Self {
        push_header(self.as_vec_mut(), 1u16, 8 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Sector at which to stop online verification.\n"]
    pub fn push_stop_sector(mut self, value: u64) -> Self {
        push_header(self.as_vec_mut(), 2u16, 8 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
}
impl<Prev: Pusher> Drop for PushStartOvParms<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushNewCurrentUuidParms<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushNewCurrentUuidParms<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushNewCurrentUuidParms<Prev> {
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
    #[doc = "Request that the peer\\'s bitmap also be cleared, marking the peer as\nfully in sync without a resync (drbdsetup new-current-uuid option\nclear-bitmap).\n"]
    pub fn push_clear_bm(mut self, value: ()) -> Self {
        push_header(self.as_vec_mut(), 1u16, 0 as u16);
        self
    }
    #[doc = "Request that the peer be marked out of sync so that a resync starts,\ninstead of assuming it already has the same data (drbdsetup\nnew-current-uuid option force-resync).\n"]
    pub fn push_force_resync(mut self, value: ()) -> Self {
        push_header(self.as_vec_mut(), 2u16, 0 as u16);
        self
    }
}
impl<Prev: Pusher> Drop for PushNewCurrentUuidParms<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushDisconnectParms<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushDisconnectParms<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushDisconnectParms<Prev> {
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
    #[doc = "Request that the connection be torn down immediately, discarding any\npending, not yet acknowledged data instead of waiting for it (drbdsetup\ndisconnect option force).\n"]
    pub fn push_force(mut self, value: ()) -> Self {
        push_header(self.as_vec_mut(), 1u16, 0 as u16);
        self
    }
}
impl<Prev: Pusher> Drop for PushDisconnectParms<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushDetachParms<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushDetachParms<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushDetachParms<Prev> {
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
    #[doc = "Request that the detach proceed immediately, marking the lower-level\ndevice failed until pending I/O completes (drbdsetup detach option\nforce).\n"]
    pub fn push_force(mut self, value: ()) -> Self {
        push_header(self.as_vec_mut(), 1u16, 0 as u16);
        self
    }
    #[doc = "Request that the device be marked permanently, intentionally diskless\ninstead of temporarily diskless (drbdsetup detach option diskless).\n"]
    pub fn push_intentional_diskless_detach(mut self, value: ()) -> Self {
        push_header(self.as_vec_mut(), 2u16, 0 as u16);
        self
    }
}
impl<Prev: Pusher> Drop for PushDetachParms<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushAttachParms<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushAttachParms<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushAttachParms<Prev> {
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
    #[doc = "Request that, should the handshake with a peer find a split brain, the\nlocal data be discarded and resynchronized from that peer (drbdsetup\nattach option discard-my-data).\n"]
    pub fn push_discard_my_data(mut self, value: ()) -> Self {
        push_header(self.as_vec_mut(), 1u16, 0 as u16);
        self
    }
}
impl<Prev: Pusher> Drop for PushAttachParms<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushDeviceConf<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushDeviceConf<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushDeviceConf<Prev> {
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
    #[doc = "Maximum size of a single I/O request the device accepts, in bytes\n(drbdsetup new-minor option max-bio-size).\n"]
    pub fn push_max_bio_size(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 1u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Whether the device is deliberately created without a local disk, e.g. as\na diskless client (drbdsetup new-minor option diskless).\n"]
    pub fn push_intentional_diskless(mut self, value: u8) -> Self {
        push_header(self.as_vec_mut(), 2u16, 1 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Logical block size the device reports, needed to combine differing\nbacking disk block sizes across nodes (drbd.conf option block-size).\n"]
    pub fn push_block_size(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 3u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Discard granularity, in bytes, advertised by the device; discard\nrequests must be aligned to and a multiple of this value (drbd.conf\noption discard-granularity).\n"]
    pub fn push_discard_granularity(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 4u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
}
impl<Prev: Pusher> Drop for PushDeviceConf<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushResourceInfo<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushResourceInfo<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushResourceInfo<Prev> {
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
    #[doc = "Role of the resource on this node.\n\nAssociated type: [`Role`] (enum)"]
    pub fn push_role(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 1u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Whether I/O on the resource is suspended by the user.\n"]
    pub fn push_susp(mut self, value: u8) -> Self {
        push_header(self.as_vec_mut(), 2u16, 1 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Whether I/O on the resource is suspended because no data is accessible.\n"]
    pub fn push_susp_nod(mut self, value: u8) -> Self {
        push_header(self.as_vec_mut(), 3u16, 1 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Whether I/O on the resource is suspended because a fence-peer handler is\nrunning.\n"]
    pub fn push_susp_fen(mut self, value: u8) -> Self {
        push_header(self.as_vec_mut(), 4u16, 1 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Whether I/O on the resource is suspended because it has lost quorum.\n"]
    pub fn push_susp_quorum(mut self, value: u8) -> Self {
        push_header(self.as_vec_mut(), 5u16, 1 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Whether all I/O on the resource fails because it was forced to demote\nfrom primary.\n"]
    pub fn push_fail_io(mut self, value: u8) -> Self {
        push_header(self.as_vec_mut(), 6u16, 1 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
}
impl<Prev: Pusher> Drop for PushResourceInfo<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushDeviceInfo<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushDeviceInfo<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushDeviceInfo<Prev> {
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
    #[doc = "Current state of the backing disk.\n\nAssociated type: [`DiskState`] (enum)"]
    pub fn push_disk_state(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 1u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Whether the device is deliberately configured without a local disk.\n"]
    pub fn push_is_intentional_diskless(mut self, value: u8) -> Self {
        push_header(self.as_vec_mut(), 2u16, 1 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Whether the device currently has quorum.\n"]
    pub fn push_has_quorum(mut self, value: u8) -> Self {
        push_header(self.as_vec_mut(), 3u16, 1 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Whether the device is currently opened by an application.\n"]
    pub fn push_is_open(mut self, value: u8) -> Self {
        push_header(self.as_vec_mut(), 4u16, 1 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Path of the backing device, present while it is still accessible after a\ndisk failure.\n"]
    pub fn push_backing_dev_path(mut self, value: &CStr) -> Self {
        push_header(
            self.as_vec_mut(),
            5u16,
            value.to_bytes_with_nul().len() as u16,
        );
        self.as_vec_mut().extend(value.to_bytes_with_nul());
        self
    }
    #[doc = "Path of the backing device, present while it is still accessible after a\ndisk failure.\n"]
    pub fn push_backing_dev_path_bytes(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 5u16, (value.len() + 1) as u16);
        self.as_vec_mut().extend(value);
        self.as_vec_mut().push(0);
        self
    }
}
impl<Prev: Pusher> Drop for PushDeviceInfo<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushConnectionInfo<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushConnectionInfo<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushConnectionInfo<Prev> {
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
    #[doc = "Current state of the connection to the peer.\n\nAssociated type: [`ConnectionState`] (enum)"]
    pub fn push_connection_state(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 1u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Current role of the peer.\n\nAssociated type: [`Role`] (enum)"]
    pub fn push_role(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 2u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
}
impl<Prev: Pusher> Drop for PushConnectionInfo<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushPeerDeviceInfo<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushPeerDeviceInfo<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushPeerDeviceInfo<Prev> {
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
    #[doc = "Current replication state with the peer.\n\nAssociated type: [`ReplState`] (enum)"]
    pub fn push_repl_state(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 1u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Current state of the peer\\'s disk.\n\nAssociated type: [`DiskState`] (enum)"]
    pub fn push_disk_state(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 2u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Whether resynchronization with this peer is suspended by the user.\n"]
    pub fn push_resync_susp_user(mut self, value: u8) -> Self {
        push_header(self.as_vec_mut(), 3u16, 1 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Whether resynchronization with this peer is suspended by the peer.\n"]
    pub fn push_resync_susp_peer(mut self, value: u8) -> Self {
        push_header(self.as_vec_mut(), 4u16, 1 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Whether resynchronization with this peer is suspended because it depends\non another volume or connection resynchronizing first.\n"]
    pub fn push_resync_susp_dependency(mut self, value: u8) -> Self {
        push_header(self.as_vec_mut(), 5u16, 1 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Whether the peer is deliberately configured without a local disk.\n"]
    pub fn push_is_intentional_diskless(mut self, value: u8) -> Self {
        push_header(self.as_vec_mut(), 6u16, 1 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Whether resynchronization with this peer is suspended to enforce the\nmaximum number of parallel resyncs.\n"]
    pub fn push_resync_susp_max_parallel(mut self, value: u8) -> Self {
        push_header(self.as_vec_mut(), 7u16, 1 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
}
impl<Prev: Pusher> Drop for PushPeerDeviceInfo<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushResourceStatistics<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushResourceStatistics<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushResourceStatistics<Prev> {
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
    #[doc = "Write ordering method currently in use for the resource\\'s backing\ndevices.\n\nAssociated type: [`WriteOrdering`] (enum)"]
    pub fn push_write_ordering(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 1u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
}
impl<Prev: Pusher> Drop for PushResourceStatistics<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushDeviceStatistics<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushDeviceStatistics<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushDeviceStatistics<Prev> {
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
    #[doc = "Capacity of the device, in sectors.\n"]
    pub fn push_size(mut self, value: u64) -> Self {
        push_header(self.as_vec_mut(), 1u16, 8 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Total data read from the device, in sectors, since it was attached.\n"]
    pub fn push_read(mut self, value: u64) -> Self {
        push_header(self.as_vec_mut(), 2u16, 8 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Total data written to the device, in sectors, since it was attached.\n"]
    pub fn push_write(mut self, value: u64) -> Self {
        push_header(self.as_vec_mut(), 3u16, 8 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Number of activity log metadata writes since the device was attached.\n"]
    pub fn push_al_writes(mut self, value: u64) -> Self {
        push_header(self.as_vec_mut(), 4u16, 8 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Number of bitmap metadata writes since the device was attached.\n"]
    pub fn push_bm_writes(mut self, value: u64) -> Self {
        push_header(self.as_vec_mut(), 5u16, 8 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Number of application I/O requests currently in flight on the device.\n"]
    pub fn push_upper_pending(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 6u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Number of I/O requests currently in flight on the backing device.\n"]
    pub fn push_lower_pending(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 7u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Whether application I/O to the device is currently throttled.\n"]
    pub fn push_upper_blocked(mut self, value: u8) -> Self {
        push_header(self.as_vec_mut(), 8u16, 1 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Whether I/O to the backing device is currently blocked.\n"]
    pub fn push_lower_blocked(mut self, value: u8) -> Self {
        push_header(self.as_vec_mut(), 9u16, 1 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Whether updates to the activity log are currently suspended.\n"]
    pub fn push_al_suspended(mut self, value: u8) -> Self {
        push_header(self.as_vec_mut(), 10u16, 1 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Data generation UUID currently exposed to peers, identifying the\ngeneration of data this node has.\n"]
    pub fn push_exposed_data_uuid(mut self, value: u64) -> Self {
        push_header(self.as_vec_mut(), 11u16, 8 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Current data generation UUID of the device\\'s metadata.\n"]
    pub fn push_current_uuid(mut self, value: u64) -> Self {
        push_header(self.as_vec_mut(), 12u16, 8 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Bitmask of the backing disk\\'s metadata flags.\n"]
    pub fn push_disk_flags(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 13u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Historical bitmap UUIDs of prior data generations, one slot per peer.\n"]
    pub fn push_history_uuids(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 14u16, value.len() as u16);
        self.as_vec_mut().extend(value);
        self
    }
}
impl<Prev: Pusher> Drop for PushDeviceStatistics<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushConnectionStatistics<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushConnectionStatistics<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushConnectionStatistics<Prev> {
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
    #[doc = "Whether the network transport to the peer is currently congested.\n"]
    pub fn push_congested(mut self, value: u8) -> Self {
        push_header(self.as_vec_mut(), 1u16, 1 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Application data sent to the peer but not yet acknowledged, in sectors.\n"]
    pub fn push_ap_in_flight(mut self, value: u64) -> Self {
        push_header(self.as_vec_mut(), 2u16, 8 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Resync data sent to the peer but not yet acknowledged, in sectors.\n"]
    pub fn push_rs_in_flight(mut self, value: u64) -> Self {
        push_header(self.as_vec_mut(), 3u16, 8 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
}
impl<Prev: Pusher> Drop for PushConnectionStatistics<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushPeerDeviceStatistics<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushPeerDeviceStatistics<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushPeerDeviceStatistics<Prev> {
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
    #[doc = "Total data received from the peer for this device, in sectors.\n"]
    pub fn push_received(mut self, value: u64) -> Self {
        push_header(self.as_vec_mut(), 1u16, 8 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Total data sent to the peer for this device, in sectors.\n"]
    pub fn push_sent(mut self, value: u64) -> Self {
        push_header(self.as_vec_mut(), 2u16, 8 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Number of requests to this peer waiting for an acknowledgement.\n"]
    pub fn push_pending(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 3u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Number of requests from this peer that have not yet been acknowledged.\n"]
    pub fn push_unacked(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 4u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Amount of data known to differ from this peer, in sectors.\n"]
    pub fn push_out_of_sync(mut self, value: u64) -> Self {
        push_header(self.as_vec_mut(), 5u16, 8 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Amount of data that failed to resynchronize with this peer, in sectors.\n"]
    pub fn push_resync_failed(mut self, value: u64) -> Self {
        push_header(self.as_vec_mut(), 6u16, 8 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Bitmap UUID recorded for this peer.\n"]
    pub fn push_bitmap_uuid(mut self, value: u64) -> Self {
        push_header(self.as_vec_mut(), 7u16, 8 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Bitmask of this peer device\\'s metadata flags.\n"]
    pub fn push_flags(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 8u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Total amount of data to resynchronize with this peer for the current\nresync, in sectors.\n"]
    pub fn push_rs_total(mut self, value: u64) -> Self {
        push_header(self.as_vec_mut(), 9u16, 8 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Sector at which the current online verify run started.\n"]
    pub fn push_ov_start_sector(mut self, value: u64) -> Self {
        push_header(self.as_vec_mut(), 10u16, 8 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Sector at which the current online verify run will stop.\n"]
    pub fn push_ov_stop_sector(mut self, value: u64) -> Self {
        push_header(self.as_vec_mut(), 11u16, 8 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Current position of an ongoing online verify run, in sectors.\n"]
    pub fn push_ov_position(mut self, value: u64) -> Self {
        push_header(self.as_vec_mut(), 12u16, 8 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Amount of data left to verify in an ongoing online verify run, in\nsectors.\n"]
    pub fn push_ov_left(mut self, value: u64) -> Self {
        push_header(self.as_vec_mut(), 13u16, 8 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Amount of data skipped during an ongoing online verify run, in sectors.\n"]
    pub fn push_ov_skipped(mut self, value: u64) -> Self {
        push_header(self.as_vec_mut(), 14u16, 8 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Amount of resync data found identical by checksum comparison and not\nretransmitted, in sectors.\n"]
    pub fn push_rs_same_csum(mut self, value: u64) -> Self {
        push_header(self.as_vec_mut(), 15u16, 8 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Time elapsed since the current resync with this peer started, in\nmilliseconds.\n"]
    pub fn push_rs_dt_start_ms(mut self, value: u64) -> Self {
        push_header(self.as_vec_mut(), 16u16, 8 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Total time the current resync with this peer has been paused, in\nmilliseconds.\n"]
    pub fn push_rs_paused_ms(mut self, value: u64) -> Self {
        push_header(self.as_vec_mut(), 17u16, 8 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Time since the most recent resync progress mark, in milliseconds.\n"]
    pub fn push_rs_dt0_ms(mut self, value: u64) -> Self {
        push_header(self.as_vec_mut(), 18u16, 8 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Data resynchronized since the most recent resync progress mark, in\nsectors.\n"]
    pub fn push_rs_db0_sectors(mut self, value: u64) -> Self {
        push_header(self.as_vec_mut(), 19u16, 8 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Time since the resync progress mark before the most recent one, in\nmilliseconds.\n"]
    pub fn push_rs_dt1_ms(mut self, value: u64) -> Self {
        push_header(self.as_vec_mut(), 20u16, 8 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Data resynchronized since the resync progress mark before the most\nrecent one, in sectors.\n"]
    pub fn push_rs_db1_sectors(mut self, value: u64) -> Self {
        push_header(self.as_vec_mut(), 21u16, 8 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Current dynamically controlled resync rate for this peer, in KiB/s.\n"]
    pub fn push_rs_c_sync_rate(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 22u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Bitmask of UUID-related flags negotiated with this peer.\n"]
    pub fn push_uuid_flags(mut self, value: u64) -> Self {
        push_header(self.as_vec_mut(), 23u16, 8 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
}
impl<Prev: Pusher> Drop for PushPeerDeviceStatistics<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushHelperInfo<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushHelperInfo<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushHelperInfo<Prev> {
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
    #[doc = "Name of the handler script being invoked.\n"]
    pub fn push_name(mut self, value: &CStr) -> Self {
        push_header(
            self.as_vec_mut(),
            1u16,
            value.to_bytes_with_nul().len() as u16,
        );
        self.as_vec_mut().extend(value.to_bytes_with_nul());
        self
    }
    #[doc = "Name of the handler script being invoked.\n"]
    pub fn push_name_bytes(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 1u16, (value.len() + 1) as u16);
        self.as_vec_mut().extend(value);
        self.as_vec_mut().push(0);
        self
    }
    #[doc = "Exit status of the handler script, present when phase is response.\n"]
    pub fn push_status(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 2u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Whether this notification announces the call or reports its result.\n\nAssociated type: [`HelperPhase`] (enum)"]
    pub fn push_phase(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 3u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
}
impl<Prev: Pusher> Drop for PushHelperInfo<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushInvalidateParms<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushInvalidateParms<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushInvalidateParms<Prev> {
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
    #[doc = "Node ID of the peer to resynchronize from; if absent, the kernel chooses\na peer.\n"]
    pub fn push_sync_from_peer_node_id(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 1u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Whether to mark the whole bitmap out of sync before starting the resync,\ninstead of resyncing only the blocks already marked out of sync\n(drbdsetup invalidate option reset-bitmap).\n"]
    pub fn push_reset_bitmap(mut self, value: u8) -> Self {
        push_header(self.as_vec_mut(), 2u16, 1 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
}
impl<Prev: Pusher> Drop for PushInvalidateParms<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushPeerDeviceConf<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushPeerDeviceConf<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushPeerDeviceConf<Prev> {
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
    #[doc = "Bandwidth DRBD may use for resynchronizing with this peer when the\ndynamic resync controller is disabled, in KiB/s (drbd.conf option\nresync-rate).\n"]
    pub fn push_resync_rate(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 1u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "How fast the dynamic resync-rate controller adapts to changes in resync\nspeed, in units of 0.1 seconds, or 0 to use a fixed resync-rate instead\n(drbd.conf option c-plan-ahead).\n"]
    pub fn push_c_plan_ahead(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 2u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Delay the dynamic resync-rate controller aims for along the resync path,\nin units of 0.1 seconds (drbd.conf option c-delay-target).\n"]
    pub fn push_c_delay_target(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 3u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Amount of resync data the dynamic resync-rate controller aims to keep in\nflight, in sectors (drbd.conf option c-fill-target).\n"]
    pub fn push_c_fill_target(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 4u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Maximum bandwidth used by the dynamic resync-rate controller, in KiB/s,\nor 0 for no limit (drbd.conf option c-max-rate).\n"]
    pub fn push_c_max_rate(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 5u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Bandwidth reserved for resync I/O against this peer when this node is\nthe sync source and primary, in KiB/s (drbd.conf option c-min-rate).\n"]
    pub fn push_c_min_rate(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 6u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Whether to allocate a bitmap slot for this peer, overriding the\nautomatic default for a diskless peer (drbd.conf option bitmap).\n"]
    pub fn push_bitmap(mut self, value: u8) -> Self {
        push_header(self.as_vec_mut(), 7u16, 1 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Whether to suspend replication of application writes to this peer while\nit is being resynchronized (drbd.conf option\nresync-without-replication).\n"]
    pub fn push_resync_without_replication(mut self, value: u8) -> Self {
        push_header(self.as_vec_mut(), 8u16, 1 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Whether this peer participates as a tiebreaker in quorum decisions even\nthough it is intentionally diskless (drbd.conf option peer-tiebreaker).\n"]
    pub fn push_peer_tiebreaker(mut self, value: u8) -> Self {
        push_header(self.as_vec_mut(), 9u16, 1 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
}
impl<Prev: Pusher> Drop for PushPeerDeviceConf<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushConnectParms<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushConnectParms<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushConnectParms<Prev> {
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
    #[doc = "Request that DRBD only determine whether a connection could be\nestablished and which resync direction would be used, without actually\nconnecting or resyncing (drbdsetup connect option tentative).\n"]
    pub fn push_tentative(mut self, value: ()) -> Self {
        push_header(self.as_vec_mut(), 1u16, 0 as u16);
        self
    }
    #[doc = "Request that local data be discarded and resynchronized from the peer\nthat has the most up-to-date data (drbdsetup connect option\ndiscard-my-data).\n"]
    pub fn push_discard_my_data(mut self, value: ()) -> Self {
        push_header(self.as_vec_mut(), 2u16, 0 as u16);
        self
    }
}
impl<Prev: Pusher> Drop for PushConnectParms<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushPathInfo<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushPathInfo<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushPathInfo<Prev> {
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
    #[doc = "Whether the path\\'s transport connection is currently established.\n"]
    pub fn push_established(mut self, value: u8) -> Self {
        push_header(self.as_vec_mut(), 1u16, 1 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
}
impl<Prev: Pusher> Drop for PushPathInfo<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushRenameParms<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushRenameParms<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushRenameParms<Prev> {
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
    #[doc = "New name for the resource.\n"]
    pub fn push_new_name(mut self, value: &CStr) -> Self {
        push_header(
            self.as_vec_mut(),
            1u16,
            value.to_bytes_with_nul().len() as u16,
        );
        self.as_vec_mut().extend(value.to_bytes_with_nul());
        self
    }
    #[doc = "New name for the resource.\n"]
    pub fn push_new_name_bytes(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 1u16, (value.len() + 1) as u16);
        self.as_vec_mut().extend(value);
        self.as_vec_mut().push(0);
        self
    }
}
impl<Prev: Pusher> Drop for PushRenameParms<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushInvalidatePeerParms<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushInvalidatePeerParms<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushInvalidatePeerParms<Prev> {
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
    #[doc = "Whether to mark the whole bitmap out of sync before starting the resync,\ninstead of resyncing only the blocks already marked out of sync\n(drbdsetup invalidate-remote option reset-bitmap).\n"]
    pub fn push_reset_bitmap(mut self, value: u8) -> Self {
        push_header(self.as_vec_mut(), 1u16, 1 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
}
impl<Prev: Pusher> Drop for PushInvalidatePeerParms<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushSuspendIoParms<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushSuspendIoParms<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushSuspendIoParms<Prev> {
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
    #[doc = "Whether to freeze the filesystem mounted on the device to bring it into\na consistent state before suspending I/O (drbdsetup suspend-io option\nbdev-freeze).\n"]
    pub fn push_bdev_freeze(mut self, value: u8) -> Self {
        push_header(self.as_vec_mut(), 1u16, 1 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Whether to wait until the peers have acknowledged the barrier that\ncloses the last epoch, so that the writes before the suspension are on\ntheir stable storage also with protocols A and B (drbdsetup suspend-io\noption flush-to-peers).\n"]
    pub fn push_flush_to_peers(mut self, value: u8) -> Self {
        push_header(self.as_vec_mut(), 2u16, 1 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
}
impl<Prev: Pusher> Drop for PushSuspendIoParms<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushCbtStartParms<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushCbtStartParms<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushCbtStartParms<Prev> {
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
    #[doc = "Caller-chosen identifier of the backup snapshot the new tracking\ninterval starts at; required.\n"]
    pub fn push_bookmark_id(mut self, value: u64) -> Self {
        push_header(self.as_vec_mut(), 1u16, 8 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
}
impl<Prev: Pusher> Drop for PushCbtStartParms<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushCbtInfo<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushCbtInfo<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushCbtInfo<Prev> {
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
    #[doc = "Whether a bitmap slot tracks the blocks changed since a bookmark.\n"]
    pub fn push_active(mut self, value: u8) -> Self {
        push_header(self.as_vec_mut(), 1u16, 1 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Whether the tracking bitmap holds every change since the bookmark;\ncleared when changes may have been missed, which requires a full backup.\n"]
    pub fn push_valid(mut self, value: u8) -> Self {
        push_header(self.as_vec_mut(), 2u16, 1 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Identifier given to the device-cbt-start that started the interval.\n"]
    pub fn push_bookmark_id(mut self, value: u64) -> Self {
        push_header(self.as_vec_mut(), 3u16, 8 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Time of that device-cbt-start, in seconds since the epoch.\n"]
    pub fn push_since(mut self, value: u64) -> Self {
        push_header(self.as_vec_mut(), 4u16, 8 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Current UUID of the device at that device-cbt-start.\n"]
    pub fn push_bitmap_uuid(mut self, value: u64) -> Self {
        push_header(self.as_vec_mut(), 5u16, 8 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Data generation tag of the Primary at that device-cbt-start; on a\nSecondary the last one received from the Primary.\n"]
    pub fn push_bitmap_dagtag(mut self, value: u64) -> Self {
        push_header(self.as_vec_mut(), 6u16, 8 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
}
impl<Prev: Pusher> Drop for PushCbtInfo<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushResource<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushResource<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushResource<Prev> {
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
    #[doc = "Identity of the object.\n"]
    pub fn nested_context(mut self) -> PushContext<Self> {
        let header_offset = push_nested_header(self.as_vec_mut(), 1u16);
        PushContext {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
    #[doc = "State of the object.\n"]
    pub fn nested_info(mut self) -> PushResourceInfo<Self> {
        let header_offset = push_nested_header(self.as_vec_mut(), 2u16);
        PushResourceInfo {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
    #[doc = "Statistics of the object.\n"]
    pub fn nested_statistics(mut self) -> PushResourceStatistics<Self> {
        let header_offset = push_nested_header(self.as_vec_mut(), 3u16);
        PushResourceStatistics {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
    #[doc = "New name of the resource; present in state-change messages with action\nrename.\n"]
    pub fn push_new_name(mut self, value: &CStr) -> Self {
        push_header(
            self.as_vec_mut(),
            4u16,
            value.to_bytes_with_nul().len() as u16,
        );
        self.as_vec_mut().extend(value.to_bytes_with_nul());
        self
    }
    #[doc = "New name of the resource; present in state-change messages with action\nrename.\n"]
    pub fn push_new_name_bytes(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 4u16, (value.len() + 1) as u16);
        self.as_vec_mut().extend(value);
        self.as_vec_mut().push(0);
        self
    }
    #[doc = "Resource-wide options in effect.\n"]
    pub fn nested_resource_opts(mut self) -> PushResourceOpts<Self> {
        let header_offset = push_nested_header(self.as_vec_mut(), 5u16);
        PushResourceOpts {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
}
impl<Prev: Pusher> Drop for PushResource<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushDevice<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushDevice<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushDevice<Prev> {
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
    #[doc = "Identity of the object.\n"]
    pub fn nested_context(mut self) -> PushContext<Self> {
        let header_offset = push_nested_header(self.as_vec_mut(), 1u16);
        PushContext {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
    #[doc = "State of the object.\n"]
    pub fn nested_info(mut self) -> PushDeviceInfo<Self> {
        let header_offset = push_nested_header(self.as_vec_mut(), 2u16);
        PushDeviceInfo {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
    #[doc = "Statistics of the object.\n"]
    pub fn nested_statistics(mut self) -> PushDeviceStatistics<Self> {
        let header_offset = push_nested_header(self.as_vec_mut(), 3u16);
        PushDeviceStatistics {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
    #[doc = "Backing disk options in effect; absent while the device is diskless.\n"]
    pub fn nested_disk_conf(mut self) -> PushDiskConf<Self> {
        let header_offset = push_nested_header(self.as_vec_mut(), 4u16);
        PushDiskConf {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
    #[doc = "Device options in effect.\n"]
    pub fn nested_device_conf(mut self) -> PushDeviceConf<Self> {
        let header_offset = push_nested_header(self.as_vec_mut(), 5u16);
        PushDeviceConf {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
}
impl<Prev: Pusher> Drop for PushDevice<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushConnection<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushConnection<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushConnection<Prev> {
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
    #[doc = "Identity of the object.\n"]
    pub fn nested_context(mut self) -> PushContext<Self> {
        let header_offset = push_nested_header(self.as_vec_mut(), 1u16);
        PushContext {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
    #[doc = "State of the object.\n"]
    pub fn nested_info(mut self) -> PushConnectionInfo<Self> {
        let header_offset = push_nested_header(self.as_vec_mut(), 2u16);
        PushConnectionInfo {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
    #[doc = "Statistics of the object.\n"]
    pub fn nested_statistics(mut self) -> PushConnectionStatistics<Self> {
        let header_offset = push_nested_header(self.as_vec_mut(), 3u16);
        PushConnectionStatistics {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
    #[doc = "Connection options in effect; absent when the connection has none.\n"]
    pub fn nested_net_conf(mut self) -> PushNetConf<Self> {
        let header_offset = push_nested_header(self.as_vec_mut(), 4u16);
        PushNetConf {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
    #[doc = "The paths of this connection, one attribute each. Also available on\ntheir own through path-get.\n\nAttribute may repeat multiple times (treat it as array)"]
    pub fn nested_path(mut self) -> PushPath<Self> {
        let header_offset = push_nested_header(self.as_vec_mut(), 5u16);
        PushPath {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
}
impl<Prev: Pusher> Drop for PushConnection<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushPeerDevice<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushPeerDevice<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushPeerDevice<Prev> {
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
    #[doc = "Identity of the object.\n"]
    pub fn nested_context(mut self) -> PushContext<Self> {
        let header_offset = push_nested_header(self.as_vec_mut(), 1u16);
        PushContext {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
    #[doc = "State of the object.\n"]
    pub fn nested_info(mut self) -> PushPeerDeviceInfo<Self> {
        let header_offset = push_nested_header(self.as_vec_mut(), 2u16);
        PushPeerDeviceInfo {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
    #[doc = "Statistics of the object.\n"]
    pub fn nested_statistics(mut self) -> PushPeerDeviceStatistics<Self> {
        let header_offset = push_nested_header(self.as_vec_mut(), 3u16);
        PushPeerDeviceStatistics {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
    #[doc = "Peer device options in effect; absent when the peer device has none.\n"]
    pub fn nested_peer_device_conf(mut self) -> PushPeerDeviceConf<Self> {
        let header_offset = push_nested_header(self.as_vec_mut(), 4u16);
        PushPeerDeviceConf {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
}
impl<Prev: Pusher> Drop for PushPeerDevice<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushPath<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushPath<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushPath<Prev> {
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
    #[doc = "Identity of the object.\n"]
    pub fn nested_context(mut self) -> PushContext<Self> {
        let header_offset = push_nested_header(self.as_vec_mut(), 1u16);
        PushContext {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
    #[doc = "State of the object.\n"]
    pub fn nested_info(mut self) -> PushPathInfo<Self> {
        let header_offset = push_nested_header(self.as_vec_mut(), 2u16);
        PushPathInfo {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
}
impl<Prev: Pusher> Drop for PushPath<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushDrbd2<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushDrbd2<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushDrbd2<Prev> {
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
    #[doc = "Object the message refers to.\n"]
    pub fn nested_context(mut self) -> PushContext<Self> {
        let header_offset = push_nested_header(self.as_vec_mut(), 1u16);
        PushContext {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
    #[doc = "Reset all options not given in this request to their defaults before\napplying it.\n"]
    pub fn push_set_defaults(mut self, value: ()) -> Self {
        push_header(self.as_vec_mut(), 2u16, 0 as u16);
        self
    }
    #[doc = "Outcome of the state change performed by the request.\n\nAssociated type: [`StateResult`] (enum)"]
    pub fn push_state_result(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 3u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Human-readable explanation accompanying state-result.\n"]
    pub fn push_message(mut self, value: &CStr) -> Self {
        push_header(
            self.as_vec_mut(),
            4u16,
            value.to_bytes_with_nul().len() as u16,
        );
        self.as_vec_mut().extend(value.to_bytes_with_nul());
        self
    }
    #[doc = "Human-readable explanation accompanying state-result.\n"]
    pub fn push_message_bytes(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 4u16, (value.len() + 1) as u16);
        self.as_vec_mut().extend(value);
        self.as_vec_mut().push(0);
        self
    }
    #[doc = "Reply of timeout-type-get.\n\nAssociated type: [`TimeoutType`] (enum)"]
    pub fn push_timeout_type(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 5u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Backing disk options for device-attach and disk-set.\n"]
    pub fn nested_disk_conf(mut self) -> PushDiskConf<Self> {
        let header_offset = push_nested_header(self.as_vec_mut(), 6u16);
        PushDiskConf {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
    #[doc = "Resource-wide options for resource-new and resource-set.\n"]
    pub fn nested_resource_opts(mut self) -> PushResourceOpts<Self> {
        let header_offset = push_nested_header(self.as_vec_mut(), 7u16);
        PushResourceOpts {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
    #[doc = "Connection options for connection-new and connection-set.\n"]
    pub fn nested_net_conf(mut self) -> PushNetConf<Self> {
        let header_offset = push_nested_header(self.as_vec_mut(), 8u16);
        PushNetConf {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
    #[doc = "Parameters of resource-primary and resource-secondary.\n"]
    pub fn nested_set_role_parms(mut self) -> PushSetRoleParms<Self> {
        let header_offset = push_nested_header(self.as_vec_mut(), 9u16);
        PushSetRoleParms {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
    #[doc = "Parameters of device-resize.\n"]
    pub fn nested_resize_parms(mut self) -> PushResizeParms<Self> {
        let header_offset = push_nested_header(self.as_vec_mut(), 10u16);
        PushResizeParms {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
    #[doc = "Parameters of peer-device-start-ov.\n"]
    pub fn nested_start_ov_parms(mut self) -> PushStartOvParms<Self> {
        let header_offset = push_nested_header(self.as_vec_mut(), 11u16);
        PushStartOvParms {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
    #[doc = "Parameters of device-new-current-uuid.\n"]
    pub fn nested_new_current_uuid_parms(mut self) -> PushNewCurrentUuidParms<Self> {
        let header_offset = push_nested_header(self.as_vec_mut(), 12u16);
        PushNewCurrentUuidParms {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
    #[doc = "Parameters of connection-disconnect and connection-del.\n"]
    pub fn nested_disconnect_parms(mut self) -> PushDisconnectParms<Self> {
        let header_offset = push_nested_header(self.as_vec_mut(), 13u16);
        PushDisconnectParms {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
    #[doc = "Parameters of device-detach.\n"]
    pub fn nested_detach_parms(mut self) -> PushDetachParms<Self> {
        let header_offset = push_nested_header(self.as_vec_mut(), 14u16);
        PushDetachParms {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
    #[doc = "Device options set when the device is created by device-new.\n"]
    pub fn nested_device_conf(mut self) -> PushDeviceConf<Self> {
        let header_offset = push_nested_header(self.as_vec_mut(), 15u16);
        PushDeviceConf {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
    #[doc = "Identity and outcome of the handler script reported by helper-ntf.\n"]
    pub fn nested_helper(mut self) -> PushHelperInfo<Self> {
        let header_offset = push_nested_header(self.as_vec_mut(), 16u16);
        PushHelperInfo {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
    #[doc = "Parameters of device-invalidate.\n"]
    pub fn nested_invalidate_parms(mut self) -> PushInvalidateParms<Self> {
        let header_offset = push_nested_header(self.as_vec_mut(), 17u16);
        PushInvalidateParms {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
    #[doc = "Peer device options for peer-device-set.\n"]
    pub fn nested_peer_device_conf(mut self) -> PushPeerDeviceConf<Self> {
        let header_offset = push_nested_header(self.as_vec_mut(), 18u16);
        PushPeerDeviceConf {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
    #[doc = "Parameters of connection-connect.\n"]
    pub fn nested_connect_parms(mut self) -> PushConnectParms<Self> {
        let header_offset = push_nested_header(self.as_vec_mut(), 19u16);
        PushConnectParms {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
    #[doc = "Parameters of resource-rename.\n"]
    pub fn nested_rename_parms(mut self) -> PushRenameParms<Self> {
        let header_offset = push_nested_header(self.as_vec_mut(), 20u16);
        PushRenameParms {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
    #[doc = "Parameters of peer-device-invalidate.\n"]
    pub fn nested_invalidate_peer_parms(mut self) -> PushInvalidatePeerParms<Self> {
        let header_offset = push_nested_header(self.as_vec_mut(), 21u16);
        PushInvalidatePeerParms {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
    #[doc = "Parameters of resource-suspend-io.\n"]
    pub fn nested_suspend_io_parms(mut self) -> PushSuspendIoParms<Self> {
        let header_offset = push_nested_header(self.as_vec_mut(), 22u16);
        PushSuspendIoParms {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
    #[doc = "A resource (dump reply).\n"]
    pub fn nested_resource(mut self) -> PushResource<Self> {
        let header_offset = push_nested_header(self.as_vec_mut(), 23u16);
        PushResource {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
    #[doc = "A device (dump reply).\n"]
    pub fn nested_device(mut self) -> PushDevice<Self> {
        let header_offset = push_nested_header(self.as_vec_mut(), 24u16);
        PushDevice {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
    #[doc = "A connection (dump reply).\n"]
    pub fn nested_connection(mut self) -> PushConnection<Self> {
        let header_offset = push_nested_header(self.as_vec_mut(), 25u16);
        PushConnection {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
    #[doc = "A peer device (dump reply).\n"]
    pub fn nested_peer_device(mut self) -> PushPeerDevice<Self> {
        let header_offset = push_nested_header(self.as_vec_mut(), 26u16);
        PushPeerDevice {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
    #[doc = "A path (dump reply).\n"]
    pub fn nested_path(mut self) -> PushPath<Self> {
        let header_offset = push_nested_header(self.as_vec_mut(), 27u16);
        PushPath {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
    #[doc = "Parameters of device-cbt-start.\n"]
    pub fn nested_cbt_start_parms(mut self) -> PushCbtStartParms<Self> {
        let header_offset = push_nested_header(self.as_vec_mut(), 28u16);
        PushCbtStartParms {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
    #[doc = "Reply of device-cbt-status.\n"]
    pub fn nested_cbt_info(mut self) -> PushCbtInfo<Self> {
        let header_offset = push_nested_header(self.as_vec_mut(), 29u16);
        PushCbtInfo {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
    #[doc = "Parameters of device-attach.\n"]
    pub fn nested_attach_parms(mut self) -> PushAttachParms<Self> {
        let header_offset = push_nested_header(self.as_vec_mut(), 30u16);
        PushAttachParms {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
}
impl<Prev: Pusher> Drop for PushDrbd2<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushStateChange<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushStateChange<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushStateChange<Prev> {
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
    #[doc = "Whether the object exists, was created, changed or destroyed.\n\nAssociated type: [`StateChangeAction`] (enum)"]
    pub fn push_action(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 1u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Further messages belong to the same state change.\n"]
    pub fn push_more(mut self, value: ()) -> Self {
        push_header(self.as_vec_mut(), 2u16, 0 as u16);
        self
    }
    #[doc = "Present when the message describes a resource.\n"]
    pub fn nested_resource(mut self) -> PushResource<Self> {
        let header_offset = push_nested_header(self.as_vec_mut(), 3u16);
        PushResource {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
    #[doc = "Present when the message describes a device.\n"]
    pub fn nested_device(mut self) -> PushDevice<Self> {
        let header_offset = push_nested_header(self.as_vec_mut(), 4u16);
        PushDevice {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
    #[doc = "Present when the message describes a connection.\n"]
    pub fn nested_connection(mut self) -> PushConnection<Self> {
        let header_offset = push_nested_header(self.as_vec_mut(), 5u16);
        PushConnection {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
    #[doc = "Present when the message describes a peer device.\n"]
    pub fn nested_peer_device(mut self) -> PushPeerDevice<Self> {
        let header_offset = push_nested_header(self.as_vec_mut(), 6u16);
        PushPeerDevice {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
    #[doc = "Present when the message describes a path.\n"]
    pub fn nested_path(mut self) -> PushPath<Self> {
        let header_offset = push_nested_header(self.as_vec_mut(), 7u16);
        PushPath {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
}
impl<Prev: Pusher> Drop for PushStateChange<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
#[doc = "Notify attributes:\n- [`.get_action()`](IterableStateChange::get_action)\n- [`.get_more()`](IterableStateChange::get_more)\n- [`.get_resource()`](IterableStateChange::get_resource)\n- [`.get_device()`](IterableStateChange::get_device)\n- [`.get_connection()`](IterableStateChange::get_connection)\n- [`.get_peer_device()`](IterableStateChange::get_peer_device)\n- [`.get_path()`](IterableStateChange::get_path)\n"]
#[derive(Debug)]
pub struct OpStateChangeNotif;
impl OpStateChangeNotif {
    pub const CMD: u8 = 39u8;
    pub fn decode_notif<'a>(buf: &'a [u8]) -> IterableStateChange<'a> {
        let (_header, attrs) = buf.split_at(buf.len().min(BuiltinNfgenmsg::len()));
        IterableStateChange::with_loc(attrs, buf.as_ptr() as usize)
    }
}
#[doc = "Notify attributes:\n- [`.get_context()`](IterableDrbd2::get_context)\n- [`.get_helper()`](IterableDrbd2::get_helper)\n"]
#[derive(Debug)]
pub struct OpHelperNotif;
impl OpHelperNotif {
    pub const CMD: u8 = 40u8;
    pub fn decode_notif<'a>(buf: &'a [u8]) -> IterableDrbd2<'a> {
        let (_header, attrs) = buf.split_at(buf.len().min(BuiltinNfgenmsg::len()));
        IterableDrbd2::with_loc(attrs, buf.as_ptr() as usize)
    }
}
pub struct NotifGroup;
impl NotifGroup {
    #[doc = "Notifications:\n- [`OpStateChangeNotif`]\n- [`OpHelperNotif`]\n"]
    pub const EVENTS: &str = "events";
    #[doc = "Notifications:\n- [`OpStateChangeNotif`]\n- [`OpHelperNotif`]\n"]
    pub const EVENTS_CSTR: &CStr = c"events";
}
#[doc = "Create a new DRBD resource.\n\nFlags: admin-perm\n\nRequest attributes:\n- [.nested_context()](PushDrbd2::nested_context)\n- [.nested_resource_opts()](PushDrbd2::nested_resource_opts)\n\nReply attributes:\n- [.get_state_result()](IterableDrbd2::get_state_result)\n- [.get_message()](IterableDrbd2::get_message)\n\n"]
#[derive(Debug)]
pub struct OpResourceNewDo<'r> {
    request: Request<'r>,
}
impl<'r> OpResourceNewDo<'r> {
    pub fn new(mut request: Request<'r>) -> Self {
        Self::write_header(request.buf_mut());
        Self { request: request }
    }
    pub fn encode_request<'buf>(buf: &'buf mut Vec<u8>) -> PushDrbd2<&'buf mut Vec<u8>> {
        Self::write_header(buf);
        PushDrbd2::new(buf)
    }
    pub fn encode(&mut self) -> PushDrbd2<&mut Vec<u8>> {
        PushDrbd2::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushDrbd2<RequestBuf<'r>> {
        PushDrbd2::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> IterableDrbd2<'a> {
        let (_header, attrs) = buf.split_at(buf.len().min(BuiltinNfgenmsg::len()));
        IterableDrbd2::with_loc(attrs, buf.as_ptr() as usize)
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
impl NetlinkRequest for OpResourceNewDo<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Generic("drbd2".as_bytes())
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = IterableDrbd2<'buf>;
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
#[doc = "Delete a DRBD resource.\n\nFlags: admin-perm\n\nRequest attributes:\n- [.nested_context()](PushDrbd2::nested_context)\n\nReply attributes:\n- [.get_state_result()](IterableDrbd2::get_state_result)\n- [.get_message()](IterableDrbd2::get_message)\n\n"]
#[derive(Debug)]
pub struct OpResourceDelDo<'r> {
    request: Request<'r>,
}
impl<'r> OpResourceDelDo<'r> {
    pub fn new(mut request: Request<'r>) -> Self {
        Self::write_header(request.buf_mut());
        Self { request: request }
    }
    pub fn encode_request<'buf>(buf: &'buf mut Vec<u8>) -> PushDrbd2<&'buf mut Vec<u8>> {
        Self::write_header(buf);
        PushDrbd2::new(buf)
    }
    pub fn encode(&mut self) -> PushDrbd2<&mut Vec<u8>> {
        PushDrbd2::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushDrbd2<RequestBuf<'r>> {
        PushDrbd2::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> IterableDrbd2<'a> {
        let (_header, attrs) = buf.split_at(buf.len().min(BuiltinNfgenmsg::len()));
        IterableDrbd2::with_loc(attrs, buf.as_ptr() as usize)
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
impl NetlinkRequest for OpResourceDelDo<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Generic("drbd2".as_bytes())
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = IterableDrbd2<'buf>;
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
#[doc = "Change resource options.\n\nFlags: admin-perm\n\nRequest attributes:\n- [.nested_context()](PushDrbd2::nested_context)\n- [.push_set_defaults()](PushDrbd2::push_set_defaults)\n- [.nested_resource_opts()](PushDrbd2::nested_resource_opts)\n\nReply attributes:\n- [.get_state_result()](IterableDrbd2::get_state_result)\n- [.get_message()](IterableDrbd2::get_message)\n\n"]
#[derive(Debug)]
pub struct OpResourceSetDo<'r> {
    request: Request<'r>,
}
impl<'r> OpResourceSetDo<'r> {
    pub fn new(mut request: Request<'r>) -> Self {
        Self::write_header(request.buf_mut());
        Self { request: request }
    }
    pub fn encode_request<'buf>(buf: &'buf mut Vec<u8>) -> PushDrbd2<&'buf mut Vec<u8>> {
        Self::write_header(buf);
        PushDrbd2::new(buf)
    }
    pub fn encode(&mut self) -> PushDrbd2<&mut Vec<u8>> {
        PushDrbd2::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushDrbd2<RequestBuf<'r>> {
        PushDrbd2::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> IterableDrbd2<'a> {
        let (_header, attrs) = buf.split_at(buf.len().min(BuiltinNfgenmsg::len()));
        IterableDrbd2::with_loc(attrs, buf.as_ptr() as usize)
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
impl NetlinkRequest for OpResourceSetDo<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Generic("drbd2".as_bytes())
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = IterableDrbd2<'buf>;
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
#[doc = "Rename a DRBD resource.\n\nFlags: admin-perm\n\nRequest attributes:\n- [.nested_context()](PushDrbd2::nested_context)\n- [.nested_rename_parms()](PushDrbd2::nested_rename_parms)\n\nReply attributes:\n- [.get_state_result()](IterableDrbd2::get_state_result)\n- [.get_message()](IterableDrbd2::get_message)\n\n"]
#[derive(Debug)]
pub struct OpResourceRenameDo<'r> {
    request: Request<'r>,
}
impl<'r> OpResourceRenameDo<'r> {
    pub fn new(mut request: Request<'r>) -> Self {
        Self::write_header(request.buf_mut());
        Self { request: request }
    }
    pub fn encode_request<'buf>(buf: &'buf mut Vec<u8>) -> PushDrbd2<&'buf mut Vec<u8>> {
        Self::write_header(buf);
        PushDrbd2::new(buf)
    }
    pub fn encode(&mut self) -> PushDrbd2<&mut Vec<u8>> {
        PushDrbd2::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushDrbd2<RequestBuf<'r>> {
        PushDrbd2::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> IterableDrbd2<'a> {
        let (_header, attrs) = buf.split_at(buf.len().min(BuiltinNfgenmsg::len()));
        IterableDrbd2::with_loc(attrs, buf.as_ptr() as usize)
    }
    fn write_header<Prev: Pusher>(prev: &mut Prev) {
        let mut header = BuiltinNfgenmsg::new();
        header.cmd = 4u8;
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
impl NetlinkRequest for OpResourceRenameDo<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Generic("drbd2".as_bytes())
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = IterableDrbd2<'buf>;
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
#[doc = "Shut down a DRBD resource.\n\nFlags: admin-perm\n\nRequest attributes:\n- [.nested_context()](PushDrbd2::nested_context)\n\nReply attributes:\n- [.get_state_result()](IterableDrbd2::get_state_result)\n- [.get_message()](IterableDrbd2::get_message)\n\n"]
#[derive(Debug)]
pub struct OpResourceDownDo<'r> {
    request: Request<'r>,
}
impl<'r> OpResourceDownDo<'r> {
    pub fn new(mut request: Request<'r>) -> Self {
        Self::write_header(request.buf_mut());
        Self { request: request }
    }
    pub fn encode_request<'buf>(buf: &'buf mut Vec<u8>) -> PushDrbd2<&'buf mut Vec<u8>> {
        Self::write_header(buf);
        PushDrbd2::new(buf)
    }
    pub fn encode(&mut self) -> PushDrbd2<&mut Vec<u8>> {
        PushDrbd2::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushDrbd2<RequestBuf<'r>> {
        PushDrbd2::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> IterableDrbd2<'a> {
        let (_header, attrs) = buf.split_at(buf.len().min(BuiltinNfgenmsg::len()));
        IterableDrbd2::with_loc(attrs, buf.as_ptr() as usize)
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
impl NetlinkRequest for OpResourceDownDo<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Generic("drbd2".as_bytes())
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = IterableDrbd2<'buf>;
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
#[doc = "Promote the resource to primary role.\n\nFlags: admin-perm\n\nRequest attributes:\n- [.nested_context()](PushDrbd2::nested_context)\n- [.nested_set_role_parms()](PushDrbd2::nested_set_role_parms)\n\nReply attributes:\n- [.get_state_result()](IterableDrbd2::get_state_result)\n- [.get_message()](IterableDrbd2::get_message)\n\n"]
#[derive(Debug)]
pub struct OpResourcePrimaryDo<'r> {
    request: Request<'r>,
}
impl<'r> OpResourcePrimaryDo<'r> {
    pub fn new(mut request: Request<'r>) -> Self {
        Self::write_header(request.buf_mut());
        Self { request: request }
    }
    pub fn encode_request<'buf>(buf: &'buf mut Vec<u8>) -> PushDrbd2<&'buf mut Vec<u8>> {
        Self::write_header(buf);
        PushDrbd2::new(buf)
    }
    pub fn encode(&mut self) -> PushDrbd2<&mut Vec<u8>> {
        PushDrbd2::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushDrbd2<RequestBuf<'r>> {
        PushDrbd2::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> IterableDrbd2<'a> {
        let (_header, attrs) = buf.split_at(buf.len().min(BuiltinNfgenmsg::len()));
        IterableDrbd2::with_loc(attrs, buf.as_ptr() as usize)
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
impl NetlinkRequest for OpResourcePrimaryDo<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Generic("drbd2".as_bytes())
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = IterableDrbd2<'buf>;
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
#[doc = "Demote the resource to secondary role.\n\nFlags: admin-perm\n\nRequest attributes:\n- [.nested_context()](PushDrbd2::nested_context)\n- [.nested_set_role_parms()](PushDrbd2::nested_set_role_parms)\n\nReply attributes:\n- [.get_state_result()](IterableDrbd2::get_state_result)\n- [.get_message()](IterableDrbd2::get_message)\n\n"]
#[derive(Debug)]
pub struct OpResourceSecondaryDo<'r> {
    request: Request<'r>,
}
impl<'r> OpResourceSecondaryDo<'r> {
    pub fn new(mut request: Request<'r>) -> Self {
        Self::write_header(request.buf_mut());
        Self { request: request }
    }
    pub fn encode_request<'buf>(buf: &'buf mut Vec<u8>) -> PushDrbd2<&'buf mut Vec<u8>> {
        Self::write_header(buf);
        PushDrbd2::new(buf)
    }
    pub fn encode(&mut self) -> PushDrbd2<&mut Vec<u8>> {
        PushDrbd2::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushDrbd2<RequestBuf<'r>> {
        PushDrbd2::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> IterableDrbd2<'a> {
        let (_header, attrs) = buf.split_at(buf.len().min(BuiltinNfgenmsg::len()));
        IterableDrbd2::with_loc(attrs, buf.as_ptr() as usize)
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
impl NetlinkRequest for OpResourceSecondaryDo<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Generic("drbd2".as_bytes())
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = IterableDrbd2<'buf>;
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
#[doc = "Suspend I/O on a resource.\n\nFlags: admin-perm\n\nRequest attributes:\n- [.nested_context()](PushDrbd2::nested_context)\n- [.nested_suspend_io_parms()](PushDrbd2::nested_suspend_io_parms)\n\nReply attributes:\n- [.get_state_result()](IterableDrbd2::get_state_result)\n- [.get_message()](IterableDrbd2::get_message)\n\n"]
#[derive(Debug)]
pub struct OpResourceSuspendIoDo<'r> {
    request: Request<'r>,
}
impl<'r> OpResourceSuspendIoDo<'r> {
    pub fn new(mut request: Request<'r>) -> Self {
        Self::write_header(request.buf_mut());
        Self { request: request }
    }
    pub fn encode_request<'buf>(buf: &'buf mut Vec<u8>) -> PushDrbd2<&'buf mut Vec<u8>> {
        Self::write_header(buf);
        PushDrbd2::new(buf)
    }
    pub fn encode(&mut self) -> PushDrbd2<&mut Vec<u8>> {
        PushDrbd2::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushDrbd2<RequestBuf<'r>> {
        PushDrbd2::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> IterableDrbd2<'a> {
        let (_header, attrs) = buf.split_at(buf.len().min(BuiltinNfgenmsg::len()));
        IterableDrbd2::with_loc(attrs, buf.as_ptr() as usize)
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
impl NetlinkRequest for OpResourceSuspendIoDo<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Generic("drbd2".as_bytes())
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = IterableDrbd2<'buf>;
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
#[doc = "Resume I/O on a resource.\n\nFlags: admin-perm\n\nRequest attributes:\n- [.nested_context()](PushDrbd2::nested_context)\n\nReply attributes:\n- [.get_state_result()](IterableDrbd2::get_state_result)\n- [.get_message()](IterableDrbd2::get_message)\n\n"]
#[derive(Debug)]
pub struct OpResourceResumeIoDo<'r> {
    request: Request<'r>,
}
impl<'r> OpResourceResumeIoDo<'r> {
    pub fn new(mut request: Request<'r>) -> Self {
        Self::write_header(request.buf_mut());
        Self { request: request }
    }
    pub fn encode_request<'buf>(buf: &'buf mut Vec<u8>) -> PushDrbd2<&'buf mut Vec<u8>> {
        Self::write_header(buf);
        PushDrbd2::new(buf)
    }
    pub fn encode(&mut self) -> PushDrbd2<&mut Vec<u8>> {
        PushDrbd2::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushDrbd2<RequestBuf<'r>> {
        PushDrbd2::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> IterableDrbd2<'a> {
        let (_header, attrs) = buf.split_at(buf.len().min(BuiltinNfgenmsg::len()));
        IterableDrbd2::with_loc(attrs, buf.as_ptr() as usize)
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
impl NetlinkRequest for OpResourceResumeIoDo<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Generic("drbd2".as_bytes())
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = IterableDrbd2<'buf>;
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
#[doc = "Add a DRBD device as a volume to a resource.\n\nFlags: admin-perm\n\nRequest attributes:\n- [.nested_context()](PushDrbd2::nested_context)\n- [.nested_device_conf()](PushDrbd2::nested_device_conf)\n\nReply attributes:\n- [.get_state_result()](IterableDrbd2::get_state_result)\n- [.get_message()](IterableDrbd2::get_message)\n\n"]
#[derive(Debug)]
pub struct OpDeviceNewDo<'r> {
    request: Request<'r>,
}
impl<'r> OpDeviceNewDo<'r> {
    pub fn new(mut request: Request<'r>) -> Self {
        Self::write_header(request.buf_mut());
        Self { request: request }
    }
    pub fn encode_request<'buf>(buf: &'buf mut Vec<u8>) -> PushDrbd2<&'buf mut Vec<u8>> {
        Self::write_header(buf);
        PushDrbd2::new(buf)
    }
    pub fn encode(&mut self) -> PushDrbd2<&mut Vec<u8>> {
        PushDrbd2::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushDrbd2<RequestBuf<'r>> {
        PushDrbd2::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> IterableDrbd2<'a> {
        let (_header, attrs) = buf.split_at(buf.len().min(BuiltinNfgenmsg::len()));
        IterableDrbd2::with_loc(attrs, buf.as_ptr() as usize)
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
impl NetlinkRequest for OpDeviceNewDo<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Generic("drbd2".as_bytes())
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = IterableDrbd2<'buf>;
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
#[doc = "Delete a DRBD device.\n\nFlags: admin-perm\n\nRequest attributes:\n- [.nested_context()](PushDrbd2::nested_context)\n\nReply attributes:\n- [.get_state_result()](IterableDrbd2::get_state_result)\n- [.get_message()](IterableDrbd2::get_message)\n\n"]
#[derive(Debug)]
pub struct OpDeviceDelDo<'r> {
    request: Request<'r>,
}
impl<'r> OpDeviceDelDo<'r> {
    pub fn new(mut request: Request<'r>) -> Self {
        Self::write_header(request.buf_mut());
        Self { request: request }
    }
    pub fn encode_request<'buf>(buf: &'buf mut Vec<u8>) -> PushDrbd2<&'buf mut Vec<u8>> {
        Self::write_header(buf);
        PushDrbd2::new(buf)
    }
    pub fn encode(&mut self) -> PushDrbd2<&mut Vec<u8>> {
        PushDrbd2::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushDrbd2<RequestBuf<'r>> {
        PushDrbd2::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> IterableDrbd2<'a> {
        let (_header, attrs) = buf.split_at(buf.len().min(BuiltinNfgenmsg::len()));
        IterableDrbd2::with_loc(attrs, buf.as_ptr() as usize)
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
impl NetlinkRequest for OpDeviceDelDo<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Generic("drbd2".as_bytes())
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = IterableDrbd2<'buf>;
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
#[doc = "Attach a local backing device to a DRBD device.\n\nFlags: admin-perm\n\nRequest attributes:\n- [.nested_context()](PushDrbd2::nested_context)\n- [.nested_disk_conf()](PushDrbd2::nested_disk_conf)\n- [.nested_attach_parms()](PushDrbd2::nested_attach_parms)\n\nReply attributes:\n- [.get_state_result()](IterableDrbd2::get_state_result)\n- [.get_message()](IterableDrbd2::get_message)\n\n"]
#[derive(Debug)]
pub struct OpDeviceAttachDo<'r> {
    request: Request<'r>,
}
impl<'r> OpDeviceAttachDo<'r> {
    pub fn new(mut request: Request<'r>) -> Self {
        Self::write_header(request.buf_mut());
        Self { request: request }
    }
    pub fn encode_request<'buf>(buf: &'buf mut Vec<u8>) -> PushDrbd2<&'buf mut Vec<u8>> {
        Self::write_header(buf);
        PushDrbd2::new(buf)
    }
    pub fn encode(&mut self) -> PushDrbd2<&mut Vec<u8>> {
        PushDrbd2::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushDrbd2<RequestBuf<'r>> {
        PushDrbd2::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> IterableDrbd2<'a> {
        let (_header, attrs) = buf.split_at(buf.len().min(BuiltinNfgenmsg::len()));
        IterableDrbd2::with_loc(attrs, buf.as_ptr() as usize)
    }
    fn write_header<Prev: Pusher>(prev: &mut Prev) {
        let mut header = BuiltinNfgenmsg::new();
        header.cmd = 12u8;
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
impl NetlinkRequest for OpDeviceAttachDo<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Generic("drbd2".as_bytes())
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = IterableDrbd2<'buf>;
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
#[doc = "Detach the local backing device from a DRBD device.\n\nFlags: admin-perm\n\nRequest attributes:\n- [.nested_context()](PushDrbd2::nested_context)\n- [.nested_detach_parms()](PushDrbd2::nested_detach_parms)\n\nReply attributes:\n- [.get_state_result()](IterableDrbd2::get_state_result)\n- [.get_message()](IterableDrbd2::get_message)\n\n"]
#[derive(Debug)]
pub struct OpDeviceDetachDo<'r> {
    request: Request<'r>,
}
impl<'r> OpDeviceDetachDo<'r> {
    pub fn new(mut request: Request<'r>) -> Self {
        Self::write_header(request.buf_mut());
        Self { request: request }
    }
    pub fn encode_request<'buf>(buf: &'buf mut Vec<u8>) -> PushDrbd2<&'buf mut Vec<u8>> {
        Self::write_header(buf);
        PushDrbd2::new(buf)
    }
    pub fn encode(&mut self) -> PushDrbd2<&mut Vec<u8>> {
        PushDrbd2::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushDrbd2<RequestBuf<'r>> {
        PushDrbd2::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> IterableDrbd2<'a> {
        let (_header, attrs) = buf.split_at(buf.len().min(BuiltinNfgenmsg::len()));
        IterableDrbd2::with_loc(attrs, buf.as_ptr() as usize)
    }
    fn write_header<Prev: Pusher>(prev: &mut Prev) {
        let mut header = BuiltinNfgenmsg::new();
        header.cmd = 13u8;
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
impl NetlinkRequest for OpDeviceDetachDo<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Generic("drbd2".as_bytes())
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = IterableDrbd2<'buf>;
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
#[doc = "Change disk options. Applies to the attached backing disk; see\ndevice-attach.\n\nFlags: admin-perm\n\nRequest attributes:\n- [.nested_context()](PushDrbd2::nested_context)\n- [.push_set_defaults()](PushDrbd2::push_set_defaults)\n- [.nested_disk_conf()](PushDrbd2::nested_disk_conf)\n\nReply attributes:\n- [.get_state_result()](IterableDrbd2::get_state_result)\n- [.get_message()](IterableDrbd2::get_message)\n\n"]
#[derive(Debug)]
pub struct OpDiskSetDo<'r> {
    request: Request<'r>,
}
impl<'r> OpDiskSetDo<'r> {
    pub fn new(mut request: Request<'r>) -> Self {
        Self::write_header(request.buf_mut());
        Self { request: request }
    }
    pub fn encode_request<'buf>(buf: &'buf mut Vec<u8>) -> PushDrbd2<&'buf mut Vec<u8>> {
        Self::write_header(buf);
        PushDrbd2::new(buf)
    }
    pub fn encode(&mut self) -> PushDrbd2<&mut Vec<u8>> {
        PushDrbd2::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushDrbd2<RequestBuf<'r>> {
        PushDrbd2::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> IterableDrbd2<'a> {
        let (_header, attrs) = buf.split_at(buf.len().min(BuiltinNfgenmsg::len()));
        IterableDrbd2::with_loc(attrs, buf.as_ptr() as usize)
    }
    fn write_header<Prev: Pusher>(prev: &mut Prev) {
        let mut header = BuiltinNfgenmsg::new();
        header.cmd = 14u8;
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
impl NetlinkRequest for OpDiskSetDo<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Generic("drbd2".as_bytes())
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = IterableDrbd2<'buf>;
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
#[doc = "Resize the DRBD device.\n\nFlags: admin-perm\n\nRequest attributes:\n- [.nested_context()](PushDrbd2::nested_context)\n- [.nested_resize_parms()](PushDrbd2::nested_resize_parms)\n\nReply attributes:\n- [.get_state_result()](IterableDrbd2::get_state_result)\n- [.get_message()](IterableDrbd2::get_message)\n\n"]
#[derive(Debug)]
pub struct OpDeviceResizeDo<'r> {
    request: Request<'r>,
}
impl<'r> OpDeviceResizeDo<'r> {
    pub fn new(mut request: Request<'r>) -> Self {
        Self::write_header(request.buf_mut());
        Self { request: request }
    }
    pub fn encode_request<'buf>(buf: &'buf mut Vec<u8>) -> PushDrbd2<&'buf mut Vec<u8>> {
        Self::write_header(buf);
        PushDrbd2::new(buf)
    }
    pub fn encode(&mut self) -> PushDrbd2<&mut Vec<u8>> {
        PushDrbd2::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushDrbd2<RequestBuf<'r>> {
        PushDrbd2::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> IterableDrbd2<'a> {
        let (_header, attrs) = buf.split_at(buf.len().min(BuiltinNfgenmsg::len()));
        IterableDrbd2::with_loc(attrs, buf.as_ptr() as usize)
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
impl NetlinkRequest for OpDeviceResizeDo<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Generic("drbd2".as_bytes())
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = IterableDrbd2<'buf>;
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
#[doc = "Mark the local data as outdated.\n\nFlags: admin-perm\n\nRequest attributes:\n- [.nested_context()](PushDrbd2::nested_context)\n\nReply attributes:\n- [.get_state_result()](IterableDrbd2::get_state_result)\n- [.get_message()](IterableDrbd2::get_message)\n\n"]
#[derive(Debug)]
pub struct OpDeviceOutdateDo<'r> {
    request: Request<'r>,
}
impl<'r> OpDeviceOutdateDo<'r> {
    pub fn new(mut request: Request<'r>) -> Self {
        Self::write_header(request.buf_mut());
        Self { request: request }
    }
    pub fn encode_request<'buf>(buf: &'buf mut Vec<u8>) -> PushDrbd2<&'buf mut Vec<u8>> {
        Self::write_header(buf);
        PushDrbd2::new(buf)
    }
    pub fn encode(&mut self) -> PushDrbd2<&mut Vec<u8>> {
        PushDrbd2::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushDrbd2<RequestBuf<'r>> {
        PushDrbd2::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> IterableDrbd2<'a> {
        let (_header, attrs) = buf.split_at(buf.len().min(BuiltinNfgenmsg::len()));
        IterableDrbd2::with_loc(attrs, buf.as_ptr() as usize)
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
impl NetlinkRequest for OpDeviceOutdateDo<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Generic("drbd2".as_bytes())
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = IterableDrbd2<'buf>;
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
#[doc = "Invalidate the local data of a DRBD device.\n\nFlags: admin-perm\n\nRequest attributes:\n- [.nested_context()](PushDrbd2::nested_context)\n- [.nested_invalidate_parms()](PushDrbd2::nested_invalidate_parms)\n\nReply attributes:\n- [.get_state_result()](IterableDrbd2::get_state_result)\n- [.get_message()](IterableDrbd2::get_message)\n\n"]
#[derive(Debug)]
pub struct OpDeviceInvalidateDo<'r> {
    request: Request<'r>,
}
impl<'r> OpDeviceInvalidateDo<'r> {
    pub fn new(mut request: Request<'r>) -> Self {
        Self::write_header(request.buf_mut());
        Self { request: request }
    }
    pub fn encode_request<'buf>(buf: &'buf mut Vec<u8>) -> PushDrbd2<&'buf mut Vec<u8>> {
        Self::write_header(buf);
        PushDrbd2::new(buf)
    }
    pub fn encode(&mut self) -> PushDrbd2<&mut Vec<u8>> {
        PushDrbd2::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushDrbd2<RequestBuf<'r>> {
        PushDrbd2::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> IterableDrbd2<'a> {
        let (_header, attrs) = buf.split_at(buf.len().min(BuiltinNfgenmsg::len()));
        IterableDrbd2::with_loc(attrs, buf.as_ptr() as usize)
    }
    fn write_header<Prev: Pusher>(prev: &mut Prev) {
        let mut header = BuiltinNfgenmsg::new();
        header.cmd = 17u8;
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
impl NetlinkRequest for OpDeviceInvalidateDo<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Generic("drbd2".as_bytes())
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = IterableDrbd2<'buf>;
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
#[doc = "Generate a new current UUID.\n\nFlags: admin-perm\n\nRequest attributes:\n- [.nested_context()](PushDrbd2::nested_context)\n- [.nested_new_current_uuid_parms()](PushDrbd2::nested_new_current_uuid_parms)\n\nReply attributes:\n- [.get_state_result()](IterableDrbd2::get_state_result)\n- [.get_message()](IterableDrbd2::get_message)\n\n"]
#[derive(Debug)]
pub struct OpDeviceNewCurrentUuidDo<'r> {
    request: Request<'r>,
}
impl<'r> OpDeviceNewCurrentUuidDo<'r> {
    pub fn new(mut request: Request<'r>) -> Self {
        Self::write_header(request.buf_mut());
        Self { request: request }
    }
    pub fn encode_request<'buf>(buf: &'buf mut Vec<u8>) -> PushDrbd2<&'buf mut Vec<u8>> {
        Self::write_header(buf);
        PushDrbd2::new(buf)
    }
    pub fn encode(&mut self) -> PushDrbd2<&mut Vec<u8>> {
        PushDrbd2::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushDrbd2<RequestBuf<'r>> {
        PushDrbd2::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> IterableDrbd2<'a> {
        let (_header, attrs) = buf.split_at(buf.len().min(BuiltinNfgenmsg::len()));
        IterableDrbd2::with_loc(attrs, buf.as_ptr() as usize)
    }
    fn write_header<Prev: Pusher>(prev: &mut Prev) {
        let mut header = BuiltinNfgenmsg::new();
        header.cmd = 18u8;
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
impl NetlinkRequest for OpDeviceNewCurrentUuidDo<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Generic("drbd2".as_bytes())
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = IterableDrbd2<'buf>;
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
#[doc = "Add a new peer to a resource.\n\nFlags: admin-perm\n\nRequest attributes:\n- [.nested_context()](PushDrbd2::nested_context)\n- [.nested_net_conf()](PushDrbd2::nested_net_conf)\n\nReply attributes:\n- [.get_state_result()](IterableDrbd2::get_state_result)\n- [.get_message()](IterableDrbd2::get_message)\n\n"]
#[derive(Debug)]
pub struct OpConnectionNewDo<'r> {
    request: Request<'r>,
}
impl<'r> OpConnectionNewDo<'r> {
    pub fn new(mut request: Request<'r>) -> Self {
        Self::write_header(request.buf_mut());
        Self { request: request }
    }
    pub fn encode_request<'buf>(buf: &'buf mut Vec<u8>) -> PushDrbd2<&'buf mut Vec<u8>> {
        Self::write_header(buf);
        PushDrbd2::new(buf)
    }
    pub fn encode(&mut self) -> PushDrbd2<&mut Vec<u8>> {
        PushDrbd2::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushDrbd2<RequestBuf<'r>> {
        PushDrbd2::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> IterableDrbd2<'a> {
        let (_header, attrs) = buf.split_at(buf.len().min(BuiltinNfgenmsg::len()));
        IterableDrbd2::with_loc(attrs, buf.as_ptr() as usize)
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
impl NetlinkRequest for OpConnectionNewDo<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Generic("drbd2".as_bytes())
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = IterableDrbd2<'buf>;
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
#[doc = "Delete a peer from a resource.\n\nFlags: admin-perm\n\nRequest attributes:\n- [.nested_context()](PushDrbd2::nested_context)\n- [.nested_disconnect_parms()](PushDrbd2::nested_disconnect_parms)\n\nReply attributes:\n- [.get_state_result()](IterableDrbd2::get_state_result)\n- [.get_message()](IterableDrbd2::get_message)\n\n"]
#[derive(Debug)]
pub struct OpConnectionDelDo<'r> {
    request: Request<'r>,
}
impl<'r> OpConnectionDelDo<'r> {
    pub fn new(mut request: Request<'r>) -> Self {
        Self::write_header(request.buf_mut());
        Self { request: request }
    }
    pub fn encode_request<'buf>(buf: &'buf mut Vec<u8>) -> PushDrbd2<&'buf mut Vec<u8>> {
        Self::write_header(buf);
        PushDrbd2::new(buf)
    }
    pub fn encode(&mut self) -> PushDrbd2<&mut Vec<u8>> {
        PushDrbd2::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushDrbd2<RequestBuf<'r>> {
        PushDrbd2::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> IterableDrbd2<'a> {
        let (_header, attrs) = buf.split_at(buf.len().min(BuiltinNfgenmsg::len()));
        IterableDrbd2::with_loc(attrs, buf.as_ptr() as usize)
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
impl NetlinkRequest for OpConnectionDelDo<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Generic("drbd2".as_bytes())
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = IterableDrbd2<'buf>;
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
#[doc = "Connect to a peer.\n\nFlags: admin-perm\n\nRequest attributes:\n- [.nested_context()](PushDrbd2::nested_context)\n- [.nested_connect_parms()](PushDrbd2::nested_connect_parms)\n\nReply attributes:\n- [.get_state_result()](IterableDrbd2::get_state_result)\n- [.get_message()](IterableDrbd2::get_message)\n\n"]
#[derive(Debug)]
pub struct OpConnectionConnectDo<'r> {
    request: Request<'r>,
}
impl<'r> OpConnectionConnectDo<'r> {
    pub fn new(mut request: Request<'r>) -> Self {
        Self::write_header(request.buf_mut());
        Self { request: request }
    }
    pub fn encode_request<'buf>(buf: &'buf mut Vec<u8>) -> PushDrbd2<&'buf mut Vec<u8>> {
        Self::write_header(buf);
        PushDrbd2::new(buf)
    }
    pub fn encode(&mut self) -> PushDrbd2<&mut Vec<u8>> {
        PushDrbd2::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushDrbd2<RequestBuf<'r>> {
        PushDrbd2::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> IterableDrbd2<'a> {
        let (_header, attrs) = buf.split_at(buf.len().min(BuiltinNfgenmsg::len()));
        IterableDrbd2::with_loc(attrs, buf.as_ptr() as usize)
    }
    fn write_header<Prev: Pusher>(prev: &mut Prev) {
        let mut header = BuiltinNfgenmsg::new();
        header.cmd = 21u8;
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
impl NetlinkRequest for OpConnectionConnectDo<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Generic("drbd2".as_bytes())
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = IterableDrbd2<'buf>;
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
#[doc = "Disconnect from a peer.\n\nFlags: admin-perm\n\nRequest attributes:\n- [.nested_context()](PushDrbd2::nested_context)\n- [.nested_disconnect_parms()](PushDrbd2::nested_disconnect_parms)\n\nReply attributes:\n- [.get_state_result()](IterableDrbd2::get_state_result)\n- [.get_message()](IterableDrbd2::get_message)\n\n"]
#[derive(Debug)]
pub struct OpConnectionDisconnectDo<'r> {
    request: Request<'r>,
}
impl<'r> OpConnectionDisconnectDo<'r> {
    pub fn new(mut request: Request<'r>) -> Self {
        Self::write_header(request.buf_mut());
        Self { request: request }
    }
    pub fn encode_request<'buf>(buf: &'buf mut Vec<u8>) -> PushDrbd2<&'buf mut Vec<u8>> {
        Self::write_header(buf);
        PushDrbd2::new(buf)
    }
    pub fn encode(&mut self) -> PushDrbd2<&mut Vec<u8>> {
        PushDrbd2::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushDrbd2<RequestBuf<'r>> {
        PushDrbd2::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> IterableDrbd2<'a> {
        let (_header, attrs) = buf.split_at(buf.len().min(BuiltinNfgenmsg::len()));
        IterableDrbd2::with_loc(attrs, buf.as_ptr() as usize)
    }
    fn write_header<Prev: Pusher>(prev: &mut Prev) {
        let mut header = BuiltinNfgenmsg::new();
        header.cmd = 22u8;
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
impl NetlinkRequest for OpConnectionDisconnectDo<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Generic("drbd2".as_bytes())
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = IterableDrbd2<'buf>;
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
#[doc = "Change network options.\n\nFlags: admin-perm\n\nRequest attributes:\n- [.nested_context()](PushDrbd2::nested_context)\n- [.push_set_defaults()](PushDrbd2::push_set_defaults)\n- [.nested_net_conf()](PushDrbd2::nested_net_conf)\n\nReply attributes:\n- [.get_state_result()](IterableDrbd2::get_state_result)\n- [.get_message()](IterableDrbd2::get_message)\n\n"]
#[derive(Debug)]
pub struct OpConnectionSetDo<'r> {
    request: Request<'r>,
}
impl<'r> OpConnectionSetDo<'r> {
    pub fn new(mut request: Request<'r>) -> Self {
        Self::write_header(request.buf_mut());
        Self { request: request }
    }
    pub fn encode_request<'buf>(buf: &'buf mut Vec<u8>) -> PushDrbd2<&'buf mut Vec<u8>> {
        Self::write_header(buf);
        PushDrbd2::new(buf)
    }
    pub fn encode(&mut self) -> PushDrbd2<&mut Vec<u8>> {
        PushDrbd2::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushDrbd2<RequestBuf<'r>> {
        PushDrbd2::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> IterableDrbd2<'a> {
        let (_header, attrs) = buf.split_at(buf.len().min(BuiltinNfgenmsg::len()));
        IterableDrbd2::with_loc(attrs, buf.as_ptr() as usize)
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
impl NetlinkRequest for OpConnectionSetDo<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Generic("drbd2".as_bytes())
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = IterableDrbd2<'buf>;
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
#[doc = "Forget a previously connected peer. The peer is identified by\ncontext.peer-node-id.\n\nFlags: admin-perm\n\nRequest attributes:\n- [.nested_context()](PushDrbd2::nested_context)\n\nReply attributes:\n- [.get_state_result()](IterableDrbd2::get_state_result)\n- [.get_message()](IterableDrbd2::get_message)\n\n"]
#[derive(Debug)]
pub struct OpConnectionForgetDo<'r> {
    request: Request<'r>,
}
impl<'r> OpConnectionForgetDo<'r> {
    pub fn new(mut request: Request<'r>) -> Self {
        Self::write_header(request.buf_mut());
        Self { request: request }
    }
    pub fn encode_request<'buf>(buf: &'buf mut Vec<u8>) -> PushDrbd2<&'buf mut Vec<u8>> {
        Self::write_header(buf);
        PushDrbd2::new(buf)
    }
    pub fn encode(&mut self) -> PushDrbd2<&mut Vec<u8>> {
        PushDrbd2::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushDrbd2<RequestBuf<'r>> {
        PushDrbd2::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> IterableDrbd2<'a> {
        let (_header, attrs) = buf.split_at(buf.len().min(BuiltinNfgenmsg::len()));
        IterableDrbd2::with_loc(attrs, buf.as_ptr() as usize)
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
impl NetlinkRequest for OpConnectionForgetDo<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Generic("drbd2".as_bytes())
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = IterableDrbd2<'buf>;
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
#[doc = "Add a new network path to a connection.\n\nFlags: admin-perm\n\nRequest attributes:\n- [.nested_context()](PushDrbd2::nested_context)\n\nReply attributes:\n- [.get_state_result()](IterableDrbd2::get_state_result)\n- [.get_message()](IterableDrbd2::get_message)\n\n"]
#[derive(Debug)]
pub struct OpPathNewDo<'r> {
    request: Request<'r>,
}
impl<'r> OpPathNewDo<'r> {
    pub fn new(mut request: Request<'r>) -> Self {
        Self::write_header(request.buf_mut());
        Self { request: request }
    }
    pub fn encode_request<'buf>(buf: &'buf mut Vec<u8>) -> PushDrbd2<&'buf mut Vec<u8>> {
        Self::write_header(buf);
        PushDrbd2::new(buf)
    }
    pub fn encode(&mut self) -> PushDrbd2<&mut Vec<u8>> {
        PushDrbd2::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushDrbd2<RequestBuf<'r>> {
        PushDrbd2::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> IterableDrbd2<'a> {
        let (_header, attrs) = buf.split_at(buf.len().min(BuiltinNfgenmsg::len()));
        IterableDrbd2::with_loc(attrs, buf.as_ptr() as usize)
    }
    fn write_header<Prev: Pusher>(prev: &mut Prev) {
        let mut header = BuiltinNfgenmsg::new();
        header.cmd = 25u8;
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
impl NetlinkRequest for OpPathNewDo<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Generic("drbd2".as_bytes())
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = IterableDrbd2<'buf>;
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
#[doc = "Delete a network path from a connection.\n\nFlags: admin-perm\n\nRequest attributes:\n- [.nested_context()](PushDrbd2::nested_context)\n\nReply attributes:\n- [.get_state_result()](IterableDrbd2::get_state_result)\n- [.get_message()](IterableDrbd2::get_message)\n\n"]
#[derive(Debug)]
pub struct OpPathDelDo<'r> {
    request: Request<'r>,
}
impl<'r> OpPathDelDo<'r> {
    pub fn new(mut request: Request<'r>) -> Self {
        Self::write_header(request.buf_mut());
        Self { request: request }
    }
    pub fn encode_request<'buf>(buf: &'buf mut Vec<u8>) -> PushDrbd2<&'buf mut Vec<u8>> {
        Self::write_header(buf);
        PushDrbd2::new(buf)
    }
    pub fn encode(&mut self) -> PushDrbd2<&mut Vec<u8>> {
        PushDrbd2::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushDrbd2<RequestBuf<'r>> {
        PushDrbd2::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> IterableDrbd2<'a> {
        let (_header, attrs) = buf.split_at(buf.len().min(BuiltinNfgenmsg::len()));
        IterableDrbd2::with_loc(attrs, buf.as_ptr() as usize)
    }
    fn write_header<Prev: Pusher>(prev: &mut Prev) {
        let mut header = BuiltinNfgenmsg::new();
        header.cmd = 26u8;
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
impl NetlinkRequest for OpPathDelDo<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Generic("drbd2".as_bytes())
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = IterableDrbd2<'buf>;
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
#[doc = "Change peer device options.\n\nFlags: admin-perm\n\nRequest attributes:\n- [.nested_context()](PushDrbd2::nested_context)\n- [.push_set_defaults()](PushDrbd2::push_set_defaults)\n- [.nested_peer_device_conf()](PushDrbd2::nested_peer_device_conf)\n\nReply attributes:\n- [.get_state_result()](IterableDrbd2::get_state_result)\n- [.get_message()](IterableDrbd2::get_message)\n\n"]
#[derive(Debug)]
pub struct OpPeerDeviceSetDo<'r> {
    request: Request<'r>,
}
impl<'r> OpPeerDeviceSetDo<'r> {
    pub fn new(mut request: Request<'r>) -> Self {
        Self::write_header(request.buf_mut());
        Self { request: request }
    }
    pub fn encode_request<'buf>(buf: &'buf mut Vec<u8>) -> PushDrbd2<&'buf mut Vec<u8>> {
        Self::write_header(buf);
        PushDrbd2::new(buf)
    }
    pub fn encode(&mut self) -> PushDrbd2<&mut Vec<u8>> {
        PushDrbd2::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushDrbd2<RequestBuf<'r>> {
        PushDrbd2::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> IterableDrbd2<'a> {
        let (_header, attrs) = buf.split_at(buf.len().min(BuiltinNfgenmsg::len()));
        IterableDrbd2::with_loc(attrs, buf.as_ptr() as usize)
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
impl NetlinkRequest for OpPeerDeviceSetDo<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Generic("drbd2".as_bytes())
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = IterableDrbd2<'buf>;
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
#[doc = "Invalidate the peer data of a DRBD peer device.\n\nFlags: admin-perm\n\nRequest attributes:\n- [.nested_context()](PushDrbd2::nested_context)\n- [.nested_invalidate_peer_parms()](PushDrbd2::nested_invalidate_peer_parms)\n\nReply attributes:\n- [.get_state_result()](IterableDrbd2::get_state_result)\n- [.get_message()](IterableDrbd2::get_message)\n\n"]
#[derive(Debug)]
pub struct OpPeerDeviceInvalidateDo<'r> {
    request: Request<'r>,
}
impl<'r> OpPeerDeviceInvalidateDo<'r> {
    pub fn new(mut request: Request<'r>) -> Self {
        Self::write_header(request.buf_mut());
        Self { request: request }
    }
    pub fn encode_request<'buf>(buf: &'buf mut Vec<u8>) -> PushDrbd2<&'buf mut Vec<u8>> {
        Self::write_header(buf);
        PushDrbd2::new(buf)
    }
    pub fn encode(&mut self) -> PushDrbd2<&mut Vec<u8>> {
        PushDrbd2::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushDrbd2<RequestBuf<'r>> {
        PushDrbd2::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> IterableDrbd2<'a> {
        let (_header, attrs) = buf.split_at(buf.len().min(BuiltinNfgenmsg::len()));
        IterableDrbd2::with_loc(attrs, buf.as_ptr() as usize)
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
impl NetlinkRequest for OpPeerDeviceInvalidateDo<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Generic("drbd2".as_bytes())
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = IterableDrbd2<'buf>;
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
#[doc = "Pause resynchronization.\n\nFlags: admin-perm\n\nRequest attributes:\n- [.nested_context()](PushDrbd2::nested_context)\n\nReply attributes:\n- [.get_state_result()](IterableDrbd2::get_state_result)\n- [.get_message()](IterableDrbd2::get_message)\n\n"]
#[derive(Debug)]
pub struct OpPeerDevicePauseSyncDo<'r> {
    request: Request<'r>,
}
impl<'r> OpPeerDevicePauseSyncDo<'r> {
    pub fn new(mut request: Request<'r>) -> Self {
        Self::write_header(request.buf_mut());
        Self { request: request }
    }
    pub fn encode_request<'buf>(buf: &'buf mut Vec<u8>) -> PushDrbd2<&'buf mut Vec<u8>> {
        Self::write_header(buf);
        PushDrbd2::new(buf)
    }
    pub fn encode(&mut self) -> PushDrbd2<&mut Vec<u8>> {
        PushDrbd2::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushDrbd2<RequestBuf<'r>> {
        PushDrbd2::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> IterableDrbd2<'a> {
        let (_header, attrs) = buf.split_at(buf.len().min(BuiltinNfgenmsg::len()));
        IterableDrbd2::with_loc(attrs, buf.as_ptr() as usize)
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
impl NetlinkRequest for OpPeerDevicePauseSyncDo<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Generic("drbd2".as_bytes())
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = IterableDrbd2<'buf>;
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
#[doc = "Resume resynchronization.\n\nFlags: admin-perm\n\nRequest attributes:\n- [.nested_context()](PushDrbd2::nested_context)\n\nReply attributes:\n- [.get_state_result()](IterableDrbd2::get_state_result)\n- [.get_message()](IterableDrbd2::get_message)\n\n"]
#[derive(Debug)]
pub struct OpPeerDeviceResumeSyncDo<'r> {
    request: Request<'r>,
}
impl<'r> OpPeerDeviceResumeSyncDo<'r> {
    pub fn new(mut request: Request<'r>) -> Self {
        Self::write_header(request.buf_mut());
        Self { request: request }
    }
    pub fn encode_request<'buf>(buf: &'buf mut Vec<u8>) -> PushDrbd2<&'buf mut Vec<u8>> {
        Self::write_header(buf);
        PushDrbd2::new(buf)
    }
    pub fn encode(&mut self) -> PushDrbd2<&mut Vec<u8>> {
        PushDrbd2::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushDrbd2<RequestBuf<'r>> {
        PushDrbd2::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> IterableDrbd2<'a> {
        let (_header, attrs) = buf.split_at(buf.len().min(BuiltinNfgenmsg::len()));
        IterableDrbd2::with_loc(attrs, buf.as_ptr() as usize)
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
impl NetlinkRequest for OpPeerDeviceResumeSyncDo<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Generic("drbd2".as_bytes())
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = IterableDrbd2<'buf>;
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
#[doc = "Start online verification.\n\nFlags: admin-perm\n\nRequest attributes:\n- [.nested_context()](PushDrbd2::nested_context)\n- [.nested_start_ov_parms()](PushDrbd2::nested_start_ov_parms)\n\nReply attributes:\n- [.get_state_result()](IterableDrbd2::get_state_result)\n- [.get_message()](IterableDrbd2::get_message)\n\n"]
#[derive(Debug)]
pub struct OpPeerDeviceStartOvDo<'r> {
    request: Request<'r>,
}
impl<'r> OpPeerDeviceStartOvDo<'r> {
    pub fn new(mut request: Request<'r>) -> Self {
        Self::write_header(request.buf_mut());
        Self { request: request }
    }
    pub fn encode_request<'buf>(buf: &'buf mut Vec<u8>) -> PushDrbd2<&'buf mut Vec<u8>> {
        Self::write_header(buf);
        PushDrbd2::new(buf)
    }
    pub fn encode(&mut self) -> PushDrbd2<&mut Vec<u8>> {
        PushDrbd2::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushDrbd2<RequestBuf<'r>> {
        PushDrbd2::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> IterableDrbd2<'a> {
        let (_header, attrs) = buf.split_at(buf.len().min(BuiltinNfgenmsg::len()));
        IterableDrbd2::with_loc(attrs, buf.as_ptr() as usize)
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
impl NetlinkRequest for OpPeerDeviceStartOvDo<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Generic("drbd2".as_bytes())
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = IterableDrbd2<'buf>;
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
#[doc = "Get the timeout type for a peer device.\n\nRequest attributes:\n- [.nested_context()](PushDrbd2::nested_context)\n\nReply attributes:\n- [.get_timeout_type()](IterableDrbd2::get_timeout_type)\n\n"]
#[derive(Debug)]
pub struct OpTimeoutTypeGetDo<'r> {
    request: Request<'r>,
}
impl<'r> OpTimeoutTypeGetDo<'r> {
    pub fn new(mut request: Request<'r>) -> Self {
        Self::write_header(request.buf_mut());
        Self { request: request }
    }
    pub fn encode_request<'buf>(buf: &'buf mut Vec<u8>) -> PushDrbd2<&'buf mut Vec<u8>> {
        Self::write_header(buf);
        PushDrbd2::new(buf)
    }
    pub fn encode(&mut self) -> PushDrbd2<&mut Vec<u8>> {
        PushDrbd2::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushDrbd2<RequestBuf<'r>> {
        PushDrbd2::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> IterableDrbd2<'a> {
        let (_header, attrs) = buf.split_at(buf.len().min(BuiltinNfgenmsg::len()));
        IterableDrbd2::with_loc(attrs, buf.as_ptr() as usize)
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
impl NetlinkRequest for OpTimeoutTypeGetDo<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Generic("drbd2".as_bytes())
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = IterableDrbd2<'buf>;
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
#[doc = "Dump all DRBD resources. Optionally filtered by context.\n\nRequest attributes:\n- [.nested_context()](PushDrbd2::nested_context)\n\nReply attributes:\n- [.get_resource()](IterableDrbd2::get_resource)\n\n"]
#[derive(Debug)]
pub struct OpResourceGetDump<'r> {
    request: Request<'r>,
}
impl<'r> OpResourceGetDump<'r> {
    pub fn new(mut request: Request<'r>) -> Self {
        Self::write_header(request.buf_mut());
        Self {
            request: request.set_dump(),
        }
    }
    pub fn encode_request<'buf>(buf: &'buf mut Vec<u8>) -> PushDrbd2<&'buf mut Vec<u8>> {
        Self::write_header(buf);
        PushDrbd2::new(buf)
    }
    pub fn encode(&mut self) -> PushDrbd2<&mut Vec<u8>> {
        PushDrbd2::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushDrbd2<RequestBuf<'r>> {
        PushDrbd2::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> IterableDrbd2<'a> {
        let (_header, attrs) = buf.split_at(buf.len().min(BuiltinNfgenmsg::len()));
        IterableDrbd2::with_loc(attrs, buf.as_ptr() as usize)
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
impl NetlinkRequest for OpResourceGetDump<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Generic("drbd2".as_bytes())
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = IterableDrbd2<'buf>;
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
#[doc = "Dump all DRBD devices. Optionally filtered by context.\n\nRequest attributes:\n- [.nested_context()](PushDrbd2::nested_context)\n\nReply attributes:\n- [.get_device()](IterableDrbd2::get_device)\n\n"]
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
    pub fn encode_request<'buf>(buf: &'buf mut Vec<u8>) -> PushDrbd2<&'buf mut Vec<u8>> {
        Self::write_header(buf);
        PushDrbd2::new(buf)
    }
    pub fn encode(&mut self) -> PushDrbd2<&mut Vec<u8>> {
        PushDrbd2::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushDrbd2<RequestBuf<'r>> {
        PushDrbd2::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> IterableDrbd2<'a> {
        let (_header, attrs) = buf.split_at(buf.len().min(BuiltinNfgenmsg::len()));
        IterableDrbd2::with_loc(attrs, buf.as_ptr() as usize)
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
impl NetlinkRequest for OpDeviceGetDump<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Generic("drbd2".as_bytes())
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = IterableDrbd2<'buf>;
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
#[doc = "Dump all DRBD connections. Optionally filtered by context.\n\nRequest attributes:\n- [.nested_context()](PushDrbd2::nested_context)\n\nReply attributes:\n- [.get_connection()](IterableDrbd2::get_connection)\n\n"]
#[derive(Debug)]
pub struct OpConnectionGetDump<'r> {
    request: Request<'r>,
}
impl<'r> OpConnectionGetDump<'r> {
    pub fn new(mut request: Request<'r>) -> Self {
        Self::write_header(request.buf_mut());
        Self {
            request: request.set_dump(),
        }
    }
    pub fn encode_request<'buf>(buf: &'buf mut Vec<u8>) -> PushDrbd2<&'buf mut Vec<u8>> {
        Self::write_header(buf);
        PushDrbd2::new(buf)
    }
    pub fn encode(&mut self) -> PushDrbd2<&mut Vec<u8>> {
        PushDrbd2::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushDrbd2<RequestBuf<'r>> {
        PushDrbd2::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> IterableDrbd2<'a> {
        let (_header, attrs) = buf.split_at(buf.len().min(BuiltinNfgenmsg::len()));
        IterableDrbd2::with_loc(attrs, buf.as_ptr() as usize)
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
impl NetlinkRequest for OpConnectionGetDump<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Generic("drbd2".as_bytes())
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = IterableDrbd2<'buf>;
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
#[doc = "Dump all DRBD peer devices. Optionally filtered by context.\n\nRequest attributes:\n- [.nested_context()](PushDrbd2::nested_context)\n\nReply attributes:\n- [.get_peer_device()](IterableDrbd2::get_peer_device)\n\n"]
#[derive(Debug)]
pub struct OpPeerDeviceGetDump<'r> {
    request: Request<'r>,
}
impl<'r> OpPeerDeviceGetDump<'r> {
    pub fn new(mut request: Request<'r>) -> Self {
        Self::write_header(request.buf_mut());
        Self {
            request: request.set_dump(),
        }
    }
    pub fn encode_request<'buf>(buf: &'buf mut Vec<u8>) -> PushDrbd2<&'buf mut Vec<u8>> {
        Self::write_header(buf);
        PushDrbd2::new(buf)
    }
    pub fn encode(&mut self) -> PushDrbd2<&mut Vec<u8>> {
        PushDrbd2::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushDrbd2<RequestBuf<'r>> {
        PushDrbd2::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> IterableDrbd2<'a> {
        let (_header, attrs) = buf.split_at(buf.len().min(BuiltinNfgenmsg::len()));
        IterableDrbd2::with_loc(attrs, buf.as_ptr() as usize)
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
impl NetlinkRequest for OpPeerDeviceGetDump<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Generic("drbd2".as_bytes())
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = IterableDrbd2<'buf>;
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
#[doc = "Dump all DRBD paths. Optionally filtered by context.\n\nRequest attributes:\n- [.nested_context()](PushDrbd2::nested_context)\n\nReply attributes:\n- [.get_path()](IterableDrbd2::get_path)\n\n"]
#[derive(Debug)]
pub struct OpPathGetDump<'r> {
    request: Request<'r>,
}
impl<'r> OpPathGetDump<'r> {
    pub fn new(mut request: Request<'r>) -> Self {
        Self::write_header(request.buf_mut());
        Self {
            request: request.set_dump(),
        }
    }
    pub fn encode_request<'buf>(buf: &'buf mut Vec<u8>) -> PushDrbd2<&'buf mut Vec<u8>> {
        Self::write_header(buf);
        PushDrbd2::new(buf)
    }
    pub fn encode(&mut self) -> PushDrbd2<&mut Vec<u8>> {
        PushDrbd2::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushDrbd2<RequestBuf<'r>> {
        PushDrbd2::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> IterableDrbd2<'a> {
        let (_header, attrs) = buf.split_at(buf.len().min(BuiltinNfgenmsg::len()));
        IterableDrbd2::with_loc(attrs, buf.as_ptr() as usize)
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
impl NetlinkRequest for OpPathGetDump<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Generic("drbd2".as_bytes())
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = IterableDrbd2<'buf>;
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
#[doc = "Dump a consistent snapshot of all resources, devices, connections, peer\ndevices and paths as state-change messages with action exists.\n\nReply attributes:\n- [.get_action()](IterableStateChange::get_action)\n- [.get_more()](IterableStateChange::get_more)\n- [.get_resource()](IterableStateChange::get_resource)\n- [.get_device()](IterableStateChange::get_device)\n- [.get_connection()](IterableStateChange::get_connection)\n- [.get_peer_device()](IterableStateChange::get_peer_device)\n- [.get_path()](IterableStateChange::get_path)\n\n"]
#[derive(Debug)]
pub struct OpStateGetDump<'r> {
    request: Request<'r>,
}
impl<'r> OpStateGetDump<'r> {
    pub fn new(mut request: Request<'r>) -> Self {
        Self::write_header(request.buf_mut());
        Self {
            request: request.set_dump(),
        }
    }
    pub fn encode_request<'buf>(buf: &'buf mut Vec<u8>) -> PushStateChange<&'buf mut Vec<u8>> {
        Self::write_header(buf);
        PushStateChange::new(buf)
    }
    pub fn encode(&mut self) -> PushStateChange<&mut Vec<u8>> {
        PushStateChange::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushStateChange<RequestBuf<'r>> {
        PushStateChange::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> IterableStateChange<'a> {
        let (_header, attrs) = buf.split_at(buf.len().min(BuiltinNfgenmsg::len()));
        IterableStateChange::with_loc(attrs, buf.as_ptr() as usize)
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
impl NetlinkRequest for OpStateGetDump<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Generic("drbd2".as_bytes())
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = IterableStateChange<'buf>;
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
#[doc = "Write the bitmaps and the superblock to the meta data device, so that a\nsnapshot of the meta data taken while I/O stays suspended carries every\nchanged block. Requires I/O suspended and drained, no peer write still\nexecuting, no resync running, and a disk that is at least Outdated.\n\nFlags: admin-perm\n\nRequest attributes:\n- [.nested_context()](PushDrbd2::nested_context)\n\nReply attributes:\n- [.get_state_result()](IterableDrbd2::get_state_result)\n- [.get_message()](IterableDrbd2::get_message)\n\n"]
#[derive(Debug)]
pub struct OpDeviceCbtFlushDo<'r> {
    request: Request<'r>,
}
impl<'r> OpDeviceCbtFlushDo<'r> {
    pub fn new(mut request: Request<'r>) -> Self {
        Self::write_header(request.buf_mut());
        Self { request: request }
    }
    pub fn encode_request<'buf>(buf: &'buf mut Vec<u8>) -> PushDrbd2<&'buf mut Vec<u8>> {
        Self::write_header(buf);
        PushDrbd2::new(buf)
    }
    pub fn encode(&mut self) -> PushDrbd2<&mut Vec<u8>> {
        PushDrbd2::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushDrbd2<RequestBuf<'r>> {
        PushDrbd2::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> IterableDrbd2<'a> {
        let (_header, attrs) = buf.split_at(buf.len().min(BuiltinNfgenmsg::len()));
        IterableDrbd2::with_loc(attrs, buf.as_ptr() as usize)
    }
    fn write_header<Prev: Pusher>(prev: &mut Prev) {
        let mut header = BuiltinNfgenmsg::new();
        header.cmd = 41u8;
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
impl NetlinkRequest for OpDeviceCbtFlushDo<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Generic("drbd2".as_bytes())
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = IterableDrbd2<'buf>;
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
#[doc = "Clear the changed block tracking bitmap and start a new tracking\ninterval at a bookmark. Allocates the tracking bitmap slot if needed.\nRequires a device-cbt-flush within the same suspension of I/O and no\nwrite to the backing device since.\n\nFlags: admin-perm\n\nRequest attributes:\n- [.nested_context()](PushDrbd2::nested_context)\n- [.nested_cbt_start_parms()](PushDrbd2::nested_cbt_start_parms)\n\nReply attributes:\n- [.get_state_result()](IterableDrbd2::get_state_result)\n- [.get_message()](IterableDrbd2::get_message)\n\n"]
#[derive(Debug)]
pub struct OpDeviceCbtStartDo<'r> {
    request: Request<'r>,
}
impl<'r> OpDeviceCbtStartDo<'r> {
    pub fn new(mut request: Request<'r>) -> Self {
        Self::write_header(request.buf_mut());
        Self { request: request }
    }
    pub fn encode_request<'buf>(buf: &'buf mut Vec<u8>) -> PushDrbd2<&'buf mut Vec<u8>> {
        Self::write_header(buf);
        PushDrbd2::new(buf)
    }
    pub fn encode(&mut self) -> PushDrbd2<&mut Vec<u8>> {
        PushDrbd2::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushDrbd2<RequestBuf<'r>> {
        PushDrbd2::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> IterableDrbd2<'a> {
        let (_header, attrs) = buf.split_at(buf.len().min(BuiltinNfgenmsg::len()));
        IterableDrbd2::with_loc(attrs, buf.as_ptr() as usize)
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
impl NetlinkRequest for OpDeviceCbtStartDo<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Generic("drbd2".as_bytes())
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = IterableDrbd2<'buf>;
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
#[doc = "Stop changed block tracking and free its bitmap slot.\n\nFlags: admin-perm\n\nRequest attributes:\n- [.nested_context()](PushDrbd2::nested_context)\n\nReply attributes:\n- [.get_state_result()](IterableDrbd2::get_state_result)\n- [.get_message()](IterableDrbd2::get_message)\n\n"]
#[derive(Debug)]
pub struct OpDeviceCbtStopDo<'r> {
    request: Request<'r>,
}
impl<'r> OpDeviceCbtStopDo<'r> {
    pub fn new(mut request: Request<'r>) -> Self {
        Self::write_header(request.buf_mut());
        Self { request: request }
    }
    pub fn encode_request<'buf>(buf: &'buf mut Vec<u8>) -> PushDrbd2<&'buf mut Vec<u8>> {
        Self::write_header(buf);
        PushDrbd2::new(buf)
    }
    pub fn encode(&mut self) -> PushDrbd2<&mut Vec<u8>> {
        PushDrbd2::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushDrbd2<RequestBuf<'r>> {
        PushDrbd2::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> IterableDrbd2<'a> {
        let (_header, attrs) = buf.split_at(buf.len().min(BuiltinNfgenmsg::len()));
        IterableDrbd2::with_loc(attrs, buf.as_ptr() as usize)
    }
    fn write_header<Prev: Pusher>(prev: &mut Prev) {
        let mut header = BuiltinNfgenmsg::new();
        header.cmd = 43u8;
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
impl NetlinkRequest for OpDeviceCbtStopDo<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Generic("drbd2".as_bytes())
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = IterableDrbd2<'buf>;
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
#[doc = "Get the changed block tracking state of a device.\n\nRequest attributes:\n- [.nested_context()](PushDrbd2::nested_context)\n\nReply attributes:\n- [.get_cbt_info()](IterableDrbd2::get_cbt_info)\n\n"]
#[derive(Debug)]
pub struct OpDeviceCbtStatusDo<'r> {
    request: Request<'r>,
}
impl<'r> OpDeviceCbtStatusDo<'r> {
    pub fn new(mut request: Request<'r>) -> Self {
        Self::write_header(request.buf_mut());
        Self { request: request }
    }
    pub fn encode_request<'buf>(buf: &'buf mut Vec<u8>) -> PushDrbd2<&'buf mut Vec<u8>> {
        Self::write_header(buf);
        PushDrbd2::new(buf)
    }
    pub fn encode(&mut self) -> PushDrbd2<&mut Vec<u8>> {
        PushDrbd2::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushDrbd2<RequestBuf<'r>> {
        PushDrbd2::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> IterableDrbd2<'a> {
        let (_header, attrs) = buf.split_at(buf.len().min(BuiltinNfgenmsg::len()));
        IterableDrbd2::with_loc(attrs, buf.as_ptr() as usize)
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
impl NetlinkRequest for OpDeviceCbtStatusDo<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Generic("drbd2".as_bytes())
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = IterableDrbd2<'buf>;
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
    #[doc = "Create a new DRBD resource.\n\nFlags: admin-perm\n\nRequest attributes:\n- [.nested_context()](PushDrbd2::nested_context)\n- [.nested_resource_opts()](PushDrbd2::nested_resource_opts)\n\nReply attributes:\n- [.get_state_result()](IterableDrbd2::get_state_result)\n- [.get_message()](IterableDrbd2::get_message)\n\n"]
    pub fn op_resource_new_do(self) -> OpResourceNewDo<'buf> {
        let mut res = OpResourceNewDo::new(self);
        res.request.do_writeback(
            res.protocol(),
            "op-resource-new-do",
            OpResourceNewDo::lookup,
        );
        res
    }
    #[doc = "Delete a DRBD resource.\n\nFlags: admin-perm\n\nRequest attributes:\n- [.nested_context()](PushDrbd2::nested_context)\n\nReply attributes:\n- [.get_state_result()](IterableDrbd2::get_state_result)\n- [.get_message()](IterableDrbd2::get_message)\n\n"]
    pub fn op_resource_del_do(self) -> OpResourceDelDo<'buf> {
        let mut res = OpResourceDelDo::new(self);
        res.request.do_writeback(
            res.protocol(),
            "op-resource-del-do",
            OpResourceDelDo::lookup,
        );
        res
    }
    #[doc = "Change resource options.\n\nFlags: admin-perm\n\nRequest attributes:\n- [.nested_context()](PushDrbd2::nested_context)\n- [.push_set_defaults()](PushDrbd2::push_set_defaults)\n- [.nested_resource_opts()](PushDrbd2::nested_resource_opts)\n\nReply attributes:\n- [.get_state_result()](IterableDrbd2::get_state_result)\n- [.get_message()](IterableDrbd2::get_message)\n\n"]
    pub fn op_resource_set_do(self) -> OpResourceSetDo<'buf> {
        let mut res = OpResourceSetDo::new(self);
        res.request.do_writeback(
            res.protocol(),
            "op-resource-set-do",
            OpResourceSetDo::lookup,
        );
        res
    }
    #[doc = "Rename a DRBD resource.\n\nFlags: admin-perm\n\nRequest attributes:\n- [.nested_context()](PushDrbd2::nested_context)\n- [.nested_rename_parms()](PushDrbd2::nested_rename_parms)\n\nReply attributes:\n- [.get_state_result()](IterableDrbd2::get_state_result)\n- [.get_message()](IterableDrbd2::get_message)\n\n"]
    pub fn op_resource_rename_do(self) -> OpResourceRenameDo<'buf> {
        let mut res = OpResourceRenameDo::new(self);
        res.request.do_writeback(
            res.protocol(),
            "op-resource-rename-do",
            OpResourceRenameDo::lookup,
        );
        res
    }
    #[doc = "Shut down a DRBD resource.\n\nFlags: admin-perm\n\nRequest attributes:\n- [.nested_context()](PushDrbd2::nested_context)\n\nReply attributes:\n- [.get_state_result()](IterableDrbd2::get_state_result)\n- [.get_message()](IterableDrbd2::get_message)\n\n"]
    pub fn op_resource_down_do(self) -> OpResourceDownDo<'buf> {
        let mut res = OpResourceDownDo::new(self);
        res.request.do_writeback(
            res.protocol(),
            "op-resource-down-do",
            OpResourceDownDo::lookup,
        );
        res
    }
    #[doc = "Promote the resource to primary role.\n\nFlags: admin-perm\n\nRequest attributes:\n- [.nested_context()](PushDrbd2::nested_context)\n- [.nested_set_role_parms()](PushDrbd2::nested_set_role_parms)\n\nReply attributes:\n- [.get_state_result()](IterableDrbd2::get_state_result)\n- [.get_message()](IterableDrbd2::get_message)\n\n"]
    pub fn op_resource_primary_do(self) -> OpResourcePrimaryDo<'buf> {
        let mut res = OpResourcePrimaryDo::new(self);
        res.request.do_writeback(
            res.protocol(),
            "op-resource-primary-do",
            OpResourcePrimaryDo::lookup,
        );
        res
    }
    #[doc = "Demote the resource to secondary role.\n\nFlags: admin-perm\n\nRequest attributes:\n- [.nested_context()](PushDrbd2::nested_context)\n- [.nested_set_role_parms()](PushDrbd2::nested_set_role_parms)\n\nReply attributes:\n- [.get_state_result()](IterableDrbd2::get_state_result)\n- [.get_message()](IterableDrbd2::get_message)\n\n"]
    pub fn op_resource_secondary_do(self) -> OpResourceSecondaryDo<'buf> {
        let mut res = OpResourceSecondaryDo::new(self);
        res.request.do_writeback(
            res.protocol(),
            "op-resource-secondary-do",
            OpResourceSecondaryDo::lookup,
        );
        res
    }
    #[doc = "Suspend I/O on a resource.\n\nFlags: admin-perm\n\nRequest attributes:\n- [.nested_context()](PushDrbd2::nested_context)\n- [.nested_suspend_io_parms()](PushDrbd2::nested_suspend_io_parms)\n\nReply attributes:\n- [.get_state_result()](IterableDrbd2::get_state_result)\n- [.get_message()](IterableDrbd2::get_message)\n\n"]
    pub fn op_resource_suspend_io_do(self) -> OpResourceSuspendIoDo<'buf> {
        let mut res = OpResourceSuspendIoDo::new(self);
        res.request.do_writeback(
            res.protocol(),
            "op-resource-suspend-io-do",
            OpResourceSuspendIoDo::lookup,
        );
        res
    }
    #[doc = "Resume I/O on a resource.\n\nFlags: admin-perm\n\nRequest attributes:\n- [.nested_context()](PushDrbd2::nested_context)\n\nReply attributes:\n- [.get_state_result()](IterableDrbd2::get_state_result)\n- [.get_message()](IterableDrbd2::get_message)\n\n"]
    pub fn op_resource_resume_io_do(self) -> OpResourceResumeIoDo<'buf> {
        let mut res = OpResourceResumeIoDo::new(self);
        res.request.do_writeback(
            res.protocol(),
            "op-resource-resume-io-do",
            OpResourceResumeIoDo::lookup,
        );
        res
    }
    #[doc = "Add a DRBD device as a volume to a resource.\n\nFlags: admin-perm\n\nRequest attributes:\n- [.nested_context()](PushDrbd2::nested_context)\n- [.nested_device_conf()](PushDrbd2::nested_device_conf)\n\nReply attributes:\n- [.get_state_result()](IterableDrbd2::get_state_result)\n- [.get_message()](IterableDrbd2::get_message)\n\n"]
    pub fn op_device_new_do(self) -> OpDeviceNewDo<'buf> {
        let mut res = OpDeviceNewDo::new(self);
        res.request
            .do_writeback(res.protocol(), "op-device-new-do", OpDeviceNewDo::lookup);
        res
    }
    #[doc = "Delete a DRBD device.\n\nFlags: admin-perm\n\nRequest attributes:\n- [.nested_context()](PushDrbd2::nested_context)\n\nReply attributes:\n- [.get_state_result()](IterableDrbd2::get_state_result)\n- [.get_message()](IterableDrbd2::get_message)\n\n"]
    pub fn op_device_del_do(self) -> OpDeviceDelDo<'buf> {
        let mut res = OpDeviceDelDo::new(self);
        res.request
            .do_writeback(res.protocol(), "op-device-del-do", OpDeviceDelDo::lookup);
        res
    }
    #[doc = "Attach a local backing device to a DRBD device.\n\nFlags: admin-perm\n\nRequest attributes:\n- [.nested_context()](PushDrbd2::nested_context)\n- [.nested_disk_conf()](PushDrbd2::nested_disk_conf)\n- [.nested_attach_parms()](PushDrbd2::nested_attach_parms)\n\nReply attributes:\n- [.get_state_result()](IterableDrbd2::get_state_result)\n- [.get_message()](IterableDrbd2::get_message)\n\n"]
    pub fn op_device_attach_do(self) -> OpDeviceAttachDo<'buf> {
        let mut res = OpDeviceAttachDo::new(self);
        res.request.do_writeback(
            res.protocol(),
            "op-device-attach-do",
            OpDeviceAttachDo::lookup,
        );
        res
    }
    #[doc = "Detach the local backing device from a DRBD device.\n\nFlags: admin-perm\n\nRequest attributes:\n- [.nested_context()](PushDrbd2::nested_context)\n- [.nested_detach_parms()](PushDrbd2::nested_detach_parms)\n\nReply attributes:\n- [.get_state_result()](IterableDrbd2::get_state_result)\n- [.get_message()](IterableDrbd2::get_message)\n\n"]
    pub fn op_device_detach_do(self) -> OpDeviceDetachDo<'buf> {
        let mut res = OpDeviceDetachDo::new(self);
        res.request.do_writeback(
            res.protocol(),
            "op-device-detach-do",
            OpDeviceDetachDo::lookup,
        );
        res
    }
    #[doc = "Change disk options. Applies to the attached backing disk; see\ndevice-attach.\n\nFlags: admin-perm\n\nRequest attributes:\n- [.nested_context()](PushDrbd2::nested_context)\n- [.push_set_defaults()](PushDrbd2::push_set_defaults)\n- [.nested_disk_conf()](PushDrbd2::nested_disk_conf)\n\nReply attributes:\n- [.get_state_result()](IterableDrbd2::get_state_result)\n- [.get_message()](IterableDrbd2::get_message)\n\n"]
    pub fn op_disk_set_do(self) -> OpDiskSetDo<'buf> {
        let mut res = OpDiskSetDo::new(self);
        res.request
            .do_writeback(res.protocol(), "op-disk-set-do", OpDiskSetDo::lookup);
        res
    }
    #[doc = "Resize the DRBD device.\n\nFlags: admin-perm\n\nRequest attributes:\n- [.nested_context()](PushDrbd2::nested_context)\n- [.nested_resize_parms()](PushDrbd2::nested_resize_parms)\n\nReply attributes:\n- [.get_state_result()](IterableDrbd2::get_state_result)\n- [.get_message()](IterableDrbd2::get_message)\n\n"]
    pub fn op_device_resize_do(self) -> OpDeviceResizeDo<'buf> {
        let mut res = OpDeviceResizeDo::new(self);
        res.request.do_writeback(
            res.protocol(),
            "op-device-resize-do",
            OpDeviceResizeDo::lookup,
        );
        res
    }
    #[doc = "Mark the local data as outdated.\n\nFlags: admin-perm\n\nRequest attributes:\n- [.nested_context()](PushDrbd2::nested_context)\n\nReply attributes:\n- [.get_state_result()](IterableDrbd2::get_state_result)\n- [.get_message()](IterableDrbd2::get_message)\n\n"]
    pub fn op_device_outdate_do(self) -> OpDeviceOutdateDo<'buf> {
        let mut res = OpDeviceOutdateDo::new(self);
        res.request.do_writeback(
            res.protocol(),
            "op-device-outdate-do",
            OpDeviceOutdateDo::lookup,
        );
        res
    }
    #[doc = "Invalidate the local data of a DRBD device.\n\nFlags: admin-perm\n\nRequest attributes:\n- [.nested_context()](PushDrbd2::nested_context)\n- [.nested_invalidate_parms()](PushDrbd2::nested_invalidate_parms)\n\nReply attributes:\n- [.get_state_result()](IterableDrbd2::get_state_result)\n- [.get_message()](IterableDrbd2::get_message)\n\n"]
    pub fn op_device_invalidate_do(self) -> OpDeviceInvalidateDo<'buf> {
        let mut res = OpDeviceInvalidateDo::new(self);
        res.request.do_writeback(
            res.protocol(),
            "op-device-invalidate-do",
            OpDeviceInvalidateDo::lookup,
        );
        res
    }
    #[doc = "Generate a new current UUID.\n\nFlags: admin-perm\n\nRequest attributes:\n- [.nested_context()](PushDrbd2::nested_context)\n- [.nested_new_current_uuid_parms()](PushDrbd2::nested_new_current_uuid_parms)\n\nReply attributes:\n- [.get_state_result()](IterableDrbd2::get_state_result)\n- [.get_message()](IterableDrbd2::get_message)\n\n"]
    pub fn op_device_new_current_uuid_do(self) -> OpDeviceNewCurrentUuidDo<'buf> {
        let mut res = OpDeviceNewCurrentUuidDo::new(self);
        res.request.do_writeback(
            res.protocol(),
            "op-device-new-current-uuid-do",
            OpDeviceNewCurrentUuidDo::lookup,
        );
        res
    }
    #[doc = "Add a new peer to a resource.\n\nFlags: admin-perm\n\nRequest attributes:\n- [.nested_context()](PushDrbd2::nested_context)\n- [.nested_net_conf()](PushDrbd2::nested_net_conf)\n\nReply attributes:\n- [.get_state_result()](IterableDrbd2::get_state_result)\n- [.get_message()](IterableDrbd2::get_message)\n\n"]
    pub fn op_connection_new_do(self) -> OpConnectionNewDo<'buf> {
        let mut res = OpConnectionNewDo::new(self);
        res.request.do_writeback(
            res.protocol(),
            "op-connection-new-do",
            OpConnectionNewDo::lookup,
        );
        res
    }
    #[doc = "Delete a peer from a resource.\n\nFlags: admin-perm\n\nRequest attributes:\n- [.nested_context()](PushDrbd2::nested_context)\n- [.nested_disconnect_parms()](PushDrbd2::nested_disconnect_parms)\n\nReply attributes:\n- [.get_state_result()](IterableDrbd2::get_state_result)\n- [.get_message()](IterableDrbd2::get_message)\n\n"]
    pub fn op_connection_del_do(self) -> OpConnectionDelDo<'buf> {
        let mut res = OpConnectionDelDo::new(self);
        res.request.do_writeback(
            res.protocol(),
            "op-connection-del-do",
            OpConnectionDelDo::lookup,
        );
        res
    }
    #[doc = "Connect to a peer.\n\nFlags: admin-perm\n\nRequest attributes:\n- [.nested_context()](PushDrbd2::nested_context)\n- [.nested_connect_parms()](PushDrbd2::nested_connect_parms)\n\nReply attributes:\n- [.get_state_result()](IterableDrbd2::get_state_result)\n- [.get_message()](IterableDrbd2::get_message)\n\n"]
    pub fn op_connection_connect_do(self) -> OpConnectionConnectDo<'buf> {
        let mut res = OpConnectionConnectDo::new(self);
        res.request.do_writeback(
            res.protocol(),
            "op-connection-connect-do",
            OpConnectionConnectDo::lookup,
        );
        res
    }
    #[doc = "Disconnect from a peer.\n\nFlags: admin-perm\n\nRequest attributes:\n- [.nested_context()](PushDrbd2::nested_context)\n- [.nested_disconnect_parms()](PushDrbd2::nested_disconnect_parms)\n\nReply attributes:\n- [.get_state_result()](IterableDrbd2::get_state_result)\n- [.get_message()](IterableDrbd2::get_message)\n\n"]
    pub fn op_connection_disconnect_do(self) -> OpConnectionDisconnectDo<'buf> {
        let mut res = OpConnectionDisconnectDo::new(self);
        res.request.do_writeback(
            res.protocol(),
            "op-connection-disconnect-do",
            OpConnectionDisconnectDo::lookup,
        );
        res
    }
    #[doc = "Change network options.\n\nFlags: admin-perm\n\nRequest attributes:\n- [.nested_context()](PushDrbd2::nested_context)\n- [.push_set_defaults()](PushDrbd2::push_set_defaults)\n- [.nested_net_conf()](PushDrbd2::nested_net_conf)\n\nReply attributes:\n- [.get_state_result()](IterableDrbd2::get_state_result)\n- [.get_message()](IterableDrbd2::get_message)\n\n"]
    pub fn op_connection_set_do(self) -> OpConnectionSetDo<'buf> {
        let mut res = OpConnectionSetDo::new(self);
        res.request.do_writeback(
            res.protocol(),
            "op-connection-set-do",
            OpConnectionSetDo::lookup,
        );
        res
    }
    #[doc = "Forget a previously connected peer. The peer is identified by\ncontext.peer-node-id.\n\nFlags: admin-perm\n\nRequest attributes:\n- [.nested_context()](PushDrbd2::nested_context)\n\nReply attributes:\n- [.get_state_result()](IterableDrbd2::get_state_result)\n- [.get_message()](IterableDrbd2::get_message)\n\n"]
    pub fn op_connection_forget_do(self) -> OpConnectionForgetDo<'buf> {
        let mut res = OpConnectionForgetDo::new(self);
        res.request.do_writeback(
            res.protocol(),
            "op-connection-forget-do",
            OpConnectionForgetDo::lookup,
        );
        res
    }
    #[doc = "Add a new network path to a connection.\n\nFlags: admin-perm\n\nRequest attributes:\n- [.nested_context()](PushDrbd2::nested_context)\n\nReply attributes:\n- [.get_state_result()](IterableDrbd2::get_state_result)\n- [.get_message()](IterableDrbd2::get_message)\n\n"]
    pub fn op_path_new_do(self) -> OpPathNewDo<'buf> {
        let mut res = OpPathNewDo::new(self);
        res.request
            .do_writeback(res.protocol(), "op-path-new-do", OpPathNewDo::lookup);
        res
    }
    #[doc = "Delete a network path from a connection.\n\nFlags: admin-perm\n\nRequest attributes:\n- [.nested_context()](PushDrbd2::nested_context)\n\nReply attributes:\n- [.get_state_result()](IterableDrbd2::get_state_result)\n- [.get_message()](IterableDrbd2::get_message)\n\n"]
    pub fn op_path_del_do(self) -> OpPathDelDo<'buf> {
        let mut res = OpPathDelDo::new(self);
        res.request
            .do_writeback(res.protocol(), "op-path-del-do", OpPathDelDo::lookup);
        res
    }
    #[doc = "Change peer device options.\n\nFlags: admin-perm\n\nRequest attributes:\n- [.nested_context()](PushDrbd2::nested_context)\n- [.push_set_defaults()](PushDrbd2::push_set_defaults)\n- [.nested_peer_device_conf()](PushDrbd2::nested_peer_device_conf)\n\nReply attributes:\n- [.get_state_result()](IterableDrbd2::get_state_result)\n- [.get_message()](IterableDrbd2::get_message)\n\n"]
    pub fn op_peer_device_set_do(self) -> OpPeerDeviceSetDo<'buf> {
        let mut res = OpPeerDeviceSetDo::new(self);
        res.request.do_writeback(
            res.protocol(),
            "op-peer-device-set-do",
            OpPeerDeviceSetDo::lookup,
        );
        res
    }
    #[doc = "Invalidate the peer data of a DRBD peer device.\n\nFlags: admin-perm\n\nRequest attributes:\n- [.nested_context()](PushDrbd2::nested_context)\n- [.nested_invalidate_peer_parms()](PushDrbd2::nested_invalidate_peer_parms)\n\nReply attributes:\n- [.get_state_result()](IterableDrbd2::get_state_result)\n- [.get_message()](IterableDrbd2::get_message)\n\n"]
    pub fn op_peer_device_invalidate_do(self) -> OpPeerDeviceInvalidateDo<'buf> {
        let mut res = OpPeerDeviceInvalidateDo::new(self);
        res.request.do_writeback(
            res.protocol(),
            "op-peer-device-invalidate-do",
            OpPeerDeviceInvalidateDo::lookup,
        );
        res
    }
    #[doc = "Pause resynchronization.\n\nFlags: admin-perm\n\nRequest attributes:\n- [.nested_context()](PushDrbd2::nested_context)\n\nReply attributes:\n- [.get_state_result()](IterableDrbd2::get_state_result)\n- [.get_message()](IterableDrbd2::get_message)\n\n"]
    pub fn op_peer_device_pause_sync_do(self) -> OpPeerDevicePauseSyncDo<'buf> {
        let mut res = OpPeerDevicePauseSyncDo::new(self);
        res.request.do_writeback(
            res.protocol(),
            "op-peer-device-pause-sync-do",
            OpPeerDevicePauseSyncDo::lookup,
        );
        res
    }
    #[doc = "Resume resynchronization.\n\nFlags: admin-perm\n\nRequest attributes:\n- [.nested_context()](PushDrbd2::nested_context)\n\nReply attributes:\n- [.get_state_result()](IterableDrbd2::get_state_result)\n- [.get_message()](IterableDrbd2::get_message)\n\n"]
    pub fn op_peer_device_resume_sync_do(self) -> OpPeerDeviceResumeSyncDo<'buf> {
        let mut res = OpPeerDeviceResumeSyncDo::new(self);
        res.request.do_writeback(
            res.protocol(),
            "op-peer-device-resume-sync-do",
            OpPeerDeviceResumeSyncDo::lookup,
        );
        res
    }
    #[doc = "Start online verification.\n\nFlags: admin-perm\n\nRequest attributes:\n- [.nested_context()](PushDrbd2::nested_context)\n- [.nested_start_ov_parms()](PushDrbd2::nested_start_ov_parms)\n\nReply attributes:\n- [.get_state_result()](IterableDrbd2::get_state_result)\n- [.get_message()](IterableDrbd2::get_message)\n\n"]
    pub fn op_peer_device_start_ov_do(self) -> OpPeerDeviceStartOvDo<'buf> {
        let mut res = OpPeerDeviceStartOvDo::new(self);
        res.request.do_writeback(
            res.protocol(),
            "op-peer-device-start-ov-do",
            OpPeerDeviceStartOvDo::lookup,
        );
        res
    }
    #[doc = "Get the timeout type for a peer device.\n\nRequest attributes:\n- [.nested_context()](PushDrbd2::nested_context)\n\nReply attributes:\n- [.get_timeout_type()](IterableDrbd2::get_timeout_type)\n\n"]
    pub fn op_timeout_type_get_do(self) -> OpTimeoutTypeGetDo<'buf> {
        let mut res = OpTimeoutTypeGetDo::new(self);
        res.request.do_writeback(
            res.protocol(),
            "op-timeout-type-get-do",
            OpTimeoutTypeGetDo::lookup,
        );
        res
    }
    #[doc = "Dump all DRBD resources. Optionally filtered by context.\n\nRequest attributes:\n- [.nested_context()](PushDrbd2::nested_context)\n\nReply attributes:\n- [.get_resource()](IterableDrbd2::get_resource)\n\n"]
    pub fn op_resource_get_dump(self) -> OpResourceGetDump<'buf> {
        let mut res = OpResourceGetDump::new(self);
        res.request.do_writeback(
            res.protocol(),
            "op-resource-get-dump",
            OpResourceGetDump::lookup,
        );
        res
    }
    #[doc = "Dump all DRBD devices. Optionally filtered by context.\n\nRequest attributes:\n- [.nested_context()](PushDrbd2::nested_context)\n\nReply attributes:\n- [.get_device()](IterableDrbd2::get_device)\n\n"]
    pub fn op_device_get_dump(self) -> OpDeviceGetDump<'buf> {
        let mut res = OpDeviceGetDump::new(self);
        res.request.do_writeback(
            res.protocol(),
            "op-device-get-dump",
            OpDeviceGetDump::lookup,
        );
        res
    }
    #[doc = "Dump all DRBD connections. Optionally filtered by context.\n\nRequest attributes:\n- [.nested_context()](PushDrbd2::nested_context)\n\nReply attributes:\n- [.get_connection()](IterableDrbd2::get_connection)\n\n"]
    pub fn op_connection_get_dump(self) -> OpConnectionGetDump<'buf> {
        let mut res = OpConnectionGetDump::new(self);
        res.request.do_writeback(
            res.protocol(),
            "op-connection-get-dump",
            OpConnectionGetDump::lookup,
        );
        res
    }
    #[doc = "Dump all DRBD peer devices. Optionally filtered by context.\n\nRequest attributes:\n- [.nested_context()](PushDrbd2::nested_context)\n\nReply attributes:\n- [.get_peer_device()](IterableDrbd2::get_peer_device)\n\n"]
    pub fn op_peer_device_get_dump(self) -> OpPeerDeviceGetDump<'buf> {
        let mut res = OpPeerDeviceGetDump::new(self);
        res.request.do_writeback(
            res.protocol(),
            "op-peer-device-get-dump",
            OpPeerDeviceGetDump::lookup,
        );
        res
    }
    #[doc = "Dump all DRBD paths. Optionally filtered by context.\n\nRequest attributes:\n- [.nested_context()](PushDrbd2::nested_context)\n\nReply attributes:\n- [.get_path()](IterableDrbd2::get_path)\n\n"]
    pub fn op_path_get_dump(self) -> OpPathGetDump<'buf> {
        let mut res = OpPathGetDump::new(self);
        res.request
            .do_writeback(res.protocol(), "op-path-get-dump", OpPathGetDump::lookup);
        res
    }
    #[doc = "Dump a consistent snapshot of all resources, devices, connections, peer\ndevices and paths as state-change messages with action exists.\n\nReply attributes:\n- [.get_action()](IterableStateChange::get_action)\n- [.get_more()](IterableStateChange::get_more)\n- [.get_resource()](IterableStateChange::get_resource)\n- [.get_device()](IterableStateChange::get_device)\n- [.get_connection()](IterableStateChange::get_connection)\n- [.get_peer_device()](IterableStateChange::get_peer_device)\n- [.get_path()](IterableStateChange::get_path)\n\n"]
    pub fn op_state_get_dump(self) -> OpStateGetDump<'buf> {
        let mut res = OpStateGetDump::new(self);
        res.request
            .do_writeback(res.protocol(), "op-state-get-dump", OpStateGetDump::lookup);
        res
    }
    #[doc = "Write the bitmaps and the superblock to the meta data device, so that a\nsnapshot of the meta data taken while I/O stays suspended carries every\nchanged block. Requires I/O suspended and drained, no peer write still\nexecuting, no resync running, and a disk that is at least Outdated.\n\nFlags: admin-perm\n\nRequest attributes:\n- [.nested_context()](PushDrbd2::nested_context)\n\nReply attributes:\n- [.get_state_result()](IterableDrbd2::get_state_result)\n- [.get_message()](IterableDrbd2::get_message)\n\n"]
    pub fn op_device_cbt_flush_do(self) -> OpDeviceCbtFlushDo<'buf> {
        let mut res = OpDeviceCbtFlushDo::new(self);
        res.request.do_writeback(
            res.protocol(),
            "op-device-cbt-flush-do",
            OpDeviceCbtFlushDo::lookup,
        );
        res
    }
    #[doc = "Clear the changed block tracking bitmap and start a new tracking\ninterval at a bookmark. Allocates the tracking bitmap slot if needed.\nRequires a device-cbt-flush within the same suspension of I/O and no\nwrite to the backing device since.\n\nFlags: admin-perm\n\nRequest attributes:\n- [.nested_context()](PushDrbd2::nested_context)\n- [.nested_cbt_start_parms()](PushDrbd2::nested_cbt_start_parms)\n\nReply attributes:\n- [.get_state_result()](IterableDrbd2::get_state_result)\n- [.get_message()](IterableDrbd2::get_message)\n\n"]
    pub fn op_device_cbt_start_do(self) -> OpDeviceCbtStartDo<'buf> {
        let mut res = OpDeviceCbtStartDo::new(self);
        res.request.do_writeback(
            res.protocol(),
            "op-device-cbt-start-do",
            OpDeviceCbtStartDo::lookup,
        );
        res
    }
    #[doc = "Stop changed block tracking and free its bitmap slot.\n\nFlags: admin-perm\n\nRequest attributes:\n- [.nested_context()](PushDrbd2::nested_context)\n\nReply attributes:\n- [.get_state_result()](IterableDrbd2::get_state_result)\n- [.get_message()](IterableDrbd2::get_message)\n\n"]
    pub fn op_device_cbt_stop_do(self) -> OpDeviceCbtStopDo<'buf> {
        let mut res = OpDeviceCbtStopDo::new(self);
        res.request.do_writeback(
            res.protocol(),
            "op-device-cbt-stop-do",
            OpDeviceCbtStopDo::lookup,
        );
        res
    }
    #[doc = "Get the changed block tracking state of a device.\n\nRequest attributes:\n- [.nested_context()](PushDrbd2::nested_context)\n\nReply attributes:\n- [.get_cbt_info()](IterableDrbd2::get_cbt_info)\n\n"]
    pub fn op_device_cbt_status_do(self) -> OpDeviceCbtStatusDo<'buf> {
        let mut res = OpDeviceCbtStatusDo::new(self);
        res.request.do_writeback(
            res.protocol(),
            "op-device-cbt-status-do",
            OpDeviceCbtStatusDo::lookup,
        );
        res
    }
}
#[cfg(test)]
mod generated_tests {
    use super::*;
    #[test]
    fn tests() {
        let _ = IterableDrbd2::get_cbt_info;
        let _ = IterableDrbd2::get_connection;
        let _ = IterableDrbd2::get_context;
        let _ = IterableDrbd2::get_device;
        let _ = IterableDrbd2::get_helper;
        let _ = IterableDrbd2::get_message;
        let _ = IterableDrbd2::get_path;
        let _ = IterableDrbd2::get_peer_device;
        let _ = IterableDrbd2::get_resource;
        let _ = IterableDrbd2::get_state_result;
        let _ = IterableDrbd2::get_timeout_type;
        let _ = IterableStateChange::get_action;
        let _ = IterableStateChange::get_connection;
        let _ = IterableStateChange::get_device;
        let _ = IterableStateChange::get_more;
        let _ = IterableStateChange::get_path;
        let _ = IterableStateChange::get_peer_device;
        let _ = IterableStateChange::get_resource;
        let _ = OpHelperNotif;
        let _ = OpStateChangeNotif;
        let _ = PushDrbd2::<&mut Vec<u8>>::nested_attach_parms;
        let _ = PushDrbd2::<&mut Vec<u8>>::nested_cbt_start_parms;
        let _ = PushDrbd2::<&mut Vec<u8>>::nested_connect_parms;
        let _ = PushDrbd2::<&mut Vec<u8>>::nested_context;
        let _ = PushDrbd2::<&mut Vec<u8>>::nested_detach_parms;
        let _ = PushDrbd2::<&mut Vec<u8>>::nested_device_conf;
        let _ = PushDrbd2::<&mut Vec<u8>>::nested_disconnect_parms;
        let _ = PushDrbd2::<&mut Vec<u8>>::nested_disk_conf;
        let _ = PushDrbd2::<&mut Vec<u8>>::nested_invalidate_parms;
        let _ = PushDrbd2::<&mut Vec<u8>>::nested_invalidate_peer_parms;
        let _ = PushDrbd2::<&mut Vec<u8>>::nested_net_conf;
        let _ = PushDrbd2::<&mut Vec<u8>>::nested_new_current_uuid_parms;
        let _ = PushDrbd2::<&mut Vec<u8>>::nested_peer_device_conf;
        let _ = PushDrbd2::<&mut Vec<u8>>::nested_rename_parms;
        let _ = PushDrbd2::<&mut Vec<u8>>::nested_resize_parms;
        let _ = PushDrbd2::<&mut Vec<u8>>::nested_resource_opts;
        let _ = PushDrbd2::<&mut Vec<u8>>::nested_set_role_parms;
        let _ = PushDrbd2::<&mut Vec<u8>>::nested_start_ov_parms;
        let _ = PushDrbd2::<&mut Vec<u8>>::nested_suspend_io_parms;
        let _ = PushDrbd2::<&mut Vec<u8>>::push_set_defaults;
    }
}
