
# Operation "newneigh"

## Do (request)

```rust
PushOpNewneighDoRequest::new(&mut vec)
  .push_dst(val) // &[u8]
  .push_lladdr(val) // &[u8]
  .push_probes(val) // u32
  .push_vlan(val) // u16
  .push_port(val) // u16
  .push_vni(val) // u32
  .push_ifindex(val) // u32
  .push_master(val) // u32
  .push_protocol(val) // u8
  .push_nh_id(val) // u32

  // Associated type: "NtfExtFlags" (enum)
  .push_flags_ext(val) // u32
  .push_fdb_ext_attrs(val) // &[u8]
  ;
```

### Do (reply)

```rust
let attrs = OpNewneighDoReply::new(buf);

// No attributes
```

# Operation "delneigh"

## Do (request)

```rust
PushOpDelneighDoRequest::new(&mut vec)
  .push_dst(val) // &[u8]
  .push_ifindex(val) // u32
  ;
```

### Do (reply)

```rust
let attrs = OpDelneighDoReply::new(buf);

// No attributes
```

# Operation "getneigh"

## Do (request)

```rust
PushOpGetneighDoRequest::new(&mut vec)
  .push_dst(val) // &[u8]
  ;
```

### Do (reply)

```rust
let attrs = OpGetneighDoReply::new(buf);

attrs.get_dst(); // &[u8]
attrs.get_lladdr(); // &[u8]
attrs.get_probes(); // u32
attrs.get_vlan(); // u16
attrs.get_port(); // u16
attrs.get_vni(); // u32
attrs.get_ifindex(); // u32
attrs.get_master(); // u32
attrs.get_protocol(); // u8
attrs.get_nh_id(); // u32

// Associated type: "NtfExtFlags" (enum)
attrs.get_flags_ext(); // u32
attrs.get_fdb_ext_attrs(); // &[u8]
```

## Dump (request)

```rust
PushOpGetneighDumpRequest::new(&mut vec)
  .push_ifindex(val) // u32
  .push_master(val) // u32
  ;
```

### Dump (reply)

```rust
let attrs = OpGetneighDumpReply::new(buf);

attrs.get_dst(); // &[u8]
attrs.get_lladdr(); // &[u8]
attrs.get_probes(); // u32
attrs.get_vlan(); // u16
attrs.get_port(); // u16
attrs.get_vni(); // u32
attrs.get_ifindex(); // u32
attrs.get_master(); // u32
attrs.get_protocol(); // u8
attrs.get_nh_id(); // u32

// Associated type: "NtfExtFlags" (enum)
attrs.get_flags_ext(); // u32
attrs.get_fdb_ext_attrs(); // &[u8]
```

# Operation "getneightbl"

## Dump (request)

```rust
PushOpGetneightblDumpRequest::new(&mut vec)
  ;
```

### Dump (reply)

```rust
let attrs = OpGetneightblDumpReply::new(buf);

attrs.get_name(); // &CStr
attrs.get_thresh1(); // u32
attrs.get_thresh2(); // u32
attrs.get_thresh3(); // u32
attrs.get_config(); // PushNdtConfig
{ // Nested Parms
  let attrs = attrs.get_parms();
  attrs.get_ifindex(); // u32
  attrs.get_refcnt(); // u32
  attrs.get_reachable_time(); // u64
  attrs.get_base_reachable_time(); // u64
  attrs.get_retrans_time(); // u64
  attrs.get_gc_staletime(); // u64
  attrs.get_delay_probe_time(); // u64
  attrs.get_queue_len(); // u32
  attrs.get_app_probes(); // u32
  attrs.get_ucast_probes(); // u32
  attrs.get_mcast_probes(); // u32
  attrs.get_anycast_delay(); // u64
  attrs.get_proxy_delay(); // u64
  attrs.get_proxy_qlen(); // u32
  attrs.get_locktime(); // u64
  attrs.get_queue_lenbytes(); // u32
  attrs.get_mcast_reprobes(); // u32
  attrs.get_pad(); // &[u8]
  attrs.get_interval_probe_time_ms(); // u64
}
attrs.get_stats(); // PushNdtStats
attrs.get_gc_interval(); // u64
```

# Operation "setneightbl"

## Do (request)

```rust
PushOpSetneightblDoRequest::new(&mut vec)
  .push_name(val) // &CStr
  .push_thresh1(val) // u32
  .push_thresh2(val) // u32
  .push_thresh3(val) // u32
  .nested_parms()
    .push_ifindex(val) // u32
    .push_refcnt(val) // u32
    .push_reachable_time(val) // u64
    .push_base_reachable_time(val) // u64
    .push_retrans_time(val) // u64
    .push_gc_staletime(val) // u64
    .push_delay_probe_time(val) // u64
    .push_queue_len(val) // u32
    .push_app_probes(val) // u32
    .push_ucast_probes(val) // u32
    .push_mcast_probes(val) // u32
    .push_anycast_delay(val) // u64
    .push_proxy_delay(val) // u64
    .push_proxy_qlen(val) // u32
    .push_locktime(val) // u64
    .push_queue_lenbytes(val) // u32
    .push_mcast_reprobes(val) // u32
    .push_pad(val) // &[u8]
    .push_interval_probe_time_ms(val) // u64
  .end_nested()
  .push_gc_interval(val) // u64
  ;
```

### Do (reply)

```rust
let attrs = OpSetneightblDoReply::new(buf);

// No attributes
```
