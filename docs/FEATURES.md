# Feature status

LDE is an early, useful X11 shell prototype, not a finished general-purpose desktop environment. “Fast”, “light” and “all architecture” need repeatable measurements and broad device testing; we do not publish unsupported benchmark or compatibility claims.

## Available in 0.1.0

- Rust executable with no third-party Rust crate dependencies.
- X11 root-window management and simple floating placement for newly mapped windows.
- Top panel with quick-launch actions and clock text.
- External terminal, file manager, browser and launcher integration.
- Super-key shortcuts, close/exit actions and nine in-memory workspaces.
- Source installer, X session entry, Linux documentation and Termux/VNC guide.

## Not available yet

- Wayland compositor, window decorations/title bar, tiling layouts, drag/resize, clipboard integration.
- Integrated settings, wallpaper/desktop icons, notifications, sound, network, battery/power, display and Bluetooth controls.
- Accessibility settings, screen reader integration, localization, session lock/logout UI, polkit integration.
- Robust EWMH/ICCCM compliance, multi-monitor and hotplug support, persistent workspaces/configuration.
- Tested builds for every CPU architecture/device or prebuilt release artifacts.

## Roadmap (aspirational)

1. Add automated X11 integration tests and test matrix across common X servers/architectures.
2. Improve ICCCM/EWMH behavior, client focus, window states, monitor layouts and graceful session lifecycle.
3. Add configurable panel/workspaces and improve app menu/accessibility.
4. Evaluate modular settings/desktop services without making the core shell unnecessarily heavy.
5. Consider Wayland only as a separately scoped compositor project; it is not a small extension to the X11 window manager.

Roadmap items have no promised release dates. See GitHub Issues for current work.
