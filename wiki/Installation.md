# Install LDE on Linux

The installer prefers a matching precompiled Linux binary from the latest GitHub Release, and builds the release source locally if no matching asset exists. A source fallback needs Rust/Cargo, a C linker and Xlib development files. You need a working X11 server/session either way.

## Dependencies

- Debian/Ubuntu: `sudo apt install build-essential libx11-dev`
- Fedora: `sudo dnf install gcc make libX11-devel`
- Arch: `sudo pacman -S base-devel libx11`
- Alpine: `sudo apk add build-base libx11-dev`
- openSUSE: `sudo zypper install gcc make libX11-devel`

Install Rust stable/Cargo separately if your distribution does not provide a sufficiently current toolchain.

## Install or update

Review the script and then run:

```sh
curl -fsSL https://raw.githubusercontent.com/lemonys-001/lemon-de/main/install.sh | bash
```

It prefers `/usr/local`, uses `sudo` when needed, and falls back to `$HOME/.local` if `sudo` is unavailable. Re-running checks versions: it prompts before updating an older install, skips the same version, and prevents accidental downgrades. Non-interactive runs proceed without prompts. Use `--yes` for automation, `--force` to reinstall/allow a downgrade, `--version VERSION` to pin a release, and `--build-from-source` to skip binaries:

```sh
curl -fsSL https://raw.githubusercontent.com/lemonys-001/lemon-de/main/install.sh | bash -s -- --update
curl -fsSL https://raw.githubusercontent.com/lemonys-001/lemon-de/main/install.sh | bash -s -- --source
```

Assets are named `lemon-vVERSION-TARGET.tar.gz` with `.sha256` sidecars. The release workflow builds x86_64, AArch64 and i686 Linux binaries. A matching `vX.Y.Z` tag automatically creates a GitHub Release with the artifacts attached; a manual workflow run produces downloadable Actions artifacts. Other/missing assets fall back to a native build where Rust and Xlib support the machine.

To build a checkout without installing:

```sh
cargo build --release --locked
./target/release/lemon --help
```

Select “LDE (Lemon)” from your display manager. Do not run it alongside another WM on the same X display. If a per-user session entry is not listed, your display manager may only scan `/usr/share/xsessions`; use a system prefix or copy the session file there.

## Architecture notes

Build natively on x86_64, AArch64/arm64, ARM or another Linux target where Rust and target Xlib are available. Source portability is not a certification that every device, distribution, display manager or architecture has been tested. Cross builds also require a target C toolchain and Xlib.

See the repository's [full install troubleshooting guide](https://github.com/lemonys-001/lemon-de/blob/main/docs/INSTALL.md).
