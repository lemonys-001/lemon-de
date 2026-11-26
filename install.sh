#!/usr/bin/env bash
set -euo pipefail

REPO="lemonys-001/lemon-de"
PREFIX="${HOME:-/tmp}/.local"
DO_UPDATE=0
FORCE_SOURCE=0
UNINSTALL=0

usage() {
    cat <<'EOF'
LDE release installer
Usage: install.sh [--prefix PATH] [--update] [--source] [--uninstall] [--help]

  --prefix PATH  Install prefix (default: $HOME/.local)
  --update       Update an existing install to the latest GitHub release
  --source       Build the latest release from source instead of using a binary
  --uninstall    Remove the LDE binary and session entry from the prefix
  --help         Show this help

Quick install (review this script before piping it into Bash):
  curl -fsSL https://raw.githubusercontent.com/lemonys-001/lemon-de/main/install.sh | bash
Update:
  curl -fsSL https://raw.githubusercontent.com/lemonys-001/lemon-de/main/install.sh | bash -s -- --update
EOF
}
while (($#)); do
    case "$1" in
        --prefix) (($# >= 2)) || { echo "--prefix needs a path" >&2; exit 2; }; PREFIX=$2; shift 2 ;;
        --update) DO_UPDATE=1; shift ;;
        --source) FORCE_SOURCE=1; shift ;;
        --uninstall) UNINSTALL=1; shift ;;
        --help|-h) usage; exit 0 ;;
        *) echo "Unknown option: $1" >&2; usage >&2; exit 2 ;;
    esac
done

BIN="$PREFIX/bin/lemon"
SESSION="$PREFIX/share/xsessions/lde.desktop"
if ((UNINSTALL)); then
    rm -f "$BIN" "$SESSION"
    echo "Removed LDE files from $PREFIX (if present)."
    exit 0
fi

command -v curl >/dev/null || { echo "curl is required." >&2; exit 1; }
command -v tar >/dev/null || { echo "tar is required." >&2; exit 1; }
[[ "$(uname -s)" == Linux ]] || { echo "LDE currently requires Linux with X11." >&2; exit 1; }

TMP_DIR=$(mktemp -d)
trap 'rm -rf "$TMP_DIR"' EXIT
LATEST_URL=$(curl -fsSL -o /dev/null -w '%{url_effective}' "https://github.com/${REPO}/releases/latest")
TAG=${LATEST_URL##*/}
[[ "$TAG" == v* ]] || { echo "Could not determine the latest LDE release (got '$TAG')." >&2; exit 1; }
VERSION=${TAG#v}

if ((DO_UPDATE)) && [[ -x "$BIN" ]]; then
    CURRENT_VERSION=$("$BIN" --version 2>/dev/null | awk '{print $3}' || true)
    if [[ "$CURRENT_VERSION" == "$VERSION" ]]; then
        echo "LDE $VERSION is already installed at $BIN."
        exit 0
    fi
    echo "Updating LDE ${CURRENT_VERSION:-unknown} to $VERSION..."
fi

TARGET=""
case "$(uname -m)" in
    x86_64|amd64) TARGET=x86_64-unknown-linux-gnu ;;
    aarch64|arm64) TARGET=aarch64-unknown-linux-gnu ;;
    i386|i486|i586|i686|x86) TARGET=i686-unknown-linux-gnu ;;
    armv7l|armv7*) TARGET=armv7-unknown-linux-gnueabihf ;;
    riscv64) TARGET=riscv64gc-unknown-linux-gnu ;;
    ppc64le) TARGET=powerpc64le-unknown-linux-gnu ;;
    s390x) TARGET=s390x-unknown-linux-gnu ;;
esac

SOURCE_BUILD=1
if ((!FORCE_SOURCE)) && [[ -n "$TARGET" ]]; then
    ASSET="lemon-${TAG}-${TARGET}.tar.gz"
    ASSET_URL="https://github.com/${REPO}/releases/download/${TAG}/${ASSET}"
    if curl -fsSL --retry 2 "$ASSET_URL" -o "$TMP_DIR/$ASSET" 2>/dev/null \
       && tar -tzf "$TMP_DIR/$ASSET" >/dev/null 2>&1; then
        mkdir "$TMP_DIR/unpacked"
        tar -xzf "$TMP_DIR/$ASSET" -C "$TMP_DIR/unpacked"
        if [[ -x "$TMP_DIR/unpacked/lemon" ]]; then
            install -Dm755 "$TMP_DIR/unpacked/lemon" "$TMP_DIR/lemon"
            SOURCE_BUILD=0
            echo "Using precompiled release asset for $TARGET."
        fi
    fi
fi

if ((SOURCE_BUILD)); then
    if ((FORCE_SOURCE)); then
        echo "Building LDE $VERSION from source as requested."
    else
        echo "No usable precompiled binary found for $(uname -m); building LDE $VERSION from source."
    fi
    command -v cargo >/dev/null || { echo "Cargo is required for the source fallback. Install Rust/Cargo and X11 development files, then retry." >&2; exit 1; }
    command -v cc >/dev/null || { echo "A C compiler is required for the source fallback." >&2; exit 1; }
    SRC="$TMP_DIR/source"
    mkdir -p "$SRC"
    curl -fsSL "https://github.com/${REPO}/archive/refs/tags/${TAG}.tar.gz" -o "$TMP_DIR/source.tar.gz"
    tar -xzf "$TMP_DIR/source.tar.gz" --strip-components=1 -C "$SRC"
    (cd "$SRC" && cargo build --release --locked)
    install -Dm755 "$SRC/target/release/lemon" "$TMP_DIR/lemon"
fi

mkdir -p "$PREFIX/bin" "$(dirname "$SESSION")"
install -m 755 "$TMP_DIR/lemon" "$BIN"
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
echo "Installed LDE $VERSION to $BIN."
echo "Add $PREFIX/bin to PATH. Select 'LDE (Lemon)' from your display manager (system session directories may require a system-prefix install)."
