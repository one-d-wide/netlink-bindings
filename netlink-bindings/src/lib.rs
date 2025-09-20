pub mod primitives;
pub mod utils;

// ```fish
// cat proto \
//     | while read i
//         echo
//         echo "#[cfg(any(feature = \"all\", feature = \"$i\"))]"
//         if string match -qr ".*-.*" -- $i
//             echo "#[path = \"$i/mod.rs\"]"
//         end
//         echo "pub mod $(string replace - _ -- $i);"
//     end
// ```

#[cfg(any(feature = "all", feature = "conntrack"))]
pub mod conntrack;

// #[cfg(any(feature = "all", feature = "devlink"))]
// pub mod devlink;
//
// #[cfg(any(feature = "all", feature = "dpll"))]
// pub mod dpll;
//
// #[cfg(any(feature = "all", feature = "ethtool"))]
// pub mod ethtool;
//
// #[cfg(any(feature = "all", feature = "fou"))]
// pub mod fou;
//
// #[cfg(any(feature = "all", feature = "handshake"))]
// pub mod handshake;
//
// #[cfg(any(feature = "all", feature = "lockd"))]
// pub mod lockd;
//
// #[cfg(any(feature = "all", feature = "mptcp_pm"))]
// pub mod mptcp_pm;
//
// #[cfg(any(feature = "all", feature = "net-shaper"))]
// #[path = "net-shaper/mod.rs"]
// pub mod net_shaper;
//
// #[cfg(any(feature = "all", feature = "netdev"))]
// pub mod netdev;
//
// #[cfg(any(feature = "all", feature = "nfsd"))]
// pub mod nfsd;
//
// #[cfg(any(feature = "all", feature = "nftables"))]
// pub mod nftables;
//
// #[cfg(any(feature = "all", feature = "nl80211"))]
// pub mod nl80211;
//
// #[cfg(any(feature = "all", feature = "nlctrl"))]
// pub mod nlctrl;
//
// #[cfg(any(feature = "all", feature = "ovpn"))]
// pub mod ovpn;
//
// #[cfg(any(feature = "all", feature = "ovs_datapath"))]
// pub mod ovs_datapath;
//
// #[cfg(any(feature = "all", feature = "ovs_flow"))]
// pub mod ovs_flow;
//
// #[cfg(any(feature = "all", feature = "ovs_vport"))]
// pub mod ovs_vport;

#[cfg(any(feature = "all", feature = "rt-addr"))]
#[path = "rt-addr/mod.rs"]
pub mod rt_addr;

#[cfg(any(feature = "all", feature = "rt-link"))]
#[path = "rt-link/mod.rs"]
pub mod rt_link;

#[cfg(any(feature = "all", feature = "rt-neigh"))]
#[path = "rt-neigh/mod.rs"]
pub mod rt_neigh;

#[cfg(any(feature = "all", feature = "rt-route"))]
#[path = "rt-route/mod.rs"]
pub mod rt_route;

#[cfg(any(feature = "all", feature = "rt-rule"))]
#[path = "rt-rule/mod.rs"]
pub mod rt_rule;

#[cfg(any(feature = "all", feature = "tc"))]
pub mod tc;

// #[cfg(any(feature = "all", feature = "tcp_metrics"))]
// pub mod tcp_metrics;
//
// #[cfg(any(feature = "all", feature = "team"))]
// pub mod team;

#[cfg(any(feature = "all", feature = "wireguard"))]
pub mod wireguard;
