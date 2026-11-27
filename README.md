# LDE — Lemon Desktop Environment

LDE is a deliberately small **Rust/X11 desktop shell and window manager**. The core remains dependency-light and delegates optional desktop services to separately installed programs.

> **Early preview (0.5.0), not a complete XFCE replacement.** LDE targets X11 only. It has no Wayland compositor, standard titlebar decorations for external apps, or bundled browser/terminal. Optional desktop tools are discovered and launched from a lightweight Control Center. Please read the [feature status](docs/FEATURES.md) before replacing a daily-driver desktop.

## What works today

- Floating windows with Alt+left-drag to move and Alt+right-drag to resize, minimize/restore (`Super+M` / `Super+Shift+M`), Alt+Tab focus cycling, and optional master/stack tiling (`Super+T`).
- Configurable top/bottom panel, app launch buttons and nine clickable/keyboard-selectable workspaces.
- `Super+Enter` terminal, `Super+D` native launcher, `Super+I` Lemon Settings, `Super+O` Control Center, `Super+C` calculator, `Super+Q` graceful close (ICCCM `WM_DELETE_WINDOW` where supported), `Super+Shift+Q` quit, and `Super+M` minimize.
- **Interactive terminal menu:** run `lemon` in a terminal without `DISPLAY` to open the menu for starting the desktop, installing optional packages, opening Lemon Settings, or using the built-in Rust command launcher and file browser. If `DISPLAY` is set—including Termux/VNC—`lemon` starts the desktop; pass `--menu` to explicitly force the terminal menu. The panel exposes Apps and Settings buttons; Settings can apply a wallpaper through optional `feh` or `nitrogen`.
- **Lemon Settings:** edit app defaults, panel position/colors, layout, wallpaper, and autostart commands without manually editing the config file.
- **Control Center (`Super+O`):** discovers optional system tools for sound, network, Bluetooth, displays, appearance, power, notifications, clipboard, accessibility, desktop icons, screen lock, session logout, and optional X11 compositor effects. These are launched only when installed; LDE does not bundle duplicate heavyweight services.
- The Rust binary includes native X11 and terminal versions of its launcher, file browser, settings, and calculator. Larger applications such as a graphical terminal, browser, and notification daemon remain optional separate packages. The menu can install supported packages via apt, dnf, pacman, zypper, apk, or Termux `pkg`, then optionally set app defaults.
- Launches installed terminal/file manager/browser/menu programs by trying common names; see [configuration](docs/CONFIGURATION.md) and [components](docs/COMPONENTS.md).
- Rust runtime has no third-party Rust crate dependencies. It links to the system Xlib (`libX11`).
- Installer creates a display-manager session entry on conventional Linux systems.

## Quick install (Linux)

The installer checks the latest GitHub release for a matching precompiled binary first. If the release has no binary for your CPU, it downloads the matching source and builds locally. The fallback needs Rust/Cargo, a C toolchain, and X11 development files (`libX11`; usually `libx11-dev`, `libX11-devel`, or `libx11`). Review the [installer script](install.sh) before running it through Bash.

```sh
curl -fsSL https://raw.githubusercontent.com/lemonys-001/lemon-de/main/install.sh | bash
```

This installs to `/usr/local` by default, using `sudo` when needed and falling back to `$HOME/.local` if `sudo` is unavailable. Re-running the installer checks the installed version: it asks before updating an older install, skips the same version, and refuses an accidental downgrade. Piped/non-interactive runs proceed without prompting. Use `--update` to make the version check explicit, `--yes` for automation, or `--force` to reinstall/allow a downgrade. Use `--version VERSION` to pin a release and `--build-from-source` to skip prebuilt binaries.

```sh
curl -fsSL https://raw.githubusercontent.com/lemonys-001/lemon-de/main/install.sh | bash -s -- --update
```

Use `--prefix /usr/local` for a system-wide install (run with appropriate privileges). The [release workflow](.github/workflows/release-binaries.yml) builds x86_64, AArch64 and i686 Linux binaries. Pushing a `vX.Y.Z` tag matching `Cargo.toml` automatically creates a GitHub Release and attaches the successful build archives plus SHA-256 sidecars. Running the workflow manually creates downloadable Actions artifacts only. Other/missing architecture assets fall back to a native source build. See [Installation](docs/INSTALL.md) for details.

## Termux and VNC

LDE cannot start an Android graphical display by itself. In Termux, use a Linux userspace (for example, a Debian/Ubuntu proot) with an **X11 VNC server** and the X11 development package installed inside that userspace. Start VNC, connect a VNC viewer, then start LDE from the VNC session. The Android/Termux guide covers a complete example and known limitations: [Termux + VNC](docs/TERMUX.md).

## Build and run from source

```sh
# Debian/Ubuntu example
sudo apt install build-essential pkg-config libx11-dev
cargo build --release
./target/release/lemon
```

Start LDE from an X11 display manager by selecting the “LDE (Lemon)” session, or from an existing X server with `DISPLAY` set. **Do not start it inside a graphical session that already has another window manager**; only one X11 window manager can own root-window redirection.

## Documentation

- [Installation across distributions and architectures](docs/INSTALL.md)
- [Termux with VNC](docs/TERMUX.md)
- [Keyboard shortcuts and application launching](docs/CONFIGURATION.md)
- [Feature status and roadmap](docs/FEATURES.md)
- [Built-in apps and package menu](docs/COMPONENTS.md)
- [Architecture and development notes](docs/architecture.md)
- [Security notes](docs/SECURITY.md)
- [Release process](docs/RELEASING.md)

Wiki-ready pages are included under [`wiki/`](wiki/Home.md). GitHub requires the first Wiki page to be created in its web UI before a repository can push Wiki content; once seeded, run `scripts/publish-wiki.sh` to publish/update the included pages. The main installation and troubleshooting docs are available immediately under [`docs/`](docs/INSTALL.md).

## Contributing

Issues and small, focused pull requests are welcome. See [CONTRIBUTING.md](CONTRIBUTING.md). LDE is MIT licensed; see [LICENSE](LICENSE).
