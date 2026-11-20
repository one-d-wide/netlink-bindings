use netlink_bindings::{
    nftables::{self, Nfgenmsg},
    rt_link::{self, RtextFilter},
    traits::NetlinkRequest,
    utils::{self, Rec},
};
use netlink_socket2::{ReplyError, std::NetlinkSocket};
use std::{
    cell::{LazyCell, RefCell},
    io::ErrorKind,
    sync::LazyLock,
};

thread_local! {
    pub static THREAD_SOCKET: RefCell<LazyCell<NetlinkSocket>>
        = RefCell::new(LazyCell::new(NetlinkSocket::new));
}

pub fn get_ifindex_str(ifname: &str) -> u32 {
    get_ifindex(ifname.as_bytes())
}

pub fn get_ifindex(ifname: &[u8]) -> u32 {
    THREAD_SOCKET.with_borrow_mut(|sock| {
        let mut req = rt_link::Request::new().op_getlink_do(&Default::default());
        req.encode()
            .push_ext_mask(RtextFilter::Vf as u32 | RtextFilter::SkipStats as u32)
            .push_ifname_bytes(ifname);

        let mut res = 0;
        if let Ok(mut iter) = sock.request(&req)
            && let Ok((header, _attrs)) = iter.recv_one()
        {
            res = header.ifi_index as u32;
        }
        res
    })
}

pub fn iptables_get_latest_gen_id(sock: &mut NetlinkSocket) -> Result<u32, ReplyError> {
    let request = nftables::Request::new().op_getgen_do(&Nfgenmsg::new());
    let mut iter = sock.request(&request)?;
    let (_, attrs) = iter.recv_one()?;

    Ok(attrs.get_id()?)
}

pub trait IptablesDo {
    fn iptables_request_chained(&mut self, chained: &nftables::Chained) -> Result<(), ReplyError>;
}

impl IptablesDo for NetlinkSocket {
    fn iptables_request_chained(&mut self, chained: &nftables::Chained) -> Result<(), ReplyError> {
        iptables_request_chained(self, chained)
    }
}

/// Submit an nftables transaction using the current generation id (genid), retrying if necessary.
///
/// For simple changes (inserting a rule or deleting it by handle) this is what you need.
///
/// For more complex changes (moving rules across entire tables) this ensures changes stay
/// consistent while a program inspects the current nftables state and executes a request that
/// modifies it. In such cases you may want write this loop yourself, ensuring that both your
/// queries and subsequent changes are performed against the same genid, retrying if necessary.
pub fn iptables_request<R: NetlinkRequest + ?Sized>(
    sock: &mut NetlinkSocket,
    req: &R,
) -> Result<(), ReplyError> {
    if req.flags() & libc::NLM_F_DUMP as u16 == libc::NLM_F_DUMP as u16 {
        *WARN_DUMP_ONCE;
    }

    let mut batch_header = nftables::Nfgenmsg::new();
    batch_header.set_res_id(10);

    let mut buf = Vec::new();

    loop {
        let gen_id = iptables_get_latest_gen_id(sock)?;

        let mut c = nftables::Chained::new_with_buf(&mut buf, sock.reserve_seq(256));

        c.request()
            .op_batch_begin_do(&batch_header)
            .encode()
            .push_genid(gen_id);

        c.with_request(req);

        c.request().op_batch_end_do(&batch_header);

        match sock.request_chained(&c.finalize())?.recv_all() {
            Ok(()) => return Ok(()),
            Err(err) if err.as_io_error().kind() == ErrorKind::Interrupted => continue,
            Err(err) => return Err(err),
        }
    }
}

pub fn iptables_request_chained(
    sock: &mut NetlinkSocket,
    chained: &nftables::Chained,
) -> Result<(), ReplyError> {
    for (h, _) in utils::IterableMessages::new(chained.buf()) {
        if h.r#flags & libc::NLM_F_DUMP as u16 == libc::NLM_F_DUMP as u16 {
            *WARN_DUMP_ONCE;
        }
    }

    let mut batch_header = nftables::Nfgenmsg::new();
    batch_header.set_res_id(10);

    let mut buf = Vec::new();
    loop {
        let gen_id = iptables_get_latest_gen_id(sock)?;

        let mut c = nftables::Chained::new_with_buf(&mut buf, sock.reserve_seq(256));

        c.request()
            .op_batch_begin_do(&batch_header)
            .encode()
            .push_genid(gen_id);

        c.with_chained(chained);

        c.request().op_batch_end_do(&batch_header);

        match sock.request_chained(&c.finalize())?.recv_all() {
            Ok(()) => return Ok(()),
            Err(err) if err.as_io_error().kind() == ErrorKind::Interrupted => continue,
            Err(err) => return Err(err),
        }
    }
}

pub fn iptables_request_raw(sock: &mut NetlinkSocket, req: &[u8]) -> Result<(), ReplyError> {
    for (h, _) in utils::IterableMessages::new(req) {
        if h.r#flags & libc::NLM_F_DUMP as u16 == libc::NLM_F_DUMP as u16 {
            *WARN_DUMP_ONCE;
        }
    }

    let mut batch_header = nftables::Nfgenmsg::new();
    batch_header.set_res_id(10);

    let mut buf = Vec::new();
    loop {
        let gen_id = iptables_get_latest_gen_id(sock)?;

        let mut c = nftables::Chained::new_with_buf(&mut buf, sock.reserve_seq(256));

        c.request()
            .op_batch_begin_do(&batch_header)
            .encode()
            .push_genid(gen_id);

        for (h, r) in utils::IterableMessages::new(req) {
            c.request()
                .set_flags(h.flags)
                .op_do(h.r#type, &Nfgenmsg::new_from_zeroed(&r))
                .encode()
                .as_vec_mut()
                .extend_from_slice(&r[Nfgenmsg::len()..]);
        }

        c.request().op_batch_end_do(&batch_header);

        match sock.request_chained(&c.finalize())?.recv_all() {
            Ok(()) => return Ok(()),
            Err(err) if err.as_io_error().kind() == ErrorKind::Interrupted => continue,
            Err(err) => return Err(err),
        }
    }
}

static WARN_DUMP_ONCE: LazyLock<()> = LazyLock::new(|| {
    eprintln!("ip-route: You're using iptables_request() to send nftables DUMP request.");
    eprintln!("ip-route: This's probably an error. Use NetlinkSocket::request() instead.");
});
