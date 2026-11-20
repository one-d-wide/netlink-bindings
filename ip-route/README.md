## Ip-route

Embed iproute2-like commands into your program.

## Overview

Ip-route allows interacting with rtnetlink subsystem using the same syntax as
the [`ip`] command.

Despite its name, ip-route isn't limited to `ip` commands, it's extensible
enough to cover other applications. For example, it understands the basic
`iptables` options, albeit support for its more obscure [extensions] would
require more work.

[`ip`]: https://man7.org/linux/man-pages/man8/ip.8.html
[extensions]: https://www.man7.org/linux/man-pages/man8/iptables-extensions.8.html

## Usage

Ip-route takes care of correctly encoding the request, while leaving the reply
fully readable using the normal decoding functionality of neltink-bindings. It
works as a `format!()`-like proc-macro.

`ip!()` macro has 2 forms: provided with a socket, it will execute a request
right away; omit it, and it'll return a `Request` struct, useful to encode
additional attributes not present in the CLI.

```rust
use ip_route::{ip, ip_dump};
use netlink_socket2::NetlinkSocket;
use std::net::IpAddr;

let mut sock = NetlinkSocket::new();

let ifname = "wg0";

// A query without a reply
// ip!(sock, ...) sends the request and receives an acknowledgment.
ip!(sock, "ip link set dev wg0 up").unwrap();
ip!(sock, "ip link set dev {ifname} up").unwrap();
ip!(sock, "ip link set dev {} up", ifname).unwrap();

// ...equivalent to
let req = ip!("ip link set dev {ifname} up");
sock.request_ack(&req).unwrap();

// A query with a single reply
// ip_dump!(sock, ...) simply sends the request
let mut iter = ip_dump!(sock, "ip link show {ifname}").unwrap();
let (_header, _attrs) = iter.recv_one().unwrap();

// ...with multiple replies
let mut iter = ip_dump!(sock, "ip link show").unwrap();
while let Some((header, attrs)) = iter.recv().transpose().unwrap() {
  println!("{}: {:?}", header.ifi_index, attrs.get_ifname().unwrap());
}

// A query with optional values and conditional parameters
let enable = true;
let set_mtu = None;
ip!(sock, "ip link set dev {ifname} mtu {set_mtu:option} [?enable: up :][: down :]").unwrap();
```

## Installation

```toml
[dependencies]
ip-route = "0.3"
```

## Compatibility

Ip-route already supports common options of `ip address`, `ip link`, `ip
route`, `ip rule`, and `iptables`. These currently exclude things like: encap
and nexthop for `ip route`, each possible kind of trailing type attributes in
`ip link`, only bridge attrs are currently supported. If you're looking for a
specific option, you can grep the source files to see whether it's currently
supported.

`ip!()` macro takes a command and encodes a single request ready to be sent to
the kernel and that's it. Any replies are best delegated to the user as any
filtering or extraction is already fairly trivial to do using the existing
decoding facilities provided by netlink-bindings. This framework works well for
iproute2-like subcommands, as they work in a similar way.

Hence, it's easy to verify that ip-route produces the same message as `ip`
given the same command. This's exactly what happens in [`./tests`](./tests),
accomplished using reverse-lookup tool to intercept the messages once from `ip`
command and once from the test itself.

Ensuring that tests cover all the options is hard. One way attempted was to use
`ip route help` output to produce a comprehensive set of commands invoking all
possible flags, but its doesn't appear to be a rigorous description, resulting
in too many unsupported combinations. Manually removing offending options
results in too many options being drooped, straying far from being
comprehensive.

`iptables` has some features that the kernel doesn't directly implement like
referring to a rule in a chain by its position (kernel refers to rules using
handles that are unique across all chains). Luckily, it seems to have only a
handful of such features, so it doesn't seem too hard to implement as a helper
function if need be.

## Inner workings

_You may stop reading here if you're only interested in using the already
supported commands through the macro._

`ip!()` macro transforms your command into code that uses netlink-bindings to
encode the message.

Internally, a command is parsed according to its regex-like description that
may additionally contain variable substitutions and code blocks helping to
shape the final message. Command descriptions reside in
[`./syntax/ip*.rs`](./syntax).

A bit simplifying, `ip!()` macro expansion looks like this:

```rust
use ip_route::ip;
use netlink_bindings::{rt_link, traits::NetlinkRequest};

let dev = "wg0";

let req = ip!("ip link add dev {dev} up");

let req_expanded = {
    // ip link add
    let mut header = rt_link::Ifinfomsg::new();
    let mut req = rt_link::Request::new()
        .set_create().set_excl()
        .op_newlink_do(&header);
    let mut attrs = req.encode();

    // dev {dev}
    {
        let ifname: &str = dev; 
        attrs = attrs.push_ifname_bytes(ifname.as_bytes());
    }

    // up
    {
        header.ifi_flags |= libc::IFF_UP as u32;
        header.ifi_change |= libc::IFF_UP as u32;
    }

    // ...

    std::mem::drop(attrs);
    *req.header_mut() = header;
    req
};

assert_eq!(req.protocol(), req_expanded.protocol());
assert_eq!(req.flags(), req_expanded.flags());
assert_eq!(req.payload(), req_expanded.payload());
```

The description of the command above might look like this:

```rust
use ip_route_syntax::{Map, Val, group};

const EXAMPLE: Map = Map::Any(&[
    // dev {ifname}
    Map::All(&[
        Map::Lit("dev"),
        Map::Val(Val::new("ifname", "&str")),
        Map::Code(stringify!{
            attrs = attrs.push_ifname_bytes(ifname.as_bytes());
        }),
    ]),
    // up
    Map::All(&[
        Map::Lit("up"),
        Map::Code(stringify!{
            header.ifi_flags |= libc::IFF_UP as u32;
            header.ifi_change |= libc::IFF_UP as u32;
        }),
    ]),
    // ...
])
.star();

// or written more succinctly using a special macro...
group! {
    const EXAMPLE_MACRO;
    ~ repeat: star,
    "dev", ifname: &str => {
        attrs = attrs.push_ifname_bytes(ifname.as_bytes());
    }
    "up" => {
        header.ifi_flags |= libc::IFF_UP as u32;
        header.ifi_change |= libc::IFF_UP as u32;
    }
    // ...
}

assert_eq!(Map::All(&[EXAMPLE]), EXAMPLE_MACRO);
```

## Syntax

A command description is represented like a regular expression with a few
notable additions:
- `Val` represents a value to be made available to the following code. It
matches either an inline constant or a substitution.
- `Code` represents a code block. It's inserted when this node is traversed, it
doesn't match anything itself.

This simple [syntax] proved to be sufficient to parse all the currently
implemented commands.

[syntax]: ./syntax/lib.rs

```rust
pub enum Map {
    All(&'static [Self]),
    Any(&'static [Self]),
    May(&'static Self),
    Star(&'static Self),
    Plus(&'static Self),

    Lit(&'static str),
    Val(Val),

    Code(&'static str),

    Last,
}

pub struct Val {
    pub name: &'static str,
    pub ty: &'static str,
    pub flags: &'static [&'static str],
}
```

For convenience, there's also a macro that simplifies common patterns, like
weirdly crafted CLI options found in iptables:

```rust
use ip_route_syntax::group;

group! {
    const EXAMPLE;
    ~ repeat: star /* "may" or "plus" or empty */,
    ~ init => { /* inserted before this group */ }
    ~ fini => { /* inserted after this group */ }

    // matches: [!] --sport 1234
    &CMP_EQ_OR_NEQ, ("--source-port" | "--sport"), port: u16 => {
        // ...
    }

    // matches: [!] --mark 0xbeef/0xffff
    &CMP_EQ_OR_NEQ, "--mark", v: (u32, Option<u32>) as "format:{}[/{}]" => {
        // ...
    }
}

group! {
    const CMP_EQ_OR_NEQ;
    ~ repeat: may,
    ~ init => { let mut cmp_op = CmpOps::Eq; }
    "!" => { cmp_op = CmpOps::Neq; }
}
```


## Interpreter

For someone looking to make a new CLI tool for a netlink subsystem, ip-route
could offer a succinct, but versatile way of describing the parameters and
their actions, making the same command syntax available as both CLI and API.

It's possible to use the same description to interpret commands at runtime. See
[`interp_poc.rs`](./syntax/interp-poc.rs) as a proof of concept. But it's
unclear how to parse the replies in a generic manner if any.

```rust,ignore
// A potential API for an interpreter

let ifname = "tun0";

ip_interp_str("ip link set dev {} up")
    .arg(ifname)
    .send(&mut sock)?
    .recv_ack()?;

// or as a CLI
ip_interp_cli(&["ip" "link" "set" "dev", ifname, "up"])
    .send(&mut sock)?
    .recv_ack()?;
```

The idea behind it is to have hierarchy of nodes that maps nicely onto the
usual Rust's scoping rules, i.e. the syntax description to be a tree. The very
same idea also makes it possible to conditionally toggle parameters by passing
an `Option` in place of any value.

Assigning each `Map` in a tree a unique index, it's possible to capture order
they're visited when parsing a particular command. For example, using the
following description to parse `ip route add 1.2.3.4 dev {ifname}` would give
sequence `[1, 2, 1, 3]`.

```rust,ignore
Map::Repeat(
    // 1
    Map::Any([
        // 2
        Map::All([
            Map::Val(/* address */),
            Map::Code(/* push address */),
        ]),
        // 3
        Map::All([
            Map::Lit("dev"),
            Map::Val(/* string */),
            Map::Code(/* push ifname */),
        ]),
    ]),
)
```

The obtained sequence can be passed to a runtime encoder that is assembled
recursively like this: `Map::All` collects code blocks in sequence, `Map::Any`
creates a `match` statement, and repetitions produce loops.

Executing the following code snippet against the obtained sequence would
accomplish the same as code resulting from the proc-macro.

```rust,ignore
let mut attrs = ...;

while next_if(1) {
    match next() {
        2 => attrs.push_address(next_arg().as_addr()?);
        3 => attrs.push_ifname(next_arg().as_str()?),
        _ => unreachable!(),
    }
}

sock.request(&attrs)?;
```
