# Controls and application launching

## Keyboard shortcuts

| Shortcut | Action |
| --- | --- |
| `Super` + `Enter` | Start a terminal |
| `Super` + `D` | Start `rofi`, `dmenu_run`, or `bemenu-run` (first available) |
| `Super` + `Q` | Close the focused client |
| `Super` + `Shift` + `Q` | Exit LDE |
| `Super` + `1` … `Super` + `9` | Switch between nine basic workspaces |

The Super key is commonly labeled Windows or Command (when mapped by the X server). Workspace state is in-memory and is reset when LDE exits.

## Panel

The panel has quick actions for terminal, files, web and apps. It searches common executable names:

- **Terminal:** `$TERMINAL`, then `foot`, `kitty`, `alacritty`, `wezterm`, `xterm`, `uxterm`, `urxvt`.
- **Files:** `thunar`, `pcmanfm`, `nemo`, `dolphin`, `xdg-open`.
- **Web:** `xdg-open`, then common Firefox/Chromium names.
- **Apps:** `rofi`, `dmenu_run`, or `bemenu-run`.

For the best compatibility set `TERMINAL` to a terminal executable available in your X session. App launching uses the current user's `PATH`. Install the desired apps with your distribution package manager.

## Current limits

This milestone does not include a settings GUI, persistent preferences, workspace names, drag-to-move/resize, hotplug handling, or an integrated application catalog. The launcher is an external program. Contributions to those areas are welcome; see [FEATURES.md](FEATURES.md).
