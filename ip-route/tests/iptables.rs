mod testbed;
use ::ip_route_proc_macro::{ip, ip_dump};
use ip_route::utils;
use netlink_bindings::nftables;
use netlink_socket2::NetlinkSocket;
use std::{io::ErrorKind, sync::LazyLock};

static SETUP: LazyLock<()> = LazyLock::new(|| {
    testbed::setup();
    let mut sock = NetlinkSocket::new();

    for fam in [libc::AF_INET, libc::AF_INET6] {
        // Table "filter" may not exist...
        let mut req = nftables::Request::new()
            .set_create()
            .op_newtable_do(&nftables::Nfgenmsg {
                nfgen_family: fam as u8,
                ..Default::default()
            });
        req.encode().push_name_bytes(b"filter");
        utils::iptables_request(&mut sock, &req).unwrap();
    }

    match ip!(sock, "iptables -N test") {
        Err(err) if err.as_io_error().kind() == ErrorKind::AlreadyExists => {}
        res => res.unwrap(),
    };
    match ip!(sock, "ip6tables -N test") {
        Err(err) if err.as_io_error().kind() == ErrorKind::AlreadyExists => {}
        res => res.unwrap(),
    };
});

macro_rules! test_list {
    ($str:literal $(, $args:expr )*) => {{
        *SETUP;
        scopeguard::defer_on_unwind! {
            eprintln!("LEFT: command, RIGHT: this crate");
            eprintln!("While checking {:?}", $str);
        };

        let mut sock = NetlinkSocket::new();

        let l = testbed::trace_cmd_str($str);

        let ctx = testbed::trace_start();
        let req = ip!($str $(, $args )* );
        let mut iter = sock.request(&req).unwrap();
        while let Ok(Some(_)) = iter.recv().transpose() {}
        let r = testbed::trace_stop(ctx);

        testbed::compare_recurse_slice(&l, &r);
    }};
}

macro_rules! test_mod {
    ($str:literal $(, $args:expr )* $(,)?) => {{
        *SETUP;
        scopeguard::defer_on_unwind! {
            eprintln!("LEFT: command, RIGHT: this crate");
            eprintln!("While checking {:?}", $str);
        };

        let l = testbed::trace_cmd_str($str);

        let ctx = testbed::trace_start();
        let req = ip!($str $(, $args )* );
        let mut sock = NetlinkSocket::new();
        ip_route::utils::iptables_request(&mut sock, &req).unwrap();
        let r = testbed::trace_stop(ctx);

        testbed::compare_recurse_slice(&l, &r);
    }};
}

#[test]
fn iptables_sock() {
    *SETUP;
    let mut sock = NetlinkSocket::new();
    ip!(sock, "iptables -F test").unwrap();
    let mut iter = ip_dump!(sock, "iptables -L test").unwrap();
    while let Some(_) = iter.recv().transpose().unwrap() {}
}

#[test]
fn iptables() {
    test_mod!("iptables -I test -p tcp -j ACCEPT");

    test_mod!("iptables -F test");
    test_mod!("ip6tables -F test");

    test_mod!("iptables -I test -p tcp -j ACCEPT");
    test_mod!("ip6tables -I test -p tcp -j ACCEPT");

    test_list!("iptables -L test");
    test_list!("ip6tables -L test");

    test_mod!("iptables -I test -p udp -m udp --sport 1 -j ACCEPT");
    test_mod!("iptables -I test -p tcp -m tcp ! --dport 123:1234 -j ACCEPT");

    test_mod!("iptables -I test -m mark --mark 0xff -j ACCEPT");
    test_mod!("iptables -I test -m mark --mark 123/12 -j ACCEPT");

    test_mod!("iptables -I test -j LOG");
    test_mod!("iptables -I test -j LOG --log-prefix asd");
    test_mod!("iptables -I test -j LOG --log-prefix asd --log-ip-options --log-level error");

    test_mod!("iptables -t nat -I OUTPUT -s 1.2.3.4 -j DNAT --to-destination 1.2.3.4");
    test_mod!(
        "iptables -t nat -I OUTPUT -s 1.2.3.4 -p tcp -j DNAT --to-destination 1.2.3.4-2.2.3.4:1234-1235/1236 --persistent"
    );
}
