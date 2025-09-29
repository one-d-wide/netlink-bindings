// Taken from libc crate

// linux/netlink.h
pub const NLA_ALIGNTO: i32 = 4;

pub const NETLINK_ROUTE: i32 = 0;
pub const NETLINK_UNUSED: i32 = 1;
pub const NETLINK_USERSOCK: i32 = 2;
pub const NETLINK_FIREWALL: i32 = 3;
pub const NETLINK_SOCK_DIAG: i32 = 4;
pub const NETLINK_NFLOG: i32 = 5;
pub const NETLINK_XFRM: i32 = 6;
pub const NETLINK_SELINUX: i32 = 7;
pub const NETLINK_ISCSI: i32 = 8;
pub const NETLINK_AUDIT: i32 = 9;
pub const NETLINK_FIB_LOOKUP: i32 = 10;
pub const NETLINK_CONNECTOR: i32 = 11;
pub const NETLINK_NETFILTER: i32 = 12;
pub const NETLINK_IP6_FW: i32 = 13;
pub const NETLINK_DNRTMSG: i32 = 14;
pub const NETLINK_KOBJECT_UEVENT: i32 = 15;
pub const NETLINK_GENERIC: i32 = 16;
pub const NETLINK_SCSITRANSPORT: i32 = 18;
pub const NETLINK_ECRYPTFS: i32 = 19;
pub const NETLINK_RDMA: i32 = 20;
pub const NETLINK_CRYPTO: i32 = 21;
pub const NETLINK_INET_DIAG: i32 = NETLINK_SOCK_DIAG;

pub const NLM_F_REQUEST: i32 = 1;
pub const NLM_F_MULTI: i32 = 2;
pub const NLM_F_ACK: i32 = 4;
pub const NLM_F_ECHO: i32 = 8;
pub const NLM_F_DUMP_INTR: i32 = 16;
pub const NLM_F_DUMP_FILTERED: i32 = 32;

pub const NLM_F_ROOT: i32 = 0x100;
pub const NLM_F_MATCH: i32 = 0x200;
pub const NLM_F_ATOMIC: i32 = 0x400;
pub const NLM_F_DUMP: i32 = NLM_F_ROOT | NLM_F_MATCH;

pub const NLM_F_REPLACE: i32 = 0x100;
pub const NLM_F_EXCL: i32 = 0x200;
pub const NLM_F_CREATE: i32 = 0x400;
pub const NLM_F_APPEND: i32 = 0x800;

pub const NLM_F_NONREC: i32 = 0x100;
pub const NLM_F_BULK: i32 = 0x200;

pub const NLM_F_CAPPED: i32 = 0x100;
pub const NLM_F_ACK_TLVS: i32 = 0x200;

pub const NETLINK_ADD_MEMBERSHIP: i32 = 1;
pub const NETLINK_DROP_MEMBERSHIP: i32 = 2;
pub const NETLINK_PKTINFO: i32 = 3;
pub const NETLINK_BROADCAST_ERROR: i32 = 4;
pub const NETLINK_NO_ENOBUFS: i32 = 5;
pub const NETLINK_RX_RING: i32 = 6;
pub const NETLINK_TX_RING: i32 = 7;
pub const NETLINK_LISTEN_ALL_NSID: i32 = 8;
pub const NETLINK_LIST_MEMBERSHIPS: i32 = 9;
pub const NETLINK_CAP_ACK: i32 = 10;
pub const NETLINK_EXT_ACK: i32 = 11;
pub const NETLINK_GET_STRICT_CHK: i32 = 12;

pub const NLA_F_NESTED: i32 = 1 << 15;
pub const NLA_F_NET_BYTEORDER: i32 = 1 << 14;
pub const NLA_TYPE_MASK: i32 = !(NLA_F_NESTED | NLA_F_NET_BYTEORDER);
