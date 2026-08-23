//! This example demonstrates basic interactions with nl80211.
//!
//! Unlike other families, nl80211 yaml specification doesn't fully annotate
//! each of the ~170 operations. Instead, use [`nl80211::Commands`] enum in
//! combination with `.op_do(cmd: u8)` to access the operations.
//!
//! Run with: `cargo run --example nl80211 --features=nl80211,rt-link`

use std::{collections::HashSet, error::Error};

use netlink_bindings::{
    nl80211::{self, Commands},
    rt_link,
};
use netlink_socket2::NetlinkSocket;

#[cfg_attr(not(feature = "async"), maybe_async::must_be_sync)]
#[cfg_attr(feature = "tokio", tokio::main(flavor = "current_thread"))]
#[cfg_attr(feature = "smol", macro_rules_attribute::apply(smol_macros::main))]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut sock = NetlinkSocket::new();

    let ifname = "nl80211-example";

    println!("Dumping wifi devices");
    let devices = dump_wiphy(&mut sock).await?;

    if let Some(ifindex) = get_interface_index(&mut sock, ifname).await? {
        println!("Interface {ifname:?} already exists. Removing it");
        wiphy_del_interface(&mut sock, ifindex).await?;
    }

    if devices.is_empty() {
        println!("No wifi devices found");
        return Ok(());
    }

    let (phy, phy_id) = devices.first().unwrap();

    println!("Adding {ifname:?} for phy {phy:?}");
    let ifindex = wiphy_add_interface(&mut sock, *phy_id, ifname).await?;

    println!("Removing {ifname:?}");
    wiphy_del_interface(&mut sock, ifindex).await?;

    Ok(())
}

#[cfg_attr(not(feature = "async"), maybe_async::must_be_sync)]
async fn dump_wiphy(sock: &mut NetlinkSocket) -> Result<Vec<(String, u32)>, Box<dyn Error>> {
    let mut request = nl80211::Request::new().op_dump(Commands::GetWiphy as u8);
    request
        .encode()
        // Allow the kernel to split reply into multiple chunks, each carrying
        // phy id, phy name and some attributes, since messages get quite large.
        .push_split_wiphy_dump(());

    let mut devices = HashSet::new();
    let mut request = sock.request(&request).await?;
    while let Some(attrs) = request.recv().await.transpose()? {
        let name = attrs.get_wiphy_name()?;
        let index = attrs.get_wiphy()?;

        if let Ok(mut commands) = attrs.get_supported_commands() {
            if commands.any(|c| c == Commands::NewInterface as u32) {
                devices.insert((name.to_string_lossy().to_string(), index));
            }
        }
    }

    Ok(devices.into_iter().collect())
}

#[cfg_attr(not(feature = "async"), maybe_async::must_be_sync)]
async fn wiphy_add_interface(
    sock: &mut NetlinkSocket,
    phy_id: u32,
    new_ifname: &str,
) -> Result<u32, Box<dyn Error>> {
    let mut request = nl80211::Request::new().op_do(Commands::NewInterface as u8);
    request.encode()
        .push_wiphy(phy_id)
        .push_ifname_bytes(new_ifname.as_bytes())
        .push_iftype(2) // aka managed
        // ...
        ;

    let mut iter = sock.request(&request).await?;
    let attrs = iter.recv_one().await?;

    Ok(attrs.get_ifindex()?)
}

#[cfg_attr(not(feature = "async"), maybe_async::must_be_sync)]
async fn wiphy_del_interface(sock: &mut NetlinkSocket, ifindex: u32) -> Result<(), Box<dyn Error>> {
    let mut request = nl80211::Request::new().op_do(Commands::DelInterface as u8);
    request.encode().push_ifindex(ifindex);
    sock.request(&request).await?.recv_ack().await?;
    Ok(())
}

#[cfg_attr(not(feature = "async"), maybe_async::must_be_sync)]
async fn get_interface_index(
    sock: &mut NetlinkSocket,
    ifname: &str,
) -> Result<Option<u32>, Box<dyn Error>> {
    let request = rt_link::Request::new().op_getlink_dump(&Default::default());

    let mut iter = sock.request(&request).await?;
    while let Some((header, attrs)) = iter.recv().await.transpose()? {
        if attrs.get_ifname().unwrap().to_bytes() == ifname.as_bytes() {
            return Ok(Some(header.ifi_index as u32));
        }
    }

    Ok(None)
}
