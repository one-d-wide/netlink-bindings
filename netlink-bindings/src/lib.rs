// Readme is run-tested as part of netlink-socket crate
#![allow(clippy::doc_lazy_continuation)]
#![cfg_attr(not(doctest), doc = include_str!("../README.md"))]
#![cfg_attr(docsrs, feature(doc_cfg))]

mod primitives;

pub mod builtin;
pub mod consts;
pub mod traits;
pub mod utils;

// ```fish
// cat proto \
//     | while read i
//         echo
//         echo "#[cfg(feature = \"$i\")]"
//         echo "#[cfg_attr(docsrs, doc(cfg(feature = \"$i\")))]"
//         if string match -qr ".*-.*" -- $i
//             echo "#[path = \"$i/mod.rs\"]"
//         end
//         echo "pub mod $(string replace - _ -- $i);"
//     end
// ```

#[cfg(feature = "conntrack")]
#[cfg_attr(docsrs, doc(cfg(feature = "conntrack")))]
pub mod conntrack;

// #[cfg(feature = "devlink")]
// pub mod devlink;
//
// #[cfg(feature = "dpll")]
// pub mod dpll;
//
// #[cfg(feature = "ethtool")]
// pub mod ethtool;
//
// #[cfg(feature = "fou")]
// pub mod fou;
//
// #[cfg(feature = "handshake")]
// pub mod handshake;
//
// #[cfg(feature = "lockd")]
// pub mod lockd;
//
// #[cfg(feature = "mptcp_pm")]
// pub mod mptcp_pm;
//
// #[cfg(feature = "net-shaper")]
// #[path = "net-shaper/mod.rs"]
// pub mod net_shaper;
//
// #[cfg(feature = "netdev")]
// pub mod netdev;
//
// #[cfg(feature = "nfsd")]
// pub mod nfsd;

#[cfg(feature = "nftables")]
#[cfg_attr(docsrs, doc(cfg(feature = "nftables")))]
pub mod nftables;

#[cfg(feature = "nl80211")]
#[cfg_attr(docsrs, doc(cfg(feature = "nl80211")))]
pub mod nl80211;

#[cfg(feature = "nlctrl")]
#[cfg_attr(docsrs, doc(cfg(feature = "nlctrl")))]
pub mod nlctrl;

// #[cfg(feature = "ovpn")]
// pub mod ovpn;
//
// #[cfg(feature = "ovs_datapath")]
// pub mod ovs_datapath;
//
// #[cfg(feature = "ovs_flow")]
// pub mod ovs_flow;
//
// #[cfg(feature = "ovs_vport")]
// pub mod ovs_vport;

#[cfg(feature = "rt-addr")]
#[cfg_attr(docsrs, doc(cfg(feature = "rt-addr")))]
#[path = "rt-addr/mod.rs"]
pub mod rt_addr;

#[cfg(feature = "rt-link")]
#[cfg_attr(docsrs, doc(cfg(feature = "rt-link")))]
#[path = "rt-link/mod.rs"]
pub mod rt_link;

#[cfg(feature = "rt-neigh")]
#[cfg_attr(docsrs, doc(cfg(feature = "rt-neigh")))]
#[path = "rt-neigh/mod.rs"]
pub mod rt_neigh;

#[cfg(feature = "rt-route")]
#[cfg_attr(docsrs, doc(cfg(feature = "rt-route")))]
#[path = "rt-route/mod.rs"]
pub mod rt_route;

#[cfg(feature = "rt-rule")]
#[cfg_attr(docsrs, doc(cfg(feature = "rt-rule")))]
#[path = "rt-rule/mod.rs"]
pub mod rt_rule;

#[cfg(feature = "tc")]
#[cfg_attr(docsrs, doc(cfg(feature = "tc")))]
pub mod tc;

// #[cfg(feature = "tcp_metrics")]
// pub mod tcp_metrics;
//
// #[cfg(feature = "team")]
// pub mod team;

#[cfg(feature = "wireguard")]
#[cfg_attr(docsrs, doc(cfg(feature = "wireguard")))]
pub mod wireguard;
