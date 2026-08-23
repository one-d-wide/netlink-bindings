//! This example dumps routing entries from the main table similar to `ip route show`.
//!
//! Run with: `cargo run --example ip-route-show --features=rt-route,rt-link`

use std::{error::Error, net::Ipv4Addr};

use netlink_bindings::{rt_link, rt_route};
use netlink_socket2::NetlinkSocket;

#[cfg_attr(not(feature = "async"), maybe_async::must_be_sync)]
#[cfg_attr(feature = "tokio", tokio::main(flavor = "current_thread"))]
#[cfg_attr(feature = "smol", macro_rules_attribute::apply(smol_macros::main))]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut sock = NetlinkSocket::new();

    let header = rt_route::Rtmsg {
        rtm_family: libc::AF_INET as u8,
        rtm_table: libc::RT_TABLE_MAIN as u8,
        ..Default::default()
    };

    let req = rt_route::Request::new().op_getroute_dump(&header);

    let mut res = sock.request(&req).await?;
    while let Some((header, attrs)) = res.recv().await.transpose()? {
        let dst = attrs.get_dst().unwrap_or(Ipv4Addr::UNSPECIFIED.into());

        println!();
        print!("{dst}/{}", header.rtm_dst_len);
        if let Ok(gateway) = attrs.get_gateway() {
            print!(" via {gateway}");
        }
        if let Ok(ifindex) = attrs.get_oif() {
            let ifname = get_ifname(ifindex).await?;
            print!(" dev {ifname}");
        }
        if let Ok(src) = attrs.get_prefsrc() {
            print!(" src {src}");
        }
        println!();

        println!("{header:?}");
        println!("{attrs:?}");
    }

    Ok(())
}

#[cfg_attr(not(feature = "async"), maybe_async::must_be_sync)]
async fn get_ifname(ifindex: u32) -> Result<String, Box<dyn Error>> {
    let req = rt_link::Request::new().op_getlink_do(&rt_link::Ifinfomsg {
        ifi_index: ifindex as i32,
        ..Default::default()
    });

    let mut sock = NetlinkSocket::new();
    let mut iter = sock.request(&req).await?;
    let (_, attrs) = iter.recv_one().await?;
    Ok(attrs.get_ifname()?.to_string_lossy().to_string())
}
