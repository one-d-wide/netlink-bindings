
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

### Do (reply)

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

### Dump (reply)

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

# Operation "get-stats"

## Dump (request)

```rust
PushOpGetStatsDumpRequest::new(&mut vec)
  ;
```

### Dump (reply)

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
