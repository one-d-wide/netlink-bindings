//! An async version of `./multicast-simple.rs`, see it first.
//!
//! Run with: `cargo run --example multicast-simple --features=netdev,rt-link,tokio`

use std::error::Error;

use netlink_bindings::{builtin::BuiltinNfgenmsg, netdev, nlctrl, rt_link, traits::NetlinkRequest};
use netlink_socket2::ReplyError;

#[cfg(feature = "smol")]
use netlink_socket2::smol::{MulticastSocketRaw, NetlinkSocket};
#[cfg(feature = "tokio")]
use netlink_socket2::tokio::{MulticastSocketRaw, NetlinkSocket};

#[cfg_attr(feature = "tokio", tokio::main(flavor = "current_thread"))]
#[cfg_attr(feature = "smol", macro_rules_attribute::apply(smol_macros::main))]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut sock = NetlinkSocket::new();
    let mut multicast_sock = MulticastSocketRaw::new(nlctrl::PROTONUM)?;

    // Before receiving message, you have to subscribe to relevant groups.
    // Looking at `<subsystem>.yaml` specification, those are usuall at the
    // bottom called "mcast-groups".
    //
    // Under the hood, .listen() calls setsockopt with NETLINK_ADD_MEMBERSHIP.
    match resolve_genl_group_id(&mut sock, netdev::PROTONAME, netdev::NotifGroup::MGMT).await {
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

    // This should emit notifications for us to process
    let link = "example-link";
    link_add(&mut sock, link).await?;
    link_del(&mut sock, link).await?;

    loop {
        let (_recv, buf) = multicast_sock.recv().await?;

        let BuiltinNfgenmsg { cmd, version, .. } = BuiltinNfgenmsg::new_from_zeroed(buf);

        println!("Message cmd={cmd:?} version={version:?}");

        // netlink_bindings::utils::dump_hex(buf);

        let attrs = netdev::OpDevGetDo::decode_reply(buf);
        dbg!(attrs);

        // Cmd is a sequential number of an operation in operation.list[]
        let op = match cmd {
            netdev::OpDevAddNotif::CMD => Some("deleting"),
            netdev::OpDevDelNotif::CMD => Some("deleting"),
            netdev::OpDevChangeNotif::CMD => Some("changing"),
            _ => None,
        };

        let ifindex = attrs.get_ifindex()?;
        if let Some(op) = op {
            println!("Cought {op} device with ifindex={ifindex}");
        }

        if std::env::var("TESTING").is_ok() && op == Some("deleting") {
            return Ok(());
        }
    }
}

// Async interfaces are used here to test that async NetlinkSocket works correctly.
// Helper functions below need (and should) *not* be async, as requests in them don't incur any waiting.
async fn resolve_genl_group_id(
    sock: &mut NetlinkSocket,
    family: &str,
    group_name: &str,
) -> Result<u32, ReplyError> {
    let mut request = nlctrl::Request::new().op_getfamily_do();
    request.encode().push_family_name_bytes(family.as_bytes());

    let mut iter = sock.request(&request).await?;
    let attrs = iter.recv_one().await?;

    for group in attrs.get_mcast_groups()? {
        if group.get_name()?.to_bytes() == group_name.as_bytes() {
            return Ok(group.get_id()?);
        }
    }

    panic!("Couldn't resolve group id by group_name={group_name:?}")
}

async fn link_add(sock: &mut NetlinkSocket, ifname: &str) -> Result<(), Box<dyn Error>> {
    let mut request = rt_link::Request::new()
        .set_create()
        .set_excl()
        .op_newlink_do(&rt_link::Ifinfomsg::new());

    request
        .encode()
        .push_ifname_bytes(ifname.as_bytes())
        .nested_linkinfo()
        .push_kind(c"dummy");

    let mut iter = sock.request(&request).await?;
    let _ = iter.recv_ack().await?;
    Ok(())
}

async fn link_del(sock: &mut NetlinkSocket, ifname: &str) -> Result<(), Box<dyn Error>> {
    // Chained request isn't strictly needed here
    let mut request = rt_link::Chained::new(sock.reserve_seq(256));
    request
        .request()
        .op_dellink_do(&Default::default())
        .encode()
        .push_ifname_bytes(ifname.as_bytes());
    let request = request.finalize();

    let mut iter = sock.request_chained(&request).await?;
    let _ = iter.recv_all().await;
    Ok(())
}
