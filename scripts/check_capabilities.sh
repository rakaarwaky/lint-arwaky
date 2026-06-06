#!/usr/bin/env bash
set -euo pipefail

echo "=== Checking capabilities layer (includes taxonomy + contract) ==="
cargo check --lib --no-default-features --features check_capabilities 2>&1
echo "=== capabilities: OK ==="
