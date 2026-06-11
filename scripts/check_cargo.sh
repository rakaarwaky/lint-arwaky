#!/usr/bin/env bash
set -euo pipefail

CRATES=(
  "shared_common" "source_parsing" "naming_rules" "import_rules"
  "output_report" "pipeline_jobs" "code_analysis" "auto_fix"
  "cli_commands" "cli_transport" "config_system" "di_containers"
  "file_system" "file_watch" "git_hooks" "language_adapters"
  "lifecycle_state" "mcp_server" "metrics_service" "multi_project"
  "orphan_detector" "plugin_system" "project_setup" "role_rules"
)

echo "=== Lint Arwaky — Cargo Check All Crates ==="
echo ""
echo "Select crates to check:"
for i in "${!CRATES[@]}"; do
  echo "  $((i+1))) ${CRATES[$i]}"
done
echo "  a) All crates (sequentially)"
echo "  q) Quit"
echo ""

read -rp "Choose [1-24, a, q]: " choice

case "$choice" in
  a|A)
    for crate in "${CRATES[@]}"; do
      echo ""
      echo "=== Checking $crate ==="
      if ! cargo check -p "$crate" 2>&1; then
        echo "=== $crate: FAILED ==="
        exit 1
      fi
      echo "=== $crate: OK ==="
    done
    echo ""
    echo "=== All crates passed ==="
    ;;
  q|Q)
    echo "Bye."
    exit 0
    ;;
  [1-9]|1[0-9]|2[0-4])
    idx=$((choice-1))
    crate="${CRATES[$idx]}"
    echo ""
    echo "=== Checking $crate ==="
    if ! cargo check -p "$crate" 2>&1; then
      echo "=== $crate: FAILED ==="
      exit 1
    fi
    echo "=== $crate: OK ==="
    ;;
  *)
    echo "Invalid choice."
    exit 1
    ;;
esac
