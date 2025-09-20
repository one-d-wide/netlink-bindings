## Netlink-bindings

Type-safe Rust bindings for encoding/decoding Netlink messages generated from
YAML [specifications].

[specifications]: https://docs.kernel.org/userspace-api/netlink/specs.html

## Features

- All the properties described in yaml specifications are available in Rust.
- No additional dependencies (only std).
- Zero unsafe code.
- Many quirks of netlink-legacy and netlink-raw protocols are supported,
including: binary structures, nested types, multi-attribute arrays, indexed
arrays, sub messages.

## Installation

```toml
[dependencies]
netlink-bindings = { version = "0.1", features = [ "wireguard" ] }

[patch.crates-io]
netlink-bindings = { git = "https://github.com/one-d-wide/netlink-bindings.git" }
```

## Encoding

Messages are encoded directly into the provided buffer. All types relevant for
encoding are prefixed with `Push`.

```rust
let mut vec = Vec::new();
push_netlink_message_header(&mut vec); // Handled externally

// Do set-device (request)
PushOpSetDeviceDoRequest::new(&mut vec)
    .push_ifname(c"wg0") // &CStr
    .push_flags(WgdeviceFlags::ReplacePeers as u32) // Remove existing peers
    .array_peers()
        .entry_nested()
        .push_public_key(&[...]) // &[u8]
        .push_endpoint("1.2.3.4:5678".parse().unwrap()) // SocketAddr
        .array_allowedips()
            .entry_nested()
            .push_family(2) // libc::AF_INET, aka ipv4
            .push_ipaddr("0.0.0.0".parse().unwrap()) // IpAddr
            .push_cidr_mask(0) // stands "/0" in "0.0.0.0/0"
            .end_nested()
            // More allowed ips...
        .end_array() // Explicitly closing isn't necessary
        .end_nested()
        // More peers...
    .end_array();

// And that's it, the message can be sent out
update_netlink_message_lenth(&mut vec);
socket.send(&vec);
```

Your LSP should be able to nicely suggest appropriate methods as you type.

And additionally, check out [all available
methods](./netlink-bindings/src/wireguard/wireguard.md), along with the in-line
documentation.

## Decoding

Under the hood, decoder is just a slice, therefore it can be cheaply cloned,
copying the current location. The low-level interface is based on iterators,
with nice-to-use wrapper on top.

```rust
// Dump get-device (request)
PushOpGetDeviceDumpRequest::new(&mut buf).push_ifname(c"wg0");
socket.send(&buf);
socket.recv(&mut buf);
// Netlink message header is handled externally...

// Dump get-device (reply)
let attrs = OpGetDeviceDumpReply::new(&buf);

println!("Ifname: {:?}", attrs.get_ifname().unwrap()); // &CStr
for peer in attrs.get_peers() {
    println!("Endpoint: {}", peer.get_endpoint().unwrap()); // SockAddr

    for addr in peer.get_allowedips() {
        let ip = addr.get_ipaddr().unwrap(); // IpAddr
        let mask = addr.get_cidr_mask().unwrap(); // u8
        println!("Allowed ip: {ip}/{mask}");
    }
}
```

See full code in the [example](./netlink-bindings/examples/wireguard.rs). And
as previously, check out [all available
methods](./netlink-bindings/src/wireguard/wireguard.md), along with the in-line
documentation.

Also note that decoding errors are elided in the simplified interface as if the
corresponding attribute is missing. The expectation is, once working, decoding
errors are very rare. But if in doubt, the whole attribute tree can always be
printed out.

```rust
println!("{:#?}", OpGetDeviceDumpReply::new(&buf));
```

Along with the full reply tree, any encountered error will be printed out as
"Err(ErrorContext{ ... })". Otherwise, decoding errors can be inspected using
the low-level interface.

## Low-level decoding

A low-level decoding interface is exposed as an iterator, that yields enum
variants, containing either a target type, e.g. SockAddr, or another iterator,
in case of a nested attribute set.

But as you can see, using it directly quickly turns very ugly.

```rust
let iter = OpGetDeviceDumpReply::new(next);

for attr in iter {
    match attr.unwrap() {
        OpGetDeviceDumpReply::Ifname(n) => println!("Ifname: {n:?}"),
        OpGetDeviceDumpReply::Peers(iter) => {
            for entry in iter {
                for attr in entry.unwrap() {
                    match attr.unwrap() {
                        Wgpeer::Endpoint(e) => println!("Endpoint: {e:?}")
                        _ => {}
                    }
                }
            }
        }
        _ => {}
    }
}
```

## Alternatives

- [Rust-netlink](https://github.com/rust-netlink).
- [Neli](https://github.com/jbaublitz/neli).

Both don't use codegen to generate bindings, hence many Netlink families are
left out. Another difference is that they represent netlink messages as lists
of rust enums, while this project works with the binary representation
directly, using an iterator interface for decoding (internally), and a builder
pattern-like interface for encoding.

## Support status

- ✅ - supported, has tests.
- ✔️ - compiles, testing needed.
- • - compiles.
- ? - not attempted.
- ❌ - doesn't compile (needs adaptations in codegen).

| subsystem | ? | comment |
| --- | --- | --- |
| [wireguard](./netlink-bindings/src/wireguard/wireguard.md) | ✅ | |
| [conntrack](./netlink-bindings/src/conntrack/conntrack.md) | ✔️ | |
| [rt-addr](./netlink-bindings/src/rt-addr/rt-addr.md) | ✔️ | |
| [rt-link](./netlink-bindings/src/rt-link/rt-link.md) | ✔️ | |
| [rt-neigh](./netlink-bindings/src/rt-neigh/rt-neigh.md) | ✔️ | |
| [rt-route](./netlink-bindings/src/rt-route/rt-route.md) | ✔️ | |
| [rt-rule](./netlink-bindings/src/rt-rule/rt-rule.md) | ✔️ | |
| [tc](./netlink-bindings/src/tc/tc.md) | ✔️ | |
| nftables | • | operations reference a non-existent "name", generate with --no-operations |
| devlink | • | |
| ethtool | • | |
| dpll | ? | |
| fou | ? | |
| handshake | ? | |
| lockd | ? | |
| mptcp_pm | ? | |
| net-shaper | ? | |
| netdev | ? | |
| nfsd | ? | |
| nl80211 | ? | |
| nlctrl | ? | |
| ovpn | ? | |
| ovs_datapath | ? | |
| ovs_flow | ? | |
| ovs_vport | ? | |
| tcp_metrics | ? | |
| team | ? | |

The following netlink features are not implemented (yet):

- Events, notifications, multicast groups.
- Sub messages which selector attribute is outside of the parent attribute set.

## Generate bindings

Update protocol bindings using the yaml description from the protocol
directory.

```sh
$ cargo run --bin codegen -- -d ./netlink-bindings/src/wireguard/
Writing "netlink-bindings/src/wireguard/mod.rs"
Dumping "netlink-bindings/src/wireguard/wireguard.md"
Dumping all "netlink-bindings/src/wireguard/wireguard-all.md"
```

Generate protocol bindings for a new family, copying yaml specification from
somewhere else. 

```sh
$ cargo run --bin codegen -- -d ./netlink-bindings/src/ linux/Documentation/netlink/specs/wireguard.yaml
Writing "netlink-bindings/src/wireguard/mod.rs"
Dumping "netlink-bindings/src/wireguard/wireguard.md"
Dumping all "netlink-bindings/src/wireguard/wireguard-all.md"
```

## To-do

- Testing (for each sensible netlink family and for parsing primitives).
- Simplify codegen logic.
- Clean up unintentional panics in encoding/decoding.
- Run-test documentation.
- Optimize generated code size, e.g. leave out code encoding kernel replies by
default.
- Improve user interface (better error reporting, tooling, etc).
- Benchmarks, fuzzing.

## Contribute

If your want to contribute, you can, for example:

- Add testing: collect netlink messages and check that they are parsed
correctly. See wireguard [tests](./netlink-bindings/src/wireguard/tests.rs) as
an example.
- Implement yet unsupported netlink functionality.
- Improve compilation time, reduce the size of generated bindings. 
- Experiment with a better Rust interface.
- Sponsor the project (contact the author for options).
