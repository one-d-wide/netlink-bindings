
# Operation "newaddr"

## Do (request)

```rust
PushOpNewaddrDoRequest::new(&mut vec)
  .push_address(val) // IpAddr
  .push_label(val) // &CStr
  .push_label_bytes(val) // &[u8]
  .push_local(val) // IpAddr
  .push_cacheinfo(val) // PushIfaCacheinfo

  // Associated type: "IfaFlags" (1 bit per enumeration)
  .push_flags(val) // u32
  ;
```

```rust
let attrs = OpNewaddrDoReply::new(buf);

// No attributes
```

### Do (reply)

```rust
PushOpNewaddrDoReply::new(&mut vec)
  ;
```

```rust
let attrs = OpNewaddrDoReply::new(buf);

// No attributes
```

## Low-level decoding

### Do (request)

```rust
let iter = OpNewaddrDoRequest::new(buf);
for attr in iter {
  match attr {
    Address(val) => {}, // IpAddr
    Label(val) => {}, // &CStr
    Local(val) => {}, // IpAddr
    Cacheinfo(val) => {}, // PushIfaCacheinfo

    // Associated type: "IfaFlags" (1 bit per enumeration)
    Flags(val) => {}, // u32
  }
}
```

### Do (reply)

```rust
let iter = OpNewaddrDoReply::new(buf);
// No attributes
```

# Operation "deladdr"

## Do (request)

```rust
PushOpDeladdrDoRequest::new(&mut vec)
  .push_address(val) // IpAddr
  .push_local(val) // IpAddr
  ;
```

```rust
let attrs = OpDeladdrDoReply::new(buf);

// No attributes
```

### Do (reply)

```rust
PushOpDeladdrDoReply::new(&mut vec)
  ;
```

```rust
let attrs = OpDeladdrDoReply::new(buf);

// No attributes
```

## Low-level decoding

### Do (request)

```rust
let iter = OpDeladdrDoRequest::new(buf);
for attr in iter {
  match attr {
    Address(val) => {}, // IpAddr
    Local(val) => {}, // IpAddr
  }
}
```

### Do (reply)

```rust
let iter = OpDeladdrDoReply::new(buf);
// No attributes
```

# Operation "getaddr"

## Dump (request)

```rust
PushOpGetaddrDumpRequest::new(&mut vec)
  ;
```

```rust
let attrs = OpGetaddrDumpReply::new(buf);

attrs.get_address(); // IpAddr
attrs.get_label(); // &CStr
attrs.get_local(); // IpAddr
attrs.get_cacheinfo(); // PushIfaCacheinfo

// Associated type: "IfaFlags" (1 bit per enumeration)
attrs.get_flags(); // u32
```

### Dump (reply)

```rust
PushOpGetaddrDumpReply::new(&mut vec)
  .push_address(val) // IpAddr
  .push_label(val) // &CStr
  .push_label_bytes(val) // &[u8]
  .push_local(val) // IpAddr
  .push_cacheinfo(val) // PushIfaCacheinfo

  // Associated type: "IfaFlags" (1 bit per enumeration)
  .push_flags(val) // u32
  ;
```

```rust
let attrs = OpGetaddrDumpReply::new(buf);

attrs.get_address(); // IpAddr
attrs.get_label(); // &CStr
attrs.get_local(); // IpAddr
attrs.get_cacheinfo(); // PushIfaCacheinfo

// Associated type: "IfaFlags" (1 bit per enumeration)
attrs.get_flags(); // u32
```

## Low-level decoding

### Dump (request)

```rust
let iter = OpGetaddrDumpRequest::new(buf);
// No attributes
```

### Dump (reply)

```rust
let iter = OpGetaddrDumpReply::new(buf);
for attr in iter {
  match attr {
    Address(val) => {}, // IpAddr
    Label(val) => {}, // &CStr
    Local(val) => {}, // IpAddr
    Cacheinfo(val) => {}, // PushIfaCacheinfo

    // Associated type: "IfaFlags" (1 bit per enumeration)
    Flags(val) => {}, // u32
  }
}
```

# Operation "getmulticast"

## Do (request)

```rust
PushOpGetmulticastDoRequest::new(&mut vec)
  ;
```

```rust
let attrs = OpGetmulticastDoReply::new(buf);

attrs.get_multicast(); // &[u8]
attrs.get_cacheinfo(); // PushIfaCacheinfo
```

### Do (reply)

```rust
PushOpGetmulticastDoReply::new(&mut vec)
  .push_multicast(val) // &[u8]
  .push_cacheinfo(val) // PushIfaCacheinfo
  ;
```

```rust
let attrs = OpGetmulticastDoReply::new(buf);

attrs.get_multicast(); // &[u8]
attrs.get_cacheinfo(); // PushIfaCacheinfo
```

## Low-level decoding

### Do (request)

```rust
let iter = OpGetmulticastDoRequest::new(buf);
// No attributes
```

### Do (reply)

```rust
let iter = OpGetmulticastDoReply::new(buf);
for attr in iter {
  match attr {
    Multicast(val) => {}, // &[u8]
    Cacheinfo(val) => {}, // PushIfaCacheinfo
  }
}
```

## Dump (request)

```rust
PushOpGetmulticastDumpRequest::new(&mut vec)
  ;
```

```rust
let attrs = OpGetmulticastDumpReply::new(buf);

attrs.get_multicast(); // &[u8]
attrs.get_cacheinfo(); // PushIfaCacheinfo
```

### Dump (reply)

```rust
PushOpGetmulticastDumpReply::new(&mut vec)
  .push_multicast(val) // &[u8]
  .push_cacheinfo(val) // PushIfaCacheinfo
  ;
```

```rust
let attrs = OpGetmulticastDumpReply::new(buf);

attrs.get_multicast(); // &[u8]
attrs.get_cacheinfo(); // PushIfaCacheinfo
```

## Low-level decoding

### Dump (request)

```rust
let iter = OpGetmulticastDumpRequest::new(buf);
// No attributes
```

### Dump (reply)

```rust
let iter = OpGetmulticastDumpReply::new(buf);
for attr in iter {
  match attr {
    Multicast(val) => {}, // &[u8]
    Cacheinfo(val) => {}, // PushIfaCacheinfo
  }
}
```
