#!/usr/bin/env bash
set -euo pipefail

echo "=== Checking infrastructure layer (includes taxonomy + contract) ==="
if ! output=$(cargo check --lib --no-default-features --features check_infrastructure 2>&1); then
  echo "$output"
  exit 1
fi
echo "=== infrastructure: OK ==="
