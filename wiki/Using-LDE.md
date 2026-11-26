# Using LDE

## Keyboard and mouse controls

| Shortcut | Action |
| --- | --- |
| `Super` + `Enter` | Launch a terminal |
| `Super` + `D` | Open the configured launcher or `rofi`, `dmenu_run`, `bemenu-run` |
| `Super` + `I` | Open the LDE configuration file in an editor |
| `Super` + `T` | Toggle floating / master-stack tiled layout on this workspace |
| `Super` + `Q` | Close the focused X client |
| `Super` + `Shift` + `Q` | Exit LDE |
| `Super` + `1` … `9` | Switch workspaces |
| `Alt` + left-drag | Move a window |
| `Alt` + right-drag | Resize a window |

Super is commonly the Windows key. The X server must map it to Mod4. Workspaces and the selected layout per workspace are in-memory; the configured default layout is persistent.

The panel quick-launch buttons try installed programs: terminals (`foot`, `kitty`, `alacritty`, `wezterm`, `xterm`, `uxterm`, `urxvt`), file managers (`thunar`, `pcmanfm`, `nemo`, `dolphin`), browsers (`firefox`, `chromium`, `chromium-browser`, `google-chrome`) and launchers (`rofi`, `dmenu_run`, `bemenu-run`). `$TERMINAL` overrides configured terminal discovery.

See [Configuration](Configuration) for user settings. Optional desktop icons, notifications and power controls can come from separate applications configured as autostart helpers; they are not built into LDE.
