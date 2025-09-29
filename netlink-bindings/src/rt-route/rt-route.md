
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

### Do (reply)

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

## Dump (request)

```rust
PushOpGetrouteDumpRequest::new(&mut vec)
  ;
```

### Dump (reply)

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

### Do (reply)

```rust
let attrs = OpNewrouteDoReply::new(buf);

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

### Do (reply)

```rust
let attrs = OpDelrouteDoReply::new(buf);

// No attributes
```
