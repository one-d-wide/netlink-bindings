//! This example queries conntrack API and prints out all tracked network
//! connections, essentially reimplementing `conntrack -L` from conntrack-tools.
//!
//! Run with: `cargo run --example conntrack --features=conntrack`

use std::{error::Error, net::IpAddr};

use netlink_bindings::conntrack;
use netlink_socket2::NetlinkSocket;

fn main() -> Result<(), Box<dyn Error>> {
    let request = conntrack::Request::new().op_get_dump(&conntrack::Nfgenmsg::new());

    let mut sock = NetlinkSocket::new();

    let mut iter = sock.request(&request)?;
    while let Some((_header, attrs)) = iter.recv().transpose()? {
        let orig = attrs.get_tuple_orig()?;
        let reply = attrs.get_tuple_reply()?;
        let proto = orig.get_tuple_proto()?;

        format_proto(proto)?;
        print!(" timeout={}", attrs.get_timeout()?);
        format_ip(orig.get_tuple_ip()?)?;
        format_port(orig.get_tuple_proto()?);
        format_ip(reply.get_tuple_ip()?)?;
        format_port(reply.get_tuple_proto()?);
        format_status(attrs)?;
        print!(" mark={}", attrs.get_mark()?);
        print!(" use={}", attrs.get_use()?);
        println!();
    }

    Ok(())
}

fn format_proto(attrs: conntrack::IterableTupleProtoAttrs<'_>) -> Result<(), Box<dyn Error>> {
    let proto = match attrs.get_proto_num()? as i32 {
        libc::IPPROTO_TCP => "tcp",
        libc::IPPROTO_UDP => "udp",
        libc::IPPROTO_ICMP => "icmp",
        _ => "other",
    };
    print!("{proto}");
    Ok(())
}

fn format_status(attrs: conntrack::IterableConntrackAttrs<'_>) -> Result<(), Box<dyn Error>> {
    let status = attrs.get_status()?;
    print!(" [");
    let mut is_first = true;
    for i in 0..32 {
        if let Some(flag) = conntrack::NfCtStatus::from_value((status & (1 << i)) as u64) {
            if !is_first {
                print!(",");
            }
            print!("{flag:?}");
            is_first = false;
        }
    }
    print!("]");
    Ok(())
}

fn format_port(attrs: conntrack::IterableTupleProtoAttrs<'_>) {
    if let Ok(src) = attrs.get_proto_src_port() {
        print!(" sport={src}");
    }
    if let Ok(dst) = attrs.get_proto_dst_port() {
        print!(" dport={dst}");
    }
}

fn format_ip(attrs: conntrack::IterableTupleIpAttrs<'_>) -> Result<(), Box<dyn Error>> {
    let src: IpAddr = attrs
        .get_ip_v4_src()
        .map(Into::into)
        .or(attrs.get_ip_v6_src().map(Into::into))?;
    let dst: IpAddr = attrs
        .get_ip_v4_dst()
        .map(Into::into)
        .or(attrs.get_ip_v6_dst().map(Into::into))?;
    print!(" src={src} dst={dst}");
    Ok(())
}
