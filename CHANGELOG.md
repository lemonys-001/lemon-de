# Changelog

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

Prebuilt targets in this release: x86_64, AArch64 and i686 GNU/Linux. Other Linux architectures may use the source fallback where Rust and Xlib are available.
