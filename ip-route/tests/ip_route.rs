mod testbed;
use ip_route::ip;
use netlink_socket2::NetlinkSocket;
use std::sync::LazyLock;

static SETUP: LazyLock<()> = LazyLock::new(|| {
    testbed::setup();

    let mut sock = NetlinkSocket::new();
    ip!(sock, "ip link set dev lo up").unwrap();
    ip!(sock, "ip route add default dev lo").ok();
});

macro_rules! test_show {
    ($str:literal $(, $args:expr )*) => {{
        scopeguard::defer_on_unwind! {
            eprintln!("LEFT: command, RIGHT: this crate");
            eprintln!("While checking {:?}", $str);
        };

        *SETUP;
        let l = testbed::trace_cmd_str($str);

        let ctx = testbed::trace_start();
        let req = ip!($str $(, $args )* );
        let mut sock = NetlinkSocket::new();
        let mut iter = sock.request(&req).unwrap();
        while let Some(_) = iter.recv() {}
        let r = testbed::trace_stop(ctx);

        testbed::compare_recurse_slice(&l, &r);
    }};
}

macro_rules! test_add {
    ($str:literal $(, $args:expr )* $(,)?) => {{
        scopeguard::defer_on_unwind! {
            eprintln!("LEFT: command, RIGHT: this crate");
            eprintln!("While checking {:?}", $str);
        };

        *SETUP;
        let l = testbed::trace_cmd_str($str);

        let ctx = testbed::trace_start();
        let req = ip!($str $(, $args )* );
        let mut sock = NetlinkSocket::new();
        let mut iter = sock.request(&req).unwrap();
        let _ = iter.recv_ack();
        let r = testbed::trace_stop(ctx);

        // dbg!($str);
        // dbg!(&r);
        // dbg!(&l);

        testbed::compare_recurse_slice(&l, &r);
    }};
}

// The code below was taken from an LLM without much thought, don't hesitate replacing it

#[test]
fn ip_route_mixed() {
    test_show!("ip route get 1.2.3.4");
    test_show!("ip route show");
    test_show!("ip route show table main");
    test_show!("ip route show table 0xbeef");
    test_add!("ip route add 192.168.1.0/24 dev lo");
    test_add!("ip route del 192.168.1.0/24 dev lo");
    test_add!("ip route change 192.168.1.0/24 dev lo");
    test_add!("ip route append 192.168.1.0/24 dev lo");
    test_add!("ip route replace 192.168.1.0/24 dev lo");
    test_add!("ip route add default table 1234");
    test_add!("ip route add default table 123 oif lo via 127.0.0.2 src 127.0.0.1");
}

#[test]
fn ip_route_add_basic() {
    // Basic unicast destination prefixes and devices
    test_add!("ip route add 192.168.1.0/24 dev lo");
    test_add!("ip route add 10.0.0.0/8 via 10.0.0.1");
    test_add!("ip route add default via 192.168.1.1");
    test_add!("ip route add default via 192.168.1.1 dev lo");
    test_add!("ip route add 172.16.0.0/12 dev lo src 172.16.0.2");
    test_add!("ip route add 192.0.2.1/32 dev lo");

    // IPv6 destinations
    test_add!("ip -6 route add 2001:db8::/32 dev lo");
    test_add!("ip -6 route add default via fe80::1 dev lo");
    test_add!("ip -6 route add 2001:db8:1::/64 via 2001:db8::1");
    test_add!("ip -6 route add 3001:db8::1 dev lo src 3001:db8::2");
}

#[test]
fn ip_route_add_tables_and_metadata() {
    // Table specifiers
    test_add!("ip route add 10.1.0.0/16 via 10.0.0.1 table 100");
    test_add!("ip route add 10.2.0.0/16 via 10.0.0.1 table main");
    test_add!("ip route add 10.3.0.0/16 via 10.0.0.1 table local");
    test_add!("ip route add 10.4.0.0/16 via 10.0.0.1 table default");

    // Routing protocol specifiers
    test_add!("ip route add 10.5.0.0/16 via 10.0.0.1 proto static");
    test_add!("ip route add 10.6.0.0/16 via 10.0.0.1 proto boot");
    test_add!("ip route add 10.7.0.0/16 via 10.0.0.1 proto kernel");
    test_add!("ip route add 10.8.0.0/16 via 10.0.0.1 proto 42");

    // Metric and priority parameters
    test_add!("ip route add 10.9.0.0/16 via 10.0.0.1 metric 500");
    test_add!("ip route add 10.10.0.0/16 via 10.0.0.1 pref 30");

    // Scope specifiers
    test_add!("ip route add 10.11.0.0/16 via 10.0.0.1 scope global");
    test_add!("ip route add 10.12.0.0/16 via 10.0.0.1 scope link");
    test_add!("ip route add 10.13.0.0/16 via 10.0.0.1 scope host");
    test_add!("ip route add 10.14.0.0/16 via 10.0.0.1 scope 200");

    // TTL propagation options
    test_add!("ip route add 10.15.0.0/16 via 10.0.0.1 ttl-propagate enabled");
    test_add!("ip route add 10.16.0.0/16 via 10.0.0.1 ttl-propagate disabled");
}

#[test]
fn ip_route_add_types() {
    // Special routing types
    test_add!("ip route add unicast 10.100.0.0/16 dev lo");
    test_add!("ip route add local 192.168.1.100 dev lo");
    test_add!("ip route add broadcast 192.168.1.255 dev lo");
    test_add!("ip route add multicast 224.0.0.0/4 dev lo");
    test_add!("ip route add throw 10.101.0.0/16");
    test_add!("ip route add unreachable 10.102.0.0/16");
    test_add!("ip route add prohibit 10.103.0.0/16");
    test_add!("ip route add blackhole 10.104.0.0/16");
    test_add!("ip route add nat 10.105.0.0/16 via 192.168.1.1");
}

#[test]
fn ip_route_add_metrics_and_features() {
    // MTU options
    test_add!("ip route add 10.50.0.0/16 via 10.0.0.1 mtu 1400");
    test_add!("ip route add 10.52.0.0/16 via 10.0.0.1 advmss 1360");

    // TCP/IP performance and window parameters
    test_add!("ip route add 10.53.0.0/16 via 10.0.0.1 rtt 100ms");
    test_add!("ip route add 10.55.0.0/16 via 10.0.0.1 rttvar 50");
    test_add!("ip route add 10.56.0.0/16 via 10.0.0.1 window 65535");
    test_add!("ip route add 10.57.0.0/16 via 10.0.0.1 cwnd 10");
    test_add!("ip route add 10.58.0.0/16 via 10.0.0.1 ssthresh 50");
    test_add!("ip route add 10.60.0.0/16 via 10.0.0.1 rto_min 200ms");
    test_add!("ip route add 10.61.0.0/16 via 10.0.0.1 initcwnd 20");
    test_add!("ip route add 10.62.0.0/16 via 10.0.0.1 initrwnd 20");

    // Advanced protocol properties
    test_add!("ip route add 10.63.0.0/16 via 10.0.0.1 features ecn");
    test_add!("ip route add 10.64.0.0/16 via 10.0.0.1 quickack 1");
    test_add!("ip route add 10.65.0.0/16 via 10.0.0.1 quickack 0");
    test_add!("ip route add 10.66.0.0/16 via 10.0.0.1 congctl cubic");
    test_add!("ip route add 10.67.0.0/16 via 10.0.0.1 congctl bbr");
    test_add!("ip route add 10.68.0.0/16 via 10.0.0.1 expires 3600");
    test_add!("ip route add 10.69.0.0/16 via 10.0.0.1 fastopen_no_cookie 1");

    // IPv6 Specific route preferences
    test_add!("ip -6 route add 2001:db8:10::/64 via 2001:db8::1 pref low");
    test_add!("ip -6 route add 2001:db8:11::/64 via 2001:db8::1 pref medium");
    test_add!("ip -6 route add 2001:db8:12::/64 via 2001:db8::1 pref high");
}

#[test]
fn ip_route_add_complex() {
    // Highly specific combined routing scenarios
    test_add!("ip route add blackhole 10.120.0.0/16 table 200 proto static metric 1000");
    test_add!(
        "ip route add 192.168.50.0/24 via 192.168.1.254 dev lo table 50 proto static scope link mtu 1420 quickack 1"
    );
    test_add!(
        "ip -6 route add 2001:db8:50::/64 via 2001:db8::fe dev lo table 10 proto boot metric 10 pref 10 ttl-propagate enabled pref high expires 7200"
    );
}

#[test]
fn ip_route_get_basic() {
    // Basic IPv4 & IPv6 destinations
    test_show!("ip route get 1.1.1.1");
    test_show!("ip route get 8.8.8.8");
    test_show!("ip route get 127.0.0.1");
    test_show!("ip route get 0.0.0.0");
    test_show!("ip route get 255.255.255.255");
    test_show!("ip route get ::1");
    test_show!("ip route get 2001:db8::1");
    test_show!("ip route get fe80::1");

    // Optional 'to' keyword prefixing the destination address
    test_show!("ip route get to 1.1.1.1");
    test_show!("ip route get to 8.8.8.8");
    test_show!("ip route get to ::1");
    test_show!("ip route get to 2001:db8::1");
}

#[test]
fn ip_route_get_route_flags() {
    // Testing fibmatch, notify
    test_show!("ip route get fibmatch 1.1.1.1");
    test_show!("ip route get notify 1.1.1.1");
    test_show!("ip route get fibmatch notify 1.1.1.1");
    test_show!("ip route get fibmatch to 1.1.1.1");
    test_show!("ip route get notify to 1.1.1.1");
}

#[test]
fn ip_route_get_from_and_iif() {
    // Source address selection and incoming interface context
    test_show!("ip route get 8.8.8.8 from 192.168.1.100");
    test_show!("ip route get 8.8.8.8 from 10.0.0.5 iif lo");
    test_show!("ip route get 2001:db8::1 from 2001:db8::100 iif lo");
    test_show!("ip route get to 8.8.8.8 from 192.168.1.100 iif lo");
}

#[test]
fn ip_route_get_oif() {
    // Forcing output interface selection
    test_show!("ip route get 8.8.8.8 oif lo");
    test_show!("ip route get 8.8.8.8 oif lo");
    test_show!("ip route get 8.8.8.8 oif lo");
    test_show!("ip route get 2001:db8::1 oif lo");
    test_show!("ip route get to 10.0.0.1 oif lo");
}

#[test]
fn ip_route_get_mark() {
    // Firewall routing marks (decimal and hex)
    test_show!("ip route get 8.8.8.8 mark 100");
    test_show!("ip route get 8.8.8.8 mark 0x100");
    test_show!("ip route get 2001:db8::1 mark 42");
    test_show!("ip route get to 8.8.8.8 from 192.168.1.1 mark 0xff00");
}

#[test]
fn ip_route_get_tos() {
    // DSField / Type of Service routing options
    test_show!("ip route get 8.8.8.8 tos 0x16");
    test_show!("ip route get 8.8.8.8 tos 0x10");
    test_show!("ip route get 2001:db8::1 tos 0x20");
}

#[test]
fn ip_route_get_transport_details() {
    // IP protocol-specific routing
    test_show!("ip route get 8.8.8.8 ipproto tcp");
    test_show!("ip route get 8.8.8.8 ipproto udp");
    test_show!("ip route get 8.8.8.8 ipproto icmp");
    test_show!("ip route get 8.8.8.8 ipproto 17"); // Protocol by number (UDP)

    // Port-based routing parameters (requires ipproto specified first)
    test_show!("ip route get 8.8.8.8 ipproto tcp sport 12345 dport 80");
    test_show!("ip route get 8.8.8.8 ipproto udp sport 53 dport 53");
    test_show!("ip route get 2001:db8::1 ipproto tcp sport 443 dport 54321");
    test_show!("ip route get to 8.8.8.8 from 192.168.1.100 ipproto tcp sport 1024 dport 22");
}

#[test]
fn ip_route_get_as_translation() {
    // Destination address translation simulation (the 'as' flag)
    test_show!("ip route get 8.8.8.8 as 10.0.0.1");
    test_show!("ip route get 1.1.1.1 as 192.168.2.1");
    test_show!("ip route get 2001:db8::1 as 2001:db8::99");
}

#[test]
fn ip_route_get_flowlabel() {
    // IPv6 flowlabel parameters
    test_show!("ip route get 2001:db8::1 flowlabel 123");
    test_show!("ip route get 2001:db8::1 flowlabel 0xabcde");
    test_show!("ip route get to 2001:db8::1 from 2001:db8::2 flowlabel 1");
}

#[test]
fn ip_route_get_uid() {
    test_show!("ip route get 8.8.8.8 uid 1000");
    test_show!("ip route get 8.8.8.8 uid 0");
}

#[test]
fn ip_route_get_complex_combinations() {
    // Testing multiple route lookup parameters combined
    test_show!("ip route get fibmatch 8.8.8.8 from 192.168.1.100 iif lo oif lo mark 0x5");
    test_show!("ip route get 2001:db8::1 from 2001:db8::2 iif lo oif lo flowlabel 0x100 uid 1000");
    test_show!(
        "ip route get notify 8.8.8.8 from 10.0.0.2 oif lo tos 0x8 ipproto tcp sport 80 dport 8080"
    );
    test_show!(
        "ip route get fibmatch notify 192.168.1.1 from 192.168.1.50 iif lo oif lo mark 42 ipproto udp sport 123 dport 123"
    );
}
