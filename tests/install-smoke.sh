#!/usr/bin/env bash
set -euo pipefail

ROOT=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)
TMP=$(mktemp -d)
trap 'rm -rf "$TMP"' EXIT
PREFIX="$TMP/prefix"
XDG_DATA_HOME="$TMP/xdg"
mkdir -p "$TMP/payloads" "$XDG_DATA_HOME/lde"

make_payload() {
    local version="$1" path="$2"
    cat > "$path" <<EOF
#!/usr/bin/env sh
if [ "\${1:-}" = "--version" ]; then
  echo "LDE lemon $version"
else
  echo "test payload $version"
fi
EOF
    chmod +x "$path"
}

make_payload 0.1.0 "$TMP/payloads/lemon-0.1.0"
first=$(LDE_PAYLOAD="$TMP/payloads/lemon-0.1.0" XDG_DATA_HOME="$XDG_DATA_HOME" \
    bash "$ROOT/install.sh" --version 0.1.0 --prefix "$PREFIX" --yes)
[[ "$first" == *"Installed LDE 0.1.0"* ]]
[[ "$("$PREFIX/bin/lemon" --version)" == "LDE lemon 0.1.0" ]]
[[ -x "$PREFIX/bin/lde" && -f "$PREFIX/share/xsessions/lde.desktop" ]]

same=$(LDE_PAYLOAD="$TMP/payloads/lemon-0.1.0" XDG_DATA_HOME="$XDG_DATA_HOME" \
    bash "$ROOT/install.sh" --version 0.1.0 --prefix "$PREFIX" --yes)
[[ "$same" == *"already installed"* ]]

make_payload 0.2.0 "$TMP/payloads/lemon-0.2.0"
updated=$(LDE_PAYLOAD="$TMP/payloads/lemon-0.2.0" XDG_DATA_HOME="$XDG_DATA_HOME" \
    bash "$ROOT/install.sh" --version 0.2.0 --prefix "$PREFIX" --yes)
[[ "$updated" == *"Updated LDE 0.1.0 -> 0.2.0"* ]]
[[ "$("$PREFIX/bin/lemon" --version)" == "LDE lemon 0.2.0" ]]

echo keep > "$XDG_DATA_HOME/lde/user-data"
XDG_DATA_HOME="$XDG_DATA_HOME" bash "$ROOT/install.sh" --uninstall --prefix "$PREFIX"
[[ ! -e "$PREFIX/bin/lemon" && ! -e "$PREFIX/share/xsessions/lde.desktop" ]]
[[ -f "$XDG_DATA_HOME/lde/user-data" ]]
echo "Installer smoke tests passed."
