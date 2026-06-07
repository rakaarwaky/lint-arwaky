#!/usr/bin/env bash
set -euo pipefail

echo "=== Checking agent layer (includes taxonomy + contract + capabilities + infrastructure) ==="
if ! output=$(cargo check --lib --no-default-features --features check_agent 2>&1); then
  echo "$output"
  exit 1
fi
echo "=== agent: OK ==="
