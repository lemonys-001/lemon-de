# LDE — Lemon Desktop Environment

LDE is a deliberately small **Rust/X11 desktop shell and window manager**. The first public milestone focuses on a compact, dependency-free Rust runtime, a usable panel, basic window management, keyboard shortcuts and a straightforward installer.

> **Status: early preview (0.1.0), not a complete feature-parity desktop environment.** LDE currently targets X11 only. It does not yet provide a settings app, compositor, notifications, power management, accessibility suite, or Wayland support. Please read the [feature status](docs/FEATURES.md) before replacing a daily-driver desktop.

## What works today

- Lightweight floating X11 window manager with a top panel and app launch buttons.
- Super+Enter terminal, Super+D application menu, Super+Q close focused client, Super+Shift+Q quit LDE.
- Nine basic workspaces: Super+1 through Super+9.
- Launches installed terminal/file manager/browser/menu programs by trying common names; see [launch behavior](docs/CONFIGURATION.md).
- Rust runtime has no third-party Rust crate dependencies. It links to the system Xlib (`libX11`).
- Installer creates a display-manager session entry on conventional Linux systems.

## Quick install (Linux)

Prerequisites: Rust/Cargo, a C toolchain, and X11 development/runtime files (`libX11`; usually package `libx11-dev`, `libX11-devel`, or `libx11`). A terminal and application launcher are also recommended.

```sh
git clone https://github.com/lemonys-001/lemon-de.git
cd lemon-de
./install.sh --prefix "$HOME/.local"
```

Add `$HOME/.local/bin` to `PATH`. For a system-wide install (default prefix `/usr/local`), build as your user and run the installer with appropriate privileges; the installer itself does not invoke `sudo` or install system packages. See [Installation](docs/INSTALL.md) for distro, source, architecture and troubleshooting guidance.

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
- [Architecture and development notes](docs/architecture.md)
- [Security notes](docs/SECURITY.md)

Wiki-ready pages are included under [`wiki/`](wiki/Home.md). GitHub requires the first Wiki page to be created in its web UI before a repository can push Wiki content; once seeded, run `scripts/publish-wiki.sh` to publish/update the included pages. The main installation and troubleshooting docs are available immediately under [`docs/`](docs/INSTALL.md).

## Contributing

Issues and small, focused pull requests are welcome. See [CONTRIBUTING.md](CONTRIBUTING.md). LDE is MIT licensed; see [LICENSE](LICENSE).
