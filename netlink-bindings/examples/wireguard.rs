// This example queries the wireguard network interface named `wg0` and reports
// its info. Similar to what `wg` command does (without arguments). Make sure
// device named `wg0` exist, and the process have CAP_NET_ADMIN capability.
//
// Run with: `cargo run --example wireguard --features=wireguard`

use netlink_bindings::{utils, wireguard};

const HEADER_LEN: usize = 16;

fn main() {
    let fd = unsafe { libc::socket(libc::AF_NETLINK, libc::SOCK_RAW, libc::NETLINK_GENERIC) };
    assert!(fd >= 0);
    let sock: std::net::UdpSocket = unsafe { std::os::fd::FromRawFd::from_raw_fd(fd) };

    let mut buf = Vec::new();
    push_netlink_message_header(&mut buf);

    // Dump (request)
    wireguard::PushOpGetDeviceDumpRequest::new(&mut buf).push_ifname(c"wg0");

    println!("Sending:");
    println!(
        "{:#?}",
        wireguard::OpGetDeviceDumpRequest::new(&buf[HEADER_LEN..])
    );

    update_netlink_message_lenth(&mut buf);
    sock.send(&buf).unwrap();

    let mut buf = Box::new([0u8; 65536]);
    loop {
        let read = sock.recv(&mut buf[..]).unwrap();

        let mut buf = &buf[..read];
        while !buf.is_empty() {
            match utils::parse_u16(&buf[4..6]).unwrap() as i32 {
                libc::NLMSG_DONE => {
                    println!("NLMSG_DONE");
                    return;
                }
                libc::NLMSG_ERROR => {
                    println!("NLMSG_ERROR");
                    println!("error code: {}", utils::parse_u32(&buf[16..20]).unwrap());
                    println!("Does a wireguard device named `wg0` exist?");
                    println!("Did you obtain CAP_NET_ADMIN?");
                    std::process::exit(1);
                }
                _ => {}
            }

            let len = utils::parse_u32(&buf[0..4]).unwrap() as usize;
            let frame = &buf[HEADER_LEN..len];

            let attrs = wireguard::OpGetDeviceDumpReply::new(frame);

            println!("Received:");
            println!("{:#?}", attrs);

            println!("Ifname: {:?}", attrs.get_ifname().unwrap()); // &CStr
            for peer in attrs.get_peers() {
                println!("Endpoint: {}", peer.get_endpoint().unwrap()); // SockAddr

                for addr in peer.get_allowedips() {
                    let ip = addr.get_ipaddr().unwrap(); // IpAddr
                    let mask = addr.get_cidr_mask().unwrap(); // u8
                    println!("Allowed ip: {ip}/{mask}");
                }
            }

            buf = &buf[len..];
        }
    }
}

fn push_netlink_message_header(buf: &mut Vec<u8>) {
    // struct nlmsghdr {
    //     __u32 nlmsg_len;    /* Size of message including header */
    //     __u16 nlmsg_type;   /* Type of message content */
    //     __u16 nlmsg_flags;  /* Additional flags */
    //     __u32 nlmsg_seq;    /* Sequence number */
    //     __u32 nlmsg_pid;    /* Sender port ID */
    // };

    let len: u32 = 0;
    let r#type: u16 = 0x26 as u16; // wireguard protocol number
    let mut flags: u16 = libc::NLM_F_REQUEST as u16 | libc::NLM_F_ACK as u16;
    flags |= libc::NLM_F_DUMP as u16; // Comment this line to send "do" request
    let seq: u32 = 42;
    let pid: u32 = 0;

    buf.extend(len.to_le_bytes());
    buf.extend(r#type.to_le_bytes());
    buf.extend(flags.to_le_bytes());
    buf.extend(seq.to_le_bytes());
    buf.extend(pid.to_le_bytes());

    assert_eq!(buf.len(), HEADER_LEN);
}

fn update_netlink_message_lenth(buf: &mut Vec<u8>) {
    let len = buf.len() as u32;
    buf[0..4].copy_from_slice(&len.to_le_bytes());
}
