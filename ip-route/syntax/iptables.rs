use super::{command, group};
use crate::{ANY, Map};
use proc_macro2::TokenStream;
use quote::quote;

fn get_header(desc: &str) -> TokenStream {
    let fam = match desc.split(" ").next().unwrap() {
        "iptables" => quote! { libc::AF_INET },
        "ip6tables" => quote! { libc::AF_INET6 },
        _ => panic!("{desc:?}"),
    };

    quote! {
        Nfgenmsg {
            nfgen_family: #fam as u8,
            ..Default::default()
        }
    }
}

command! {
    const COMMANDS;
    prelude => {
        use std::mem::drop;
        use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};
        use ipnet::{IpNet, Ipv4Net};
        use netlink_bindings::traits::Pusher;
        use netlink_bindings::nftables;

        use nftables::{
            Nfgenmsg, Registers, PayloadBase, MetaKeys, CmpOps, RangeOps, VerdictCode,
            LogLevel, XtLogInfo, XtLogFlag,
        };

        fn clone_addr(buf: &mut [u8; 16], addr: IpAddr) {
            match addr {
                IpAddr::V4(addr) => buf[..4].clone_from_slice(&addr.to_bits().to_be_bytes()),
                IpAddr::V6(addr) => buf[..16].clone_from_slice(&addr.to_bits().to_be_bytes()),
            }
        }

        macro_rules! ip_off {
            ($h:ident, proto) => { ip_off!($h, ipv4: 9, ipv6: 6) };
            ($h:ident, src) => { ip_off!($h, ipv4: 12, ipv6: 8) };
            ($h:ident, dst) => { ip_off!($h, ipv4: 16, ipv6: 24) };
            ($h:ident, ipv4: $ipv4:expr, ipv6: $ipv6:expr) => {
                if $h.nfgen_family == libc::AF_INET as u8 {
                    $ipv4
                } else {
                    $ipv6
                }
            };
        };

        macro_rules! op {
            ($attrs:ident, load, base: $base:ident, off: $off:expr, len: $len:expr) => {
                $attrs = $attrs
                    .nested_elem()
                    .nested_data_payload()
                    .push_dreg(Registers::Reg1 as u32)
                    .push_base(PayloadBase::$base as u32)
                    .push_offset($off)
                    .push_len($len)
                    .end_nested()
                    .end_nested();
            };
            ($attrs:ident, cmp, op: $op:expr, val: $val:expr) => {
                $attrs = $attrs
                    .nested_elem()
                    .nested_data_cmp()
                    .push_sreg(Registers::Reg1 as u32)
                    .push_op($op as u32)
                    .nested_data()
                    .push_value(&$val)
                    .end_nested()
                    .end_nested()
                    .end_nested()
            };
            ($attrs:ident, range, op: $op:expr, from: $from:expr, to: $to:expr) => {
                $attrs = $attrs
                    .nested_elem()
                    .nested_data_range()
                    .push_sreg(Registers::Reg1 as u32)
                    .push_op($op as u32)
                    .nested_from_data()
                    .push_value(&$from)
                    .end_nested()
                    .nested_to_data()
                    .push_value(&$to)
                    .end_nested()
                    .end_nested()
                    .end_nested()
            };
            ($attrs:ident, mask_xor, mask: $mask:expr, xor: $xor:expr, len: $len:expr) => {
                $attrs = $attrs
                    .nested_elem()
                    .nested_data_bitwise()
                    .push_sreg(Registers::Reg1 as u32)
                    .push_dreg(Registers::Reg1 as u32)
                    .push_len($len)
                    .nested_mask()
                    .push_value(&$mask)
                    .end_nested()
                    .nested_xor()
                    .push_value(&$xor)
                    .end_nested()
                    .end_nested()
                    .end_nested()
            };
            ($attrs:ident, load_meta, key: $key:expr) => {
                $attrs = $attrs
                    .nested_elem()
                    .nested_data_meta()
                    .push_key($key as u32)
                    .push_dreg(Registers::Reg1 as u32)
                    .end_nested()
                    .end_nested();
            };
            ($attrs:ident, counter $(, bytes: $bytes:expr)? $(, bytes: $packets:expr)?) => {
                $attrs = $attrs
                    .nested_elem()
                    .nested_data_counter()
                    .push_bytes({ 0 $(; $bytes )? })
                    .push_packets({ 0 $(; $packets )? })
                    .end_nested()
                    .end_nested();
            };
            ($attrs:ident, store_verdict, val: $val:expr $(, chain: $chain:expr)?) => {
                $attrs = $attrs
                    .nested_elem()
                    .nested_data_immediate()
                    .push_dreg(Registers::RegVerdict as u32)
                    .nested_data()
                    .nested_verdict()
                    .push_code($val as u32)
                 $( .push_chain_bytes($chain) )?
                    .end_nested()
                    .end_nested()
                    .end_nested()
                    .end_nested();
            };
        }
    }
    "iptables/ip6tables -L/--list": [FLAGS, MAY_TABLE, CHAIN.may()],
    filter_map |desc, tok| => {
        let header = get_header(desc);
        quote! {
            let header = #header;
            let mut buf = Vec::new();
            let mut attrs = nftables::PushRuleAttrs::new(&mut buf);
            #tok
            let mut req = nftables::Request::new().op_getrule_dump(&header);
            req.encode().as_vec_mut().extend_from_slice(attrs.as_vec());
            req
        }
    },
    "iptables/ip6tables -I/--insert": [FLAGS, MAY_TABLE, CHAIN, RULE_NUM.may(), RULE_SPEC],
    filter_map |desc, tok| => {
        let header = get_header(desc);
        quote! {
            let header = #header;
            let mut buf = Vec::new();
            let mut attrs = nftables::PushRuleAttrs::new(&mut buf);
            #tok
            let mut req = nftables::Request::new().set_create().op_newrule_do(&header);
            req.encode().as_vec_mut().extend_from_slice(attrs.as_vec());
            req
        }
    },
    "iptables/ip6tables -D/--delete": [FLAGS, MAY_TABLE, CHAIN, RULE_NUM],
    filter_map |desc, tok| => {
        let header = get_header(desc);
        quote! {
            let header = #header;
            let mut buf = Vec::new();
            let mut attrs = nftables::PushRuleAttrs::new(&mut buf);
            #tok
            let mut req = nftables::Request::new().op_delrule_do(&header);
            req.encode().as_vec_mut().extend_from_slice(attrs.as_vec());
            req
        }
    },
    "iptables/ip6tables -R/--replace": [FLAGS, MAY_TABLE, CHAIN, RULE_NUM, RULE_SPEC]:
    filter_map |desc, tok| => {
        let header = get_header(desc);
        quote! {
            let header = #header;
            let mut buf = Vec::new();
            let mut attrs = nftables::PushRuleAttrs::new(&mut buf);
            #tok
            let mut req = nftables::Request::new().set_replace().op_newrule_do(&header);
            req.encode().as_vec_mut().extend_from_slice(attrs.as_vec());
            req
        }
    },
    "iptables/ip6tables -F/--flush": [FLAGS, MAY_TABLE, CHAIN.may()]:
    filter_map |desc, tok| => {
        let header = get_header(desc);
        quote! {
            let header = #header;
            let mut buf = Vec::new();
            let mut attrs = nftables::PushRuleAttrs::new(&mut buf);
            #tok
            let mut req = nftables::Request::new().op_delrule_do(&header);
            req.encode().as_vec_mut().extend_from_slice(attrs.as_vec());
            req
        }
    },
    "iptables/ip6tables -N/--new": [FLAGS, MAY_TABLE, CHAIN_NAME]:
    filter_map |desc, tok| => {
        let header = get_header(desc);
        quote! {
            let header = #header;
            let mut buf = Vec::new();
            let mut attrs = nftables::PushChainAttrs::new(&mut buf);
            #tok
            let mut req = nftables::Request::new().set_excl().op_newchain_do(&header);
            req.encode().as_vec_mut().extend_from_slice(attrs.as_vec());
            req
        }
    },
    "iptables/ip6tables -D/--delete": [FLAGS, MAY_TABLE, CHAIN_NAME]:
    filter_map |desc, tok| => {
        let header = get_header(desc);
        quote! {
            let mut header = #header;
            let mut buf = Vec::new();
            let mut attrs = nftables::PushChainAttrs::new(&mut buf);
            #tok
            let mut req = nftables::Request::new().op_delchain_do(&header);
            req.encode().as_vec_mut().extend_from_slice(attrs.as_vec());
            req
        }
    },
    "iptables/ip6tables -E/--rename": [ANY.star()]: |_tok| => {
        compile_error!("This creates a chain and moves the rules individually (todo: make a helper maybe)");
    },
    "iptables/ip6tables -Z/--zero": [ANY.star()]: |_tok| => {
        compile_error!("This replaces the rules individually (todo: make a helper maybe)");
    },
}

group! {
    const FLAGS;
    ~ repeat: star,
    "-4" => { header.nfgen_family = libc::AF_INET as u8; }
    "-6" => { header.nfgen_family = libc::AF_INET6 as u8; }
}

group! {
    const MAY_TABLE;
    ~ repeat: may,
    ~ init => { let mut table = "filter"; }
    ~ fini => { attrs = attrs.push_table_bytes(table.as_bytes()); }
    "-t", name: &str => { table = name; }
}

const RULE_SPEC: Map = Map::All(&[
    Map::Code("let mut attrs = attrs.nested_expressions();"),
    Map::Any(&[OPTIONS, MODULE_OPTIONS]).star(),
    TARGET_OPTIONS.may(),
    Map::Code("let mut attrs = attrs.end_nested();"),
]);

group! {
    const RULE_NUM;
    "--handle", handle: u64 => {
        attrs = attrs.push_handle(handle);
    }
    num: u64 => {
        compile_error!("Unlike `iptables` command, kernel expects a rule handle number, not its position.
Obtain a rule handle from `iptables -L` and provided it as --handle.
If you control this rule, consider using .push_userdata().");
    }
}

group! {
    const CHAIN;
    chain: &str => {
        attrs = attrs.push_chain_bytes(chain.as_bytes());
    }
}

group! {
    const CHAIN_NAME;
    chain: &str => {
        attrs = attrs.push_name_bytes(chain.as_bytes());
    }
}

group! {
    const CMP_EQ_OR_NEQ;
    ~ repeat: may,
    ~ init => { let mut cmp_op = CmpOps::Eq; }
    "!" => { cmp_op = CmpOps::Neq; }
}

group! {
    const CMP_NEQ_OR_EQ;
    ~ repeat: may,
    ~ init => { let mut cmp_op = CmpOps::Neq; }
    "!" => { cmp_op = CmpOps::Eq; }
}

group! {
    const RANGE_EQ_OR_NEQ;
    ~ repeat: may,
    ~ init => { let mut range_op = RangeOps::Eq; }
    "!" => { range_op = RangeOps::Neq; }
}

group! {
    const OPTIONS;
    &CMP_EQ_OR_NEQ, ("-p" | "--protocol"), &PROTO_CHOOSE => {
        if header.nfgen_family == libc::AF_INET as u8 {
            op!(attrs, load, base: NetworkHeader, off: ip_off!(header, proto), len: 1);
        } else {
            op!(attrs, load_meta, key: MetaKeys::L4Proto);
        }
        op!(attrs, cmp, op: cmp_op, val: [proto]);
    }
    &CMP_EQ_OR_NEQ, ("-s" | "--source"), &IPV4_ADDR_MASK => {
        if mask > 0 {
            let mask = (u32::MAX << (32 - mask)).to_be();
            let addr = addr.to_bits().to_be() & mask;
            op!(attrs, load, base: NetworkHeader, off: ip_off!(header, src), len: 4);
            if mask != u32::MAX {
                op!(attrs, mask_xor, mask: mask.to_ne_bytes(), xor: 0u32.to_ne_bytes(), len: 4);
            }
            op!(attrs, cmp, op: cmp_op, val: addr.to_ne_bytes());
        }
    }
    &CMP_EQ_OR_NEQ, ("-d" | "--destination"), &IPV4_ADDR_MASK => {
        if mask > 0 {
            let mask = (u32::MAX << (32 - mask)).to_be();
            let addr = addr.to_bits().to_be() & mask;
            op!(attrs, load, base: NetworkHeader, off: ip_off!(header, dst), len: 4);
            if mask != u32::MAX {
                op!(attrs, mask_xor, mask: mask.to_ne_bytes(), xor: 0u32.to_ne_bytes(), len: 4);
            }
            op!(attrs, cmp, op: cmp_op, val: addr.to_ne_bytes());
        }
    }
    &CMP_EQ_OR_NEQ, ("-i" | "--in-interface"), name: &str => {
        op!(attrs, load_meta, key: MetaKeys::Iifname);
        op!(attrs, cmp, op: cmp_op, val: name.as_bytes());
    },
    &CMP_EQ_OR_NEQ, ("-o" | "--out-interface"), name: &str => {
        op!(attrs, load_meta, key: MetaKeys::Oifname);
        op!(attrs, cmp, op: cmp_op, val: name.as_bytes());
    },
    &CMP_NEQ_OR_EQ, ("-f" | "--fragment") => {
        op!(attrs, load, base: NetworkHeader, off: 6, len: 2);
        op!(attrs, mask_xor, mask: [0x1f, 0xff], xor: [0x00, 0x00], len: 2);
        op!(attrs, cmp, op: cmp_op, val: [0x00, 0x00]);
    }
}

group! {
    const TARGET_CHOOSE;
    ~ init => { let target; }
    "DROP" => { target = VerdictCode::Drop; }
    "ACCEPT" => { target = VerdictCode::Accept; }
    // ...
    target: u32 => {}
}

group! {
    const PROTO_CHOOSE;
    ~ init => { let proto; }
    "tcp"     => { proto = libc::IPPROTO_TCP as u8; }
    "udp"     => { proto = libc::IPPROTO_UDP as u8; }
    "udplite" => { proto = libc::IPPROTO_UDPLITE as u8; }
    "icmp"    => { proto = libc::IPPROTO_ICMP as u8; }
    "icmpv6"  => { proto = libc::IPPROTO_ICMPV6 as u8; }
    // ...
    num: u8 => { proto = num; }
}

group! {
    const IPV4_ADDR_MASK;
    ~ init => { let (addr, mask); }
    _addr: Ipv4Addr => {
        mask = 32;
        addr = _addr;
    }
    prefix: Ipv4Net => {
        mask = prefix.prefix_len();
        addr = prefix.addr();
    }
    addr: (&str, &str) as "format:{},{}" => {
        compile_error!("Can't process multiple --source {},{} addresses, split them up");
    }
}

group! {
    const TARGET_OPTIONS;
    ("-j" | "--jump"), &TARGET_CHOOSE => {
        op!(attrs, counter);
        op!(attrs, store_verdict, val: target);
    }
    ("-g" | "--goto"), chain: &str => {
        op!(attrs, counter);
        op!(attrs, store_verdict, val: VerdictCode::Goto, chain: chain.as_bytes());
    }
    ("-j" | "--jump"), &EXT_TARGETS => {}
}

group! {
    const MODULE_OPTIONS;
    "-m", "tcp", &EXT_MATCH_TCP_UDP_PORT, &EXT_MATCH_TCP => {}
    "-m", "udp", &EXT_MATCH_TCP_UDP_PORT => {}
    "-m", "mark", &EXT_MATCH_MARK => {}
    // ...
    "-m", module: &str, &ANY => {
        compile_error!("-m module is unimplemented");
    }
}

group! {
    const EXT_MATCH_TCP;
    ~ repeat: star,
    // ...
}

group! {
    const EXT_MATCH_TCP_UDP_PORT;
    ~ repeat: star,
    &CMP_EQ_OR_NEQ, ("--source-port" | "--sport"), port: u16 => {
        op!(attrs, load, base: TransportHeader, off: 0, len: 2);
        op!(attrs, cmp, op: cmp_op, val: port.to_be_bytes());
    }
    &RANGE_EQ_OR_NEQ, ("--source-port" | "--sport"), v: (Option<u16>, Option<u16>) as "format:[{}][:[{}]]" => {
        let min_port = v.0.unwrap_or(u16::MIN);
        let max_port = v.1.unwrap_or(u16::MAX);
        op!(attrs, load, base: TransportHeader, off: 0, len: 2);
        op!(attrs, range, op: range_op, from: min_port.to_be_bytes(), to: max_port.to_be_bytes());
    }
    &CMP_EQ_OR_NEQ, ("--destination-port" | "--dport"), port: u16 => {
        op!(attrs, load, base: TransportHeader, off: 2, len: 2);
        op!(attrs, cmp, op: cmp_op, val: port.to_be_bytes());
    }
    &RANGE_EQ_OR_NEQ, ("--destination-port" | "--dport"), v: (Option<u16>, Option<u16>) as "format:[{}][:[{}]]" => {
        let min_port = v.0.unwrap_or(u16::MIN);
        let max_port = v.1.unwrap_or(u16::MAX);
        op!(attrs, load, base: TransportHeader, off: 2, len: 2);
        op!(attrs, range, op: range_op, from: min_port.to_be_bytes(), to: max_port.to_be_bytes());
    }
}

group! {
    const EXT_MATCH_MARK;
    ~ repeat: star,
    &CMP_EQ_OR_NEQ, "--mark", v: (u32, Option<u32>) as "format:{}[/{}]" => {
        let mask = v.1.unwrap_or(u32::MAX);
        let mark = v.0;
        if mask > 0 {
            op!(attrs, load_meta, key: MetaKeys::Mark);
            if mask != u32::MAX {
                op!(attrs, mask_xor, mask: mask.to_ne_bytes(), xor: 0u32.to_be_bytes(), len: 4);
            }
            op!(attrs, cmp, op: cmp_op, val: mark.to_ne_bytes());
        }
    }
}

group! {
    const EXT_TARGETS;
    "LOG", &EXT_TARGET_LOG => {}
    "DNAT", &EXT_TARGET_DNAT => {}
    // ...
}

group! {
    const LOG_LEVEL;
    ~ init => { let log_level; }
    "emerg"   => { log_level = LogLevel::Emerg; }
    "alert"   => { log_level = LogLevel::Alert; }
    "crit"    => { log_level = LogLevel::Crit; }
    "error"   => { log_level = LogLevel::Err; }
    "warning" => { log_level = LogLevel::Warning; }
    "notice"  => { log_level = LogLevel::Notice; }
    "info"    => { log_level = LogLevel::Info; }
    "debug"   => { log_level = LogLevel::Debug; }
    "audit"   => { log_level = LogLevel::Audit; }
}

group! {
    const EXT_TARGET_LOG;
    ~ repeat: star,
    ~ init => {
        let mut log = nftables::XtLogInfo::new();
        log.level = LogLevel::Warning as u8;
    }
    ~ fini => {
        op!(attrs, counter);
        attrs = attrs
            .nested_elem()
            .nested_data_target()
            .push_name_bytes(b"LOG")
            .push_rev(0)
            .push_info(log.as_slice())
            .end_nested()
            .end_nested();
    }
    "--log-level", &LOG_LEVEL => {
        log.level = log_level as u8;
    }
    "--log-prefix", prefix: &str => {
        let len = prefix.len().min(29);
        log.prefix[..len].clone_from_slice(&prefix.as_bytes()[..len]);
    }
    "--log-tcp-sequence" => { log.logflags |= XtLogFlag::Tcpseq as u8; }
    "--log-tcp-options"  => { log.logflags |= XtLogFlag::Tcpopt as u8; }
    "--log-ip-options"   => { log.logflags |= XtLogFlag::Ipopt as u8; }
    "--log-uid"          => { log.logflags |= XtLogFlag::Uid as u8; }
    "--log-macdecode"    => { log.logflags |= XtLogFlag::Macdecode as u8; }
}

group! {
    const EXT_TARGET_DNAT;
    ~ repeat: plus,
    ~ init => {
        let mut dnat = nftables::NatRange2::new();
    }
    ~ fini => {
        op!(attrs, counter);
        attrs = attrs
            .nested_elem()
            .nested_data_target()
            .push_name_bytes(b"DNAT")
            .push_rev(2)
            .push_info(dnat.as_slice())
            .end_nested()
            .end_nested();
    }
    "--to-destination",
        v: (Option<IpAddr>, Option<IpAddr>, Option<u16>, Option<u16>, Option<u16>)
        as "format:[{}[-{}]][:{}[-{}]][/{}]"
    => {
        if let Some(addr) = v.0 {
            dnat.flags |= nftables::NatRangeFlags::MapIps as u32;
            clone_addr(&mut dnat.min_addr, addr);
            clone_addr(&mut dnat.max_addr, v.1.unwrap_or(addr));
        }
        if let Some(port) = v.2 {
            dnat.flags |= nftables::NatRangeFlags::ProtoSpecified as u32;
            dnat.set_min_port(port);
            dnat.set_max_port(v.3.unwrap_or(port));
        }
        if let Some(base_port) = v.4 {
            dnat.flags |= nftables::NatRangeFlags::ProtoOffset as u32;
            dnat.set_base_port(base_port);
        }
    }
    "--random" => {
        dnat.flags |= nftables::NatRangeFlags::ProtoRandom as u32;
    }
    "--persistent" => {
        dnat.flags |= nftables::NatRangeFlags::Persistent as u32;
    }
}
