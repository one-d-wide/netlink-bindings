
# Operation "get"

## Do (request)

```rust
PushOpGetDoRequest::new(&mut vec)

  // conntrack l3+l4 protocol information, original direction
  .nested_tuple_orig()

    // conntrack l3 information
    .nested_tuple_ip()

      // ipv4 source address
      .push_ip_v4_src(val) // Ipv4Addr

      // ipv4 destination address
      .push_ip_v4_dst(val) // Ipv4Addr

      // ipv6 source address
      .push_ip_v6_src(val) // Ipv6Addr

      // ipv6 destination address
      .push_ip_v6_dst(val) // Ipv6Addr
    .end_nested()

    // conntrack l4 information
    .nested_tuple_proto()

      // l4 protocol number
      .push_proto_num(val) // u8

      // l4 source port
      .push_proto_src_port(val) // u16

      // l4 source port
      .push_proto_dst_port(val) // u16

      // l4 icmp id
      .push_proto_icmp_id(val) // u16
      .push_proto_icmp_type(val) // u8
      .push_proto_icmp_code(val) // u8

      // l4 icmp id
      .push_proto_icmpv6_id(val) // u16
      .push_proto_icmpv6_type(val) // u8
      .push_proto_icmpv6_code(val) // u8
    .end_nested()

    // conntrack zone id
    .push_tuple_zone(val) // u16
  .end_nested()

  // conntrack l3+l4 protocol information, reply direction
  .nested_tuple_reply()

    // conntrack l3 information
    .nested_tuple_ip()

      // ipv4 source address
      .push_ip_v4_src(val) // Ipv4Addr

      // ipv4 destination address
      .push_ip_v4_dst(val) // Ipv4Addr

      // ipv6 source address
      .push_ip_v6_src(val) // Ipv6Addr

      // ipv6 destination address
      .push_ip_v6_dst(val) // Ipv6Addr
    .end_nested()

    // conntrack l4 information
    .nested_tuple_proto()

      // l4 protocol number
      .push_proto_num(val) // u8

      // l4 source port
      .push_proto_src_port(val) // u16

      // l4 source port
      .push_proto_dst_port(val) // u16

      // l4 icmp id
      .push_proto_icmp_id(val) // u16
      .push_proto_icmp_type(val) // u8
      .push_proto_icmp_code(val) // u8

      // l4 icmp id
      .push_proto_icmpv6_id(val) // u16
      .push_proto_icmpv6_type(val) // u8
      .push_proto_icmpv6_code(val) // u8
    .end_nested()

    // conntrack zone id
    .push_tuple_zone(val) // u16
  .end_nested()

  // conntrack zone id
  .push_zone(val) // u16
  ;
```

```rust
let attrs = OpGetDoReply::new(buf);

{ // Nested TupleOrig

  // conntrack l3+l4 protocol information, original direction
  let attrs = attrs.get_tuple_orig();
  { // Nested TupleIp

    // conntrack l3 information
    let attrs = attrs.get_tuple_ip();

    // ipv4 source address
    attrs.get_ip_v4_src(); // Ipv4Addr

    // ipv4 destination address
    attrs.get_ip_v4_dst(); // Ipv4Addr

    // ipv6 source address
    attrs.get_ip_v6_src(); // Ipv6Addr

    // ipv6 destination address
    attrs.get_ip_v6_dst(); // Ipv6Addr
  }
  { // Nested TupleProto

    // conntrack l4 information
    let attrs = attrs.get_tuple_proto();

    // l4 protocol number
    attrs.get_proto_num(); // u8

    // l4 source port
    attrs.get_proto_src_port(); // u16

    // l4 source port
    attrs.get_proto_dst_port(); // u16

    // l4 icmp id
    attrs.get_proto_icmp_id(); // u16
    attrs.get_proto_icmp_type(); // u8
    attrs.get_proto_icmp_code(); // u8

    // l4 icmp id
    attrs.get_proto_icmpv6_id(); // u16
    attrs.get_proto_icmpv6_type(); // u8
    attrs.get_proto_icmpv6_code(); // u8
  }

  // conntrack zone id
  attrs.get_tuple_zone(); // u16
}
{ // Nested TupleReply

  // conntrack l3+l4 protocol information, reply direction
  let attrs = attrs.get_tuple_reply();
  { // Nested TupleIp

    // conntrack l3 information
    let attrs = attrs.get_tuple_ip();

    // ipv4 source address
    attrs.get_ip_v4_src(); // Ipv4Addr

    // ipv4 destination address
    attrs.get_ip_v4_dst(); // Ipv4Addr

    // ipv6 source address
    attrs.get_ip_v6_src(); // Ipv6Addr

    // ipv6 destination address
    attrs.get_ip_v6_dst(); // Ipv6Addr
  }
  { // Nested TupleProto

    // conntrack l4 information
    let attrs = attrs.get_tuple_proto();

    // l4 protocol number
    attrs.get_proto_num(); // u8

    // l4 source port
    attrs.get_proto_src_port(); // u16

    // l4 source port
    attrs.get_proto_dst_port(); // u16

    // l4 icmp id
    attrs.get_proto_icmp_id(); // u16
    attrs.get_proto_icmp_type(); // u8
    attrs.get_proto_icmp_code(); // u8

    // l4 icmp id
    attrs.get_proto_icmpv6_id(); // u16
    attrs.get_proto_icmpv6_type(); // u8
    attrs.get_proto_icmpv6_code(); // u8
  }

  // conntrack zone id
  attrs.get_tuple_zone(); // u16
}

// conntrack flag bits
// Associated type: "NfCtStatus" (1 bit per enumeration)
attrs.get_status(); // u32
{ // Nested Protoinfo
  let attrs = attrs.get_protoinfo();
  { // Nested ProtoinfoTcp

    // conntrack tcp state information
    let attrs = attrs.get_protoinfo_tcp();

    // tcp connection state
    // Associated type: "NfCtTcpState" (enum)
    attrs.get_tcp_state(); // u8

    // window scaling factor in original direction
    attrs.get_tcp_wscale_original(); // u8

    // window scaling factor in reply direction
    attrs.get_tcp_wscale_reply(); // u8
    attrs.get_tcp_flags_original(); // PushNfCtTcpFlagsMask
    attrs.get_tcp_flags_reply(); // PushNfCtTcpFlagsMask
  }
  { // Nested ProtoinfoDccp

    // conntrack dccp state information
    let attrs = attrs.get_protoinfo_dccp();

    // dccp connection state
    attrs.get_dccp_state(); // u8
    attrs.get_dccp_role(); // u8
    attrs.get_dccp_handshake_seq(); // u64
    attrs.get_dccp_pad(); // &[u8]
  }
  { // Nested ProtoinfoSctp

    // conntrack sctp state information
    let attrs = attrs.get_protoinfo_sctp();

    // sctp connection state
    // Associated type: "NfCtSctpState" (enum)
    attrs.get_sctp_state(); // u8
    attrs.get_vtag_original(); // u32
    attrs.get_vtag_reply(); // u32
  }
}
{ // Nested Help
  let attrs = attrs.get_help();

  // helper name
  attrs.get_help_name(); // &CStr
}
{ // Nested NatSrc
  let attrs = attrs.get_nat_src();
  attrs.get_nat_v4_minip(); // u32
  attrs.get_nat_v4_maxip(); // u32
  attrs.get_nat_v6_minip(); // &[u8]
  attrs.get_nat_v6_maxip(); // &[u8]
  { // Nested NatProto
    let attrs = attrs.get_nat_proto();
    attrs.get_nat_port_min(); // u16
    attrs.get_nat_port_max(); // u16
  }
}
{ // Nested NatDst
  let attrs = attrs.get_nat_dst();
  attrs.get_nat_v4_minip(); // u32
  attrs.get_nat_v4_maxip(); // u32
  attrs.get_nat_v6_minip(); // &[u8]
  attrs.get_nat_v6_maxip(); // &[u8]
  { // Nested NatProto
    let attrs = attrs.get_nat_proto();
    attrs.get_nat_port_min(); // u16
    attrs.get_nat_port_max(); // u16
  }
}
attrs.get_timeout(); // u32
attrs.get_mark(); // u32
{ // Nested CountersOrig
  let attrs = attrs.get_counters_orig();
  attrs.get_packets(); // u64
  attrs.get_bytes(); // u64
  attrs.get_packets_old(); // u32
  attrs.get_bytes_old(); // u32
  attrs.get_pad(); // &[u8]
}
{ // Nested CountersReply
  let attrs = attrs.get_counters_reply();
  attrs.get_packets(); // u64
  attrs.get_bytes(); // u64
  attrs.get_packets_old(); // u32
  attrs.get_bytes_old(); // u32
  attrs.get_pad(); // &[u8]
}
attrs.get_use(); // u32
attrs.get_id(); // u32
{ // Nested NatDst
  let attrs = attrs.get_nat_dst();
  attrs.get_nat_v4_minip(); // u32
  attrs.get_nat_v4_maxip(); // u32
  attrs.get_nat_v6_minip(); // &[u8]
  attrs.get_nat_v6_maxip(); // &[u8]
  { // Nested NatProto
    let attrs = attrs.get_nat_proto();
    attrs.get_nat_port_min(); // u16
    attrs.get_nat_port_max(); // u16
  }
}
{ // Nested TupleMaster
  let attrs = attrs.get_tuple_master();
  { // Nested TupleIp

    // conntrack l3 information
    let attrs = attrs.get_tuple_ip();

    // ipv4 source address
    attrs.get_ip_v4_src(); // Ipv4Addr

    // ipv4 destination address
    attrs.get_ip_v4_dst(); // Ipv4Addr

    // ipv6 source address
    attrs.get_ip_v6_src(); // Ipv6Addr

    // ipv6 destination address
    attrs.get_ip_v6_dst(); // Ipv6Addr
  }
  { // Nested TupleProto

    // conntrack l4 information
    let attrs = attrs.get_tuple_proto();

    // l4 protocol number
    attrs.get_proto_num(); // u8

    // l4 source port
    attrs.get_proto_src_port(); // u16

    // l4 source port
    attrs.get_proto_dst_port(); // u16

    // l4 icmp id
    attrs.get_proto_icmp_id(); // u16
    attrs.get_proto_icmp_type(); // u8
    attrs.get_proto_icmp_code(); // u8

    // l4 icmp id
    attrs.get_proto_icmpv6_id(); // u16
    attrs.get_proto_icmpv6_type(); // u8
    attrs.get_proto_icmpv6_code(); // u8
  }

  // conntrack zone id
  attrs.get_tuple_zone(); // u16
}
{ // Nested SeqAdjOrig
  let attrs = attrs.get_seq_adj_orig();
  attrs.get_correction_pos(); // u32
  attrs.get_offset_before(); // u32
  attrs.get_offset_after(); // u32
}
{ // Nested SeqAdjReply
  let attrs = attrs.get_seq_adj_reply();
  attrs.get_correction_pos(); // u32
  attrs.get_offset_before(); // u32
  attrs.get_offset_after(); // u32
}

// conntrack zone id
attrs.get_zone(); // u16
{ // Nested Secctx
  let attrs = attrs.get_secctx();
  attrs.get_secctx_name(); // &CStr
}
attrs.get_labels(); // &[u8]
{ // Nested Synproxy
  let attrs = attrs.get_synproxy();
  attrs.get_isn(); // u32
  attrs.get_its(); // u32
  attrs.get_tsoff(); // u32
}
```

### Do (reply)

```rust
PushOpGetDoReply::new(&mut vec)

  // conntrack l3+l4 protocol information, original direction
  .nested_tuple_orig()

    // conntrack l3 information
    .nested_tuple_ip()

      // ipv4 source address
      .push_ip_v4_src(val) // Ipv4Addr

      // ipv4 destination address
      .push_ip_v4_dst(val) // Ipv4Addr

      // ipv6 source address
      .push_ip_v6_src(val) // Ipv6Addr

      // ipv6 destination address
      .push_ip_v6_dst(val) // Ipv6Addr
    .end_nested()

    // conntrack l4 information
    .nested_tuple_proto()

      // l4 protocol number
      .push_proto_num(val) // u8

      // l4 source port
      .push_proto_src_port(val) // u16

      // l4 source port
      .push_proto_dst_port(val) // u16

      // l4 icmp id
      .push_proto_icmp_id(val) // u16
      .push_proto_icmp_type(val) // u8
      .push_proto_icmp_code(val) // u8

      // l4 icmp id
      .push_proto_icmpv6_id(val) // u16
      .push_proto_icmpv6_type(val) // u8
      .push_proto_icmpv6_code(val) // u8
    .end_nested()

    // conntrack zone id
    .push_tuple_zone(val) // u16
  .end_nested()

  // conntrack l3+l4 protocol information, reply direction
  .nested_tuple_reply()

    // conntrack l3 information
    .nested_tuple_ip()

      // ipv4 source address
      .push_ip_v4_src(val) // Ipv4Addr

      // ipv4 destination address
      .push_ip_v4_dst(val) // Ipv4Addr

      // ipv6 source address
      .push_ip_v6_src(val) // Ipv6Addr

      // ipv6 destination address
      .push_ip_v6_dst(val) // Ipv6Addr
    .end_nested()

    // conntrack l4 information
    .nested_tuple_proto()

      // l4 protocol number
      .push_proto_num(val) // u8

      // l4 source port
      .push_proto_src_port(val) // u16

      // l4 source port
      .push_proto_dst_port(val) // u16

      // l4 icmp id
      .push_proto_icmp_id(val) // u16
      .push_proto_icmp_type(val) // u8
      .push_proto_icmp_code(val) // u8

      // l4 icmp id
      .push_proto_icmpv6_id(val) // u16
      .push_proto_icmpv6_type(val) // u8
      .push_proto_icmpv6_code(val) // u8
    .end_nested()

    // conntrack zone id
    .push_tuple_zone(val) // u16
  .end_nested()

  // conntrack flag bits
  // Associated type: "NfCtStatus" (1 bit per enumeration)
  .push_status(val) // u32
  .nested_protoinfo()

    // conntrack tcp state information
    .nested_protoinfo_tcp()

      // tcp connection state
      // Associated type: "NfCtTcpState" (enum)
      .push_tcp_state(val) // u8

      // window scaling factor in original direction
      .push_tcp_wscale_original(val) // u8

      // window scaling factor in reply direction
      .push_tcp_wscale_reply(val) // u8
      .push_tcp_flags_original(val) // PushNfCtTcpFlagsMask
      .push_tcp_flags_reply(val) // PushNfCtTcpFlagsMask
    .end_nested()

    // conntrack dccp state information
    .nested_protoinfo_dccp()

      // dccp connection state
      .push_dccp_state(val) // u8
      .push_dccp_role(val) // u8
      .push_dccp_handshake_seq(val) // u64
      .push_dccp_pad(val) // &[u8]
    .end_nested()

    // conntrack sctp state information
    .nested_protoinfo_sctp()

      // sctp connection state
      // Associated type: "NfCtSctpState" (enum)
      .push_sctp_state(val) // u8
      .push_vtag_original(val) // u32
      .push_vtag_reply(val) // u32
    .end_nested()
  .end_nested()
  .nested_help()

    // helper name
    .push_help_name(val) // &CStr
    .push_help_name_bytes(val) // &[u8]
  .end_nested()
  .nested_nat_src()
    .push_nat_v4_minip(val) // u32
    .push_nat_v4_maxip(val) // u32
    .push_nat_v6_minip(val) // &[u8]
    .push_nat_v6_maxip(val) // &[u8]
    .nested_nat_proto()
      .push_nat_port_min(val) // u16
      .push_nat_port_max(val) // u16
    .end_nested()
  .end_nested()
  .nested_nat_dst()
    .push_nat_v4_minip(val) // u32
    .push_nat_v4_maxip(val) // u32
    .push_nat_v6_minip(val) // &[u8]
    .push_nat_v6_maxip(val) // &[u8]
    .nested_nat_proto()
      .push_nat_port_min(val) // u16
      .push_nat_port_max(val) // u16
    .end_nested()
  .end_nested()
  .push_timeout(val) // u32
  .push_mark(val) // u32
  .nested_counters_orig()
    .push_packets(val) // u64
    .push_bytes(val) // u64
    .push_packets_old(val) // u32
    .push_bytes_old(val) // u32
    .push_pad(val) // &[u8]
  .end_nested()
  .nested_counters_reply()
    .push_packets(val) // u64
    .push_bytes(val) // u64
    .push_packets_old(val) // u32
    .push_bytes_old(val) // u32
    .push_pad(val) // &[u8]
  .end_nested()
  .push_use(val) // u32
  .push_id(val) // u32
  .nested_nat_dst()
    .push_nat_v4_minip(val) // u32
    .push_nat_v4_maxip(val) // u32
    .push_nat_v6_minip(val) // &[u8]
    .push_nat_v6_maxip(val) // &[u8]
    .nested_nat_proto()
      .push_nat_port_min(val) // u16
      .push_nat_port_max(val) // u16
    .end_nested()
  .end_nested()
  .nested_tuple_master()

    // conntrack l3 information
    .nested_tuple_ip()

      // ipv4 source address
      .push_ip_v4_src(val) // Ipv4Addr

      // ipv4 destination address
      .push_ip_v4_dst(val) // Ipv4Addr

      // ipv6 source address
      .push_ip_v6_src(val) // Ipv6Addr

      // ipv6 destination address
      .push_ip_v6_dst(val) // Ipv6Addr
    .end_nested()

    // conntrack l4 information
    .nested_tuple_proto()

      // l4 protocol number
      .push_proto_num(val) // u8

      // l4 source port
      .push_proto_src_port(val) // u16

      // l4 source port
      .push_proto_dst_port(val) // u16

      // l4 icmp id
      .push_proto_icmp_id(val) // u16
      .push_proto_icmp_type(val) // u8
      .push_proto_icmp_code(val) // u8

      // l4 icmp id
      .push_proto_icmpv6_id(val) // u16
      .push_proto_icmpv6_type(val) // u8
      .push_proto_icmpv6_code(val) // u8
    .end_nested()

    // conntrack zone id
    .push_tuple_zone(val) // u16
  .end_nested()
  .nested_seq_adj_orig()
    .push_correction_pos(val) // u32
    .push_offset_before(val) // u32
    .push_offset_after(val) // u32
  .end_nested()
  .nested_seq_adj_reply()
    .push_correction_pos(val) // u32
    .push_offset_before(val) // u32
    .push_offset_after(val) // u32
  .end_nested()

  // conntrack zone id
  .push_zone(val) // u16
  .nested_secctx()
    .push_secctx_name(val) // &CStr
    .push_secctx_name_bytes(val) // &[u8]
  .end_nested()
  .push_labels(val) // &[u8]
  .nested_synproxy()
    .push_isn(val) // u32
    .push_its(val) // u32
    .push_tsoff(val) // u32
  .end_nested()
  ;
```

```rust
let attrs = OpGetDoReply::new(buf);

{ // Nested TupleOrig

  // conntrack l3+l4 protocol information, original direction
  let attrs = attrs.get_tuple_orig();
  { // Nested TupleIp

    // conntrack l3 information
    let attrs = attrs.get_tuple_ip();

    // ipv4 source address
    attrs.get_ip_v4_src(); // Ipv4Addr

    // ipv4 destination address
    attrs.get_ip_v4_dst(); // Ipv4Addr

    // ipv6 source address
    attrs.get_ip_v6_src(); // Ipv6Addr

    // ipv6 destination address
    attrs.get_ip_v6_dst(); // Ipv6Addr
  }
  { // Nested TupleProto

    // conntrack l4 information
    let attrs = attrs.get_tuple_proto();

    // l4 protocol number
    attrs.get_proto_num(); // u8

    // l4 source port
    attrs.get_proto_src_port(); // u16

    // l4 source port
    attrs.get_proto_dst_port(); // u16

    // l4 icmp id
    attrs.get_proto_icmp_id(); // u16
    attrs.get_proto_icmp_type(); // u8
    attrs.get_proto_icmp_code(); // u8

    // l4 icmp id
    attrs.get_proto_icmpv6_id(); // u16
    attrs.get_proto_icmpv6_type(); // u8
    attrs.get_proto_icmpv6_code(); // u8
  }

  // conntrack zone id
  attrs.get_tuple_zone(); // u16
}
{ // Nested TupleReply

  // conntrack l3+l4 protocol information, reply direction
  let attrs = attrs.get_tuple_reply();
  { // Nested TupleIp

    // conntrack l3 information
    let attrs = attrs.get_tuple_ip();

    // ipv4 source address
    attrs.get_ip_v4_src(); // Ipv4Addr

    // ipv4 destination address
    attrs.get_ip_v4_dst(); // Ipv4Addr

    // ipv6 source address
    attrs.get_ip_v6_src(); // Ipv6Addr

    // ipv6 destination address
    attrs.get_ip_v6_dst(); // Ipv6Addr
  }
  { // Nested TupleProto

    // conntrack l4 information
    let attrs = attrs.get_tuple_proto();

    // l4 protocol number
    attrs.get_proto_num(); // u8

    // l4 source port
    attrs.get_proto_src_port(); // u16

    // l4 source port
    attrs.get_proto_dst_port(); // u16

    // l4 icmp id
    attrs.get_proto_icmp_id(); // u16
    attrs.get_proto_icmp_type(); // u8
    attrs.get_proto_icmp_code(); // u8

    // l4 icmp id
    attrs.get_proto_icmpv6_id(); // u16
    attrs.get_proto_icmpv6_type(); // u8
    attrs.get_proto_icmpv6_code(); // u8
  }

  // conntrack zone id
  attrs.get_tuple_zone(); // u16
}

// conntrack flag bits
// Associated type: "NfCtStatus" (1 bit per enumeration)
attrs.get_status(); // u32
{ // Nested Protoinfo
  let attrs = attrs.get_protoinfo();
  { // Nested ProtoinfoTcp

    // conntrack tcp state information
    let attrs = attrs.get_protoinfo_tcp();

    // tcp connection state
    // Associated type: "NfCtTcpState" (enum)
    attrs.get_tcp_state(); // u8

    // window scaling factor in original direction
    attrs.get_tcp_wscale_original(); // u8

    // window scaling factor in reply direction
    attrs.get_tcp_wscale_reply(); // u8
    attrs.get_tcp_flags_original(); // PushNfCtTcpFlagsMask
    attrs.get_tcp_flags_reply(); // PushNfCtTcpFlagsMask
  }
  { // Nested ProtoinfoDccp

    // conntrack dccp state information
    let attrs = attrs.get_protoinfo_dccp();

    // dccp connection state
    attrs.get_dccp_state(); // u8
    attrs.get_dccp_role(); // u8
    attrs.get_dccp_handshake_seq(); // u64
    attrs.get_dccp_pad(); // &[u8]
  }
  { // Nested ProtoinfoSctp

    // conntrack sctp state information
    let attrs = attrs.get_protoinfo_sctp();

    // sctp connection state
    // Associated type: "NfCtSctpState" (enum)
    attrs.get_sctp_state(); // u8
    attrs.get_vtag_original(); // u32
    attrs.get_vtag_reply(); // u32
  }
}
{ // Nested Help
  let attrs = attrs.get_help();

  // helper name
  attrs.get_help_name(); // &CStr
}
{ // Nested NatSrc
  let attrs = attrs.get_nat_src();
  attrs.get_nat_v4_minip(); // u32
  attrs.get_nat_v4_maxip(); // u32
  attrs.get_nat_v6_minip(); // &[u8]
  attrs.get_nat_v6_maxip(); // &[u8]
  { // Nested NatProto
    let attrs = attrs.get_nat_proto();
    attrs.get_nat_port_min(); // u16
    attrs.get_nat_port_max(); // u16
  }
}
{ // Nested NatDst
  let attrs = attrs.get_nat_dst();
  attrs.get_nat_v4_minip(); // u32
  attrs.get_nat_v4_maxip(); // u32
  attrs.get_nat_v6_minip(); // &[u8]
  attrs.get_nat_v6_maxip(); // &[u8]
  { // Nested NatProto
    let attrs = attrs.get_nat_proto();
    attrs.get_nat_port_min(); // u16
    attrs.get_nat_port_max(); // u16
  }
}
attrs.get_timeout(); // u32
attrs.get_mark(); // u32
{ // Nested CountersOrig
  let attrs = attrs.get_counters_orig();
  attrs.get_packets(); // u64
  attrs.get_bytes(); // u64
  attrs.get_packets_old(); // u32
  attrs.get_bytes_old(); // u32
  attrs.get_pad(); // &[u8]
}
{ // Nested CountersReply
  let attrs = attrs.get_counters_reply();
  attrs.get_packets(); // u64
  attrs.get_bytes(); // u64
  attrs.get_packets_old(); // u32
  attrs.get_bytes_old(); // u32
  attrs.get_pad(); // &[u8]
}
attrs.get_use(); // u32
attrs.get_id(); // u32
{ // Nested NatDst
  let attrs = attrs.get_nat_dst();
  attrs.get_nat_v4_minip(); // u32
  attrs.get_nat_v4_maxip(); // u32
  attrs.get_nat_v6_minip(); // &[u8]
  attrs.get_nat_v6_maxip(); // &[u8]
  { // Nested NatProto
    let attrs = attrs.get_nat_proto();
    attrs.get_nat_port_min(); // u16
    attrs.get_nat_port_max(); // u16
  }
}
{ // Nested TupleMaster
  let attrs = attrs.get_tuple_master();
  { // Nested TupleIp

    // conntrack l3 information
    let attrs = attrs.get_tuple_ip();

    // ipv4 source address
    attrs.get_ip_v4_src(); // Ipv4Addr

    // ipv4 destination address
    attrs.get_ip_v4_dst(); // Ipv4Addr

    // ipv6 source address
    attrs.get_ip_v6_src(); // Ipv6Addr

    // ipv6 destination address
    attrs.get_ip_v6_dst(); // Ipv6Addr
  }
  { // Nested TupleProto

    // conntrack l4 information
    let attrs = attrs.get_tuple_proto();

    // l4 protocol number
    attrs.get_proto_num(); // u8

    // l4 source port
    attrs.get_proto_src_port(); // u16

    // l4 source port
    attrs.get_proto_dst_port(); // u16

    // l4 icmp id
    attrs.get_proto_icmp_id(); // u16
    attrs.get_proto_icmp_type(); // u8
    attrs.get_proto_icmp_code(); // u8

    // l4 icmp id
    attrs.get_proto_icmpv6_id(); // u16
    attrs.get_proto_icmpv6_type(); // u8
    attrs.get_proto_icmpv6_code(); // u8
  }

  // conntrack zone id
  attrs.get_tuple_zone(); // u16
}
{ // Nested SeqAdjOrig
  let attrs = attrs.get_seq_adj_orig();
  attrs.get_correction_pos(); // u32
  attrs.get_offset_before(); // u32
  attrs.get_offset_after(); // u32
}
{ // Nested SeqAdjReply
  let attrs = attrs.get_seq_adj_reply();
  attrs.get_correction_pos(); // u32
  attrs.get_offset_before(); // u32
  attrs.get_offset_after(); // u32
}

// conntrack zone id
attrs.get_zone(); // u16
{ // Nested Secctx
  let attrs = attrs.get_secctx();
  attrs.get_secctx_name(); // &CStr
}
attrs.get_labels(); // &[u8]
{ // Nested Synproxy
  let attrs = attrs.get_synproxy();
  attrs.get_isn(); // u32
  attrs.get_its(); // u32
  attrs.get_tsoff(); // u32
}
```

## Low-level decoding

### Do (request)

```rust
let iter = OpGetDoRequest::new(buf);
for attr in iter {
  match attr {

    // conntrack l3+l4 protocol information, original direction
    TupleOrig(iter) => {
      for attr in iter {
        match attr {

          // conntrack l3 information
          TupleIp(iter) => {
            for attr in iter {
              match attr {

                // ipv4 source address
                IpV4Src(val) => {}, // Ipv4Addr

                // ipv4 destination address
                IpV4Dst(val) => {}, // Ipv4Addr

                // ipv6 source address
                IpV6Src(val) => {}, // Ipv6Addr

                // ipv6 destination address
                IpV6Dst(val) => {}, // Ipv6Addr
              }
            }
          },

          // conntrack l4 information
          TupleProto(iter) => {
            for attr in iter {
              match attr {

                // l4 protocol number
                ProtoNum(val) => {}, // u8

                // l4 source port
                ProtoSrcPort(val) => {}, // u16

                // l4 source port
                ProtoDstPort(val) => {}, // u16

                // l4 icmp id
                ProtoIcmpId(val) => {}, // u16
                ProtoIcmpType(val) => {}, // u8
                ProtoIcmpCode(val) => {}, // u8

                // l4 icmp id
                ProtoIcmpv6Id(val) => {}, // u16
                ProtoIcmpv6Type(val) => {}, // u8
                ProtoIcmpv6Code(val) => {}, // u8
              }
            }
          },

          // conntrack zone id
          TupleZone(val) => {}, // u16
        }
      }
    },

    // conntrack l3+l4 protocol information, reply direction
    TupleReply(iter) => {
      for attr in iter {
        match attr {

          // conntrack l3 information
          TupleIp(iter) => {
            for attr in iter {
              match attr {

                // ipv4 source address
                IpV4Src(val) => {}, // Ipv4Addr

                // ipv4 destination address
                IpV4Dst(val) => {}, // Ipv4Addr

                // ipv6 source address
                IpV6Src(val) => {}, // Ipv6Addr

                // ipv6 destination address
                IpV6Dst(val) => {}, // Ipv6Addr
              }
            }
          },

          // conntrack l4 information
          TupleProto(iter) => {
            for attr in iter {
              match attr {

                // l4 protocol number
                ProtoNum(val) => {}, // u8

                // l4 source port
                ProtoSrcPort(val) => {}, // u16

                // l4 source port
                ProtoDstPort(val) => {}, // u16

                // l4 icmp id
                ProtoIcmpId(val) => {}, // u16
                ProtoIcmpType(val) => {}, // u8
                ProtoIcmpCode(val) => {}, // u8

                // l4 icmp id
                ProtoIcmpv6Id(val) => {}, // u16
                ProtoIcmpv6Type(val) => {}, // u8
                ProtoIcmpv6Code(val) => {}, // u8
              }
            }
          },

          // conntrack zone id
          TupleZone(val) => {}, // u16
        }
      }
    },

    // conntrack zone id
    Zone(val) => {}, // u16
  }
}
```

### Do (reply)

```rust
let iter = OpGetDoReply::new(buf);
for attr in iter {
  match attr {

    // conntrack l3+l4 protocol information, original direction
    TupleOrig(iter) => {
      for attr in iter {
        match attr {

          // conntrack l3 information
          TupleIp(iter) => {
            for attr in iter {
              match attr {

                // ipv4 source address
                IpV4Src(val) => {}, // Ipv4Addr

                // ipv4 destination address
                IpV4Dst(val) => {}, // Ipv4Addr

                // ipv6 source address
                IpV6Src(val) => {}, // Ipv6Addr

                // ipv6 destination address
                IpV6Dst(val) => {}, // Ipv6Addr
              }
            }
          },

          // conntrack l4 information
          TupleProto(iter) => {
            for attr in iter {
              match attr {

                // l4 protocol number
                ProtoNum(val) => {}, // u8

                // l4 source port
                ProtoSrcPort(val) => {}, // u16

                // l4 source port
                ProtoDstPort(val) => {}, // u16

                // l4 icmp id
                ProtoIcmpId(val) => {}, // u16
                ProtoIcmpType(val) => {}, // u8
                ProtoIcmpCode(val) => {}, // u8

                // l4 icmp id
                ProtoIcmpv6Id(val) => {}, // u16
                ProtoIcmpv6Type(val) => {}, // u8
                ProtoIcmpv6Code(val) => {}, // u8
              }
            }
          },

          // conntrack zone id
          TupleZone(val) => {}, // u16
        }
      }
    },

    // conntrack l3+l4 protocol information, reply direction
    TupleReply(iter) => {
      for attr in iter {
        match attr {

          // conntrack l3 information
          TupleIp(iter) => {
            for attr in iter {
              match attr {

                // ipv4 source address
                IpV4Src(val) => {}, // Ipv4Addr

                // ipv4 destination address
                IpV4Dst(val) => {}, // Ipv4Addr

                // ipv6 source address
                IpV6Src(val) => {}, // Ipv6Addr

                // ipv6 destination address
                IpV6Dst(val) => {}, // Ipv6Addr
              }
            }
          },

          // conntrack l4 information
          TupleProto(iter) => {
            for attr in iter {
              match attr {

                // l4 protocol number
                ProtoNum(val) => {}, // u8

                // l4 source port
                ProtoSrcPort(val) => {}, // u16

                // l4 source port
                ProtoDstPort(val) => {}, // u16

                // l4 icmp id
                ProtoIcmpId(val) => {}, // u16
                ProtoIcmpType(val) => {}, // u8
                ProtoIcmpCode(val) => {}, // u8

                // l4 icmp id
                ProtoIcmpv6Id(val) => {}, // u16
                ProtoIcmpv6Type(val) => {}, // u8
                ProtoIcmpv6Code(val) => {}, // u8
              }
            }
          },

          // conntrack zone id
          TupleZone(val) => {}, // u16
        }
      }
    },

    // conntrack flag bits
    // Associated type: "NfCtStatus" (1 bit per enumeration)
    Status(val) => {}, // u32
    Protoinfo(iter) => {
      for attr in iter {
        match attr {

          // conntrack tcp state information
          ProtoinfoTcp(iter) => {
            for attr in iter {
              match attr {

                // tcp connection state
                // Associated type: "NfCtTcpState" (enum)
                TcpState(val) => {}, // u8

                // window scaling factor in original direction
                TcpWscaleOriginal(val) => {}, // u8

                // window scaling factor in reply direction
                TcpWscaleReply(val) => {}, // u8
                TcpFlagsOriginal(val) => {}, // PushNfCtTcpFlagsMask
                TcpFlagsReply(val) => {}, // PushNfCtTcpFlagsMask
              }
            }
          },

          // conntrack dccp state information
          ProtoinfoDccp(iter) => {
            for attr in iter {
              match attr {

                // dccp connection state
                DccpState(val) => {}, // u8
                DccpRole(val) => {}, // u8
                DccpHandshakeSeq(val) => {}, // u64
                DccpPad(val) => {}, // &[u8]
              }
            }
          },

          // conntrack sctp state information
          ProtoinfoSctp(iter) => {
            for attr in iter {
              match attr {

                // sctp connection state
                // Associated type: "NfCtSctpState" (enum)
                SctpState(val) => {}, // u8
                VtagOriginal(val) => {}, // u32
                VtagReply(val) => {}, // u32
              }
            }
          },
        }
      }
    },
    Help(iter) => {
      for attr in iter {
        match attr {

          // helper name
          HelpName(val) => {}, // &CStr
        }
      }
    },
    NatSrc(iter) => {
      for attr in iter {
        match attr {
          NatV4Minip(val) => {}, // u32
          NatV4Maxip(val) => {}, // u32
          NatV6Minip(val) => {}, // &[u8]
          NatV6Maxip(val) => {}, // &[u8]
          NatProto(iter) => {
            for attr in iter {
              match attr {
                NatPortMin(val) => {}, // u16
                NatPortMax(val) => {}, // u16
              }
            }
          },
        }
      }
    },
    NatDst(iter) => {
      for attr in iter {
        match attr {
          NatV4Minip(val) => {}, // u32
          NatV4Maxip(val) => {}, // u32
          NatV6Minip(val) => {}, // &[u8]
          NatV6Maxip(val) => {}, // &[u8]
          NatProto(iter) => {
            for attr in iter {
              match attr {
                NatPortMin(val) => {}, // u16
                NatPortMax(val) => {}, // u16
              }
            }
          },
        }
      }
    },
    Timeout(val) => {}, // u32
    Mark(val) => {}, // u32
    CountersOrig(iter) => {
      for attr in iter {
        match attr {
          Packets(val) => {}, // u64
          Bytes(val) => {}, // u64
          PacketsOld(val) => {}, // u32
          BytesOld(val) => {}, // u32
          Pad(val) => {}, // &[u8]
        }
      }
    },
    CountersReply(iter) => {
      for attr in iter {
        match attr {
          Packets(val) => {}, // u64
          Bytes(val) => {}, // u64
          PacketsOld(val) => {}, // u32
          BytesOld(val) => {}, // u32
          Pad(val) => {}, // &[u8]
        }
      }
    },
    Use(val) => {}, // u32
    Id(val) => {}, // u32
    NatDst(iter) => {
      for attr in iter {
        match attr {
          NatV4Minip(val) => {}, // u32
          NatV4Maxip(val) => {}, // u32
          NatV6Minip(val) => {}, // &[u8]
          NatV6Maxip(val) => {}, // &[u8]
          NatProto(iter) => {
            for attr in iter {
              match attr {
                NatPortMin(val) => {}, // u16
                NatPortMax(val) => {}, // u16
              }
            }
          },
        }
      }
    },
    TupleMaster(iter) => {
      for attr in iter {
        match attr {

          // conntrack l3 information
          TupleIp(iter) => {
            for attr in iter {
              match attr {

                // ipv4 source address
                IpV4Src(val) => {}, // Ipv4Addr

                // ipv4 destination address
                IpV4Dst(val) => {}, // Ipv4Addr

                // ipv6 source address
                IpV6Src(val) => {}, // Ipv6Addr

                // ipv6 destination address
                IpV6Dst(val) => {}, // Ipv6Addr
              }
            }
          },

          // conntrack l4 information
          TupleProto(iter) => {
            for attr in iter {
              match attr {

                // l4 protocol number
                ProtoNum(val) => {}, // u8

                // l4 source port
                ProtoSrcPort(val) => {}, // u16

                // l4 source port
                ProtoDstPort(val) => {}, // u16

                // l4 icmp id
                ProtoIcmpId(val) => {}, // u16
                ProtoIcmpType(val) => {}, // u8
                ProtoIcmpCode(val) => {}, // u8

                // l4 icmp id
                ProtoIcmpv6Id(val) => {}, // u16
                ProtoIcmpv6Type(val) => {}, // u8
                ProtoIcmpv6Code(val) => {}, // u8
              }
            }
          },

          // conntrack zone id
          TupleZone(val) => {}, // u16
        }
      }
    },
    SeqAdjOrig(iter) => {
      for attr in iter {
        match attr {
          CorrectionPos(val) => {}, // u32
          OffsetBefore(val) => {}, // u32
          OffsetAfter(val) => {}, // u32
        }
      }
    },
    SeqAdjReply(iter) => {
      for attr in iter {
        match attr {
          CorrectionPos(val) => {}, // u32
          OffsetBefore(val) => {}, // u32
          OffsetAfter(val) => {}, // u32
        }
      }
    },

    // conntrack zone id
    Zone(val) => {}, // u16
    Secctx(iter) => {
      for attr in iter {
        match attr {
          SecctxName(val) => {}, // &CStr
        }
      }
    },
    Labels(val) => {}, // &[u8]
    Synproxy(iter) => {
      for attr in iter {
        match attr {
          Isn(val) => {}, // u32
          Its(val) => {}, // u32
          Tsoff(val) => {}, // u32
        }
      }
    },
  }
}
```

## Dump (request)

```rust
PushOpGetDumpRequest::new(&mut vec)
  .push_mark(val) // u32
  .nested_filter()

    // conntrack l3 information
    .nested_tuple_ip()

      // ipv4 source address
      .push_ip_v4_src(val) // Ipv4Addr

      // ipv4 destination address
      .push_ip_v4_dst(val) // Ipv4Addr

      // ipv6 source address
      .push_ip_v6_src(val) // Ipv6Addr

      // ipv6 destination address
      .push_ip_v6_dst(val) // Ipv6Addr
    .end_nested()

    // conntrack l4 information
    .nested_tuple_proto()

      // l4 protocol number
      .push_proto_num(val) // u8

      // l4 source port
      .push_proto_src_port(val) // u16

      // l4 source port
      .push_proto_dst_port(val) // u16

      // l4 icmp id
      .push_proto_icmp_id(val) // u16
      .push_proto_icmp_type(val) // u8
      .push_proto_icmp_code(val) // u8

      // l4 icmp id
      .push_proto_icmpv6_id(val) // u16
      .push_proto_icmpv6_type(val) // u8
      .push_proto_icmpv6_code(val) // u8
    .end_nested()

    // conntrack zone id
    .push_tuple_zone(val) // u16
  .end_nested()

  // conntrack flag bits
  // Associated type: "NfCtStatus" (1 bit per enumeration)
  .push_status(val) // u32

  // conntrack zone id
  .push_zone(val) // u16
  ;
```

```rust
let attrs = OpGetDumpReply::new(buf);

{ // Nested TupleOrig

  // conntrack l3+l4 protocol information, original direction
  let attrs = attrs.get_tuple_orig();
  { // Nested TupleIp

    // conntrack l3 information
    let attrs = attrs.get_tuple_ip();

    // ipv4 source address
    attrs.get_ip_v4_src(); // Ipv4Addr

    // ipv4 destination address
    attrs.get_ip_v4_dst(); // Ipv4Addr

    // ipv6 source address
    attrs.get_ip_v6_src(); // Ipv6Addr

    // ipv6 destination address
    attrs.get_ip_v6_dst(); // Ipv6Addr
  }
  { // Nested TupleProto

    // conntrack l4 information
    let attrs = attrs.get_tuple_proto();

    // l4 protocol number
    attrs.get_proto_num(); // u8

    // l4 source port
    attrs.get_proto_src_port(); // u16

    // l4 source port
    attrs.get_proto_dst_port(); // u16

    // l4 icmp id
    attrs.get_proto_icmp_id(); // u16
    attrs.get_proto_icmp_type(); // u8
    attrs.get_proto_icmp_code(); // u8

    // l4 icmp id
    attrs.get_proto_icmpv6_id(); // u16
    attrs.get_proto_icmpv6_type(); // u8
    attrs.get_proto_icmpv6_code(); // u8
  }

  // conntrack zone id
  attrs.get_tuple_zone(); // u16
}
{ // Nested TupleReply

  // conntrack l3+l4 protocol information, reply direction
  let attrs = attrs.get_tuple_reply();
  { // Nested TupleIp

    // conntrack l3 information
    let attrs = attrs.get_tuple_ip();

    // ipv4 source address
    attrs.get_ip_v4_src(); // Ipv4Addr

    // ipv4 destination address
    attrs.get_ip_v4_dst(); // Ipv4Addr

    // ipv6 source address
    attrs.get_ip_v6_src(); // Ipv6Addr

    // ipv6 destination address
    attrs.get_ip_v6_dst(); // Ipv6Addr
  }
  { // Nested TupleProto

    // conntrack l4 information
    let attrs = attrs.get_tuple_proto();

    // l4 protocol number
    attrs.get_proto_num(); // u8

    // l4 source port
    attrs.get_proto_src_port(); // u16

    // l4 source port
    attrs.get_proto_dst_port(); // u16

    // l4 icmp id
    attrs.get_proto_icmp_id(); // u16
    attrs.get_proto_icmp_type(); // u8
    attrs.get_proto_icmp_code(); // u8

    // l4 icmp id
    attrs.get_proto_icmpv6_id(); // u16
    attrs.get_proto_icmpv6_type(); // u8
    attrs.get_proto_icmpv6_code(); // u8
  }

  // conntrack zone id
  attrs.get_tuple_zone(); // u16
}

// conntrack flag bits
// Associated type: "NfCtStatus" (1 bit per enumeration)
attrs.get_status(); // u32
{ // Nested Protoinfo
  let attrs = attrs.get_protoinfo();
  { // Nested ProtoinfoTcp

    // conntrack tcp state information
    let attrs = attrs.get_protoinfo_tcp();

    // tcp connection state
    // Associated type: "NfCtTcpState" (enum)
    attrs.get_tcp_state(); // u8

    // window scaling factor in original direction
    attrs.get_tcp_wscale_original(); // u8

    // window scaling factor in reply direction
    attrs.get_tcp_wscale_reply(); // u8
    attrs.get_tcp_flags_original(); // PushNfCtTcpFlagsMask
    attrs.get_tcp_flags_reply(); // PushNfCtTcpFlagsMask
  }
  { // Nested ProtoinfoDccp

    // conntrack dccp state information
    let attrs = attrs.get_protoinfo_dccp();

    // dccp connection state
    attrs.get_dccp_state(); // u8
    attrs.get_dccp_role(); // u8
    attrs.get_dccp_handshake_seq(); // u64
    attrs.get_dccp_pad(); // &[u8]
  }
  { // Nested ProtoinfoSctp

    // conntrack sctp state information
    let attrs = attrs.get_protoinfo_sctp();

    // sctp connection state
    // Associated type: "NfCtSctpState" (enum)
    attrs.get_sctp_state(); // u8
    attrs.get_vtag_original(); // u32
    attrs.get_vtag_reply(); // u32
  }
}
{ // Nested Help
  let attrs = attrs.get_help();

  // helper name
  attrs.get_help_name(); // &CStr
}
{ // Nested NatSrc
  let attrs = attrs.get_nat_src();
  attrs.get_nat_v4_minip(); // u32
  attrs.get_nat_v4_maxip(); // u32
  attrs.get_nat_v6_minip(); // &[u8]
  attrs.get_nat_v6_maxip(); // &[u8]
  { // Nested NatProto
    let attrs = attrs.get_nat_proto();
    attrs.get_nat_port_min(); // u16
    attrs.get_nat_port_max(); // u16
  }
}
{ // Nested NatDst
  let attrs = attrs.get_nat_dst();
  attrs.get_nat_v4_minip(); // u32
  attrs.get_nat_v4_maxip(); // u32
  attrs.get_nat_v6_minip(); // &[u8]
  attrs.get_nat_v6_maxip(); // &[u8]
  { // Nested NatProto
    let attrs = attrs.get_nat_proto();
    attrs.get_nat_port_min(); // u16
    attrs.get_nat_port_max(); // u16
  }
}
attrs.get_timeout(); // u32
attrs.get_mark(); // u32
{ // Nested CountersOrig
  let attrs = attrs.get_counters_orig();
  attrs.get_packets(); // u64
  attrs.get_bytes(); // u64
  attrs.get_packets_old(); // u32
  attrs.get_bytes_old(); // u32
  attrs.get_pad(); // &[u8]
}
{ // Nested CountersReply
  let attrs = attrs.get_counters_reply();
  attrs.get_packets(); // u64
  attrs.get_bytes(); // u64
  attrs.get_packets_old(); // u32
  attrs.get_bytes_old(); // u32
  attrs.get_pad(); // &[u8]
}
attrs.get_use(); // u32
attrs.get_id(); // u32
{ // Nested NatDst
  let attrs = attrs.get_nat_dst();
  attrs.get_nat_v4_minip(); // u32
  attrs.get_nat_v4_maxip(); // u32
  attrs.get_nat_v6_minip(); // &[u8]
  attrs.get_nat_v6_maxip(); // &[u8]
  { // Nested NatProto
    let attrs = attrs.get_nat_proto();
    attrs.get_nat_port_min(); // u16
    attrs.get_nat_port_max(); // u16
  }
}
{ // Nested TupleMaster
  let attrs = attrs.get_tuple_master();
  { // Nested TupleIp

    // conntrack l3 information
    let attrs = attrs.get_tuple_ip();

    // ipv4 source address
    attrs.get_ip_v4_src(); // Ipv4Addr

    // ipv4 destination address
    attrs.get_ip_v4_dst(); // Ipv4Addr

    // ipv6 source address
    attrs.get_ip_v6_src(); // Ipv6Addr

    // ipv6 destination address
    attrs.get_ip_v6_dst(); // Ipv6Addr
  }
  { // Nested TupleProto

    // conntrack l4 information
    let attrs = attrs.get_tuple_proto();

    // l4 protocol number
    attrs.get_proto_num(); // u8

    // l4 source port
    attrs.get_proto_src_port(); // u16

    // l4 source port
    attrs.get_proto_dst_port(); // u16

    // l4 icmp id
    attrs.get_proto_icmp_id(); // u16
    attrs.get_proto_icmp_type(); // u8
    attrs.get_proto_icmp_code(); // u8

    // l4 icmp id
    attrs.get_proto_icmpv6_id(); // u16
    attrs.get_proto_icmpv6_type(); // u8
    attrs.get_proto_icmpv6_code(); // u8
  }

  // conntrack zone id
  attrs.get_tuple_zone(); // u16
}
{ // Nested SeqAdjOrig
  let attrs = attrs.get_seq_adj_orig();
  attrs.get_correction_pos(); // u32
  attrs.get_offset_before(); // u32
  attrs.get_offset_after(); // u32
}
{ // Nested SeqAdjReply
  let attrs = attrs.get_seq_adj_reply();
  attrs.get_correction_pos(); // u32
  attrs.get_offset_before(); // u32
  attrs.get_offset_after(); // u32
}

// conntrack zone id
attrs.get_zone(); // u16
{ // Nested Secctx
  let attrs = attrs.get_secctx();
  attrs.get_secctx_name(); // &CStr
}
attrs.get_labels(); // &[u8]
{ // Nested Synproxy
  let attrs = attrs.get_synproxy();
  attrs.get_isn(); // u32
  attrs.get_its(); // u32
  attrs.get_tsoff(); // u32
}
```

### Dump (reply)

```rust
PushOpGetDumpReply::new(&mut vec)

  // conntrack l3+l4 protocol information, original direction
  .nested_tuple_orig()

    // conntrack l3 information
    .nested_tuple_ip()

      // ipv4 source address
      .push_ip_v4_src(val) // Ipv4Addr

      // ipv4 destination address
      .push_ip_v4_dst(val) // Ipv4Addr

      // ipv6 source address
      .push_ip_v6_src(val) // Ipv6Addr

      // ipv6 destination address
      .push_ip_v6_dst(val) // Ipv6Addr
    .end_nested()

    // conntrack l4 information
    .nested_tuple_proto()

      // l4 protocol number
      .push_proto_num(val) // u8

      // l4 source port
      .push_proto_src_port(val) // u16

      // l4 source port
      .push_proto_dst_port(val) // u16

      // l4 icmp id
      .push_proto_icmp_id(val) // u16
      .push_proto_icmp_type(val) // u8
      .push_proto_icmp_code(val) // u8

      // l4 icmp id
      .push_proto_icmpv6_id(val) // u16
      .push_proto_icmpv6_type(val) // u8
      .push_proto_icmpv6_code(val) // u8
    .end_nested()

    // conntrack zone id
    .push_tuple_zone(val) // u16
  .end_nested()

  // conntrack l3+l4 protocol information, reply direction
  .nested_tuple_reply()

    // conntrack l3 information
    .nested_tuple_ip()

      // ipv4 source address
      .push_ip_v4_src(val) // Ipv4Addr

      // ipv4 destination address
      .push_ip_v4_dst(val) // Ipv4Addr

      // ipv6 source address
      .push_ip_v6_src(val) // Ipv6Addr

      // ipv6 destination address
      .push_ip_v6_dst(val) // Ipv6Addr
    .end_nested()

    // conntrack l4 information
    .nested_tuple_proto()

      // l4 protocol number
      .push_proto_num(val) // u8

      // l4 source port
      .push_proto_src_port(val) // u16

      // l4 source port
      .push_proto_dst_port(val) // u16

      // l4 icmp id
      .push_proto_icmp_id(val) // u16
      .push_proto_icmp_type(val) // u8
      .push_proto_icmp_code(val) // u8

      // l4 icmp id
      .push_proto_icmpv6_id(val) // u16
      .push_proto_icmpv6_type(val) // u8
      .push_proto_icmpv6_code(val) // u8
    .end_nested()

    // conntrack zone id
    .push_tuple_zone(val) // u16
  .end_nested()

  // conntrack flag bits
  // Associated type: "NfCtStatus" (1 bit per enumeration)
  .push_status(val) // u32
  .nested_protoinfo()

    // conntrack tcp state information
    .nested_protoinfo_tcp()

      // tcp connection state
      // Associated type: "NfCtTcpState" (enum)
      .push_tcp_state(val) // u8

      // window scaling factor in original direction
      .push_tcp_wscale_original(val) // u8

      // window scaling factor in reply direction
      .push_tcp_wscale_reply(val) // u8
      .push_tcp_flags_original(val) // PushNfCtTcpFlagsMask
      .push_tcp_flags_reply(val) // PushNfCtTcpFlagsMask
    .end_nested()

    // conntrack dccp state information
    .nested_protoinfo_dccp()

      // dccp connection state
      .push_dccp_state(val) // u8
      .push_dccp_role(val) // u8
      .push_dccp_handshake_seq(val) // u64
      .push_dccp_pad(val) // &[u8]
    .end_nested()

    // conntrack sctp state information
    .nested_protoinfo_sctp()

      // sctp connection state
      // Associated type: "NfCtSctpState" (enum)
      .push_sctp_state(val) // u8
      .push_vtag_original(val) // u32
      .push_vtag_reply(val) // u32
    .end_nested()
  .end_nested()
  .nested_help()

    // helper name
    .push_help_name(val) // &CStr
    .push_help_name_bytes(val) // &[u8]
  .end_nested()
  .nested_nat_src()
    .push_nat_v4_minip(val) // u32
    .push_nat_v4_maxip(val) // u32
    .push_nat_v6_minip(val) // &[u8]
    .push_nat_v6_maxip(val) // &[u8]
    .nested_nat_proto()
      .push_nat_port_min(val) // u16
      .push_nat_port_max(val) // u16
    .end_nested()
  .end_nested()
  .nested_nat_dst()
    .push_nat_v4_minip(val) // u32
    .push_nat_v4_maxip(val) // u32
    .push_nat_v6_minip(val) // &[u8]
    .push_nat_v6_maxip(val) // &[u8]
    .nested_nat_proto()
      .push_nat_port_min(val) // u16
      .push_nat_port_max(val) // u16
    .end_nested()
  .end_nested()
  .push_timeout(val) // u32
  .push_mark(val) // u32
  .nested_counters_orig()
    .push_packets(val) // u64
    .push_bytes(val) // u64
    .push_packets_old(val) // u32
    .push_bytes_old(val) // u32
    .push_pad(val) // &[u8]
  .end_nested()
  .nested_counters_reply()
    .push_packets(val) // u64
    .push_bytes(val) // u64
    .push_packets_old(val) // u32
    .push_bytes_old(val) // u32
    .push_pad(val) // &[u8]
  .end_nested()
  .push_use(val) // u32
  .push_id(val) // u32
  .nested_nat_dst()
    .push_nat_v4_minip(val) // u32
    .push_nat_v4_maxip(val) // u32
    .push_nat_v6_minip(val) // &[u8]
    .push_nat_v6_maxip(val) // &[u8]
    .nested_nat_proto()
      .push_nat_port_min(val) // u16
      .push_nat_port_max(val) // u16
    .end_nested()
  .end_nested()
  .nested_tuple_master()

    // conntrack l3 information
    .nested_tuple_ip()

      // ipv4 source address
      .push_ip_v4_src(val) // Ipv4Addr

      // ipv4 destination address
      .push_ip_v4_dst(val) // Ipv4Addr

      // ipv6 source address
      .push_ip_v6_src(val) // Ipv6Addr

      // ipv6 destination address
      .push_ip_v6_dst(val) // Ipv6Addr
    .end_nested()

    // conntrack l4 information
    .nested_tuple_proto()

      // l4 protocol number
      .push_proto_num(val) // u8

      // l4 source port
      .push_proto_src_port(val) // u16

      // l4 source port
      .push_proto_dst_port(val) // u16

      // l4 icmp id
      .push_proto_icmp_id(val) // u16
      .push_proto_icmp_type(val) // u8
      .push_proto_icmp_code(val) // u8

      // l4 icmp id
      .push_proto_icmpv6_id(val) // u16
      .push_proto_icmpv6_type(val) // u8
      .push_proto_icmpv6_code(val) // u8
    .end_nested()

    // conntrack zone id
    .push_tuple_zone(val) // u16
  .end_nested()
  .nested_seq_adj_orig()
    .push_correction_pos(val) // u32
    .push_offset_before(val) // u32
    .push_offset_after(val) // u32
  .end_nested()
  .nested_seq_adj_reply()
    .push_correction_pos(val) // u32
    .push_offset_before(val) // u32
    .push_offset_after(val) // u32
  .end_nested()

  // conntrack zone id
  .push_zone(val) // u16
  .nested_secctx()
    .push_secctx_name(val) // &CStr
    .push_secctx_name_bytes(val) // &[u8]
  .end_nested()
  .push_labels(val) // &[u8]
  .nested_synproxy()
    .push_isn(val) // u32
    .push_its(val) // u32
    .push_tsoff(val) // u32
  .end_nested()
  ;
```

```rust
let attrs = OpGetDumpReply::new(buf);

{ // Nested TupleOrig

  // conntrack l3+l4 protocol information, original direction
  let attrs = attrs.get_tuple_orig();
  { // Nested TupleIp

    // conntrack l3 information
    let attrs = attrs.get_tuple_ip();

    // ipv4 source address
    attrs.get_ip_v4_src(); // Ipv4Addr

    // ipv4 destination address
    attrs.get_ip_v4_dst(); // Ipv4Addr

    // ipv6 source address
    attrs.get_ip_v6_src(); // Ipv6Addr

    // ipv6 destination address
    attrs.get_ip_v6_dst(); // Ipv6Addr
  }
  { // Nested TupleProto

    // conntrack l4 information
    let attrs = attrs.get_tuple_proto();

    // l4 protocol number
    attrs.get_proto_num(); // u8

    // l4 source port
    attrs.get_proto_src_port(); // u16

    // l4 source port
    attrs.get_proto_dst_port(); // u16

    // l4 icmp id
    attrs.get_proto_icmp_id(); // u16
    attrs.get_proto_icmp_type(); // u8
    attrs.get_proto_icmp_code(); // u8

    // l4 icmp id
    attrs.get_proto_icmpv6_id(); // u16
    attrs.get_proto_icmpv6_type(); // u8
    attrs.get_proto_icmpv6_code(); // u8
  }

  // conntrack zone id
  attrs.get_tuple_zone(); // u16
}
{ // Nested TupleReply

  // conntrack l3+l4 protocol information, reply direction
  let attrs = attrs.get_tuple_reply();
  { // Nested TupleIp

    // conntrack l3 information
    let attrs = attrs.get_tuple_ip();

    // ipv4 source address
    attrs.get_ip_v4_src(); // Ipv4Addr

    // ipv4 destination address
    attrs.get_ip_v4_dst(); // Ipv4Addr

    // ipv6 source address
    attrs.get_ip_v6_src(); // Ipv6Addr

    // ipv6 destination address
    attrs.get_ip_v6_dst(); // Ipv6Addr
  }
  { // Nested TupleProto

    // conntrack l4 information
    let attrs = attrs.get_tuple_proto();

    // l4 protocol number
    attrs.get_proto_num(); // u8

    // l4 source port
    attrs.get_proto_src_port(); // u16

    // l4 source port
    attrs.get_proto_dst_port(); // u16

    // l4 icmp id
    attrs.get_proto_icmp_id(); // u16
    attrs.get_proto_icmp_type(); // u8
    attrs.get_proto_icmp_code(); // u8

    // l4 icmp id
    attrs.get_proto_icmpv6_id(); // u16
    attrs.get_proto_icmpv6_type(); // u8
    attrs.get_proto_icmpv6_code(); // u8
  }

  // conntrack zone id
  attrs.get_tuple_zone(); // u16
}

// conntrack flag bits
// Associated type: "NfCtStatus" (1 bit per enumeration)
attrs.get_status(); // u32
{ // Nested Protoinfo
  let attrs = attrs.get_protoinfo();
  { // Nested ProtoinfoTcp

    // conntrack tcp state information
    let attrs = attrs.get_protoinfo_tcp();

    // tcp connection state
    // Associated type: "NfCtTcpState" (enum)
    attrs.get_tcp_state(); // u8

    // window scaling factor in original direction
    attrs.get_tcp_wscale_original(); // u8

    // window scaling factor in reply direction
    attrs.get_tcp_wscale_reply(); // u8
    attrs.get_tcp_flags_original(); // PushNfCtTcpFlagsMask
    attrs.get_tcp_flags_reply(); // PushNfCtTcpFlagsMask
  }
  { // Nested ProtoinfoDccp

    // conntrack dccp state information
    let attrs = attrs.get_protoinfo_dccp();

    // dccp connection state
    attrs.get_dccp_state(); // u8
    attrs.get_dccp_role(); // u8
    attrs.get_dccp_handshake_seq(); // u64
    attrs.get_dccp_pad(); // &[u8]
  }
  { // Nested ProtoinfoSctp

    // conntrack sctp state information
    let attrs = attrs.get_protoinfo_sctp();

    // sctp connection state
    // Associated type: "NfCtSctpState" (enum)
    attrs.get_sctp_state(); // u8
    attrs.get_vtag_original(); // u32
    attrs.get_vtag_reply(); // u32
  }
}
{ // Nested Help
  let attrs = attrs.get_help();

  // helper name
  attrs.get_help_name(); // &CStr
}
{ // Nested NatSrc
  let attrs = attrs.get_nat_src();
  attrs.get_nat_v4_minip(); // u32
  attrs.get_nat_v4_maxip(); // u32
  attrs.get_nat_v6_minip(); // &[u8]
  attrs.get_nat_v6_maxip(); // &[u8]
  { // Nested NatProto
    let attrs = attrs.get_nat_proto();
    attrs.get_nat_port_min(); // u16
    attrs.get_nat_port_max(); // u16
  }
}
{ // Nested NatDst
  let attrs = attrs.get_nat_dst();
  attrs.get_nat_v4_minip(); // u32
  attrs.get_nat_v4_maxip(); // u32
  attrs.get_nat_v6_minip(); // &[u8]
  attrs.get_nat_v6_maxip(); // &[u8]
  { // Nested NatProto
    let attrs = attrs.get_nat_proto();
    attrs.get_nat_port_min(); // u16
    attrs.get_nat_port_max(); // u16
  }
}
attrs.get_timeout(); // u32
attrs.get_mark(); // u32
{ // Nested CountersOrig
  let attrs = attrs.get_counters_orig();
  attrs.get_packets(); // u64
  attrs.get_bytes(); // u64
  attrs.get_packets_old(); // u32
  attrs.get_bytes_old(); // u32
  attrs.get_pad(); // &[u8]
}
{ // Nested CountersReply
  let attrs = attrs.get_counters_reply();
  attrs.get_packets(); // u64
  attrs.get_bytes(); // u64
  attrs.get_packets_old(); // u32
  attrs.get_bytes_old(); // u32
  attrs.get_pad(); // &[u8]
}
attrs.get_use(); // u32
attrs.get_id(); // u32
{ // Nested NatDst
  let attrs = attrs.get_nat_dst();
  attrs.get_nat_v4_minip(); // u32
  attrs.get_nat_v4_maxip(); // u32
  attrs.get_nat_v6_minip(); // &[u8]
  attrs.get_nat_v6_maxip(); // &[u8]
  { // Nested NatProto
    let attrs = attrs.get_nat_proto();
    attrs.get_nat_port_min(); // u16
    attrs.get_nat_port_max(); // u16
  }
}
{ // Nested TupleMaster
  let attrs = attrs.get_tuple_master();
  { // Nested TupleIp

    // conntrack l3 information
    let attrs = attrs.get_tuple_ip();

    // ipv4 source address
    attrs.get_ip_v4_src(); // Ipv4Addr

    // ipv4 destination address
    attrs.get_ip_v4_dst(); // Ipv4Addr

    // ipv6 source address
    attrs.get_ip_v6_src(); // Ipv6Addr

    // ipv6 destination address
    attrs.get_ip_v6_dst(); // Ipv6Addr
  }
  { // Nested TupleProto

    // conntrack l4 information
    let attrs = attrs.get_tuple_proto();

    // l4 protocol number
    attrs.get_proto_num(); // u8

    // l4 source port
    attrs.get_proto_src_port(); // u16

    // l4 source port
    attrs.get_proto_dst_port(); // u16

    // l4 icmp id
    attrs.get_proto_icmp_id(); // u16
    attrs.get_proto_icmp_type(); // u8
    attrs.get_proto_icmp_code(); // u8

    // l4 icmp id
    attrs.get_proto_icmpv6_id(); // u16
    attrs.get_proto_icmpv6_type(); // u8
    attrs.get_proto_icmpv6_code(); // u8
  }

  // conntrack zone id
  attrs.get_tuple_zone(); // u16
}
{ // Nested SeqAdjOrig
  let attrs = attrs.get_seq_adj_orig();
  attrs.get_correction_pos(); // u32
  attrs.get_offset_before(); // u32
  attrs.get_offset_after(); // u32
}
{ // Nested SeqAdjReply
  let attrs = attrs.get_seq_adj_reply();
  attrs.get_correction_pos(); // u32
  attrs.get_offset_before(); // u32
  attrs.get_offset_after(); // u32
}

// conntrack zone id
attrs.get_zone(); // u16
{ // Nested Secctx
  let attrs = attrs.get_secctx();
  attrs.get_secctx_name(); // &CStr
}
attrs.get_labels(); // &[u8]
{ // Nested Synproxy
  let attrs = attrs.get_synproxy();
  attrs.get_isn(); // u32
  attrs.get_its(); // u32
  attrs.get_tsoff(); // u32
}
```

## Low-level decoding

### Dump (request)

```rust
let iter = OpGetDumpRequest::new(buf);
for attr in iter {
  match attr {
    Mark(val) => {}, // u32
    Filter(iter) => {
      for attr in iter {
        match attr {

          // conntrack l3 information
          TupleIp(iter) => {
            for attr in iter {
              match attr {

                // ipv4 source address
                IpV4Src(val) => {}, // Ipv4Addr

                // ipv4 destination address
                IpV4Dst(val) => {}, // Ipv4Addr

                // ipv6 source address
                IpV6Src(val) => {}, // Ipv6Addr

                // ipv6 destination address
                IpV6Dst(val) => {}, // Ipv6Addr
              }
            }
          },

          // conntrack l4 information
          TupleProto(iter) => {
            for attr in iter {
              match attr {

                // l4 protocol number
                ProtoNum(val) => {}, // u8

                // l4 source port
                ProtoSrcPort(val) => {}, // u16

                // l4 source port
                ProtoDstPort(val) => {}, // u16

                // l4 icmp id
                ProtoIcmpId(val) => {}, // u16
                ProtoIcmpType(val) => {}, // u8
                ProtoIcmpCode(val) => {}, // u8

                // l4 icmp id
                ProtoIcmpv6Id(val) => {}, // u16
                ProtoIcmpv6Type(val) => {}, // u8
                ProtoIcmpv6Code(val) => {}, // u8
              }
            }
          },

          // conntrack zone id
          TupleZone(val) => {}, // u16
        }
      }
    },

    // conntrack flag bits
    // Associated type: "NfCtStatus" (1 bit per enumeration)
    Status(val) => {}, // u32

    // conntrack zone id
    Zone(val) => {}, // u16
  }
}
```

### Dump (reply)

```rust
let iter = OpGetDumpReply::new(buf);
for attr in iter {
  match attr {

    // conntrack l3+l4 protocol information, original direction
    TupleOrig(iter) => {
      for attr in iter {
        match attr {

          // conntrack l3 information
          TupleIp(iter) => {
            for attr in iter {
              match attr {

                // ipv4 source address
                IpV4Src(val) => {}, // Ipv4Addr

                // ipv4 destination address
                IpV4Dst(val) => {}, // Ipv4Addr

                // ipv6 source address
                IpV6Src(val) => {}, // Ipv6Addr

                // ipv6 destination address
                IpV6Dst(val) => {}, // Ipv6Addr
              }
            }
          },

          // conntrack l4 information
          TupleProto(iter) => {
            for attr in iter {
              match attr {

                // l4 protocol number
                ProtoNum(val) => {}, // u8

                // l4 source port
                ProtoSrcPort(val) => {}, // u16

                // l4 source port
                ProtoDstPort(val) => {}, // u16

                // l4 icmp id
                ProtoIcmpId(val) => {}, // u16
                ProtoIcmpType(val) => {}, // u8
                ProtoIcmpCode(val) => {}, // u8

                // l4 icmp id
                ProtoIcmpv6Id(val) => {}, // u16
                ProtoIcmpv6Type(val) => {}, // u8
                ProtoIcmpv6Code(val) => {}, // u8
              }
            }
          },

          // conntrack zone id
          TupleZone(val) => {}, // u16
        }
      }
    },

    // conntrack l3+l4 protocol information, reply direction
    TupleReply(iter) => {
      for attr in iter {
        match attr {

          // conntrack l3 information
          TupleIp(iter) => {
            for attr in iter {
              match attr {

                // ipv4 source address
                IpV4Src(val) => {}, // Ipv4Addr

                // ipv4 destination address
                IpV4Dst(val) => {}, // Ipv4Addr

                // ipv6 source address
                IpV6Src(val) => {}, // Ipv6Addr

                // ipv6 destination address
                IpV6Dst(val) => {}, // Ipv6Addr
              }
            }
          },

          // conntrack l4 information
          TupleProto(iter) => {
            for attr in iter {
              match attr {

                // l4 protocol number
                ProtoNum(val) => {}, // u8

                // l4 source port
                ProtoSrcPort(val) => {}, // u16

                // l4 source port
                ProtoDstPort(val) => {}, // u16

                // l4 icmp id
                ProtoIcmpId(val) => {}, // u16
                ProtoIcmpType(val) => {}, // u8
                ProtoIcmpCode(val) => {}, // u8

                // l4 icmp id
                ProtoIcmpv6Id(val) => {}, // u16
                ProtoIcmpv6Type(val) => {}, // u8
                ProtoIcmpv6Code(val) => {}, // u8
              }
            }
          },

          // conntrack zone id
          TupleZone(val) => {}, // u16
        }
      }
    },

    // conntrack flag bits
    // Associated type: "NfCtStatus" (1 bit per enumeration)
    Status(val) => {}, // u32
    Protoinfo(iter) => {
      for attr in iter {
        match attr {

          // conntrack tcp state information
          ProtoinfoTcp(iter) => {
            for attr in iter {
              match attr {

                // tcp connection state
                // Associated type: "NfCtTcpState" (enum)
                TcpState(val) => {}, // u8

                // window scaling factor in original direction
                TcpWscaleOriginal(val) => {}, // u8

                // window scaling factor in reply direction
                TcpWscaleReply(val) => {}, // u8
                TcpFlagsOriginal(val) => {}, // PushNfCtTcpFlagsMask
                TcpFlagsReply(val) => {}, // PushNfCtTcpFlagsMask
              }
            }
          },

          // conntrack dccp state information
          ProtoinfoDccp(iter) => {
            for attr in iter {
              match attr {

                // dccp connection state
                DccpState(val) => {}, // u8
                DccpRole(val) => {}, // u8
                DccpHandshakeSeq(val) => {}, // u64
                DccpPad(val) => {}, // &[u8]
              }
            }
          },

          // conntrack sctp state information
          ProtoinfoSctp(iter) => {
            for attr in iter {
              match attr {

                // sctp connection state
                // Associated type: "NfCtSctpState" (enum)
                SctpState(val) => {}, // u8
                VtagOriginal(val) => {}, // u32
                VtagReply(val) => {}, // u32
              }
            }
          },
        }
      }
    },
    Help(iter) => {
      for attr in iter {
        match attr {

          // helper name
          HelpName(val) => {}, // &CStr
        }
      }
    },
    NatSrc(iter) => {
      for attr in iter {
        match attr {
          NatV4Minip(val) => {}, // u32
          NatV4Maxip(val) => {}, // u32
          NatV6Minip(val) => {}, // &[u8]
          NatV6Maxip(val) => {}, // &[u8]
          NatProto(iter) => {
            for attr in iter {
              match attr {
                NatPortMin(val) => {}, // u16
                NatPortMax(val) => {}, // u16
              }
            }
          },
        }
      }
    },
    NatDst(iter) => {
      for attr in iter {
        match attr {
          NatV4Minip(val) => {}, // u32
          NatV4Maxip(val) => {}, // u32
          NatV6Minip(val) => {}, // &[u8]
          NatV6Maxip(val) => {}, // &[u8]
          NatProto(iter) => {
            for attr in iter {
              match attr {
                NatPortMin(val) => {}, // u16
                NatPortMax(val) => {}, // u16
              }
            }
          },
        }
      }
    },
    Timeout(val) => {}, // u32
    Mark(val) => {}, // u32
    CountersOrig(iter) => {
      for attr in iter {
        match attr {
          Packets(val) => {}, // u64
          Bytes(val) => {}, // u64
          PacketsOld(val) => {}, // u32
          BytesOld(val) => {}, // u32
          Pad(val) => {}, // &[u8]
        }
      }
    },
    CountersReply(iter) => {
      for attr in iter {
        match attr {
          Packets(val) => {}, // u64
          Bytes(val) => {}, // u64
          PacketsOld(val) => {}, // u32
          BytesOld(val) => {}, // u32
          Pad(val) => {}, // &[u8]
        }
      }
    },
    Use(val) => {}, // u32
    Id(val) => {}, // u32
    NatDst(iter) => {
      for attr in iter {
        match attr {
          NatV4Minip(val) => {}, // u32
          NatV4Maxip(val) => {}, // u32
          NatV6Minip(val) => {}, // &[u8]
          NatV6Maxip(val) => {}, // &[u8]
          NatProto(iter) => {
            for attr in iter {
              match attr {
                NatPortMin(val) => {}, // u16
                NatPortMax(val) => {}, // u16
              }
            }
          },
        }
      }
    },
    TupleMaster(iter) => {
      for attr in iter {
        match attr {

          // conntrack l3 information
          TupleIp(iter) => {
            for attr in iter {
              match attr {

                // ipv4 source address
                IpV4Src(val) => {}, // Ipv4Addr

                // ipv4 destination address
                IpV4Dst(val) => {}, // Ipv4Addr

                // ipv6 source address
                IpV6Src(val) => {}, // Ipv6Addr

                // ipv6 destination address
                IpV6Dst(val) => {}, // Ipv6Addr
              }
            }
          },

          // conntrack l4 information
          TupleProto(iter) => {
            for attr in iter {
              match attr {

                // l4 protocol number
                ProtoNum(val) => {}, // u8

                // l4 source port
                ProtoSrcPort(val) => {}, // u16

                // l4 source port
                ProtoDstPort(val) => {}, // u16

                // l4 icmp id
                ProtoIcmpId(val) => {}, // u16
                ProtoIcmpType(val) => {}, // u8
                ProtoIcmpCode(val) => {}, // u8

                // l4 icmp id
                ProtoIcmpv6Id(val) => {}, // u16
                ProtoIcmpv6Type(val) => {}, // u8
                ProtoIcmpv6Code(val) => {}, // u8
              }
            }
          },

          // conntrack zone id
          TupleZone(val) => {}, // u16
        }
      }
    },
    SeqAdjOrig(iter) => {
      for attr in iter {
        match attr {
          CorrectionPos(val) => {}, // u32
          OffsetBefore(val) => {}, // u32
          OffsetAfter(val) => {}, // u32
        }
      }
    },
    SeqAdjReply(iter) => {
      for attr in iter {
        match attr {
          CorrectionPos(val) => {}, // u32
          OffsetBefore(val) => {}, // u32
          OffsetAfter(val) => {}, // u32
        }
      }
    },

    // conntrack zone id
    Zone(val) => {}, // u16
    Secctx(iter) => {
      for attr in iter {
        match attr {
          SecctxName(val) => {}, // &CStr
        }
      }
    },
    Labels(val) => {}, // &[u8]
    Synproxy(iter) => {
      for attr in iter {
        match attr {
          Isn(val) => {}, // u32
          Its(val) => {}, // u32
          Tsoff(val) => {}, // u32
        }
      }
    },
  }
}
```

# Operation "get-stats"

## Dump (request)

```rust
PushOpGetStatsDumpRequest::new(&mut vec)
  ;
```

```rust
let attrs = OpGetStatsDumpReply::new(buf);

// obsolete
attrs.get_searched(); // u32
attrs.get_found(); // u32
attrs.get_insert(); // u32
attrs.get_insert_failed(); // u32
attrs.get_drop(); // u32
attrs.get_early_drop(); // u32
attrs.get_error(); // u32
attrs.get_search_restart(); // u32
attrs.get_clash_resolve(); // u32
attrs.get_chain_toolong(); // u32
```

### Dump (reply)

```rust
PushOpGetStatsDumpReply::new(&mut vec)

  // obsolete
  .push_searched(val) // u32
  .push_found(val) // u32
  .push_insert(val) // u32
  .push_insert_failed(val) // u32
  .push_drop(val) // u32
  .push_early_drop(val) // u32
  .push_error(val) // u32
  .push_search_restart(val) // u32
  .push_clash_resolve(val) // u32
  .push_chain_toolong(val) // u32
  ;
```

```rust
let attrs = OpGetStatsDumpReply::new(buf);

// obsolete
attrs.get_searched(); // u32
attrs.get_found(); // u32
attrs.get_insert(); // u32
attrs.get_insert_failed(); // u32
attrs.get_drop(); // u32
attrs.get_early_drop(); // u32
attrs.get_error(); // u32
attrs.get_search_restart(); // u32
attrs.get_clash_resolve(); // u32
attrs.get_chain_toolong(); // u32
```

## Low-level decoding

### Dump (request)

```rust
let iter = OpGetStatsDumpRequest::new(buf);
// No attributes
```

### Dump (reply)

```rust
let iter = OpGetStatsDumpReply::new(buf);
for attr in iter {
  match attr {

    // obsolete
    Searched(val) => {}, // u32
    Found(val) => {}, // u32
    Insert(val) => {}, // u32
    InsertFailed(val) => {}, // u32
    Drop(val) => {}, // u32
    EarlyDrop(val) => {}, // u32
    Error(val) => {}, // u32
    SearchRestart(val) => {}, // u32
    ClashResolve(val) => {}, // u32
    ChainToolong(val) => {}, // u32
  }
}
```
