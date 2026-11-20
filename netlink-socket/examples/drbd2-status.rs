//! This example lists the DRBD resources of this node with their devices and
//! connections, roughly what `drbdsetup status` prints:
//!
//! ```sh
//! drbdsetup status
//! ```
//!
//! It needs a DRBD 9 kernel module that offers the "drbd2" generic netlink
//! family. DRBD 9 is not part of the mainline kernel yet, see
//! <https://github.com/LINBIT/drbd>.
//!
//! Run with: `cargo run --example drbd2-status --features=drbd2`

use std::{error::Error, ffi::CStr};

use netlink_bindings::drbd2::{self, ConnectionState, DiskState, Role};
use netlink_socket2::NetlinkSocket;

fn main() -> Result<(), Box<dyn Error>> {
    let mut sock = NetlinkSocket::new();

    let resources = dump_resources(&mut sock)?;
    if resources.is_empty() {
        println!("No DRBD resources configured");
        return Ok(());
    }

    for (name, role) in resources {
        println!("{name:?} role:{role}");
        dump_devices(&mut sock, &name)?;
        dump_connections(&mut sock, &name)?;
        println!();
    }

    Ok(())
}

fn dump_resources(
    sock: &mut NetlinkSocket,
) -> Result<Vec<(std::ffi::CString, String)>, Box<dyn Error>> {
    let request = drbd2::Request::new().op_resource_get_dump();

    let mut resources = Vec::new();
    let mut iter = sock.request(&request)?;
    while let Some(attrs) = iter.recv().transpose()? {
        let resource = attrs.get_resource()?;
        let name = resource.get_context()?.get_resource_name()?;
        let role = resource.get_info()?.get_role()?;
        resources.push((name.to_owned(), name_of(Role::from_value(role.into()))));
    }

    Ok(resources)
}

fn dump_devices(sock: &mut NetlinkSocket, resource: &CStr) -> Result<(), Box<dyn Error>> {
    let mut request = drbd2::Request::new().op_device_get_dump();
    // Without a context the kernel dumps the devices of all resources
    request
        .encode()
        .nested_context()
        .push_resource_name(resource);

    let mut iter = sock.request(&request)?;
    while let Some(attrs) = iter.recv().transpose()? {
        let device = attrs.get_device()?;
        let context = device.get_context()?;
        let disk = device.get_info()?.get_disk_state()?;
        println!(
            "  volume:{} minor:{} disk:{}",
            context.get_volume()?,
            context.get_minor()?,
            name_of(DiskState::from_value(disk.into())),
        );
    }

    Ok(())
}

fn dump_connections(sock: &mut NetlinkSocket, resource: &CStr) -> Result<(), Box<dyn Error>> {
    let mut request = drbd2::Request::new().op_connection_get_dump();
    request
        .encode()
        .nested_context()
        .push_resource_name(resource);

    let mut iter = sock.request(&request)?;
    while let Some(attrs) = iter.recv().transpose()? {
        let connection = attrs.get_connection()?;
        let context = connection.get_context()?;
        let info = connection.get_info()?;
        println!(
            "  {:?} node-id:{} connection:{} role:{}",
            context.get_connection_name()?,
            context.get_peer_node_id()?,
            name_of(ConnectionState::from_value(
                info.get_connection_state()?.into()
            )),
            name_of(Role::from_value(info.get_role()?.into())),
        );
    }

    Ok(())
}

/// A value this version of the bindings has no name for is printed as "?"
fn name_of<T: std::fmt::Debug>(value: Option<T>) -> String {
    value.map_or_else(|| "?".into(), |v| format!("{v:?}"))
}
