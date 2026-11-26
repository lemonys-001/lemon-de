# Changelog

## 0.3.0 — 2026-11-26

- Add an interactive terminal menu to `lemon` for starting LDE, installing optional desktop packages, changing settings, and opening built-in tools.
- Add Lemon Settings TUI for application defaults, panel placement and colors, layout, wallpaper, and autostart commands.
- Add optional package-manager integration for apt, dnf, pacman, zypper, apk, and Termux `pkg`, with user confirmation and post-install default selection.
- Add dependency-light Rust terminal command launcher and file browser applets.
- Document which graphical applications still remain optional external packages; this release does not claim to replace browsers or full graphical desktop applications.

## 0.2.0 — 2026-11-26

- Add a user config file and `Super+I` editor entry for app commands, panel position/colors, wallpaper, default layout and optional autostart helpers.
- Add top/bottom panel positioning, clickable workspace selectors, a master/stack tiling mode and Alt+mouse move/resize.
- Keep wallpaper, notifications, desktop icons and power management as optional external helpers to avoid growing the core's dependency footprint.

## 0.1.1 — 2026-11-26

- Add version-aware install/update behavior with explicit downgrade/reinstall controls.
- Try release binaries and source-archive `dist/` payloads before building from source; verify release SHA-256 sidecars when present.
- Add tested i686 prebuilt Linux binary alongside x86_64 and AArch64.
- Automate GitHub Release creation and binary uploads for version tags matching `Cargo.toml`.
- Add release/tagging documentation.
- v0.1.1 prebuilt targets: x86_64, AArch64 and i686 GNU/Linux. Other Linux architectures may use the source fallback where Rust and Xlib are available.
