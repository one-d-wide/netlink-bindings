
# Operation "getfamily"

## Do (request)

```rust
PushOpGetfamilyDoRequest::new(&mut vec)
  .push_family_name(val) // &CStr
  .push_family_name_bytes(val) // &[u8]
  ;
```

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

### Do (reply)

```rust
PushOpGetfamilyDoReply::new(&mut vec)
  .push_family_id(val) // u16
  .push_family_name(val) // &CStr
  .push_family_name_bytes(val) // &[u8]
  .push_hdrsize(val) // u32
  .push_maxattr(val) // u32
  .array_mcast_groups()
    .entry_nested()
      .push_name(val) // &CStr
      .push_name_bytes(val) // &[u8]
      .push_id(val) // u32
    .end_nested()
  .end_array()
  .array_ops()
    .entry_nested()
      .push_id(val) // u32

      // Associated type: "OpFlags" (1 bit per enumeration)
      .push_flags(val) // u32
    .end_nested()
  .end_array()
  .push_version(val) // u32
  ;
```

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

## Low-level decoding

### Do (request)

```rust
let iter = OpGetfamilyDoRequest::new(buf);
for attr in iter {
  match attr {
    FamilyName(val) => {}, // &CStr
  }
}
```

### Do (reply)

```rust
let iter = OpGetfamilyDoReply::new(buf);
for attr in iter {
  match attr {
    FamilyId(val) => {}, // u16
    FamilyName(val) => {}, // &CStr
    Hdrsize(val) => {}, // u32
    Maxattr(val) => {}, // u32
    McastGroups(iter) => {
      for entry in iter {
        for attr in entry {
          match attr {
            Name(val) => {}, // &CStr
            Id(val) => {}, // u32
          }
        }
      }
    },
    Ops(iter) => {
      for entry in iter {
        for attr in entry {
          match attr {
            Id(val) => {}, // u32

            // Associated type: "OpFlags" (1 bit per enumeration)
            Flags(val) => {}, // u32
          }
        }
      }
    },
    Version(val) => {}, // u32
  }
}
```

## Dump (request)

```rust
PushOpGetfamilyDumpRequest::new(&mut vec)
  ;
```

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

### Dump (reply)

```rust
PushOpGetfamilyDumpReply::new(&mut vec)
  .push_family_id(val) // u16
  .push_family_name(val) // &CStr
  .push_family_name_bytes(val) // &[u8]
  .push_hdrsize(val) // u32
  .push_maxattr(val) // u32
  .array_mcast_groups()
    .entry_nested()
      .push_name(val) // &CStr
      .push_name_bytes(val) // &[u8]
      .push_id(val) // u32
    .end_nested()
  .end_array()
  .array_ops()
    .entry_nested()
      .push_id(val) // u32

      // Associated type: "OpFlags" (1 bit per enumeration)
      .push_flags(val) // u32
    .end_nested()
  .end_array()
  .push_version(val) // u32
  ;
```

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

## Low-level decoding

### Dump (request)

```rust
let iter = OpGetfamilyDumpRequest::new(buf);
// No attributes
```

### Dump (reply)

```rust
let iter = OpGetfamilyDumpReply::new(buf);
for attr in iter {
  match attr {
    FamilyId(val) => {}, // u16
    FamilyName(val) => {}, // &CStr
    Hdrsize(val) => {}, // u32
    Maxattr(val) => {}, // u32
    McastGroups(iter) => {
      for entry in iter {
        for attr in entry {
          match attr {
            Name(val) => {}, // &CStr
            Id(val) => {}, // u32
          }
        }
      }
    },
    Ops(iter) => {
      for entry in iter {
        for attr in entry {
          match attr {
            Id(val) => {}, // u32

            // Associated type: "OpFlags" (1 bit per enumeration)
            Flags(val) => {}, // u32
          }
        }
      }
    },
    Version(val) => {}, // u32
  }
}
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

### Dump (reply)

```rust
PushOpGetpolicyDumpReply::new(&mut vec)
  .push_family_id(val) // u16
  .nested_op_policy()
    .push_do(val) // u32
    .push_dump(val) // u32
  .end_nested()
  .nested_policy()

    // Associated type: "AttrType" (enum)
    .push_type(val) // u32
    .push_min_value_s(val) // i64
    .push_max_value_s(val) // i64
    .push_min_value_u(val) // u64
    .push_max_value_u(val) // u64
    .push_min_length(val) // u32
    .push_max_length(val) // u32
    .push_policy_idx(val) // u32
    .push_policy_maxtype(val) // u32
    .push_bitfield32_mask(val) // u32
    .push_mask(val) // u64
    .push_pad(val) // &[u8]
  .end_nested()
  ;
```

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

## Low-level decoding

### Dump (request)

```rust
let iter = OpGetpolicyDumpRequest::new(buf);
for attr in iter {
  match attr {
    FamilyName(val) => {}, // &CStr
    FamilyId(val) => {}, // u16
    Op(val) => {}, // u32
  }
}
```

### Dump (reply)

```rust
let iter = OpGetpolicyDumpReply::new(buf);
for attr in iter {
  match attr {
    FamilyId(val) => {}, // u16
    OpPolicy(iter) => {
      for attr in iter {
        match attr {
          Do(val) => {}, // u32
          Dump(val) => {}, // u32
        }
      }
    },
    Policy(iter) => {
      for attr in iter {
        match attr {

          // Associated type: "AttrType" (enum)
          Type(val) => {}, // u32
          MinValueS(val) => {}, // i64
          MaxValueS(val) => {}, // i64
          MinValueU(val) => {}, // u64
          MaxValueU(val) => {}, // u64
          MinLength(val) => {}, // u32
          MaxLength(val) => {}, // u32
          PolicyIdx(val) => {}, // u32
          PolicyMaxtype(val) => {}, // u32
          Bitfield32Mask(val) => {}, // u32
          Mask(val) => {}, // u64
          Pad(val) => {}, // &[u8]
        }
      }
    },
  }
}
```
