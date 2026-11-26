# Feature status

LDE is an early, useful X11 shell prototype, not a finished general-purpose desktop environment. “Fast”, “light” and “all architecture” need repeatable measurements and broad device testing; we do not publish unsupported benchmark or compatibility claims.

## Available in 0.2.0

- Rust executable with no third-party Rust crate dependencies.
- X11 root-window management and simple floating placement for newly mapped windows.
- Top or bottom panel with clock, quick-launch actions and clickable workspace selectors.
- Floating windows, Alt+left-drag move, Alt+right-drag resize, and a master/stack tile layout toggle.
- Super-key shortcuts, close/exit actions and nine in-memory workspaces.
- Lightweight text configuration for app commands, panel colors/position and default layout; Super+I opens settings in an installed editor.
- Optional wallpaper, desktop, notification and power helper autostart integrations; these remain separate programs.
- Source installer, X session entry, Linux documentation and Termux/VNC guide.

## Added on the current development branch (not yet released or runtime-validated)

- Interactive terminal menu in the primary `lemon` binary, with desktop start, package install, settings, launcher and file-browser actions.
- Lemon Settings TUI for app defaults, panel placement/colors, layout, wallpaper and autostart commands.
- Package-manager integration for optional application packages through apt, dnf, pacman, zypper, apk, and Termux `pkg`, including confirmation and default selection after installation.
- Built-in Rust terminal command launcher and file browser.

These additions have not yet been compiled or exercised in this environment because Cargo/Rust tooling is unavailable here. Their availability is not included in the published 0.2.0 release.

## Not available yet

- Wayland compositor, titlebar/window buttons, compositor effects, clipboard integration and built-in wallpaper/desktop icons.
- Visual control-center, integrated notifications, sound/network/battery/display/Bluetooth controls, session lock/logout UI and polkit integration.
- Accessibility control-center, screen-reader configuration, localization and assistive-input settings.
- Robust EWMH/ICCCM compliance, multi-monitor/hotplug support and persistent named workspaces. Wallpaper, notification, power and desktop helpers can be configured as separate optional apps.
- Tested builds for every CPU architecture/device or prebuilt release artifacts.

## Roadmap (aspirational)

1. Add automated X11 integration tests and test matrix across common X servers/architectures.
2. Improve ICCCM/EWMH behavior, client focus, window states, monitor layouts and graceful session lifecycle.
3. Add configurable panel/workspaces and improve app menu/accessibility.
4. Evaluate modular settings/desktop services without making the core shell unnecessarily heavy.
5. Consider Wayland only as a separately scoped compositor project; it is not a small extension to the X11 window manager.

Roadmap items have no promised release dates. See GitHub Issues for current work.
