## Netlink-bindings

Type-safe Rust bindings for encoding/decoding Netlink messages generated from
YAML [specifications].

[netlink]: https://docs.kernel.org/userspace-api/netlink/index.html
[specifications]: https://docs.kernel.org/userspace-api/netlink/specs.html
[list-of-specs]: https://www.kernel.org/doc/html/latest/netlink/specs/

## Overview

[Netlink][netlink] is a structured way for various kernel subsystems to expose
their userspace API using hierarchical format with binary-encoded messages
exchanged over a socket. Many kernel [subsystems][list-of-specs] already have a
machine-readable API descriptions, which we use to generate Rust bindings.

This project provides easy-to-use type-safe interface, while being reasonably
fast and supporting all properties of all sensible Netlink families.

## Features

- Simple type-safe interface, see below [Making requests](#making-requests) and [Examples](#other-examples).
- Support for all documented netlink subsystems, see [Support status](#support-status).
- Netlink messages can be Debug-printed, with enum variants and flags annotated.
- Examine netlink messages of existing programs using [reverse-lookup](#working-off-of-existing-tools).

## Support status

All [upstream specifications][list-of-specs] are supported as of Linux 7.2.

- ✅ supported, has tests:
[conntrack](./netlink-socket/examples/conntrack.rs),
[inet-diag](./netlink-socket/examples/tcp-rtt.rs),
[nftables](./netlink-socket/examples/nftables.rs),
[nl80211](./netlink-socket/examples/nl80211.rs),
[nlctrl](./netlink-bindings/src/nlctrl/nlctrl.md),
[rt-addr](./netlink-socket/examples/wireguard-setup.rs),
[rt-route](./netlink-socket/examples/ip-route-show.rs),
[rt-link](./netlink-socket/examples/wireguard-setup.rs),
[tc](./netlink-socket/examples/tc-prio.rs),
[wireguard](./netlink-socket/examples/wireguard-setup.rs).
- ✔️ compiles, testing needed: binder, dev-energymodel, devlink, dpll, drm-ras,
ethtool, fou, handshake, lockd, mptcp_pm, netdev, net-shaper, nfsd, ovpn,
ovs_datapath, ovs_flow, ovs_packet, ovs_vport, psp, rt-neigh, rt-rule, sunrpc,
tcp_metrics, team, unix-diag.

## Installation

```toml
[dependencies]
netlink-bindings = { version = "0.3", features = [ "wireguard" ] }
netlink-socket2 = { version = "0.3", features = [ ] }
```

## Making requests

A typical Netlink family, say wireguard, supports multiple operations:
"get-device", "set-device", etc. Each operation may be of kind "do" or "dump".

As an example, to gather info about a device you would use a "dump" kind
(returning multiple replies) of "get-device" request. That's usually what it
means, although different subsystems may imply different things. A typical
request looks like this:

```rust
use netlink_bindings::wireguard;
use netlink_socket2::NetlinkSocket;

let mut sock = NetlinkSocket::new();

let mut request = wireguard::Request::new()
    .op_get_device_dump();

request.encode()
    .push_ifname_bytes(b"wg0");

let mut iter = sock.request(&request).unwrap();
while let Some(attrs) = iter.recv().transpose().unwrap() {
    // An attribute may be missing or failing to parse
    let listen_port = attrs.get_listen_port().unwrap();
    println!("Interface is listening on {listen_port}");

    // Print out all the attributes using the Debug formatter.
    println!("{:#?}", attrs);
}
```

Your LSP should be able to nicely suggest appropriate methods both for encoding
and decoding as you type.

## More complicated requests

Let's say you have a network interface and you want to assign it an ip address.
This is domain of "rt-addr" family. It was one of the first subsystems created,
inheriting some now-discouraged quirks like a fixed-header - a struct that's
always present in a message. Its use depends on the request type, with unused
fields usually zeroed-out.

The relevant operation is "newaddr" of kind "do" (only returning an
acknowledgment).

```rust
use std::net::IpAddr;
use netlink_bindings::rt_addr;
use netlink_socket2::NetlinkSocket;

let mut sock = NetlinkSocket::new();

let addr: IpAddr = "10.0.0.1".parse().unwrap();
let ifindex = unsafe { libc::if_nametoindex(c"wg0".as_ptr()) };

// Create fixed-header for the request
let header = rt_addr::Ifaddrmsg {
    ifa_index: ifindex,
    ifa_family: libc::AF_INET as u8, // aka ipv4
    ifa_prefixlen: 32, // stands for "/32" in "10.0.0.1/32"
    ..Default::default()
};

let mut request = rt_addr::Request::new()
    .set_change() // Don't fail if address already assigned
    .op_newaddr_do(&header);

request.encode()
    .push_local(addr);

sock.request(&request).unwrap()
    .recv_ack().unwrap();
```

You may also notice ".set_change()" setting a flag. Similar to the
fixed-header, these flags may trigger additional behavior in certain
operations, or do nothing in others.

See full code in the [example](./netlink-socket/examples/wireguard-setup.rs).

## Async sockets

Generally, Netlink requests resolve immediately, so you can safely use the
default `NetlinkSocket` issuing "blocking" syscalls in the async context, which
is what recommended.

The only exceptions are a multicast socket, receiving notification
asynchronously, or a hypothetical subsystem, choosing to deliberately delay
replies.

Netlink-socket crate allows the facilities for different runtimes to coexist
under different paths, i.e. `netlink_socket2::tokio::{NetlinkSocket,
MulticastSocketRaw}`, with async functionality available with the same exact
structure as the "blocking" one.

```toml
[dependencies]
netlink-socket2 = { ... , features = [ "std", "tokio" ] } # or "smol"
```

## Other examples

See [netlink-socket/examples](./netlink-socket/examples):

- [wireguard-setup](./netlink-socket/examples/wireguard-setup.rs) - Create and
configure wireguard interface.
- [ip-route-show](./netlink-socket/examples/ip-route-show.rs) - Dump routing
entries.
- [conntrack](./netlink-socket/examples/conntrack.rs) - Dump tracked network
connections, similar to `conntrack -L`.
- [extack](./netlink-socket/examples/extack.rs) - Showcase handing of extended
ack attributes in error reporting.
- [nftables](./netlink-socket/examples/nftables.rs) - Create nftables rules.
- [nftables-api](./netlink-socket/examples/nftables-api.rs) - A high-level
wrapper for creating nftables rules.
- [nl80211](./netlink-socket/examples/nl80211.rs) - Basic interactions with nl80211.
- [nl80211-raw](./netlink-socket/examples/nl80211-raw.rs) - Same as nl80211,
but manually encoding/decoding netlink messages.
- [tc-prio](./netlink-socket/examples/tc-prio.rs) - Add, show, and delete
traffic control queueing discipline.
- [tcp-rtt](./netlink-socket/examples/tcp-rtt.rs) - Dump socket information,
including RTT of a TCP socket.
- [multicast-simple](./netlink-socket/examples/multicast-simple.rs) - Listen
for multicast notifications emitted when a network device is created, changed,
or deleted.
- [multicast-generic](./netlink-socket/examples/multicast-generic.rs) - Listen
for all multicast notifications on a generic netlink subsystem.
- [multicast-raw](./netlink-socket/examples/multicast-rtnetlink.rs) - Listen
for multicast notifications on legacy rtnetlink subsystem.


## Advanced usage

### Working off of existing tools

If there's an existing tool using Netlink, you can use `reverse-lookup` tool to
decipher its Netlink communications and work off of that.

Let's say you want to see what `wg` command does:

```sh
$ cargo install --git https://github.com/one-d-wide/netlink-bindings reverse-lookup --features all-subsystems
$ ~/.cargo/bin/reverse-lookup -- wg
Decoding request in family ROUTE flags=[REQUEST,ACK,DUMP,REPLACE,EXCL] Raw { protonum: 0, request_type: 0 }
...
Decoding reply in genl family wireguard flags=[MULTI] Generic("wireguard")
Wgdevice {
    ListenPort: 0,
    Fwmark: 0,
    Ifindex: 10,
    Ifname: "wg0",
}
...
```

This way you can study the structure of the real messages the kernel expects
and replies with. In order to translate it into code, you simply need to
convert attributes from CamelCase to snake_case, adding occasional `.get_*()`,
`.push_*()`, or `.nested_*()` prefix.

This tool is merely interpreting the output of `strace(1)`, see `reverse-lookup
--help` for more details.

### Attribute encoding

Under the hood, calling `.encode()` is just a convenience to switch to a
correct `Push*` wrapper for encoding, which is given an internal buffer. For
example, directly encoding a "do" request of "set-device" operation looks like
this:

```rust
use netlink_bindings::wireguard as wg;

let mut vec = Vec::new();

// Do set-device (request)
wg::OpSetDeviceDo::encode_request(&mut vec)
    .push_ifname(c"wg0") // &CStr
    // .push_ifname_bytes("wg0".as_bytes()) // &[u8]
    .push_flags(wg::WgdeviceFlags::ReplacePeers as u32) // Remove existing peers
    .array_peers()
        .entry_nested()
        .push_public_key(&[/* ... */]) // &[u8]
        .push_endpoint("127.0.0.1:12345".parse().unwrap()) // SocketAddr
        .array_allowedips()
            .entry_nested()
            .push_family(libc::AF_INET as u16) // aka ipv4
            .push_ipaddr("0.0.0.0".parse().unwrap()) // IpAddr
            .push_cidr_mask(0) // stands for "/0" in "0.0.0.0/0"
            .end_nested()
            // More allowed ips...
        .end_array() // Explicitly closing isn't necessary
        .end_nested()
        // More peers...
    .end_array();
```

Additionally, check out [all available
methods](./netlink-bindings/src/wireguard/wireguard.md), along with the in-line
documentation.

### Attribute decoding

Similarly, under the hood, receiving a reply yields an attribute decoder. The
decoder itself is just a wrapper on a slice, therefore it can be cheaply
cloned, copying its frame. The low-level interface is based on iterators, with
nicer helper functions on top.

```rust,should_panic
use netlink_bindings::traits::NetlinkRequest;
use netlink_bindings::wireguard::OpGetDeviceDump;

// Dump get-device (reply)
let attrs = OpGetDeviceDump::decode_reply(&[/* ... */]);

println!("Ifname: {:?}", attrs.get_ifname().unwrap()); // &CStr
for peer in attrs.get_peers().unwrap() {
    println!("Endpoint: {}", peer.get_endpoint().unwrap()); // SockAddr

    for addr in peer.get_allowedips().unwrap() {
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

### Cloning attrset contents

In some cases, it's infeasible to construct the message in one go, for example
when you want to be able to freely alternate between encoding two different
attribute sets.

The kernel only expects a single nested attribute set of a certain kind to
appear at the same level of nesting, so you have to first encode them in
separate temporary buffers before cloning them into the final message. This
trick applies to reusing parts of an already encoded message received from the
kernel.

Writing raw attributes is possible using `.as_vec_mut()` method method of
[`Pusher`](https://docs.rs/netlink-bindings/latest/netlink_bindings/traits/trait.Pusher.html)
trait, implemented for all encoding `Push*` structs, giving access to the
internal `&mut Vec<u8>` buffer. And a corresponding `.get_buf()` method
available on all decoding structs returning their `&[u8]` frame.

```rust
use netlink_bindings::{traits::Pusher, rt_link};

let mut link_attrs = Vec::new();
let mut bridge_attrs = Vec::new();

rt_link::PushLinkinfoBridgeAttrs::new(&mut bridge_attrs)
    .push_priority(1000);

rt_link::PushLinkAttrs::new(&mut link_attrs)
    .push_link(42);

// ... 

let mut req = rt_link::Request::new()
    .op_getlink_do(&Default::default());

req.encode()
    .as_vec_mut()
    .extend_from_slice(&link_attrs);

req.encode()
    .nested_linkinfo()
        .nested_data_bridge()
        .as_vec_mut()
        .extend_from_slice(&bridge_attrs);
```

### Low-level decoding

A low-level decoding interface is exposed as an iterator, that yields enum
variants, containing either a target type, e.g. SockAddr, or another iterator,
in case of a nested attribute set. But as you can see, using it directly
quickly turns very ugly.

```rust
use netlink_bindings::traits::NetlinkRequest;
use netlink_bindings::wireguard::{OpGetDeviceDump, Wgdevice, Wgpeer};

for attr in OpGetDeviceDump::decode_reply(&[/* ... */]) {
    match attr.unwrap() {
        Wgdevice::Ifname(n) => println!("Ifname: {n:?}"),
        Wgdevice::Peers(iter) => {
            for entry in iter {
                for attr in entry.unwrap() {
                    match attr.unwrap() {
                        Wgpeer::Endpoint(e) => println!("Endpoint: {e:?}"),
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
not supported.

Another difference is that they represent netlink messages as lists of rust
enums, while this project works with the binary representation directly, with a
separate interfaces for encoding and decoding: a builder pattern-like interface
for encoding, and an iterator interface for decoding (internally).

## Contribute

See [CONTRIBUTING.md](./CONTRIBUTING.md) for information on how to work with
this repo, its structure, codegen stuff, etc.

If your want to contribute, you can, for example:

- Just use netlink-bindings. If you encounter a shortcoming of the current API, report it.
- Write straightforward higher-level abstractions on top of netlink-bindings.
- Add testing: collect netlink messages and check that they are parsed
correctly. See wireguard [tests](./netlink-bindings/src/wireguard/tests.rs) as
an example. Additional [examples](./netlink-socket/examples) are also very
welcome.
- Implement yet unsupported netlink functionality.
- Improve compilation time, reduce the size of generated bindings.
- Experiment with a better Rust interface (for encoding/decoding and the sockets).
- Sponsor the project (contact the author for options).
