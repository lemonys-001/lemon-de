# Configuration

LDE reads `~/.config/lde/config` (or `$XDG_CONFIG_HOME/lde/config`). Start with `lemon --init-config`; use `lemon --config-path` to locate it or press `Super+I` to edit it. Settings are applied when LDE starts.

Example:

```ini
terminal = foot
launcher = rofi -show drun
file_manager = thunar
browser = firefox
wallpaper = /home/me/Pictures/wallpaper.jpg
panel_position = bottom
layout = tiled
panel_color = 0x222534
accent_color = #ffb84d
background_color = 0x171923
autostart = dunst
autostart = xfce4-power-manager
autostart = pcmanfm --desktop
```

Values for commands are run through your shell as your user. Only configure commands you trust. Optional programs such as `feh`/`nitrogen`, `dunst`, `xfce4-power-manager` and `pcmanfm` are not bundled with LDE. Keep components optional so a minimal install only uses the core shell.

See the [full configuration guide](https://github.com/lemonys-001/lemon-de/blob/main/docs/CONFIGURATION.md).
