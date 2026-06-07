#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
CARGO_BIN="$HOME/.cargo/bin"

mkdir -p "$CARGO_BIN"

echo "Installing lint-arwaky binaries..."
cd "$PROJECT_ROOT"

if [ -f "$PROJECT_ROOT/target/release/lint-arwaky-cli" ]; then
    cp "$PROJECT_ROOT/target/release/lint-arwaky-cli" "$CARGO_BIN/"
    cp "$PROJECT_ROOT/target/release/lint-arwaky-mcp" "$CARGO_BIN/"
else
    cp "$PROJECT_ROOT/target/debug/lint-arwaky-cli" "$CARGO_BIN/"
    cp "$PROJECT_ROOT/target/debug/lint-arwaky-mcp" "$CARGO_BIN/"
fi

chmod +x "$CARGO_BIN/lint-arwaky-cli" "$CARGO_BIN/lint-arwaky-mcp"

echo "Installed lint-arwaky-cli and lint-arwaky-mcp to $CARGO_BIN"