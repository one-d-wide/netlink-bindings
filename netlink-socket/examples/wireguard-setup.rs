//! This example demonstrates a standard procedure for setting up a wireguard
//! device, equivalent to the following:
//!
//! ```sh
//! ip link add dev wg-example type wireguard
//! ip addr add 10.0.0.1 dev wg-example
//! wg set wg-example peer AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA= endpoint 127.0.0.1:12345 allowed-ips 10.0.0.1/32
//! ip link del dev wg-example
//! ```
//!
//! Run with: `cargo run --example wireguard-setup --features=wireguard,rt-link,rt-addr`

use std::{
    error::Error,
    net::{IpAddr, SocketAddr},
};

use netlink_bindings::{
    rt_addr::{self, Ifaddrmsg},
    rt_link::{self, Ifinfomsg},
    wireguard,
};
use netlink_socket2::NetlinkSocket;

#[cfg_attr(not(feature = "async"), maybe_async::must_be_sync)]
#[cfg_attr(feature = "tokio", tokio::main(flavor = "current_thread"))]
#[cfg_attr(feature = "smol", macro_rules_attribute::apply(smol_macros::main))]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut sock = NetlinkSocket::new();

    let ifname = "wg-example";
    let addr: IpAddr = "10.0.0.1".parse().unwrap();
    let prefix = 32;
    let peer_key = [0u8; wireguard::KEY_LEN as usize];
    let peer: SocketAddr = "127.0.0.1:12345".parse().unwrap();

    println!("Adding {ifname:?}");
    link_add(&mut sock, ifname).await?;

    let ifindex = link_get_ifindex(&mut sock, ifname).await?;
    println!("Device {ifname:?} received ifindex {ifindex}");

    println!("Assigning {ifname:?} address {addr}/{prefix}");
    addr_add(&mut sock, ifindex, addr, prefix).await?;

    println!("Configuring wireguard parameters");
    wg_set(&mut sock, ifname, addr, peer, &peer_key[..]).await?;

    println!("Dumping wireguard devices (all)");
    wg_dump(&mut sock).await?;

    // Comment these lines to inspect manually from the console
    println!("Deleting {ifname:?}");
    link_del(&mut sock, ifname).await?;

    Ok(())
}

#[cfg_attr(not(feature = "async"), maybe_async::must_be_sync)]
async fn wg_dump(sock: &mut NetlinkSocket) -> Result<(), Box<dyn Error>> {
    let mut sock_wg = NetlinkSocket::new();

    let request_links = rt_link::Request::new().op_getlink_dump(&Ifinfomsg::new());
    let mut iter = sock.request(&request_links).await?;
    while let Some((_header, attrs)) = iter.recv().await.transpose()? {
        let link = attrs.get_ifname()?;

        if Ok(c"wireguard") != attrs.get_linkinfo().unwrap_or_default().get_kind() {
            println!("Skipping {link:?}");
            continue;
        };
        println!("Dumping {link:?}");

        let mut request = wireguard::Request::new().op_get_device_dump();
        request.encode().push_ifname(link);

        let mut iter = sock_wg.request(&request).await?;
        while let Some(attrs) = iter.recv().await.transpose()? {
            println!("{:#?}", attrs);
        }

        println!();
    }

    Ok(())
}

#[cfg_attr(not(feature = "async"), maybe_async::must_be_sync)]
async fn wg_set(
    sock: &mut NetlinkSocket,
    ifname: &str,
    addr: IpAddr,
    peer: SocketAddr,
    peer_key: &[u8],
) -> Result<(), Box<dyn Error>> {
    let mut request = wireguard::Request::new().op_set_device_do();

    request
        .encode()
        .push_ifname_bytes(ifname.as_bytes())
        .push_flags(wireguard::WgdeviceFlags::ReplacePeers as u32) // Remove existing peers
        .array_peers()
        .entry_nested()
        .push_public_key(peer_key)
        .push_endpoint(peer)
        .array_allowedips()
        .entry_nested()
        .push_family(libc_addr_family(&addr) as u16)
        .push_ipaddr(addr)
        .push_cidr_mask(32)
        .end_nested()
        .end_array()
        .end_nested()
        .end_array();

    sock.request(&request).await?.recv_ack().await?;

    Ok(())
}

#[cfg_attr(not(feature = "async"), maybe_async::must_be_sync)]
async fn addr_add(
    sock: &mut NetlinkSocket,
    ifindex: u32,
    addr: IpAddr,
    addr_prefix: u8,
) -> Result<(), Box<dyn Error>> {
    let header = Ifaddrmsg {
        ifa_family: libc_addr_family(&addr) as u8,
        ifa_index: ifindex,
        ifa_prefixlen: addr_prefix,
        ..Default::default()
    };

    let mut request = rt_addr::Request::new()
        .set_change() // Don't fail if address already assigned
        .op_newaddr_do(&header);

    request.encode().push_local(addr);

    sock.request(&request).await?.recv_ack().await?;

    Ok(())
}

/// Equivalent to `ip link add dev {ifname} type wireguard`
#[cfg_attr(not(feature = "async"), maybe_async::must_be_sync)]
async fn link_add(sock: &mut NetlinkSocket, ifname: &str) -> Result<(), Box<dyn Error>> {
    let mut request = rt_link::Request::new()
        .set_create()
        // .set_excl() // If exclusive flag set, existing device will cause an error
        .op_newlink_do(&rt_link::Ifinfomsg::new());

    request
        .encode()
        .push_ifname_bytes(ifname.as_bytes())
        .nested_linkinfo()
        .push_kind(c"wireguard");

    sock.request(&request).await?.recv_ack().await?;

    Ok(())
}

#[cfg_attr(not(feature = "async"), maybe_async::must_be_sync)]
async fn link_get_ifindex(sock: &mut NetlinkSocket, ifname: &str) -> Result<u32, Box<dyn Error>> {
    let mut request = rt_link::Request::new().op_getlink_do(&rt_link::Ifinfomsg::new());

    request.encode().push_ifname_bytes(ifname.as_bytes());

    let mut iter = sock.request(&request).await?;
    let (header, _attrs) = iter.recv_one().await?;

    Ok(header.ifi_index as u32)
}

/// Equivalent to `ip link del dev {ifname}`
#[cfg_attr(not(feature = "async"), maybe_async::must_be_sync)]
async fn link_del(sock: &mut NetlinkSocket, ifname: &str) -> Result<(), Box<dyn Error>> {
    let mut request = rt_link::Request::new().op_dellink_do(&Default::default());

    request.encode().push_ifname_bytes(ifname.as_bytes());

    sock.request(&request).await?.recv_ack().await?;

    Ok(())
}

fn libc_addr_family(addr: &IpAddr) -> i32 {
    if addr.is_ipv4() {
        libc::AF_INET
    } else {
        libc::AF_INET6
    }
}
