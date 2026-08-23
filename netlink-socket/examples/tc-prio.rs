//! This example demonstrates basic usage of the API for traffic control
//! queueing disciplines. Given an existing network device (`wg0` by default),
//! it creates, displays, and deletes a tc prio qdisc with 8 bands, equivalent
//! to the following:
//!
//! ```sh
//! tc qdisc add dev wg0 root handle 0xbeef: prio bands 8 priomap 7 6 5 4 3 2 1 0
//! tc qdisc show dev wg0
//! tc qdisc del dev wg0 root
//! ```
//!
//! Run with: `cargo run --example tc-prio --features=tc,rt-link [-- ifname]`

use std::error::Error;

use netlink_bindings::{
    rt_link,
    tc::{self, TcPrioQopt, Tcmsg},
};
use netlink_socket2::NetlinkSocket;

const TC_H_ROOT: u32 = 0xffffffff;

#[cfg_attr(not(feature = "async"), maybe_async::must_be_sync)]
#[cfg_attr(feature = "tokio", tokio::main(flavor = "current_thread"))]
#[cfg_attr(feature = "smol", macro_rules_attribute::apply(smol_macros::main))]
async fn main() -> Result<(), Box<dyn Error>> {
    let ifname = std::env::args().skip(1).next().unwrap_or("wg0".to_string());

    let mut sock = NetlinkSocket::new();
    let ifi = link_get_ifindex(&mut sock, &ifname).await?;

    // Handle id consists of major:minor parts, each is 16 bits.
    // Qdisc handle has only the major part, minor is used by classes.
    let qdisc_handle = 0xbeef << 16;

    tc_prio_add(&mut sock, ifi, qdisc_handle).await?;
    tc_prio_show(&mut sock, ifi, qdisc_handle).await?;
    tc_prio_del(&mut sock, ifi, qdisc_handle).await?;

    Ok(())
}

#[cfg_attr(not(feature = "async"), maybe_async::must_be_sync)]
async fn tc_prio_add(
    sock: &mut NetlinkSocket,
    ifi: i32,
    handle: u32,
) -> Result<(), Box<dyn Error>> {
    let header = Tcmsg {
        family: 0,
        ifindex: ifi,
        handle: handle,
        parent: TC_H_ROOT,
        ..Default::default()
    };

    let mut req = tc::Request::new()
        .set_create()
        .set_excl()
        .op_newqdisc_do(&header);

    let priomap: [u8; 16] = [7, 6, 5, 4, 3, 2, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0];

    let tc_prio_opt = TcPrioQopt {
        bands: 8,
        priomap: priomap,
    };

    req.encode().nested_options_prio(&tc_prio_opt);

    sock.request(&req).await?.recv_ack().await?;

    println!("tc prio add on ifi {} OK", ifi);

    Ok(())
}

#[cfg_attr(not(feature = "async"), maybe_async::must_be_sync)]
async fn tc_prio_show(
    sock: &mut NetlinkSocket,
    ifi: i32,
    qdisc_handle: u32,
) -> Result<(), Box<dyn Error>> {
    let header = Tcmsg {
        family: 0,
        ifindex: ifi,
        handle: qdisc_handle,
        parent: TC_H_ROOT,
        info: 0,
        ..Default::default()
    };

    let req = tc::Request::new().op_getqdisc_dump(&header);

    let mut iter = sock.request(&req).await?;
    while let Some((header, attrs)) = iter.recv().await.transpose()? {
        if header.ifindex == ifi && header.handle == qdisc_handle {
            println!("{:#?}", (header, attrs));
            println!("tc prio show on ifi {} OK", ifi);
            return Ok(());
        }
    }

    panic!("No qdisc found");
}

#[cfg_attr(not(feature = "async"), maybe_async::must_be_sync)]
async fn tc_prio_del(
    sock: &mut NetlinkSocket,
    ifi: i32,
    qdisc_handle: u32,
) -> Result<(), Box<dyn Error>> {
    let req = tc::Request::new().op_delqdisc_do(&Tcmsg {
        family: 0,
        ifindex: ifi,
        handle: qdisc_handle,
        parent: TC_H_ROOT,
        info: 0,
        ..Default::default()
    });

    sock.request(&req).await?.recv_ack().await?;

    println!("tc prio del on ifi {} OK", ifi);

    Ok(())
}

#[cfg_attr(not(feature = "async"), maybe_async::must_be_sync)]
pub async fn link_get_ifindex(
    sock: &mut NetlinkSocket,
    ifname: &str,
) -> Result<i32, Box<dyn Error>> {
    let mut request = rt_link::Request::new().op_getlink_do(&Default::default());
    request.encode().push_ifname_bytes(ifname.as_bytes());

    let mut iter = sock.request(&request).await?;
    let (header, _attrs) = iter.recv_one().await?;

    Ok(header.ifi_index)
}
