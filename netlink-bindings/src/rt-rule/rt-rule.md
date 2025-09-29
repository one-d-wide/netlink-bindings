
# Operation "newrule"

## Do (request)

```rust
PushOpNewruleDoRequest::new(&mut vec)
  .push_iifname(val) // &CStr
  .push_iifname_bytes(val) // &[u8]
  .push_oifname(val) // &CStr
  .push_oifname_bytes(val) // &[u8]
  .push_priority(val) // u32
  .push_fwmark(val) // u32
  .push_flow(val) // u32
  .push_tun_id(val) // u64
  .push_fwmask(val) // u32
  .push_table(val) // u32
  .push_suppress_prefixlen(val) // u32
  .push_suppress_ifgroup(val) // u32
  .push_goto(val) // u32
  .push_l3mdev(val) // u8
  .push_uid_range(val) // PushFibRuleUidRange
  .push_protocol(val) // u8
  .push_ip_proto(val) // u8
  .push_sport_range(val) // PushFibRulePortRange
  .push_dport_range(val) // PushFibRulePortRange
  .push_dscp(val) // u8
  .push_flowlabel(val) // u32
  .push_flowlabel_mask(val) // u32
  .push_sport_mask(val) // u16
  .push_dport_mask(val) // u16
  .push_dscp_mask(val) // u8
  ;
```

### Do (reply)

```rust
let attrs = OpNewruleDoReply::new(buf);

// No attributes
```

# Operation "delrule"

## Do (request)

```rust
PushOpDelruleDoRequest::new(&mut vec)
  .push_iifname(val) // &CStr
  .push_iifname_bytes(val) // &[u8]
  .push_oifname(val) // &CStr
  .push_oifname_bytes(val) // &[u8]
  .push_priority(val) // u32
  .push_fwmark(val) // u32
  .push_flow(val) // u32
  .push_tun_id(val) // u64
  .push_fwmask(val) // u32
  .push_table(val) // u32
  .push_suppress_prefixlen(val) // u32
  .push_suppress_ifgroup(val) // u32
  .push_goto(val) // u32
  .push_l3mdev(val) // u8
  .push_uid_range(val) // PushFibRuleUidRange
  .push_protocol(val) // u8
  .push_ip_proto(val) // u8
  .push_sport_range(val) // PushFibRulePortRange
  .push_dport_range(val) // PushFibRulePortRange
  .push_dscp(val) // u8
  .push_flowlabel(val) // u32
  .push_flowlabel_mask(val) // u32
  .push_sport_mask(val) // u16
  .push_dport_mask(val) // u16
  .push_dscp_mask(val) // u8
  ;
```

### Do (reply)

```rust
let attrs = OpDelruleDoReply::new(buf);

// No attributes
```

# Operation "getrule"

## Dump (request)

```rust
PushOpGetruleDumpRequest::new(&mut vec)
  ;
```

### Dump (reply)

```rust
let attrs = OpGetruleDumpReply::new(buf);

attrs.get_iifname(); // &CStr
attrs.get_oifname(); // &CStr
attrs.get_priority(); // u32
attrs.get_fwmark(); // u32
attrs.get_flow(); // u32
attrs.get_tun_id(); // u64
attrs.get_fwmask(); // u32
attrs.get_table(); // u32
attrs.get_suppress_prefixlen(); // u32
attrs.get_suppress_ifgroup(); // u32
attrs.get_goto(); // u32
attrs.get_l3mdev(); // u8
attrs.get_uid_range(); // PushFibRuleUidRange
attrs.get_protocol(); // u8
attrs.get_ip_proto(); // u8
attrs.get_sport_range(); // PushFibRulePortRange
attrs.get_dport_range(); // PushFibRulePortRange
attrs.get_dscp(); // u8
attrs.get_flowlabel(); // u32
attrs.get_flowlabel_mask(); // u32
attrs.get_sport_mask(); // u16
attrs.get_dport_mask(); // u16
attrs.get_dscp_mask(); // u8
```
