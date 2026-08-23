use super::{command, group};
use crate::{ANY, Map, ip_route::IPPROTO};
use quote::quote;

command! {
    const COMMANDS;
    prelude => {
        use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};
        use ipnet::IpNet;
        use netlink_bindings::traits::Pusher;
        use netlink_bindings::rt_rule;

        fn addr_family(addr: &IpAddr) -> u8 {
            match addr {
                IpAddr::V4(_) => libc::AF_INET as u8,
                IpAddr::V6(_) => libc::AF_INET6 as u8,
            }
        }

        use rt_rule::{FibRulePortRange, FibRuleUidRange, FrAct};

        let mut header = rt_rule::FibRuleHdr::new();
        let mut buf = Vec::new();
        let mut attrs = rt_rule::PushFibRuleAttrs::new(&mut buf);
    }
    "ip rule /list/show": [FLAGS, WARN_ON_ARGS.may()]:
    filter_map |desc, tok| => {
        match desc.rsplit(" ").next().unwrap() {
            "" | "list" | "show" => {},
            _ => panic!("{desc:?}"),
        }

        quote! {
            header.family = libc::AF_INET as u8;

            #tok

            let mut req = rt_rule::Request::new().op_getrule_dump(&header);
            req.encode().as_vec_mut().extend_from_slice(attrs.as_vec());
            req
        }
    },
    "ip rule add/del": [FLAGS, Map::Any(&[SELECTOR, ACTION]).star()]:
    filter_map |desc, tok| => {
        let req = match desc.rsplit(" ").next().unwrap() {
            "add" => quote! {{
                if header.action == 0 {
                    header.action = FrAct::ToTbl as u8;
                }
                rt_rule::Request::new()
                    .set_create()
                    .set_excl()
                    .op_newrule_do(&header)
            }},
            "del" => quote! {
                rt_rule::Request::new()
                    .op_delrule_do(&header)
            },
            _ => panic!("{desc:?}"),
        };

        quote! {
            header.family = libc::AF_INET as u8;

            #tok

            let mut req = #req;
            req.encode().as_vec_mut().extend_from_slice(attrs.as_vec());
            req
        }
    },
    "ip rule flush/save/restore": [ANY.star()]:
    filter_map |desc, _tok| => {
        let err = format!("{desc} is unimplemented");
        quote! {
            compile_error!(#err);
        }
    },
}

const WARN_ON_ARGS: Map = Map::All(&[
    ANY.plus(),
    Map::Code(stringify! {
        compile_error!("Don't use arguments with `ip rule list ...`, kernel doesn't do filtering itself");
    }),
]);

group! {
    const FLAGS;
    ~ repeat: star,
    "-4" => { header.family = libc::AF_INET as u8; }
    "-6" => { header.family = libc::AF_INET6 as u8; }
}

group! {
    const PREFIX;
    ~ init => { let prefix: IpNet; }
    "default" => {
        prefix = if header.family == libc::AF_INET6 as u8 {
            IpNet::new_assert(Ipv6Addr::UNSPECIFIED.into(), 128)
        } else {
            header.family = libc::AF_INET as u8;
            IpNet::new_assert(Ipv4Addr::UNSPECIFIED.into(), 32)
        };
    }
    _prefix: IpNet => {
        prefix = _prefix;
    }
}

group! {
    const SELECTOR;
    ~ repeat: star,
    "not" => {
        header.flags |= 0x02; // FIB_RULE_INVERT
    }
    "from", &PREFIX => {
        header.family = addr_family(&prefix.addr());
        header.src_len = prefix.prefix_len();
        attrs = attrs.push_src(prefix.addr());
    }
    "to", &PREFIX => {
        header.family = addr_family(&prefix.addr());
        header.dst_len = prefix.prefix_len();
        attrs = attrs.push_dst(prefix.addr());
    }
    "tos", tos: &str as "static-str" => {
        compile_error!("Unknown tos STRING. Use its numeric code");
    }
    "tos", tos: u8 as "hex" => {
        header.tos = tos;
    }
    "fwmark", val: (u32, u32) as "format:0/1" => {
        attrs = attrs.push_fwmark(val.0);
        attrs = attrs.push_fwmask(val.1);
    }
    "fwmark", val: u32 => {
        attrs = attrs.push_fwmark(val);
    }
    "iif", ifname: &str => {
        attrs = attrs.push_iifname_bytes(ifname.as_bytes());
    }
    "oif", ifname: &str => {
        attrs = attrs.push_oifname_bytes(ifname.as_bytes());
    }
    "pref", num: u32 => {
        attrs = attrs.push_priority(num);
    }
    ("prio" | "priority"), num: u32 => {
        attrs = attrs.push_priority(num);
    }
    "l3mdev" => {
        attrs = attrs.push_l3mdev(1);
    }
    "uidrange", range: (u32, u32) as "format:{}-{}" => {
        attrs = attrs.push_uid_range(FibRuleUidRange { start: range.0, end: range.1 });
    }
    "sport", v: (u16, Option<u16>, Option<u16>) as "format:{}[-{}][/{}]" => {
        attrs = attrs.push_sport_range(FibRulePortRange { start: v.0, end: v.1.unwrap_or(v.0) });
        if let Some(mask) = v.2 {
            attrs = attrs.push_sport_mask(mask);
        }
    }
    &IPPROTO => {},
    "dport", v: (u16, Option<u16>, Option<u16>) as "format:{}[-{}][/{}]" => {
        attrs = attrs.push_dport_range(FibRulePortRange { start: v.0, end: v.1.unwrap_or(v.0) });
        if let Some(mask) = v.2 {
            attrs = attrs.push_dport_mask(mask);
        }
    }
    "dscp", v: (u8, u8) as "format:0/1" => {
        attrs = attrs.push_dscp(v.0);
        if v.1 != 0x3f { // DSCP_MAX_MASK
            attrs = attrs.push_dscp_mask(v.1);
        }
    }
    "dscp", v: u8 => {
        attrs = attrs.push_dscp(v);
    }
    "flowlabel", v: (u32, Option<u32>) as "format:{}[/{}]" => {
        attrs = attrs.push_flowlabel(v.0);
        attrs = attrs.push_flowlabel_mask(v.1.unwrap_or(0xfffff)); // LABEL_MAX_MASK
    }
}

group! {
    const ACTION;
    ~ repeat: plus,
    &TABLE => {}
    &PROTOCOL => {}
    "lookup", "local" => {
        header.table = libc::RT_TABLE_LOCAL as u8;
        header.action = FrAct::ToTbl as u8;
    }
    "lookup", "main" => {
        header.table = libc::RT_TABLE_MAIN as u8;
        header.action = FrAct::ToTbl as u8;
    }
    "lookup", "default" => {
        header.table = libc::RT_TABLE_DEFAULT as u8;
        header.action = FrAct::ToTbl as u8;
    }
    "lookup", table_id: u32 => {
        header.action = FrAct::ToTbl as u8;
        if table_id < 256 {
            header.table = table_id as u8;
        } else {
            attrs = attrs.push_table(table_id);
        }
    }
    "nat" => {
        compile_error!("nat rules are deprecated");
    }
    "goto", num: u32 => {
        header.action = rt_rule::FrAct::Goto as u8;
        attrs = attrs.push_goto(num);
    }
    "blackhole" => {
        header.action = FrAct::Blackhole as u8;
    }
    "unreachable" => {
        header.action = FrAct::Unreachable as u8;
    }
    "prohibit" => {
        header.action = FrAct::Prohibit as u8;
    }
    "suppress_prefixlength", num: u32 => {
        attrs = attrs.push_suppress_prefixlen(num);
    }
    "suppress_ifgroup", num: u32 => {
        attrs = attrs.push_suppress_ifgroup(num);
    }
}

group! {
    const TABLE;
    "table", "local" => {
        header.table = libc::RT_TABLE_LOCAL as u8;
        // header.action = FrAct::ToTbl as u8;
    }
    "table", "main" => {
        header.table = libc::RT_TABLE_MAIN as u8;
        // header.action = FrAct::ToTbl as u8;
    }
    "table", "default" => {
        header.table = libc::RT_TABLE_DEFAULT as u8;
        // header.action = FrAct::ToTbl as u8;
    }
    "table", table_id: u32 => {
        // header.action = FrAct::ToTbl as u8;
        if table_id < 256 {
            header.table = table_id as u8;
        } else {
            attrs = attrs.push_table(table_id);
        }
    }
}

group! {
    const PROTOCOL;
    "protocol", "kernel" => {
        attrs = attrs.push_protocol(libc::RTPROT_KERNEL as u8);
    }
    "protocol", "boot" => {
        attrs = attrs.push_protocol(libc::RTPROT_BOOT as u8);
    }
    "protocol", "static" => {
        attrs = attrs.push_protocol(libc::RTPROT_STATIC as u8);
    }
    "protocol", name: &str as "static-str" => {
        compile_error!("Unknown protocol STRING. Use its numeric code");
    }
    "protocol", proto: u8 => {
        attrs = attrs.push_protocol(proto);
    }

}
