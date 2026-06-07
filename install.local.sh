#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$SCRIPT_DIR"
CARGO_BIN="$HOME/.cargo/bin"

mkdir -p "$CARGO_BIN"

echo "Building lint-arwaky..."
cargo build --release

echo "Installing lint-arwaky binaries..."

cp "$PROJECT_ROOT/target/release/lint-arwaky-cli" "$CARGO_BIN/"
cp "$PROJECT_ROOT/target/release/lint-arwaky-mcp" "$CARGO_BIN/"

chmod +x "$CARGO_BIN/lint-arwaky-cli" "$CARGO_BIN/lint-arwaky-mcp"

echo "Installed lint-arwaky-cli and lint-arwaky-mcp to $CARGO_BIN"