# Configure LDE

LDE stays small by keeping settings in one text file and optional desktop services as separate processes. Run `lemon --init-config` to create the default file, `lemon --config-path` to print its location, or `lemon --settings` in a terminal to open the interactive Lemon Settings menu. Press **Super+I** in an LDE session to open the file in an installed graphical text editor (then `$EDITOR` in a terminal as a fallback). Running `lemon` with no arguments in a terminal opens the main menu; in a graphical session it starts the desktop.

Default location: `~/.config/lde/config`, or `$XDG_CONFIG_HOME/lde/config` when `XDG_CONFIG_HOME` is set. Changes to app commands, colors, panel position, and default layout apply the next time LDE starts. Automatic update checks run in a short-lived background process no more than once every 24 hours; disable them with `check_updates = false` in settings. LDE only reports newer versions (using `notify-send` if available) and never downloads or installs updates automatically. The result is cached under `$XDG_CACHE_HOME/lde/update-check` or `~/.cache/lde/update-check`.

```ini
# Commands are optional; blank values use built-in executable discovery.
terminal = foot
launcher = rofi -show drun
file_manager = thunar
browser = firefox

# Use feh or nitrogen if installed; leave blank for the solid background.
wallpaper = /home/alex/Pictures/wallpaper.jpg

# panel_position supports top or bottom; layout supports floating or tiled.
panel_position = top
layout = floating
panel_color = 0x172033
accent_color = #60a5fa
background_color = 0x111827

# Automatic GitHub release check at most once per day. Set false to disable it.
check_updates = true

# Optional desktop/session components. One command per setting line.
autostart = dunst
autostart = xfce4-power-manager
autostart = pcmanfm --desktop
```

Commands from this file are run through the user's shell as that same user. Only put commands you trust in the file. Optional programs are not bundled with LDE. `feh` or `nitrogen` can apply wallpaper; `dunst` can provide notifications; `xfce4-power-manager` can provide power-management UI; and `pcmanfm --desktop` can provide icons. Choose compatible components and avoid starting a second panel or window manager.

## Window layout and mouse

- **Super+T** toggles the current workspace between floating and a simple master/stack tiled layout. The workspace's prior floating geometry is restored when switching back.
- **Alt+left-drag** moves a window; **Alt+right-drag** resizes it, with a minimum size clamp.
- **Super+M** minimizes the focused window; **Super+Shift+M** restores the most recently minimized window.
- **Alt+Tab** cycles focus through visible windows. A normal click focuses a window and is replayed to the app so its controls still receive the click.
- **Super+D** or the panel's **Apps** button opens Lemon Launcher; **Super+I** or the panel's **Settings** button opens Lemon Settings; **Super+C** opens Lemon Calculator.
- The panel has Terminal, Files, Web, Calculator, Apps and Settings actions; these start the configured programs or lightweight built-in apps.
- **Super+1 … Super+9** switches workspaces; the workspace numbers are also clickable in the panel on wider screens.

Built-in Lemon apps use a compact in-window header with a close action; external windows do not yet have separate WM titlebar decorations. LDE includes an opt-in basic X11 compositor (`lemon --compositor`) with a Control Center toggle, but no advanced blur/shadows or integrated desktop-icon/notification/power/accessibility services. External helpers can be autostarted without adding their dependencies to LDE's small core.
