#!/bin/bash
# Install Graph-It-Live extension to Antigravity IDE
# Usage: ./scripts/install_graphit_live.sh

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"
EXTENSION_DIR="$PROJECT_ROOT/Graph-It-Live-arwaky"
VsixFile="$EXTENSION_DIR/graph-it-live-0.0.1.vsix"

# Antigravity extensions directory
ANTIGRAVITY_EXT_DIR="$HOME/.antigravity-ide/extensions/magic5644.graph-it-live-0.0.1"
# Cache directory
ANTIGRAVITY_CACHE_DIR="$HOME/.antigravity-ide/CachedExtensionVSIXs"

echo "=== Building Graph-It-Live extension ==="
cd "$EXTENSION_DIR"

rm -f "$VsixFile"
npm run package

if [ ! -f "$VsixFile" ]; then
    echo "Error: Build failed, VSIX not found"
    exit 1
fi

echo ""
echo "=== Uninstalling old extension ==="
if [ -d "$ANTIGRAVITY_EXT_DIR" ]; then
    rm -rf "$ANTIGRAVITY_EXT_DIR"
    echo "Deleted: $ANTIGRAVITY_EXT_DIR"
fi

# Clear VSIX cache so Antigravity re-reads fresh
if [ -d "$ANTIGRAVITY_CACHE_DIR" ]; then
    rm -rf "$ANTIGRAVITY_CACHE_DIR"
    echo "Cleared cache: $ANTIGRAVITY_CACHE_DIR"
fi

echo ""
echo "=== Installing new extension ==="
TEMP_DIR="/tmp/graphitlive-install-$$"
mkdir -p "$TEMP_DIR"

unzip -o "$VsixFile" -d "$TEMP_DIR" > /dev/null

mkdir -p "$ANTIGRAVITY_EXT_DIR"
cp -r "$TEMP_DIR/extension/"* "$ANTIGRAVITY_EXT_DIR/"

rm -rf "$TEMP_DIR"

echo ""
echo "=== Done ==="
echo "Extension installed to: $ANTIGRAVITY_EXT_DIR"
echo ""
echo "IMPORTANT: You must do BOTH:"
echo "  1. Close Antigravity completely (Ctrl+Q)"
echo "  2. Reopen Antigravity"
echo ""
echo "Just reloading the window is NOT enough."
echo "The extension cache must be cleared for changes to take effect."
