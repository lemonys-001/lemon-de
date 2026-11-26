# Feature status

## Available in 0.3.1

- Single-process Rust X11 shell with system Xlib and no third-party Rust crates.
- Floating/tiled layouts, top or bottom panel, clock, app launch buttons, and nine in-memory workspaces.
- Interactive terminal menu, Lemon Settings TUI, package install/default selection flow, and built-in Rust command launcher and terminal file browser.
- External terminal, graphical file manager, browser, graphical app launcher, and optional notification/power integrations remain separate packages.
- Source installer, X session entry, and Termux/VNC documentation.

## Not implemented

LDE does not yet provide Rust replacements for every desktop application. It has no bundled graphical terminal, browser, graphical file manager or graphical app menu. Other gaps include Wayland, compositor, titlebars, robust EWMH/ICCCM, multi-monitor/hotplug, visual control center, wallpaper/icons, integrated notifications, accessibility integration, session lock/logout, and power/display/network/sound/Bluetooth controls. Prebuilt binaries are currently for x86_64, AArch64 and i686 GNU/Linux, not every architecture.

“Lightweight” and “all architectures” are goals, not established benchmark or compatibility claims. This is an early preview, not a complete daily-driver environment. See the repository [roadmap](https://github.com/lemonys-001/lemon-de/blob/main/docs/FEATURES.md).
