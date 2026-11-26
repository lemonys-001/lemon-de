# Feature status

LDE is an early X11 shell prototype, not a finished general-purpose desktop environment. “Fast”, “light” and “all architecture” need repeatable measurements and broad device testing; we do not publish unsupported benchmark or compatibility claims.

## Available in 0.3.0

- Rust executable with no third-party Rust crate dependencies.
- X11 root-window management and simple floating placement for newly mapped windows.
- Top or bottom panel with clock, quick-launch actions and clickable workspace selectors.
- Floating windows, Alt+left-drag move, Alt+right-drag resize, and a master/stack tile layout toggle.
- Super-key shortcuts, close/exit actions and nine in-memory workspaces.
- Interactive terminal menu in the primary `lemon` binary for starting the desktop, installing optional packages, settings, and built-in tools.
- Lemon Settings TUI for application commands/defaults, panel placement/colors, layout, wallpaper, and autostart commands.
- Package-manager integration for apt, dnf, pacman, zypper, apk, and Termux `pkg`, with confirmation and optional default selection after installation.
- Built-in Rust terminal command launcher and file browser.
- Optional wallpaper, desktop, notification and power helpers remain separate programs; source installer, X session entry, Linux documentation and Termux/VNC guide.

The 0.3.0 code is built and smoke-tested on x86_64 and AArch64 by CI. The package-manager workflows are not run against real systems in CI; package availability depends on distribution and repositories.

## Not available yet

- **LDE does not yet provide Rust replacements for every desktop application.** The in-binary launcher and file browser are terminal tools; LDE still relies on separate terminal emulators, graphical file managers, web browsers, graphical app menus and optional notification/power services. Installing them is optional and user-controlled.
- Wayland compositor, titlebar/window buttons, compositor effects, clipboard integration and built-in wallpaper/desktop icons.
- Visual control center, integrated notifications, sound/network/battery/display/Bluetooth controls, session lock/logout UI and polkit integration.
- Accessibility control center, screen-reader configuration, localization and assistive-input settings.
- Robust EWMH/ICCCM compliance, multi-monitor/hotplug support and persistent named workspaces.
- Tested builds for every CPU architecture/device. Prebuilt release targets remain x86_64, AArch64 and i686 GNU/Linux.

## Roadmap (aspirational)

1. Add automated X11 integration tests and test matrix across common X servers/architectures.
2. Improve ICCCM/EWMH behavior, client focus, window states, monitor layouts and graceful session lifecycle.
3. Expand lightweight Rust applets, then assess which commonly used graphical components can realistically be implemented in Rust without bundling a web engine or bloating the core.
4. Add configurable panel/workspaces and improve graphical app-menu/accessibility support.
5. Consider Wayland only as a separately scoped compositor project; it is not a small extension to the X11 window manager.

Roadmap items have no promised release dates. See GitHub Issues for current work.
