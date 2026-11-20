//! List, get, and set numeric ids on network namespaces.
//!
//! ```sh
//! ip netns list-id
//! ip netns set netns-id-example 42
//! ip netns list-id nsid 42
//! # Get netns id from an fd
//! ```
//!
//! Note that rt-nsid subsystem doesn't handle creating and deleting namespaces.
//!
//! Run with: `cargo run --example netns-id --features=rt-nsid`

use std::{collections::HashMap, error::Error, fs, os::fd::AsRawFd, path::Path};

use netlink_bindings::rt_nsid;
use netlink_socket2::NetlinkSocket;

fn main() -> Result<(), Box<dyn Error>> {
    let mut sock = NetlinkSocket::new();

    {
        let netns = Path::new("/proc/self/ns/net");
        let fd = fs::File::open(&netns)?;

        let nsid = get_netns_id(&mut sock, fd.as_raw_fd())?;
        println!("Current netns {netns:?} has id: {nsid}");
    }

    println!();
    list_netns(&mut sock)?;

    if std::env::var("TESTING").is_ok() && unsafe { libc::getuid() != 0 } {
        return Ok(());
    }

    let name = "netns-id-example";
    let new_nsid = 42;
    let netns = Path::new("/var/run/netns").join(name);

    println!();
    println!("Creating netns {name:?}");

    std::process::Command::new("ip")
        .args(["netns", "add", name])
        .status()?;

    let fd = fs::File::open(&netns)?;

    println!();
    list_netns(&mut sock)?;

    println!();
    println!("Setting {name:?} id to {new_nsid}");
    set_netns_id(&mut sock, fd.as_raw_fd(), new_nsid)?;

    println!();
    list_netns(&mut sock)?;

    let nsid = get_netns_id(&mut sock, fd.as_raw_fd())?;
    println!();
    println!("Netns {name:?} has id {nsid}");

    println!();
    println!("Removing {name:?}");

    std::mem::drop(fd);

    std::process::Command::new("ip")
        .args(["netns", "del", name])
        .status()?;

    println!();
    list_netns(&mut sock)?;

    Ok(())
}

fn list_netns(sock: &mut NetlinkSocket) -> Result<(), Box<dyn Error>> {
    let mut nsid_to_name = HashMap::new();
    if let Ok(iter) = fs::read_dir("/var/run/netns") {
        for dir in iter.flatten() {
            let Ok(fd) = fs::File::open(dir.path()) else {
                continue;
            };
            let Ok(nsid) = get_netns_id(sock, fd.as_raw_fd()) else {
                continue;
            };
            nsid_to_name.insert(nsid, dir.file_name().to_string_lossy().to_string());
        }
    }

    let request = rt_nsid::Request::new().op_getnsid_dump(&rt_nsid::Rtgenmsg::default());
    let mut iter = sock.request(&request)?;

    println!("Listing netns:");
    while let Some((_header, attrs)) = iter.recv().transpose()? {
        if let Ok(nsid) = attrs.get_nsid() {
            let name = nsid_to_name
                .get(&nsid)
                .map(String::as_str)
                .unwrap_or("unknown");
            println!("nsid {nsid} ({name})");
        }
    }

    Ok(())
}

fn set_netns_id(sock: &mut NetlinkSocket, netns_fd: i32, nsid: i32) -> Result<(), Box<dyn Error>> {
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
