#!/bin/sh

set -e

if ! test -f netlink-bindings/src/lib.rs; then
  echo "Should be run from project root"
  exit 1
fi

for subsystem in netlink-bindings/src/*/; do
  cargo run --bin codegen -- -d "$subsystem"
done

cargo run --bin codegen -- -d ./netlink-bindings/src --reverse-lookup ./reverse-lookup/src/generated.rs
