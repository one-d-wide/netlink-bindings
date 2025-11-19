//! This example demonstrates basic interactions with nl80211.
//!
//! Same as ./nl80211.rs , but netlink messages are encoded manually,
//! incurring some boilerplate like a custom wrapper type for requests.
//!
//! Run with: `cargo run --example nl80211-raw --features=nl80211,rt-link`

use std::collections::HashSet;

use netlink_bindings::{
    builtin::PushBuiltinNfgenmsg,
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

#[cfg_attr(not(feature = "async"), maybe_async::maybe_async)]
#[cfg_attr(feature = "tokio", tokio::main(flavor = "current_thread"))]
#[cfg_attr(feature = "smol", macro_rules_attribute::apply(smol_macros::main))]
async fn main() {
    let mut sock = NetlinkSocket::new();

    let ifname = "nl80211-example";

    println!("Dumping wifi devices");
    let devices = dump_wiphy(&mut sock).await;

    if let Some(ifindex) = get_interface_index(&mut sock, ifname).await {
        println!("Interface {ifname:?} already exists. Removing it");
        wiphy_del_interface(&mut sock, ifindex).await;
    }

    if devices.is_empty() {
        println!("No wifi devices found");
        return;
    }

    let (phy, phy_id) = devices.first().unwrap();

    println!("Adding {ifname:?} for phy {phy:?}");
    let ifindex = wiphy_add_interface(&mut sock, *phy_id, ifname).await;

    println!("Removing {ifname:?}");
    wiphy_del_interface(&mut sock, ifindex).await;
}

#[cfg_attr(not(feature = "async"), maybe_async::maybe_async)]
async fn dump_wiphy(sock: &mut NetlinkSocket) -> Vec<(String, u32)> {
    let mut request = RawRequest::new();

    // Request may specify multiple of NLM_F_* flags.
    // Those usually trigger special behaviors in certain operation types.
    request.flags |= consts::NLM_F_DUMP as u16;

    // First the message header.
    // Nl80211 is a generic netlink family, so it uses PushBuiltinNfgenmsg.
    let mut header = PushBuiltinNfgenmsg::new();
    header.set_cmd(Commands::GetWiphy as u8);
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
    let mut request = sock.request(&request).await.unwrap();
    while let Some(res) = request.recv().await {
        // Parse a reply. Genetlink reply header doesn't carry any significant
        // information, so it can be ignored.
        let (_genl_header, attrs) = res.unwrap().split_at(PushBuiltinNfgenmsg::len());
        let attrs = Nl80211Attrs::new(attrs);

        // And here we go
        // dbg!(attrs);

        // Collect names of wifi devices
        let name = attrs.get_wiphy_name().unwrap();
        let index = attrs.get_wiphy().unwrap();

        if let Ok(mut commands) = attrs.get_supported_commands() {
            if commands.any(|c| c == Commands::NewInterface as u32) {
                devices.insert((name.to_string_lossy().to_string(), index));
            }
        }
    }

    devices.into_iter().collect()
}

#[cfg_attr(not(feature = "async"), maybe_async::maybe_async)]
async fn wiphy_add_interface(sock: &mut NetlinkSocket, phy_id: u32, new_ifname: &str) -> u32 {
    let mut request = RawRequest::new();

    let mut header = PushBuiltinNfgenmsg::new();
    header.set_cmd(Commands::NewInterface as u8);
    request.buf.extend(header.as_slice());

    PushNl80211Attrs::new(&mut request.buf)
        .push_wiphy(phy_id)
        .push_ifname_bytes(new_ifname.as_bytes())
        .push_iftype(2) // aka managed
        // ...
        ;

    let mut iter = sock.request(&request).await.unwrap();
    let reply = iter.recv_one().await.unwrap();
    let (_genl_header, attrs) = reply.split_at(PushBuiltinNfgenmsg::len());
    let attrs = Nl80211Attrs::new(attrs);

    // dbg!(attrs);

    attrs.get_ifindex().unwrap()
}

#[cfg_attr(not(feature = "async"), maybe_async::maybe_async)]
async fn wiphy_del_interface(sock: &mut NetlinkSocket, ifindex: u32) {
    let mut request = RawRequest::new();

    let mut header = PushBuiltinNfgenmsg::new();
    header.set_cmd(Commands::DelInterface as u8);
    request.buf.extend(header.as_slice());

    PushNl80211Attrs::new(&mut request.buf).push_ifindex(ifindex);

    let mut iter = sock.request(&request).await.unwrap();
    iter.recv_ack().await.unwrap();
}

#[cfg_attr(not(feature = "async"), maybe_async::maybe_async)]
async fn get_interface_index(sock: &mut NetlinkSocket, ifname: &str) -> Option<u32> {
    let request = rt_link::Request::new().op_getlink_dump_request(&Default::default());

    let mut iter = sock.request(&request).await.unwrap();
    while let Some(res) = iter.recv().await {
        let (header, attrs) = res.unwrap();
        if attrs.get_ifname().unwrap().to_bytes() == ifname.as_bytes() {
            return Some(header.ifi_index() as u32);
        }
    }

    None
}
