use argh::FromArgs;
use std::{
    cell::Cell,
    collections::HashMap,
    ffi::OsStr,
    fs::File,
    io::{self, BufRead, BufReader, BufWriter, Write},
    os::unix::{ffi::OsStrExt, fs::MetadataExt, process::ExitStatusExt},
    path::{Path, PathBuf},
    process::Stdio,
    sync::{
        atomic::{self, AtomicU32},
        mpsc,
    },
    time::Duration,
};

use netlink_bindings::{
    builtin::{self, BuiltinNfgenmsg, Nlmsghdr},
    consts, nlctrl,
    traits::Protocol,
    utils,
};
use netlink_socket2::NetlinkSocket;

mod generated;

pub use generated::{get_operation_genl, get_operation_raw, ReverseLookup};

pub fn get_op(proto: Protocol, buf: &[u8]) -> Vec<&'static str> {
    match proto {
        Protocol::Raw {
            protonum,
            request_type,
        } => {
            let mut res = get_operation_raw(protonum, request_type);
            if res.is_empty() {
                if let Some(proto) = family_from_id(protonum) {
                    res.push(proto);
                }
            }
            res
        }
        Protocol::Generic(proto) => {
            let mut res = get_operation_genl(proto, BuiltinNfgenmsg::from_slice(buf).cmd);
            if res.is_empty() {
                if let Ok(proto) = str::from_utf8(proto) {
                    res.push(proto);
                }
            }
            res
        }
    }
}

#[derive(FromArgs, Debug, Default, Clone)]
#[argh(help_triggers("-h", "--help"))]
#[argh(description = "
Dump Netlink communications using strace live:

    {command_name} <command...>

Or from a file:

    strace -o ./output_file --decode-fd=socket -e execve,%network --{{write,read}}=$(seq -s, 0 100) -- <command...>
    {command_name} ./output_file

")]
pub struct CliArgs {
    /// echo strace output
    #[argh(switch)]
    #[argh(short = 'e')]
    #[argh(usage)]
    pub echo: bool,

    /// print message body in hex
    #[argh(switch)]
    #[argh(short = 'd')]
    #[argh(usage)]
    pub dump: bool,

    /// read strace file
    #[argh(switch)]
    #[argh(short = 'f')]
    pub file: bool,

    /// run command live
    #[argh(switch)]
    #[argh(short = 'c')]
    pub command: bool,

    /// attach to pid
    #[argh(option, arg_name = "pid")]
    #[argh(short = 'p')]
    pub pid: Option<u32>,

    /// save the strace
    #[argh(switch)]
    #[argh(short = 's')]
    pub save: bool,

    /// the save file directory
    #[argh(option, arg_name = "path")]
    pub save_dir: Option<PathBuf>,

    /// additional arguments when running strace
    #[argh(option, arg_name = "args")]
    pub strace_args: Vec<String>,

    #[argh(positional, greedy, arg_name = "command_or_file")]
    #[argh(usage)]
    pub args: Vec<String>,
}

#[derive(Debug, Default, Clone)]
pub struct ApiArgs {
    /// Avoid setting up signal handlers using `libc::signal()`
    pub redirect_signals: bool,
    /// Receive strace process pid (after strace attaches).
    pub strace_pid_tx: Option<std::sync::mpsc::Sender<u32>>,
    pub frames_tx: Option<mpsc::Sender<ReverseLookup>>,
    pub no_follow_fork: bool,
}

pub fn run(mut args: CliArgs, api: ApiArgs) -> bool {
    args.save |= args.save_dir.is_some();
    args.command |= args.pid.is_some();

    if args.command && args.file {
        eprintln!("Can't use --command and --file simultaneously");
        return false;
    }

    if args.pid.is_some() && !args.args.is_empty() {
        eprintln!("No inputs expected with --pid");
        return false;
    } else if args.args.is_empty() && args.pid.is_none() {
        eprintln!("No inputs specified. See --help");
        return false;
    }

    let exe = args.args.first().map_or("", |s| s.as_str());
    let hint_exe = args.args.len() > 1 || exe_exists(exe);

    if args.save && !args.command && (args.file || !hint_exe) {
        eprintln!("Can't use --save when input is already a file. Try --command");
        return false;
    }

    if !args.command && (args.file || !hint_exe) {
        for path in &args.args {
            let file = File::open(path).unwrap();
            let _ = read(&args, &mut BufReader::new(file), api.frames_tx.as_ref());
        }
        return true;
    }

    if args.pid.is_none() && !exe_exists(exe) {
        eprintln!("Assuming {exe:?} is a program. Try --file or prefix files with \"./\"");
        eprintln!("Can't find executable in PATH: {exe:?}");
        return false;
    }

    if !exe_exists("strace") {
        eprintln!("Can't find executable in PATH: {:?}", "strace");
        return false;
    }

    let mut fds = "0".to_string();
    for i in 1..1000 {
        use std::fmt::Write;
        write!(fds, ",{i}").unwrap();
    }

    let mut cmd = std::process::Command::new("strace");

    if !api.no_follow_fork {
        cmd.arg("-ff");
    }

    cmd.args("--decode-fd=socket -e execve,%network,readv,writev".split(' '))
        .args(format!("--read={fds} --write={fds}").split(' '))
        .args(&args.strace_args);

    if let Some(pid) = args.pid {
        cmd.arg("-p").arg(&format!("{pid}"));
    } else {
        cmd.arg("--");
        if !api.no_follow_fork {
            cmd.args(["sh", "-c", "exec 2>&1 -- \"$@\"", "--"]);
        }
        cmd.args(&args.args);
    }

    // if args.echo {
    //     eprint!("Running: {}", cmd.get_program().to_string_lossy());
    //     for arg in cmd.get_args() {
    //         eprint!(" {}", arg.to_string_lossy());
    //     }
    //     eprintln!();
    // }

    cmd.stderr(Stdio::piped());
    let res = cmd.spawn();

    let mut proc = match res {
        Ok(proc) => proc,
        Err(err) => {
            eprintln!("Can't spawn {exe:?}: {err}");
            return false;
        }
    };

    if api.redirect_signals {
        static STRACE_PID: AtomicU32 = AtomicU32::new(0);
        STRACE_PID.store(proc.id(), atomic::Ordering::Relaxed);

        extern "C" fn redirect_signal(signum: libc::c_int) {
            match STRACE_PID.load(atomic::Ordering::Relaxed) {
                0 if signum == libc::SIGINT || signum == libc::SIGTERM => std::process::exit(0),
                0 => return,
                pid => unsafe {
                    libc::kill(pid as libc::pid_t, signum);
                },
            }
        }

        for signum in 0..32 {
            unsafe {
                libc::signal(signum, redirect_signal as *const () as usize);
            }
        }
    }

    let mut buf = BufReader::new(proc.stderr.take().unwrap());

    while buf.fill_buf().is_ok_and(|b| b.is_empty()) {
        std::thread::sleep(Duration::from_millis(10));
    }

    if buf
        .fill_buf()
        .is_ok_and(|s| s.starts_with(b"strace: attach: "))
    {
        // Strace has probably encountered an error
        let _ = std::io::copy(&mut buf, &mut std::io::stderr().lock());
    } else {
        if let Some(tx) = api.strace_pid_tx {
            tx.send(proc.id()).unwrap();
        }

        let _ = read(&args, &mut buf, api.frames_tx.as_ref());
    }

    let mut code = 1;
    match proc.wait() {
        Ok(s) if s.signal().is_some_and(|s| s == libc::SIGINT) => code = 0,
        Ok(s) => code = s.code().unwrap_or(1),
        Err(err) => eprintln!("Can't wait for {exe:?}: {err}"),
    }

    code == 0
}

fn exe_exists(exe: impl AsRef<Path>) -> bool {
    let exe = exe.as_ref();

    if exe.as_os_str().as_bytes().contains(&b'/') {
        return std::fs::metadata(exe).is_ok_and(|m| m.is_file() && m.mode() & 0o111 != 0);
    }

    std::env::var_os("PATH")
        .unwrap_or_default()
        .as_bytes()
        .split(|c| *c == b':')
        .filter(|p| !p.is_empty())
        .filter(|p| !p.iter().all(|c| c.is_ascii_alphabetic()))
        .any(|p| PathBuf::from(OsStr::from_bytes(p)).join(exe).exists())
}

fn read(
    args: &CliArgs,
    reader: &mut dyn BufRead,
    sender: Option<&mpsc::Sender<ReverseLookup>>,
) -> io::Result<()> {
    let read_syscalls = ["read", "readv", "recv", "recvfrom", "recvmsg", "recvmmsg"];
    let write_syscalls = ["write", "writev", "send", "sendto", "sendmsg", "sendmmsg"];

    let mut save = None;
    if args.save {
        let dir = args.save_dir.clone().unwrap_or_else(|| PathBuf::from("./"));
        let mut cmd = args.args.join("_");

        if let Some(pid) = args.pid {
            if let Ok(mut buf) = std::fs::read(&format!("/proc/{pid}/cmdline")) {
                buf.pop();
                buf.truncate(128);
                for b in &mut buf {
                    match b {
                        b'\0' | b'/' => *b = b'_',
                        _ => {}
                    }
                }
                cmd = String::from_utf8_lossy(&buf).to_string();
            } else {
                cmd = format!("{pid}");
            }
        }

        for i in 0.. {
            let path = dir.join(format!("strace_{cmd}.{i}"));
            match File::options().write(true).create_new(true).open(&path) {
                Ok(f) => {
                    eprintln!("Saving {path:?}");
                    save = Some(BufWriter::new(f));
                    break;
                }
                Err(e) if e.kind() == io::ErrorKind::AlreadyExists => continue,
                Err(err) => {
                    eprintln!("Can't create file {path:?}: {err}");
                    return Err(err);
                }
            }
        }
    }

    let genl = genl_families();

    let mut last_proto = None;
    let mut is_netlink = false;
    let mut is_request = false;
    let mut is_dump = false;
    let mut last_request_value = None;
    let mut last_request_genl_family: Option<&'static str> = None;
    let mut last_filter = Default::default();

    let mut stream_buf = None;
    let out = if sender.is_some() {
        stream_buf = Some(EchoOnError::default());
        stream_buf.as_mut().unwrap() as &mut dyn Write
    } else {
        &mut std::io::stderr().lock() as &mut dyn Write
    };
    let mut out = BufWriter::with_capacity(1_000_000, out);

    // (pid, fd) -> family
    let mut hint_fd_to_family = HashMap::new();

    let mut lines = reader.lines().peekable();
    while let Some(Ok(ref line)) = lines.next() {
        if args.echo {
            writeln!(out, "{line}")?;
        }

        if let Some(save) = &mut save {
            save.write_all(line.as_bytes())?;
            save.write_all(b"\n")?;
        }

        if !out.buffer().is_empty() {
            out.flush()?;
        }

        if line.is_empty() {
            continue;
        }

        if let Some((pid, syscall, args, res)) = split_syscall(line) {
            let args: Vec<_> = args.split(", ").collect();

            // socket(AF_NETLINK, SOCK_RAW|SOCK_CLOEXEC, NETLINK_ROUTE) = 12<NETLINK:[123456]>
            if syscall == "socket" && args[0] == "AF_NETLINK" {
                let fd = parse_fd(res).unwrap_or_default();
                hint_fd_to_family.insert(
                    (pid.to_string(), fd.to_string()),
                    (
                        family_to_id(args[2]).unwrap_or_default(),
                        args[2].to_string(),
                    ),
                );
                continue;
            }

            let is_read = read_syscalls.contains(&syscall);
            let is_write = write_syscalls.contains(&syscall);
            if is_read || is_write {
                let res = args[0];
                let fd = parse_fd(res).unwrap_or_default();
                is_netlink = parse_fd_proto(res) == Some("NETLINK");
                last_proto = parse_fd_proto_family(res).or_else(|| {
                    hint_fd_to_family
                        .get(&(pid.to_string(), fd.to_string()))
                        .cloned()
                });
                is_request = is_write;
                if is_netlink && is_write {
                    is_dump = false;
                    last_request_value = None;
                    last_request_genl_family = None;
                }
            }
        } else if let Some(dump) = line.strip_prefix(" | ") {
            let mut buf = Vec::new();
            parse_dump_line(&mut buf, dump);

            while let Some(Ok(line)) = lines.peek() {
                match () {
                    _ if line.starts_with(" * ") => {}
                    _ if line.starts_with(" | ") => parse_dump_line(&mut buf, line),
                    _ => break,
                }
                if let Some(save) = &mut save {
                    save.write_all(line.as_bytes())?;
                    save.write_all(b"\n")?;
                }
                if args.echo {
                    writeln!(out, "{line}")?;
                }
                lines.next();
            }

            if !is_netlink {
                continue;
            }

            let mut remaining = &buf[..];
            while !remaining.is_empty() {
                let header = Nlmsghdr::new_from_zeroed(remaining);
                if Nlmsghdr::len() > header.len as usize || remaining.len() < header.len as usize {
                    let rem = remaining.len();
                    writeln!(out, "Skipping {rem} bytes. Can't make sense of the rest")?;
                    break;
                }
                let buf = &remaining[Nlmsghdr::len()..header.len as usize];
                remaining = &remaining[header.len as usize..];

                let Some((protonum, family)) = last_proto.as_ref() else {
                    continue;
                };

                writeln!(out)?;
                if is_request {
                    write!(out, "Decoding request")?;
                } else {
                    write!(out, "Decoding reply")?;
                }
                write!(out, " in ")?;

                let request_type = header.r#type;
                let proto;
                let value;
                let value_str;

                match *protonum as i32 {
                    libc::NETLINK_GENERIC => {
                        let buf = BuiltinNfgenmsg::new_from_slice(&buf[..4]).unwrap();

                        let family = if matches!(
                            request_type as i32,
                            libc::NLMSG_NOOP
                                | libc::NLMSG_DONE
                                | libc::NLMSG_ERROR
                                | libc::NLMSG_OVERRUN
                        ) {
                            last_request_genl_family.unwrap()
                        } else {
                            let Some(family) = genl.get(&request_type) else {
                                panic!("Unknown genl family type {request_type}");
                            };
                            last_request_genl_family = Some(family);
                            family
                        };

                        write!(out, "genl family {family}")?;

                        proto = Protocol::Generic(family.as_bytes());
                        value = buf.cmd as u16;
                        value_str = generated::get_operation_genl(family.as_bytes(), buf.cmd);
                    }
                    _ => {
                        write!(out, "family {family}")?;
                        proto = Protocol::Raw {
                            protonum: *protonum,
                            request_type, // Not used in the lookup
                        };
                        value = request_type;
                        value_str = generated::get_operation_raw(*protonum, request_type);
                    }
                }

                if is_request {
                    last_request_value = Some(value);
                }

                write!(out, " ")?;
                print_request_flags(&mut out, header.flags)?;
                write!(out, " ")?;
                if let Some(op) = value_str.first() {
                    let op = op
                        .strip_suffix("Do")
                        .or(op.strip_suffix("Dump"))
                        .unwrap_or(op);
                    write!(out, "{op} [value={value}]")?;
                } else {
                    match proto {
                        Protocol::Generic(family) => {
                            write!(out, "Generic({:?})", String::from_utf8_lossy(family))?
                        }
                        _ => write!(out, "{proto:?}")?,
                    }
                }
                writeln!(out)?;

                if args.dump {
                    out.flush()?;
                    utils::dump_hex(buf);
                }

                match header.r#type as i32 {
                    libc::NLMSG_NOOP => {
                        writeln!(out, "NLMSG_NOOP")?;
                        continue;
                    }
                    libc::NLMSG_DONE | libc::NLMSG_ERROR => {
                        if header.r#type == libc::NLMSG_DONE as u16 {
                            writeln!(out, "NLMSG_DONE")?;
                        } else {
                            writeln!(out, "NLMSG_ERROR")?;
                        }

                        let Some(code) = buf.get(0..4) else {
                            continue;
                        };
                        let code = utils::parse_i32(code).unwrap();
                        writeln!(out, "Error code: {}", io::Error::from_raw_os_error(-code))?;
                        if code == 0 {
                            continue;
                        }

                        let mut echo_end = 4;
                        if code != 0 && header.r#type == libc::NLMSG_ERROR as u16 {
                            let Some(echo_header) = buf.get(4..(4 + Nlmsghdr::len())) else {
                                continue;
                            };

                            if header.flags & libc::NLM_F_CAPPED as u16 == 0 {
                                let start = Nlmsghdr::from_slice(echo_header).len;
                                if buf.len() < start as usize + 4 {
                                    continue;
                                }

                                echo_end += start as usize;
                            } else {
                                echo_end += Nlmsghdr::len();
                            }
                        };

                        let mut ext_ack_start = buf.len();
                        if header.flags & libc::NLM_F_ACK_TLVS as u16 != 0 {
                            ext_ack_start =
                                utils::align_up(echo_end, utils::NLA_ALIGNTO).min(buf.len());
                        }

                        let ext_ack = builtin::NlmsgerrAttrs::new(&buf[ext_ack_start..]);
                        write!(out, "Extended ACK: ",)?;
                        if ext_ack.get_buf().is_empty() {
                            writeln!(out, "(empty)")?;
                        } else {
                            writeln!(out, "{ext_ack:?}")?;
                        }

                        continue;
                    }
                    libc::NLMSG_OVERRUN => {
                        writeln!(out, "NLMSG_OVERRUN")?;
                        continue;
                    }
                    _ => {}
                };

                is_dump |= header.flags & consts::NLM_F_DUMP as u16 == consts::NLM_F_DUMP as u16;
                let lookup = ReverseLookup {
                    header,
                    proto,
                    value,
                    request_value: if is_request { None } else { last_request_value },
                    is_dump,
                    last_filter: Cell::new(last_filter),
                    buf: buf.into(),
                };

                if let Some(tx) = sender.as_ref() {
                    tx.send(lookup.clone()).unwrap(); // Clone before it's modified
                }

                writeln!(out, "{:#?}", lookup)?;
                last_filter = lookup.last_filter.get();
            }

            out.flush()?;
        }
    }

    drop(out);
    if let Some(x) = &mut stream_buf {
        x.is_ok = !args.echo && !args.dump;
    }

    Ok(())
}

fn parse_fd(line: &str) -> Option<&str> {
    // 3<NETLINK:[GENERIC:...]>
    let (fd, _) = line.split_once("<")?;
    Some(fd)
}

fn parse_fd_proto(line: &str) -> Option<&str> {
    // 3<NETLINK:[GENERIC:...]>
    let (_, rem) = line.split_once("<")?;
    let (proto, _) = rem.split_once(":[")?;
    Some(proto)
}

fn parse_fd_proto_family(line: &str) -> Option<(u16, String)> {
    // 3<NETLINK:[GENERIC:...]>
    let (_, rem) = line.split_once("<")?;
    let (proto, rem) = rem.split_once(":[")?;
    let (family, _) = rem.split_once(":")?;
    family_to_id(&format!("{proto}_{family}")).map(|fd| (fd, family.to_string()))
}

fn split_syscall(mut line: &str) -> Option<(&str, &str, &str, &str)> {
    // [pid 1234] foo(1, 2, 3) = 4

    if line.is_empty() || (!line.as_bytes()[0].is_ascii_alphanumeric() && !line.starts_with("[")) {
        return None;
    }

    let mut pid = "";
    if let Some(rem) = line.strip_prefix("[") {
        (pid, line) = rem.split_once("] ").unwrap_or(("", line));
    }

    let (syscall, rem) = line.split_once("(")?;

    let (args, res) = rem.rsplit_once(") = ").unwrap_or((rem, ""));

    Some((pid, syscall, args, res))
}

fn parse_dump_line(buf: &mut Vec<u8>, line: &str) {
    //  | 00000  00 01 02 03 04 05 06 07  08 09 0a 0b 0c 0d 0e 0f  ................ |

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
    let request = nlctrl::Request::new().op_getfamily_dump();
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

#[derive(Default)]
struct EchoOnError {
    buf: Vec<u8>,
    is_ok: bool,
}
impl Write for EchoOnError {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.buf.write_all(buf).unwrap();
        Ok(buf.len())
    }
    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}
impl Drop for EchoOnError {
    fn drop(&mut self) {
        if !self.is_ok {
            let _ = std::io::stderr().lock().write_all(&self.buf);
        }
    }
}

macro_rules! print_flags {
    (fn $func:ident, $($flag:ident => $str:expr,)*) => {
        pub fn $func(mut out: impl Write, flags: u16) -> io::Result<()> {
            write!(out, "flags=")?;
            let values = [
                $((consts::$flag as u16, $str),)*
            ];
            write!(out, "[")?;
            let mut first = true;
            for (bits, str) in values {
                if flags & bits != bits {
                    continue;
                }
                if first {
                    first = false;
                } else {
                    write!(out, ",")?;
                }
                write!(out, "{str}")?;
            }
            write!(out, "]")?;
            Ok(())
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
