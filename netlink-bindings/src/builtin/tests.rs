use crate::utils::*;

use crate::builtin;
use crate::builtin::*;

#[test]
fn message_header() {
    #[cfg_attr(any(), rustfmt::skip)]
    let payload: Vec<u8> = std::iter::empty()
        .chain(1u32.to_ne_bytes()) // len
        .chain(2u16.to_ne_bytes()) // type
        .chain(3u16.to_ne_bytes()) // flags
        .chain(4u32.to_ne_bytes()) // seq
        .chain(5u32.to_ne_bytes()) // pid
        .collect();

    let h = PushNlmsghdr::new_from_slice(&payload).unwrap();
    assert_eq!(h.get_len(), 1);
    assert_eq!(h.get_type(), 2);
    assert_eq!(h.flags(), 3);
    assert_eq!(h.seq(), 4);
    assert_eq!(h.pid(), 5);

    let mut w = PushNlmsghdr::new();
    w.set_len(1);
    w.set_type(2);
    w.set_flags(3);
    w.set_seq(4);
    w.set_pid(5);

    assert_eq!(h.as_slice(), &payload[..]);
    assert_eq!(w.as_slice(), &payload[..]);
}

#[test]
fn ext_ack1() {
    #[cfg_attr(any(), rustfmt::skip)]
    let payload = &[
                                0x2f, 0x00, 0x01, 0x00, 0x50, 0x72, 0x6f, 0x76, 0x69, 0x64, 0x65, 0x64,
        0x20, 0x64, 0x65, 0x66, 0x61, 0x75, 0x6c, 0x74, 0x20, 0x78, 0x6d, 0x69, 0x74, 0x20, 0x70, 0x6f,
        0x6c, 0x69, 0x63, 0x79, 0x20, 0x6e, 0x6f, 0x74, 0x20, 0x73, 0x75, 0x70, 0x70, 0x6f, 0x72, 0x74,
        0x65, 0x64, 0x00, 0x00, 0x08, 0x00, 0x02, 0x00, 0x34, 0x00, 0x00, 0x00,
    ];

    dump_hex(payload);

    let req = NlmsgerrAttrs::new(payload);
    println!("{req:#?}");

    assert_eq!(
        req.get_msg(),
        Ok(c"Provided default xmit policy not supported")
    );
    assert_eq!(req.get_offset(), Ok(52));

    let mut buf = Vec::new();
    PushNlmsgerrAttrs::new(&mut buf)
        .push_msg(c"Provided default xmit policy not supported")
        .push_offset(52);
    assert_eq!(&payload[..], &buf[..]);
}

#[test]
fn ext_ack2() {
    #[cfg_attr(any(), rustfmt::skip)]
    let payload = &[
                                0x27, 0x00, 0x01, 0x00, 0x41, 0x74, 0x74, 0x72, 0x69, 0x62, 0x75, 0x74,
        0x65, 0x20, 0x66, 0x61, 0x69, 0x6c, 0x65, 0x64, 0x20, 0x70, 0x6f, 0x6c, 0x69, 0x63, 0x79, 0x20,
        0x76, 0x61, 0x6c, 0x69, 0x64, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x00, 0x00, 0x08, 0x00, 0x02, 0x00,
        0x20, 0x00, 0x00, 0x00, 0x14, 0x00, 0x04, 0x80, 0x08, 0x00, 0x07, 0x00, 0x0f, 0x00, 0x00, 0x00,
        0x08, 0x00, 0x01, 0x00, 0x0b, 0x00, 0x00, 0x00,
    ];

    dump_hex(payload);

    let req = NlmsgerrAttrs::new(payload);
    println!("{req:#?}");

    assert_eq!(req.get_msg(), Ok(c"Attribute failed policy validation"));
    assert_eq!(req.get_offset(), Ok(32));
    let pol = req.get_policy().unwrap();
    assert_eq!(pol.get_max_length(), Ok(15));
    assert_eq!(pol.get_type(), Ok(11));

    let mut buf = Vec::new();
    PushNlmsgerrAttrs::new(&mut buf)
        .push_msg(c"Attribute failed policy validation")
        .push_offset(32)
        .nested_policy()
        .push_max_length(15)
        .push_type(11);
    assert_eq!(&payload[..], &buf[..]);
}
