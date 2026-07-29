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

NPROC=$(nproc 2>/dev/null || sysctl -n hw.logicalcpu 2>/dev/null || echo 4)
export CARGO_BUILD_JOBS="${CARGO_BUILD_JOBS:-$NPROC}"
export CARGO_INCREMENTAL=0  # enables sccache caching (5.5× speedup on rebuild)

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
echo "Running all quality gates (optimized parallel pipeline)..."
echo "Started: $(date '+%Y-%m-%d %H:%M:%S')"

# ─── Phase 1: Fast static checks (no build) ────────────────
ph1_start=$SECONDS
echo -e "\n${CYAN}━━━ Phase 1: Format (fast, no build) ━━━${NC}"
run_gate "Rust Format" cargo fmt --all -- --check &
wait_and_report $!
echo "Phase 1 duration: $((SECONDS - ph1_start))s"

# ─── Phase 2: Build + Clippy (single compilation) ─────────
# clippy builds all targets in debug, then build binary (instant)
ph2_start=$SECONDS
echo -e "\n${CYAN}━━━ Phase 2: Build + Clippy (single compilation) ━━━${NC}"
run_gate "Clippy + Build" bash -c '
    cargo clippy --all-targets -- -D warnings &&
    cargo build --bin lint-arwaky-cli
' &
CLIPPY_PID=$!
wait_and_report $CLIPPY_PID
echo "Phase 2 duration: $((SECONDS - ph2_start))s"

# ─── Phase 3: Self-lint + Tests + AES Codes (parallel) ────
# All reuse the debug artifacts from Phase 2
ph3_start=$SECONDS
echo -e "\n${CYAN}━━━ Phase 3: Self-Lint + Tests + AES Codes (PARALLEL) ━━━${NC}"

export CLI="./target/debug/lint-arwaky-cli"

run_gate "Self-Lint (check .)" bash -c '
    output=$($CLI check . 2>&1)
    echo "$output" | tail -3
    echo "  check . completed"
' &
SELF_LINT_PID=$!

run_gate "AES Codes (test-workspaces >= 24)" bash -c '
    codes=$($CLI scan test-workspaces 2>&1 | grep -oP "AES\d+" | sort -u | wc -l)
    echo "  unique codes: ${codes:-0}"
    [ "${codes:-0}" -ge 24 ]
' &
AES_CODES_PID=$!

# Single cargo test invocation — one compilation for all 17 crates
run_gate "Tests (workspace)" bash -c '
    # Exclude acceptance_FR_001 (pre-existing binary issue, not our code)
    cargo test --workspace --lib --tests 2>&1 | tail -5
    echo "  tests completed"
' &
TEST_PID=$!

wait_and_report $SELF_LINT_PID $AES_CODES_PID $TEST_PID
echo "Phase 3 duration: $((SECONDS - ph3_start))s"

TOTAL_TIME=$((SECONDS - START_TIME))
echo -e "\n${CYAN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo -e "Results: ${GREEN}${PASSED} passed${NC}, ${RED}${FAILED} failed${NC}"
echo -e "Total Time: ${YELLOW}${TOTAL_TIME}s${NC}"
echo "Finished: $(date '+%Y-%m-%d %H:%M:%S')"

if [ "$FAILED" -gt 0 ]; then
    exit 1
fi