#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
DIST_DIR="$SCRIPT_DIR/dist"
CARGO_BIN="$HOME/.cargo/bin"

CLI_SRC="$DIST_DIR/lint-arwaky-cli"
MCP_SRC="$DIST_DIR/lint-arwaky-mcp"

if [ ! -f "$CLI_SRC" ]; then
    echo "[error] $CLI_SRC not found. Run build.local.sh first."
    exit 1
fi
if [ ! -f "$MCP_SRC" ]; then
    echo "[error] $MCP_SRC not found. Run build.local.sh first."
    exit 1
fi

mkdir -p "$CARGO_BIN"

echo "Installing from dist/..."
cp "$CLI_SRC" "$CARGO_BIN/"
cp "$MCP_SRC" "$CARGO_BIN/"

chmod +x "$CARGO_BIN/lint-arwaky-cli" "$CARGO_BIN/lint-arwaky-mcp"

echo "Done: installed from dist/"
