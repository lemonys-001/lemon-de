#!/bin/sh
set -eu

VERSION="0.1.0"
PREFIX="/usr/local"
UNINSTALL=0

usage() {
    cat <<'EOF'
LDE installer
Usage: ./install.sh [--prefix PATH] [--uninstall] [--help]

  --prefix PATH  Installation prefix (default: /usr/local)
  --uninstall    Remove the lemon binary and LDE session entry from that prefix
  --help         Show this help

The installer builds from the current source tree. It does not install system
packages. For dependency/package-manager instructions, see docs/INSTALL.md.
EOF
}
while [ "$#" -gt 0 ]; do
    case "$1" in
        --prefix) [ "$#" -ge 2 ] || { echo "--prefix needs a path" >&2; exit 2; }; PREFIX=$2; shift 2 ;;
        --uninstall) UNINSTALL=1; shift ;;
        --help|-h) usage; exit 0 ;;
        *) echo "Unknown option: $1" >&2; usage >&2; exit 2 ;;
    esac
done

BIN="$PREFIX/bin/lemon"
SESSION="$PREFIX/share/xsessions/lde.desktop"
if [ "$UNINSTALL" -eq 1 ]; then
    rm -f "$BIN" "$SESSION"
    echo "Removed LDE files from $PREFIX (if present)."
    exit 0
fi

if ! command -v cargo >/dev/null 2>&1; then
    echo "Cargo was not found. Install Rust/Cargo first: https://www.rust-lang.org/tools/install" >&2
    exit 1
fi
if ! command -v cc >/dev/null 2>&1; then
    echo "A C compiler (cc) is required by the system X11 linker." >&2
    exit 1
fi
cargo build --release --locked
[ -x target/release/lemon ] || { echo "Build did not produce target/release/lemon" >&2; exit 1; }
mkdir -p "$PREFIX/bin" "$(dirname "$SESSION")"
install -m 755 target/release/lemon "$BIN"
cat > "$SESSION" <<EOF
[Desktop Entry]
Name=LDE (Lemon)
Comment=Lightweight X11 desktop environment
Exec="$BIN"
TryExec="$BIN"
Type=Application
DesktopNames=LDE
EOF
chmod 644 "$SESSION"
echo "Installed LDE $VERSION to $PREFIX. Select 'LDE (Lemon)' at your display manager."
