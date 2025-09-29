
# Operation "getfamily"

## Do (request)

```rust
PushOpGetfamilyDoRequest::new(&mut vec)
  .push_family_name(val) // &CStr
  .push_family_name_bytes(val) // &[u8]
  ;
```

### Do (reply)

```rust
let attrs = OpGetfamilyDoReply::new(buf);

attrs.get_family_id(); // u16
attrs.get_family_name(); // &CStr
attrs.get_hdrsize(); // u32
attrs.get_maxattr(); // u32

for entry in attrs.get_mcast_groups() {
  entry.get_name(); // &CStr
  entry.get_id(); // u32
}

for entry in attrs.get_ops() {
  entry.get_id(); // u32

  // Associated type: "OpFlags" (1 bit per enumeration)
  entry.get_flags(); // u32
}
attrs.get_version(); // u32
```

## Dump (request)

```rust
PushOpGetfamilyDumpRequest::new(&mut vec)
  ;
```

### Dump (reply)

```rust
let attrs = OpGetfamilyDumpReply::new(buf);

attrs.get_family_id(); // u16
attrs.get_family_name(); // &CStr
attrs.get_hdrsize(); // u32
attrs.get_maxattr(); // u32

for entry in attrs.get_mcast_groups() {
  entry.get_name(); // &CStr
  entry.get_id(); // u32
}

for entry in attrs.get_ops() {
  entry.get_id(); // u32

  // Associated type: "OpFlags" (1 bit per enumeration)
  entry.get_flags(); // u32
}
attrs.get_version(); // u32
```

# Operation "getpolicy"

## Dump (request)

```rust
PushOpGetpolicyDumpRequest::new(&mut vec)
  .push_family_name(val) // &CStr
  .push_family_name_bytes(val) // &[u8]
  .push_family_id(val) // u16
  .push_op(val) // u32
  ;
```

### Dump (reply)

```rust
let attrs = OpGetpolicyDumpReply::new(buf);

attrs.get_family_id(); // u16
{ // Nested OpPolicy
  let attrs = attrs.get_op_policy();
  attrs.get_do(); // u32
  attrs.get_dump(); // u32
}
{ // Nested Policy
  let attrs = attrs.get_policy();

  // Associated type: "AttrType" (enum)
  attrs.get_type(); // u32
  attrs.get_min_value_s(); // i64
  attrs.get_max_value_s(); // i64
  attrs.get_min_value_u(); // u64
  attrs.get_max_value_u(); // u64
  attrs.get_min_length(); // u32
  attrs.get_max_length(); // u32
  attrs.get_policy_idx(); // u32
  attrs.get_policy_maxtype(); // u32
  attrs.get_bitfield32_mask(); // u32
  attrs.get_mask(); // u64
  attrs.get_pad(); // &[u8]
}
```
