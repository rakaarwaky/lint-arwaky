#!/usr/bin/env bash
set -euo pipefail

echo "=== Checking taxonomy layer ==="
cargo check --lib --no-default-features --features check_taxonomy 2>&1
echo "=== taxonomy: OK ==="
