
# Operation "getroute"

## Do (request)

```rust
PushOpGetrouteDoRequest::new(&mut vec)
  .push_src(val) // &[u8]
  .push_dst(val) // &[u8]
  .push_iif(val) // u32
  .push_oif(val) // u32
  .push_ip_proto(val) // u8
  .push_sport(val) // u16
  .push_dport(val) // u16
  .push_mark(val) // u32
  .push_uid(val) // u32
  .push_flowlabel(val) // u32
  ;
```

```rust
let attrs = OpGetrouteDoReply::new(buf);

attrs.get_dst(); // &[u8]
attrs.get_src(); // &[u8]
attrs.get_iif(); // u32
attrs.get_oif(); // u32
attrs.get_gateway(); // &[u8]
attrs.get_priority(); // u32
attrs.get_prefsrc(); // &[u8]
{ // Nested Metrics
  let attrs = attrs.get_metrics();
  attrs.get_lock(); // u32
  attrs.get_mtu(); // u32
  attrs.get_window(); // u32
  attrs.get_rtt(); // u32
  attrs.get_rttvar(); // u32
  attrs.get_ssthresh(); // u32
  attrs.get_cwnd(); // u32
  attrs.get_advmss(); // u32
  attrs.get_reordering(); // u32
  attrs.get_hoplimit(); // u32
  attrs.get_initcwnd(); // u32
  attrs.get_features(); // u32
  attrs.get_rto_min(); // u32
  attrs.get_initrwnd(); // u32
  attrs.get_quickack(); // u32
  attrs.get_cc_algo(); // &CStr
  attrs.get_fastopen_no_cookie(); // u32
}
attrs.get_multipath(); // &[u8]
attrs.get_flow(); // u32
attrs.get_cacheinfo(); // PushRtaCacheinfo
attrs.get_table(); // u32
attrs.get_mark(); // u32
attrs.get_mfc_stats(); // &[u8]
attrs.get_via(); // &[u8]
attrs.get_newdst(); // &[u8]
attrs.get_pref(); // u8
attrs.get_encap_type(); // u16
attrs.get_encap(); // &[u8]
attrs.get_expires(); // u32
attrs.get_pad(); // &[u8]
attrs.get_uid(); // u32
attrs.get_ttl_propagate(); // u8
attrs.get_ip_proto(); // u8
attrs.get_sport(); // u16
attrs.get_dport(); // u16
attrs.get_nh_id(); // u32
attrs.get_flowlabel(); // u32
```

### Do (reply)

```rust
PushOpGetrouteDoReply::new(&mut vec)
  .push_dst(val) // &[u8]
  .push_src(val) // &[u8]
  .push_iif(val) // u32
  .push_oif(val) // u32
  .push_gateway(val) // &[u8]
  .push_priority(val) // u32
  .push_prefsrc(val) // &[u8]
  .nested_metrics()
    .push_lock(val) // u32
    .push_mtu(val) // u32
    .push_window(val) // u32
    .push_rtt(val) // u32
    .push_rttvar(val) // u32
    .push_ssthresh(val) // u32
    .push_cwnd(val) // u32
    .push_advmss(val) // u32
    .push_reordering(val) // u32
    .push_hoplimit(val) // u32
    .push_initcwnd(val) // u32
    .push_features(val) // u32
    .push_rto_min(val) // u32
    .push_initrwnd(val) // u32
    .push_quickack(val) // u32
    .push_cc_algo(val) // &CStr
    .push_cc_algo_bytes(val) // &[u8]
    .push_fastopen_no_cookie(val) // u32
  .end_nested()
  .push_multipath(val) // &[u8]
  .push_flow(val) // u32
  .push_cacheinfo(val) // PushRtaCacheinfo
  .push_table(val) // u32
  .push_mark(val) // u32
  .push_mfc_stats(val) // &[u8]
  .push_via(val) // &[u8]
  .push_newdst(val) // &[u8]
  .push_pref(val) // u8
  .push_encap_type(val) // u16
  .push_encap(val) // &[u8]
  .push_expires(val) // u32
  .push_pad(val) // &[u8]
  .push_uid(val) // u32
  .push_ttl_propagate(val) // u8
  .push_ip_proto(val) // u8
  .push_sport(val) // u16
  .push_dport(val) // u16
  .push_nh_id(val) // u32
  .push_flowlabel(val) // u32
  ;
```

```rust
let attrs = OpGetrouteDoReply::new(buf);

attrs.get_dst(); // &[u8]
attrs.get_src(); // &[u8]
attrs.get_iif(); // u32
attrs.get_oif(); // u32
attrs.get_gateway(); // &[u8]
attrs.get_priority(); // u32
attrs.get_prefsrc(); // &[u8]
{ // Nested Metrics
  let attrs = attrs.get_metrics();
  attrs.get_lock(); // u32
  attrs.get_mtu(); // u32
  attrs.get_window(); // u32
  attrs.get_rtt(); // u32
  attrs.get_rttvar(); // u32
  attrs.get_ssthresh(); // u32
  attrs.get_cwnd(); // u32
  attrs.get_advmss(); // u32
  attrs.get_reordering(); // u32
  attrs.get_hoplimit(); // u32
  attrs.get_initcwnd(); // u32
  attrs.get_features(); // u32
  attrs.get_rto_min(); // u32
  attrs.get_initrwnd(); // u32
  attrs.get_quickack(); // u32
  attrs.get_cc_algo(); // &CStr
  attrs.get_fastopen_no_cookie(); // u32
}
attrs.get_multipath(); // &[u8]
attrs.get_flow(); // u32
attrs.get_cacheinfo(); // PushRtaCacheinfo
attrs.get_table(); // u32
attrs.get_mark(); // u32
attrs.get_mfc_stats(); // &[u8]
attrs.get_via(); // &[u8]
attrs.get_newdst(); // &[u8]
attrs.get_pref(); // u8
attrs.get_encap_type(); // u16
attrs.get_encap(); // &[u8]
attrs.get_expires(); // u32
attrs.get_pad(); // &[u8]
attrs.get_uid(); // u32
attrs.get_ttl_propagate(); // u8
attrs.get_ip_proto(); // u8
attrs.get_sport(); // u16
attrs.get_dport(); // u16
attrs.get_nh_id(); // u32
attrs.get_flowlabel(); // u32
```

## Low-level decoding

### Do (request)

```rust
let iter = OpGetrouteDoRequest::new(buf);
for attr in iter {
  match attr {
    Src(val) => {}, // &[u8]
    Dst(val) => {}, // &[u8]
    Iif(val) => {}, // u32
    Oif(val) => {}, // u32
    IpProto(val) => {}, // u8
    Sport(val) => {}, // u16
    Dport(val) => {}, // u16
    Mark(val) => {}, // u32
    Uid(val) => {}, // u32
    Flowlabel(val) => {}, // u32
  }
}
```

### Do (reply)

```rust
let iter = OpGetrouteDoReply::new(buf);
for attr in iter {
  match attr {
    Dst(val) => {}, // &[u8]
    Src(val) => {}, // &[u8]
    Iif(val) => {}, // u32
    Oif(val) => {}, // u32
    Gateway(val) => {}, // &[u8]
    Priority(val) => {}, // u32
    Prefsrc(val) => {}, // &[u8]
    Metrics(iter) => {
      for attr in iter {
        match attr {
          Lock(val) => {}, // u32
          Mtu(val) => {}, // u32
          Window(val) => {}, // u32
          Rtt(val) => {}, // u32
          Rttvar(val) => {}, // u32
          Ssthresh(val) => {}, // u32
          Cwnd(val) => {}, // u32
          Advmss(val) => {}, // u32
          Reordering(val) => {}, // u32
          Hoplimit(val) => {}, // u32
          Initcwnd(val) => {}, // u32
          Features(val) => {}, // u32
          RtoMin(val) => {}, // u32
          Initrwnd(val) => {}, // u32
          Quickack(val) => {}, // u32
          CcAlgo(val) => {}, // &CStr
          FastopenNoCookie(val) => {}, // u32
        }
      }
    },
    Multipath(val) => {}, // &[u8]
    Flow(val) => {}, // u32
    Cacheinfo(val) => {}, // PushRtaCacheinfo
    Table(val) => {}, // u32
    Mark(val) => {}, // u32
    MfcStats(val) => {}, // &[u8]
    Via(val) => {}, // &[u8]
    Newdst(val) => {}, // &[u8]
    Pref(val) => {}, // u8
    EncapType(val) => {}, // u16
    Encap(val) => {}, // &[u8]
    Expires(val) => {}, // u32
    Pad(val) => {}, // &[u8]
    Uid(val) => {}, // u32
    TtlPropagate(val) => {}, // u8
    IpProto(val) => {}, // u8
    Sport(val) => {}, // u16
    Dport(val) => {}, // u16
    NhId(val) => {}, // u32
    Flowlabel(val) => {}, // u32
  }
}
```

## Dump (request)

```rust
PushOpGetrouteDumpRequest::new(&mut vec)
  ;
```

```rust
let attrs = OpGetrouteDumpReply::new(buf);

attrs.get_dst(); // &[u8]
attrs.get_src(); // &[u8]
attrs.get_iif(); // u32
attrs.get_oif(); // u32
attrs.get_gateway(); // &[u8]
attrs.get_priority(); // u32
attrs.get_prefsrc(); // &[u8]
{ // Nested Metrics
  let attrs = attrs.get_metrics();
  attrs.get_lock(); // u32
  attrs.get_mtu(); // u32
  attrs.get_window(); // u32
  attrs.get_rtt(); // u32
  attrs.get_rttvar(); // u32
  attrs.get_ssthresh(); // u32
  attrs.get_cwnd(); // u32
  attrs.get_advmss(); // u32
  attrs.get_reordering(); // u32
  attrs.get_hoplimit(); // u32
  attrs.get_initcwnd(); // u32
  attrs.get_features(); // u32
  attrs.get_rto_min(); // u32
  attrs.get_initrwnd(); // u32
  attrs.get_quickack(); // u32
  attrs.get_cc_algo(); // &CStr
  attrs.get_fastopen_no_cookie(); // u32
}
attrs.get_multipath(); // &[u8]
attrs.get_flow(); // u32
attrs.get_cacheinfo(); // PushRtaCacheinfo
attrs.get_table(); // u32
attrs.get_mark(); // u32
attrs.get_mfc_stats(); // &[u8]
attrs.get_via(); // &[u8]
attrs.get_newdst(); // &[u8]
attrs.get_pref(); // u8
attrs.get_encap_type(); // u16
attrs.get_encap(); // &[u8]
attrs.get_expires(); // u32
attrs.get_pad(); // &[u8]
attrs.get_uid(); // u32
attrs.get_ttl_propagate(); // u8
attrs.get_ip_proto(); // u8
attrs.get_sport(); // u16
attrs.get_dport(); // u16
attrs.get_nh_id(); // u32
attrs.get_flowlabel(); // u32
```

### Dump (reply)

```rust
PushOpGetrouteDumpReply::new(&mut vec)
  .push_dst(val) // &[u8]
  .push_src(val) // &[u8]
  .push_iif(val) // u32
  .push_oif(val) // u32
  .push_gateway(val) // &[u8]
  .push_priority(val) // u32
  .push_prefsrc(val) // &[u8]
  .nested_metrics()
    .push_lock(val) // u32
    .push_mtu(val) // u32
    .push_window(val) // u32
    .push_rtt(val) // u32
    .push_rttvar(val) // u32
    .push_ssthresh(val) // u32
    .push_cwnd(val) // u32
    .push_advmss(val) // u32
    .push_reordering(val) // u32
    .push_hoplimit(val) // u32
    .push_initcwnd(val) // u32
    .push_features(val) // u32
    .push_rto_min(val) // u32
    .push_initrwnd(val) // u32
    .push_quickack(val) // u32
    .push_cc_algo(val) // &CStr
    .push_cc_algo_bytes(val) // &[u8]
    .push_fastopen_no_cookie(val) // u32
  .end_nested()
  .push_multipath(val) // &[u8]
  .push_flow(val) // u32
  .push_cacheinfo(val) // PushRtaCacheinfo
  .push_table(val) // u32
  .push_mark(val) // u32
  .push_mfc_stats(val) // &[u8]
  .push_via(val) // &[u8]
  .push_newdst(val) // &[u8]
  .push_pref(val) // u8
  .push_encap_type(val) // u16
  .push_encap(val) // &[u8]
  .push_expires(val) // u32
  .push_pad(val) // &[u8]
  .push_uid(val) // u32
  .push_ttl_propagate(val) // u8
  .push_ip_proto(val) // u8
  .push_sport(val) // u16
  .push_dport(val) // u16
  .push_nh_id(val) // u32
  .push_flowlabel(val) // u32
  ;
```

```rust
let attrs = OpGetrouteDumpReply::new(buf);

attrs.get_dst(); // &[u8]
attrs.get_src(); // &[u8]
attrs.get_iif(); // u32
attrs.get_oif(); // u32
attrs.get_gateway(); // &[u8]
attrs.get_priority(); // u32
attrs.get_prefsrc(); // &[u8]
{ // Nested Metrics
  let attrs = attrs.get_metrics();
  attrs.get_lock(); // u32
  attrs.get_mtu(); // u32
  attrs.get_window(); // u32
  attrs.get_rtt(); // u32
  attrs.get_rttvar(); // u32
  attrs.get_ssthresh(); // u32
  attrs.get_cwnd(); // u32
  attrs.get_advmss(); // u32
  attrs.get_reordering(); // u32
  attrs.get_hoplimit(); // u32
  attrs.get_initcwnd(); // u32
  attrs.get_features(); // u32
  attrs.get_rto_min(); // u32
  attrs.get_initrwnd(); // u32
  attrs.get_quickack(); // u32
  attrs.get_cc_algo(); // &CStr
  attrs.get_fastopen_no_cookie(); // u32
}
attrs.get_multipath(); // &[u8]
attrs.get_flow(); // u32
attrs.get_cacheinfo(); // PushRtaCacheinfo
attrs.get_table(); // u32
attrs.get_mark(); // u32
attrs.get_mfc_stats(); // &[u8]
attrs.get_via(); // &[u8]
attrs.get_newdst(); // &[u8]
attrs.get_pref(); // u8
attrs.get_encap_type(); // u16
attrs.get_encap(); // &[u8]
attrs.get_expires(); // u32
attrs.get_pad(); // &[u8]
attrs.get_uid(); // u32
attrs.get_ttl_propagate(); // u8
attrs.get_ip_proto(); // u8
attrs.get_sport(); // u16
attrs.get_dport(); // u16
attrs.get_nh_id(); // u32
attrs.get_flowlabel(); // u32
```

## Low-level decoding

### Dump (request)

```rust
let iter = OpGetrouteDumpRequest::new(buf);
// No attributes
```

### Dump (reply)

```rust
let iter = OpGetrouteDumpReply::new(buf);
for attr in iter {
  match attr {
    Dst(val) => {}, // &[u8]
    Src(val) => {}, // &[u8]
    Iif(val) => {}, // u32
    Oif(val) => {}, // u32
    Gateway(val) => {}, // &[u8]
    Priority(val) => {}, // u32
    Prefsrc(val) => {}, // &[u8]
    Metrics(iter) => {
      for attr in iter {
        match attr {
          Lock(val) => {}, // u32
          Mtu(val) => {}, // u32
          Window(val) => {}, // u32
          Rtt(val) => {}, // u32
          Rttvar(val) => {}, // u32
          Ssthresh(val) => {}, // u32
          Cwnd(val) => {}, // u32
          Advmss(val) => {}, // u32
          Reordering(val) => {}, // u32
          Hoplimit(val) => {}, // u32
          Initcwnd(val) => {}, // u32
          Features(val) => {}, // u32
          RtoMin(val) => {}, // u32
          Initrwnd(val) => {}, // u32
          Quickack(val) => {}, // u32
          CcAlgo(val) => {}, // &CStr
          FastopenNoCookie(val) => {}, // u32
        }
      }
    },
    Multipath(val) => {}, // &[u8]
    Flow(val) => {}, // u32
    Cacheinfo(val) => {}, // PushRtaCacheinfo
    Table(val) => {}, // u32
    Mark(val) => {}, // u32
    MfcStats(val) => {}, // &[u8]
    Via(val) => {}, // &[u8]
    Newdst(val) => {}, // &[u8]
    Pref(val) => {}, // u8
    EncapType(val) => {}, // u16
    Encap(val) => {}, // &[u8]
    Expires(val) => {}, // u32
    Pad(val) => {}, // &[u8]
    Uid(val) => {}, // u32
    TtlPropagate(val) => {}, // u8
    IpProto(val) => {}, // u8
    Sport(val) => {}, // u16
    Dport(val) => {}, // u16
    NhId(val) => {}, // u32
    Flowlabel(val) => {}, // u32
  }
}
```

# Operation "newroute"

## Do (request)

```rust
PushOpNewrouteDoRequest::new(&mut vec)
  .push_dst(val) // &[u8]
  .push_src(val) // &[u8]
  .push_iif(val) // u32
  .push_oif(val) // u32
  .push_gateway(val) // &[u8]
  .push_priority(val) // u32
  .push_prefsrc(val) // &[u8]
  .nested_metrics()
    .push_lock(val) // u32
    .push_mtu(val) // u32
    .push_window(val) // u32
    .push_rtt(val) // u32
    .push_rttvar(val) // u32
    .push_ssthresh(val) // u32
    .push_cwnd(val) // u32
    .push_advmss(val) // u32
    .push_reordering(val) // u32
    .push_hoplimit(val) // u32
    .push_initcwnd(val) // u32
    .push_features(val) // u32
    .push_rto_min(val) // u32
    .push_initrwnd(val) // u32
    .push_quickack(val) // u32
    .push_cc_algo(val) // &CStr
    .push_cc_algo_bytes(val) // &[u8]
    .push_fastopen_no_cookie(val) // u32
  .end_nested()
  .push_multipath(val) // &[u8]
  .push_flow(val) // u32
  .push_cacheinfo(val) // PushRtaCacheinfo
  .push_table(val) // u32
  .push_mark(val) // u32
  .push_mfc_stats(val) // &[u8]
  .push_via(val) // &[u8]
  .push_newdst(val) // &[u8]
  .push_pref(val) // u8
  .push_encap_type(val) // u16
  .push_encap(val) // &[u8]
  .push_expires(val) // u32
  .push_pad(val) // &[u8]
  .push_uid(val) // u32
  .push_ttl_propagate(val) // u8
  .push_ip_proto(val) // u8
  .push_sport(val) // u16
  .push_dport(val) // u16
  .push_nh_id(val) // u32
  .push_flowlabel(val) // u32
  ;
```

```rust
let attrs = OpNewrouteDoReply::new(buf);

// No attributes
```

### Do (reply)

```rust
PushOpNewrouteDoReply::new(&mut vec)
  ;
```

```rust
let attrs = OpNewrouteDoReply::new(buf);

// No attributes
```

## Low-level decoding

### Do (request)

```rust
let iter = OpNewrouteDoRequest::new(buf);
for attr in iter {
  match attr {
    Dst(val) => {}, // &[u8]
    Src(val) => {}, // &[u8]
    Iif(val) => {}, // u32
    Oif(val) => {}, // u32
    Gateway(val) => {}, // &[u8]
    Priority(val) => {}, // u32
    Prefsrc(val) => {}, // &[u8]
    Metrics(iter) => {
      for attr in iter {
        match attr {
          Lock(val) => {}, // u32
          Mtu(val) => {}, // u32
          Window(val) => {}, // u32
          Rtt(val) => {}, // u32
          Rttvar(val) => {}, // u32
          Ssthresh(val) => {}, // u32
          Cwnd(val) => {}, // u32
          Advmss(val) => {}, // u32
          Reordering(val) => {}, // u32
          Hoplimit(val) => {}, // u32
          Initcwnd(val) => {}, // u32
          Features(val) => {}, // u32
          RtoMin(val) => {}, // u32
          Initrwnd(val) => {}, // u32
          Quickack(val) => {}, // u32
          CcAlgo(val) => {}, // &CStr
          FastopenNoCookie(val) => {}, // u32
        }
      }
    },
    Multipath(val) => {}, // &[u8]
    Flow(val) => {}, // u32
    Cacheinfo(val) => {}, // PushRtaCacheinfo
    Table(val) => {}, // u32
    Mark(val) => {}, // u32
    MfcStats(val) => {}, // &[u8]
    Via(val) => {}, // &[u8]
    Newdst(val) => {}, // &[u8]
    Pref(val) => {}, // u8
    EncapType(val) => {}, // u16
    Encap(val) => {}, // &[u8]
    Expires(val) => {}, // u32
    Pad(val) => {}, // &[u8]
    Uid(val) => {}, // u32
    TtlPropagate(val) => {}, // u8
    IpProto(val) => {}, // u8
    Sport(val) => {}, // u16
    Dport(val) => {}, // u16
    NhId(val) => {}, // u32
    Flowlabel(val) => {}, // u32
  }
}
```

### Do (reply)

```rust
let iter = OpNewrouteDoReply::new(buf);
// No attributes
```

# Operation "delroute"

## Do (request)

```rust
PushOpDelrouteDoRequest::new(&mut vec)
  .push_dst(val) // &[u8]
  .push_src(val) // &[u8]
  .push_iif(val) // u32
  .push_oif(val) // u32
  .push_gateway(val) // &[u8]
  .push_priority(val) // u32
  .push_prefsrc(val) // &[u8]
  .nested_metrics()
    .push_lock(val) // u32
    .push_mtu(val) // u32
    .push_window(val) // u32
    .push_rtt(val) // u32
    .push_rttvar(val) // u32
    .push_ssthresh(val) // u32
    .push_cwnd(val) // u32
    .push_advmss(val) // u32
    .push_reordering(val) // u32
    .push_hoplimit(val) // u32
    .push_initcwnd(val) // u32
    .push_features(val) // u32
    .push_rto_min(val) // u32
    .push_initrwnd(val) // u32
    .push_quickack(val) // u32
    .push_cc_algo(val) // &CStr
    .push_cc_algo_bytes(val) // &[u8]
    .push_fastopen_no_cookie(val) // u32
  .end_nested()
  .push_multipath(val) // &[u8]
  .push_flow(val) // u32
  .push_cacheinfo(val) // PushRtaCacheinfo
  .push_table(val) // u32
  .push_mark(val) // u32
  .push_mfc_stats(val) // &[u8]
  .push_via(val) // &[u8]
  .push_newdst(val) // &[u8]
  .push_pref(val) // u8
  .push_encap_type(val) // u16
  .push_encap(val) // &[u8]
  .push_expires(val) // u32
  .push_pad(val) // &[u8]
  .push_uid(val) // u32
  .push_ttl_propagate(val) // u8
  .push_ip_proto(val) // u8
  .push_sport(val) // u16
  .push_dport(val) // u16
  .push_nh_id(val) // u32
  .push_flowlabel(val) // u32
  ;
```

```rust
let attrs = OpDelrouteDoReply::new(buf);

// No attributes
```

### Do (reply)

```rust
PushOpDelrouteDoReply::new(&mut vec)
  ;
```

```rust
let attrs = OpDelrouteDoReply::new(buf);

// No attributes
```

## Low-level decoding

### Do (request)

```rust
let iter = OpDelrouteDoRequest::new(buf);
for attr in iter {
  match attr {
    Dst(val) => {}, // &[u8]
    Src(val) => {}, // &[u8]
    Iif(val) => {}, // u32
    Oif(val) => {}, // u32
    Gateway(val) => {}, // &[u8]
    Priority(val) => {}, // u32
    Prefsrc(val) => {}, // &[u8]
    Metrics(iter) => {
      for attr in iter {
        match attr {
          Lock(val) => {}, // u32
          Mtu(val) => {}, // u32
          Window(val) => {}, // u32
          Rtt(val) => {}, // u32
          Rttvar(val) => {}, // u32
          Ssthresh(val) => {}, // u32
          Cwnd(val) => {}, // u32
          Advmss(val) => {}, // u32
          Reordering(val) => {}, // u32
          Hoplimit(val) => {}, // u32
          Initcwnd(val) => {}, // u32
          Features(val) => {}, // u32
          RtoMin(val) => {}, // u32
          Initrwnd(val) => {}, // u32
          Quickack(val) => {}, // u32
          CcAlgo(val) => {}, // &CStr
          FastopenNoCookie(val) => {}, // u32
        }
      }
    },
    Multipath(val) => {}, // &[u8]
    Flow(val) => {}, // u32
    Cacheinfo(val) => {}, // PushRtaCacheinfo
    Table(val) => {}, // u32
    Mark(val) => {}, // u32
    MfcStats(val) => {}, // &[u8]
    Via(val) => {}, // &[u8]
    Newdst(val) => {}, // &[u8]
    Pref(val) => {}, // u8
    EncapType(val) => {}, // u16
    Encap(val) => {}, // &[u8]
    Expires(val) => {}, // u32
    Pad(val) => {}, // &[u8]
    Uid(val) => {}, // u32
    TtlPropagate(val) => {}, // u8
    IpProto(val) => {}, // u8
    Sport(val) => {}, // u16
    Dport(val) => {}, // u16
    NhId(val) => {}, // u32
    Flowlabel(val) => {}, // u32
  }
}
```

### Do (reply)

```rust
let iter = OpDelrouteDoReply::new(buf);
// No attributes
```
