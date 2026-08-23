use netlink_bindings::{
    builtin::Nlmsghdr,
    nftables::{self, Nfgenmsg},
    rt_link::{self, RtextFilter},
    traits::{NetlinkChained, NetlinkRequest, Protocol},
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

/// A dyn-compatible version of NetlinkRequest.
///
/// It doesn't support decoding replies and doesn't do attribute lookup for extended-ack error info
/// (not like it's that widely supported in the kernel anyway).
pub trait NetlinkRequestAcked {
    fn protocol(&self) -> Protocol;
    fn flags(&self) -> u16;
    fn payload(&self) -> &[u8];
}

impl<T: NetlinkRequest> NetlinkRequestAcked for T {
    fn protocol(&self) -> Protocol {
        self.protocol()
    }
    fn flags(&self) -> u16 {
        self.flags()
    }
    fn payload(&self) -> &[u8] {
        self.payload()
    }
}

pub trait IptablesDo {
    fn iptables_request(&mut self, req: &dyn NetlinkRequestAcked) -> Result<(), ReplyError>;
    fn iptables_request_chained(&mut self, chained: &dyn NetlinkChained) -> Result<(), ReplyError>;
    fn iptables_request_raw(&mut self, req: &[u8]) -> Result<(), ReplyError>;
}

impl IptablesDo for NetlinkSocket {
    fn iptables_request(&mut self, req: &dyn NetlinkRequestAcked) -> Result<(), ReplyError> {
        iptables_request(self, req)
    }
    fn iptables_request_chained(&mut self, chained: &dyn NetlinkChained) -> Result<(), ReplyError> {
        iptables_request_chained(self, chained)
    }
    fn iptables_request_raw(&mut self, req: &[u8]) -> Result<(), ReplyError> {
        iptables_request_raw(self, req)
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
pub fn iptables_request(
    sock: &mut NetlinkSocket,
    req: &dyn NetlinkRequestAcked,
) -> Result<(), ReplyError> {
    if req.flags() & libc::NLM_F_DUMP as u16 == libc::NLM_F_DUMP as u16 {
        *WARN_DUMP_ONCE;
    }

    let mut batch_header = nftables::Nfgenmsg::new();
    batch_header.set_res_id(10);

    let mut buf = Vec::new();

    let Protocol::Raw { request_type, .. } = req.protocol() else {
        unreachable!()
    };

    loop {
        let gen_id = iptables_get_latest_gen_id(sock)?;

        let mut c = nftables::Chained::new_with_buf(&mut buf, sock.reserve_seq(256));

        c.request()
            .op_batch_begin_do(&batch_header)
            .encode()
            .push_genid(gen_id);

        c.request()
            .set_flags(req.flags())
            .op_do(request_type, &Nfgenmsg::new_from_zeroed(req.payload()))
            .encode()
            .as_vec_mut()
            .extend_from_slice(&req.payload()[nftables::Nfgenmsg::len()..]);

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
    chained: &dyn NetlinkChained,
) -> Result<(), ReplyError> {
    iptables_request_raw(sock, chained.payload())
}

pub fn iptables_request_raw(sock: &mut NetlinkSocket, req: &[u8]) -> Result<(), ReplyError> {
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

        let mut buf = req;
        while !buf.is_empty() {
            let h = Nlmsghdr::new_from_zeroed(buf);
            let len = (h.len as usize).min(buf.len());

            if h.r#flags & libc::NLM_F_DUMP as u16 == libc::NLM_F_DUMP as u16 {
                *WARN_DUMP_ONCE;
            }

            c.request()
                .set_flags(h.flags)
                .op_do(h.r#type, &Nfgenmsg::new_from_zeroed(&buf[16..]))
                .encode()
                .as_vec_mut()
                .extend_from_slice(&req[16 + nftables::Nfgenmsg::len()..]);

            buf = &buf[utils::nla_align_up(len)..];
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
