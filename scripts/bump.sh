#!/usr/bin/env bash
# bump.sh — semantic version bumper for Cargo.toml
#
# Usage:
#   bash scripts/bump.sh patch          # 1.10.14 → 1.10.15
#   bash scripts/bump.sh minor          # 1.10.14 → 1.11.0
#   bash scripts/bump.sh major          # 1.10.14 → 2.0.0
#   bash scripts/bump.sh 1.20.0         # explicit version
#   bash scripts/bump.sh --dry-run patch # show what would happen
#
# Options:
#   --dry-run     Show changes without applying
#   --no-commit   Skip git/jj commit after bump
#   -y, --yes     Auto-confirm (no prompts)
#   -h, --help    Show this help
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
CARGO_TOML="$PROJECT_ROOT/Cargo.toml"

# ── Colors ──────────────────────────────────────────────────────────────────────
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
CYAN='\033[0;36m'
BOLD='\033[1m'
NC='\033[0m'

pass() { echo -e " ${GREEN}✔${NC} $1"; }
info() { echo -e " ${CYAN}→${NC} $1"; }
warn() { echo -e " ${YELLOW}⚠${NC} $1"; }
fail() { echo -e " ${RED}✘${NC} $1"; }
die()  { fail "$1"; exit 1; }

# ── Defaults ────────────────────────────────────────────────────────────────────
DRY_RUN=false
NO_COMMIT=false
AUTO=false
BUMP_TYPE=""
NEW_VERSION=""

# ── Parse arguments ─────────────────────────────────────────────────────────────
usage() {
  cat << 'EOF'
bump.sh — semantic version bumper for Cargo.toml

Usage:
  bash scripts/bump.sh patch          # 1.10.14 → 1.10.15
  bash scripts/bump.sh minor          # 1.10.14 → 1.11.0
  bash scripts/bump.sh major          # 1.10.14 → 2.0.0
  bash scripts/bump.sh 1.20.0         # explicit version
  bash scripts/bump.sh --dry-run patch # show what would happen

Options:
  --dry-run     Show changes without applying
  --no-commit   Skip git/jj commit after bump
  -y, --yes     Auto-confirm (no prompts)
  -h, --help    Show this help
EOF
  exit 0
}

while [[ $# -gt 0 ]]; do
  case "$1" in
    -h|--help) usage ;;
    --dry-run) DRY_RUN=true; shift ;;
    --no-commit) NO_COMMIT=true; shift ;;
    -y|--yes) AUTO=true; shift ;;
    patch|minor|major) BUMP_TYPE="$1"; shift ;;
    [0-9]*.[0-9]*.[0-9]*) NEW_VERSION="$1"; shift ;;
    *) die "Unknown option: $1 (use -h for help)" ;;
  esac
done

if [[ -z "$BUMP_TYPE" ]] && [[ -z "$NEW_VERSION" ]]; then
  die "Usage: bump.sh <patch|minor|major|X.Y.Z>"
fi

# ── Helpers ─────────────────────────────────────────────────────────────────────
current_version() {
  grep '^version = ' "$CARGO_TOML" | head -1 | sed -E 's/version = "([^"]+)"/\1/'
}

bump_version() {
  local current="$1"
  local bump_type="$2"
  current="${current//[^0-9.]/}"
  IFS='.' read -r major minor patch <<< "$current"
  major="${major//[^0-9]/}"; minor="${minor//[^0-9]/}"; patch="${patch//[^0-9]/}"
  case "$bump_type" in
    patch) patch=$((patch + 1)) ;;
    minor) minor=$((minor + 1)); patch=0 ;;
    major) major=$((major + 1)); minor=0; patch=0 ;;
    *) die "Invalid bump type: $bump_type (expected patch|minor|major)" ;;
  esac
  echo "${major}.${minor}.${patch}"
}

update_cargo_version() {
  local new_version="$1"
  sed -i -E "s/^version = \"[^\"]+\"/version = \"${new_version}\"/" "$CARGO_TOML"
}

# ── Resolve version ─────────────────────────────────────────────────────────────
if [[ ! -f "$CARGO_TOML" ]]; then
  die "$CARGO_TOML not found. Run from project root."
fi

CURRENT_VERSION="$(current_version)"
if [[ ! "$CURRENT_VERSION" =~ ^[0-9]+\.[0-9]+\.[0-9]+ ]]; then
  die "Cannot parse current version: $CURRENT_VERSION"
fi

if [[ -n "$BUMP_TYPE" ]]; then
  CALCULATED_VERSION="$(bump_version "$CURRENT_VERSION" "$BUMP_TYPE")"
else
  CALCULATED_VERSION="$NEW_VERSION"
fi

# ── Display ─────────────────────────────────────────────────────────────────────
echo ""
echo -e "${BOLD}━━━ Version Bump ━━━${NC}"
echo ""
echo "  Current:  ${CYAN}$CURRENT_VERSION${NC}"
echo "  New:      ${GREEN}$CALCULATED_VERSION${NC}"
echo "  File:     $CARGO_TOML"
echo ""

if $DRY_RUN; then
  info "[DRY-RUN] Would update $CARGO_TOML: $CURRENT_VERSION → $CALCULATED_VERSION"
  exit 0
fi

# ── Confirm ─────────────────────────────────────────────────────────────────────
if [ "$AUTO" = false ]; then
  read -rp "Bump version to $CALCULATED_VERSION? [Y/n] " ans
  [[ "$ans" =~ ^[nN] ]] && die "Aborted by user"
fi

# ── Apply ───────────────────────────────────────────────────────────────────────
update_cargo_version "$CALCULATED_VERSION"
pass "Version bumped: $CURRENT_VERSION → $CALCULATED_VERSION"

# ── Commit ──────────────────────────────────────────────────────────────────────
if [ "$NO_COMMIT" = false ]; then
  echo ""
  info "Committing version bump..."

  if command -v jj &>/dev/null; then
    jj describe -m "chore: bump version to $CALCULATED_VERSION" 2>/dev/null && \
      pass "Committed via jj" || warn "jj commit failed"
  elif command -v git &>/dev/null; then
    git add "$CARGO_TOML" 2>/dev/null
    git commit -m "chore: bump version to $CALCULATED_VERSION" 2>/dev/null && \
      pass "Committed via git" || warn "git commit failed (no changes?)"
  else
    warn "No jj or git found — skipping commit"
  fi
fi

# ── Done ────────────────────────────────────────────────────────────────────────
echo ""
echo -e "${BOLD}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo -e "${GREEN}✅ Done: $CALCULATED_VERSION${NC}"
echo -e "${BOLD}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
