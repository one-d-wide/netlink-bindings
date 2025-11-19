#!/bin/sh

set -e

if ! test -f netlink-bindings/src/lib.rs; then
  echo "Should be run from project root"
  exit 1
fi

for subsystem in netlink-bindings/src/*/; do
  cargo run --bin codegen -- -d "$subsystem"
done

# For most subsystems, dump files get too large to be useful
for subsys in nlctrl wireguard; do
  subsys_prefix="netlink-bindings/src/$subsys/$subsys"
  cargo run --bin codegen -- \
    "${subsys_prefix}.yaml" \
    --dump "${subsys_prefix}.md"
done

cargo run --bin codegen -- -d ./netlink-bindings/src --reverse-lookup ./reverse-lookup/src/generated.rs
