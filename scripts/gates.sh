#!/usr/bin/env bash
set -euo pipefail

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
CYAN='\033[0;36m'
NC='\033[0m'

PASSED=0
FAILED=0

gate() {
    local name="$1"
    shift
    echo -e "\n${CYAN}━━━ Gate: ${name} ━━━${NC}"
    if "$@"; then
        echo -e "${GREEN}✅ ${name} PASSED${NC}"
        PASSED=$((PASSED + 1))
    else
        echo -e "${RED}❌ ${name} FAILED${NC}"
        FAILED=$((FAILED + 1))
    fi
}

echo -e "${YELLOW}Lint Arwaky — Gate Checker${NC}"
echo "Running all quality gates locally..."
echo "Started: $(date '+%Y-%m-%d %H:%M:%S')"

gate "Rust Format" cargo fmt --all -- --check

gate "Clippy" cargo clippy --all-targets -- -D warnings

gate "AES Self-Lint (check . = 0 violations)" bash -c '
    output=$(cargo run --bin lint-arwaky-cli -- check . 2>&1)
    violations=$(echo "$output" | grep "Violations:" | grep -oP "\d+")
    echo "  violations: ${violations:-0}"
    [ "${violations:-0}" = "0" ]
'

gate "AES Codes (test-workspaces >= 24)" bash -c '
    codes=$(cargo run --bin lint-arwaky-cli -- scan test-workspaces 2>&1 | grep -oP "AES\d+" | sort -u | wc -l)
    echo "  unique codes: ${codes:-0}"
    [ "${codes:-0}" -ge 24 ]
'

gate "Tests" bash -c '
    total_passed=0
    total_failed=0
    for crate in shared-lint-arwaky code-analysis-lint-arwaky import-rules-lint-arwaky naming-rules-lint-arwaky role-rules-lint-arwaky config-system-lint-arwaky auto-fix-lint-arwaky file-watch-lint-arwaky orphan-detector-lint-arwaky external-lint-lint-arwaky maintenance-lint-arwaky git-hooks-lint-arwaky project-setup-lint-arwaky report-formatter-lint-arwaky cli-commands-lint-arwaky mcp-server-lint-arwaky tui-lint-arwaky; do
        output=$(cargo test -p "$crate" 2>&1) || {
            echo "  FAILED on $crate"
            echo "$output" | grep "^error" | head -10 || true
            exit 1
        }
        passed=$(echo "$output" | grep "^test result:" | sed "s/.*ok\. //" | awk -F";" "{sum+=\$1} END{print sum+0}")
        failed=$(echo "$output" | grep "^test result:" | sed "s/.*ok\. //" | awk -F";" "{sum+=\$2} END{print sum+0}")
        total_passed=$((total_passed + passed))
        total_failed=$((total_failed + failed))
    done
    echo "  passed: ${total_passed}, failed: ${total_failed}"
    [ "${total_failed}" = "0" ]
'

echo -e "\n${CYAN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo -e "Results: ${GREEN}${PASSED} passed${NC}, ${RED}${FAILED} failed${NC}"
echo "Finished: $(date '+%Y-%m-%d %H:%M:%S')"

if [ "$FAILED" -gt 0 ]; then
    exit 1
fi
