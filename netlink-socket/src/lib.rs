use std::{
    collections::{hash_map::Entry, HashMap},
    fmt,
    io::{self, ErrorKind, IoSlice, Read, Write},
    marker::PhantomData,
    net::TcpStream,
    os::fd::{AsRawFd, FromRawFd, OwnedFd},
    sync::Arc,
};

use netlink_bindings::{
    builtin::{NlmsgerrAttrs, PushNlmsghdr},
    nlctrl,
    utils::{self, Iterable},
    NetlinkRequest, Protocol,
};

// Netlink documentation recommends max(8192, page_size)
pub const RECV_BUF_SIZE: usize = 8192;

pub struct NetlinkSocket {
    buf: Arc<[u8; RECV_BUF_SIZE]>,
    cache: HashMap<&'static [u8], u16>,
    sock: HashMap<u16, TcpStream>,
    seq: u32,
}

impl NetlinkSocket {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            buf: Arc::new([0u8; RECV_BUF_SIZE]),
            cache: HashMap::default(),
            sock: HashMap::new(),
            seq: 0,
        }
    }

    fn get_socket(family: u16) -> io::Result<TcpStream> {
        let fd = unsafe {
            libc::socket(
                libc::AF_NETLINK,
                libc::SOCK_RAW | libc::SOCK_CLOEXEC,
                family as i32,
            )
        };
        if fd < 0 {
            return Err(io::Error::from_raw_os_error(-fd));
        }
        let fd = unsafe { OwnedFd::from_raw_fd(fd) };

        // Enable extended attributes in libc::NLMSG_ERROR and libc::NLMSG_DONE
        let res = unsafe {
            libc::setsockopt(
                fd.as_raw_fd(),
                libc::SOL_NETLINK,
                libc::NETLINK_EXT_ACK,
                (&1u32) as *const u32 as *const libc::c_void,
                4,
            )
        };
        if res < 0 {
            return Err(io::Error::from_raw_os_error(-res));
        }

        Ok(fd.into())
    }

    pub fn resolve(&mut self, family_name: &'static [u8]) -> io::Result<u16> {
        if let Some(id) = self.cache.get(family_name) {
            return Ok(*id);
        }

        let mut request = nlctrl::Request::new().op_getfamily_do_request();
        request.encode().push_family_name_bytes(family_name);

        match request.protocol() {
            Protocol::Raw {
                protonum,
                request_type,
            } => {
                assert_eq!(protonum, libc::NETLINK_GENERIC as u16);
                assert_eq!(request_type, libc::GENL_ID_CTRL as u16);
            }
            _ => unreachable!(),
        }

        let mut iter = self.request(&request)?;
        if let Some(reply) = iter.recv() {
            let Ok(id) = reply?.get_family_id() else {
                return Err(ErrorKind::Unsupported.into());
            };
            self.cache.insert(family_name, id);
            return Ok(id);
        }

        Err(ErrorKind::UnexpectedEof.into())
    }

    pub fn request<'sock, Request>(
        &'sock mut self,
        request: &Request,
    ) -> io::Result<NetlinkReply<'sock, Request>>
    where
        Request: NetlinkRequest,
    {
        let (protonum, request_type) = match request.protocol() {
            Protocol::Raw {
                protonum,
                request_type,
            } => (protonum, request_type),
            Protocol::Generic(name) => (libc::GENL_ID_CTRL as u16, self.resolve(name)?),
        };

        let mut header = PushNlmsghdr::new();
        header.set_len(header.as_slice().len() as u32 + request.payload().len() as u32);
        header.set_type(request_type);
        header.set_flags(request.flags() | libc::NLM_F_REQUEST as u16 | libc::NLM_F_ACK as u16);
        header.set_seq({
            self.seq = self.seq.wrapping_add(1);
            self.seq
        });

        let sock = match self.sock.entry(protonum) {
            Entry::Occupied(sock) => sock.into_mut(),
            Entry::Vacant(ent) => {
                let sock = Self::get_socket(protonum)?;
                ent.insert(sock)
            }
        };

        let sent = sock.write_vectored(&[
            IoSlice::new(header.as_slice()),
            IoSlice::new(request.payload()),
        ])?;
        if sent != header.get_len() as usize {
            return Err(io::Error::other("Couldn't send the whole message"));
        }

        Ok(NetlinkReply {
            sock: self,
            buf_offset: 0,
            buf_read: 0,
            protonum,
            seq: header.seq(),
            done: false,
            phantom: PhantomData,
        })
    }
}

pub struct NetlinkReply<'sock, Request: NetlinkRequest> {
    sock: &'sock mut NetlinkSocket,
    buf_offset: usize,
    buf_read: usize,
    protonum: u16,
    seq: u32,
    done: bool,
    phantom: PhantomData<Request>,
}

impl<Request: NetlinkRequest> NetlinkReply<'_, Request> {
    pub fn recv(&mut self) -> Option<Result<Request::ReplyType<'_>, ReplyError>> {
        if self.done {
            return None;
        }

        loop {
            let buf = Arc::make_mut(&mut self.sock.buf);

            if self.buf_offset == self.buf_read {
                let sock = self.sock.sock.get_mut(&self.protonum).unwrap();
                let read = match sock.read(&mut buf[..]) {
                    Ok(read) => read,
                    Err(err) if err.kind() == ErrorKind::Interrupted => continue,
                    Err(err) => return Some(Err(ReplyError::from_io_err::<Request>(err))),
                };

                self.buf_read = read;
                self.buf_offset = 0;
            }

            let packet = &buf[self.buf_offset..self.buf_read];

            let too_short_err = || {
                Some(Err(ReplyError::from_io_err::<Request>(io::Error::other(
                    "Received packet is too short",
                ))))
            };

            let Some(header) = packet.get(..PushNlmsghdr::len()) else {
                return too_short_err();
            };
            let header = PushNlmsghdr::new_from_slice(header).unwrap();

            let payload_start = self.buf_offset + PushNlmsghdr::len();
            self.buf_offset += header.get_len() as usize;

            if header.seq() != self.seq {
                continue;
            }

            match header.get_type() as i32 {
                libc::NLMSG_NOOP => continue,
                libc::NLMSG_DONE | libc::NLMSG_ERROR => {
                    self.done = true;

                    let Some(code) = packet.get(16..20) else {
                        return too_short_err();
                    };
                    let code = utils::parse_i32(code).unwrap();
                    if code == 0 {
                        return None;
                    }

                    let (echo_start, echo_end) = if header.get_type() == libc::NLMSG_DONE as u16 {
                        (20, 20)
                    } else {
                        let Some(echo_header) = packet.get(20..(20 + PushNlmsghdr::len())) else {
                            return too_short_err();
                        };
                        let echo_header = PushNlmsghdr::new_from_slice(echo_header).unwrap();

                        if echo_header.flags() & libc::NLM_F_CAPPED as u16 == 0 {
                            let start = echo_header.get_len();
                            if packet.len() < start as usize + 20 {
                                return too_short_err();
                            }

                            (20 + 16, 20 + start as usize)
                        } else {
                            let ext_ack_start = 20 + PushNlmsghdr::len();
                            (ext_ack_start, ext_ack_start)
                        }
                    };

                    return Some(Err(ReplyError {
                        code: io::Error::from_raw_os_error(-code),
                        reply_buf: Some(self.sock.buf.clone()),
                        request_bounds: (echo_start as u32, echo_end as u32),
                        ext_ack_bounds: (echo_end as u32, self.buf_offset as u32),
                        lookup: Request::lookup,
                    }));
                }
                libc::NLMSG_OVERRUN => {
                    return Some(Err(ReplyError::from_io_err::<Request>(io::Error::other(
                        "Received NLMSG_OVERRUN",
                    ))));
                }
                _ => {
                    return Some(Ok(Request::decode_reply(
                        &self.sock.buf[payload_start..self.buf_offset],
                    )))
                }
            }
        }
    }
}

// For an error type to be convenient to use it has to be unconstrained by
// lifetime bounds and generics, so it can be freely passed around in a call
// chain, therefore the data buffers are ref counter.
pub struct ReplyError {
    code: io::Error,
    reply_buf: Option<Arc<[u8; RECV_BUF_SIZE]>>,
    ext_ack_bounds: (u32, u32),
    request_bounds: (u32, u32),
    #[allow(clippy::type_complexity)]
    lookup: fn(&[u8], usize, Option<u16>) -> (Vec<(&'static str, usize)>, Option<&'static str>),
}

impl ReplyError {
    fn from_io_err<Request: NetlinkRequest>(err: io::Error) -> Self {
        Self {
            code: err,
            reply_buf: None,
            request_bounds: (0, 0),
            ext_ack_bounds: (0, 0),
            lookup: Request::lookup,
        }
    }

    pub fn ext_ack(&self) -> Option<Iterable<'_, NlmsgerrAttrs<'_>>> {
        let Some(reply_buf) = &self.reply_buf else {
            return None;
        };
        Some(NlmsgerrAttrs::new(
            &reply_buf[self.ext_ack_bounds.0 as usize..self.ext_ack_bounds.1 as usize],
        ))
    }

    pub fn request(&self) -> Option<&[u8]> {
        let Some(reply_buf) = &self.reply_buf else {
            return None;
        };
        Some(&reply_buf[self.request_bounds.0 as usize..self.request_bounds.1 as usize])
    }
}

impl From<ReplyError> for io::Error {
    fn from(value: ReplyError) -> Self {
        value.code
    }
}

impl std::error::Error for ReplyError {}

impl fmt::Debug for ReplyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

impl fmt::Display for ReplyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.code)?;

        let Some(ext_ack) = self.ext_ack() else {
            return Ok(());
        };

        if let Ok(msg) = ext_ack.get_msg() {
            f.write_str(": ")?;
            match msg.to_str() {
                Ok(m) => write!(f, "{m}")?,
                Err(_) => write!(f, "{msg:?}")?,
            }
        }

        if let Ok(missing_offset) = ext_ack.get_missing_nest() {
            let missing_attr = ext_ack.get_missing_type().ok();

            let (trace, attr) = (self.lookup)(
                self.request().unwrap(),
                missing_offset as usize - PushNlmsghdr::len(),
                missing_attr,
            );

            if let Some(attr) = attr {
                write!(f, ": missing {attr:?}")?;
            }
            for (attrs, _) in trace.iter() {
                write!(f, " in {attrs:?}")?;
            }
        }

        if let Ok(invalid_offset) = ext_ack.get_offset() {
            let (trace, _) = (self.lookup)(
                self.request().unwrap(),
                invalid_offset as usize - PushNlmsghdr::len(),
                None,
            );

            if let Some((attr, _)) = trace.first() {
                write!(f, ": attribute {attr:?}")?;
            }
            for (attrs, _) in trace.iter().skip(1) {
                write!(f, " in {attrs:?}")?;
            }
            if let Ok(policy) = ext_ack.get_policy() {
                write!(f, ": {policy:?}")?;
            }
        }

        if ext_ack.get_buf().is_empty() {
            write!(f, " (no extended ack)")?;
        }

        Ok(())
    }
}
