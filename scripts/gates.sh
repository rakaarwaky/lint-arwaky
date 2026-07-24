#!/usr/bin/env bash
set -euo pipefail

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
CYAN='\033[0;36m'
NC='\033[0m'

PASSED=0
FAILED=0
GATE_RESULTS=""

# Run a gate in background, capture result to a temp file.
# Usage: gate_bg "name" command...
gate_bg() {
    local name="$1"
    shift
    local tmpfile
    tmpfile=$(mktemp)
    (
        if "$@" > "$tmpfile" 2>&1; then
            echo "PASS" > "$tmpfile.result"
        else
            echo "FAIL" > "$tmpfile.result"
        fi
    ) &
    GATE_RESULTS="${GATE_RESULTS}${name}|${tmpfile}\n"
}

# Wait for all background gates and print results.
wait_and_report() {
    local total=0
    local failed=0

    while IFS= read -r entry; do
        local name="${entry%%|*}"
        local tmpfile="${entry##*|}"

        # Wait for background process (we don't track PID, but the file won't exist yet)
        # Sleep briefly to let bg jobs finish
        local wait_count=0
        while [ ! -f "${tmpfile}.result" ] && [ "$wait_count" -lt 120 ]; do
            sleep 0.1
            wait_count=$((wait_count + 1))
        done

        local result
        result=$(cat "${tmpfile}.result" 2>/dev/null || echo "FAIL")
        rm -f "$tmpfile" "${tmpfile}.result"

        total=$((total + 1))
        if [ "$result" = "PASS" ]; then
            echo -e "${GREEN}✅ ${name} PASSED${NC}"
            PASSED=$((PASSED + 1))
        else
            echo -e "${RED}❌ ${name} FAILED${NC}"
            # Show last 20 lines of output on failure
            tail -20 "$tmpfile" 2>/dev/null || true
            FAILED=$((FAILED + 1))
        fi
    done < <(printf '%b' "$GATE_RESULTS")
}

echo -e "${YELLOW}Lint Arwaky — Gate Checker${NC}"
echo "Running all quality gates locally..."
echo "Started: $(date '+%Y-%m-%d %H:%M:%S')"

# ─── Phase 1: Fast static checks (parallel) ───────────────
gate_bg "Rust Format" cargo fmt --all -- --check
gate_bg "Clippy" cargo clippy --all-targets -- -D warnings
wait_and_report

# ─── Phase 2: CLI build (shared for self-lint + AES codes) ─
echo -e "\n${CYAN}━━━ Building lint-arwaky-cli ━━━${NC}"
cargo build --bin lint-arwaky-cli 2>&1
echo -e "${GREEN}✅ CLI build complete${NC}"

# ─── Phase 3: Lint gates (parallel, reuse binary) ─────────
gate_bg "AES Self-Lint (check . = 0 violations)" bash -c '
    output=$(cargo run --bin lint-arwaky-cli -- check . 2>&1)
    violations=$(echo "$output" | grep "Violations:" | grep -oP "\d+")
    echo "  violations: ${violations:-0}"
    [ "${violations:-0}" = "0" ]
'
gate_bg "AES Codes (test-workspaces >= 24)" bash -c '
    codes=$(cargo run --bin lint-arwaky-cli -- scan test-workspaces 2>&1 | grep -oP "AES\d+" | sort -u | wc -l)
    echo "  unique codes: ${codes:-0}"
    [ "${codes:-0}" -ge 24 ]
'
wait_and_report

# ─── Phase 4: Tests (uses cargo test, incremental from clippy build) ─
echo -e "\n${CYAN}━━━ Gate: Tests ━━━${NC}"
test_output=$(cargo test --workspace --lib --tests 2>&1) || {
    echo -e "${RED}❌ Tests FAILED (compilation/runtime)${NC}"
    echo "$test_output" | grep "^error" | head -10 || true
    FAILED=$((FAILED + 1))
    # Continue to print summary
    test_output=""
}
if [ -n "$test_output" ]; then
    # Single awk pass instead of grep|sed|awk chain
    read -r total_passed total_failed <<< $(echo "$test_output" | awk '/^test result:/{for(i=1;i<=NF;i++){if($i=="ok.")p=$(i+1); if($i=="failed")f=$(i-1)}} END{gsub(/;/,"",p); gsub(/;/,"",f); print p+0, f+0}')
    echo "  passed: ${total_passed}, failed: ${total_failed}"
    if [ "${total_failed}" = "0" ]; then
        echo -e "${GREEN}✅ Tests PASSED${NC}"
        PASSED=$((PASSED + 1))
    else
        echo -e "${RED}❌ Tests FAILED${NC}"
        FAILED=$((FAILED + 1))
    fi
fi

echo -e "\n${CYAN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo -e "Results: ${GREEN}${PASSED} passed${NC}, ${RED}${FAILED} failed${NC}"
echo "Finished: $(date '+%Y-%m-%d %H:%M:%S')"

if [ "$FAILED" -gt 0 ]; then
    exit 1
fi
