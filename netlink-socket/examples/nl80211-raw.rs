//! This example demonstrates basic interactions with nl80211.
//!
//! Same as ./nl80211.rs , but netlink messages are encoded manually,
//! incurring some boilerplate like a custom wrapper type for requests.
//!
//! Run with: `cargo run --example nl80211-raw --features=nl80211,rt-link`

use std::{collections::HashSet, error::Error};

use netlink_bindings::{
    builtin::BuiltinNfgenmsg,
    consts,
    nl80211::{Commands, Nl80211Attrs, PushNl80211Attrs},
    rt_link,
    traits::{NetlinkRequest, Protocol},
};
use netlink_socket2::NetlinkSocket;

struct RawRequest {
    buf: Vec<u8>,
    flags: u16,
}

impl RawRequest {
    fn new() -> Self {
        Self {
            buf: Vec::new(),
            flags: 0,
        }
    }
}

impl NetlinkRequest for RawRequest {
    fn protocol(&self) -> Protocol {
        Protocol::Generic("nl80211".as_bytes())
    }

    fn flags(&self) -> u16 {
        self.flags
    }

    fn payload(&self) -> &[u8] {
        &self.buf
    }

    type ReplyType<'buf> = &'buf [u8];
    fn decode_reply(buf: &[u8]) -> Self::ReplyType<'_> {
        buf
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut sock = NetlinkSocket::new();

    let ifname = "nl80211-example";

    println!("Dumping wifi devices");
    let devices = dump_wiphy(&mut sock)?;

    if let Some(ifindex) = get_interface_index(&mut sock, ifname)? {
        println!("Interface {ifname:?} already exists. Removing it");
        wiphy_del_interface(&mut sock, ifindex)?;
    }

    if devices.is_empty() {
        println!("No wifi devices found");
        return Ok(());
    }

    let (phy, phy_id) = devices.first().unwrap();

    println!("Adding {ifname:?} for phy {phy:?}");
    let ifindex = wiphy_add_interface(&mut sock, *phy_id, ifname)?;

    println!("Removing {ifname:?}");
    wiphy_del_interface(&mut sock, ifindex)?;

    Ok(())
}

fn dump_wiphy(sock: &mut NetlinkSocket) -> Result<Vec<(String, u32)>, Box<dyn Error>> {
    let mut request = RawRequest::new();

    // Request may specify multiple of NLM_F_* flags.
    // Those usually trigger special behaviors in certain operation types.
    request.flags |= consts::NLM_F_DUMP as u16;

    // First the message header.
    // Nl80211 is a generic netlink family, so it uses BuiltinNfgenmsg.
    let mut header = BuiltinNfgenmsg::new();
    header.cmd = Commands::GetWiphy as u8;
    request.buf.extend(header.as_slice());

    // Then the actual attributes. Top-level attributes may be concatenated by
    // simply calling PushSomeAttrs::new(&mut buf) multiple times.
    PushNl80211Attrs::new(&mut request.buf)
        // Allow the kernel to split reply into multiple chunks, each carrying
        // phy id, phy name and some attributes, since messages get quite large.
        .push_split_wiphy_dump(())
        // ...
        ;

    let mut devices = HashSet::new();
    let mut request = sock.request(&request)?;
    while let Some(res) = request.recv().transpose()? {
        // Parse a reply. Genetlink reply header doesn't carry any significant
        // information, so it can be ignored.
        let (_genl_header, attrs) = res.split_at(BuiltinNfgenmsg::len());
        let attrs = Nl80211Attrs::new(attrs);

        // And here we go
        // dbg!(attrs);

        // Collect names of wifi devices
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

fn wiphy_add_interface(
    sock: &mut NetlinkSocket,
    phy_id: u32,
    new_ifname: &str,
) -> Result<u32, Box<dyn Error>> {
    let mut request = RawRequest::new();

    let mut header = BuiltinNfgenmsg::new();
    header.cmd = Commands::NewInterface as u8;
    request.buf.extend(header.as_slice());

    PushNl80211Attrs::new(&mut request.buf)
        .push_wiphy(phy_id)
        .push_ifname_bytes(new_ifname.as_bytes())
        .push_iftype(2) // aka managed
        // ...
        ;

    let mut iter = sock.request(&request)?;
    let (_genl_header, attrs) = iter.recv_one()?.split_at(BuiltinNfgenmsg::len());
    let attrs = Nl80211Attrs::new(attrs);

    // dbg!(attrs);

    Ok(attrs.get_ifindex()?)
}

fn wiphy_del_interface(sock: &mut NetlinkSocket, ifindex: u32) -> Result<(), Box<dyn Error>> {
    let mut request = RawRequest::new();

    let mut header = BuiltinNfgenmsg::new();
    header.cmd = Commands::DelInterface as u8;
    request.buf.extend(header.as_slice());

    PushNl80211Attrs::new(&mut request.buf).push_ifindex(ifindex);

    sock.request(&request)?.recv_ack()?;

    Ok(())
}

fn get_interface_index(
    sock: &mut NetlinkSocket,
    ifname: &str,
) -> Result<Option<u32>, Box<dyn Error>> {
    let request = rt_link::Request::new().op_getlink_dump(&Default::default());

    let mut iter = sock.request(&request).unwrap();
    while let Some((header, attrs)) = iter.recv().transpose()? {
        if attrs.get_ifname().unwrap().to_bytes() == ifname.as_bytes() {
            return Ok(Some(header.ifi_index as u32));
        }
    }

    Ok(None)
}
