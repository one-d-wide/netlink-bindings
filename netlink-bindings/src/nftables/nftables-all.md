
# Operation "compat-get"

## Do (request)

```rust
PushOpCompatGetDoRequest::new(&mut vec)
  .push_name(val) // &CStr
  .push_name_bytes(val) // &[u8]
  .push_rev(val) // u32
  .push_type(val) // u32
  ;
```

```rust
let attrs = OpCompatGetDoReply::new(buf);

attrs.get_name(); // &CStr
attrs.get_rev(); // u32
attrs.get_type(); // u32
```

### Do (reply)

```rust
PushOpCompatGetDoReply::new(&mut vec)
  .push_name(val) // &CStr
  .push_name_bytes(val) // &[u8]
  .push_rev(val) // u32
  .push_type(val) // u32
  ;
```

```rust
let attrs = OpCompatGetDoReply::new(buf);

attrs.get_name(); // &CStr
attrs.get_rev(); // u32
attrs.get_type(); // u32
```

## Low-level decoding

### Do (request)

```rust
let iter = OpCompatGetDoRequest::new(buf);
for attr in iter {
  match attr {
    Name(val) => {}, // &CStr
    Rev(val) => {}, // u32
    Type(val) => {}, // u32
  }
}
```

### Do (reply)

```rust
let iter = OpCompatGetDoReply::new(buf);
for attr in iter {
  match attr {
    Name(val) => {}, // &CStr
    Rev(val) => {}, // u32
    Type(val) => {}, // u32
  }
}
```

## Dump (request)

```rust
PushOpCompatGetDumpRequest::new(&mut vec)
  ;
```

```rust
let attrs = OpCompatGetDumpReply::new(buf);

// No attributes
```

### Dump (reply)

```rust
PushOpCompatGetDumpReply::new(&mut vec)
  ;
```

```rust
let attrs = OpCompatGetDumpReply::new(buf);

// No attributes
```

## Low-level decoding

### Dump (request)

```rust
let iter = OpCompatGetDumpRequest::new(buf);
// No attributes
```

### Dump (reply)

```rust
let iter = OpCompatGetDumpReply::new(buf);
// No attributes
```

# Operation "batch-begin"

## Do (request)

```rust
PushOpBatchBeginDoRequest::new(&mut vec)
  .push_genid(val) // u32
  ;
```

```rust
let attrs = OpBatchBeginDoReply::new(buf);

attrs.get_genid(); // u32
```

### Do (reply)

```rust
PushOpBatchBeginDoReply::new(&mut vec)
  .push_genid(val) // u32
  ;
```

```rust
let attrs = OpBatchBeginDoReply::new(buf);

attrs.get_genid(); // u32
```

## Low-level decoding

### Do (request)

```rust
let iter = OpBatchBeginDoRequest::new(buf);
for attr in iter {
  match attr {
    Genid(val) => {}, // u32
  }
}
```

### Do (reply)

```rust
let iter = OpBatchBeginDoReply::new(buf);
for attr in iter {
  match attr {
    Genid(val) => {}, // u32
  }
}
```

# Operation "batch-end"

## Do (request)

```rust
PushOpBatchEndDoRequest::new(&mut vec)
  .push_genid(val) // u32
  ;
```

```rust
let attrs = OpBatchEndDoReply::new(buf);

// No attributes
```

### Do (reply)

```rust
PushOpBatchEndDoReply::new(&mut vec)
  ;
```

```rust
let attrs = OpBatchEndDoReply::new(buf);

// No attributes
```

## Low-level decoding

### Do (request)

```rust
let iter = OpBatchEndDoRequest::new(buf);
for attr in iter {
  match attr {
    Genid(val) => {}, // u32
  }
}
```

### Do (reply)

```rust
let iter = OpBatchEndDoReply::new(buf);
// No attributes
```

# Operation "newtable"

## Do (request)

```rust
PushOpNewtableDoRequest::new(&mut vec)

  // name of the table
  .push_name(val) // &CStr
  .push_name_bytes(val) // &[u8]
  ;
```

```rust
let attrs = OpNewtableDoReply::new(buf);

// No attributes
```

### Do (reply)

```rust
PushOpNewtableDoReply::new(&mut vec)
  ;
```

```rust
let attrs = OpNewtableDoReply::new(buf);

// No attributes
```

## Low-level decoding

### Do (request)

```rust
let iter = OpNewtableDoRequest::new(buf);
for attr in iter {
  match attr {

    // name of the table
    Name(val) => {}, // &CStr
  }
}
```

### Do (reply)

```rust
let iter = OpNewtableDoReply::new(buf);
// No attributes
```

# Operation "gettable"

## Do (request)

```rust
PushOpGettableDoRequest::new(&mut vec)
  ;
```

```rust
let attrs = OpGettableDoReply::new(buf);

// No attributes
```

### Do (reply)

```rust
PushOpGettableDoReply::new(&mut vec)
  ;
```

```rust
let attrs = OpGettableDoReply::new(buf);

// No attributes
```

## Low-level decoding

### Do (request)

```rust
let iter = OpGettableDoRequest::new(buf);
// No attributes
```

### Do (reply)

```rust
let iter = OpGettableDoReply::new(buf);
// No attributes
```

## Dump (request)

```rust
PushOpGettableDumpRequest::new(&mut vec)
  ;
```

```rust
let attrs = OpGettableDumpReply::new(buf);

// name of the table
attrs.get_name(); // &CStr

// number of chains in this table
attrs.get_use(); // u32

// numeric handle of the table
attrs.get_handle(); // u64

// bitmask of flags
// Associated type: "TableFlags" (1 bit per enumeration)
attrs.get_flags(); // u32
```

### Dump (reply)

```rust
PushOpGettableDumpReply::new(&mut vec)

  // name of the table
  .push_name(val) // &CStr
  .push_name_bytes(val) // &[u8]

  // number of chains in this table
  .push_use(val) // u32

  // numeric handle of the table
  .push_handle(val) // u64

  // bitmask of flags
  // Associated type: "TableFlags" (1 bit per enumeration)
  .push_flags(val) // u32
  ;
```

```rust
let attrs = OpGettableDumpReply::new(buf);

// name of the table
attrs.get_name(); // &CStr

// number of chains in this table
attrs.get_use(); // u32

// numeric handle of the table
attrs.get_handle(); // u64

// bitmask of flags
// Associated type: "TableFlags" (1 bit per enumeration)
attrs.get_flags(); // u32
```

## Low-level decoding

### Dump (request)

```rust
let iter = OpGettableDumpRequest::new(buf);
// No attributes
```

### Dump (reply)

```rust
let iter = OpGettableDumpReply::new(buf);
for attr in iter {
  match attr {

    // name of the table
    Name(val) => {}, // &CStr

    // number of chains in this table
    Use(val) => {}, // u32

    // numeric handle of the table
    Handle(val) => {}, // u64

    // bitmask of flags
    // Associated type: "TableFlags" (1 bit per enumeration)
    Flags(val) => {}, // u32
  }
}
```

# Operation "deltable"

## Do (request)

```rust
PushOpDeltableDoRequest::new(&mut vec)

  // name of the table
  .push_name(val) // &CStr
  .push_name_bytes(val) // &[u8]
  ;
```

```rust
let attrs = OpDeltableDoReply::new(buf);

// No attributes
```

### Do (reply)

```rust
PushOpDeltableDoReply::new(&mut vec)
  ;
```

```rust
let attrs = OpDeltableDoReply::new(buf);

// No attributes
```

## Low-level decoding

### Do (request)

```rust
let iter = OpDeltableDoRequest::new(buf);
for attr in iter {
  match attr {

    // name of the table
    Name(val) => {}, // &CStr
  }
}
```

### Do (reply)

```rust
let iter = OpDeltableDoReply::new(buf);
// No attributes
```

# Operation "destroytable"

## Do (request)

```rust
PushOpDestroytableDoRequest::new(&mut vec)

  // name of the table
  .push_name(val) // &CStr
  .push_name_bytes(val) // &[u8]
  ;
```

```rust
let attrs = OpDestroytableDoReply::new(buf);

// No attributes
```

### Do (reply)

```rust
PushOpDestroytableDoReply::new(&mut vec)
  ;
```

```rust
let attrs = OpDestroytableDoReply::new(buf);

// No attributes
```

## Low-level decoding

### Do (request)

```rust
let iter = OpDestroytableDoRequest::new(buf);
for attr in iter {
  match attr {

    // name of the table
    Name(val) => {}, // &CStr
  }
}
```

### Do (reply)

```rust
let iter = OpDestroytableDoReply::new(buf);
// No attributes
```

# Operation "newchain"

## Do (request)

```rust
PushOpNewchainDoRequest::new(&mut vec)

  // name of the chain
  .push_name(val) // &CStr
  .push_name_bytes(val) // &[u8]
  ;
```

```rust
let attrs = OpNewchainDoReply::new(buf);

// No attributes
```

### Do (reply)

```rust
PushOpNewchainDoReply::new(&mut vec)
  ;
```

```rust
let attrs = OpNewchainDoReply::new(buf);

// No attributes
```

## Low-level decoding

### Do (request)

```rust
let iter = OpNewchainDoRequest::new(buf);
for attr in iter {
  match attr {

    // name of the chain
    Name(val) => {}, // &CStr
  }
}
```

### Do (reply)

```rust
let iter = OpNewchainDoReply::new(buf);
// No attributes
```

# Operation "getchain"

## Do (request)

```rust
PushOpGetchainDoRequest::new(&mut vec)

  // name of the chain
  .push_name(val) // &CStr
  .push_name_bytes(val) // &[u8]
  ;
```

```rust
let attrs = OpGetchainDoReply::new(buf);

// name of the chain
attrs.get_name(); // &CStr
```

### Do (reply)

```rust
PushOpGetchainDoReply::new(&mut vec)

  // name of the chain
  .push_name(val) // &CStr
  .push_name_bytes(val) // &[u8]
  ;
```

```rust
let attrs = OpGetchainDoReply::new(buf);

// name of the chain
attrs.get_name(); // &CStr
```

## Low-level decoding

### Do (request)

```rust
let iter = OpGetchainDoRequest::new(buf);
for attr in iter {
  match attr {

    // name of the chain
    Name(val) => {}, // &CStr
  }
}
```

### Do (reply)

```rust
let iter = OpGetchainDoReply::new(buf);
for attr in iter {
  match attr {

    // name of the chain
    Name(val) => {}, // &CStr
  }
}
```

## Dump (request)

```rust
PushOpGetchainDumpRequest::new(&mut vec)
  ;
```

```rust
let attrs = OpGetchainDumpReply::new(buf);

// name of the table containing the chain
attrs.get_table(); // &CStr

// name of the chain
attrs.get_name(); // &CStr

// numeric handle of the chain
attrs.get_handle(); // u64
{ // Nested Hook

  // hook specification for basechains
  let attrs = attrs.get_hook();
  attrs.get_num(); // u32
  attrs.get_priority(); // i32

  // net device name
  attrs.get_dev(); // &CStr
  { // Nested Devs

    // list of net devices
    let attrs = attrs.get_devs();

    // Attribute may repeat multiple times (treat it as array)
    for entry in attrs.get_name() {
      entry; // &CStr
    }
  }
}

// numeric policy of the chain
attrs.get_policy(); // u32

// type name of the chain
attrs.get_type(); // &CStr
{ // Nested Counters

  // counter specification of the chain
  let attrs = attrs.get_counters();
  attrs.get_bytes(); // u64
  attrs.get_packets(); // u64
}

// uniquely identifies a chain in a transaction
attrs.get_id(); // u32

// number of references to this chain
attrs.get_use(); // u32
```

### Dump (reply)

```rust
PushOpGetchainDumpReply::new(&mut vec)

  // name of the table containing the chain
  .push_table(val) // &CStr
  .push_table_bytes(val) // &[u8]

  // name of the chain
  .push_name(val) // &CStr
  .push_name_bytes(val) // &[u8]

  // numeric handle of the chain
  .push_handle(val) // u64

  // hook specification for basechains
  .nested_hook()
    .push_num(val) // u32
    .push_priority(val) // i32

    // net device name
    .push_dev(val) // &CStr
    .push_dev_bytes(val) // &[u8]

    // list of net devices
    .nested_devs()

      // Attribute may repeat multiple times (treat it as array)
      .push_name(val) // &CStr
      .push_name_bytes(val) // &[u8]
    .end_nested()
  .end_nested()

  // numeric policy of the chain
  .push_policy(val) // u32

  // type name of the chain
  .push_type(val) // &CStr
  .push_type_bytes(val) // &[u8]

  // counter specification of the chain
  .nested_counters()
    .push_bytes(val) // u64
    .push_packets(val) // u64
  .end_nested()

  // uniquely identifies a chain in a transaction
  .push_id(val) // u32

  // number of references to this chain
  .push_use(val) // u32
  ;
```

```rust
let attrs = OpGetchainDumpReply::new(buf);

// name of the table containing the chain
attrs.get_table(); // &CStr

// name of the chain
attrs.get_name(); // &CStr

// numeric handle of the chain
attrs.get_handle(); // u64
{ // Nested Hook

  // hook specification for basechains
  let attrs = attrs.get_hook();
  attrs.get_num(); // u32
  attrs.get_priority(); // i32

  // net device name
  attrs.get_dev(); // &CStr
  { // Nested Devs

    // list of net devices
    let attrs = attrs.get_devs();

    // Attribute may repeat multiple times (treat it as array)
    for entry in attrs.get_name() {
      entry; // &CStr
    }
  }
}

// numeric policy of the chain
attrs.get_policy(); // u32

// type name of the chain
attrs.get_type(); // &CStr
{ // Nested Counters

  // counter specification of the chain
  let attrs = attrs.get_counters();
  attrs.get_bytes(); // u64
  attrs.get_packets(); // u64
}

// uniquely identifies a chain in a transaction
attrs.get_id(); // u32

// number of references to this chain
attrs.get_use(); // u32
```

## Low-level decoding

### Dump (request)

```rust
let iter = OpGetchainDumpRequest::new(buf);
// No attributes
```

### Dump (reply)

```rust
let iter = OpGetchainDumpReply::new(buf);
for attr in iter {
  match attr {

    // name of the table containing the chain
    Table(val) => {}, // &CStr

    // name of the chain
    Name(val) => {}, // &CStr

    // numeric handle of the chain
    Handle(val) => {}, // u64

    // hook specification for basechains
    Hook(iter) => {
      for attr in iter {
        match attr {
          Num(val) => {}, // u32
          Priority(val) => {}, // i32

          // net device name
          Dev(val) => {}, // &CStr

          // list of net devices
          Devs(iter) => {
            for attr in iter {
              match attr {

                // Attribute may repeat multiple times (treat it as array)
                Name(val) => {}, // &CStr
              }
            }
          },
        }
      }
    },

    // numeric policy of the chain
    Policy(val) => {}, // u32

    // type name of the chain
    Type(val) => {}, // &CStr

    // counter specification of the chain
    Counters(iter) => {
      for attr in iter {
        match attr {
          Bytes(val) => {}, // u64
          Packets(val) => {}, // u64
        }
      }
    },

    // uniquely identifies a chain in a transaction
    Id(val) => {}, // u32

    // number of references to this chain
    Use(val) => {}, // u32
  }
}
```

# Operation "delchain"

## Do (request)

```rust
PushOpDelchainDoRequest::new(&mut vec)

  // name of the chain
  .push_name(val) // &CStr
  .push_name_bytes(val) // &[u8]
  ;
```

```rust
let attrs = OpDelchainDoReply::new(buf);

// No attributes
```

### Do (reply)

```rust
PushOpDelchainDoReply::new(&mut vec)
  ;
```

```rust
let attrs = OpDelchainDoReply::new(buf);

// No attributes
```

## Low-level decoding

### Do (request)

```rust
let iter = OpDelchainDoRequest::new(buf);
for attr in iter {
  match attr {

    // name of the chain
    Name(val) => {}, // &CStr
  }
}
```

### Do (reply)

```rust
let iter = OpDelchainDoReply::new(buf);
// No attributes
```

# Operation "destroychain"

## Do (request)

```rust
PushOpDestroychainDoRequest::new(&mut vec)

  // name of the chain
  .push_name(val) // &CStr
  .push_name_bytes(val) // &[u8]
  ;
```

```rust
let attrs = OpDestroychainDoReply::new(buf);

// No attributes
```

### Do (reply)

```rust
PushOpDestroychainDoReply::new(&mut vec)
  ;
```

```rust
let attrs = OpDestroychainDoReply::new(buf);

// No attributes
```

## Low-level decoding

### Do (request)

```rust
let iter = OpDestroychainDoRequest::new(buf);
for attr in iter {
  match attr {

    // name of the chain
    Name(val) => {}, // &CStr
  }
}
```

### Do (reply)

```rust
let iter = OpDestroychainDoReply::new(buf);
// No attributes
```

# Operation "newrule"

## Do (request)

```rust
PushOpNewruleDoRequest::new(&mut vec)

  // name of the table containing the rule
  .push_table(val) // &CStr
  .push_table_bytes(val) // &[u8]

  // name of the chain containing the rule
  .push_chain(val) // &CStr
  .push_chain_bytes(val) // &[u8]

  // list of expressions
  .nested_expressions()

    // Attribute may repeat multiple times (treat it as array)
    .nested_elem()

      // name of the expression type
      .push_name(val) // &CStr
      .push_name_bytes(val) // &[u8]

      // type specific data
      .sub_nested_data_bitwise()
        .push_sreg(val) // u32
        .push_dreg(val) // u32
        .push_len(val) // u32
        .nested_mask()
          .push_value(val) // &[u8]
          .nested_verdict()

            // nf_tables verdict
            // Associated type: "VerdictCode" (enum)
            .push_code(val) // u32

            // jump target chain name
            .push_chain(val) // &CStr
            .push_chain_bytes(val) // &[u8]

            // jump target chain ID
            .push_chain_id(val) // u32
          .end_nested()
        .end_nested()
        .nested_xor()
          .push_value(val) // &[u8]
          .nested_verdict()

            // nf_tables verdict
            // Associated type: "VerdictCode" (enum)
            .push_code(val) // u32

            // jump target chain name
            .push_chain(val) // &CStr
            .push_chain_bytes(val) // &[u8]

            // jump target chain ID
            .push_chain_id(val) // u32
          .end_nested()
        .end_nested()

        // Associated type: "BitwiseOps" (enum)
        .push_op(val) // u32
        .nested_data()
          .push_value(val) // &[u8]
          .nested_verdict()

            // nf_tables verdict
            // Associated type: "VerdictCode" (enum)
            .push_code(val) // u32

            // jump target chain name
            .push_chain(val) // &CStr
            .push_chain_bytes(val) // &[u8]

            // jump target chain ID
            .push_chain_id(val) // u32
          .end_nested()
        .end_nested()
      .end_nested()
      .sub_nested_data_cmp()
        .push_sreg(val) // u32

        // Associated type: "CmpOps" (enum)
        .push_op(val) // u32
        .nested_data()
          .push_value(val) // &[u8]
          .nested_verdict()

            // nf_tables verdict
            // Associated type: "VerdictCode" (enum)
            .push_code(val) // u32

            // jump target chain name
            .push_chain(val) // &CStr
            .push_chain_bytes(val) // &[u8]

            // jump target chain ID
            .push_chain_id(val) // u32
          .end_nested()
        .end_nested()
      .end_nested()
      .sub_nested_data_counter()

        // Number of bytes
        .push_bytes(val) // u64

        // Number of packets
        .push_packets(val) // u64
        .push_pad(val) // &[u8]
      .end_nested()
      .sub_nested_data_ct()
        .push_dreg(val) // u32

        // Associated type: "CtKeys" (enum)
        .push_key(val) // u32

        // Associated type: "CtDirection" (enum)
        .push_direction(val) // u8
        .push_sreg(val) // u32
      .end_nested()
      .sub_nested_data_fib()
        .push_dreg(val) // u32

        // Associated type: "FibResult" (enum)
        .push_result(val) // u32

        // Associated type: "FibFlags" (enum)
        .push_flags(val) // u32
      .end_nested()
      .sub_nested_data_flow_offload()

        // Flow offload table name
        .push_name(val) // &CStr
        .push_name_bytes(val) // &[u8]
      .end_nested()
      .sub_nested_data_immediate()
        .push_dreg(val) // u32
        .nested_data()
          .push_value(val) // &[u8]
          .nested_verdict()

            // nf_tables verdict
            // Associated type: "VerdictCode" (enum)
            .push_code(val) // u32

            // jump target chain name
            .push_chain(val) // &CStr
            .push_chain_bytes(val) // &[u8]

            // jump target chain ID
            .push_chain_id(val) // u32
          .end_nested()
        .end_nested()
      .end_nested()
      .sub_nested_data_lookup()

        // Name of set to use
        .push_set(val) // &CStr
        .push_set_bytes(val) // &[u8]

        // ID of set to use
        .push_set_id(val) // u32
        .push_sreg(val) // u32
        .push_dreg(val) // u32

        // Associated type: "LookupFlags" (enum)
        .push_flags(val) // u32
      .end_nested()
      .sub_nested_data_meta()
        .push_dreg(val) // u32

        // Associated type: "MetaKeys" (enum)
        .push_key(val) // u32
        .push_sreg(val) // u32
      .end_nested()
      .sub_nested_data_nat()
        .push_type(val) // u32
        .push_family(val) // u32
        .push_reg_addr_min(val) // u32
        .push_reg_addr_max(val) // u32
        .push_reg_proto_min(val) // u32
        .push_reg_proto_max(val) // u32

        // Associated type: "NatRangeFlags" (1 bit per enumeration)
        .push_flags(val) // u32
      .end_nested()
      .sub_nested_data_objref()
        .push_imm_type(val) // u32

        // object name
        .push_imm_name(val) // &CStr
        .push_imm_name_bytes(val) // &[u8]
        .push_set_sreg(val) // u32

        // name of object map
        .push_set_name(val) // &CStr
        .push_set_name_bytes(val) // &[u8]

        // id of object map
        .push_set_id(val) // u32
      .end_nested()
      .sub_nested_data_payload()

        // destination register to load data into
        // Associated type: "Registers" (enum)
        .push_dreg(val) // u32

        // payload base
        // Associated type: "PayloadBase" (enum)
        .push_base(val) // u32

        // payload offset relative to base
        .push_offset(val) // u32

        // payload length
        .push_len(val) // u32

        // source register to load data from
        // Associated type: "Registers" (enum)
        .push_sreg(val) // u32

        // checksum type
        .push_csum_type(val) // u32

        // checksum offset relative to base
        .push_csum_offset(val) // u32

        // checksum flags
        .push_csum_flags(val) // u32
      .end_nested()
      .sub_nested_data_quota()
        .push_bytes(val) // u64

        // Associated type: "QuotaFlags" (enum)
        .push_flags(val) // u32
        .push_pad(val) // &[u8]
        .push_consumed(val) // u64
      .end_nested()
      .sub_nested_data_reject()

        // Associated type: "RejectTypes" (enum)
        .push_type(val) // u32
        .push_icmp_code(val) // u8
      .end_nested()
      .sub_nested_data_target()
        .push_name(val) // &CStr
        .push_name_bytes(val) // &[u8]
        .push_rev(val) // u32
        .push_info(val) // &[u8]
      .end_nested()
      .sub_nested_data_tproxy()
        .push_family(val) // u32
        .push_reg_addr(val) // u32
        .push_reg_port(val) // u32
      .end_nested()
      .sub_nested_data_match()
        .push_name(val) // &CStr
        .push_name_bytes(val) // &[u8]
        .push_rev(val) // u32
        .push_info(val) // &[u8]
      .end_nested()
      .sub_nested_data_range()

        // source register of data to compare
        // Associated type: "Registers" (enum)
        .push_sreg(val) // u32

        // cmp operation
        // Associated type: "RangeOps" (enum)
        .push_op(val) // u32

        // data range from
        .nested_from_data()
          .push_value(val) // &[u8]
          .nested_verdict()

            // nf_tables verdict
            // Associated type: "VerdictCode" (enum)
            .push_code(val) // u32

            // jump target chain name
            .push_chain(val) // &CStr
            .push_chain_bytes(val) // &[u8]

            // jump target chain ID
            .push_chain_id(val) // u32
          .end_nested()
        .end_nested()

        // data range to
        .nested_to_data()
          .push_value(val) // &[u8]
          .nested_verdict()

            // nf_tables verdict
            // Associated type: "VerdictCode" (enum)
            .push_code(val) // u32

            // jump target chain name
            .push_chain(val) // &CStr
            .push_chain_bytes(val) // &[u8]

            // jump target chain ID
            .push_chain_id(val) // u32
          .end_nested()
        .end_nested()
      .end_nested()
      .sub_nested_data_numgen()

        // destination register
        // Associated type: "Registers" (enum)
        .push_dreg(val) // u32

        // maximum counter value
        .push_modulus(val) // u32

        // operation type
        // Associated type: "NumgenTypes" (enum)
        .push_type(val) // u32

        // offset to be added to the counter
        .push_offset(val) // u32
      .end_nested()
      .sub_nested_data_log()

        // netlink group to send messages to
        .push_group(val) // u16

        // prefix to prepend to log messages
        .push_prefix(val) // &CStr
        .push_prefix_bytes(val) // &[u8]

        // length of payload to include in netlink message
        .push_snaplen(val) // u32

        // queue threshold
        .push_qthreshold(val) // u16

        // log level
        // Associated type: "LogLevel" (enum)
        .push_level(val) // u32

        // logging flags
        // Associated type: "LogFlags" (enum)
        .push_flags(val) // u32
      .end_nested()
    .end_nested()
  .end_nested()

  // compatibility specifications of the rule
  .nested_compat()

    // numeric value of the handled protocol
    .push_proto(val) // u32

    // bitmask of flags
    .push_flags(val) // u32
  .end_nested()
  ;
```

```rust
let attrs = OpNewruleDoReply::new(buf);

// No attributes
```

### Do (reply)

```rust
PushOpNewruleDoReply::new(&mut vec)
  ;
```

```rust
let attrs = OpNewruleDoReply::new(buf);

// No attributes
```

## Low-level decoding

### Do (request)

```rust
let iter = OpNewruleDoRequest::new(buf);
for attr in iter {
  match attr {

    // name of the table containing the rule
    Table(val) => {}, // &CStr

    // name of the chain containing the rule
    Chain(val) => {}, // &CStr

    // list of expressions
    Expressions(iter) => {
      for attr in iter {
        match attr {

          // Attribute may repeat multiple times (treat it as array)
          Elem(iter) => {
            for attr in iter {
              match attr {

                // name of the expression type
                Name(val) => {}, // &CStr

                // type specific data
                Data(val) => {}, // submessage
              }
            }
          },
        }
      }
    },

    // compatibility specifications of the rule
    Compat(iter) => {
      for attr in iter {
        match attr {

          // numeric value of the handled protocol
          Proto(val) => {}, // u32

          // bitmask of flags
          Flags(val) => {}, // u32
        }
      }
    },
  }
}
```

### Do (reply)

```rust
let iter = OpNewruleDoReply::new(buf);
// No attributes
```

# Operation "getrule"

## Do (request)

```rust
PushOpGetruleDoRequest::new(&mut vec)
  ;
```

```rust
let attrs = OpGetruleDoReply::new(buf);

// No attributes
```

### Do (reply)

```rust
PushOpGetruleDoReply::new(&mut vec)
  ;
```

```rust
let attrs = OpGetruleDoReply::new(buf);

// No attributes
```

## Low-level decoding

### Do (request)

```rust
let iter = OpGetruleDoRequest::new(buf);
// No attributes
```

### Do (reply)

```rust
let iter = OpGetruleDoReply::new(buf);
// No attributes
```

## Dump (request)

```rust
PushOpGetruleDumpRequest::new(&mut vec)

  // name of the table containing the rule
  .push_table(val) // &CStr
  .push_table_bytes(val) // &[u8]

  // name of the chain containing the rule
  .push_chain(val) // &CStr
  .push_chain_bytes(val) // &[u8]
  ;
```

```rust
let attrs = OpGetruleDumpReply::new(buf);

// name of the table containing the rule
attrs.get_table(); // &CStr

// name of the chain containing the rule
attrs.get_chain(); // &CStr

// numeric handle of the rule
attrs.get_handle(); // u64

// numeric handle of the previous rule
attrs.get_position(); // u64
{ // Nested Expressions

  // list of expressions
  let attrs = attrs.get_expressions();
  { // Nested Elem

    // Attribute may repeat multiple times (treat it as array)
    for entry in attrs.get_elem() {

      // name of the expression type
      entry.get_name(); // &CStr

      // type specific data
      entry.get_data(); // submessage
    }
  }
}
```

### Dump (reply)

```rust
PushOpGetruleDumpReply::new(&mut vec)

  // name of the table containing the rule
  .push_table(val) // &CStr
  .push_table_bytes(val) // &[u8]

  // name of the chain containing the rule
  .push_chain(val) // &CStr
  .push_chain_bytes(val) // &[u8]

  // numeric handle of the rule
  .push_handle(val) // u64

  // numeric handle of the previous rule
  .push_position(val) // u64

  // list of expressions
  .nested_expressions()

    // Attribute may repeat multiple times (treat it as array)
    .nested_elem()

      // name of the expression type
      .push_name(val) // &CStr
      .push_name_bytes(val) // &[u8]

      // type specific data
      .sub_nested_data_bitwise()
        .push_sreg(val) // u32
        .push_dreg(val) // u32
        .push_len(val) // u32
        .nested_mask()
          .push_value(val) // &[u8]
          .nested_verdict()

            // nf_tables verdict
            // Associated type: "VerdictCode" (enum)
            .push_code(val) // u32

            // jump target chain name
            .push_chain(val) // &CStr
            .push_chain_bytes(val) // &[u8]

            // jump target chain ID
            .push_chain_id(val) // u32
          .end_nested()
        .end_nested()
        .nested_xor()
          .push_value(val) // &[u8]
          .nested_verdict()

            // nf_tables verdict
            // Associated type: "VerdictCode" (enum)
            .push_code(val) // u32

            // jump target chain name
            .push_chain(val) // &CStr
            .push_chain_bytes(val) // &[u8]

            // jump target chain ID
            .push_chain_id(val) // u32
          .end_nested()
        .end_nested()

        // Associated type: "BitwiseOps" (enum)
        .push_op(val) // u32
        .nested_data()
          .push_value(val) // &[u8]
          .nested_verdict()

            // nf_tables verdict
            // Associated type: "VerdictCode" (enum)
            .push_code(val) // u32

            // jump target chain name
            .push_chain(val) // &CStr
            .push_chain_bytes(val) // &[u8]

            // jump target chain ID
            .push_chain_id(val) // u32
          .end_nested()
        .end_nested()
      .end_nested()
      .sub_nested_data_cmp()
        .push_sreg(val) // u32

        // Associated type: "CmpOps" (enum)
        .push_op(val) // u32
        .nested_data()
          .push_value(val) // &[u8]
          .nested_verdict()

            // nf_tables verdict
            // Associated type: "VerdictCode" (enum)
            .push_code(val) // u32

            // jump target chain name
            .push_chain(val) // &CStr
            .push_chain_bytes(val) // &[u8]

            // jump target chain ID
            .push_chain_id(val) // u32
          .end_nested()
        .end_nested()
      .end_nested()
      .sub_nested_data_counter()

        // Number of bytes
        .push_bytes(val) // u64

        // Number of packets
        .push_packets(val) // u64
        .push_pad(val) // &[u8]
      .end_nested()
      .sub_nested_data_ct()
        .push_dreg(val) // u32

        // Associated type: "CtKeys" (enum)
        .push_key(val) // u32

        // Associated type: "CtDirection" (enum)
        .push_direction(val) // u8
        .push_sreg(val) // u32
      .end_nested()
      .sub_nested_data_fib()
        .push_dreg(val) // u32

        // Associated type: "FibResult" (enum)
        .push_result(val) // u32

        // Associated type: "FibFlags" (enum)
        .push_flags(val) // u32
      .end_nested()
      .sub_nested_data_flow_offload()

        // Flow offload table name
        .push_name(val) // &CStr
        .push_name_bytes(val) // &[u8]
      .end_nested()
      .sub_nested_data_immediate()
        .push_dreg(val) // u32
        .nested_data()
          .push_value(val) // &[u8]
          .nested_verdict()

            // nf_tables verdict
            // Associated type: "VerdictCode" (enum)
            .push_code(val) // u32

            // jump target chain name
            .push_chain(val) // &CStr
            .push_chain_bytes(val) // &[u8]

            // jump target chain ID
            .push_chain_id(val) // u32
          .end_nested()
        .end_nested()
      .end_nested()
      .sub_nested_data_lookup()

        // Name of set to use
        .push_set(val) // &CStr
        .push_set_bytes(val) // &[u8]

        // ID of set to use
        .push_set_id(val) // u32
        .push_sreg(val) // u32
        .push_dreg(val) // u32

        // Associated type: "LookupFlags" (enum)
        .push_flags(val) // u32
      .end_nested()
      .sub_nested_data_meta()
        .push_dreg(val) // u32

        // Associated type: "MetaKeys" (enum)
        .push_key(val) // u32
        .push_sreg(val) // u32
      .end_nested()
      .sub_nested_data_nat()
        .push_type(val) // u32
        .push_family(val) // u32
        .push_reg_addr_min(val) // u32
        .push_reg_addr_max(val) // u32
        .push_reg_proto_min(val) // u32
        .push_reg_proto_max(val) // u32

        // Associated type: "NatRangeFlags" (1 bit per enumeration)
        .push_flags(val) // u32
      .end_nested()
      .sub_nested_data_objref()
        .push_imm_type(val) // u32

        // object name
        .push_imm_name(val) // &CStr
        .push_imm_name_bytes(val) // &[u8]
        .push_set_sreg(val) // u32

        // name of object map
        .push_set_name(val) // &CStr
        .push_set_name_bytes(val) // &[u8]

        // id of object map
        .push_set_id(val) // u32
      .end_nested()
      .sub_nested_data_payload()

        // destination register to load data into
        // Associated type: "Registers" (enum)
        .push_dreg(val) // u32

        // payload base
        // Associated type: "PayloadBase" (enum)
        .push_base(val) // u32

        // payload offset relative to base
        .push_offset(val) // u32

        // payload length
        .push_len(val) // u32

        // source register to load data from
        // Associated type: "Registers" (enum)
        .push_sreg(val) // u32

        // checksum type
        .push_csum_type(val) // u32

        // checksum offset relative to base
        .push_csum_offset(val) // u32

        // checksum flags
        .push_csum_flags(val) // u32
      .end_nested()
      .sub_nested_data_quota()
        .push_bytes(val) // u64

        // Associated type: "QuotaFlags" (enum)
        .push_flags(val) // u32
        .push_pad(val) // &[u8]
        .push_consumed(val) // u64
      .end_nested()
      .sub_nested_data_reject()

        // Associated type: "RejectTypes" (enum)
        .push_type(val) // u32
        .push_icmp_code(val) // u8
      .end_nested()
      .sub_nested_data_target()
        .push_name(val) // &CStr
        .push_name_bytes(val) // &[u8]
        .push_rev(val) // u32
        .push_info(val) // &[u8]
      .end_nested()
      .sub_nested_data_tproxy()
        .push_family(val) // u32
        .push_reg_addr(val) // u32
        .push_reg_port(val) // u32
      .end_nested()
      .sub_nested_data_match()
        .push_name(val) // &CStr
        .push_name_bytes(val) // &[u8]
        .push_rev(val) // u32
        .push_info(val) // &[u8]
      .end_nested()
      .sub_nested_data_range()

        // source register of data to compare
        // Associated type: "Registers" (enum)
        .push_sreg(val) // u32

        // cmp operation
        // Associated type: "RangeOps" (enum)
        .push_op(val) // u32

        // data range from
        .nested_from_data()
          .push_value(val) // &[u8]
          .nested_verdict()

            // nf_tables verdict
            // Associated type: "VerdictCode" (enum)
            .push_code(val) // u32

            // jump target chain name
            .push_chain(val) // &CStr
            .push_chain_bytes(val) // &[u8]

            // jump target chain ID
            .push_chain_id(val) // u32
          .end_nested()
        .end_nested()

        // data range to
        .nested_to_data()
          .push_value(val) // &[u8]
          .nested_verdict()

            // nf_tables verdict
            // Associated type: "VerdictCode" (enum)
            .push_code(val) // u32

            // jump target chain name
            .push_chain(val) // &CStr
            .push_chain_bytes(val) // &[u8]

            // jump target chain ID
            .push_chain_id(val) // u32
          .end_nested()
        .end_nested()
      .end_nested()
      .sub_nested_data_numgen()

        // destination register
        // Associated type: "Registers" (enum)
        .push_dreg(val) // u32

        // maximum counter value
        .push_modulus(val) // u32

        // operation type
        // Associated type: "NumgenTypes" (enum)
        .push_type(val) // u32

        // offset to be added to the counter
        .push_offset(val) // u32
      .end_nested()
      .sub_nested_data_log()

        // netlink group to send messages to
        .push_group(val) // u16

        // prefix to prepend to log messages
        .push_prefix(val) // &CStr
        .push_prefix_bytes(val) // &[u8]

        // length of payload to include in netlink message
        .push_snaplen(val) // u32

        // queue threshold
        .push_qthreshold(val) // u16

        // log level
        // Associated type: "LogLevel" (enum)
        .push_level(val) // u32

        // logging flags
        // Associated type: "LogFlags" (enum)
        .push_flags(val) // u32
      .end_nested()
    .end_nested()
  .end_nested()
  ;
```

```rust
let attrs = OpGetruleDumpReply::new(buf);

// name of the table containing the rule
attrs.get_table(); // &CStr

// name of the chain containing the rule
attrs.get_chain(); // &CStr

// numeric handle of the rule
attrs.get_handle(); // u64

// numeric handle of the previous rule
attrs.get_position(); // u64
{ // Nested Expressions

  // list of expressions
  let attrs = attrs.get_expressions();
  { // Nested Elem

    // Attribute may repeat multiple times (treat it as array)
    for entry in attrs.get_elem() {

      // name of the expression type
      entry.get_name(); // &CStr

      // type specific data
      entry.get_data(); // submessage
    }
  }
}
```

## Low-level decoding

### Dump (request)

```rust
let iter = OpGetruleDumpRequest::new(buf);
for attr in iter {
  match attr {

    // name of the table containing the rule
    Table(val) => {}, // &CStr

    // name of the chain containing the rule
    Chain(val) => {}, // &CStr
  }
}
```

### Dump (reply)

```rust
let iter = OpGetruleDumpReply::new(buf);
for attr in iter {
  match attr {

    // name of the table containing the rule
    Table(val) => {}, // &CStr

    // name of the chain containing the rule
    Chain(val) => {}, // &CStr

    // numeric handle of the rule
    Handle(val) => {}, // u64

    // numeric handle of the previous rule
    Position(val) => {}, // u64

    // list of expressions
    Expressions(iter) => {
      for attr in iter {
        match attr {

          // Attribute may repeat multiple times (treat it as array)
          Elem(iter) => {
            for attr in iter {
              match attr {

                // name of the expression type
                Name(val) => {}, // &CStr

                // type specific data
                Data(val) => {}, // submessage
              }
            }
          },
        }
      }
    },
  }
}
```

# Operation "getrule-reset"

## Do (request)

```rust
PushOpGetruleResetDoRequest::new(&mut vec)
  ;
```

```rust
let attrs = OpGetruleResetDoReply::new(buf);

// No attributes
```

### Do (reply)

```rust
PushOpGetruleResetDoReply::new(&mut vec)
  ;
```

```rust
let attrs = OpGetruleResetDoReply::new(buf);

// No attributes
```

## Low-level decoding

### Do (request)

```rust
let iter = OpGetruleResetDoRequest::new(buf);
// No attributes
```

### Do (reply)

```rust
let iter = OpGetruleResetDoReply::new(buf);
// No attributes
```

## Dump (request)

```rust
PushOpGetruleResetDumpRequest::new(&mut vec)
  ;
```

```rust
let attrs = OpGetruleResetDumpReply::new(buf);

// No attributes
```

### Dump (reply)

```rust
PushOpGetruleResetDumpReply::new(&mut vec)
  ;
```

```rust
let attrs = OpGetruleResetDumpReply::new(buf);

// No attributes
```

## Low-level decoding

### Dump (request)

```rust
let iter = OpGetruleResetDumpRequest::new(buf);
// No attributes
```

### Dump (reply)

```rust
let iter = OpGetruleResetDumpReply::new(buf);
// No attributes
```

# Operation "delrule"

## Do (request)

```rust
PushOpDelruleDoRequest::new(&mut vec)
  ;
```

```rust
let attrs = OpDelruleDoReply::new(buf);

// No attributes
```

### Do (reply)

```rust
PushOpDelruleDoReply::new(&mut vec)
  ;
```

```rust
let attrs = OpDelruleDoReply::new(buf);

// No attributes
```

## Low-level decoding

### Do (request)

```rust
let iter = OpDelruleDoRequest::new(buf);
// No attributes
```

### Do (reply)

```rust
let iter = OpDelruleDoReply::new(buf);
// No attributes
```

# Operation "destroyrule"

## Do (request)

```rust
PushOpDestroyruleDoRequest::new(&mut vec)
  ;
```

```rust
let attrs = OpDestroyruleDoReply::new(buf);

// No attributes
```

### Do (reply)

```rust
PushOpDestroyruleDoReply::new(&mut vec)
  ;
```

```rust
let attrs = OpDestroyruleDoReply::new(buf);

// No attributes
```

## Low-level decoding

### Do (request)

```rust
let iter = OpDestroyruleDoRequest::new(buf);
// No attributes
```

### Do (reply)

```rust
let iter = OpDestroyruleDoReply::new(buf);
// No attributes
```

# Operation "newset"

## Do (request)

```rust
PushOpNewsetDoRequest::new(&mut vec)
  ;
```

```rust
let attrs = OpNewsetDoReply::new(buf);

// No attributes
```

### Do (reply)

```rust
PushOpNewsetDoReply::new(&mut vec)
  ;
```

```rust
let attrs = OpNewsetDoReply::new(buf);

// No attributes
```

## Low-level decoding

### Do (request)

```rust
let iter = OpNewsetDoRequest::new(buf);
// No attributes
```

### Do (reply)

```rust
let iter = OpNewsetDoReply::new(buf);
// No attributes
```

# Operation "getset"

## Do (request)

```rust
PushOpGetsetDoRequest::new(&mut vec)
  ;
```

```rust
let attrs = OpGetsetDoReply::new(buf);

// No attributes
```

### Do (reply)

```rust
PushOpGetsetDoReply::new(&mut vec)
  ;
```

```rust
let attrs = OpGetsetDoReply::new(buf);

// No attributes
```

## Low-level decoding

### Do (request)

```rust
let iter = OpGetsetDoRequest::new(buf);
// No attributes
```

### Do (reply)

```rust
let iter = OpGetsetDoReply::new(buf);
// No attributes
```

## Dump (request)

```rust
PushOpGetsetDumpRequest::new(&mut vec)

  // table name
  .push_table(val) // &CStr
  .push_table_bytes(val) // &[u8]
  ;
```

```rust
let attrs = OpGetsetDumpReply::new(buf);

// No attributes
```

### Dump (reply)

```rust
PushOpGetsetDumpReply::new(&mut vec)
  ;
```

```rust
let attrs = OpGetsetDumpReply::new(buf);

// No attributes
```

## Low-level decoding

### Dump (request)

```rust
let iter = OpGetsetDumpRequest::new(buf);
for attr in iter {
  match attr {

    // table name
    Table(val) => {}, // &CStr
  }
}
```

### Dump (reply)

```rust
let iter = OpGetsetDumpReply::new(buf);
// No attributes
```

# Operation "delset"

## Do (request)

```rust
PushOpDelsetDoRequest::new(&mut vec)

  // set name
  .push_name(val) // &CStr
  .push_name_bytes(val) // &[u8]
  ;
```

```rust
let attrs = OpDelsetDoReply::new(buf);

// No attributes
```

### Do (reply)

```rust
PushOpDelsetDoReply::new(&mut vec)
  ;
```

```rust
let attrs = OpDelsetDoReply::new(buf);

// No attributes
```

## Low-level decoding

### Do (request)

```rust
let iter = OpDelsetDoRequest::new(buf);
for attr in iter {
  match attr {

    // set name
    Name(val) => {}, // &CStr
  }
}
```

### Do (reply)

```rust
let iter = OpDelsetDoReply::new(buf);
// No attributes
```

# Operation "destroyset"

## Do (request)

```rust
PushOpDestroysetDoRequest::new(&mut vec)

  // set name
  .push_name(val) // &CStr
  .push_name_bytes(val) // &[u8]
  ;
```

```rust
let attrs = OpDestroysetDoReply::new(buf);

// No attributes
```

### Do (reply)

```rust
PushOpDestroysetDoReply::new(&mut vec)
  ;
```

```rust
let attrs = OpDestroysetDoReply::new(buf);

// No attributes
```

## Low-level decoding

### Do (request)

```rust
let iter = OpDestroysetDoRequest::new(buf);
for attr in iter {
  match attr {

    // set name
    Name(val) => {}, // &CStr
  }
}
```

### Do (reply)

```rust
let iter = OpDestroysetDoReply::new(buf);
// No attributes
```

# Operation "newsetelem"

## Do (request)

```rust
PushOpNewsetelemDoRequest::new(&mut vec)
  ;
```

```rust
let attrs = OpNewsetelemDoReply::new(buf);

// No attributes
```

### Do (reply)

```rust
PushOpNewsetelemDoReply::new(&mut vec)
  ;
```

```rust
let attrs = OpNewsetelemDoReply::new(buf);

// No attributes
```

## Low-level decoding

### Do (request)

```rust
let iter = OpNewsetelemDoRequest::new(buf);
// No attributes
```

### Do (reply)

```rust
let iter = OpNewsetelemDoReply::new(buf);
// No attributes
```

# Operation "getsetelem"

## Do (request)

```rust
PushOpGetsetelemDoRequest::new(&mut vec)
  ;
```

```rust
let attrs = OpGetsetelemDoReply::new(buf);

// No attributes
```

### Do (reply)

```rust
PushOpGetsetelemDoReply::new(&mut vec)
  ;
```

```rust
let attrs = OpGetsetelemDoReply::new(buf);

// No attributes
```

## Low-level decoding

### Do (request)

```rust
let iter = OpGetsetelemDoRequest::new(buf);
// No attributes
```

### Do (reply)

```rust
let iter = OpGetsetelemDoReply::new(buf);
// No attributes
```

## Dump (request)

```rust
PushOpGetsetelemDumpRequest::new(&mut vec)
  ;
```

```rust
let attrs = OpGetsetelemDumpReply::new(buf);

// No attributes
```

### Dump (reply)

```rust
PushOpGetsetelemDumpReply::new(&mut vec)
  ;
```

```rust
let attrs = OpGetsetelemDumpReply::new(buf);

// No attributes
```

## Low-level decoding

### Dump (request)

```rust
let iter = OpGetsetelemDumpRequest::new(buf);
// No attributes
```

### Dump (reply)

```rust
let iter = OpGetsetelemDumpReply::new(buf);
// No attributes
```

# Operation "getsetelem-reset"

## Do (request)

```rust
PushOpGetsetelemResetDoRequest::new(&mut vec)
  ;
```

```rust
let attrs = OpGetsetelemResetDoReply::new(buf);

// No attributes
```

### Do (reply)

```rust
PushOpGetsetelemResetDoReply::new(&mut vec)
  ;
```

```rust
let attrs = OpGetsetelemResetDoReply::new(buf);

// No attributes
```

## Low-level decoding

### Do (request)

```rust
let iter = OpGetsetelemResetDoRequest::new(buf);
// No attributes
```

### Do (reply)

```rust
let iter = OpGetsetelemResetDoReply::new(buf);
// No attributes
```

## Dump (request)

```rust
PushOpGetsetelemResetDumpRequest::new(&mut vec)
  ;
```

```rust
let attrs = OpGetsetelemResetDumpReply::new(buf);

// No attributes
```

### Dump (reply)

```rust
PushOpGetsetelemResetDumpReply::new(&mut vec)
  ;
```

```rust
let attrs = OpGetsetelemResetDumpReply::new(buf);

// No attributes
```

## Low-level decoding

### Dump (request)

```rust
let iter = OpGetsetelemResetDumpRequest::new(buf);
// No attributes
```

### Dump (reply)

```rust
let iter = OpGetsetelemResetDumpReply::new(buf);
// No attributes
```

# Operation "delsetelem"

## Do (request)

```rust
PushOpDelsetelemDoRequest::new(&mut vec)
  ;
```

```rust
let attrs = OpDelsetelemDoReply::new(buf);

// No attributes
```

### Do (reply)

```rust
PushOpDelsetelemDoReply::new(&mut vec)
  ;
```

```rust
let attrs = OpDelsetelemDoReply::new(buf);

// No attributes
```

## Low-level decoding

### Do (request)

```rust
let iter = OpDelsetelemDoRequest::new(buf);
// No attributes
```

### Do (reply)

```rust
let iter = OpDelsetelemDoReply::new(buf);
// No attributes
```

# Operation "destroysetelem"

## Do (request)

```rust
PushOpDestroysetelemDoRequest::new(&mut vec)
  ;
```

```rust
let attrs = OpDestroysetelemDoReply::new(buf);

// No attributes
```

### Do (reply)

```rust
PushOpDestroysetelemDoReply::new(&mut vec)
  ;
```

```rust
let attrs = OpDestroysetelemDoReply::new(buf);

// No attributes
```

## Low-level decoding

### Do (request)

```rust
let iter = OpDestroysetelemDoRequest::new(buf);
// No attributes
```

### Do (reply)

```rust
let iter = OpDestroysetelemDoReply::new(buf);
// No attributes
```

# Operation "getgen"

## Do (request)

```rust
PushOpGetgenDoRequest::new(&mut vec)
  ;
```

```rust
let attrs = OpGetgenDoReply::new(buf);

// ruleset generation id
attrs.get_id(); // u32
attrs.get_proc_pid(); // u32
attrs.get_proc_name(); // &CStr
```

### Do (reply)

```rust
PushOpGetgenDoReply::new(&mut vec)

  // ruleset generation id
  .push_id(val) // u32
  .push_proc_pid(val) // u32
  .push_proc_name(val) // &CStr
  .push_proc_name_bytes(val) // &[u8]
  ;
```

```rust
let attrs = OpGetgenDoReply::new(buf);

// ruleset generation id
attrs.get_id(); // u32
attrs.get_proc_pid(); // u32
attrs.get_proc_name(); // &CStr
```

## Low-level decoding

### Do (request)

```rust
let iter = OpGetgenDoRequest::new(buf);
// No attributes
```

### Do (reply)

```rust
let iter = OpGetgenDoReply::new(buf);
for attr in iter {
  match attr {

    // ruleset generation id
    Id(val) => {}, // u32
    ProcPid(val) => {}, // u32
    ProcName(val) => {}, // &CStr
  }
}
```

## Dump (request)

```rust
PushOpGetgenDumpRequest::new(&mut vec)
  ;
```

```rust
let attrs = OpGetgenDumpReply::new(buf);

// No attributes
```

### Dump (reply)

```rust
PushOpGetgenDumpReply::new(&mut vec)
  ;
```

```rust
let attrs = OpGetgenDumpReply::new(buf);

// No attributes
```

## Low-level decoding

### Dump (request)

```rust
let iter = OpGetgenDumpRequest::new(buf);
// No attributes
```

### Dump (reply)

```rust
let iter = OpGetgenDumpReply::new(buf);
// No attributes
```

# Operation "newobj"

## Do (request)

```rust
PushOpNewobjDoRequest::new(&mut vec)

  // name of this expression type
  .push_name(val) // &CStr
  .push_name_bytes(val) // &[u8]
  ;
```

```rust
let attrs = OpNewobjDoReply::new(buf);

// No attributes
```

### Do (reply)

```rust
PushOpNewobjDoReply::new(&mut vec)
  ;
```

```rust
let attrs = OpNewobjDoReply::new(buf);

// No attributes
```

## Low-level decoding

### Do (request)

```rust
let iter = OpNewobjDoRequest::new(buf);
for attr in iter {
  match attr {

    // name of this expression type
    Name(val) => {}, // &CStr
  }
}
```

### Do (reply)

```rust
let iter = OpNewobjDoReply::new(buf);
// No attributes
```

# Operation "getobj"

## Do (request)

```rust
PushOpGetobjDoRequest::new(&mut vec)

  // name of this expression type
  .push_name(val) // &CStr
  .push_name_bytes(val) // &[u8]
  ;
```

```rust
let attrs = OpGetobjDoReply::new(buf);

// name of this expression type
attrs.get_name(); // &CStr
```

### Do (reply)

```rust
PushOpGetobjDoReply::new(&mut vec)

  // name of this expression type
  .push_name(val) // &CStr
  .push_name_bytes(val) // &[u8]
  ;
```

```rust
let attrs = OpGetobjDoReply::new(buf);

// name of this expression type
attrs.get_name(); // &CStr
```

## Low-level decoding

### Do (request)

```rust
let iter = OpGetobjDoRequest::new(buf);
for attr in iter {
  match attr {

    // name of this expression type
    Name(val) => {}, // &CStr
  }
}
```

### Do (reply)

```rust
let iter = OpGetobjDoReply::new(buf);
for attr in iter {
  match attr {

    // name of this expression type
    Name(val) => {}, // &CStr
  }
}
```

## Dump (request)

```rust
PushOpGetobjDumpRequest::new(&mut vec)
  ;
```

```rust
let attrs = OpGetobjDumpReply::new(buf);

// No attributes
```

### Dump (reply)

```rust
PushOpGetobjDumpReply::new(&mut vec)
  ;
```

```rust
let attrs = OpGetobjDumpReply::new(buf);

// No attributes
```

## Low-level decoding

### Dump (request)

```rust
let iter = OpGetobjDumpRequest::new(buf);
// No attributes
```

### Dump (reply)

```rust
let iter = OpGetobjDumpReply::new(buf);
// No attributes
```

# Operation "delobj"

## Do (request)

```rust
PushOpDelobjDoRequest::new(&mut vec)

  // name of this expression type
  .push_name(val) // &CStr
  .push_name_bytes(val) // &[u8]
  ;
```

```rust
let attrs = OpDelobjDoReply::new(buf);

// No attributes
```

### Do (reply)

```rust
PushOpDelobjDoReply::new(&mut vec)
  ;
```

```rust
let attrs = OpDelobjDoReply::new(buf);

// No attributes
```

## Low-level decoding

### Do (request)

```rust
let iter = OpDelobjDoRequest::new(buf);
for attr in iter {
  match attr {

    // name of this expression type
    Name(val) => {}, // &CStr
  }
}
```

### Do (reply)

```rust
let iter = OpDelobjDoReply::new(buf);
// No attributes
```

# Operation "destroyobj"

## Do (request)

```rust
PushOpDestroyobjDoRequest::new(&mut vec)

  // name of this expression type
  .push_name(val) // &CStr
  .push_name_bytes(val) // &[u8]
  ;
```

```rust
let attrs = OpDestroyobjDoReply::new(buf);

// No attributes
```

### Do (reply)

```rust
PushOpDestroyobjDoReply::new(&mut vec)
  ;
```

```rust
let attrs = OpDestroyobjDoReply::new(buf);

// No attributes
```

## Low-level decoding

### Do (request)

```rust
let iter = OpDestroyobjDoRequest::new(buf);
for attr in iter {
  match attr {

    // name of this expression type
    Name(val) => {}, // &CStr
  }
}
```

### Do (reply)

```rust
let iter = OpDestroyobjDoReply::new(buf);
// No attributes
```

# Operation "newflowtable"

## Do (request)

```rust
PushOpNewflowtableDoRequest::new(&mut vec)
  .push_name(val) // &CStr
  .push_name_bytes(val) // &[u8]
  ;
```

```rust
let attrs = OpNewflowtableDoReply::new(buf);

// No attributes
```

### Do (reply)

```rust
PushOpNewflowtableDoReply::new(&mut vec)
  ;
```

```rust
let attrs = OpNewflowtableDoReply::new(buf);

// No attributes
```

## Low-level decoding

### Do (request)

```rust
let iter = OpNewflowtableDoRequest::new(buf);
for attr in iter {
  match attr {
    Name(val) => {}, // &CStr
  }
}
```

### Do (reply)

```rust
let iter = OpNewflowtableDoReply::new(buf);
// No attributes
```

# Operation "getflowtable"

## Do (request)

```rust
PushOpGetflowtableDoRequest::new(&mut vec)
  .push_name(val) // &CStr
  .push_name_bytes(val) // &[u8]
  ;
```

```rust
let attrs = OpGetflowtableDoReply::new(buf);

attrs.get_name(); // &CStr
```

### Do (reply)

```rust
PushOpGetflowtableDoReply::new(&mut vec)
  .push_name(val) // &CStr
  .push_name_bytes(val) // &[u8]
  ;
```

```rust
let attrs = OpGetflowtableDoReply::new(buf);

attrs.get_name(); // &CStr
```

## Low-level decoding

### Do (request)

```rust
let iter = OpGetflowtableDoRequest::new(buf);
for attr in iter {
  match attr {
    Name(val) => {}, // &CStr
  }
}
```

### Do (reply)

```rust
let iter = OpGetflowtableDoReply::new(buf);
for attr in iter {
  match attr {
    Name(val) => {}, // &CStr
  }
}
```

## Dump (request)

```rust
PushOpGetflowtableDumpRequest::new(&mut vec)
  ;
```

```rust
let attrs = OpGetflowtableDumpReply::new(buf);

// No attributes
```

### Dump (reply)

```rust
PushOpGetflowtableDumpReply::new(&mut vec)
  ;
```

```rust
let attrs = OpGetflowtableDumpReply::new(buf);

// No attributes
```

## Low-level decoding

### Dump (request)

```rust
let iter = OpGetflowtableDumpRequest::new(buf);
// No attributes
```

### Dump (reply)

```rust
let iter = OpGetflowtableDumpReply::new(buf);
// No attributes
```

# Operation "delflowtable"

## Do (request)

```rust
PushOpDelflowtableDoRequest::new(&mut vec)
  .push_name(val) // &CStr
  .push_name_bytes(val) // &[u8]
  ;
```

```rust
let attrs = OpDelflowtableDoReply::new(buf);

// No attributes
```

### Do (reply)

```rust
PushOpDelflowtableDoReply::new(&mut vec)
  ;
```

```rust
let attrs = OpDelflowtableDoReply::new(buf);

// No attributes
```

## Low-level decoding

### Do (request)

```rust
let iter = OpDelflowtableDoRequest::new(buf);
for attr in iter {
  match attr {
    Name(val) => {}, // &CStr
  }
}
```

### Do (reply)

```rust
let iter = OpDelflowtableDoReply::new(buf);
// No attributes
```

# Operation "destroyflowtable"

## Do (request)

```rust
PushOpDestroyflowtableDoRequest::new(&mut vec)
  .push_name(val) // &CStr
  .push_name_bytes(val) // &[u8]
  ;
```

```rust
let attrs = OpDestroyflowtableDoReply::new(buf);

// No attributes
```

### Do (reply)

```rust
PushOpDestroyflowtableDoReply::new(&mut vec)
  ;
```

```rust
let attrs = OpDestroyflowtableDoReply::new(buf);

// No attributes
```

## Low-level decoding

### Do (request)

```rust
let iter = OpDestroyflowtableDoRequest::new(buf);
for attr in iter {
  match attr {
    Name(val) => {}, // &CStr
  }
}
```

### Do (reply)

```rust
let iter = OpDestroyflowtableDoReply::new(buf);
// No attributes
```
