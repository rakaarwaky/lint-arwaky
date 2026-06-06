#!/usr/bin/env bash
set -euo pipefail

echo "=== Checking agent layer (includes taxonomy + contract + capabilities + infrastructure) ==="
cargo check --lib --no-default-features --features check_agent 2>&1
echo "=== agent: OK ==="
