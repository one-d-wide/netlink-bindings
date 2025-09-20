
# Operation "get-device"

## Dump (request)

```rust
PushOpGetDeviceDumpRequest::new(&mut vec)
  .push_ifindex(val) // u32
  .push_ifname(val) // &CStr

  // Set to all zeros to remove.
  .push_private_key(val) // &[u8]
  .push_public_key(val) // &[u8]

  // 0 or WGDEVICE_F_REPLACE_PEERS if all current peers
  // should be removed prior to adding the list below.
  // 
  // Associated type: "WgdeviceFlags" (enum)
  .push_flags(val) // u32

  // Set as 0 to choose randomly.
  .push_listen_port(val) // u16

  // Set as 0 to disable.
  .push_fwmark(val) // u32
  .array_peers()
    .entry_nested()
      .push_public_key(val) // &[u8]

      // Set as all zeros to remove.
      .push_preshared_key(val) // &[u8]

      // 0 and/or WGPEER_F_REMOVE_ME if the specified peer should not
      // exist at the end of the operation, rather than added/updated
      // and/or WGPEER_F_REPLACE_ALLOWEDIPS if all current allowed IPs
      // of this peer should be removed prior to adding the list below
      // and/or WGPEER_F_UPDATE_ONLY if the peer should only be set if
      // it already exists.
      // 
      // Associated type: "WgpeerFlags" (enum)
      .push_flags(val) // u32

      // struct sockaddr_in or struct sockaddr_in6
      .push_endpoint(val) // SocketAddr

      // Set as 0 to disable.
      .push_persistent_keepalive_interval(val) // u16
      .push_last_handshake_time(val) // PushKernelTimespec
      .push_rx_bytes(val) // u64
      .push_tx_bytes(val) // u64
      .array_allowedips()
        .entry_nested()
          .push_family(val) // u16

          // struct in_addr or struct in6_add
          .push_ipaddr(val) // IpAddr
          .push_cidr_mask(val) // u8

          // WGALLOWEDIP_F_REMOVE_ME if the specified IP should be removed;
          // otherwise, this IP will be added if it is not already present.
          // 
          // Associated type: "WgallowedipFlags" (enum)
          .push_flags(val) // u32
        .end_nested()
      .end_array()

      // should not be set or used at all by most users of this API,
      // as the most recent protocol will be used when this is unset.
      // Otherwise, must be set to 1.
      .push_protocol_version(val) // u32
    .end_nested()
  .end_array()
  ;
```

### Dump (reply)

```rust
let attrs = OpGetDeviceDumpReply::new(buf);

attrs.get_ifindex(); // u32
attrs.get_ifname(); // &CStr

// Set to all zeros to remove.
attrs.get_private_key(); // &[u8]
attrs.get_public_key(); // &[u8]

// 0 or WGDEVICE_F_REPLACE_PEERS if all current peers
// should be removed prior to adding the list below.
// 
// Associated type: "WgdeviceFlags" (enum)
attrs.get_flags(); // u32

// Set as 0 to choose randomly.
attrs.get_listen_port(); // u16

// Set as 0 to disable.
attrs.get_fwmark(); // u32

for entry in attrs.get_peers() {
  entry.get_public_key(); // &[u8]

  // Set as all zeros to remove.
  entry.get_preshared_key(); // &[u8]

  // 0 and/or WGPEER_F_REMOVE_ME if the specified peer should not
  // exist at the end of the operation, rather than added/updated
  // and/or WGPEER_F_REPLACE_ALLOWEDIPS if all current allowed IPs
  // of this peer should be removed prior to adding the list below
  // and/or WGPEER_F_UPDATE_ONLY if the peer should only be set if
  // it already exists.
  // 
  // Associated type: "WgpeerFlags" (enum)
  entry.get_flags(); // u32

  // struct sockaddr_in or struct sockaddr_in6
  entry.get_endpoint(); // SocketAddr

  // Set as 0 to disable.
  entry.get_persistent_keepalive_interval(); // u16
  entry.get_last_handshake_time(); // PushKernelTimespec
  entry.get_rx_bytes(); // u64
  entry.get_tx_bytes(); // u64

  for entry in entry.get_allowedips() {
    entry.get_family(); // u16

    // struct in_addr or struct in6_add
    entry.get_ipaddr(); // IpAddr
    entry.get_cidr_mask(); // u8

    // WGALLOWEDIP_F_REMOVE_ME if the specified IP should be removed;
    // otherwise, this IP will be added if it is not already present.
    // 
    // Associated type: "WgallowedipFlags" (enum)
    entry.get_flags(); // u32
  }

  // should not be set or used at all by most users of this API,
  // as the most recent protocol will be used when this is unset.
  // Otherwise, must be set to 1.
  entry.get_protocol_version(); // u32
}
```

# Operation "set-device"

## Do (request)

```rust
PushOpSetDeviceDoRequest::new(&mut vec)
  .push_ifindex(val) // u32
  .push_ifname(val) // &CStr

  // Set to all zeros to remove.
  .push_private_key(val) // &[u8]
  .push_public_key(val) // &[u8]

  // 0 or WGDEVICE_F_REPLACE_PEERS if all current peers
  // should be removed prior to adding the list below.
  // 
  // Associated type: "WgdeviceFlags" (enum)
  .push_flags(val) // u32

  // Set as 0 to choose randomly.
  .push_listen_port(val) // u16

  // Set as 0 to disable.
  .push_fwmark(val) // u32
  .array_peers()
    .entry_nested()
      .push_public_key(val) // &[u8]

      // Set as all zeros to remove.
      .push_preshared_key(val) // &[u8]

      // 0 and/or WGPEER_F_REMOVE_ME if the specified peer should not
      // exist at the end of the operation, rather than added/updated
      // and/or WGPEER_F_REPLACE_ALLOWEDIPS if all current allowed IPs
      // of this peer should be removed prior to adding the list below
      // and/or WGPEER_F_UPDATE_ONLY if the peer should only be set if
      // it already exists.
      // 
      // Associated type: "WgpeerFlags" (enum)
      .push_flags(val) // u32

      // struct sockaddr_in or struct sockaddr_in6
      .push_endpoint(val) // SocketAddr

      // Set as 0 to disable.
      .push_persistent_keepalive_interval(val) // u16
      .push_last_handshake_time(val) // PushKernelTimespec
      .push_rx_bytes(val) // u64
      .push_tx_bytes(val) // u64
      .array_allowedips()
        .entry_nested()
          .push_family(val) // u16

          // struct in_addr or struct in6_add
          .push_ipaddr(val) // IpAddr
          .push_cidr_mask(val) // u8

          // WGALLOWEDIP_F_REMOVE_ME if the specified IP should be removed;
          // otherwise, this IP will be added if it is not already present.
          // 
          // Associated type: "WgallowedipFlags" (enum)
          .push_flags(val) // u32
        .end_nested()
      .end_array()

      // should not be set or used at all by most users of this API,
      // as the most recent protocol will be used when this is unset.
      // Otherwise, must be set to 1.
      .push_protocol_version(val) // u32
    .end_nested()
  .end_array()
  ;
```

### Do (reply)

```rust
let attrs = OpSetDeviceDoReply::new(buf);

// No attributes
```
