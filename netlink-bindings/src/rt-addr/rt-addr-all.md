
# Operation "newaddr"

## Do (request)

```rust
PushOpNewaddrDoRequest::new(&mut vec)
  .push_address(val) // &[u8]
  .push_label(val) // &CStr
  .push_local(val) // &[u8]
  .push_cacheinfo(val) // PushIfaCacheinfo
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
    Address(val) => {}, // &[u8]
    Label(val) => {}, // &CStr
    Local(val) => {}, // &[u8]
    Cacheinfo(val) => {}, // PushIfaCacheinfo
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
  .push_address(val) // &[u8]
  .push_local(val) // &[u8]
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
    Address(val) => {}, // &[u8]
    Local(val) => {}, // &[u8]
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

attrs.get_address(); // &[u8]
attrs.get_label(); // &CStr
attrs.get_local(); // &[u8]
attrs.get_cacheinfo(); // PushIfaCacheinfo
```

### Dump (reply)

```rust
PushOpGetaddrDumpReply::new(&mut vec)
  .push_address(val) // &[u8]
  .push_label(val) // &CStr
  .push_local(val) // &[u8]
  .push_cacheinfo(val) // PushIfaCacheinfo
  ;
```

```rust
let attrs = OpGetaddrDumpReply::new(buf);

attrs.get_address(); // &[u8]
attrs.get_label(); // &CStr
attrs.get_local(); // &[u8]
attrs.get_cacheinfo(); // PushIfaCacheinfo
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
    Address(val) => {}, // &[u8]
    Label(val) => {}, // &CStr
    Local(val) => {}, // &[u8]
    Cacheinfo(val) => {}, // PushIfaCacheinfo
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
