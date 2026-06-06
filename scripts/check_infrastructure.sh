#!/usr/bin/env bash
set -euo pipefail

echo "=== Checking infrastructure layer (includes taxonomy + contract) ==="
cargo check --lib --no-default-features --features check_infrastructure 2>&1
echo "=== infrastructure: OK ==="
