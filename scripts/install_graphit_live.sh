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

echo "Building Graph-It-Live extension..."
cd "$EXTENSION_DIR"

# Clean previous build
rm -f "$VsixFile"

# Build
npm run package

if [ ! -f "$VsixFile" ]; then
    echo "Error: Build failed, VSIX not found"
    exit 1
fi

echo "Installing to Antigravity IDE..."

# Create temp directory for extraction
TEMP_DIR="/tmp/graphitlive-install-$$"
mkdir -p "$TEMP_DIR"

# Extract VSIX
unzip -o "$VsixFile" -d "$TEMP_DIR" > /dev/null

# Copy files to extension directory
mkdir -p "$ANTIGRAVITY_EXT_DIR"
cp -r "$TEMP_DIR/extension/"* "$ANTIGRAVITY_EXT_DIR/"

# Cleanup
rm -rf "$TEMP_DIR"

echo "Extension installed to: $ANTIGRAVITY_EXT_DIR"
echo "Please reload Antigravity window (Ctrl+Shift+P -> Reload Window)"
