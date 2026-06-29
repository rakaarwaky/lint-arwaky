#!/usr/bin/env bash
# install.local.sh — bump patch + release build + XDG-aware install + checksums
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
CARGO_TOML="$PROJECT_ROOT/Cargo.toml"
RELEASE_DIR="$PROJECT_ROOT/target/release"
DIST_DIR="$PROJECT_ROOT/dist"

if [ "$(id -u)" -eq 0 ]; then
    INSTALL_BIN="${LINT_ARWAKY_INSTALL_BIN:-/usr/local/bin}"
    CONFIG_DIR="${LINT_ARWAKY_CONFIG_DIR:-/etc/lint-arwaky}"
    REPORT_DIR="${LINT_ARWAKY_REPORT_DIR:-/var/lib/lint-arwaky/reports}"
else
    INSTALL_BIN="${LINT_ARWAKY_INSTALL_BIN:-$HOME/.cargo/bin}"
    CONFIG_DIR="${LINT_ARWAKY_CONFIG_DIR:-$HOME/.config/lint-arwaky}"
    REPORT_DIR="${LINT_ARWAKY_REPORT_DIR:-$HOME/.local/share/lint-arwaky/reports}"
fi

BINARIES=(lint-arwaky-cli lint-arwaky-mcp lint-arwaky-tui)

# 1. Instal XDG layout sebelum build
mkdir -p "$CONFIG_DIR/rules" "$REPORT_DIR" "$DIST_DIR" "$INSTALL_BIN"

# 2. Bump patch version
OLD_VERSION=$(grep '^version' "$CARGO_TOML" | head -1 | sed 's/version = "\(.*\)"/\1/' | tr -d '\r')
IFS='.' read -r MAJOR MINOR PATCH <<< "$OLD_VERSION"
PATCH=$(( PATCH + 1 ))
NEW_VERSION="$MAJOR.$MINOR.$PATCH"
sed -i "0,/^version = \"$OLD_VERSION\"/s/^version = \"$OLD_VERSION\"/version = \"$NEW_VERSION\"/" "$CARGO_TOML"
echo "  $OLD_VERSION  ->  $NEW_VERSION"

# 3. Build (increase stack size to prevent LLVM SIGSEGV during LTO)
RUST_MIN_STACK=16777216 cargo build --release

# 4. Checksums + install
pushd "$RELEASE_DIR" >/dev/null
sha256sum "${BINARIES[@]}" > "$DIST_DIR/SHA256SUMS.txt"
popd >/dev/null

for BIN in "${BINARIES[@]}"; do
    install -m 0755 "$RELEASE_DIR/$BIN" "$INSTALL_BIN/$BIN"
    echo "  -> $INSTALL_BIN/$BIN"
done

echo "Done: $NEW_VERSION, config=$CONFIG_DIR, reports=$REPORT_DIR"
