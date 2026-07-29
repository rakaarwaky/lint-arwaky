#!/usr/bin/env bash
set -euo pipefail

usage() {
    echo "Usage: bash scripts/gates.sh [options]"
    echo ""
    echo "Runs all quality gates: format, clippy, self-lint, AES codes, tests."
    echo ""
    echo "Options:"
    echo "  -h, --help    Show this help"
    exit 0
}

case "${1:-}" in
    -h|--help) usage ;;
esac

export CARGO_BUILD_JOBS="${CARGO_BUILD_JOBS:-2}"
export CARGO_INCREMENTAL=0
export CARGO_TARGET_X86_64_UNKNOWN_LINUX_GNU_LINKER="${CARGO_TARGET_X86_64_UNKNOWN_LINUX_GNU_LINKER:-cc}"

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
CYAN='\033[0;36m'
NC='\033[0m'

PASSED=0
FAILED=0

TMPDIR=$(mktemp -d)
trap 'rm -rf "$TMPDIR"' EXIT

START_TIME=$SECONDS

run_gate() {
    local name="$1"
    shift
    local out="$TMPDIR/${name// /_}.out"
    local result="$TMPDIR/${name// /_}.res"
    local g_start=$SECONDS

    if "$@" > "$out" 2>&1; then
        local dur=$((SECONDS - g_start))
        echo "PASS|${dur}" > "$result"
    else
        local dur=$((SECONDS - g_start))
        echo "FAIL|${dur}" > "$result"
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
        local res_content
        res_content=$(cat "$f")
        local status="${res_content%%|*}"
        local dur="${res_content##*|}"

        if [ "$status" = "PASS" ]; then
            echo -e "${GREEN}✅ ${name} PASSED (${dur}s)${NC}"
            PASSED=$((PASSED + 1))
        else
            echo -e "${RED}❌ ${name} FAILED (${dur}s)${NC}"
            tail -n 20 "$out" 2>/dev/null || true
            FAILED=$((FAILED + 1))
        fi
    done
    rm -f "$TMPDIR"/*.res "$TMPDIR"/*.out
}

echo -e "${YELLOW}Lint Arwaky — Gate Checker${NC}"
echo "Running all quality gates locally (PARALLEL)..."
echo "Started: $(date '+%Y-%m-%d %H:%M:%S')"

# ─── Stage 1: Static checks + CLI build (PARALLEL) ──────
st1_start=$SECONDS
echo -e "\n${CYAN}━━━ Stage 1: Format + Clippy + CLI Build (PARALLEL) ━━━${NC}"

run_gate "Rust Format" cargo fmt --all -- --check &
RUN_FMT_PID=$!
run_gate "Clippy" cargo clippy --all-targets -- -D warnings &
RUN_CLIPY_PID=$!
run_gate "CLI Build" cargo build --bin lint-arwaky-cli 2>&1 &
RUN_BUILD_PID=$!

wait_and_report $RUN_FMT_PID $RUN_CLIPY_PID $RUN_BUILD_PID
echo "Stage 1 duration: $((SECONDS - st1_start))s"

# ─── Stage 2: Self-Lint + AES Codes (PARALLEL, needs CLI) ──
st2_start=$SECONDS
echo -e "\n${CYAN}━━━ Stage 2: Self-Lint + AES Codes (PARALLEL) ━━━${NC}"
run_gate "AES Self-Lint (check . = 0 violations)" bash -c '
    output=$(./target/debug/lint-arwaky-cli check . 2>&1)
    violations=$(echo "$output" | grep "Violations:" | grep -oP "\d+")
    echo "  violations: ${violations:-0}"
    [ "${violations:-0}" = "0" ]
' &
RUN_AES_LINT_PID=$!
run_gate "AES Codes (test-workspaces >= 24)" bash -c '
    codes=$(./target/debug/lint-arwaky-cli scan test-workspaces 2>&1 | grep -oP "AES\d+" | sort -u | wc -l)
    echo "  unique codes: ${codes:-0}"
    [ "${codes:-0}" -ge 24 ]
' &
RUN_AES_CODES_PID=$!

wait_and_report $RUN_AES_LINT_PID $RUN_AES_CODES_PID
echo "Stage 2 duration: $((SECONDS - st2_start))s"

# ─── Stage 3: Tests (PARALLEL crate-by-crate) ─────────────
st3_start=$SECONDS
echo -e "\n${CYAN}━━━ Stage 3: Tests (PARALLEL, $CARGO_BUILD_JOBS jobs) ━━━${NC}"

# Run all crates in parallel using run_gate
declare -a TEST_PIDS=()
for crate in shared-lint-arwaky code-analysis-lint-arwaky import-rules-lint-arwaky naming-rules-lint-arwaky role-rules-lint-arwaky config-system-lint-arwaky auto-fix-lint-arwaky file-watch-lint-arwaky orphan-detector-lint-arwaky external-lint-lint-arwaky maintenance-lint-arwaky git-hooks-lint-arwaky project-setup-lint-arwaky report-formatter-lint-arwaky cli-commands-lint-arwaky mcp-server-lint-arwaky tui-lint-arwaky; do
    run_gate "Test $crate" cargo test -p "$crate" --lib --tests 2>&1 &
    TEST_PIDS+=($!)
done

wait_and_report "${TEST_PIDS[@]}"
echo "Stage 3 duration: $((SECONDS - st3_start))s"

TOTAL_TIME=$((SECONDS - START_TIME))
echo -e "\n${CYAN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo -e "Results: ${GREEN}${PASSED} passed${NC}, ${RED}${FAILED} failed${NC}"
echo -e "Total Time: ${YELLOW}${TOTAL_TIME}s${NC}"
echo "Finished: $(date '+%Y-%m-%d %H:%M:%S')"

if [ "$FAILED" -gt 0 ]; then
    exit 1
fi
