#!/usr/bin/env bash
set -euo pipefail
root=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)
binary=${LEMON_BIN:-"$root/target/release/lemon"}
tmp=$(mktemp -d)
trap 'rm -rf "$tmp"' EXIT

printf '1\nxterm -fa mono\n6\nbottom\n9\n#abcdef\n11\ntrue\n0\n' |
  XDG_CONFIG_HOME="$tmp/config" "$binary" --settings >/dev/null
config="$tmp/config/lde/config"
grep -qx 'terminal = xterm -fa mono' "$config"
grep -qx 'panel_position = bottom' "$config"
grep -qx 'accent_color = 0xabcdef' "$config"
grep -qx 'autostart = true' "$config"

path=$(XDG_CONFIG_HOME="$tmp/config" "$binary" --config-path)
[[ "$path" == "$config" ]]
printf '0\n' | "$binary" --menu >/dev/null
printf 'q\n' | "$binary" --files "$tmp" >/dev/null
printf 'Terminal UI smoke tests passed.\n'
