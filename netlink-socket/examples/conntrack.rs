//! This example queries conntrack API and prints out all tracked network
//! connections, essentially reimplementing `conntrack -L` from conntrack-tools.
//!
//! Run with: `cargo run --example conntrack --features=conntrack`

use std::net::IpAddr;

use netlink_bindings::conntrack;

#[cfg_attr(not(feature = "async"), maybe_async::maybe_async)]
#[cfg_attr(feature = "tokio", tokio::main(flavor = "current_thread"))]
#[cfg_attr(feature = "smol", macro_rules_attribute::apply(smol_macros::main))]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let request = conntrack::Request::new().op_get_dump_request(&conntrack::PushNfgenmsg::new());

    let mut sock = netlink_socket2::NetlinkSocket::new();

    let mut iter = sock.request(&request).await?;
    while let Some(res) = iter.recv().await {
        let (_header, attrs) = res.unwrap();
        let orig = attrs.get_tuple_orig().unwrap();
        let reply = attrs.get_tuple_reply().unwrap();
        let proto = orig.get_tuple_proto().unwrap();

        format_proto(proto);
        print!(" timeout={}", attrs.get_timeout().unwrap());
        format_ip(orig.get_tuple_ip().unwrap());
        format_port(orig.get_tuple_proto().unwrap());
        format_ip(reply.get_tuple_ip().unwrap());
        format_port(reply.get_tuple_proto().unwrap());
        format_status(attrs);
        print!(" mark={}", attrs.get_mark().unwrap());
        print!(" use={}", attrs.get_use().unwrap());
        println!();
    }

    Ok(())
}

fn format_proto(attrs: conntrack::IterableTupleProtoAttrs<'_>) {
    let proto = match attrs.get_proto_num().unwrap() as i32 {
        libc::IPPROTO_TCP => "tcp",
        libc::IPPROTO_UDP => "udp",
        libc::IPPROTO_ICMP => "icmp",
        _ => "other",
    };

    print!("{proto}");
}

fn format_status(attrs: conntrack::IterableOpGetDumpReply<'_>) {
    let status = attrs.get_status().unwrap();
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
}

fn format_port(attrs: conntrack::IterableTupleProtoAttrs<'_>) {
    if let Ok(src) = attrs.get_proto_src_port() {
        print!(" sport={src}");
    }
    if let Ok(dst) = attrs.get_proto_dst_port() {
        print!(" dport={dst}");
    }
}

fn format_ip(attrs: conntrack::IterableTupleIpAttrs<'_>) {
    let src: IpAddr = attrs
        .get_ip_v4_src()
        .map(Into::into)
        .or(attrs.get_ip_v6_src().map(Into::into))
        .unwrap();
    let dst: IpAddr = attrs
        .get_ip_v4_dst()
        .map(Into::into)
        .or(attrs.get_ip_v6_dst().map(Into::into))
        .unwrap();
    print!(" src={src} dst={dst}");
}
