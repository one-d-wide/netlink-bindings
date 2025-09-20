
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

### Do (reply)

```rust
let attrs = OpNewaddrDoReply::new(buf);

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

### Do (reply)

```rust
let attrs = OpDeladdrDoReply::new(buf);

// No attributes
```

# Operation "getaddr"

## Dump (request)

```rust
PushOpGetaddrDumpRequest::new(&mut vec)
  ;
```

### Dump (reply)

```rust
let attrs = OpGetaddrDumpReply::new(buf);

attrs.get_address(); // &[u8]
attrs.get_label(); // &CStr
attrs.get_local(); // &[u8]
attrs.get_cacheinfo(); // PushIfaCacheinfo
```

# Operation "getmulticast"

## Do (request)

```rust
PushOpGetmulticastDoRequest::new(&mut vec)
  ;
```

### Do (reply)

```rust
let attrs = OpGetmulticastDoReply::new(buf);

attrs.get_multicast(); // &[u8]
attrs.get_cacheinfo(); // PushIfaCacheinfo
```

## Dump (request)

```rust
PushOpGetmulticastDumpRequest::new(&mut vec)
  ;
```

### Dump (reply)

```rust
let attrs = OpGetmulticastDumpReply::new(buf);

attrs.get_multicast(); // &[u8]
attrs.get_cacheinfo(); // PushIfaCacheinfo
```
