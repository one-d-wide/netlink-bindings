## Table of contents
<!-- toc:base-level=2 -->
1. [Structure of this repository](#structure-of-this-repository)
2. [How to update a specification or introduce a new one](#how-to-update-a-specification-or-introduce-a-new-one)
3. [Codegen structure](#codegen-structure)
4. [Codegen specific options](#codegen-specific-options)

## Structure of this repository

- `./netlink-socket/` - a crate for actually interacting with the kernel.
- `./netlink-socket/examples/` - a place for the examples.
- `./netlink-bindings/` - a crate for subsystem specific bindings.
- `./netlink-bindings/examples/` - examples with explicit socket interactions.
- `./netlink-bindings/subsystems` - a list of all supported subsystems.
- `./netlink-bindings/src/utils.rs` - common routines used in bindings, maybe
some day it will be structured better.
- `./netlink-bindings/src/builtin/` - bindings for the common netlink types.
- `./netlink-bindings/src/<subsystem>/<subsystem>.yaml` - an upstream
specification.
- `./netlink-bindings/src/<subsystem>/<subsystem>.overrides.yaml` - overrides
for the specification.
- `./netlink-bindings/src/<subsystem>/mod.rs` - generated bindings for a
particular subsystem.
- `./netlink-bindings/src/<subsystem>/tests.rs` - tests for the subsystem,
optional.
- `./reverse-lookup/` - a tool to analyze netlink communications of an existing
program.
- `./reverse-lookup/src/generated.rs` - mappings between message values and
netlink-bindings types.
- `./codegen` - A code-generating tool to convert `<subsystem>.yaml` into Rust
bindings.
- `./scripts/` - a place for all other automation.
- `./scripts/collect_subsystems.sh` - update the list of subsystems in lib.rs
and Cargo.toml files.
- `./scripts/regenerate.sh` - run codegen for everything that changed (hint: it
can filter subsystems by name).
- `./scripts/run_tests.sh` - run all the tests including cargo check, cargo
test, cargo run --example, and then again in the VMs.
- `./scripts/vm_tests.nix` - a description of testing VMs for different kernel versions.

## How to update a specification or introduce a new one

Kernel has its yaml specifications in
[`Documentation/netlink/specs/`](https://elixir.bootlin.com/linux/latest/source/Documentation/netlink/specs)
choose the one for the subsystem you need from there, let's say it's called
wireguard. If you don't find the subsystem you want there, check [the mailing
list](https://lore.kernel.org/netdev/). Chances are, someone already had made
the effort, but the patches are still making their way into upstream.

First, you should copy over the upstream specifications:

```sh
$ ./scripts/copy_specs_from.sh .../linux
```

Which simply runs:

```sh
$ mkdir -p ./netlink-bindings/src/wireguard
$ cp .../linux/Documentation/netlink/specs/wireguard.yaml ./netlink-bindings/src/wireguard/wireguard.yaml
```

If any other change is needed to the spec, for example, adding a [codegen
specific option](#codegen-specific-options), it would go into
`wireguard.overrides.yaml`. The codegen will automatically pick it up and merge
it with the main specification.

Then, you should re-generate the bindings:

```sh
$ ./scripts/regenerate.sh wireg
$ ./scripts/collect_subsystems.sh
```

Which is roughly equivalent to running:

```sh
$ ./target/debug/codegen -d ./netlink-bindings/src/wireguard/
$ ./target/debug/codegen -d ./netlink-bindings/src --reverse-lookup ./reverse-lookup/src/generated.rs
$ echo wireguard >> ./netlink-bindings/subsystems
$ echo ... >> ./netlink-bindings/src/lib.rs
```

With any luck, the specification is usable as-is. If that's not the case,
however, you might have to check out [Codegen specific
options](#codegen-specific-options), and [Codegen
structure](#codegen-structure).

If you adding a new subsystem (or even if you updating one) you should strongly
consider adding an example for it. I guess you already have at least some idea
of what could it do if you're reading this. You can take
`./netlink-socket/examples/wireguard-setup.rs` as a starting point (or any
other example there, really).

Some reasons to include the example: Netlink subsystems may have some fairly
unintuitive conventions of what attributes they expect, and they rarely report
the exact value that missing, besides returning a generic error code. Also each
example is executed as part of the test suite, so adding one goes some way
toward ensuring they will work on past (and future) kernel versions.

## Codegen structure

Codegen is a single binary crate used for compiling .yaml specifications into
Rust code.

- `main()` - reads Spec and composes output
- `Spec::parse_with_override()` - parses .yaml files into serde-derived Rust structs.
- `Spec::check()` - does some minimal checks.
- `Spec::fixup()` - converts `Spec` into a more operable form, e.g. by
replacing type: bitfield32 with our builtin-bitfield32 struct.
- `gen_defs()` - constants and enums.
- `gen_cstruct()` - structs (c-style)
- `gen_attrsets()` - readable attribute sets
    - `gen_iterable_attrs()` - readable readable attribute sets iterators (`Iterable*`)
    - `gen_debug_attrs()` - Debug impls for those iterators
    - `gen_lookup()` - lookup encoded attribute by byte offset
- `gen_writable()` - writable attribute sets (`Push*`)
- `gen_notifs()` - multicast notification wrappers (`NoitfGroup::*`, `Op*Notif`)
- `gen_ops()`, `gen_request()` - request wrappers (`Request`, `Chained`, and `Op*`)
- `gen_reverse_lookup()` - match message value to it's decoder type and Debug-print it

## Codegen specific options

Yaml attributes specific to our codegen that may be helpful when dealing with
incomplete netlink specifications:

- `operations.fallback-attrs: <attrset>` - create a placeholder request type
with an operation type provided at runtime. Also, the provided attribute set is
used as a fallback in reverse lookup if operation type wasn't recognized.
- `operations.transparent: true` or `operations[].transparent: true` - make
request types use common encoding/decoding types, instead of generating new
ones that are narrowed down. Reduces generated code size.
- `operations[].request_type_at_runtime: true` - allow operation type to be
provided at runtime.
- `operations.all-attrs: true` or `operations[].all-attrs: true` - don't
narrow down attributes in generated request types.
- `operations[].no_ack: true` - operation doesn't support ack on success. This
option only affects chained requests.
- `definitions[].shrinkable: true` - C struct is padded with zeros or
truncated when needed, e.g. the struct was expanded between the kernel
versions. The default behavior is to return a decoding error.
- `operations[].{do,dump}.{request,reply}.attribute-set: <attrset>` -
attribute-set can be specific to request/response, needed for certain
netlink-raw families.
- `operations[].rust-filter` - Rust closure to differentiate between
netlink-raw operations when `value` isn't enough. Only used in reverse-lookup.
The closure is of type `fn(&[u8]) -> bool`, it checks message payload, which
usually starts with subsystem-specific header struct.
- `operations[].rust-filter-{request,reply}` - Same, but applied only for
requests and replies respectively.
- `definitions[].attributes[].display-hint: <type>[]` - display as a C-like
array, useful when it may encode data beyond a single type.
- `definitions[].attributes[].display-hint: string` - display bytes a raw
string.
- `definitions[].members.type: cbitfield` - a new type to support C bitfields.
Needs `sub-type: {u8,u16,u32}` and `bits: <n>`.

Experimental options:

- `experimental.struct-prefix: false` - disable "Push" prefix for structs.

Feature flags:

- `--features=netlink-bindings/deny-unknown-attrs` - treat unknown attributes
as errors.

Additional attributes can specified in .override.yaml file located alongside
the main specification file.

The override files follow the structure of the spec, in maps matching
attributes are traversed (or inserted if new), in lists attributes with
matching "name" are traversed (or appended otherwise).

Overrides have special attributes:

- `_override_entirely: true` - replaces the contents of an attrset entirely.
