#!/usr/bin/env bash
# install.local.sh — bump version + build + install (1 perintah)
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
CARGO_TOML="$SCRIPT_DIR/Cargo.toml"
RELEASE_DIR="$SCRIPT_DIR/target/release"
DIST_DIR="$SCRIPT_DIR/dist"
CARGO_BIN="$HOME/.cargo/bin"

BINARIES=("lint-arwaky-cli" "lint-arwaky-mcp" "lint-arwaky-tui")

# ── 1. Bump patch version ─────────────────────────────────────────────────────
echo "━━━ [1/3] Bump version ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
OLD_VERSION=$(grep '^version' "$CARGO_TOML" | head -1 | sed 's/version = "\(.*\)"/\1/' | tr -d '\r')

# Parse major.minor.patch
IFS='.' read -r MAJOR MINOR PATCH <<< "$OLD_VERSION"
PATCH=$(( PATCH + 1 ))
NEW_VERSION="$MAJOR.$MINOR.$PATCH"

# Update Cargo.toml (hanya baris version di [package], bukan dependency versions)
sed -i "0,/^version = \"$OLD_VERSION\"/s/^version = \"$OLD_VERSION\"/version = \"$NEW_VERSION\"/" "$CARGO_TOML"

echo "  $OLD_VERSION  →  $NEW_VERSION"

# ── 2. Build release ──────────────────────────────────────────────────────────
echo ""
echo "━━━ [2/3] Build ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
cd "$SCRIPT_DIR"
cargo build --release

mkdir -p "$DIST_DIR"

echo "  Copying to dist/..."
for BIN in "${BINARIES[@]}"; do
    cp "$RELEASE_DIR/$BIN" "$DIST_DIR/"
done

# ── 3. Install ────────────────────────────────────────────────────────────────
echo ""
echo "━━━ [3/3] Install ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
mkdir -p "$CARGO_BIN"

for BIN in "${BINARIES[@]}"; do
    cp "$DIST_DIR/$BIN" "$CARGO_BIN/"
    chmod +x "$CARGO_BIN/$BIN"
    echo "  ✓ $CARGO_BIN/$BIN"
done

echo ""
echo "━━━ Done — v$NEW_VERSION ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "  dist/      → binary bersih"
echo "  cargo/bin/ → installed & ready di PATH"