#!/usr/bin/env bash
# LDE Linux installer: release binary -> binary in source archive -> native build.
set -euo pipefail

APP_NAME="LDE (Lemon)"
APP_ID="lde"
BIN_NAME="lemon"
REPO="${LDE_REPO:-lemonys-001/lemon-de}"
PREFIX="${PREFIX:-/usr/local}"
BINDIR="$PREFIX/bin"
SESSION_DIR="$PREFIX/share/xsessions"
DESKTOP_DIR="${XDG_DATA_HOME:-${HOME:-/tmp}/.local/share}/applications"
VERSION=""
VERSION_EXPLICIT=0
ARCH=""
FROM_SOURCE=0
ASSUME_YES=0
FORCE=0
UNINSTALL=0
UPDATE_ONLY=0
PAYLOAD="${LDE_PAYLOAD:-}"

usage() {
    cat <<'EOF'
LDE Linux installer

Usage: install.sh [options]
  --update             Check/install the latest version (the default also checks)
  --version VERSION    Install a specific release, e.g. 0.1.0 or v0.1.0
  --arch ARCH          Select amd64, arm64, arm32, x86, riscv64, ppc64le, or s390x
  --prefix PATH        Install prefix (default: /usr/local; falls back to ~/.local)
  --build-from-source  Skip binary assets and build the selected release locally
  --yes, -y            Do not prompt; useful for automation
  --force              Reinstall the same version or explicitly allow a downgrade
  --uninstall          Remove LDE binaries and session entries (keeps user data)
  --help, -h           Show this help

Install/update one-liner (inspect the script before piping it to Bash):
  curl -fsSL https://raw.githubusercontent.com/lemonys-001/lemon-de/main/install.sh | bash

When updating interactively, the installer asks before replacing an older version.
Without a TTY it proceeds, which makes the curl | bash flow usable. It will not
silently downgrade a newer install. Missing release binaries fall back to source.
EOF
}

while (($#)); do
    case "$1" in
        --update) UPDATE_ONLY=1; shift ;;
        --version) (($# >= 2)) || { echo "--version needs a value" >&2; exit 2; }; VERSION=$2; VERSION_EXPLICIT=1; shift 2 ;;
        --arch) (($# >= 2)) || { echo "--arch needs a value" >&2; exit 2; }; ARCH=$2; shift 2 ;;
        --prefix) (($# >= 2)) || { echo "--prefix needs a value" >&2; exit 2; }; PREFIX=$2; BINDIR="$PREFIX/bin"; SESSION_DIR="$PREFIX/share/xsessions"; shift 2 ;;
        --prefix=*) PREFIX=${1#*=}; BINDIR="$PREFIX/bin"; SESSION_DIR="$PREFIX/share/xsessions"; shift ;;
        --build-from-source|--source) FROM_SOURCE=1; shift ;;
        --yes|-y) ASSUME_YES=1; shift ;;
        --force|--reinstall) FORCE=1; shift ;;
        --uninstall) UNINSTALL=1; shift ;;
        --help|-h) usage; exit 0 ;;
        *) echo "Unknown option: $1 (try --help)" >&2; exit 2 ;;
    esac
done

log()  { printf '[LDE] %s\n' "$*"; }
warn() { printf '[LDE:warn] %s\n' "$*" >&2; }
die()  { printf '[LDE:error] %s\n' "$*" >&2; exit 1; }

# Select install paths before any network access, so uninstall works offline.
# Inspect the nearest existing parent too: ~/.local/bin may not exist yet but
# its parent is user-writable and should not trigger an unnecessary sudo call.
write_probe="$BINDIR"
while [[ ! -e "$write_probe" && "$write_probe" != / ]]; do write_probe=${write_probe%/*}; done
SUDO=()
if [[ ! -w "$write_probe" && "$EUID" -ne 0 ]]; then
    if command -v sudo >/dev/null 2>&1; then
        SUDO=(sudo)
        warn "$BINDIR is not writable; using sudo for system installation."
    else
        warn "$BINDIR is not writable and sudo is unavailable; installing to ~/.local/bin."
        PREFIX="${HOME:?HOME must be set for a user install}/.local"
        BINDIR="$PREFIX/bin"
        SESSION_DIR="$PREFIX/share/xsessions"
        mkdir -p "$BINDIR"
    fi
fi
BIN="$BINDIR/$BIN_NAME"
SESSION="$SESSION_DIR/$APP_ID.desktop"
DESKTOP_ENTRY="$DESKTOP_DIR/$APP_ID.desktop"

if ((UNINSTALL)); then
    log "Removing LDE from $PREFIX (user data is kept)."
    "${SUDO[@]}" rm -f "$BIN" "$BINDIR/lde" "$SESSION"
    rm -f "$DESKTOP_ENTRY"
    exit 0
fi

# Validate supported Linux hosts and map their uname/--arch value to Rust targets.
normalize_arch() {
    case "$1" in
        x86_64|amd64|x64) echo amd64 ;;
        aarch64|arm64) echo arm64 ;;
        armv7l|armv7|armhf|arm32) echo arm32 ;;
        i386|i486|i586|i686|x86) echo x86 ;;
        riscv64|riscv64gc) echo riscv64 ;;
        ppc64le|powerpc64le) echo ppc64le ;;
        s390x) echo s390x ;;
        *) echo unknown ;;
    esac
}
rust_target_for() {
    case "$1" in
        amd64) echo x86_64-unknown-linux-gnu ;;
        arm64) echo aarch64-unknown-linux-gnu ;;
        arm32) echo armv7-unknown-linux-gnueabihf ;;
        x86) echo i686-unknown-linux-gnu ;;
        riscv64) echo riscv64gc-unknown-linux-gnu ;;
        ppc64le) echo powerpc64le-unknown-linux-gnu ;;
        s390x) echo s390x-unknown-linux-gnu ;;
        *) echo "" ;;
    esac
}
if [[ -z "$ARCH" ]]; then ARCH=$(normalize_arch "$(uname -m 2>/dev/null || echo unknown)"); else ARCH=$(normalize_arch "$ARCH"); fi
[[ "$ARCH" != unknown ]] || die "Unsupported CPU architecture: $(uname -m). Use a Rust/Xlib-capable Linux build or pass --help."
TARGET=$(rust_target_for "$ARCH")
[[ "$(uname -s)" == Linux ]] || die "LDE currently requires Linux with X11."

# Determine the version from the latest release redirect unless the user pinned one.
command -v curl >/dev/null 2>&1 || die "curl is required to resolve GitHub releases and download LDE."
if ((VERSION_EXPLICIT)); then
    VERSION=${VERSION#v}
    TAG="v$VERSION"
else
    latest_url=$(curl -fsSL -o /dev/null -w '%{url_effective}' "https://github.com/$REPO/releases/latest") \
        || die "Could not resolve the latest GitHub release. Check your connection or use --version."
    TAG=${latest_url##*/}
    [[ "$TAG" == v* ]] || die "Unexpected latest release URL: $latest_url"
    VERSION=${TAG#v}
fi
[[ "$VERSION" =~ ^[0-9]+\.[0-9]+\.[0-9]+([.-][A-Za-z0-9.-]+)?$ ]] || die "Invalid release version: $VERSION"
ASSET="lemon-${TAG}-${TARGET}.tar.gz"
log "Target: $ARCH ($TARGET); release: $TAG"
((UPDATE_ONLY)) && log "Checking installed version for updates." || true

version_of() {
    [[ -x "$1" ]] || return 0
    "$1" --version 2>/dev/null | awk '{for(i=1;i<=NF;i++) if($i ~ /^[0-9]+\.[0-9]+\.[0-9]+([.-][A-Za-z0-9.-]+)?$/) {print $i; exit}}' || true
}
version_cmp() {
    local a="$1" b="$2" i x y n
    local -a A=() B=()
    IFS=. read -r -a A <<<"${a%%[-+]*}"
    IFS=. read -r -a B <<<"${b%%[-+]*}"
    n=${#A[@]}; (( ${#B[@]} > n )) && n=${#B[@]}
    for ((i=0; i<n; i++)); do
        x=${A[i]:-0}; y=${B[i]:-0}
        [[ "$x" =~ ^[0-9]+$ ]] || x=0
        [[ "$y" =~ ^[0-9]+$ ]] || y=0
        ((10#$x > 10#$y)) && { echo 1; return; }
        ((10#$x < 10#$y)) && { echo -1; return; }
    done
    echo 0
}
confirm() {
    local answer=""
    ((ASSUME_YES)) && return 0
    [[ -t 0 ]] || return 0
    read -r -p "$1 [y/N] " answer || return 1
    [[ "$answer" == y || "$answer" == Y || "$answer" == yes || "$answer" == YES ]]
}

INSTALLED_VERSION=$(version_of "$BIN")
WAS_UPDATE=0
if [[ -n "$INSTALLED_VERSION" ]]; then
    case "$(version_cmp "$INSTALLED_VERSION" "$VERSION")" in
        -1)
            log "LDE $INSTALLED_VERSION is installed; $VERSION is available."
            confirm "Update LDE $INSTALLED_VERSION -> $VERSION now?" || { log "Update declined; leaving current install untouched."; exit 0; }
            WAS_UPDATE=1
            ;;
        0)
            if ((FORCE)); then log "Reinstalling LDE $VERSION (--force)."; else log "LDE $VERSION is already installed."; exit 0; fi
            ;;
        1)
            if ((VERSION_EXPLICIT || FORCE)); then
                warn "Installed LDE $INSTALLED_VERSION is newer than requested $VERSION."
                confirm "Replace the newer version?" || { log "Leaving current install untouched."; exit 0; }
                WAS_UPDATE=1
            else
                warn "LDE $INSTALLED_VERSION is newer than this release; refusing to downgrade."
                log "Use --version VERSION or --force to explicitly allow replacement."
                exit 0
            fi
            ;;
    esac
elif [[ -e "$BIN" ]]; then
    warn "$BIN exists but does not report a version; replacing it."
fi

WORK=$(mktemp -d)
trap 'rm -rf "$WORK"' EXIT
PAYLOAD_PATH=""
ASSET_URL="https://github.com/$REPO/releases/download/$TAG/$ASSET"

fetch() {
    curl -fsSL --connect-timeout 10 --retry 2 -o "$2" "$1"
}

verify_checksum() {
    local file="$1" sidecar="$2" expected actual
    [[ -s "$sidecar" ]] || { warn "No checksum was published for $(basename "$file"); continuing without one."; return 0; }
    expected=$(awk 'NR==1 {print $1}' "$sidecar")
    [[ "$expected" =~ ^[[:xdigit:]]{64}$ ]] || die "Malformed SHA-256 sidecar: $sidecar"
    actual=$(sha256sum "$file" | awk '{print $1}')
    [[ "$actual" == "$expected" ]] || die "SHA-256 mismatch for $(basename "$file"); refusing to install."
    log "SHA-256 verified: $(basename "$file")"
}

# Optional local payload supports offline installs and mirrors.
if [[ -n "$PAYLOAD" ]]; then
    [[ -f "$PAYLOAD" ]] || die "LDE_PAYLOAD is not a file: $PAYLOAD"
    PAYLOAD_PATH=$PAYLOAD
    log "Using local payload: $PAYLOAD"
fi

# 1. Prebuilt release asset (normal path).
if [[ -z "$PAYLOAD_PATH" && "$FROM_SOURCE" -eq 0 ]]; then
    log "Trying release binary $ASSET"
    if fetch "$ASSET_URL" "$WORK/$ASSET" && tar -tzf "$WORK/$ASSET" >/dev/null 2>&1; then
        mkdir -p "$WORK/release"
        tar -xzf "$WORK/$ASSET" -C "$WORK/release"
        [[ -x "$WORK/release/$BIN_NAME" ]] || die "Release archive does not contain an executable $BIN_NAME."
        PAYLOAD_PATH="$WORK/release/$BIN_NAME"
        fetch "$ASSET_URL.sha256" "$WORK/$ASSET.sha256" 2>/dev/null || true
        verify_checksum "$WORK/$ASSET" "$WORK/$ASSET.sha256"
    fi
fi

# 2. Optional committed dist/ payload in the source archive. This permits
#    publishing a prebuilt binary in git even before a release asset is attached.
SOURCE_TAR="$WORK/source.tar.gz"
SOURCE_READY=0
if [[ -z "$PAYLOAD_PATH" && "$FROM_SOURCE" -eq 0 ]]; then
    SOURCE_URL="https://codeload.github.com/$REPO/tar.gz/refs/tags/$TAG"
    log "Checking the source archive for a committed dist/ binary."
    if fetch "$SOURCE_URL" "$SOURCE_TAR" && tar -tzf "$SOURCE_TAR" >/dev/null 2>&1; then
        SOURCE_READY=1
        ROOT=$(tar -tzf "$SOURCE_TAR" | head -n1 | cut -d/ -f1)
        mkdir -p "$WORK/archive"
        for candidate in "dist/lemon-linux-$ARCH" "dist/$ASSET"; do
            if tar -xzf "$SOURCE_TAR" -C "$WORK/archive" --strip-components=1 "$ROOT/$candidate" 2>/dev/null; then
                if [[ "$candidate" == *.tar.gz ]]; then
                    mkdir -p "$WORK/dist-binary"
                    tar -xzf "$WORK/archive/$candidate" -C "$WORK/dist-binary"
                    [[ -x "$WORK/dist-binary/$BIN_NAME" ]] && PAYLOAD_PATH="$WORK/dist-binary/$BIN_NAME"
                elif [[ -x "$WORK/archive/$candidate" ]]; then
                    PAYLOAD_PATH="$WORK/archive/$candidate"
                fi
                if [[ -n "$PAYLOAD_PATH" ]]; then
                    log "Using prebuilt binary committed under dist/ in the source archive."
                    break
                fi
            fi
        done
    fi
fi

# 3. No asset exists for this architecture: compile the release source locally.
if [[ -z "$PAYLOAD_PATH" ]]; then
    log "No prebuilt binary found for $ARCH; building LDE $VERSION from source."
    command -v cargo >/dev/null 2>&1 || die "Cargo is required for this architecture's source fallback. Install Rust/Cargo, a C compiler and Xlib development files, then retry."
    command -v cc >/dev/null 2>&1 || die "A C compiler (cc) is required for the source fallback."
    if ((SOURCE_READY == 0)); then
        SOURCE_URL="https://codeload.github.com/$REPO/tar.gz/refs/tags/$TAG"
        fetch "$SOURCE_URL" "$SOURCE_TAR" || die "Could not download the source archive for $TAG."
    fi
    SRC="$WORK/source"
    mkdir -p "$SRC"
    tar -xzf "$SOURCE_TAR" --strip-components=1 -C "$SRC"
    (cd "$SRC" && cargo build --release --locked)
    [[ -x "$SRC/target/release/$BIN_NAME" ]] || die "Source build did not produce $BIN_NAME."
    PAYLOAD_PATH="$SRC/target/release/$BIN_NAME"
fi

# Install the binary and register a basic X11 display-manager/app-grid entry.
"${SUDO[@]}" mkdir -p "$BINDIR" "$SESSION_DIR"
"${SUDO[@]}" install -m 755 "$PAYLOAD_PATH" "$BIN"
"${SUDO[@]}" ln -sf "$BIN" "$BINDIR/lde"
mkdir -p "$DESKTOP_DIR"
cat > "$WORK/lde.desktop" <<EOF
[Desktop Entry]
Name=LDE (Lemon)
Comment=Lightweight X11 desktop environment
Exec="$BIN"
TryExec="$BIN"
Type=Application
DesktopNames=LDE
EOF
install -m 644 "$WORK/lde.desktop" "$DESKTOP_ENTRY"
"${SUDO[@]}" install -m 644 "$WORK/lde.desktop" "$SESSION"

if ((WAS_UPDATE)); then
    log "Updated LDE $INSTALLED_VERSION -> $VERSION for $ARCH."
else
    log "Installed LDE $VERSION for $ARCH."
fi
printf '\n  Binary : %s\n  Alias  : %s/lde\n  Menu   : %s\n  Session: %s\n' "$BIN" "$BINDIR" "$DESKTOP_ENTRY" "$SESSION"
printf '\nRun: lemon --help  |  Update: curl -fsSL https://raw.githubusercontent.com/lemonys-001/lemon-de/main/install.sh | bash -s -- --update\n'
