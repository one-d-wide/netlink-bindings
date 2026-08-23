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
fn ip_addr_show_and_list() {
    // Testing command aliases
    test_show!("ip addr show");
    test_show!("ip addr list");
    test_show!("ip addr show");
    test_show!("ip addr list");
    test_show!("ip addr");
}

#[test]
fn ip_addr_add_basic() {
    test_mod!("ip addr add 192.168.1.2/24 dev lo");
    test_mod!("ip addr add 10.0.0.1/8 dev lo");
    test_mod!("ip addr add 2001:db8::1/64 dev lo");
}

#[test]
fn ip_addr_add_complex() {
    // Peer-to-peer configurations
    test_mod!("ip addr add 192.168.1.2 peer 192.168.1.3 dev lo");
    test_mod!("ip addr add 2001:db8::1 peer 2001:db8::2 dev lo");

    // Broadcast and anycast attributes
    test_mod!("ip addr add 192.168.1.2/24 broadcast 192.168.1.255 dev lo");
    test_mod!("ip addr add 192.168.1.2/24 anycast 192.168.1.1 dev lo");

    // Labels, scopes, metrics, and protocols
    test_mod!("ip addr add 192.168.1.2/24 dev lo label lo:1");
    test_mod!("ip addr add 192.168.1.2/24 dev lo scope link");
    test_mod!("ip addr add 192.168.1.2/24 dev lo scope host");
    test_mod!("ip addr add 192.168.1.2/24 dev lo scope 150");
    test_mod!("ip addr add 192.168.1.2/24 dev lo metric 100");
    test_mod!("ip addr add 192.168.1.2/24 dev lo proto 4");

    // Full property aggregation
    test_mod!(
        "ip addr add 192.168.10.1/24 peer 192.168.10.2 broadcast 192.168.10.255 anycast 192.168.10.10 label lo:2 scope link metric 42 proto 32 dev lo"
    );
}

#[test]
fn ip_addr_add_lifetimes_and_flags() {
    // Lifetime parameters
    test_mod!("ip addr add 192.168.1.2/24 dev lo valid_lft 3600 preferred_lft 1800");
    test_mod!("ip addr add 192.168.1.2/24 dev lo valid_lft forever preferred_lft forever");

    // Config flags
    test_mod!("ip addr add ::1 dev lo home");
    test_mod!("ip addr add ::1 dev lo nodad");
    test_mod!("ip addr add ::1 dev lo mngtmpaddr");
    test_mod!("ip addr add ::1 dev lo noprefixroute");
    test_mod!("ip addr add ff00::1 dev lo autojoin");

    // Combined lifetimes and config flags
    test_mod!(
        "ip addr add ff00::1 dev lo valid_lft 600 preferred_lft 300 home nodad mngtmpaddr noprefixroute autojoin"
    );
}

#[test]
fn ip_addr_change_replace() {
    // addr replacement and modification commands
    test_mod!("ip addr change 192.168.1.2/24 dev lo valid_lft 1200 preferred_lft 600");
    test_mod!("ip addr change ::1 dev lo nodad");

    test_mod!("ip addr replace 192.168.1.2/24 dev lo valid_lft 1200 preferred_lft 600");
    test_mod!("ip addr replace ::1 dev lo nodad");
}

#[test]
fn ip_addr_del() {
    // Deleting addres
    test_mod!("ip addr del 192.168.1.2/24 dev lo");
    test_mod!("ip addr del 2001:db8::1/64 dev lo mngtmpaddr");
}
