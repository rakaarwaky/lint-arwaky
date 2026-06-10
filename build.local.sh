#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
CARGO_BIN="$HOME/.cargo/bin"
DIST_DIR="$SCRIPT_DIR/dist"
RELEASE_DIR="$SCRIPT_DIR/target/release"

mkdir -p "$CARGO_BIN"
mkdir -p "$DIST_DIR"

echo "Building lint-arwaky into target/..."
cd "$SCRIPT_DIR"
cargo build --release

echo "Copying binary only to dist/..."
cp "$RELEASE_DIR/lint-arwaky-cli" "$DIST_DIR/"
cp "$RELEASE_DIR/lint-arwaky-mcp" "$DIST_DIR/"

echo "Installing to $CARGO_BIN..."
cp "$DIST_DIR/lint-arwaky-cli" "$CARGO_BIN/"
cp "$DIST_DIR/lint-arwaky-mcp" "$CARGO_BIN/"

chmod +x "$CARGO_BIN/lint-arwaky-cli" "$CARGO_BIN/lint-arwaky-mcp"

echo "Done."
echo "  dist/       → binary bersih (lint-arwaky-cli, lint-arwaky-mcp)"
echo "  target/     → build cache, fingerprint, .d files (berguna untuk incremental build)"
echo "  cargo/bin/  → installed binaries"
