#!/usr/bin/env bash
set -euo pipefail

echo "=== Checking taxonomy layer ==="
if ! output=$(cargo check --lib --no-default-features --features check_taxonomy 2>&1); then
  echo "$output"
  exit 1
fi
echo "=== taxonomy: OK ==="
