#!/usr/bin/env bash
set -euo pipefail

echo "=== Checking contract layer (includes taxonomy) ==="
if ! output=$(cargo check --lib --no-default-features --features check_contract 2>&1); then
  echo "$output"
  exit 1
fi
echo "=== contract: OK ==="
