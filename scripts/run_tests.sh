#!/usr/bin/env bash
set -euo pipefail

if test -z "${UNSHARE:-}"; then
  export UNSHARE=1
  exec unshare --user --net --keep-caps -- bash -c "$0" -- "$@"
fi

export TESTING=1

features="
  conntrack,
  rt-link,
  rt-addr,
  rt-route,
  wireguard,
  nftables,
  nl80211,
  tc,
  inet-diag,
  netdev
"

examples="
  conntrack
  wireguard
  wireguard-setup
  ip-route-show
  nftables
  nftables-api
  nl80211
  nl80211-raw
  tc-prio
  tcp-rtt
  multicast-simple
  multicast-generic
  multicast-rtnetlink
"

targets="
  $(uname -m)-unknown-linux-gnu
  $(uname -m)-unknown-linux-musl
"

err=

if type -P rustup &>/dev/null; then
  for target in $targets; do
    if ! rustup target list | grep -F -- "$target (installed)" &>/dev/null; then
      err="${err}Target $target not installed.\n"
      err="${err}Run: rustup target install $target\n"
    fi
  done
else
  err="${err}Error: 'rustup' command not found\n"
fi

if test -n "$err"; then
  echo -ne "\e[0;31m$err\e[0m"
  exit 1
fi

run() {
  echo >&2
  echo ">" "$@" >&2
  command "$@"
}

cargo() {
  run cargo "$@" --features="$(echo $features | tr -d " ")" --target="$target"
}

matches() {
  if ! rg --passthru -- "$1"; then
    echo
    echo "Error: Pattern didn't match. Expected: $1"
    exit 1
  fi
}

if ip link show dev lo up | cmp /dev/null; then
  ip link add dev lo type lo || true
  ip link set dev lo up
fi

if ! ip link show wg0 &>/dev/null; then
  # Create "wg0" interface for doctests in readme
  ip link add dev wg0 type wireguard
fi

rm -rf ./target/bin_dir
mkdir -p ./target/bin_dir

for target in $targets; do
  cargo check --all-features

  cargo test

  for runtime in std tokio smol; do
    cargo run --example=extack 2>&1 |
      matches 'Attribute failed policy validation: attribute "Ifname" in "LinkAttrs": PolicyTypeAttrs \{ MaxLength: 15, Type: 11 \}'

    for example in $examples; do
      cargo run --example="$example" --features="$runtime"
      bin="$(cargo run --example="$example" --features="$runtime" --config 'target."cfg(true)".runner="echo"')"
      cp -- "$bin" "./target/bin_dir/$target-$runtime-$example"
    done
  done
done

# Run the same examples in a VM against a bunch of different kernel versions
if type -P nix &>/dev/null; then
  vm_out="$(
    run nix build \
      --print-out-paths --no-link \
      -f ./scripts/vm_tests.nix \
      --argstr bin_dir "target/bin_dir" \
      driver
  )"

  # To debug inside the vm run `$vm_runner --interactive`, type
  # `machine.start()`, wait until it boots, and ssh root@vsock/7502{0,1,2,...}
  run "$vm_out/bin/nixos-test-driver" # [--interactive]
else
  err="${err}Skipping vm tests: 'nix' command not found\n"
fi

if test -n "$err"; then
  echo -ne "\e[0;31m$err\e[0m"
  exit 1
fi
