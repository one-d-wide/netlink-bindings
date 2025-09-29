// This example queries the wireguard network interface named `wg0` and reports
// its info. Similar to what `wg` command does (without arguments). Make sure
// device named `wg0` exist, and the process have CAP_NET_ADMIN capability.
//
// Run with: `cargo run --example wireguard --features=wireguard,nlctrl`

use netlink_bindings::{builtin::PushNlmsghdr, nlctrl, utils, wireguard};

fn main() {
    let fd = unsafe { libc::socket(libc::AF_NETLINK, libc::SOCK_RAW, libc::NETLINK_GENERIC) };
    assert!(fd >= 0);

    let sock: std::net::UdpSocket = unsafe { std::os::fd::FromRawFd::from_raw_fd(fd) };

    let mut buf = Vec::new();

    // Genetlink protocols have their family_id allocated at run-time.
    let family_id = resolve_family_id(&sock, "wireguard");

    push_netlink_message_header(&mut buf, family_id);

    // Dump (request)
    wireguard::PushOpGetDeviceDumpRequest::new(&mut buf).push_ifname(c"wg0");

    println!("Sending:");
    println!(
        "{:#?}",
        wireguard::OpGetDeviceDumpRequest::new(&buf[PushNlmsghdr::len()..])
    );

    update_netlink_message_lenth(&mut buf);
    sock.send(&buf).unwrap();

    let mut buf = Box::new([0u8; 8192]);
    loop {
        let read = sock.recv(&mut buf[..]).unwrap();

        let mut buf = &buf[..read];
        while !buf.is_empty() {
            let reply = PushNlmsghdr::new_from_slice(&buf[..PushNlmsghdr::len()]).unwrap();
            let frame = &buf[PushNlmsghdr::len()..reply.get_len() as usize];

            match reply.get_type() as i32 {
                libc::NLMSG_DONE => {
                    println!("NLMSG_DONE");
                    println!("DUMP operation succeeded");
                    return;
                }
                libc::NLMSG_ERROR => {
                    let raw_err = utils::parse_i32(&buf[16..20]).unwrap();
                    if raw_err == 0 {
                        println!("DO operation succeeded");
                        return;
                    }

                    println!("NLMSG_ERROR");
                    println!("Error: {}", std::io::Error::from_raw_os_error(-raw_err));
                    println!("Does a wireguard device named `wg0` exist?");
                    println!("Did you obtain CAP_NET_ADMIN?");
                    std::process::exit(1);
                }
                _ => {}
            }

            let attrs = wireguard::OpGetDeviceDumpReply::new(frame);

            println!("Received:");
            println!("{:#?}", attrs);

            println!("Ifname: {:?}", attrs.get_ifname().unwrap()); // &CStr
            for peer in attrs.get_peers().unwrap() {
                if let Ok(endpoint) = peer.get_endpoint() {
                    println!("Endpoint: {endpoint}"); // SockAddr
                } else {
                    println!("Endpoint: (not set)");
                }

                for addr in peer.get_allowedips().unwrap() {
                    let ip = addr.get_ipaddr().unwrap(); // IpAddr
                    let mask = addr.get_cidr_mask().unwrap(); // u8
                    println!("Allowed ip: {ip}/{mask}");
                }
            }

            buf = &buf[reply.get_len() as usize..];
        }
    }
}

fn push_netlink_message_header(buf: &mut Vec<u8>, family_id: u16) {
    let mut header = PushNlmsghdr::new();
    header.set_len(0);
    header.set_type(family_id);
    header.set_flags(libc::NLM_F_DUMP as u16 | libc::NLM_F_REQUEST as u16 | libc::NLM_F_ACK as u16);
    header.set_seq(42);

    buf.extend(header.as_slice());
}

fn update_netlink_message_lenth(buf: &mut Vec<u8>) {
    let len = buf.len() as u32;
    buf[0..4].copy_from_slice(&len.to_le_bytes());
}

fn resolve_family_id(sock: &std::net::UdpSocket, name: &str) -> u16 {
    let mut buf = Vec::new();

    let mut header = PushNlmsghdr::new();
    header.set_len(0);
    header.set_type(libc::GENL_ID_CTRL as u16);
    header.set_flags(libc::NLM_F_REQUEST as u16 | libc::NLM_F_ACK as u16);
    header.set_seq(1);

    buf.extend(header.as_slice());

    nlctrl::PushOpGetfamilyDoRequest::new(&mut buf).push_family_name_bytes(name.as_bytes());

    let len = buf.len() as u32;
    buf[0..4].copy_from_slice(&len.to_le_bytes());

    sock.send(&buf[..]).unwrap();

    let mut buf = Box::new([0u8; 8192]);
    let mut family_id = None;
    loop {
        let read = sock.recv(&mut buf[..]).unwrap();

        let mut buf = &buf[..read];
        while !buf.is_empty() {
            let reply = PushNlmsghdr::new_from_slice(&buf[..PushNlmsghdr::len()]).unwrap();

            match reply.get_type() as i32 {
                libc::NLMSG_ERROR => {
                    let raw_err = utils::parse_i32(&buf[16..20]).unwrap();
                    if raw_err == 0 {
                        return family_id.unwrap();
                    }

                    println!("NLMSG_ERROR");
                    println!("Error: {}", std::io::Error::from_raw_os_error(-raw_err));
                    println!("Can't resolve wireguard in genl(7)");
                    println!("Is wireguard listed in `genl ctrl list`?");
                    std::process::exit(1);
                }
                _ => {}
            }

            let attrs = nlctrl::OpGetfamilyDoReply::new(&buf[PushNlmsghdr::len()..]);
            family_id = attrs.get_family_id().ok();

            buf = &buf[reply.get_len() as usize..];
        }
    }
}
