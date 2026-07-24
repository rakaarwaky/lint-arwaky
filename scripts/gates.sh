#!/usr/bin/env bash
set -euo pipefail

export CARGO_BUILD_JOBS="${CARGO_BUILD_JOBS:-4}"
export RUST_MIN_STACK="${RUST_MIN_STACK:-134217728}"

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
echo "Running all quality gates locally..."
echo "Started: $(date '+%Y-%m-%d %H:%M:%S')"

# ─── Stage 1: Fast static checks (parallel) ───────────────
st1_start=$SECONDS
echo -e "\n${CYAN}━━━ Stage 1: Format + Clippy (parallel) ━━━${NC}"
PIDS=()
run_gate "Rust Format" cargo fmt --all -- --check &
PIDS+=($!)
run_gate "Clippy" cargo clippy --all-targets -- -D warnings &
PIDS+=($!)
wait_and_report "${PIDS[@]}"
echo "Stage 1 duration: $((SECONDS - st1_start))s"

# ─── Stage 2: CLI build (shared for self-lint + AES codes) ─
st2_start=$SECONDS
echo -e "\n${CYAN}━━━ Stage 2: Building lint-arwaky-cli ━━━${NC}"
cargo build --bin lint-arwaky-cli 2>&1
echo -e "${GREEN}✅ CLI build complete ($((SECONDS - st2_start))s)${NC}"

# ─── Stage 3: Self-Lint + AES Codes (parallel) ────────────
st3_start=$SECONDS
echo -e "\n${CYAN}━━━ Stage 3: Self-Lint + AES Codes (parallel) ━━━${NC}"
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
echo "Stage 3 duration: $((SECONDS - st3_start))s"

# ─── Stage 4: Tests (cargo nextest if available, fallback to cargo test)
st4_start=$SECONDS
echo -e "\n${CYAN}━━━ Stage 4: Tests ━━━${NC}"
if cargo nextest --version &> /dev/null; then
    echo "  Using cargo-nextest runner..."
    if test_out=$(cargo nextest run --workspace --lib --tests -j 4 2>&1); then
        echo "$test_out" | tail -n 5
        echo -e "${GREEN}✅ Tests PASSED (nextest, $((SECONDS - st4_start))s)${NC}"
        PASSED=$((PASSED + 1))
    else
        echo -e "${RED}❌ Tests FAILED (nextest, $((SECONDS - st4_start))s)${NC}"
        echo "$test_out" | tail -n 15 || true
        FAILED=$((FAILED + 1))
    fi
elif test_out=$(cargo test --workspace --lib --tests --no-fail-fast 2>&1); then
    passed=$(echo "$test_out" | grep "^test result:" | sed "s/.*ok\. //" | awk -F";" '{sum+=$1} END{print sum+0}')
    failed=$(echo "$test_out" | grep "^test result:" | sed "s/.*ok\. //" | awk -F";" '{sum+=$2} END{print sum+0}')
    echo "  passed: ${passed:-0}, failed: ${failed:-0}"
    if [ "${failed:-0}" -eq 0 ]; then
        echo -e "${GREEN}✅ Tests PASSED ($((SECONDS - st4_start))s)${NC}"
        PASSED=$((PASSED + 1))
    else
        echo -e "${RED}❌ Tests FAILED ($((SECONDS - st4_start))s)${NC}"
        FAILED=$((FAILED + 1))
    fi
else
    echo -e "${RED}❌ Tests FAILED (compilation/runtime, $((SECONDS - st4_start))s)${NC}"
    echo "$test_out" | grep "^error" | head -10 || true
    FAILED=$((FAILED + 1))
fi

TOTAL_TIME=$((SECONDS - START_TIME))
echo -e "\n${CYAN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo -e "Results: ${GREEN}${PASSED} passed${NC}, ${RED}${FAILED} failed${NC}"
echo -e "Total Time: ${YELLOW}${TOTAL_TIME}s${NC}"
echo "Finished: $(date '+%Y-%m-%d %H:%M:%S')"

if [ "$FAILED" -gt 0 ]; then
    exit 1
fi
