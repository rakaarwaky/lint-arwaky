#!/usr/bin/env bash
set -euo pipefail

CRATES=(
  "shared-lint-arwaky" "source_parsing-lint-arwaky" "naming_rules-lint-arwaky" "import_rules-lint-arwaky"
  "output_report-lint-arwaky" "pipeline_jobs-lint-arwaky" "code_analysis-lint-arwaky" "auto_fix-lint-arwaky"
  "cli_commands-lint-arwaky" "config_system-lint-arwaky" "file_system-lint-arwaky" "file_watch-lint-arwaky"
  "git_hooks-lint-arwaky" "language_adapters-lint-arwaky" "lifecycle_state-lint-arwaky" "mcp_server-lint-arwaky"
  "metrics_service-lint-arwaky" "multi_project-lint-arwaky" "orphan_detector-lint-arwaky" "plugin_system-lint-arwaky"
  "project_setup-lint-arwaky" "role_rules-lint-arwaky"
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

read -rp "Choose [1-22, a, q]: " choice

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
  [1-9]|1[0-9]|2[0-2])
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
