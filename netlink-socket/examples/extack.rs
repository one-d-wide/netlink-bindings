//! This example demonstrates how extended ACK attributes are reported on error.
//! See https://docs.kernel.org/userspace-api/netlink/intro.html#extended-ack
//!
//! Unfortunately, only validation policy violations is consistently reported,
//! and there're not many netlink families correctly reporting missing attributes.
//!
//! Run with: `cargo run --example extack --features=rt-link`

use netlink_bindings::rt_link;

#[cfg_attr(not(feature = "async"), maybe_async::maybe_async)]
#[cfg_attr(feature = "tokio", tokio::main(flavor = "current_thread"))]
#[cfg_attr(feature = "smol", macro_rules_attribute::apply(smol_macros::main))]
async fn main() {
    let header = rt_link::PushIfinfomsg::new();
    let mut request = rt_link::Request::new()
        .set_create()
        .op_newlink_do_request(&header);

    request
        .encode()
        .push_ifname(c"12345678901234567890") // Interface name is too long
        .nested_linkinfo()
        .push_kind(c"bridge");

    let mut sock = netlink_socket::NetlinkSocket::new();

    let mut iter = sock.request(&request).await.unwrap();
    while let Some(res) = iter.recv().await {
        println!("{:?}", res);
    }
}
