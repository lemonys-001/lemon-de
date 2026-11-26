# Installation guide

The installer checks the latest GitHub release for a binary matching the host architecture. If an asset is absent or unusable, it downloads the tagged source and builds natively on the user's machine. This fallback needs Rust/Cargo, a C linker, and Xlib development files. The release build workflow produces one `.tar.gz` and `.sha256` artifact per target; maintainers can attach the generated archives to the GitHub release. Native-build fallback is intended for Linux architectures with Rust and Xlib support.

## Requirements

- A working X11 server/session. LDE does **not** support Wayland as a compositor.
- For a matching release binary: `curl`, `tar`, and a Linux X11 runtime (`libX11`).
- For source fallback: Rust stable/Cargo, a C compiler/linker and Xlib development library. The Rust code calls the Xlib ABI directly; no third-party Rust crates are needed.
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

## Install or update from a release

Review the script, then use the one-line installer:

```sh
curl -fsSL https://raw.githubusercontent.com/lemonys-001/lemon-de/main/install.sh | bash
```

It installs to `$HOME/.local` by default, picks a matching release asset when available, and otherwise builds the latest release source. Add `$HOME/.local/bin` to `PATH`. To update an existing install, pass `--update`; to force a source build, pass `--source`:

```sh
curl -fsSL https://raw.githubusercontent.com/lemonys-001/lemon-de/main/install.sh | bash -s -- --update
curl -fsSL https://raw.githubusercontent.com/lemonys-001/lemon-de/main/install.sh | bash -s -- --source
```

Use `--prefix /usr/local` for a system-wide install (run with privileges appropriate for that prefix). The `.desktop` session descriptor is written under `<prefix>/share/xsessions/lde.desktop`. Some display managers only scan `/usr/share/xsessions`; in that case use a system prefix or copy the session entry into the display manager's session directory.

Remove an install with `--uninstall` (and the same prefix if it differs from the default):

```sh
curl -fsSL https://raw.githubusercontent.com/lemonys-001/lemon-de/main/install.sh | bash -s -- --uninstall
```

## Build the checkout

To compile the source tree you have checked out (rather than the latest tagged release):

```sh
cargo build --release --locked
./target/release/lemon --help
```

## Manual build

```sh
cargo build --release --locked
./target/release/lemon --help
./target/release/lemon
```

`lemon` requires a running X server and `DISPLAY`. Do not launch from a terminal in a desktop session already managed by another WM. To try safely, use a nested X server such as Xephyr if available, or select the LDE session at the login screen.

## CPU architectures

The release-binary workflow currently builds and tests `x86_64-unknown-linux-gnu` and `aarch64-unknown-linux-gnu`. Binary archives are named `lemon-vVERSION-TARGET.tar.gz`; the installer looks for that exact name. Workflow artifacts must be downloaded and attached to a GitHub release before the installer can use them.

The installer recognizes common GNU/Linux targets including i686, ARMv7, RISC-V64, PowerPC64LE and s390x as well. Until a release asset is attached for one of those targets, it falls back to a native source build. This requires Rust, a compatible C toolchain, Xlib development files and an X11-capable OS; it does not mean every CPU/device has been tested.

## Troubleshooting

- **`cannot open X display`**: Start an X11 server and ensure `DISPLAY` is exported; for SSH, use X forwarding if configured.
- **`XOpenDisplay` or `-lX11` linker error**: Install the Xlib runtime/development package and linker tools for your system.
- **Another window manager owns the display**: Log out and choose the LDE session, or test under a nested X server. Do not start two WMs on the same root window.
- **Super key doesn't work**: Ensure your keyboard layout/window server maps the Super/Windows key to Mod4; check desktop-level grabs.
- **No app launches**: Install a supported application (see [launch behavior](CONFIGURATION.md)); LDE logs a hint to stderr if it can't find one.
- **Black or sparse desktop**: This preview does not draw wallpaper, desktop icons, or provide a compositor. Set wallpaper with an external X11 utility if desired.
