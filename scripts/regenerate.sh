#!/usr/bin/env bash
set -euo pipefail

force=
no_reverse_lookup=
changed=
filters=()

usage() {
  echo "Usage: $0 [OPTIONS] [--] [FILTERS]"
  echo "Options:"
  echo "  -h, --help"
  echo "  -f, --force"
  echo "  -r, --no-reverse-lookup"
}

run() {
  echo >&2
  echo ">" "$@" >&2
  command "$@"
}

SHORT_OPTS="hfr"
LONG_OPTS="help,force,no-reverse-lookup"

opts="$(getopt -n "$0" -o "$SHORT_OPTS" --long "$LONG_OPTS" -- "$@" || (usage >&2 && exit 1))"
eval set -- "$opts"

while [ $# != 0 ]; do
  opt="$1"
  shift

  case "$opt" in
  -h | --help)
    usage
    exit 0
    ;;
  -f | --force) force=1 ;;
  -r | --no-reverse-lookup) no_reverse_lookup=1 ;;
  --) filters+=("$@") && break ;;
  *) echo "Unrecognized option '$opt'" && exit 1 ;;
  esac
done

if ! test -f netlink-bindings/src/lib.rs; then
  echo "Should be run from project root"
  exit 1
fi

codegen="$(cargo run --bin codegen --config 'target."cfg(true)".runner="echo"')"

for subsys in netlink-bindings/src/*/; do
  subsys_match=
  subsys_changed=1
  subsys_name="$(basename -- "$subsys")"
  yaml="${subsys}$subsys_name.yaml"
  overrides_yaml="${subsys}$subsys_name.overrides.yaml"
  mod="${subsys}/mod.rs"
  dump="${subsys}$subsys_name.md"

  if [ -z "$force" ] &&
    [ -e "$mod" ] &&
    [ "$mod" -nt "$codegen" ] &&
    [ -e "$yaml" -o -e "$overrides_yaml" ] &&
    [ ! -e "$yaml" -o "$mod" -nt "$yaml" ] &&
    [ ! -e "$overrides_yaml" -o "$mod" -nt "$overrides_yaml" ]; then
    subsys_changed=
  fi

  if [ "${#filters[@]}" -gt 0 ]; then
    for f in "${filters[@]}"; do
      case "$subsys_name" in
      *"$f"*) subsys_match=1 ;;
      esac
    done
  else
    subsys_match=1
  fi

  if [ -z "$subsys_match" ]; then
    continue
  fi

  if [ -z "$subsys_changed" ]; then
    echo Skipping "$subsys_name"
    continue
  fi

  changed=1

  args=()
  case "$subsys_name" in
  # For most subsystems, dump files get too large to be useful
  nlctrl | wireguard)
    args+=("--dump" "$dump")
    ;;
  esac

  echo Generating "$subsys_name"
  run "$codegen" -d "$subsys" "${args[@]}"
done

if [ -n "$no_reverse_lookup" ]; then
  exit 0
fi

gen="./reverse-lookup/src/generated.rs"
if [ -n "$force" ] || [ ! -e "$gen" ] || [ -n "$changed" ]; then
  echo Generating reverse-lookup
  run "$codegen" -d ./netlink-bindings/src --reverse-lookup "$gen"
else
  echo Skipping reverse-lookup
fi
