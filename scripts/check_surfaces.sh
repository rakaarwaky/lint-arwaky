#!/usr/bin/env bash
set -euo pipefail

echo "=== Checking surfaces layer (includes all inner layers) ==="
if ! output=$(cargo check --lib --no-default-features --features check_surfaces 2>&1); then
  echo "$output"
  exit 1
fi
echo "=== surfaces: OK ==="
