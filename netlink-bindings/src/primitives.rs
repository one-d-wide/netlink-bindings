pub use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr, SocketAddrV4, SocketAddrV6};

macro_rules! gen_parse {
    ($name:ident => $from:ident, $type:ident) => {
        pub fn $name(buf: &[u8]) -> Option<$type> {
            if buf.len() != size_of::<$type>() {
                return None;
            }
            Some($type::$from(buf.try_into().unwrap()))
        }
    };
    ($($name:ident => $from:ident, $type:ident;)+) => {
        $(gen_parse!($name => $from, $type);)+
    }
}

gen_parse! {
    parse_u8      => from_ne_bytes, u8;
    parse_u16     => from_ne_bytes, u16;
    parse_u32     => from_ne_bytes, u32;
    parse_u64     => from_ne_bytes, u64;
    parse_u128    => from_ne_bytes, u128;

    parse_be_u8   => from_be_bytes, u8;
    parse_be_u16  => from_be_bytes, u16;
    parse_be_u32  => from_be_bytes, u32;
    parse_be_u64  => from_be_bytes, u64;
    parse_be_u128 => from_be_bytes, u128;

    parse_le_u8   => from_le_bytes, u8;
    parse_le_u16  => from_le_bytes, u16;
    parse_le_u32  => from_le_bytes, u32;
    parse_le_u64  => from_le_bytes, u64;
    parse_le_u128 => from_le_bytes, u128;

    parse_i8      => from_ne_bytes, i8;
    parse_i16     => from_ne_bytes, i16;
    parse_i32     => from_ne_bytes, i32;
    parse_i64     => from_ne_bytes, i64;
    parse_i128    => from_ne_bytes, i128;

    parse_be_i8   => from_be_bytes, i8;
    parse_be_i16  => from_be_bytes, i16;
    parse_be_i32  => from_be_bytes, i32;
    parse_be_i64  => from_be_bytes, i64;
    parse_be_i128 => from_be_bytes, i128;

    parse_le_i8   => from_le_bytes, i8;
    parse_le_i16  => from_le_bytes, i16;
    parse_le_i32  => from_le_bytes, i32;
    parse_le_i64  => from_le_bytes, i64;
    parse_le_i128 => from_le_bytes, i128;
}

pub fn parse_addr_v4(buf: &[u8]) -> Option<Ipv4Addr> {
    // TODO: check family
    parse_be_u32(buf).map(Ipv4Addr::from_bits)
}

pub fn parse_addr_v6(buf: &[u8]) -> Option<Ipv6Addr> {
    // TODO: check family
    parse_be_u128(buf).map(Ipv6Addr::from_bits)
}

pub fn parse_addr(buf: &[u8]) -> Option<IpAddr> {
    match buf.len() {
        4 => Some(IpAddr::V4(parse_addr_v4(buf).unwrap())),
        8 => Some(IpAddr::V6(parse_addr_v6(buf).unwrap())),
        _ => None,
    }
}

pub fn parse_sockaddr_v4(buf: &[u8]) -> Option<SocketAddrV4> {
    // TODO: check family
    if buf.len() != 16 {
        return None;
    }
    Some(SocketAddrV4::new(
        Ipv4Addr::from_bits(parse_be_u32(&buf[4..8])?),
        parse_be_u16(&buf[2..4])?,
    ))
}

pub fn parse_sockaddr_v6(buf: &[u8]) -> Option<SocketAddrV6> {
    // TODO: check family
    if buf.len() != 28 {
        return None;
    }
    Some(SocketAddrV6::new(
        Ipv6Addr::from_bits(parse_be_u128(&buf[8..24])?),
        parse_be_u16(&buf[2..4])?,
        parse_u32(&buf[4..8])?,
        parse_u32(&buf[24..28])?,
    ))
}

pub fn parse_sockaddr(buf: &[u8]) -> Option<SocketAddr> {
    match buf.len() {
        16 => Some(SocketAddr::V4(parse_sockaddr_v4(buf).unwrap())),
        28 => Some(SocketAddr::V6(parse_sockaddr_v6(buf).unwrap())),
        _ => None,
    }
}

pub fn encode_ip_v4(buf: &mut Vec<u8>, val: Ipv4Addr) {
    buf.extend(&val.to_bits().to_be_bytes())
}

pub fn encode_ip_v6(buf: &mut Vec<u8>, val: Ipv6Addr) {
    buf.extend(&val.to_bits().to_be_bytes())
}

pub fn encode_ip(buf: &mut Vec<u8>, val: IpAddr) {
    match val {
        IpAddr::V4(addr) => encode_ip_v4(buf, addr),
        IpAddr::V6(addr) => encode_ip_v6(buf, addr),
    }
}

pub fn encode_sockaddr_v4(buf: &mut Vec<u8>, val: SocketAddrV4) {
    buf.extend(&2u16.to_ne_bytes()); // AF_INET
    buf.extend(&val.port().to_be_bytes());
    buf.extend(&val.ip().to_bits().to_be_bytes());
    buf.extend(std::iter::repeat_n(0u8, 8)); // padding
}

pub fn encode_sockaddr_v6(buf: &mut Vec<u8>, val: SocketAddrV6) {
    buf.extend(&10u16.to_ne_bytes()); // AF_INET6
    buf.extend(&val.port().to_be_bytes());
    buf.extend(&val.flowinfo().to_ne_bytes());
    buf.extend(&val.ip().to_bits().to_be_bytes());
    buf.extend(&val.scope_id().to_ne_bytes());
}

pub fn encode_sockaddr(buf: &mut Vec<u8>, val: SocketAddr) {
    match val {
        SocketAddr::V4(addr) => encode_sockaddr_v4(buf, addr),
        SocketAddr::V6(addr) => encode_sockaddr_v6(buf, addr),
    }
}
