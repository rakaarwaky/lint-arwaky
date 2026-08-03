#!/usr/bin/env bash
# install.sh — Install extension ke Antigravity IDE
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
IDE_DIR="/home/raka/App/Antigravity IDE"
EXT_DIR="$HOME/.antigravity-ide/extensions"
PKG_NAME="lint-arwaky-vscode"
PKG_VERSION="0.0.1"
TARGET="$EXT_DIR/lint-arwaky.lint-arwaky-vscode-$PKG_VERSION-universal"

echo "==> Building..."
cd "$SCRIPT_DIR"
npx tsc -p ./

echo "==> Installing to $TARGET ..."
rm -rf "$TARGET"
mkdir -p "$TARGET"
cp -r dist/ package.json src/ "$TARGET/"

echo "==> Done. Restart Antigravity IDE to load the extension."
