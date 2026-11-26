#!/bin/sh
set -eu
REPO="https://github.com/lemonys-001/lemon-de.wiki.git"
ROOT=$(CDPATH= cd -- "$(dirname -- "$0")/.." && pwd)
TMP=$(mktemp -d)
trap 'rm -rf "$TMP"' EXIT HUP INT TERM
if ! git clone "$REPO" "$TMP/wiki"; then
    cat >&2 <<'EOF'
The GitHub Wiki has not been initialized yet. Create its first page at
https://github.com/lemonys-001/lemon-de/wiki/_new, then rerun this script.
The repository's ready-to-publish wiki pages are in ./wiki/.
EOF
    exit 1
fi
cp "$ROOT"/wiki/*.md "$TMP/wiki/"
cd "$TMP/wiki"
git add *.md
if git diff --cached --quiet; then
    echo "Wiki is already up to date."
    exit 0
fi
git -c user.name="LDE Wiki Publisher" -c user.email="noreply@github.com" commit -m "Publish LDE user guides"
git push origin HEAD
