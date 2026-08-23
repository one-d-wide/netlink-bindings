#![allow(unused)]

use netlink_bindings::traits::Protocol;
use netlink_bindings::utils::{self, IterableChunks};
use netlink_bindings::{consts, nftables, rt_addr, rt_link, rt_route, rt_rule};
use netlink_socket2::NetlinkSocket;
use reverse_lookup::{self as rl, ReverseLookup};
use std::io::ErrorKind;
use std::sync::mpsc;
use std::thread::JoinHandle;

pub fn setup() {
    let mut sock = NetlinkSocket::new();
    let req = nftables::Request::new().op_getgen_do(&Default::default());
    let mut iter = sock.request(&req).unwrap();
    if let Err(err) = iter.recv_one()
        && err.as_io_error().kind() == ErrorKind::PermissionDenied
    {
        panic!(
            "
        Lacking permission to manipulate network devices.
        Run tests under: unshare --user --net --keep-caps -- ...
"
        );
    }
    // TODO: check if running in the global network namespace
}

fn filter_message(r: &rl::ReverseLookup) -> bool {
    r.request_value.is_none()
}

fn is_nftables(r: &ReverseLookup) -> bool {
    matches!(r.proto, Protocol::Raw { protonum, .. } if protonum == libc::NETLINK_NETFILTER as u16)
}

fn skip(r: &rl::ReverseLookup) -> usize {
    match r.proto {
        Protocol::Raw {
            protonum,
            request_type,
        } => match protonum as i32 {
            libc::NETLINK_NETFILTER => nftables::Nfgenmsg::len(),
            libc::NETLINK_ROUTE => match request_type & !0b11 {
                libc::RTM_NEWLINK => rt_link::Ifinfomsg::len(),
                libc::RTM_NEWROUTE => rt_route::Rtmsg::len(),
                libc::RTM_NEWRULE => rt_rule::FibRuleHdr::len(),
                libc::RTM_NEWADDR => rt_addr::Ifaddrmsg::len(),
                v => unimplemented!("Unknown ROUTE family: {v}"),
            },
            prot => unimplemented!("Unknown protocol: {prot}"),
        },
        prot => unimplemented!("Unknown protocol: {prot:?}"),
    }
}

pub fn compare_recurse_slice(l: &[rl::ReverseLookup], r: &[rl::ReverseLookup]) {
    'next: for (_ri, r) in r.iter().enumerate() {
        if let Protocol::Raw {
            protonum,
            request_type,
        } = r.proto
            && protonum == libc::NETLINK_NETFILTER as u16
            && matches!(request_type, 0x10 | 0x11)
        {
            continue; // Skip batch-{begin,end}
        }

        for (_li, l) in l.iter().enumerate() {
            // eprintln!("Comparing request l[{_li}] with r[{_ri}]");

            if compare_recurse(l, r) {
                // eprintln!("Matched");
                continue 'next;
            }
        }

        // TODO: highlight the most similar message

        scopeguard::defer_on_unwind! {
            // eprintln!("RIGHT: {r:#?}");

            let supress_flags = libc::NLM_F_ACK | libc::NLM_F_ATOMIC;
            let rflags = r.header.flags & !supress_flags as u16;

            rl::print_request_flags(std::io::stderr().lock(), rflags).unwrap();
            eprintln!();
            eprintln!("--- LEFT HEADERS ---");


            for (i, l) in l.iter().enumerate() {
                let lflags = l.header.flags & !supress_flags as u16;
                rl::print_request_flags(std::io::stderr().lock(), lflags).unwrap();
                eprintln!();
                eprintln!("LEFT HEADER {i}: {:#?}", l.header)
            }
        }

        panic!(
            "RIGHT has a message that is not in LEFT:\n--- RIGHT ---\n{:#?}\n{r:#?}\n--- LEFT ---\n{l:#?}",
            r.header,
        );
    }

    assert!(r.len() > 0);
}

fn attrs_sort_shallow(rl: &ReverseLookup, buf: &[u8], skip: usize, stack: &Vec<u16>) -> Vec<u8> {
    let mut vec = Vec::new();
    for (header, buf) in IterableChunks::new(&buf[skip..]) {
        // Skip rule-attrs.compat
        if is_nftables(rl) && stack == &[] && header.r#type == 5 {
            continue;
        }

        // Skip rule-attrs.expressions.elem (if elem.name == "counter")
        // if is_nftables(rl)
        //     && stack == &[4]
        //     && header.r#type == 1
        //     && buf.starts_with(b"\x0c\x00\x01\x00counter\0")
        // {
        //     continue;
        // }

        vec.push((header.r#type, header, buf));
    }

    if is_nftables(rl) && stack == &[1] {
        // Don't sort rule-attrs.expressions[]
    } else {
        vec.sort();
    }

    let mut res = Vec::with_capacity(buf.len());
    res.extend_from_slice(&buf[..skip]);
    for (_, header, buf) in vec {
        utils::push_header_type(&mut res, header.r#type, buf.len() as u16, header.is_nested);
        res.extend_from_slice(&buf);
    }

    res
}

pub fn compare_recurse(l: &rl::ReverseLookup, r: &rl::ReverseLookup) -> bool {
    macro_rules! assert_eq {
        ($l:expr, $r:expr) => {
            if $l != $r {
                return false;
            }
        };
    }
    macro_rules! panic {
        ($first:expr $(,$rest:expr)*) => {
            eprintln!($first $(,$rest)*);
            return false;
        };
    }

    scopeguard::defer_on_unwind! {
        eprintln!("^^^ WHILE TESTING ^^^\n{l:#?}\n--- AGAINST ---\n{r:#?}");
    };

    {
        let supress_flags = libc::NLM_F_ACK | libc::NLM_F_ATOMIC;
        let lflags = l.header.flags & !supress_flags as u16;
        let rflags = r.header.flags & !supress_flags as u16;

        scopeguard::defer_on_unwind! {
            eprintln!("^^^ WHILE TESTING ^^^\n{:#?}\n--- AGAINST ---\n{:#?}", l.header, r.header);
            eprint!("LEFT: ");
            rl::print_request_flags(std::io::stderr().lock(), lflags).unwrap();
            eprintln!();
            eprint!("RIGHT: ");
            rl::print_request_flags(std::io::stderr().lock(), rflags).unwrap();
            eprintln!();
        };

        // assert_eq!(l.header.len, r.header.len);
        assert_eq!(l.header.r#type, r.header.r#type);
        assert_eq!(lflags, rflags);
    }

    assert_eq!(l.proto, r.proto);
    assert_eq!(l.value, r.value);
    assert_eq!(l.is_dump, r.is_dump);
    let skip = skip(l);

    {
        scopeguard::defer_on_unwind! {
            eprintln!("HEADER");
        };
        // utils::dump_assert_eq(&l.buf[..skip], &r.buf[..skip]);
        if l.buf[..skip] != r.buf[..skip] {
            panic!("HEADER DIFFER");
        }
    }

    attrs_compare_recurse(r, &l.buf, &r.buf, skip, &Vec::new())
}

pub fn attrs_compare_recurse(
    rl: &ReverseLookup,
    l: &[u8],
    r: &[u8],
    skip: usize,
    stack: &Vec<u16>,
) -> bool {
    macro_rules! panic {
        ($first:expr $(,$rest:expr)*) => {
            let ops = rl::get_op(rl.proto, &rl.buf);
            let op = ops.first().unwrap_or(&"(unknown op)");
            eprintln!("While comparing {op} stack={stack:?}",);
            eprintln!($first $(,$rest)*);
            return false;
        };
    }
    macro_rules! assert_eq {
        ($l:expr, $r:expr) => {
            if $l != $r {
                panic!("l={:?}\n!=r={:?}", $l, $r);
            }
        };
    }

    let l = attrs_sort_shallow(rl, l, skip, stack);
    let r = attrs_sort_shallow(rl, r, skip, stack);
    let mut l = IterableChunks::new(&l[skip..]);
    let mut r = IterableChunks::new(&r[skip..]);
    loop {
        let l = l.next();
        let r = r.next();
        scopeguard::defer_on_unwind! {
            eprintln!("^^^ WHILE TESTING ^^^\n{l:#?}\nAGAINST\n{r:#?}");
        };
        match (l, r) {
            (Some((lh, lbuf)), Some((rh, rbuf))) => {
                assert_eq!(&lh.r#type, &rh.r#type);
                // assert_eq!(&lh.is_nested, &rh.is_nested);
                assert_eq!(&lbuf, &lbuf);

                if rh.is_nested {
                    let mut stack = stack.clone();
                    stack.push(rh.r#type);

                    let lbuf = attrs_sort_shallow(rl, lbuf, 0, &stack);
                    let rbuf = attrs_sort_shallow(rl, rbuf, 0, &stack);

                    if !attrs_compare_recurse(rl, &lbuf, &rbuf, 0, &stack) {
                        return false;
                    }
                } else {
                    if lbuf.len() + 1 == rbuf.len()
                        && lbuf[..] == rbuf[..rbuf.len() - 1]
                        && rbuf.last() == Some(&b'\0')
                    {
                        // A truncated C string
                    } else if rbuf.len() > 32 // arbitrary
                        && lbuf.starts_with(rbuf)
                        && lbuf.iter().skip(rbuf.len()).all(|b| *b == 0)
                    {
                        // Zero-padded struct, for some reason
                    } else {
                        scopeguard::defer_on_unwind! {
                            utils::dump_assert_eq(lbuf, rbuf);
                        };
                        if lbuf != rbuf {
                            panic!("BYTES DIFFER");
                        }
                    }
                };
            }
            (Some((lh, lbuf)), None) => {
                panic!("Left has more attrs: {lh:?} len={}", lbuf.len());
            }
            (None, Some((rh, rbuf))) => {
                panic!("Right has more attrs: {rh:?} len={}", rbuf.len());
            }
            (None, None) => return true,
        }
    }
}

pub fn trace_cmd_str(cmd: &str) -> Vec<rl::ReverseLookup> {
    trace_cmd(
        cmd.split(" ")
            .filter(|a| !a.is_empty())
            .map(|x| x.to_string())
            .collect(),
    )
}

pub fn trace_cmd(cmd: Vec<String>) -> Vec<rl::ReverseLookup> {
    assert!(matches!(cmd[0].as_str(), "ip" | "iptables" | "ip6tables"));

    let frames = mpsc::channel();

    let args = rl::CliArgs {
        command: true,
        // echo: true,
        args: cmd.clone(),
        ..Default::default()
    };

    let api = rl::ApiArgs {
        frames_tx: Some(frames.0),
        ..Default::default()
    };

    if !rl::run(args, api) {
        eprintln!("EXITED WITH ERROR: {:?}", cmd.join(" "));
    }

    let mut cmd_frames = Vec::new();
    while let Ok(f) = frames.1.try_recv() {
        if !filter_message(&f) {
            continue;
        }
        cmd_frames.push(f);
    }

    cmd_frames
}

pub struct TraceCtx {
    strace_pid: u32,
    frames_rx: mpsc::Receiver<rl::ReverseLookup>,
    join: JoinHandle<()>,
}

pub fn trace_start() -> TraceCtx {
    let strace_pid = mpsc::channel();
    let frames = mpsc::channel();

    let args = rl::CliArgs {
        pid: Some(unsafe { libc::gettid() as u32 }),
        ..Default::default()
    };

    let api = rl::ApiArgs {
        strace_pid_tx: Some(strace_pid.0),
        frames_tx: Some(frames.0),
        no_follow_fork: true,
        ..Default::default()
    };

    unsafe {
        libc::prctl(libc::PR_SET_PTRACER, libc::PR_SET_PTRACER_ANY);
    }

    let join = std::thread::spawn(|| {
        if !rl::run(args, api) {
            eprintln!("Process exited with error!");
        }
    });

    let strace_pid = strace_pid.1.recv().unwrap();

    TraceCtx {
        strace_pid,
        frames_rx: frames.1,
        join,
    }
}

pub fn trace_stop(c: TraceCtx) -> Vec<rl::ReverseLookup> {
    unsafe { libc::kill(c.strace_pid as i32, libc::SIGINT) };
    c.join.join().unwrap();

    let mut res = Vec::new();
    while let Ok(f) = c.frames_rx.try_recv() {
        if !filter_message(&f) {
            continue;
        }
        res.push(f);
    }

    res
}

pub fn trace_fn(f: impl FnOnce()) -> Vec<rl::ReverseLookup> {
    let ctx = trace_start();
    f();
    trace_stop(ctx)
}
