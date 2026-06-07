#!/usr/bin/env bash
set -euo pipefail

echo "=== Checking capabilities layer (includes taxonomy + contract) ==="
if ! output=$(cargo check --lib --no-default-features --features check_capabilities 2>&1); then
  echo "$output"
  exit 1
fi
echo "=== capabilities: OK ==="
