//! This example demonstrates basic usage of inet-diag interface to dump socket
//! info. This example is mostly equivalent to the following:
//!
//! ```sh
//! nc -l 127.0.0.1 12345 &
//! nc 127.0.0.1 12345 &
//! sleep 1
//! ss -i src 127.0.0.1 sport 12345
//! ```
//!
//! Run with: `cargo run --example tcp-rtt --features=inet-diag`

use std::{
    error::Error,
    net::{IpAddr, SocketAddr},
    time::Duration,
};

use netlink_bindings::{
    inet_diag::{self, BytecodeOp, BytecodeOpCode, Hostcond, Sockid, TcpState},
    utils::{self, FormatEnum},
};
use netlink_socket2::NetlinkSocket;

#[cfg_attr(not(feature = "async"), maybe_async::must_be_sync)]
#[cfg_attr(feature = "tokio", tokio::main(flavor = "current_thread"))]
#[cfg_attr(feature = "smol", macro_rules_attribute::apply(smol_macros::main))]
async fn main() -> Result<(), Box<dyn Error>> {
    let sock = std::net::TcpListener::bind("127.0.0.1:0")?;
    let addr = sock.local_addr()?;

    std::thread::spawn(move || {
        let _conn = std::net::TcpStream::connect(addr).unwrap();
        std::thread::sleep(Duration::MAX);
    });

    let (conn, peer_addr) = sock.accept()?;
    let sockaddr = conn.local_addr()?;

    let header = inet_diag::ReqV2 {
        family: libc::AF_INET as u8,
        protocol: libc::IPPROTO_TCP as u8,
        states: 1 << TcpState::Established as u32,
        // states: u32::MAX, // to just dump everything, including the listening socket
        ext: u8::MAX,
        // Unfortunately, sockid in here doesn't filter IP address (only src/dst port)
        sockid: inet_diag::Sockid {
            _sport_be: sockaddr.port().to_be(), // You may use .set_sport(<port>) instead
            _dport_be: peer_addr.port().to_be(),
            ..Default::default()
        },
        ..Default::default()
    };

    // To actually filter out by ip address, we have to compile a special bytecode
    let mut bytecode = Vec::new();
    {
        let mut saddr_cond = BytecodeOp {
            code: BytecodeOpCode::SaddrCond as u8,
            yes: 0, // Where to jump after this instruction in bytes on match
            no: 0,  // And on non-match
        };

        let hc = Hostcond {
            family: libc::AF_INET as u8,
            prefix_len: u32::BITS as u8, // Number of leading bits to compare in ip address
            port: sockaddr.port() as i32, // Port number (or -1 to match all, it's i32)
            ..Default::default()
        };

        let start = bytecode.len();
        bytecode.extend(saddr_cond.as_slice());
        bytecode.extend(hc.as_slice());
        utils::encode_ip(&mut bytecode, sockaddr.ip());

        // Now, substitute actual "yes" and "no" offsets in the first instruction
        let len = bytecode.len();
        saddr_cond.yes = len as u8; // Stepping on len+0 instruction means successful match
        saddr_cond.no = (len + 4) as u16; // Idk why it only accepts len+4
        bytecode[start..(start + BytecodeOp::len())].clone_from_slice(saddr_cond.as_slice());
    }

    // Same subsystem supports dumping info about TCP, UDP, and other inet protocols.
    // UNIX sockets are also supported, though with a slightly different header format.
    let mut request = inet_diag::Request::new() //
        .op_tcp_diag_dump(&header);

    request.encode().push_bytecode(&bytecode);

    let mut sock = NetlinkSocket::new();

    let mut iter = sock.request(&request).await?;
    while let Some((header, attrs)) = iter.recv().await.transpose()? {
        let (src, dst) = decode_sockid(header.family, &header.sockid);
        print!(
            "{src} -> {dst} state={:?}",
            FormatEnum(header.state as u64, TcpState::from_value)
        );

        // Listening socket doesn't have a tcp-info attribute
        if let Ok(tcp) = attrs.get_tcp_info() {
            print!(" rtt={}ms, rttvar={}ms", to_ms(tcp.rtt), to_ms(tcp.rttvar));
        }

        println!();
    }

    // std::thread::sleep(Duration::MAX);

    Ok(())
}

fn to_ms(micros: u32) -> f32 {
    micros as f32 / 1000.0
}

fn decode_sockid(family: u8, sockid: &Sockid) -> (SocketAddr, SocketAddr) {
    (
        SocketAddr::new(decode_sockid_ip(family, &sockid.src), sockid.sport()),
        SocketAddr::new(decode_sockid_ip(family, &sockid.dst), sockid.dport()),
    )
}

fn decode_sockid_ip(family: u8, buf: &[u8]) -> IpAddr {
    if family == libc::AF_INET as u8 {
        utils::parse_ipv4(&buf[..4]).unwrap().into()
    } else {
        utils::parse_ipv6(&buf[..4]).unwrap().into()
    }
}
