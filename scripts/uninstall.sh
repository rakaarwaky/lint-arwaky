#!/usr/bin/env bash
# scripts/uninstall.sh — Clean uninstaller for lint-arwaky (XDG Base Directory)
set -euo pipefail

TOOL_NAME="lint-arwaky"
BIN_DIR="${XDG_BIN_HOME:-$HOME/.local/bin}"
DATA_DIR="${XDG_DATA_HOME:-$HOME/.local/share}/$TOOL_NAME"
CONFIG_DIR="${XDG_CONFIG_HOME:-$HOME/.config}/$TOOL_NAME"
CACHE_DIR="${XDG_CACHE_HOME:-$HOME/.cache}/$TOOL_NAME"
STATE_DIR="${XDG_STATE_HOME:-$HOME/.local/state}/$TOOL_NAME"
PROJECT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

# Additional lint-arwaky specific directories
REPORT_DIR="$DATA_DIR/reports"

usage() {
    echo "Usage: bash scripts/uninstall.sh [options]"
    echo ""
    echo "Removes installed binaries, config, cache, and data."
    echo ""
    echo "Options:"
    echo "  --purge       Remove all data including persistent data"
    echo "  --dry-run     Show what would be removed without removing"
    echo "  -h, --help    Show this help"
    exit 0
}

DRY_RUN=false
PURGE=false

while [[ $# -gt 0 ]]; do
    case "$1" in
        --purge)    PURGE=true; shift ;;
        --dry-run)  DRY_RUN=true; shift ;;
        -h|--help)  usage ;;
        *)          echo "Unknown option: $1 (use -h for help)"; exit 1 ;;
    esac
done

remove_item() {
    local path="$1"
    if [ -f "$path" ] || [ -d "$path" ] || [ -L "$path" ]; then
        if $DRY_RUN; then
            echo "  [dry-run] Would remove: $path"
        else
            rm -rf "$path"
            echo "✓ Removed $path"
        fi
    fi
}

echo "=== Uninstalling $TOOL_NAME ==="

# Remove bin launchers
COMMANDS=("lint-arwaky" "la" "lint-arwaky-cli" "lint-arwaky-mcp" "lint-arwaky-tui" "lac")
for cmd in "${COMMANDS[@]}"; do
    remove_item "$BIN_DIR/$cmd"
done

# Remove in-tree .venv/venv symlinks and target/ directory
for name in ".venv" "venv" "target"; do
    remove_item "$PROJECT_DIR/$name"
done

# Remove XDG config & cache (always)
if [ -d "$CONFIG_DIR" ]; then
    remove_item "$CONFIG_DIR"
fi
if [ -d "$CACHE_DIR" ]; then
    remove_item "$CACHE_DIR"
fi

# Remove reports (always)
if [ -d "$REPORT_DIR" ]; then
    remove_item "$REPORT_DIR"
fi

# Remove data & state with --purge
if $PURGE; then
    for d in "$DATA_DIR" "$STATE_DIR"; do
        if [ -d "$d" ]; then
            remove_item "$d"
        fi
    done
fi

echo ""
if $DRY_RUN; then
    echo "Dry run complete. No files were removed."
else
    echo "Uninstall complete."
fi
