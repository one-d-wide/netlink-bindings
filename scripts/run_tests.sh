#!/bin/sh

set -e

features="conntrack,rt-link,rt-addr,wireguard,nftables,nl80211"

cargo() {
  echo
  echo ">" "cargo $@ --features=$features"
  command cargo "$@" --features="$features"
}

matches() {
  if ! rg --passthru -- "$1"; then
    echo
    echo "Error: Pattern didn't match. Expected: $1"
    exit 1
  fi
}

cargo build -p netlink-bindings --all-features

if ! ip link show wg0 >/dev/null; then
  # Create "wg0" interface for readme doctests
  ip link add dev wg0 type wireguard
fi

cargo test

for runtime in "" --features={tokio,smol}; do
  cargo run --example=extack |
    matches 'Attribute failed policy validation: attribute "Ifname" in "OpNewlinkDoRequest": PolicyTypeAttrs \{ MaxLength: 15, Type: 11 \}'

  examples="conntrack wireguard-setup nftables nftables-api nl80211 nl80211-raw"
  for example in $examples; do
    cargo run --example="$example" $runtime
  done
done
