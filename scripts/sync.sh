#!/usr/bin/env bash
# ==============================================================================
# sync.sh — synchronizes local main and develop branches with origin
# ==============================================================================
set -euo pipefail

usage() {
    echo "Usage: bash scripts/sync.sh [options]"
    echo ""
    echo "Synchronizes local main and develop branches with origin."
    echo "WARNING: This resets local main to origin/main (destructive)."
    echo ""
    echo "Options:"
    echo "  -h, --help    Show this help"
    exit 0
}

case "${1:-}" in
    -h|--help) usage ;;
esac

# Ensure we run from repository root
ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR"

echo "=== Fetching all changes from origin ==="
env -u GITHUB_TOKEN git fetch --all

echo ""
echo "=== Resetting local main to origin/main ==="
git checkout main
git reset --hard origin/main

echo ""
echo "=== Merging main into develop ==="
git checkout develop
git merge main

echo ""
echo "=== Pushing updated develop to origin ==="
env -u GITHUB_TOKEN git push origin develop

echo ""
echo "=== Branch synchronization complete ==="
git status
