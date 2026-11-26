# Installation guide

LDE is source-first: the installer compiles for the machine it runs on. That avoids shipping one binary per CPU and works wherever Rust, a C linker, and Xlib are available. There are no prebuilt binaries attached to the preview release; maintainers can attach tested builds later.

## Requirements

- A working X11 server/session. LDE does **not** support Wayland as a compositor.
- Rust stable and Cargo (edition 2021).
- C compiler/linker and Xlib development library. The Rust code calls the Xlib ABI directly; no Rust crates are downloaded.
- For a useful session: a terminal (`xterm`, `foot`, `kitty`, etc.), file manager, browser, and optionally `rofi` or `dmenu`.

### Install build dependencies

Pick the matching command for the Linux distribution. Package names can differ across releases.

| Distribution | Packages (examples) |
| --- | --- |
| Debian / Ubuntu / Linux Mint | `sudo apt install build-essential libx11-dev` |
| Fedora | `sudo dnf install gcc make libX11-devel` |
| Arch / Manjaro | `sudo pacman -S base-devel libx11` |
| Alpine | `sudo apk add build-base libx11-dev` |
| openSUSE | `sudo zypper install gcc make libX11-devel` |
| Termux (inside a supported Linux proot) | `apt install build-essential libx11-dev` |

Install Rust with your distribution's Rust package or [rustup](https://www.rust-lang.org/tools/install). The installer intentionally does not invoke a package manager or request root access on your behalf.

## Install from a checkout

```sh
git clone https://github.com/lemonys-001/lemon-de.git
cd lemon-de
./install.sh
```

The default prefix is `/usr/local`; installing there typically needs administrator privileges. For a per-user install:

```sh
./install.sh --prefix "$HOME/.local"
```

Make sure `$HOME/.local/bin` is in `PATH`. The `.desktop` session descriptor is written under `<prefix>/share/xsessions/lde.desktop`. Some display managers only scan `/usr/share/xsessions`; in that case use a system prefix or copy the session entry into the display manager's session directory. The session entry starts the installed `lemon` binary.

To remove the installed binary and session entry:

```sh
./install.sh --prefix "$HOME/.local" --uninstall
```

## Manual build

```sh
cargo build --release --locked
./target/release/lemon --help
./target/release/lemon
```

`lemon` requires a running X server and `DISPLAY`. Do not launch from a terminal in a desktop session already managed by another WM. To try safely, use a nested X server such as Xephyr if available, or select the LDE session at the login screen.

## CPU architectures

The code has no architecture-specific Rust dependencies and is intended to build natively on x86_64, AArch64/arm64, 32-bit ARM (subject to toolchain/Xlib availability), and other Rust-supported Linux targets with Xlib. That is a portability goal, not a claim that every architecture has been tested or has a packaged X server. Build on the target device for the most reliable result. Cross-compilation additionally requires a target C toolchain and target Xlib.

## Troubleshooting

- **`cannot open X display`**: Start an X11 server and ensure `DISPLAY` is exported; for SSH, use X forwarding if configured.
- **`XOpenDisplay` or `-lX11` linker error**: Install the Xlib runtime/development package and linker tools for your system.
- **Another window manager owns the display**: Log out and choose the LDE session, or test under a nested X server. Do not start two WMs on the same root window.
- **Super key doesn't work**: Ensure your keyboard layout/window server maps the Super/Windows key to Mod4; check desktop-level grabs.
- **No app launches**: Install a supported application (see [launch behavior](CONFIGURATION.md)); LDE logs a hint to stderr if it can't find one.
- **Black or sparse desktop**: This preview does not draw wallpaper, desktop icons, or provide a compositor. Set wallpaper with an external X11 utility if desired.
