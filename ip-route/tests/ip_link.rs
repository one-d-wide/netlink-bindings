mod testbed;
use ip_route::ip;
use netlink_socket2::NetlinkSocket;
use std::sync::LazyLock;

static SETUP: LazyLock<()> = LazyLock::new(|| {
    testbed::setup();
});

macro_rules! test_show {
    ($str:literal $(, $args:expr )*) => {{
        scopeguard::defer_on_unwind! {
            eprintln!("LEFT: command, RIGHT: this crate");
            eprintln!("While checking {:?}", $str);
        };

        *SETUP;
        let mut sock = NetlinkSocket::new();

        let l = testbed::trace_cmd_str(&format!($str $(, $args )*));

        let ctx = testbed::trace_start();
        let req = ip!($str $(, $args )* );
        let mut iter = sock.request(&req).unwrap();
        while let Some(_) = iter.recv() {}
        let r = testbed::trace_stop(ctx);

        testbed::compare_recurse_slice(&l, &r);
    }};
}

macro_rules! test_mod {
    ($str:literal $(, $args:expr )* $(,)?) => {{
        scopeguard::defer_on_unwind! {
            eprintln!("LEFT: command, RIGHT: this crate");
            eprintln!("While checking {:?}", $str);
        };

        *SETUP;
        let mut sock = NetlinkSocket::new();

        let l = testbed::trace_cmd_str(&format!($str $(, $args )*));

        let ctx = testbed::trace_start();
        let req = ip!($str $(, $args )* );
        let mut iter = sock.request(&req).unwrap();
        let _ = iter.recv_ack();
        let r = testbed::trace_stop(ctx);

        testbed::compare_recurse_slice(&l, &r);
    }};
}

// The code below was taken from an LLM without much thought, don't hesitate replacing it

#[test]
fn ip_link_show_and_list() {
    test_show!("ip link show");
    test_show!("ip link list");

    test_show!("ip link show dev lo");
    test_show!("ip link show lo");

    test_show!("ip link show type dummy");
    test_show!("ip link show type bridge");
    test_show!("ip link show type veth");

    test_show!("ip link show master lo");
    test_show!("ip link show nomaster");
}

#[test]
fn ip_link_add_del_basic() {
    test_mod!("ip link add dev lo type dummy");
    test_mod!("ip link add name lo type dummy");
    test_mod!("ip link add lo type dummy");

    test_mod!(
        "ip link add lo txqueuelen 1200 address 02:00:00:00:00:01 broadcast ff:ff:ff:ff:ff:ff mtu 1400 index 1001 type dummy"
    );
    test_mod!("ip link add dummy_basic5 numtxqueues 2 numrxqueues 2 type dummy");

    test_mod!("ip link del dev lo");
    test_mod!("ip link del lo");
}

/// Tests modifying standard interface attributes (`ip link set`).
#[test]
fn ip_link_set_basic() {
    test_mod!("ip link add lo type dummy");

    // State up / down
    test_mod!("ip link set dev lo up");
    test_mod!("ip link set dev lo down");

    // Interface flag modifications
    test_mod!("ip link set dev lo arp on");
    test_mod!("ip link set dev lo arp off");
    test_mod!("ip link set dev lo dynamic on");
    test_mod!("ip link set dev lo dynamic off");
    test_mod!("ip link set dev lo multicast on");
    test_mod!("ip link set dev lo multicast off");
    test_mod!("ip link set dev lo allmulticast on");
    test_mod!("ip link set dev lo allmulticast off");
    test_mod!("ip link set dev lo promisc on");
    test_mod!("ip link set dev lo promisc off");
    test_mod!("ip link set dev lo trailers on");
    test_mod!("ip link set dev lo trailers off");
    test_mod!("ip link set dev lo carrier on");
    test_mod!("ip link set dev lo carrier off");

    // Standard configuration options
    test_mod!("ip link set dev lo mtu 1450");
    test_mod!("ip link set dev lo txqueuelen 500");
    test_mod!("ip link set dev lo address 02:00:00:00:00:02");
    test_mod!("ip link set dev lo broadcast ff:ff:ff:ff:ff:fe");
    test_mod!("ip link set dev lo alias test_alias_name");

    test_mod!("ip link del lo");
}

/// Tests modifying interface parameters using the alternative `change` action (`ip link change`).
#[test]
fn ip_link_change() {
    test_mod!("ip link add lo type dummy");

    // Parameter updates using "change"
    test_mod!("ip link change dev lo mtu 1400");
    test_mod!("ip link change dev lo txqueuelen 2000");

    test_mod!("ip link del lo");
}

/// Tests creating interfaces with advanced GSO (Generic Segmentation Offload) and GRO (Generic Receive Offload) attributes.
#[test]
fn ip_link_add_gso_gro() {
    test_mod!(
        "ip link add lo gso_max_size 60000 gso_ipv4_max_size 60000 gso_max_segs 128 gro_max_size 65536 gro_ipv4_max_size 65536 type dummy"
    );
    test_mod!("ip link del lo");
}

/// Tests modifying advanced GSO and GRO attributes on existing interfaces.
#[test]
fn ip_link_set_gso_gro() {
    test_mod!("ip link add lo type dummy");

    test_mod!("ip link set dev lo gso_max_size 50000 gso_ipv4_max_size 50000 gso_max_segs 64");
    test_mod!("ip link set dev lo gro_max_size 60000 gro_ipv4_max_size 60000");

    test_mod!("ip link del lo");
}

#[test]
fn ip_link_add_bridge_basic() {
    // Basic bridge interface creation and deletion
    test_mod!("ip link add lo type bridge");
    test_mod!("ip link add name lo type bridge");
    test_mod!("ip link del dev lo");
    test_mod!("ip link del name lo type bridge");
}

#[test]
fn ip_link_add_bridge_stp_and_timers() {
    // Spanning Tree Protocol (STP) options and ageing/forward delay configurations
    test_mod!(
        "ip link add br-stp type bridge \
         stp_state 1 \
         priority 4096 \
         forward_delay 15 \
         hello_time 2 \
         max_age 20 \
         ageing_time 300"
    );

    // Additional STP and Forwarding Database (FDB) operational limits
    test_mod!(
        "ip link add br-stp-opt type bridge \
         mst_enabled 1 \
         fdb_max_learned 2048 \
         no_linklocal_learn 1 \
         fdb_local_vlan_0 1"
    );
}

#[test]
fn ip_link_add_bridge_vlan() {
    // VLAN filtering, default PVID, protocol, and stats configuration (802.1Q)
    test_mod!(
        "ip link add br-vlan type bridge \
         vlan_filtering 1 \
         vlan_protocol 129 \
         vlan_default_pvid 10 \
         vlan_stats_enabled 1 \
         vlan_stats_per_port 1"
    );

    // VLAN configuration using alternative protocols (QinQ / 802.1ad) and disabling filtering
    test_mod!(
        "ip link add br-vlan-qinq type bridge \
         vlan_filtering 0 \
         vlan_protocol 0x8100 \
         vlan_default_pvid 0 \
         vlan_stats_enabled 0"
    );
}

#[test]
fn ip_link_add_bridge_multicast() {
    // IGMP/MLD multicast snooping, querier, and routing modes
    test_mod!(
        "ip link add br-mcast type bridge \
         mcast_snooping 1 \
         mcast_router 2 \
         mcast_querier 1 \
         mcast_query_use_ifaddr 1 \
         mcast_igmp_version 3 \
         mcast_mld_version 2 \
         mcast_stats_enabled 1"
    );

    // Multicast hash limits and query count bounds
    test_mod!(
        "ip link add br-mcast-limits type bridge \
         mcast_hash_elasticity 16 \
         mcast_hash_max 4096 \
         mcast_last_member_count 2 \
         mcast_startup_query_count 2"
    );
}

#[test]
fn ip_link_add_bridge_multicast_intervals() {
    // Multicast timing/interval options (typically configured in jiffies or milliseconds)
    test_mod!(
        "ip link add br-mcast-timers type bridge \
         mcast_last_member_interval 100 \
         mcast_membership_interval 26000 \
         mcast_querier_interval 25500 \
         mcast_query_interval 12500 \
         mcast_query_response_interval 1000 \
         mcast_startup_query_interval 3125"
    );
}

#[test]
fn ip_link_add_bridge_netfilter_and_filtering() {
    // Netfilter integration (iptables/arptables) and frame-filtering overrides
    test_mod!(
        "ip link add br-nf type bridge \
         nf_call_iptables 1 \
         nf_call_ip6tables 1 \
         nf_call_arptables 0 \
         group_fwd_mask 65535 \
         group_address 01:80:c2:00:00:0e"
    );
}

#[test]
fn ip_link_set_bridge_attributes() {
    // Modifying existing bridge properties using 'ip link set'
    test_mod!("ip link set dev lo type bridge stp_state 0 hello_time 4");
    test_mod!("ip link set lo type bridge vlan_filtering 1 vlan_default_pvid 100");
    test_mod!("ip link set dev lo type bridge mcast_snooping 0 nf_call_iptables 1");
}

#[test]
fn ip_link_set_bridge_ports() {
    // Port management: binding (enslaving) and unbinding interfaces to/from a bridge
    test_mod!("ip link set dev lo master lo");
    test_mod!("ip link set dev lo nomaster");
}
