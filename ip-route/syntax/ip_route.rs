use super::{command, group};
use crate::{ANY, Map};
use quote::quote;

command! {
    const COMMANDS;
    prelude => {
        use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};
        use ipnet::IpNet;
        use netlink_bindings::traits::Pusher;
        use netlink_bindings::rt_route;

        fn addr_bits(addr: &IpAddr) -> u8 {
            match addr {
                IpAddr::V4(_) => 32,
                IpAddr::V6(_) => 128,
            }
        }
        fn addr_family(addr: &IpAddr) -> u8 {
            match addr {
                IpAddr::V4(_) => libc::AF_INET as u8,
                IpAddr::V6(_) => libc::AF_INET6 as u8,
            }
        }

        use rt_route::RtmType;

        let mut header = rt_route::Rtmsg::new();
        let mut buf = Vec::new();
        let mut attrs = rt_route::PushRouteAttrs::new(&mut buf);
    }
    "ip route get": [FLAGS, ROUTE_GET]:
    |tok| => {
        header.rtm_family = libc::AF_INET as u8;

        { #tok }

        if header.rtm_family == libc::AF_INET as u8 {
            header.rtm_flags |= rt_route::RtmFlag::LookupTable as u32;
        }

        let mut req = rt_route::Request::new().op_getroute_do(&header);
        req.encode().as_vec_mut().extend_from_slice(attrs.as_vec());
        req
    }
    "ip route list/show": [FLAGS, SELECTOR]:
    filter_map |desc, tok| => {
        match desc.rsplit(" ").next().unwrap() {
            "list" | "show" => {},
            _ => panic!("{desc:?}"),
        };

        quote! {
            header.rtm_family = libc::AF_INET as u8;

            #tok

            let parsed_attrs = rt_route::RouteAttrs::new(attrs.as_vec());
            if header.rtm_table == 0 && parsed_attrs.get_table().is_err() {
                attrs = attrs.push_table(libc::RT_TABLE_MAIN as u32);
            } else if header.rtm_table != 0 {
                attrs = attrs.push_table(header.rtm_table as u32);
                header.rtm_table = 0;
            }

            let mut req = rt_route::Request::new().op_getroute_dump(&header);
            req.encode().as_vec_mut().extend_from_slice(attrs.as_vec());
            req
        }
    },
    "ip route add/change/append/replace/del": [FLAGS, ROUTE]:
    filter_map |desc, tok| => {
        let req = match desc.rsplit(" ").next().unwrap() {
            "add" => quote! {
                rt_route::Request::new()
                    .set_create()
                    .set_excl()
                    .op_newroute_do(&header)
            },
            "change" => quote! {
                rt_route::Request::new()
                    .set_change()
                    .op_newroute_do(&header)
            },
            "append" => quote! {
                rt_route::Request::new()
                    .set_append()
                    .op_newroute_do(&header)
            },
            "replace" => quote! {
                rt_route::Request::new()
                    .set_replace()
                    .op_newroute_do(&header)
            },
            "del" => quote! {{
                header.rtm_type = 0;
                header.rtm_protocol = 0;
                header.rtm_scope = libc::RT_SCOPE_NOWHERE as u8;
                rt_route::Request::new()
                    .op_delroute_do(&header)
            }},
            _ => panic!("{desc:?}"),
        };

        quote! {
            let mut buf = Vec::new();
            let mut metrics = rt_route::PushMetrics::new(&mut buf);
            let mut mxlock = 0;

            // header.rtm_family = libc::AF_INET as u8;
            header.rtm_protocol = libc::RTPROT_BOOT as u8;
            header.rtm_scope = 255; // Sentinel for "unset"
            header.rtm_type = libc::RTN_UNICAST as u8;

            #tok

            if header.rtm_family == 0 {
                header.rtm_family = libc::AF_INET as u8;
            }

            let parsed_attrs = rt_route::RouteAttrs::new(attrs.as_vec());

            if header.rtm_table == 0 && parsed_attrs.get_table().is_err() {
                if header.rtm_type == libc::RTN_LOCAL as u8
                    || header.rtm_type == libc::RTN_BROADCAST as u8
                    || header.rtm_type == libc::RTN_NAT as u8
                    || header.rtm_type == libc::RTN_ANYCAST as u8
                {
                    header.rtm_table = libc::RT_TABLE_LOCAL as u8;
                } else {
                    header.rtm_table = libc::RT_TABLE_MAIN as u8;
                }
            }

            if header.rtm_scope == 255 {
                if header.rtm_family == libc::AF_INET6 as u8
                    || header.rtm_family == libc::AF_MPLS as u8
                {
                    header.rtm_scope = libc::RT_SCOPE_UNIVERSE as u8;
                } else if header.rtm_type == libc::RTN_LOCAL as u8
                    || header.rtm_type == libc::RTN_NAT as u8
                {
                    header.rtm_scope = libc::RT_SCOPE_HOST as u8;
                } else if header.rtm_type == libc::RTN_BROADCAST as u8
                    || header.rtm_type == libc::RTN_MULTICAST as u8
                    || header.rtm_type == libc::RTN_ANYCAST as u8
                {
                    header.rtm_scope = libc::RT_SCOPE_LINK as u8;
                } else if header.rtm_type == libc::RTN_UNICAST as u8
                    && parsed_attrs.get_gateway().is_err()
                    && parsed_attrs.get_via().is_err()
                {
                    header.rtm_scope = libc::RT_SCOPE_LINK as u8;
                } else {
                    header.rtm_scope = libc::RT_SCOPE_UNIVERSE as u8;
                }
            }

            let mut req = #req;
            req.encode().as_vec_mut().extend_from_slice(attrs.as_vec());
            if mxlock != 0 {
                metrics = metrics.push_lock(mxlock);
            }
            if !metrics.as_vec().is_empty() {
                req.encode().nested_metrics().as_vec_mut().extend_from_slice(metrics.as_vec());
            }

            req
        }
    },
    "ip route flush/save/restore/showdump": [ANY.star()]:
    filter_map |desc, _tok| => {
        let err = format!("{desc} is unimplemented");
        quote! {
            compile_error!(#err);
        }
    },
}

const ROUTE: Map = Map::Any(&[INFO_SPEC, OPTIONS, NODE_SPEC]).star();
const NODE_SPEC: Map = Map::Any(&[TABLE_ID, PROTO, TYPE, SCOPE, NODE_SPEC_INNER]).star();
const SELECTOR: Map = Map::Any(&[TABLE_ID, PROTO, TYPE, SCOPE, SELECTOR_INNER]).star();

group! {
    const FLAGS;
    ~ repeat: star,
    "-4" => { header.rtm_family = libc::AF_INET as u8; }
    "-6" => { header.rtm_family = libc::AF_INET6 as u8; }
}

group! {
    const ROUTE_GET;
    ~ repeat: star,
    "from", prefix: IpNet => {
        header.rtm_family = addr_family(&prefix.addr());
        header.rtm_src_len = prefix.prefix_len();
        attrs = attrs.push_src(prefix.addr());
    }
    "iif", ifname: &str => {
        attrs = attrs.push_iif(::ip_route::utils::get_ifindex_str(ifname));
    }
    "iif-index", ifindex: u32 => {
        attrs = attrs.push_iif(ifindex);
    }
    "oif", ifname: &str => {
        attrs = attrs.push_oif(::ip_route::utils::get_ifindex_str(ifname));
    }
    "oif-index", ifindex: u32 => {
        attrs = attrs.push_oif(ifindex);
    }
    "fibmatch" => {
        header.rtm_flags |= rt_route::RtmFlag::FibMatch as u32;
    }
    "connected" => {
        compile_error!("connected flag is unimplemented");
    }
    "notify" => {
        header.rtm_flags |= rt_route::RtmFlag::Notify as u32;
    }
    "vrf", name: &str => {
        compile_error!("vrf is unimplemented");
    }
    "uid", uid: u32 => {
        attrs = attrs.push_uid(uid);
    }
    "mark", mark: u32 => {
        attrs = attrs.push_mark(mark);
    }
    "tos", tos: &str as "static-str" => {
        compile_error!("Unknown tos STRING. Use its numeric code");
    }
    "tos", tos: u8 as "hex" => {
        header.rtm_tos = tos;
    }
    &IPPROTO => {}
    "sport", num: u16 => {
        attrs = attrs.push_sport(num);
    }
    "dport", num: u16 => {
        attrs = attrs.push_dport(num);
    }
    "as", addr: IpAddr => {
        header.rtm_family = addr_family(&addr);
        attrs = attrs.push_newdst(addr);
    }
    "flowlabel", flowlabel: u32 => {
        attrs = attrs.push_flowlabel(flowlabel);
    }
    "to", "default" => {
        header.rtm_dst_len = 0;
    }
    "to", addr: IpAddr => {
        header.rtm_family = addr_family(&addr);
        header.rtm_dst_len = addr_bits(&addr);;
        attrs = attrs.push_dst(addr);
    }
    "default" => {
        header.rtm_dst_len = 0;
    }
    addr: IpAddr => {
        header.rtm_family = addr_family(&addr);
        header.rtm_dst_len = addr_bits(&addr);;
        attrs = attrs.push_dst(addr);
    }
}

group! {
    const TABLE_ID;
    "table", "local" => {
        // attrs = attrs.push_table(libc::RT_TABLE_LOCAL as u32);
        header.rtm_table = libc::RT_TABLE_LOCAL as u8;
    }
    "table", "main" => {
        // attrs = attrs.push_table(libc::RT_TABLE_MAIN as u32);
        header.rtm_table = libc::RT_TABLE_MAIN as u8;
    }
    "table", "default" => {
        // attrs = attrs.push_table(libc::RT_TABLE_DEFAULT as u32);
        header.rtm_table = libc::RT_TABLE_DEFAULT as u8;
    }
    "table", table_id: u32 => {
        if table_id < 255 {
            header.rtm_table = table_id as u8;
        } else {
            attrs = attrs.push_table(table_id);
        }
    }
}

group! {
    const IPPROTO;
    "ipproto", "icmp" => {
        attrs = attrs.push_ip_proto(libc::IPPROTO_ICMP as u8);
    }
    "ipproto", "icmpv6" => {
        attrs = attrs.push_ip_proto(libc::IPPROTO_ICMPV6 as u8);
    }
    "ipproto", "udp" => {
        attrs = attrs.push_ip_proto(libc::IPPROTO_UDP as u8);
    }
    "ipproto", "tcp" => {
        attrs = attrs.push_ip_proto(libc::IPPROTO_TCP as u8);
    }
    "ipproto", proto: u8 => {
        attrs = attrs.push_ip_proto(proto);
    }
}

group! {
    const PROTO;
    "proto", "kernel" => {
        header.rtm_protocol = libc::RTPROT_KERNEL as u8;
    }
    "proto", "boot" => {
        header.rtm_protocol = libc::RTPROT_BOOT as u8;
    }
    "proto", "static" => {
        header.rtm_protocol = libc::RTPROT_STATIC as u8;
    }
    "proto", proto: u8 => {
        header.rtm_protocol = proto;
    }
}

group! {
    const SCOPE;
    "scope", "host" => {
        header.rtm_scope = libc::RT_SCOPE_HOST as u8;
    }
    "scope", "link" => {
        header.rtm_scope = libc::RT_SCOPE_LINK as u8;
    }
    "scope", "global" => {
        header.rtm_scope = libc::RT_SCOPE_UNIVERSE as u8;
    }
    "scope", scope: u8 => {
        header.rtm_scope = scope;
    }
}

group! {
    const TYPE;
    "unicast" => {
        header.rtm_type = RtmType::Unicast as u8;
    }
    "local" => {
        header.rtm_type = RtmType::Local as u8;
    }
    "broadcast" => {
        header.rtm_type = RtmType::Broadcast as u8;
    }
    "multicast" => {
        header.rtm_type = RtmType::Multicast as u8;
    }
    "throw" => {
        header.rtm_type = RtmType::Throw as u8;
    }
    "unreachable" => {
        header.rtm_type = RtmType::Unreachable as u8;
    }
    "prohibit" => {
        header.rtm_type = RtmType::Prohibit as u8;
    }
    "blackhole" => {
        header.rtm_type = RtmType::Blackhole as u8;
    }
    "nat" => {
        header.rtm_type = RtmType::Nat as u8;
    }
}

group! {
    const DEV;
    "dev", dev: &str => {
        attrs = attrs.push_oif(::ip_route::utils::get_ifindex_str(dev));
    }
    "dev-index", ifindex: u32 => {
        attrs = attrs.push_oif(ifindex);
    }
}

group! {
    const SELECTOR_INNER;
    &DEV => {}
}

group! {
    const INFO_SPEC;
    &DEV => {}
    "via", addr: IpAddr => {
        if header.rtm_family == 0
            || header.rtm_family == addr_family(&addr)
        {
            attrs = attrs.push_gateway(addr);
        } else {
            attrs = attrs.push_via(addr);
        }
    }
    "nexthop" => {
        compile_error!("nexthop is unimplemented");
    }
    "encap" => {
        compile_error!("encap is unimplemented");
    }
}

group! {
    const NODE_SPEC_INNER;
    "from", prefix: IpNet => {
        header.rtm_family = addr_family(&prefix.addr());
        header.rtm_src_len = prefix.prefix_len();
        attrs = attrs.push_src(prefix.addr());
    }
    "tos", tos: u8 as "hex" => {
        header.rtm_tos = tos;
    }
    "metric", metric: u32 => {
        attrs = attrs.push_priority(metric);
    }
    "ttl-propagate", "enabled" => {
        attrs = attrs.push_ttl_propagate(1);
    }
    "ttl-propagate", "disabled" => {
        attrs = attrs.push_ttl_propagate(0);
    }
    "default" => {
        header.rtm_dst_len = 0;
        match header.rtm_family as i32 {
            libc::AF_INET => attrs = attrs.push_dst(Ipv4Addr::UNSPECIFIED.into()),
            libc::AF_INET6 => attrs = attrs.push_dst(Ipv6Addr::UNSPECIFIED.into()),
            _ => {},
        };
    }
    prefix: IpNet => {
        header.rtm_family = addr_family(&prefix.addr());
        header.rtm_dst_len = prefix.prefix_len();
        attrs = attrs.push_dst(prefix.addr());
    }
}

group! {
    const OPTIONS;
    "mtu", number: u32 => {
        metrics = metrics.push_mtu(number);
    }
    "advmss", number: u32 => {
        metrics = metrics.push_advmss(number);
    }
    "rtt", time: u32 as "time-ms" "mul8-suffix" => {
        metrics = metrics.push_rtt(time);
    }
    "rttvar", time: u32 as "time-ms" "mul4-suffix" => {
        metrics = metrics.push_rttvar(time);
    }
    "reordering", number: u32 => {
        metrics = metrics.push_reordering(number);
    }
    "window", number: u32 => {
        metrics = metrics.push_window(number);
    }
    "cwnd", number: u32 => {
        metrics = metrics.push_cwnd(number);
    }
    "ssthresh", number: u32 => {
        metrics = metrics.push_ssthresh(number);
    }
    "hoplimit", number: u32 => {
        metrics = metrics.push_hoplimit(number);
    }
    "initcwnd", number: u32 => {
        metrics = metrics.push_initcwnd(number);
    }
    "initrwnd", number: u32 => {
        metrics = metrics.push_initrwnd(number);
    }
    "rto_min", time: u32 as "time-ms" => {
        metrics = metrics.push_rto_min(time);
        mxlock |= 1 << 13; // RTAX_RTO_MIN
    }
    "features", "ecn" => {
        metrics = metrics.push_features(1);
    }
    "quickack", val: u32 => {
        metrics = metrics.push_quickack(val);
    }
    "congctl", name: &str => {
        metrics = metrics.push_cc_algo_bytes(name.as_bytes());
    }
    "pref", "low" => {
        attrs = attrs.push_pref(0x03);
    }
    "pref", "medium" => {
        attrs = attrs.push_pref(0x00);
    }
    "pref", "high" => {
        attrs = attrs.push_pref(0x01);
    }
    "pref", pref: u8 => {
        attrs = attrs.push_pref(pref);
    }
    "expires", time: u32 => {
        attrs = attrs.push_expires(time);
    }
    "fastopen_no_cookie", val: u32 => {
        metrics = metrics.push_fastopen_no_cookie(val);
    }
    "src", addr: IpAddr => {
        attrs = attrs.push_prefsrc(addr);
    }
    "as", "to", addr: IpAddr => {
        attrs = attrs.push_newdst(addr);
    }
    "as", addr: IpAddr => {
        attrs = attrs.push_newdst(addr);
    }
    "iif", ifname: &str => {
        attrs = attrs.push_iif(::ip_route::utils::get_ifindex_str(ifname));
    }
    "iif-index", ifindex: u32 => {
        attrs = attrs.push_iif(ifindex);
    }
    "oif", ifname: &str => {
        attrs = attrs.push_oif(::ip_route::utils::get_ifindex_str(ifname));
    }
    "oif-index", ifindex: u32 => {
        attrs = attrs.push_oif(ifindex);
    }
}
