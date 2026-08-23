#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Protocol {
    /// Legacy Netlink protocol, aka netlink-raw
    Raw {
        /// Value supplied to socket(2)
        protonum: u16,
        /// Value of `type` field in the message header
        request_type: u16,
    },
    /// Generic Netlink protocol, aka genetlink or genl
    ///
    /// Note that the message also carries a command type in the required Nfgenmsg header.
    Generic(&'static [u8]),
}

/// A trait describing how to handle a particular request.
/// It designed to be used by a netlink socket implementation.
pub trait NetlinkRequest {
    /// Netlink protocol to use
    fn protocol(&self) -> Protocol;

    /// Additional `flags` specified in the message header
    fn flags(&self) -> u16;

    /// Encoded payload of the message (without message header)
    fn payload(&self) -> &[u8];

    // type RequestType<'buf>;
    // fn decode_request(buf: &[u8]) -> Self::RequestType<'_>;

    type ReplyType<'buf>;
    fn decode_reply(buf: &[u8]) -> Self::ReplyType<'_>;

    /// Lookup an attribute and it's parents in the request payload by offset
    fn lookup(
        buf: &[u8],
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let _ = buf;
        let _ = offset;
        let _ = missing_type;
        (Vec::new(), None)
    }
}

/// Function signature of [`NetlinkRequest::lookup`]
pub type LookupFn =
    fn(&[u8], usize, Option<u16>) -> (Vec<(&'static str, usize)>, Option<&'static str>);

/// A chain of requests encoded into the single buffer (experimental)
pub trait NetlinkChained {
    fn protonum(&self) -> u16;

    /// Encoded payload of the messages (including message headers)
    fn payload(&self) -> &[u8];

    /// Number of messages in the chain
    fn chain_len(&self) -> usize;

    fn get_index(&self, seq: u32) -> Option<usize>;

    fn name(&self, index: usize) -> &'static str;

    fn lookup(&self, index: usize) -> LookupFn {
        let _ = index;
        |_, _, _| Default::default()
    }

    /// Packet supports ack on success with NLM_F_ACK (assumed true by default).
    /// To date, this's only used to workaround a bug in nftables prior to linux 6.10.
    ///
    /// Caller sequentially peeks indexes 0..chain_len() until it encounters None.
    #[doc(hidden)]
    fn supports_ack(&self, index: usize) -> Option<bool> {
        let _ = index;
        None
    }
}

/// A trait for `Push*` structs to access the internal buffer.
///
/// You can use it to inspect, modify, or append attributes,
/// e.g. by copying from another attribute set.
///
/// Use this trait with caution as there's no further type checks!
pub trait Pusher {
    fn as_vec(&self) -> &Vec<u8>;
    fn as_vec_mut(&mut self) -> &mut Vec<u8>;

    // TODO: drop in v0.4
    #[deprecated = "Use .as_vec() instead (rec -> vec)"]
    fn as_rec(&self) -> &Vec<u8> {
        self.as_vec()
    }
    #[deprecated = "Use .as_vec_mut() instead (rec -> vec)"]
    fn as_rec_mut(&mut self) -> &mut Vec<u8> {
        self.as_vec_mut()
    }
}

impl Pusher for Vec<u8> {
    fn as_vec(&self) -> &Vec<u8> {
        self
    }
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self
    }
}

impl Pusher for &mut Vec<u8> {
    fn as_vec(&self) -> &Vec<u8> {
        self
    }
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self
    }
}
