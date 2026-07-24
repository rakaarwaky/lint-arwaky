#!/usr/bin/env bash
set -euo pipefail

export CARGO_BUILD_JOBS="${CARGO_BUILD_JOBS:-4}"
export RUST_MIN_STACK="${RUST_MIN_STACK:-33554432}"

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
CYAN='\033[0;36m'
NC='\033[0m'

PASSED=0
FAILED=0

TMPDIR=$(mktemp -d)
trap 'rm -rf "$TMPDIR"' EXIT

# Run a gate in background, print result when complete.
# Usage: run_gate "name" command...
run_gate() {
    local name="$1"
    shift
    local out="$TMPDIR/${name// /_}.out"
    local result="$TMPDIR/${name// /_}.res"

    if "$@" > "$out" 2>&1; then
        echo "PASS" > "$result"
    else
        echo "FAIL" > "$result"
    fi
}

wait_and_report() {
    local pids=("$@")
    for pid in "${pids[@]}"; do
        wait "$pid" 2>/dev/null || true
    done

    for f in "$TMPDIR"/*.res; do
        [ -f "$f" ] || continue
        local name
        name=$(basename "$f" .res)
        name="${name//_/ }"
        local out="${f%.res}.out"
        local status
        status=$(cat "$f")

        if [ "$status" = "PASS" ]; then
            echo -e "${GREEN}✅ ${name} PASSED${NC}"
            PASSED=$((PASSED + 1))
        else
            echo -e "${RED}❌ ${name} FAILED${NC}"
            tail -20 "$out" 2>/dev/null || true
            FAILED=$((FAILED + 1))
        fi
    done
    rm -f "$TMPDIR"/*.res "$TMPDIR"/*.out
}

echo -e "${YELLOW}Lint Arwaky — Gate Checker${NC}"
echo "Running all quality gates locally..."
echo "Started: $(date '+%Y-%m-%d %H:%M:%S')"

# ─── Phase 1: Fast static checks (parallel) ───────────────
echo -e "\n${CYAN}━━━ Phase 1: Format + Clippy (parallel) ━━━${NC}"
PIDS=()
run_gate "Rust Format" cargo fmt --all -- --check &
PIDS+=($!)
run_gate "Clippy" cargo clippy --all-targets -- -D warnings &
PIDS+=($!)
wait_and_report "${PIDS[@]}"

# ─── Phase 2: CLI build (shared for self-lint + AES codes) ─
echo -e "\n${CYAN}━━━ Building lint-arwaky-cli ━━━${NC}"
cargo build --bin lint-arwaky-cli 2>&1
echo -e "${GREEN}✅ CLI build complete${NC}"

# ─── Phase 3: Lint gates (parallel, reuse binary) ─────────
echo -e "\n${CYAN}━━━ Phase 3: Self-Lint + AES Codes (parallel) ━━━${NC}"
PIDS=()
run_gate "AES Self-Lint (check . = 0 violations)" bash -c '
    output=$(./target/debug/lint-arwaky-cli check . 2>&1)
    violations=$(echo "$output" | grep "Violations:" | grep -oP "\d+")
    echo "  violations: ${violations:-0}"
    [ "${violations:-0}" = "0" ]
' &
PIDS+=($!)
run_gate "AES Codes (test-workspaces >= 24)" bash -c '
    codes=$(./target/debug/lint-arwaky-cli scan test-workspaces 2>&1 | grep -oP "AES\d+" | sort -u | wc -l)
    echo "  unique codes: ${codes:-0}"
    [ "${codes:-0}" -ge 24 ]
' &
PIDS+=($!)
wait_and_report "${PIDS[@]}"

# ─── Phase 4: Tests (incremental per crate) ────────────────
echo -e "\n${CYAN}━━━ Gate: Tests ━━━${NC}"
total_passed=0
test_failed=0
crates="shared-lint-arwaky code-analysis-lint-arwaky import-rules-lint-arwaky naming-rules-lint-arwaky role-rules-lint-arwaky config-system-lint-arwaky auto-fix-lint-arwaky file-watch-lint-arwaky orphan-detector-lint-arwaky external-lint-lint-arwaky maintenance-lint-arwaky git-hooks-lint-arwaky project-setup-lint-arwaky report-formatter-lint-arwaky cli-commands-lint-arwaky mcp-server-lint-arwaky tui-lint-arwaky"

for crate in $crates; do
    if test_out=$(cargo test -p "$crate" 2>&1); then
        passed=$(echo "$test_out" | grep "^test result:" | sed "s/.*ok\. //" | awk -F";" '{sum+=$1} END{print sum+0}')
        total_passed=$((total_passed + passed))
    else
        echo -e "${RED}❌ Tests FAILED on $crate${NC}"
        echo "$test_out" | grep "^error" | head -10 || true
        test_failed=1
        break
    fi
done

if [ "$test_failed" -eq 0 ]; then
    echo "  passed: ${total_passed}, failed: 0"
    echo -e "${GREEN}✅ Tests PASSED${NC}"
    PASSED=$((PASSED + 1))
else
    FAILED=$((FAILED + 1))
fi

echo -e "\n${CYAN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo -e "Results: ${GREEN}${PASSED} passed${NC}, ${RED}${FAILED} failed${NC}"
echo "Finished: $(date '+%Y-%m-%d %H:%M:%S')"

if [ "$FAILED" -gt 0 ]; then
    exit 1
fi
