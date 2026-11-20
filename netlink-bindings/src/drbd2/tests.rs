use crate::utils::*;

use crate::drbd2;
use crate::drbd2::*;

use std::net::Ipv6Addr;

/// `drbdsetup new-path test 1 ipv6 [2001:db8::1]:7000 ipv6 [2001:db8::2]:7000`
/// as sent by the C drbdsetup. The addresses are in network byte order.
#[cfg_attr(any(), rustfmt::skip)]
const NEW_PATH_IPV6: &[u8] = &[
    0x19, 0x01, 0x00, 0x00, // genlmsghdr: cmd=path-new version=1
    0x68, 0x00, 0x01, 0x80, // context
    0x09, 0x00, 0x01, 0x00, 0x74, 0x65, 0x73, 0x74, 0x00, 0x00, 0x00, 0x00, // resource-name "test"
    0x08, 0x00, 0x04, 0x00, 0x01, 0x00, 0x00, 0x00, // peer-node-id 1
    0x28, 0x00, 0x06, 0x80, // my-address
    0x06, 0x00, 0x01, 0x00, 0x0a, 0x00, 0x00, 0x00, // family AF_INET6
    0x06, 0x00, 0x02, 0x00, 0x1b, 0x58, 0x00, 0x00, // port 7000 (big-endian)
    0x14, 0x00, 0x04, 0x00, // ipv6 2001:db8::1
    0x20, 0x01, 0x0d, 0xb8, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01,
    0x28, 0x00, 0x07, 0x80, // peer-address
    0x06, 0x00, 0x01, 0x00, 0x0a, 0x00, 0x00, 0x00, // family AF_INET6
    0x06, 0x00, 0x02, 0x00, 0x1b, 0x58, 0x00, 0x00, // port 7000 (big-endian)
    0x14, 0x00, 0x04, 0x00, // ipv6 2001:db8::2
    0x20, 0x01, 0x0d, 0xb8, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x02,
];

const MY_ADDR: Ipv6Addr = Ipv6Addr::new(0x2001, 0xdb8, 0, 0, 0, 0, 0, 1);
const PEER_ADDR: Ipv6Addr = Ipv6Addr::new(0x2001, 0xdb8, 0, 0, 0, 0, 0, 2);

#[test]
fn encode_ipv6_network_order() {
    let mut req = drbd2::Request::new().op_path_new_do();
    req.encode()
        .nested_context()
        .push_resource_name_bytes(b"test")
        .push_peer_node_id(1)
        .nested_my_address()
        .push_family(libc::AF_INET6 as u16)
        .push_port(7000)
        .push_ipv6(MY_ADDR)
        .end_nested()
        .nested_peer_address()
        .push_family(libc::AF_INET6 as u16)
        .push_port(7000)
        .push_ipv6(PEER_ADDR)
        .end_nested()
        .end_nested();

    dump_hex(req.payload());
    assert_eq!(req.payload(), NEW_PATH_IPV6);
}

#[test]
fn decode_ipv6_network_order() {
    dump_hex(NEW_PATH_IPV6);
    let ctx = OpPathNewDo::decode_request(NEW_PATH_IPV6)
        .get_context()
        .unwrap();

    assert_eq!(ctx.get_my_address().unwrap().get_ipv6(), Ok(MY_ADDR));
    assert_eq!(ctx.get_peer_address().unwrap().get_ipv6(), Ok(PEER_ADDR));
}
