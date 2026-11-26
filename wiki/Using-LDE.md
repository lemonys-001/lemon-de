# Using LDE

## Keyboard shortcuts

| Shortcut | Action |
| --- | --- |
| `Super` + `Enter` | Launch a terminal |
| `Super` + `D` | Open `rofi`, `dmenu_run` or `bemenu-run` |
| `Super` + `Q` | Close the focused X client |
| `Super` + `Shift` + `Q` | Exit LDE |
| `Super` + `1` … `9` | Switch workspaces |

Super is commonly the Windows key. The X server must map it to Mod4. Workspaces are in-memory and reset at exit.

The panel quick-launch buttons try installed programs: terminals (`foot`, `kitty`, `alacritty`, `wezterm`, `xterm`, `uxterm`, `urxvt`), file managers (`thunar`, `pcmanfm`, `nemo`, `dolphin`), browsers (`firefox`, `chromium`, `chromium-browser`, `google-chrome`) and launchers (`rofi`, `dmenu_run`, `bemenu-run`). `$TERMINAL` overrides terminal discovery.

This preview has no settings app, persistent configuration, window drag/resize or integrated app catalog. See [Feature Status](Feature-Status).
