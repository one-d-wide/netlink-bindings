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

        let l = testbed::trace_cmd_str($str);

        let ctx = testbed::trace_start();
        let req = ip!($str $(, $args )* );
        let mut iter = sock.request(&req).unwrap();
        while let Ok(Some(_)) = iter.recv().transpose() {}
        let r = testbed::trace_stop(ctx);

        dbg!($str);
        dbg!(&r);
        dbg!(&l);

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
        let l = testbed::trace_cmd_str($str);

        let ctx = testbed::trace_start();
        let req = ip!($str $(, $args )* );
        let mut sock = NetlinkSocket::new();
        let mut iter = sock.request(&req).unwrap();
        let _ = iter.recv_ack();
        let r = testbed::trace_stop(ctx);

        testbed::compare_recurse_slice(&l, &r);
    }};
}

// The code below was taken from an LLM without much thought, don't hesitate replacing it

#[test]
fn ip_rule_list_queries() {
    test_show!("ip rule");
    test_show!("ip rule list");
    test_show!("ip rule show");

    test_show!("ip -4 rule show");
    test_show!("ip -6 rule list");
}

#[test]
fn ip_rule_basic_add_del() {
    test_mod!("ip rule add from 192.168.10.0/24 table 100");
    test_mod!("ip rule del from 192.168.10.0/24 table 100");

    test_mod!("ip -6 rule add to 2001:db8::/32 table 200");
    test_mod!("ip -6 rule del to 2001:db8::/32 table 200");

    test_mod!("ip rule add from 10.1.1.5 table main");
    test_mod!("ip rule add from 10.1.1.6 table local");
    test_mod!("ip rule add from 10.1.1.7 table default");
}

#[test]
fn ip_rule_selectors() {
    test_mod!("ip rule add not from 192.168.1.0/24 table 10");

    test_mod!("ip rule add tos 0x10 table main");
    test_mod!("ip rule add tos 0x16 table 15");

    test_mod!("ip rule add fwmark 0x100 table 100");
    test_mod!("ip rule add fwmark 0x200/0xff table 200");

    test_mod!("ip rule add iif lo table 10");
    test_mod!("ip rule add oif lo table 20");

    test_mod!("ip rule add from 192.168.50.0/24 pref 500 table main");
    test_mod!("ip rule add from 192.168.60.0/24 priority 1000 table 20");

    test_mod!("ip rule add l3mdev");

    test_mod!("ip rule add uidrange 1000-2000 table main");

    test_mod!("ip rule add ipproto tcp table 80");

    test_mod!("ip rule add ipproto tcp sport 80 table 80");
    test_mod!("ip rule add ipproto udp dport 1000-2000 table 90");
    test_mod!("ip rule add ipproto tcp sport 0x50/0xff table 100");

    test_mod!("ip rule add dscp 0x04 table 40");
    test_mod!("ip rule add dscp 4/0x10 table 41");
    test_mod!("ip rule add dscp 4/0x1f table 41");
    test_mod!("ip rule add dscp 4/0x3f table 41");
    test_mod!("ip rule add dscp 0xff/0x1f table 41");
    test_mod!("ip rule add dscp 0xff/0x3f table 41");
    test_mod!("ip rule add flowlabel 0x12 table 50");
}

#[test]
fn ip_rule_actions() {
    test_mod!("ip rule add from 172.16.0.0/12 protocol static table 100");
    test_mod!("ip rule add from 172.16.0.0/12 protocol 4 table 100");

    test_mod!("ip rule add from 10.200.0.0/16 goto 10000");
}

#[test]
fn ip_rule_suppressors() {
    test_mod!("ip rule add from 192.168.100.0/24 table main suppress_prefixlength 24");

    test_mod!("ip rule add from 192.168.101.0/24 table main suppress_ifgroup 1");
}

#[test]
fn ip_rule_complex_scenarios() {
    test_mod!("ip rule add pref 11000 iif lo oif lo uidrange 1000-1005 lookup 100");

    test_mod!(
        "ip -6 rule add not from 2001:db8::/64 fwmark 0x4/0xf ipproto tcp dport 443 lookup 300"
    );

    test_mod!("ip rule add pref 20000 from 10.0.0.0/8 table default suppress_prefixlength 8");
}
