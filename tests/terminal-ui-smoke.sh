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
  xvfb-run -a sh -c '
    set -eu
    binary=$1
    "$binary" --no-menu &
    wm=$!
    sleep 0.3
    kill -0 "$wm" || { echo "Window manager exited before app launch" >&2; exit 1; }
    check_wm() {
      if ! kill -0 "$wm"; then
        wm_status=0
        wait "$wm" || wm_status=$?
        echo "Window manager died when an app mapped (exit $wm_status)" >&2
        exit 1
      fi
      state=$(ps -o stat= -p "$wm" 2>/dev/null || true)
      case "$state" in Z*) echo "Lemon became a zombie after app launch" >&2; exit 1 ;; esac
    }
    for app in --launcher --files --settings --calculator; do
      status=0
      if [ "$app" = "--files" ]; then timeout 3s "$binary" "$app" . || status=$?
      else timeout 2s "$binary" "$app" || status=$?; fi
      [ "$status" -eq 124 ] || { echo "$app client exited unexpectedly: $status" >&2; exit 1; }
      sleep 0.1
      check_wm
    done
    if command -v xdotool >/dev/null 2>&1; then
      timeout 5s "$binary" --calculator >/dev/null 2>&1 &
      client=$!
      sleep 0.3
      window=$(xdotool search --onlyvisible --name "Lemon Calculator" | tail -n 1)
      eval "$(xdotool getwindowgeometry --shell "$window")"
      [ "$WIDTH" -eq 720 ] && [ "$HEIGHT" -eq 560 ] || { echo "Built-in app has unexpected size: ${WIDTH}x${HEIGHT}" >&2; exit 1; }
      xdotool mousemove --sync "$((X + WIDTH - 50))" "$((Y + 34))" click 1
      client_status=0
      wait "$client" || client_status=$?
      [ "$client_status" -eq 0 ] || { echo "Mouse click did not close the built-in calculator (status $client_status)" >&2; exit 1; }
      check_wm
    fi
    kill "$wm"
    wait "$wm" 2>/dev/null || true
  ' sh "$binary"
fi
printf 'Terminal, GUI app, and X11 session smoke tests passed.\n'
