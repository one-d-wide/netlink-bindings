use crate::{ANY, ASSERT_LAST, EPSILON, Map, command, group};
use quote::quote;

command! {
    const COMMANDS;
    prelude => {
        use netlink_bindings::traits::Pusher;
        use netlink_bindings::rt_link;
        let mut header = rt_link::Ifinfomsg::new();
        let mut buf = Vec::new();
        let mut attrs = rt_link::PushLinkAttrs::new(&mut buf);

        use rt_link::{LinkAttrs, RtextFilter, BrBooloptMulti, BrBooloptId};
    }

    "ip link /list/show": [FLAGS, LINK_FILTER]:
    filter_map |desc, tok| => {
        match desc.rsplit(" ").next().unwrap() {
            "" | "list" | "show" => {},
            _ => panic!("{desc:?}"),
        };

        quote! {
            header.ifi_family = libc::AF_PACKET as u8;
            let mut ext_mask = RtextFilter::Vf as u32 | RtextFilter::SkipStats as u32;

            #tok

            attrs = attrs.push_ext_mask(ext_mask);
            let mut req = ::netlink_bindings::rt_link::Request::new()
                .set_flags(
                    if header.ifi_index == 0
                        && LinkAttrs::new(attrs.as_vec()).get_ifname().is_err()
                    {
                        libc::NLM_F_DUMP as u16
                    } else {
                        0
                    }
                )
                .op_getlink_do(&header);
            req.encode().as_vec_mut().extend_from_slice(attrs.as_vec());
            req
        }
    },

    "ip link add/del/delete/set/change": [FLAGS, LINK_DEVICE, LINK_PROPS, LINK_TYPE.may()]:
    filter_map |desc, tok| => {
        let verb = desc.rsplit(" ").next().unwrap();
        let req = match verb {
            "add" => quote! {
                rt_link::Request::new()
                    .set_create()
                    .set_excl()
                    .op_newlink_do(&header)
            },
            "del" | "delete" => quote! {
                rt_link::Request::new()
                    .op_dellink_do(&header)
            },
            "set" => quote! {
                rt_link::Request::new()
                    .op_newlink_do(&header)
            },
            "change" => quote! {
                rt_link::Request::new()
                    .op_newlink_do(&header)
            },
            _ => panic!("{desc:?}"),
        };

        let skip_ifi = matches!(verb, "add");

        quote! {
            let mut skip_ifi = #skip_ifi;

            #tok

            let mut req = #req;
            req.encode().as_vec_mut().extend_from_slice(attrs.as_vec());
            req
        }
    },

    "ip link property/afstats/xfstats": [ANY.star()]:
    filter_map |desc, _tok| => {
        let err = format!("ip link {desc} attributes are not defined in upstream spec");
        quote! {
            compile_error!(#err);
        }
    },
}

group! {
    const FLAGS;
    ~ repeat: star,
    "-4" => { header.ifi_family = libc::AF_INET as u8; }
    "-6" => { header.ifi_family = libc::AF_INET6 as u8; }
}

group! {
    const LINK_DEVICE;
    ("dev" | "name"), ifname: &str => {
        if skip_ifi {
            attrs = attrs.push_ifname_bytes(ifname.as_bytes());
        } else {
            header.ifi_index = ::ip_route::utils::get_ifindex_str(ifname) as i32;
        }
    }
    "group", group_id: u32 => {
        attrs = attrs.push_group(group_id);
    }
    ifname: &str => {
        if skip_ifi {
            attrs = attrs.push_ifname_bytes(ifname.as_bytes());
        } else {
            header.ifi_index = ::ip_route::utils::get_ifindex_str(ifname) as i32;
        }
    }
}

group! {
    const LINK_PROPS;
    ~ repeat: star,
    "up" => {
        header.ifi_flags |= libc::IFF_UP as u32;
        header.ifi_change |= libc::IFF_UP as u32;
    }
    "down" => {
        header.ifi_change |= libc::IFF_UP as u32;
    }
    "arp", "on" => {
        header.ifi_change |= libc::IFF_NOARP as u32;
    }
    "arp", "off" => {
        header.ifi_flags |= libc::IFF_NOARP as u32;
        header.ifi_change |= libc::IFF_NOARP as u32;
    }
    "dynamic", "on" => {
        header.ifi_flags |= libc::IFF_DYNAMIC as u32;
        header.ifi_change |= libc::IFF_DYNAMIC as u32;
    }
    "dynamic", "off" => {
        header.ifi_change |= libc::IFF_DYNAMIC as u32;
    }
    "multicast", "on" => {
        header.ifi_flags |= libc::IFF_MULTICAST as u32;
        header.ifi_change |= libc::IFF_MULTICAST as u32;
    }
    "multicast", "off" => {
        header.ifi_change |= libc::IFF_MULTICAST as u32;
    }
    "allmulticast", "on" => {
        header.ifi_flags |= libc::IFF_ALLMULTI as u32;
        header.ifi_change |= libc::IFF_ALLMULTI as u32;
    }
    "allmulticast", "off" => {
        header.ifi_change |= libc::IFF_ALLMULTI as u32;
    }
    "promisc", "on" => {
        header.ifi_flags |= libc::IFF_PROMISC as u32;
        header.ifi_change |= libc::IFF_PROMISC as u32;
    }
    "promisc", "off" => {
        header.ifi_change |= libc::IFF_PROMISC as u32;
    }
    "trailers", "on" => {
        header.ifi_change |= libc::IFF_NOTRAILERS as u32;
    }
    "trailers", "off" => {
        header.ifi_flags |= libc::IFF_NOTRAILERS as u32;
        header.ifi_change |= libc::IFF_NOTRAILERS as u32;
    }
    "carrier", "on" => {
        attrs = attrs.push_carrier(1);
    }
    "carrier", "off" => {
        attrs = attrs.push_carrier(0);
    }
    "txqueuelen", val: u32 => {
        attrs = attrs.push_txqlen(val);
    }
    "txqlen", val: u32 => {
        attrs = attrs.push_txqlen(val);
    }
    "name", ifname: &str => {
        if skip_ifi {
            attrs = attrs.push_ifname_bytes(ifname.as_bytes());
        } else {
            header.ifi_index = ::ip_route::utils::get_ifindex_str(ifname) as i32;
        }
    }
    "address", mac: &[u8] as "mac" => {
        attrs = attrs.push_address(mac);
    }
    "broadcast", mac: &[u8] as "mac" => {
        attrs = attrs.push_broadcast(mac);
    }
    "mtu", val: u32 => {
        attrs = attrs.push_mtu(val);
    }
    "index", val: u32 => {
        header.ifi_index = val as i32;
    }
    "numtxqueues", num: u32 => {
        attrs = attrs.push_num_tx_queues(num);
    }
    "numrxqueues", num: u32 => {
        attrs = attrs.push_num_rx_queues(num);
    }
    "netns", pid: u32 => {
        attrs = attrs.push_net_ns_pid(pid);
    }
    "link", name: &str => {
        attrs = attrs.push_link(::ip_route::utils::get_ifindex_str(name));
    }
    "link-netnsid", id: u32 => {
        attrs = attrs.push_link_netnsid(id as i32);
    }
    "link-netns", name: &str => {
        compile_error!("\"link-netns\" not supported. Use \"link-netnsid\"");
    }
    "alias", name: &str => {
        attrs = attrs.push_ifalias_bytes(name.as_bytes());
    }
    "master", name: &str => {
        attrs = attrs.push_master(::ip_route::utils::get_ifindex_str(name));
    }
    "vrf", name: &str => {
        attrs = attrs.push_master(::ip_route::utils::get_ifindex_str(name));
    }
    "nomaster" => {
        attrs = attrs.push_master(0);
    }
    "protodown", "on" => {
        attrs = attrs.push_proto_down(1);
    }
    "protodown", "off" => {
        attrs = attrs.push_proto_down(0);
    }
    "gso_max_size", val: u32 => {
        attrs = attrs.push_gso_max_size(val);
    }
    "gso_ipv4_max_size", val: u32 => {
        attrs = attrs.push_gso_ipv4_max_size(val);
    }
    "gso_max_segs", val: u32 => {
        attrs = attrs.push_gso_max_segs(val);
    }
    "gro_max_size", val: u32 => {
        attrs = attrs.push_gro_max_size(val);
    }
    "gro_ipv4_max_size", val: u32 => {
        attrs = attrs.push_gro_ipv4_max_size(val);
    }
}

group! {
    const LINK_TYPE;
    "type", &LINK_TYPE_INNER, &ASSERT_LAST => {}
}

mod bridge;
use bridge::TYPE_BRIDGE;

group! {
    const LINK_TYPE_INNER;
    kind: &str, &EPSILON => {
        attrs = attrs
            .nested_linkinfo()
            .push_kind_bytes(kind.as_bytes())
            .end_nested();
    }
    "bridge",  &TYPE_BRIDGE => {}
}

group! {
    const LINK_FILTER;
    "index", val: u32 => {
        header.ifi_index = val as i32;
    }
    "dev", ifname: &str => {
        attrs = attrs.push_ifname_bytes(ifname.as_bytes());
    }
    "group" => {
        compile_error!("group is unimplemented");
    }
    "up" => {
        compile_error!("check whether link is up using: (header.ifi_flags & rt_link::IfinfoFlags::Up as u32) != 0");
    }
    "down" => {
        compile_error!("check whether link is down using: (header.ifi_flags & rt_link::IfinfoFlags::Up as u32) == 0");
    }
    "master", name: &str => {
        attrs = attrs.push_master(::ip_route::utils::get_ifindex_str(name));
    }
    "vrf", name: &str => {
        attrs = attrs.push_master(::ip_route::utils::get_ifindex_str(name));
    }
    "type", kind: &str => {
        attrs = attrs
            .nested_linkinfo()
            .push_kind_bytes(kind.as_bytes())
            .end_nested();
    }
    "nomaster" => {
        attrs = attrs.push_master(u32::MAX);
    }
    "novf" => {}
    ifname: &str => {
        attrs = attrs.push_ifname_bytes(ifname.as_bytes());
    }
}
