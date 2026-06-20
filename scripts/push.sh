#!/usr/bin/env bash
# push.sh — lightweight: build → lint → verify → commit → push
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR"

GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
CYAN='\033[0;36m'
BOLD='\033[1m'
NC='\033[0m'

pass() { echo -e " ${GREEN}✔${NC} $1"; }
warn() { echo -e " ${YELLOW}⚠${NC} $1"; }
fail() { echo -e " ${RED}✘${NC} $1"; }
die()  { fail "$1"; exit 1; }

COMMIT_MSG=""
TARGET="lint-arwaky-cli"
SKIP_BUILD=false
FAST=false
AUTO=false

while [[ $# -gt 0 ]]; do
  case "$1" in
    -m|--message) COMMIT_MSG="$2"; shift 2 ;;
    --target) TARGET="$2"; shift 2 ;;
    --no-build) SKIP_BUILD=true; shift ;;
    --fast) FAST=true; shift ;;
    -y|--yes) AUTO=true; shift ;;
    -h|--help)
      echo "Usage: bash scripts/push.sh [options]"
      echo "  -m, --message MSG   Commit message (interactive if omitted)"
      echo "  --target TARGET      Binary to build (default: lint-arwaky-cli)"
      echo "  --no-build           Skip build step"
      echo "  --fast               Skip fmt + clippy"
      echo "  -y, --yes            Auto-confirm"
      exit 0 ;;
    *) die "Unknown option: $1 (use -h for help)" ;;
  esac
done

CI_EXIT=0
ci_step() {
  local name="$1"; shift
  echo ""
  echo "  === $name ==="
  if eval "$@"; then
    pass "$name"
  else
    fail "$name"
    CI_EXIT=1
  fi
}

# ── 1. Build ──
echo ""
echo -e "${BOLD}━━━ [1/4] Build ━━━${NC}"
if $SKIP_BUILD; then
  warn "Build skipped"
else
  ci_step "cargo build --bin $TARGET" "cargo build --bin '$TARGET' 2>&1"
  [ $CI_EXIT -ne 0 ] && die "Build failed."
  pass "Build OK"
fi

# ── 2. Lint ──
echo ""
echo -e "${BOLD}━━━ [2/4] Self-Lint ━━━${NC}"
ci_step "self-lint (AES)" "timeout 60 ./target/debug/$TARGET check crates/ --filter AES 2>&1" || true
if [ $CI_EXIT -ne 0 ] && [ "$AUTO" = false ]; then
  read -rp "Lint issues found. Continue? [y/N] " ans
  [[ "$ans" =~ ^[yY] ]] || die "Aborted."
fi

# ── 3. Verify ──
echo ""
echo -e "${BOLD}━━━ [3/4] Verify ━━━${NC}"
CI_EXIT=0
if ! $FAST; then
  ci_step "cargo fmt --check" "cargo fmt --check 2>&1"
  ci_step "cargo clippy" "cargo clippy --all-targets -- -D warnings 2>&1 | tail -20"
fi
if [ $CI_EXIT -ne 0 ]; then
  warn "Verify failed. Fix issues and retry."
  [ "$AUTO" = false ] && read -rp "Continue anyway? [y/N] " ans && [[ "$ans" =~ ^[yY] ]] || die "Aborted."
fi

# ── 4. Commit + Push ──
echo ""
echo -e "${BOLD}━━━ [4/4] Commit & Push ━━━${NC}"

if ! command -v jj &>/dev/null; then
  die "jj (Jujutsu) not found."
fi

PENDING=$(jj st 2>/dev/null | grep -c "^M\|^A\|^+\|^-" || true) || true
if [ "${PENDING:-0}" -eq 0 ]; then
  info "No pending changes."
else
  jj st 2>/dev/null || true
  echo ""

  if [ -z "$COMMIT_MSG" ]; then
    echo "Enter commit message (conventional commit):"
    echo "  fix:     Bug fix"
    echo "  feat:    New feature"
    echo "  chore:   Maintenance"
    echo "  refactor: Code restructuring"
    echo ""
    read -rp "Message: " COMMIT_MSG
    [ -z "$COMMIT_MSG" ] && COMMIT_MSG="chore: auto-push $(date +%Y-%m-%d)"
  fi

  if [ "$AUTO" = false ]; then
    read -rp "Commit & push? [Y/n] " ans
    [[ "$ans" =~ ^[nN] ]] && die "Aborted."
  fi

  jj describe -m "$COMMIT_MSG"
  pass "Committed: $COMMIT_MSG"

  jj git push 2>&1 && pass "Pushed to remote" || die "Push failed."
fi

echo ""
echo -e "${BOLD}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo -e "${GREEN}✅ Done.${NC}"
echo ""
