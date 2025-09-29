#[derive(Debug, Clone, Copy)]
pub enum Protocol {
    /// Netlink-raw protocol.
    /// Protonum is the protocol value the socket(2) was created with.
    /// Request type is the type value from the message header.
    Raw { protonum: u16, request_type: u16 },
    /// Name of a generic netlink family
    Generic(&'static [u8]),
}

/// Common trait describing how to handle a particular request.
/// Designed to be used by the netlink socket implementations.
pub trait NetlinkRequest {
    fn protocol(&self) -> Protocol;
    /// Additional flags specified in the message header
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
