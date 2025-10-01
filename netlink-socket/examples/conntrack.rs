//! This example queries conntrack API and prints out all tracked network
//! connections, essentially reimplementing `conntrack -L` from conntrack-tools.
//!
//! Run with: `cargo run --example conntrack --features=conntrack`

use std::net::IpAddr;

use netlink_bindings::{conntrack, utils};

#[cfg_attr(not(feature = "async"), maybe_async::maybe_async)]
#[cfg_attr(feature = "tokio", tokio::main(flavor = "current_thread"))]
#[cfg_attr(feature = "smol", macro_rules_attribute::apply(smol_macros::main))]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let request = conntrack::Request::new().op_get_dump_request(&conntrack::PushNfgenmsg::new());

    let mut sock = netlink_socket::NetlinkSocket::new();

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

fn format_proto(attrs: utils::Iterable<'_, conntrack::TupleProtoAttrs>) {
    let proto = match attrs.get_proto_num().unwrap() as i32 {
        libc::IPPROTO_TCP => "tcp",
        libc::IPPROTO_UDP => "udp",
        libc::IPPROTO_ICMP => "icmp",
        _ => "other",
    };

    print!("{proto}");
}

fn format_status(attrs: utils::Iterable<'_, conntrack::OpGetDumpReply>) {
    use conntrack::NfCtStatus as S;
    let mut flags = Vec::new();
    let status = attrs.get_status().unwrap();
    for i in 0..32 {
        let flag = match status & (1 << i) {
            f if f == S::Expected as u32 => "Expected",
            f if f == S::SeenReply as u32 => "SeenReply",
            f if f == S::Assured as u32 => "Assured",
            f if f == S::Confirmed as u32 => "Confirmed",
            f if f == S::SrcNat as u32 => "SrcNat",
            f if f == S::DstNat as u32 => "DstNat",
            f if f == S::SeqAdj as u32 => "SeqAdj",
            f if f == S::SrcNatDone as u32 => "SrcNatDone",
            f if f == S::DstNatDone as u32 => "DstNatDone",
            f if f == S::Dying as u32 => "Dying",
            f if f == S::FixedTimeout as u32 => "FixedTimeout",
            f if f == S::Template as u32 => "Template",
            f if f == S::NatClash as u32 => "NatClash",
            f if f == S::Helper as u32 => "Helper",
            f if f == S::Offload as u32 => "Offload",
            f if f == S::HwOffload as u32 => "HwOffload",
            _ => "",
        };
        if !flag.is_empty() {
            flags.push(flag);
        }
    }

    print!(" [{}]", flags.join(","));
}

fn format_port(attrs: utils::Iterable<'_, conntrack::TupleProtoAttrs>) {
    if let Ok(src) = attrs.get_proto_src_port() {
        print!(" sport={src}");
    }
    if let Ok(dst) = attrs.get_proto_dst_port() {
        print!(" dport={dst}");
    }
}

fn format_ip(attrs: utils::Iterable<'_, conntrack::TupleIpAttrs>) {
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
