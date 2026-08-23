//! This example demonstrates how extended ACK attributes are reported on error.
//! See https://docs.kernel.org/userspace-api/netlink/intro.html#extended-ack
//!
//! Unfortunately, only validation policy violations is consistently reported,
//! and there're not many netlink families correctly reporting missing attributes.
//!
//! Run with: `cargo run --example extack --features=rt-link`

use std::error::Error;

use netlink_bindings::rt_link;

fn main() -> Result<(), Box<dyn Error>> {
    let mut request = rt_link::Request::new()
        .set_create()
        .op_newlink_do(&rt_link::Ifinfomsg::new());

    request
        .encode()
        .push_ifname(c"12345678901234567890") // Interface name is too long
        .nested_linkinfo()
        .push_kind(c"bridge");

    let mut sock = netlink_socket2::NetlinkSocket::new();

    let mut iter = sock.request(&request)?;
    while let Some(res) = iter.recv() {
        println!("{:?}", res);
    }

    Ok(())
}
