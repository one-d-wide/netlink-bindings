use std::{error::Error, os::fd::AsRawFd};

use netlink_bindings::rt_nsid;
use netlink_socket2::NetlinkSocket;
use netns_rs::{get_from_current_thread, NetNs};

fn main() -> Result<(), Box<dyn Error>> {
    let mut sock = NetlinkSocket::new();

    let current_netns = get_from_current_thread()?;
    let nsid = get_netns_id(&mut sock, current_netns.file().as_raw_fd())?;
    println!("Netns {} has id: {nsid}", current_netns.path().display());

    const NETNS_NAME: &str = "netlink-test";

    println!();
    list_netns_id(&mut sock)?;

    println!();
    println!("Create netns: {NETNS_NAME}");
    let test_netns = NetNs::new(NETNS_NAME)?;

    println!();
    list_netns_id(&mut sock)?;

    println!();
    println!("Assign netns id 1 to: {NETNS_NAME}");
    assign_netns_id(&mut sock, test_netns.file().as_raw_fd(), 1)?;

    println!();
    list_netns_id(&mut sock)?;

    let nsid = get_netns_id(&mut sock, test_netns.file().as_raw_fd())?;
    println!();
    println!("Netns {NETNS_NAME} has id: {nsid}");

    println!();
    println!("Remove netns: {NETNS_NAME}");
    test_netns.remove()?;

    println!();
    list_netns_id(&mut sock)?;

    Ok(())
}

fn list_netns_id(sock: &mut NetlinkSocket) -> Result<(), Box<dyn Error>> {
    let request = rt_nsid::Request::new().op_getnsid_dump(&rt_nsid::Rtgenmsg::default());
    let mut iter = sock.request(&request)?;

    println!("Nsid list:");

    while let Some((_header, attrs)) = iter.recv().transpose()? {
        if let Ok(nsid) = attrs.get_nsid() {
            println!("- {nsid}");
        }
    }

    Ok(())
}

fn assign_netns_id(
    sock: &mut NetlinkSocket,
    netns_fd: i32,
    nsid: i32,
) -> Result<(), Box<dyn Error>> {
    let mut request = rt_nsid::Request::new()
        .set_create()
        .set_excl()
        .op_newnsid_do(&rt_nsid::Rtgenmsg::default());

    request.encode().push_fd(netns_fd as u32).push_nsid(nsid);

    sock.request(&request)?.recv_ack()?;

    Ok(())
}

fn get_netns_id(sock: &mut NetlinkSocket, netns_fd: i32) -> Result<i32, Box<dyn Error>> {
    let mut request = rt_nsid::Request::new().op_getnsid_do(&rt_nsid::Rtgenmsg::default());

    request.encode().push_fd(netns_fd as u32);

    let mut reply = sock.request(&request)?;
    let (_header, attrs) = reply.recv_one()?;

    Ok(attrs.get_nsid()?)
}
