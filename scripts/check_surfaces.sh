#!/usr/bin/env bash
set -euo pipefail

echo "=== Checking surfaces layer (includes all inner layers) ==="
cargo check --lib --no-default-features --features check_surfaces 2>&1
echo "=== surfaces: OK ==="
