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
calculator=$(printf '2+2\nq\n' | env -u DISPLAY "$binary" --calculator)
grep -q '= 4' <<<"$calculator"

if command -v xvfb-run >/dev/null 2>&1; then
  gui_smoke() {
    local status=0
    timeout 2s xvfb-run -a "$binary" "$@" >/dev/null 2>&1 || status=$?
    if [[ $status -ne 124 ]]; then
      printf 'GUI app failed to remain open (%s): exit %s\n' "$*" "$status" >&2
      return 1
    fi
  }
  gui_smoke --launcher
  gui_smoke --files "$tmp"
  gui_smoke --settings
  gui_smoke --calculator
fi
printf 'Terminal and GUI app smoke tests passed.\n'
