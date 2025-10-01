#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Protocol {
    /// Netlink-raw protocol.
    /// Protonum is the `protocol` value supplied to socket(2).
    /// Request type is the value of `type` field in the message header.
    Raw { protonum: u16, request_type: u16 },
    /// Generic netlink protocol.
    /// Name of a generic netlink family.
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
