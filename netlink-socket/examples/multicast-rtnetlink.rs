//! This example demonstrates receiving Netlink multicast notifications emitted
//! by rtnetlink subsystem (rt-link, rt-addr, rt-route, tc, etc), for example,
//! notifying when a new network device is created using `ip link add ...`
//!
//! Multicast notification are quite sparsely documented, so netlink-bindings
//! only provides a "raw" socket, meaning you have to provide `group_id` to
//! listen to and later to choose how to decode received messages yourself.
//!
//! Run with: `cargo run --example multicast-raw --features=rt-link`

use std::error::Error;

use netlink_bindings::{rt_link, traits::NetlinkRequest};
use netlink_socket2::{MulticastSocketRaw, NetlinkSocket};

#[cfg_attr(not(feature = "async"), maybe_async::must_be_sync)]
#[cfg_attr(feature = "tokio", tokio::main(flavor = "current_thread"))]
#[cfg_attr(feature = "smol", macro_rules_attribute::apply(smol_macros::main))]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut sock = NetlinkSocket::new();

    // The same protonum is also used by rt_addr, rt_neigh, tc, etc.
    let protonum = rt_link::PROTONUM /* 0 */;
    let mut multicast_sock = MulticastSocketRaw::new(protonum)?;

    // Before receiving notifications, you have to subscribe to relevant groups.
    // Check the [`netdev::NotifGroup`] for available groups, or the upstream
    // `<subsystem>.yaml` specification, here notification groups are called
    // "mcast-groups" and are usually placed at the bottom of the file.
    //
    // Under the hood, .listen() calls setsockopt*() with NETLINK_ADD_MEMBERSHIP.
    multicast_sock.listen(libc::RTNLGRP_LINK)?;

    // Group ids for rtnetlink are allocated statically, we subscribe to a bunch
    // of them blinds simply to surface notifications drifting on your system.
    for i in 0..100 {
        if multicast_sock.listen(i).is_ok() {
            println!("Group {i}: {}", group_from_id(i).unwrap_or("(unknown)"));
        }
    }

    // This should emit notifications for us to process
    let link = "example-link";
    link_add(&mut sock, link).await?;
    link_del(&mut sock, link).await?;

    loop {
        let (recv, buf) = multicast_sock.recv().await?;

        let multicast_group = recv.multicast_group;
        let value = recv.message_type;

        let value_str = format!(
            "{} ({})",
            rt_op_from_id(value).unwrap_or("(unknown)"),
            value
        );
        let group = group_from_id(multicast_group).unwrap_or("(unknown)");

        println!("Message from group={group} value={value_str}");

        // netlink_bindings::utils::dump_hex(buf);

        // Rtnetlink message value are aligned to groups of 4: new, del, get, set
        match value & !0b11 {
            libc::RTM_NEWLINK => {
                let (_header, attrs) = rt_link::OpGetlinkDo::decode_reply(buf);

                let op = rt_op_from_id(value).unwrap().strip_prefix("RTM_").unwrap();
                let link = attrs.get_ifname()?;
                let r#type = attrs
                    .get_linkinfo()
                    .unwrap_or_default()
                    .get_kind()
                    .unwrap_or(c"(unspecified)");

                println!("{op} link {link:?} of type {type:?}");

                if std::env::var("TESTING").is_ok() && op == "DELLINK" {
                    return Ok(());
                }
            }

            // ...
            _ => {}
        }
    }
}

macro_rules! to_from_enum {
    (
        fn $from_id:ident;
        fn $to_id:ident;
        #[repr($type:ident)]
        enum $enum:ident {
            $($ent:ident $(= $val:expr)?,)*
        }
    ) => {
        #[repr($type)]
        #[allow(non_camel_case_types)]
        enum $enum {
            $($ent $(= $val)?,)*
        }
        fn $from_id(val: $type) -> Option<&'static str> {
            $(
            if val == $enum::$ent as $type {
                return Some(stringify!($ent));
            }
            )*
            None
        }
        #[allow(unused)]
        fn $to_id(val: &str) -> Option<$type> {
            let res = match val {
                $(stringify!($ent) => $enum::$ent as $type,)*
                _ => return None,
            };
            Some(res)
        }
    };
}

// Taken from linux/rtnetlink.h
to_from_enum! {
    fn group_from_id;
    fn group_to_id;
    #[repr(u32)]
    enum Group {
        RTNLGRP_NONE,
        RTNLGRP_LINK,
        RTNLGRP_NOTIFY,
        RTNLGRP_NEIGH,
        RTNLGRP_TC,
        RTNLGRP_IPV4_IFADDR,
        RTNLGRP_IPV4_MROUTE,
        RTNLGRP_IPV4_ROUTE,
        RTNLGRP_IPV4_RULE,
        RTNLGRP_IPV6_IFADDR,
        RTNLGRP_IPV6_MROUTE,
        RTNLGRP_IPV6_ROUTE,
        RTNLGRP_IPV6_IFINFO,
        RTNLGRP_DECnet_IFADDR,
        RTNLGRP_NOP2,
        RTNLGRP_DECnet_ROUTE,
        RTNLGRP_DECnet_RULE,
        RTNLGRP_NOP4,
        RTNLGRP_IPV6_PREFIX,
        RTNLGRP_IPV6_RULE,
        RTNLGRP_ND_USEROPT,
        RTNLGRP_PHONET_IFADDR,
        RTNLGRP_PHONET_ROUTE,
        RTNLGRP_DCB,
        RTNLGRP_IPV4_NETCONF,
        RTNLGRP_IPV6_NETCONF,
        RTNLGRP_MDB,
        RTNLGRP_MPLS_ROUTE,
        RTNLGRP_NSID,
        RTNLGRP_MPLS_NETCONF,
        RTNLGRP_IPV4_MROUTE_R,
        RTNLGRP_IPV6_MROUTE_R,
        RTNLGRP_NEXTHOP,
        RTNLGRP_BRVLAN,
        RTNLGRP_MCTP_IFADDR,
        RTNLGRP_TUNNEL,
        RTNLGRP_STATS,
        RTNLGRP_IPV4_MCADDR,
        RTNLGRP_IPV6_MCADDR,
        RTNLGRP_IPV6_ACADDR,
    }
}

// Taken from linux/rtnetlink.h
to_from_enum! {
    fn rt_op_from_id;
    fn rt_op_to_id;
    #[repr(u16)]
    enum RtOp {
        RTM_NEWLINK = 16,
        RTM_DELLINK,
        RTM_GETLINK,
        RTM_SETLINK,
        RTM_NEWADDR = 20,
        RTM_DELADDR,
        RTM_GETADDR,
        RTM_NEWROUTE = 24,
        RTM_DELROUTE,
        RTM_GETROUTE,
        RTM_NEWNEIGH = 28,
        RTM_DELNEIGH,
        RTM_GETNEIGH,
        RTM_NEWRULE = 32,
        RTM_DELRULE,
        RTM_GETRULE,
        RTM_NEWQDISC = 36,
        RTM_DELQDISC,
        RTM_GETQDISC,
        RTM_NEWTCLASS = 40,
        RTM_DELTCLASS,
        RTM_GETTCLASS,
        RTM_NEWTFILTER = 44,
        RTM_DELTFILTER,
        RTM_GETTFILTER,
        RTM_NEWACTION = 48,
        RTM_DELACTION,
        RTM_GETACTION,
        RTM_NEWPREFIX = 52,
        RTM_NEWMULTICAST = 56,
        RTM_DELMULTICAST,
        RTM_GETMULTICAST,
        RTM_NEWANYCAST = 60,
        RTM_DELANYCAST,
        RTM_GETANYCAST,
        RTM_NEWNEIGHTBL = 64,
        RTM_GETNEIGHTBL = 66,
        RTM_SETNEIGHTBL,
        RTM_NEWNDUSEROPT = 68,
        RTM_NEWADDRLABEL = 72,
        RTM_DELADDRLABEL,
        RTM_GETADDRLABEL,
        RTM_GETDCB = 78,
        RTM_SETDCB,
        RTM_NEWNETCONF = 80,
        RTM_DELNETCONF,
        RTM_GETNETCONF = 82,
        RTM_NEWMDB = 84,
        RTM_DELMDB = 85,
        RTM_GETMDB = 86,
        RTM_NEWNSID = 88,
        RTM_DELNSID = 89,
        RTM_GETNSID = 90,
        RTM_NEWSTATS = 92,
        RTM_GETSTATS = 94,
        RTM_SETSTATS,
        RTM_NEWCACHEREPORT = 96,
        RTM_NEWCHAIN = 100,
        RTM_DELCHAIN,
        RTM_GETCHAIN,
        RTM_NEWNEXTHOP = 104,
        RTM_DELNEXTHOP,
        RTM_GETNEXTHOP,
        RTM_NEWLINKPROP = 108,
        RTM_DELLINKPROP,
        RTM_GETLINKPROP,
        RTM_NEWVLAN = 112,
        RTM_DELVLAN,
        RTM_GETVLAN,
        RTM_NEWNEXTHOPBUCKET = 116,
        RTM_DELNEXTHOPBUCKET,
        RTM_GETNEXTHOPBUCKET,
        RTM_NEWTUNNEL = 120,
        RTM_DELTUNNEL,
        RTM_GETTUNNEL,
    }
}

/// Equivalent to `ip link add dev {ifname} type dummy`
#[cfg_attr(not(feature = "async"), maybe_async::must_be_sync)]
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

/// Equivalent to `ip link del dev {ifname}`
#[cfg_attr(not(feature = "async"), maybe_async::must_be_sync)]
async fn link_del(sock: &mut NetlinkSocket, ifname: &str) -> Result<(), Box<dyn Error>> {
    let mut request = rt_link::Request::new().op_dellink_do(&Default::default());

    request.encode().push_ifname_bytes(ifname.as_bytes());

    let mut iter = sock.request(&request).await?;
    let _ = iter.recv_ack().await;
    Ok(())
}
