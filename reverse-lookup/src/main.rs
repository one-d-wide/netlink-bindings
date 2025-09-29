use argh::FromArgs;
use std::{
    collections::HashMap,
    io::{self, BufRead, BufReader, Read},
    path::PathBuf,
};

use netlink_bindings::{
    builtin::{self, PushBuiltinNfgenmsg, PushNlmsghdr},
    consts, utils, Protocol,
};
use netlink_socket::NetlinkSocket;

mod generated;

use generated::ReverseLookup;

#[derive(FromArgs, Debug, Clone)]
/// Dump Netlink communications from strace file
#[argh(help_triggers("-h", "--help"))]
struct CliArgs {
    /// print message body in hex
    #[argh(switch)]
    #[argh(usage)]
    dump: bool,

    #[argh(positional, arg_name = "path_to_strace")]
    #[argh(usage)]
    strace: Vec<PathBuf>,
}

fn main() {
    let args: CliArgs = argh::from_env();

    if args.strace.is_empty() {
        println!("No input strace specified");
        println!("Run: strace -o ./output_file --decode-fd=socket -e %network --{{write,read}}=$(seq -s, 0 100) -- <command...>");
        let buf = std::fs::read_link("/proc/self/exe").unwrap_or_else(|_| "<this_command>".into());
        let cwd = std::env::current_dir().unwrap_or_else(|_| "".into());
        let self_loc = buf.strip_prefix(cwd).unwrap_or(&buf);
        println!("Then: {} -- ./output_file", self_loc.to_string_lossy());
        std::process::exit(1);
    }

    for path in &args.strace {
        read(&args, std::fs::File::open(path).unwrap());
    }
}

fn read(args: &CliArgs, reader: impl Read) {
    let read_syscalls = ["read", "recv", "recvfrom", "recvmsg", "recvmmsg"];
    let write_syscalls = ["write", "send", "sendto", "sendmsg", "sendmmsg"];

    let genl = genl_families();

    let mut last_proto = None;
    let mut is_dump = false;
    let mut last_request_value = None;
    let mut last_request_genl_family: Option<&'static str> = None;

    let mut lines = BufReader::new(reader)
        .lines()
        .map_while(Result::ok)
        .peekable();
    while let Some(ref line) = lines.next() {
        if let Some((syscall, args, _)) = split_syscall(line) {
            let is_read = read_syscalls.contains(&syscall);
            let is_write = write_syscalls.contains(&syscall);
            if is_read || is_write {
                let fd = args.split(", ").next().unwrap();
                last_proto = parse_fd_netlink_proto(fd).map(|(fd, family)| (fd, family, is_write));
                if last_proto.is_some() && is_write {
                    is_dump = false;
                    last_request_value = None;
                    last_request_genl_family = None;
                }
            }
        } else if let Some(dump) = line.strip_prefix(" | ") {
            let mut buf = Vec::new();
            parse_dump_line(dump, &mut buf);

            while let Some(line) = lines.peek() {
                if let Some(dump) = line.strip_prefix(" | ") {
                    parse_dump_line(dump, &mut buf);
                    lines.next();
                    continue;
                }
                break;
            }

            let Some((protonum, family, is_request)) = last_proto.as_ref() else {
                continue;
            };

            let mut remaining = &buf[..];
            while !remaining.is_empty() {
                let header =
                    PushNlmsghdr::new_from_slice(&remaining[..PushNlmsghdr::len()]).unwrap();
                let buf = &remaining[PushNlmsghdr::len()..header.get_len() as usize];
                remaining = &remaining[header.get_len() as usize..];

                println!();
                if *is_request {
                    print!("Decoding request");
                } else {
                    print!("Decoding reply");
                }
                print!(" in ");
                let request_type = header.get_type();
                let (proto, value) = match *protonum as i32 {
                    libc::NETLINK_GENERIC => {
                        let buf = PushBuiltinNfgenmsg::new_from_slice(&buf[..4]).unwrap();

                        if matches!(
                            request_type as i32,
                            libc::NLMSG_NOOP
                                | libc::NLMSG_DONE
                                | libc::NLMSG_ERROR
                                | libc::NLMSG_OVERRUN
                        ) {
                            let family = last_request_genl_family.unwrap();
                            print!("genl family {family}");
                            (Protocol::Generic(family.as_bytes()), buf.cmd() as u16)
                        } else {
                            let Some(family) = genl.get(&request_type) else {
                                panic!("Unknown genl family type {request_type}");
                            };
                            print!("genl family {family}");
                            last_request_genl_family = Some(family);

                            (Protocol::Generic(family.as_bytes()), buf.cmd() as u16)
                        }
                    }
                    _ => {
                        print!("family {family}");
                        (
                            Protocol::Raw {
                                protonum: *protonum,
                                request_type: 0, // Not used in the lookup
                            },
                            request_type,
                        )
                    }
                };
                if *is_request {
                    last_request_value = Some(value);
                }
                print!(" ");
                print_request_flags(header.flags());
                print!(" {:?}", proto);
                println!();

                if args.dump {
                    utils::dump_hex(buf);
                }

                match header.get_type() as i32 {
                    libc::NLMSG_NOOP => {
                        println!("NLMSG_NOOP");
                        continue;
                    }
                    libc::NLMSG_DONE | libc::NLMSG_ERROR => {
                        if header.get_type() == libc::NLMSG_DONE as u16 {
                            println!("NLMSG_DONE");
                        } else {
                            println!("NLMSG_ERROR");
                        }

                        let Some(code) = buf.get(0..4) else {
                            continue;
                        };
                        let code = utils::parse_i32(code).unwrap();
                        println!("Error code: {}", io::Error::from_raw_os_error(-code));
                        if code == 0 {
                            continue;
                        }

                        let echo_end = if header.get_type() == libc::NLMSG_DONE as u16 {
                            4
                        } else {
                            let Some(echo_header) = buf.get(4..(4 + PushNlmsghdr::len())) else {
                                continue;
                            };
                            let echo_header = PushNlmsghdr::new_from_slice(echo_header).unwrap();

                            if echo_header.flags() & libc::NLM_F_CAPPED as u16 == 0 {
                                let start = echo_header.get_len();
                                if buf.len() < start as usize + 4 {
                                    continue;
                                }

                                4 + start as usize
                            } else {
                                4 + PushNlmsghdr::len()
                            }
                        };

                        let ext_ack = builtin::NlmsgerrAttrs::new(&buf[echo_end..]);
                        print!("Extended ACK: ",);
                        if ext_ack.get_buf().is_empty() {
                            println!("(empty)");
                        } else {
                            println!("{ext_ack:?}");
                        }

                        continue;
                    }
                    libc::NLMSG_OVERRUN => {
                        println!("NLMSG_OVERRUN");
                        continue;
                    }
                    _ => {}
                };

                is_dump |= header.flags() & consts::NLM_F_DUMP as u16 == consts::NLM_F_DUMP as u16;
                let lookup = ReverseLookup {
                    proto,
                    value,
                    request_value: if *is_request {
                        None
                    } else {
                        last_request_value
                    },
                    is_dump,
                    buf,
                };

                println!("{:#?}", lookup);
            }
        }
    }
}

fn parse_fd_netlink_proto(line: &str) -> Option<(u16, String)> {
    // 3<NETLINK:[GENERIC:...]>

    let (_, rem) = line.split_once("<")?;
    let (proto, rem) = rem.split_once(":[")?;
    let (family, _) = rem.split_once(":")?;

    family_to_id(&format!("{proto}_{family}")).map(|fd| (fd, family.to_string()))
}

fn split_syscall(line: &str) -> Option<(&str, &str, &str)> {
    if line
        .chars()
        .next()
        .map(|c| !c.is_ascii_alphabetic())
        .unwrap_or(true)
    {
        return None;
    }

    let (syscall, rem) = line.split_once("(").unwrap();
    let (args, res) = rem.rsplit_once(") = ").unwrap();

    Some((syscall, args, res))
}

fn parse_dump_line(line: &str, buf: &mut Vec<u8>) {
    let mut iter = line.split("  ");
    iter.next();
    let bytes1 = iter.next().unwrap();
    let bytes2 = iter.next().unwrap();

    for byte in bytes1.split(" ").chain(bytes2.split(" ")) {
        if byte.is_empty() {
            continue;
        }
        buf.push(u8::from_str_radix(byte, 16).unwrap());
    }
}

fn genl_families() -> HashMap<u16, &'static str> {
    let mut acc = HashMap::new();
    let request = netlink_bindings::nlctrl::Request::new().op_getfamily_dump_request();
    let mut sock = NetlinkSocket::new();
    let mut iter = sock.request(&request).unwrap();
    while let Some(res) = iter.recv() {
        let attrs = res.unwrap();
        let id = attrs.get_family_id().unwrap();
        let name = attrs.get_family_name().unwrap().to_str().unwrap();
        acc.insert(id, name.to_string().leak() as &str);
    }
    acc
}

macro_rules! print_flags {
    (fn $func:ident, $($flag:ident => $str:expr,)*) => {
        fn $func(flags: u16) {
            print!("flags=");
            let values = [
                $((consts::$flag as u16, $str),)*
            ];
            print!("[");
            let mut first = true;
            for (bits, str) in values {
                if flags & bits != bits {
                    continue;
                }
                if first {
                    first = false;
                } else {
                    print!(",");
                }
                print!("{str}");
            }
            print!("]");
        }
    }
}

print_flags!(
    fn print_request_flags,
    NLM_F_REQUEST => "REQUEST",
    NLM_F_MULTI => "MULTI",
    NLM_F_ACK => "ACK",
    NLM_F_ECHO => "ECHO",
    NLM_F_DUMP_INTR => "DUMP_INTR",
    NLM_F_DUMP_FILTERED => "DUMP_FILTERED",

    NLM_F_ATOMIC => "ATOMIC",
    NLM_F_DUMP => "DUMP",

    NLM_F_REPLACE => "REPLACE",
    NLM_F_EXCL => "EXCL",
    NLM_F_CREATE => "CREATE",
    NLM_F_APPEND => "APPEND",
);

macro_rules! family_to_from_id {
    ($($family:ident,)*) => {
        #[allow(unused)]
        fn family_from_id(family: u16) -> Option<&'static str> {
            let res = match family as i32 {
                $(consts::$family => stringify!($family),)*
                _ => return None,
            };
            Some(res)
        }
        fn family_to_id(family: &str) -> Option<u16> {
            let res = match family {
                $(stringify!($family) => consts::$family as u16,)*
                _ => return None,
            };
            Some(res)
        }
    };
}

family_to_from_id!(
    NETLINK_ROUTE,
    NETLINK_UNUSED,
    NETLINK_USERSOCK,
    NETLINK_FIREWALL,
    NETLINK_SOCK_DIAG,
    NETLINK_NFLOG,
    NETLINK_XFRM,
    NETLINK_SELINUX,
    NETLINK_ISCSI,
    NETLINK_AUDIT,
    NETLINK_FIB_LOOKUP,
    NETLINK_CONNECTOR,
    NETLINK_NETFILTER,
    NETLINK_IP6_FW,
    NETLINK_DNRTMSG,
    NETLINK_KOBJECT_UEVENT,
    NETLINK_GENERIC,
    NETLINK_SCSITRANSPORT,
    NETLINK_ECRYPTFS,
    NETLINK_RDMA,
    NETLINK_CRYPTO,
);
