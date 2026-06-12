#!/usr/bin/env bash
set -euo pipefail

CRATES=(
  # LEVEL 0: Foundation
  "shared-lint-arwaky"
  # LEVEL 1: Depends only on shared
  "source_parsing-lint-arwaky"
  # LEVEL 2: Depends on shared + source-parsing
  "file_system-lint-arwaky" "file_watch-lint-arwaky" "metrics_service-lint-arwaky" "multi_project-lint-arwaky" "code_analysis-lint-arwaky"
  # LEVEL 3: Depends on Level 2
  "lifecycle_state-lint-arwaky" "import_rules-lint-arwaky" "output_report-lint-arwaky" "pipeline_jobs-lint-arwaky" "config_system-lint-arwaky" "naming_rules-lint-arwaky" "git_hooks-lint-arwaky" "role_rules-lint-arwaky"
  # LEVEL 4: Depends on Level 3
  "auto_fix-lint-arwaky" "language_adapters-lint-arwaky" "plugin_system-lint-arwaky" "orphan_detector-lint-arwaky" "project_setup-lint-arwaky"
  # LEVEL 5: Depends on Level 4
  "cli_commands-lint-arwaky"
  # LEVEL 6: Top-level
  "mcp_server-lint-arwaky"
)

while true; do
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
      failed=0
      for crate in "${CRATES[@]}"; do
        echo ""
        echo "=== Checking $crate ==="
        if ! cargo check -p "$crate" 2>&1; then
          echo "=== $crate: FAILED ==="
          failed=1
          break
        fi
        echo "=== $crate: OK ==="
      done
      echo ""
      if [ $failed -eq 0 ]; then
        echo "=== All crates passed ==="
      else
        echo "=== Some crates failed ==="
      fi
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
      else
        echo "=== $crate: OK ==="
      fi
      ;;
    *)
      echo "Invalid choice."
      ;;
  esac
  echo ""
  echo "--------------------------------------------------"
  read -rp "Press Enter to return to the main menu... " _unused
  echo ""
done
