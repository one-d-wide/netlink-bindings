
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

```rust
let attrs = OpNewneighDoReply::new(buf);

// No attributes
```

### Do (reply)

```rust
PushOpNewneighDoReply::new(&mut vec)
  ;
```

```rust
let attrs = OpNewneighDoReply::new(buf);

// No attributes
```

## Low-level decoding

### Do (request)

```rust
let iter = OpNewneighDoRequest::new(buf);
for attr in iter {
  match attr {
    Dst(val) => {}, // &[u8]
    Lladdr(val) => {}, // &[u8]
    Probes(val) => {}, // u32
    Vlan(val) => {}, // u16
    Port(val) => {}, // u16
    Vni(val) => {}, // u32
    Ifindex(val) => {}, // u32
    Master(val) => {}, // u32
    Protocol(val) => {}, // u8
    NhId(val) => {}, // u32

    // Associated type: "NtfExtFlags" (enum)
    FlagsExt(val) => {}, // u32
    FdbExtAttrs(val) => {}, // &[u8]
  }
}
```

### Do (reply)

```rust
let iter = OpNewneighDoReply::new(buf);
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

```rust
let attrs = OpDelneighDoReply::new(buf);

// No attributes
```

### Do (reply)

```rust
PushOpDelneighDoReply::new(&mut vec)
  ;
```

```rust
let attrs = OpDelneighDoReply::new(buf);

// No attributes
```

## Low-level decoding

### Do (request)

```rust
let iter = OpDelneighDoRequest::new(buf);
for attr in iter {
  match attr {
    Dst(val) => {}, // &[u8]
    Ifindex(val) => {}, // u32
  }
}
```

### Do (reply)

```rust
let iter = OpDelneighDoReply::new(buf);
// No attributes
```

# Operation "getneigh"

## Do (request)

```rust
PushOpGetneighDoRequest::new(&mut vec)
  .push_dst(val) // &[u8]
  ;
```

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

### Do (reply)

```rust
PushOpGetneighDoReply::new(&mut vec)
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

## Low-level decoding

### Do (request)

```rust
let iter = OpGetneighDoRequest::new(buf);
for attr in iter {
  match attr {
    Dst(val) => {}, // &[u8]
  }
}
```

### Do (reply)

```rust
let iter = OpGetneighDoReply::new(buf);
for attr in iter {
  match attr {
    Dst(val) => {}, // &[u8]
    Lladdr(val) => {}, // &[u8]
    Probes(val) => {}, // u32
    Vlan(val) => {}, // u16
    Port(val) => {}, // u16
    Vni(val) => {}, // u32
    Ifindex(val) => {}, // u32
    Master(val) => {}, // u32
    Protocol(val) => {}, // u8
    NhId(val) => {}, // u32

    // Associated type: "NtfExtFlags" (enum)
    FlagsExt(val) => {}, // u32
    FdbExtAttrs(val) => {}, // &[u8]
  }
}
```

## Dump (request)

```rust
PushOpGetneighDumpRequest::new(&mut vec)
  .push_ifindex(val) // u32
  .push_master(val) // u32
  ;
```

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

### Dump (reply)

```rust
PushOpGetneighDumpReply::new(&mut vec)
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

## Low-level decoding

### Dump (request)

```rust
let iter = OpGetneighDumpRequest::new(buf);
for attr in iter {
  match attr {
    Ifindex(val) => {}, // u32
    Master(val) => {}, // u32
  }
}
```

### Dump (reply)

```rust
let iter = OpGetneighDumpReply::new(buf);
for attr in iter {
  match attr {
    Dst(val) => {}, // &[u8]
    Lladdr(val) => {}, // &[u8]
    Probes(val) => {}, // u32
    Vlan(val) => {}, // u16
    Port(val) => {}, // u16
    Vni(val) => {}, // u32
    Ifindex(val) => {}, // u32
    Master(val) => {}, // u32
    Protocol(val) => {}, // u8
    NhId(val) => {}, // u32

    // Associated type: "NtfExtFlags" (enum)
    FlagsExt(val) => {}, // u32
    FdbExtAttrs(val) => {}, // &[u8]
  }
}
```

# Operation "getneightbl"

## Dump (request)

```rust
PushOpGetneightblDumpRequest::new(&mut vec)
  ;
```

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

### Dump (reply)

```rust
PushOpGetneightblDumpReply::new(&mut vec)
  .push_name(val) // &CStr
  .push_name_bytes(val) // &[u8]
  .push_thresh1(val) // u32
  .push_thresh2(val) // u32
  .push_thresh3(val) // u32
  .push_config(val) // PushNdtConfig
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
  .push_stats(val) // PushNdtStats
  .push_gc_interval(val) // u64
  ;
```

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

## Low-level decoding

### Dump (request)

```rust
let iter = OpGetneightblDumpRequest::new(buf);
// No attributes
```

### Dump (reply)

```rust
let iter = OpGetneightblDumpReply::new(buf);
for attr in iter {
  match attr {
    Name(val) => {}, // &CStr
    Thresh1(val) => {}, // u32
    Thresh2(val) => {}, // u32
    Thresh3(val) => {}, // u32
    Config(val) => {}, // PushNdtConfig
    Parms(iter) => {
      for attr in iter {
        match attr {
          Ifindex(val) => {}, // u32
          Refcnt(val) => {}, // u32
          ReachableTime(val) => {}, // u64
          BaseReachableTime(val) => {}, // u64
          RetransTime(val) => {}, // u64
          GcStaletime(val) => {}, // u64
          DelayProbeTime(val) => {}, // u64
          QueueLen(val) => {}, // u32
          AppProbes(val) => {}, // u32
          UcastProbes(val) => {}, // u32
          McastProbes(val) => {}, // u32
          AnycastDelay(val) => {}, // u64
          ProxyDelay(val) => {}, // u64
          ProxyQlen(val) => {}, // u32
          Locktime(val) => {}, // u64
          QueueLenbytes(val) => {}, // u32
          McastReprobes(val) => {}, // u32
          Pad(val) => {}, // &[u8]
          IntervalProbeTimeMs(val) => {}, // u64
        }
      }
    },
    Stats(val) => {}, // PushNdtStats
    GcInterval(val) => {}, // u64
  }
}
```

# Operation "setneightbl"

## Do (request)

```rust
PushOpSetneightblDoRequest::new(&mut vec)
  .push_name(val) // &CStr
  .push_name_bytes(val) // &[u8]
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

```rust
let attrs = OpSetneightblDoReply::new(buf);

// No attributes
```

### Do (reply)

```rust
PushOpSetneightblDoReply::new(&mut vec)
  ;
```

```rust
let attrs = OpSetneightblDoReply::new(buf);

// No attributes
```

## Low-level decoding

### Do (request)

```rust
let iter = OpSetneightblDoRequest::new(buf);
for attr in iter {
  match attr {
    Name(val) => {}, // &CStr
    Thresh1(val) => {}, // u32
    Thresh2(val) => {}, // u32
    Thresh3(val) => {}, // u32
    Parms(iter) => {
      for attr in iter {
        match attr {
          Ifindex(val) => {}, // u32
          Refcnt(val) => {}, // u32
          ReachableTime(val) => {}, // u64
          BaseReachableTime(val) => {}, // u64
          RetransTime(val) => {}, // u64
          GcStaletime(val) => {}, // u64
          DelayProbeTime(val) => {}, // u64
          QueueLen(val) => {}, // u32
          AppProbes(val) => {}, // u32
          UcastProbes(val) => {}, // u32
          McastProbes(val) => {}, // u32
          AnycastDelay(val) => {}, // u64
          ProxyDelay(val) => {}, // u64
          ProxyQlen(val) => {}, // u32
          Locktime(val) => {}, // u64
          QueueLenbytes(val) => {}, // u32
          McastReprobes(val) => {}, // u32
          Pad(val) => {}, // &[u8]
          IntervalProbeTimeMs(val) => {}, // u64
        }
      }
    },
    GcInterval(val) => {}, // u64
  }
}
```

### Do (reply)

```rust
let iter = OpSetneightblDoReply::new(buf);
// No attributes
```
