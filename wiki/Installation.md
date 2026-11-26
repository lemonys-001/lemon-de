# Install LDE on Linux

LDE is a source build for the current machine. Runtime dependencies are Rust's standard library and system Xlib; no external Rust crates are needed. You need a working X11 server/session and a C linker.

## Dependencies

- Debian/Ubuntu: `sudo apt install build-essential libx11-dev`
- Fedora: `sudo dnf install gcc make libX11-devel`
- Arch: `sudo pacman -S base-devel libx11`
- Alpine: `sudo apk add build-base libx11-dev`
- openSUSE: `sudo zypper install gcc make libX11-devel`

Install Rust stable/Cargo separately if your distribution does not provide a sufficiently current toolchain.

## Build and install

```sh
git clone https://github.com/lemonys-001/lemon-de.git
cd lemon-de
./install.sh --prefix "$HOME/.local"
```

Add `$HOME/.local/bin` to `PATH`. For a system install, run `./install.sh` with privileges appropriate for `/usr/local`; it builds the release executable and creates `share/xsessions/lde.desktop`. The script never installs packages or silently escalates privileges.

To build without installing:

```sh
cargo build --release --locked
./target/release/lemon --help
```

Select “LDE (Lemon)” from your display manager. Do not run it alongside another WM on the same X display. If a per-user session entry is not listed, your display manager may only scan `/usr/share/xsessions`; use a system prefix or copy the session file there.

## Architecture notes

Build natively on x86_64, AArch64/arm64, ARM or another Linux target where Rust and target Xlib are available. Source portability is not a certification that every device, distribution, display manager or architecture has been tested. Cross builds also require a target C toolchain and Xlib.

See the repository's [full install troubleshooting guide](https://github.com/lemonys-001/lemon-de/blob/main/docs/INSTALL.md).
