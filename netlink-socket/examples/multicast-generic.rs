//! This example demonstrates receiving Netlink multicast notifications emitted
//! by generic netlink subsystems, aka genetlink.
//!
//! Multicast notification are quite sparsely documented, so netlink-bindings
//! only provides a "raw" socket, meaning you have to provide `group_id` to
//! listen to and later to choose how to decode received messages yourself.
//!
//! Run with: `cargo run --example multicast-generic --features=netdev,rt-link`

use std::error::Error;

use netlink_bindings::{builtin::BuiltinNfgenmsg, netdev, nlctrl, rt_link, traits::NetlinkRequest};
use netlink_socket2::{MulticastSocketRaw, NetlinkSocket, ReplyError};

fn main() -> Result<(), Box<dyn Error>> {
    let mut sock = NetlinkSocket::new();
    let mut multicast_sock = MulticastSocketRaw::new(nlctrl::PROTONUM)?;

    // Before receiving notifications, you have to subscribe to relevant groups.
    // Check the [`netdev::NotifGroup`] for available groups, or the upstream
    // `<subsystem>.yaml` specification, here notification groups are called
    // "mcast-groups" and are usually placed at the bottom of the file.
    //
    // Under the hood, .listen() calls setsockopt() with NETLINK_ADD_MEMBERSHIP.
    match resolve_genl_group_id(&mut sock, netdev::PROTONAME, netdev::NotifGroup::MGMT) {
        Ok(group_id) => multicast_sock.listen(group_id)?,
        Err(err) => {
            println!("Can't resolve group id: {err}");
            println!("Netdev notifications were added in Linux 6.3. The current kernel is older");
            if std::env::var("TESTING").is_ok()
                && err.as_io_error().kind() == std::io::ErrorKind::NotFound
            {
                return Ok(());
            }
            std::process::exit(1);
        }
    }

    // Group ids are allocated mostly sequentially, we subscribe to a bunch of
    // them blinds simply to surface notifications drifting on your system.
    //
    // Genetlink allows us to later query their names at runtime.
    for i in 0..100 {
        if multicast_sock.listen(i).is_ok() {
            let group = lookup_genl_group(&mut sock, i)?;
            println!("Group {i}: {group}");
        }
    }

    // This should emit notifications for us to process
    let link = "example-link";
    link_add(&mut sock, link)?;
    link_del(&mut sock, link)?;

    loop {
        let (recv, buf) = multicast_sock.recv()?;

        let _family_id = recv.message_type;
        let multicast_group = recv.multicast_group;

        let group = lookup_genl_group(&mut sock, multicast_group)?;

        let BuiltinNfgenmsg { cmd, version, .. } = BuiltinNfgenmsg::new_from_zeroed(buf);

        println!("Message from group={group:?} cmd={cmd:?} version={version:?}");

        // netlink_bindings::utils::dump_hex(buf);

        match group.as_str() {
            "netdev: mgmt" => {
                let attrs = netdev::OpDevGetDo::decode_reply(buf);
                dbg!(attrs);

                // Cmd is a sequential number of an operation in operation.list[]
                let op = match cmd {
                    netdev::OpDevAddNotif::CMD    /* 2 */ => Some("creating"),
                    netdev::OpDevDelNotif::CMD    /* 3 */ => Some("deleting"),
                    netdev::OpDevChangeNotif::CMD /* 4 */ => Some("changing"),
                    _ => None,
                };

                let ifindex = attrs.get_ifindex()?;
                if let Some(op) = op {
                    println!("Caught {op} device with ifindex={ifindex}");
                }

                if std::env::var("TESTING").is_ok() && op == Some("deleting") {
                    return Ok(());
                }
            }

            "netdev: page-pool" => {
                dbg!(netlink_bindings::netdev::OpPagePoolGetDo::decode_reply(buf));
            }

            // Nl80211 emits a message when you, for example, try to search for wifi networks
            _ if group.starts_with("nl80211") => {}

            // ...
            _ => {}
        }
    }
}

fn resolve_genl_group_id(
    sock: &mut NetlinkSocket,
    family: &str,
    group_name: &str,
) -> Result<u32, ReplyError> {
    let mut request = nlctrl::Request::new().op_getfamily_do();
    request.encode().push_family_name_bytes(family.as_bytes());

    let mut iter = sock.request(&request)?;
    let attrs = iter.recv_one()?;

    for group in attrs.get_mcast_groups()? {
        if group.get_name()?.to_bytes() == group_name.as_bytes() {
            return Ok(group.get_id()?);
        }
    }

    panic!("Couldn't resolve group id by group_name={group_name:?}")
}

fn lookup_genl_group(sock: &mut NetlinkSocket, group_id: u32) -> Result<String, Box<dyn Error>> {
    let request = nlctrl::Request::new().op_getfamily_dump();
    let mut iter = sock.request(&request)?;

    while let Some(attrs) = iter.recv().transpose()? {
        for group in attrs.get_mcast_groups().unwrap_or_default() {
            if group.get_id()? == group_id {
                let family = attrs.get_family_name()?.to_str()?;
                let group = group.get_name()?.to_str()?;

                return Ok(format!("{family}: {group}"));
            }
        }
    }

    Ok("(unknown)".to_string())
}

/// Equivalent to `ip link add dev {ifname} type dummy`
fn link_add(sock: &mut NetlinkSocket, ifname: &str) -> Result<(), Box<dyn Error>> {
    let mut request = rt_link::Request::new()
        .set_create()
        .set_excl()
        .op_newlink_do(&rt_link::Ifinfomsg::new());

    request
        .encode()
        .push_ifname_bytes(ifname.as_bytes())
        .nested_linkinfo()
        .push_kind(c"dummy");

    let mut iter = sock.request(&request)?;
    let _ = iter.recv_ack();
    Ok(())
}

/// Equivalent to `ip link del dev {ifname}`
fn link_del(sock: &mut NetlinkSocket, ifname: &str) -> Result<(), Box<dyn Error>> {
    let mut request = rt_link::Request::new().op_dellink_do(&Default::default());

    request.encode().push_ifname_bytes(ifname.as_bytes());

    let mut iter = sock.request(&request)?;
    let _ = iter.recv_ack();
    Ok(())
}
