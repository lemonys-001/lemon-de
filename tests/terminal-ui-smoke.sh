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
    local status=0 log="$tmp/gui-${1#--}.log"
    timeout 2s xvfb-run -a "$binary" "$@" >"$log" 2>&1 || status=$?
    if [[ $status -ne 124 ]]; then
      detail=$(tail -n 20 "$log" | tr "\n" " ")
      echo "::error title=GUI smoke failed::$* exited with $status: $detail"
      return 1
    fi
  }
  gui_smoke --launcher
  gui_smoke --files "$tmp"
  gui_smoke --settings
  gui_smoke --control-center
  gui_smoke --calculator
  compositor_status=0
  timeout 2s xvfb-run -a "$binary" --compositor >"$tmp/compositor.log" 2>&1 || compositor_status=$?
  if [[ $compositor_status -ne 124 ]] || ! grep -q 'LDE lite compositor active' "$tmp/compositor.log"; then
    detail=$(tail -n 20 "$tmp/compositor.log" | tr "\\n" " ")
    echo "::error title=Built-in compositor smoke failed::status $compositor_status: $detail"
    exit 1
  fi
  xvfb-run -a sh -c '
    set -eu
    binary=$1
    wm_log=$2
    "$binary" --no-menu >"$wm_log" 2>&1 &
    wm=$!
    sleep 0.3
    if ! kill -0 "$wm"; then
      wm_status=0; wait "$wm" || wm_status=$?
      detail=$(tail -n 20 "$wm_log" | tr "\n" " ")
      echo "::error title=Window manager startup::exited with $wm_status: $detail"
      exit 1
    fi
    root_properties=$(xprop -root _NET_NUMBER_OF_DESKTOPS 2>&1 || true)
    if ! printf "%s\n" "$root_properties" | grep -q "= 9"; then
      echo "::error title=EWMH property missing::$root_properties"
      exit 1
    fi
    check_wm() {
      if ! kill -0 "$wm"; then
        wm_status=0; wait "$wm" || wm_status=$?
        detail=$(tail -n 20 "$wm_log" | tr "\n" " ")
        echo "::error title=Window manager crashed::exit $wm_status: $detail"
        exit 1
      fi
      state=$(ps -o stat= -p "$wm" 2>/dev/null || true)
      case "$state" in Z*) echo "Lemon became a zombie after app launch" >&2; exit 1 ;; esac
    }
    for app in --launcher --files --settings --control-center --calculator; do
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
      desktop_property=$(xprop -id "$window" _NET_WM_DESKTOP 2>&1 || true)
      if ! printf "%s\n" "$desktop_property" | grep -q "= 0"; then echo "::error title=EWMH client property missing::$desktop_property"; exit 1; fi
      xdotool mousemove --sync "$((X + WIDTH - 50))" "$((Y + 34))" click 1
      client_status=0
      wait "$client" || client_status=$?
      [ "$client_status" -eq 0 ] || { echo "Mouse click did not close the built-in calculator (status $client_status)" >&2; exit 1; }
      check_wm
    fi
    kill "$wm"
    wait "$wm" 2>/dev/null || true
  ' sh "$binary" "$tmp/wm.log"
fi
printf 'Terminal, GUI app, and X11 session smoke tests passed.\n'
