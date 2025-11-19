//! This example demonstrates basic interactions with nl80211.
//!
//! Unlike other families, nl80211 yaml specification doesn't fully annotate
//! each of the ~170 operations. Instead, use [`nl80211::Commands`] enum in
//! combination with `.op_do_request(cmd: u8)` to access the operations.
//!
//! Run with: `cargo run --example nl80211 --features=nl80211,rt-link`

use std::collections::HashSet;

use netlink_bindings::{
    nl80211::{self, Commands},
    rt_link,
};
use netlink_socket2::NetlinkSocket;

#[cfg_attr(not(feature = "async"), maybe_async::maybe_async)]
#[cfg_attr(feature = "tokio", tokio::main(flavor = "current_thread"))]
#[cfg_attr(feature = "smol", macro_rules_attribute::apply(smol_macros::main))]
async fn main() {
    let mut sock = NetlinkSocket::new();

    let ifname = "nl80211-example";

    println!("Dumping wifi devices");
    let devices = dump_wiphy(&mut sock).await;

    if let Some(ifindex) = get_interface_index(&mut sock, ifname).await {
        println!("Interface {ifname:?} already exists. Removing it");
        wiphy_del_interface(&mut sock, ifindex).await;
    }

    if devices.is_empty() {
        println!("No wifi devices found");
        return;
    }

    let (phy, phy_id) = devices.first().unwrap();

    println!("Adding {ifname:?} for phy {phy:?}");
    let ifindex = wiphy_add_interface(&mut sock, *phy_id, ifname).await;

    println!("Removing {ifname:?}");
    wiphy_del_interface(&mut sock, ifindex).await;
}

#[cfg_attr(not(feature = "async"), maybe_async::maybe_async)]
async fn dump_wiphy(sock: &mut NetlinkSocket) -> Vec<(String, u32)> {
    let mut request = nl80211::Request::new().op_dump_request(Commands::GetWiphy as u8);
    request
        .encode()
        // Allow the kernel to split reply into multiple chunks, each carrying
        // phy id, phy name and some attributes, since messages get quite large.
        .push_split_wiphy_dump(());

    let mut devices = HashSet::new();
    let mut request = sock.request(&request).await.unwrap();
    while let Some(res) = request.recv().await {
        let attrs = res.unwrap();

        let name = attrs.get_wiphy_name().unwrap();
        let index = attrs.get_wiphy().unwrap();

        if let Ok(mut commands) = attrs.get_supported_commands() {
            if commands.any(|c| c == Commands::NewInterface as u32) {
                devices.insert((name.to_string_lossy().to_string(), index));
            }
        }
    }

    devices.into_iter().collect()
}

#[cfg_attr(not(feature = "async"), maybe_async::maybe_async)]
async fn wiphy_add_interface(sock: &mut NetlinkSocket, phy_id: u32, new_ifname: &str) -> u32 {
    let mut request = nl80211::Request::new().op_do_request(Commands::NewInterface as u8);
    request.encode()
        .push_wiphy(phy_id)
        .push_ifname_bytes(new_ifname.as_bytes())
        .push_iftype(2) // aka managed
        // ...
        ;

    let mut iter = sock.request(&request).await.unwrap();
    let attrs = iter.recv_one().await.unwrap();

    attrs.get_ifindex().unwrap()
}

#[cfg_attr(not(feature = "async"), maybe_async::maybe_async)]
async fn wiphy_del_interface(sock: &mut NetlinkSocket, ifindex: u32) {
    let mut request = nl80211::Request::new().op_do_request(Commands::DelInterface as u8);
    request.encode().push_ifindex(ifindex);

    let mut iter = sock.request(&request).await.unwrap();
    iter.recv_ack().await.unwrap();
}

#[cfg_attr(not(feature = "async"), maybe_async::maybe_async)]
async fn get_interface_index(sock: &mut NetlinkSocket, ifname: &str) -> Option<u32> {
    let request = rt_link::Request::new().op_getlink_dump_request(&Default::default());

    let mut iter = sock.request(&request).await.unwrap();
    while let Some(res) = iter.recv().await {
        let (header, attrs) = res.unwrap();
        if attrs.get_ifname().unwrap().to_bytes() == ifname.as_bytes() {
            return Some(header.ifi_index() as u32);
        }
    }

    None
}
