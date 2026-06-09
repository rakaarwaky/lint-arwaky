#!/usr/bin/env bash
set -euo pipefail

LAYERS=(
  "taxonomy:check_taxonomy:taxonomy layer"
  "contract:check_contract:contract layer (includes taxonomy)"
  "capabilities:check_capabilities:capabilities layer (includes taxonomy + contract)"
  "infrastructure:check_infrastructure:infrastructure layer (includes taxonomy + contract)"
  "agent:check_agent:agent layer (includes taxonomy + contract + capabilities + infrastructure)"
  "surfaces:check_surfaces:surfaces layer (includes all inner layers)"
)

echo "=== Lint Arwaky — Layer Check Menu ==="
echo ""
for i in "${!LAYERS[@]}"; do
  IFS=":" read -r name feature desc <<< "${LAYERS[$i]}"
  echo "  $((i+1))) $name — $desc"
done
echo "  a) All layers (sequentially)"
echo "  q) Quit"
echo ""

read -rp "Choose layer [1-6, a, q]: " choice

case "$choice" in
  a|A)
    echo ""
    for entry in "${LAYERS[@]}"; do
      IFS=":" read -r name feature desc <<< "$entry"
      echo "=== Checking $name ($desc) ==="
      if ! output=$(cargo check --lib --no-default-features --features "$feature" 2>&1); then
        echo "$output"
        echo "=== $name: FAILED ==="
        exit 1
      fi
      echo "=== $name: OK ==="
      echo ""
    done
    echo "=== All layers passed ==="
    ;;
  q|Q)
    echo "Bye."
    exit 0
    ;;
  [1-6])
    idx=$((choice-1))
    IFS=":" read -r name feature desc <<< "${LAYERS[$idx]}"
    echo ""
    echo "=== Checking $name ($desc) ==="
    if ! output=$(cargo check --lib --no-default-features --features "$feature" 2>&1); then
      echo "$output"
      exit 1
    fi
    echo "=== $name: OK ==="
    ;;
  *)
    echo "Invalid choice."
    exit 1
    ;;
esac
