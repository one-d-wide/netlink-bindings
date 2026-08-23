use ip_route::ip;

#[test]
fn test_substitutions() {
    let tos = 123;
    let addr = "1.2.3.4".parse::<std::net::IpAddr>().unwrap();
    let dev = "lo";

    ip!("ip route get tos {tos} to {addr} oif {dev}");
    ip!("ip route get tos {} to {addr} oif {}", tos, dev);
    ip!("ip route get tos {} to {} oif {}", tos, addr, dev);
}

#[test]
fn test_conditionals() {
    let oif = 123;
    let mark = 1234;
    let src = Some("1.2.3.4".parse().unwrap());
    let gateway = Some("1.2.3.4".parse().unwrap());
    let ipv4 = true;

    ip!(
        "ip [?ipv4: -4 :][: -6 :] route add default table {mark} [?oif>=100: oif-index {oif} src {src:option} :] via {gateway:option}"
    );
}

#[test]
fn test_comands_compile() {
    #[allow(unused)]
    #[allow(unreachable_code)]
    fn only_compile() {
        // Using include!() here because rustc doesn't provide any context for
        // errors originating inside the proc-macro.
        // The path is usually ./target/tmp/tests.rs
        include!(ip_route_proc_macro::_write_integration_tests!());
    }
}
