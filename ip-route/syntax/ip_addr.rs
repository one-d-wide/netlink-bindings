use super::{command, group};
use crate::{ANY, Map};
use quote::quote;

command! {
    const COMMANDS;
    prelude => {
        use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};
        use ipnet::IpNet;
        use netlink_bindings::traits::Pusher;
        use netlink_bindings::rt_addr;

        fn addr_family(addr: &IpAddr) -> u8 {
            match addr {
                IpAddr::V4(_) => libc::AF_INET as u8,
                IpAddr::V6(_) => libc::AF_INET6 as u8,
            }
        }

        let mut header = rt_addr::Ifaddrmsg::default();
        let mut buf = Vec::new();
        let mut attrs = rt_addr::PushAddrAttrs::new(&mut buf);

        let mut flags: u32 = 0;
        let mut flags_mask: u32 = 0;
        let mut cacheinfo = rt_addr::IfaCacheinfo::default();
    }
    "ip addr /list/show": ADDR_SHOW:
    filter_map |desc, tok| => {
        match desc.rsplit(" ").next().unwrap() {
            "" | "list" | "show" => {},
            _ => panic!("{desc:?}"),
        };

        quote! {{
            #tok

            if flags == 0 {
            } else if flags <= 0xff {
                header.ifa_flags = flags as u8;
            } else {
                attrs = attrs.push_flags(flags);
            }

            if cacheinfo != Default::default() {
                attrs = attrs.push_cacheinfo(cacheinfo);
            }

            let mut req = rt_addr::Request::new().op_getaddr_dump(&header);
            req.encode().as_vec_mut().extend_from_slice(attrs.as_vec());
            req
        }}
    },
    "ip addr add/change/replace/del": ADDR_MOD:
    filter_map |desc, tok| => {
        let req = match desc.rsplit(" ").next().unwrap() {
            "add" => quote! {
                rt_addr::Request::new()
                    .set_create()
                    .set_excl()
                    .op_newaddr_do(&header)
            },
            "change" => quote! {
                rt_addr::Request::new()
                    .set_replace()
                    .op_newaddr_do(&header)
            },
            "replace" => quote! {
                rt_addr::Request::new()
                    .set_create()
                    .set_replace()
                    .op_newaddr_do(&header)
            },
            "del" => quote! {
                rt_addr::Request::new()
                    .op_deladdr_do(&header)
            },
            _ => panic!("{desc:?}"),
        };

        quote! {{
            #tok

            if flags == 0 {
            } else if flags < 256 {
                header.ifa_flags = flags as u8;
            } else {
                attrs = attrs.push_flags(flags);
            }

            if cacheinfo != Default::default() {
                attrs = attrs.push_cacheinfo(cacheinfo);
            }

            let mut req = #req;
            req.encode().as_vec_mut().extend_from_slice(attrs.as_vec());
            req
        }}
    },
    "ip addr flush/save/restore/showdump": [ANY.star()]:
    filter_map |desc, _tok| => {
        let err = format!("{desc} is unimplemented");
        quote! {
            compile_error!(#err);
        }
    },
}

const ADDR_SHOW: Map = Map::Any(&[FLAGS, DEV, SCOPE]).star();
const ADDR_MOD: Map = Map::Any(&[FLAGS, IFADDR, IFADDR_OPTIONS, DEV, LIFETIME, CONFFLAGS]).star();

group! {
    const FLAGS;
    ~ repeat: star,
    "-4" => { header.ifa_family = libc::AF_INET as u8; }
    "-6" => { header.ifa_family = libc::AF_INET6 as u8; }
}

group! {
    const DEV;
    "dev", ifname: &str => {
        header.ifa_index = ::ip_route::utils::get_ifindex_str(ifname);
    }
    "dev-ifindex", ifindex: u32 => {
        header.ifa_index = ifindex;
    }
}

group! {
    const IFADDR_OPTIONS;
    ~ repeat: star,
    "broadcast", addr: Ipv4Addr => {
        attrs = attrs.push_broadcast(addr);
    }
    "brd", addr: Ipv4Addr => {
        attrs = attrs.push_broadcast(addr);
    }
    "anycast", addr: IpAddr => {
        attrs = attrs.push_anycast(addr);
    }
    "label", label: &str => {
        attrs = attrs.push_label_bytes(label.as_bytes());
    }
    &SCOPE => {}
    "metric", metric: u32 => {
        attrs = attrs.push_rt_priority(metric);
    }
    "proto", proto: u8 => {
        attrs = attrs.push_proto(proto);
    }
}

group! {
    const PEER_ADDR;
    ~ init => { let peer_addr; }
    "local", addr: IpNet => {
        peer_addr = addr.addr();
    }
    addr: IpNet => {
        peer_addr = addr.addr();
    }
}

group! {
    const IFADDR;
    &PEER_ADDR, "peer", prefix: IpNet => {
        header.ifa_family = addr_family(&peer_addr);
        header.ifa_prefixlen = prefix.prefix_len();
        attrs = attrs.push_local(peer_addr);
        attrs = attrs.push_address(prefix.addr());
    }
    "local", prefix: IpNet => {
        let addr = prefix.addr();
        header.ifa_family = addr_family(&addr);
        header.ifa_prefixlen = prefix.prefix_len();
        attrs = attrs.push_local(addr);
        attrs = attrs.push_address(addr);
    }
    prefix: IpNet => {
        let addr = prefix.addr();
        header.ifa_family = addr_family(&addr);
        header.ifa_prefixlen = prefix.prefix_len();
        attrs = attrs.push_local(addr);
        attrs = attrs.push_address(addr);
    }
}

group! {
    const SCOPE;
    "scope", "host" => {
        header.ifa_scope = libc::RT_SCOPE_HOST as u8;
    }
    "scope", "link" => {
        header.ifa_scope = libc::RT_SCOPE_LINK as u8;
    }
    "scope", "global" => {
        header.ifa_scope = libc::RT_SCOPE_UNIVERSE as u8;
    }
    "scope", "site" => {
        header.ifa_scope = libc::RT_SCOPE_SITE as u8;
    }
    "scope", scope: u8 => {
        header.ifa_scope = scope;
    }
}

group! {
    const LIFETIME;
    "valid_lft", "forever" => {
        cacheinfo.ifa_valid = 0xffffffffu32;
    }
    "valid_lft", lft: u32 => {
        cacheinfo.ifa_valid = lft;
    }
    "preferred_lft", "forever" => {
        cacheinfo.ifa_prefered = 0xffffffffu32;
    }
    "preferred_lft", lft: u32 => {
        cacheinfo.ifa_prefered = lft;
    }
}

group! {
    const CONFFLAGS;
    ~ repeat: star,
    "home" => {
        flags |= 0x10; // IFA_F_HOMEADDRESS
    }
    "nodad" => {
        flags |= 0x02; // IFA_F_NODAD
    }
    "mngtmpaddr" => {
        flags |= 0x100; // IFA_F_MANAGETEMPADDR
    }
    "noprefixroute" => {
        flags |= 0x200; // IFA_F_NOPREFIXROUTE
    }
    "autojoin" => {
        flags |= 0x400; // IFA_F_MCAUTOJOIN
    }
}
