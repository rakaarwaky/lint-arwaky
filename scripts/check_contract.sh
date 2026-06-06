#!/usr/bin/env bash
set -euo pipefail

echo "=== Checking contract layer (includes taxonomy) ==="
cargo check --lib --no-default-features --features check_contract 2>&1
echo "=== contract: OK ==="
