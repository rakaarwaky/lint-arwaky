#!/usr/bin/env bash
# ==============================================================================
# release.sh — lint-arwaky unified pipeline + release automation
#
# This file replaces both the previous `pipeline.sh` and `release.sh`.
#
# Usage:
#   bash scripts/release.sh [options]
#
# Pipeline:
#   1. Build   (cargo build --bin <target>)
#   2. Lint    (AES self-check)
#   3. Verify  (cargo fmt --check + clippy)
#   4. Status  (jj st)
#   5. Commit  (jj describe with message)
#   6. Bump    (optional semver bump)
#   7. Tag     (optional GitHub tag)
#   8. Release (optional GitHub Release / crates.io publish)
#
# Options:
#   -m, --message MSG      Commit message (interactive if omitted)
#     --target TARGET       Binary to build (default: lint-arwaky-cli)
#     --no-build            Skip build step
#     --no-lint             Skip self-lint step
#     --no-fmt              Skip cargo fmt --check
#     --no-clippy           Skip cargo clippy
#     --no-commit           Skip jj commit (dry-run mode)
#     --amend               Amend the last commit instead of a new one
#     --ci-only             Run CI checks only, skip commit
#     --fast                Skip slow checks (fmt, clippy)
#     -y, --yes             Auto-confirm (no prompts)
#     --dry-run             Show what would happen without making changes
#     --bump BUMP_TYPE      Bump version: patch|minor|major|X.Y.Z
#     --no-tag              Skip create/push tag
#     --no-gh-release       Skip GitHub Release creation
#     --publish             Publish to crates.io when token is present
#     -h, --help            Show this help
# ==============================================================================
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR"

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
TARGET="lint-arwaky-cli"
CARGO_TOML="Cargo.toml"

COMMIT_MSG=""
SKIP_BUILD=false
SKIP_LINT=false
SKIP_FMT=false
SKIP_CLIPPY=false
SKIP_COMMIT=false
AMEND=false
FAST=false
AUTO=false
CI_ONLY=false
DRY_RUN=false
BUMP_MODE=""
SKIP_TAG=false
NO_GH_RELEASE=false
PUBLISH=false
NEW_VER=""

# ── Parse arguments ─────────────────────────────────────────────────────────────
usage() {
  sed -n '3,30p' "$0" | sed 's/^#//; s/^ //'
  exit 0
}

while [[ $# -gt 0 ]]; do
  case "$1" in
    -h|--help) usage ;;
    -m|--message) COMMIT_MSG="$2"; shift 2 ;;
    --target) TARGET="$2"; shift 2 ;;
    --no-build) SKIP_BUILD=true; shift ;;
    --no-lint) SKIP_LINT=true; shift ;;
    --no-fmt) SKIP_FMT=true; shift ;;
    --no-clippy) SKIP_CLIPPY=true; shift ;;
    --no-commit) SKIP_COMMIT=true; shift ;;
    --amend) AMEND=true; shift ;;
    --fast) FAST=true; shift ;;
    -y|--yes) AUTO=true; shift ;;
    --ci-only) CI_ONLY=true; shift ;;
    --dry-run) DRY_RUN=true; shift ;;
    --bump) BUMP_MODE="$2"; shift 2 ;;
    --no-tag) SKIP_TAG=true; shift ;;
    --no-gh-release) NO_GH_RELEASE=true; shift ;;
    --publish) PUBLISH=true; shift ;;
    *) die "Unknown option: $1 (use -h for help)" ;;
  esac
done

if $FAST; then
  SKIP_FMT=true
  SKIP_CLIPPY=true
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
    major) major=$((minor + 1)); minor=0; patch=0 ;;
    *) die "Invalid bump type: $bump_type (expected patch|minor|major|X.Y.Z)" ;;
  esac
  echo "${major}.${minor}.${patch}"
}

update_cargo_version() {
  local new_version="$1"
  sed -i -E "s/^version = \"[^\"]+\"/version = \"${new_version}\"/" "$CARGO_TOML"
  echo "Updated $CARGO_TOML: version = \"${new_version}\""
}

check_cargo_publish_token() {
  if [[ -z "${CARGO_REGISTRY_TOKEN:-}" ]]; then
    warn "CARGO_REGISTRY_TOKEN not set — will skip crates.io publish"
    warn "Set it via: export CARGO_REGISTRY_TOKEN=your_token"
    return 1
  fi
  return 0
}

run() {
  if $DRY_RUN; then
    echo "  [DRY-RUN] $*"
  else
    eval "$@"
  fi
}

CI_EXIT=0
ci_step() {
  local name="$1"
  shift
  echo ""
  echo "  === $name ==="
  if eval "$@"; then
    pass "$name"
  else
    fail "$name"
    CI_EXIT=1
  fi
}

# ── Resolve version bump ────────────────────────────────────────────────────────
NEW_VER=""
resolve_bump_version() {
  if [[ ! -f "$CARGO_TOML" ]]; then
    die "$CARGO_TOML not found. Run from project root."
  fi

  local current
  current="$(current_version)"
  if [[ ! "$current" =~ ^[0-9]+\.[0-9]+\.[0-9]+ ]]; then
    die "Cannot parse current version: $current"
  fi

  if [[ -n "$BUMP_MODE" ]]; then
    if [[ "$BUMP_MODE" =~ ^(patch|minor|major)$ ]]; then
      NEW_VER="$(bump_version "$current" "$BUMP_MODE")"
    elif [[ "$BUMP_MODE" =~ ^[0-9]+\.[0-9]+\.[0-9]+ ]]; then
      NEW_VER="$BUMP_MODE"
    else
      die "Invalid --bump value: $BUMP_MODE (expected patch|minor|major|X.Y.Z)"
    fi
  else
    info "Current version: $current"
    NEW_VER="$current"
  fi
}

# ── 1. Build ────────────────────────────────────────────────────────────────────
echo ""
echo -e "${BOLD}━━━ [1/8] Build — cargo build --bin $TARGET ━━━${NC}"

if $SKIP_BUILD; then
  warn "Build skipped"
else
  if $DRY_RUN; then
    info "Would run: cargo build --bin $TARGET"
  else
    ci_step "cargo build --bin $TARGET" "cargo build --bin '$TARGET' 2>&1"
    if [ $CI_EXIT -ne 0 ]; then
      die "Build failed. Fix errors and retry."
    fi
  fi
  pass "Build succeeded"
fi

# ── 2. Self-lint ────────────────────────────────────────────────────────────────
echo ""
echo -e "${BOLD}━━━ [2/8] Self-Lint — AES Rules ━━━${NC}"

if $SKIP_LINT; then
  warn "Self-lint skipped"
else
  if $DRY_RUN; then
    info "Would run: ./target/debug/$TARGET check crates/ --filter AES"
  else
    ci_step "self-lint (AES)" "timeout 60 ./target/debug/$TARGET check crates/ --filter AES 2>&1"
    if [ $CI_EXIT -ne 0 ]; then
      warn "Self-lint found issues or timed out. Review above."
      if [ "$AUTO" = false ] && [ "$CI_ONLY" = false ]; then
        read -rp "Continue? [y/N] " ans
        [[ "$ans" =~ ^[yY] ]] || die "Aborted by user"
      fi
    fi
  fi
  pass "Self-lint done"
fi

# ── 3. Format & Clippy ──────────────────────────────────────────────────────────
echo ""
echo -e "${BOLD}━━━ [3/8] Code Quality ━━━${NC}"

if $SKIP_FMT; then
  warn "cargo fmt --check skipped"
else
  ci_step "cargo fmt --check" "cargo fmt --check 2>&1"
fi

if $SKIP_CLIPPY; then
  warn "cargo clippy skipped"
else
  ci_step "cargo clippy (all targets)" \
    "cargo clippy --all-targets -- -D warnings 2>&1 | tail -20"
fi

if [ $CI_EXIT -ne 0 ]; then
  if $CI_ONLY; then
    die "CI checks failed. Fix issues and retry."
  else
    warn "CI checks failed. Continuing (review above)."
    if [ "$AUTO" = false ] && [ "$CI_ONLY" = false ]; then
      read -rp "Continue? [y/N] " ans
      [[ "$ans" =~ ^[yY] ]] || die "Aborted by user"
    fi
  fi
fi

# ── Early exit for --ci-only ────────────────────────────────────────────────────
if $CI_ONLY; then
  echo ""
  echo -e "${BOLD}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
  echo -e "${GREEN}✅ CI checks complete.${NC}"
  echo ""
  echo "  Project: lint-arwaky"
  echo "  Target:  $TARGET"
  if command -v jj &>/dev/null; then
    echo "  Commit:  $(jj log --no-graph -r @ -T 'commit_id.shortest(8)' 2>/dev/null || echo '?')"
  fi
  echo ""
  echo -e "${BOLD}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
  exit $CI_EXIT
fi

# ── 4. Status ───────────────────────────────────────────────────────────────────
echo ""
echo -e "${BOLD}━━━ [4/8] Repository Status ━━━${NC}"

if command -v jj &>/dev/null; then
  if $DRY_RUN; then
    info "[DRY-RUN] Would show: jj st"
  fi
  jj st 2>/dev/null || warn "jj st failed"
else
  warn "jj (Jujutsu) not found — status unavailable"
fi

# ── 5. Commit ───────────────────────────────────────────────────────────────────
echo ""
echo -e "${BOLD}━━━ [5/8] Commit ━━━${NC}"

if $SKIP_COMMIT; then
  warn "Commit skipped (--no-commit)"
else
  PENDING=0
  if command -v jj &>/dev/null; then
    PENDING=$(jj st 2>/dev/null | grep -c "^M\|^A\|^+\|^-" || true) || true
  fi

  if [ "${PENDING:-0}" -eq 0 ]; then
    info "No pending changes to commit."
  else
    info "Pending changes ($PENDING files):"
    jj st 2>/dev/null || true
    echo ""

    if [ -z "$COMMIT_MSG" ]; then
      echo "Enter commit message (conventional commit format):"
      echo "  fix:    Bug fix"
      echo "  feat:   New feature"
      echo "  chore:  Maintenance / docs / config"
      echo "  refactor: Code restructuring"
      echo "  perf:   Performance improvement"
      echo "  test:   Tests"
      echo "  docs:   Documentation"
      echo ""
      read -rp "Message: " COMMIT_MSG
      if [ -z "$COMMIT_MSG" ]; then
        COMMIT_MSG="chore: auto-pipeline $(date +%Y-%m-%d)"
        warn "Empty message. Using default: $COMMIT_MSG"
      fi
    fi

    echo ""
    echo "  jj describe -m \"$COMMIT_MSG\""
    [ "$AMEND" = true ] && echo "  (--amend: update last commit)"

    if [ "$AUTO" = false ]; then
      read -rp "Proceed? [Y/n] " ans
      [[ "$ans" =~ ^[nN] ]] && die "Aborted by user"
    fi

    if $DRY_RUN; then
      info "[DRY-RUN] Would run: jj describe -m \"$COMMIT_MSG\""
    else
      run "jj describe -m '$COMMIT_MSG'"
      pass "Committed: $COMMIT_MSG"
    fi
  fi
fi

# ── 6. Bump Version ─────────────────────────────────────────────────────────────
echo ""
echo -e "${BOLD}━━━ [6/8] Version ━━━${NC}"

if [[ -n "$BUMP_MODE" ]]; then
  resolve_bump_version
  echo ""
  echo "  Would update $CARGO_TOML -> $NEW_VER"
  if $DRY_RUN; then
    info "[DRY-RUN] Would bump version to $NEW_VER"
  else
    if [ "$AUTO" = false ]; then
      read -rp "Bump and update $CARGO_TOML to $NEW_VER? [y/N] " ans
      [[ "$ans" =~ ^[yY]$ ]] || die "Aborted by user"
    fi
    update_cargo_version "$NEW_VER"
    pass "Version bumped: $NEW_VER"
  fi
else
  info "Version bump skipped (use --bump)"
fi

# ── 7. Tag ──────────────────────────────────────────────────────────────────────
echo ""
echo -e "${BOLD}━━━ [7/8] Tag ━━━${NC}"

if $SKIP_TAG || [[ -z "${NEW_VER:-}" ]]; then
  warn "Tag skipped (--no-tag or no --bump)"
elif ! command -v jj &>/dev/null; then
  warn "jj (Jujutsu) not found — cannot create tag"
else
  TAG="v$NEW_VER"
  if $DRY_RUN; then
    info "[DRY-RUN] Would create tag: $TAG"
  else
    run "jj bookmark set '$TAG'"
    pass "Tag created: $TAG"
  fi
fi

# ── 8. Release ──────────────────────────────────────────────────────────────────
echo ""
echo -e "${BOLD}━━━ [8/8] Release ━━━${NC}"

if $NO_GH_RELEASE || [[ -z "${NEW_VER:-}" ]]; then
  warn "GitHub Release skipped (--no-gh-release or no --bump)"
elif ! command -v gh &>/dev/null; then
  warn "gh CLI not installed — skipping GitHub Release creation"
else
  # Best-effort release notes from recent commits
  prev_tag="$(git tag --sort=-creatordate | head -2 | tail -1 || true)"
  if [ -n "$prev_tag" ]; then
    release_notes="$(git log --oneline --no-decorate "${prev_tag}..HEAD" 2>/dev/null || true)"
  else
    release_notes="$(git log --oneline --no-decorate -20 2>/dev/null || true)"
  fi
  release_notes="## What's Changed

${release_notes:-Initial release $NEW_VER}"

  if $DRY_RUN; then
    info "[DRY-RUN] Would create GitHub Release: $NEW_VER"
  else
    run "gh release create 'v$NEW_VER' --title 'v$NEW_VER' --notes \"$release_notes\""
    pass "GitHub Release created: v$NEW_VER"
  fi
fi

# ── Optional crates.io publish ──────────────────────────────────────────────────
if $PUBLISH; then
  if ! command -v cargo &>/dev/null; then
    warn "cargo not found — skipping crates.io publish"
  else
    echo ""
    echo -e "${BOLD}━━━ Publish to crates.io ━━━${NC}"

    # Copy config files into shared crate for packaging
    SHARED_CONFIG_DIR="crates/shared/src/config-system"
    for cfg in lint_arwaky.config.*.yaml; do
      if [ -f "$cfg" ]; then
        cp "$cfg" "$SHARED_CONFIG_DIR/"
        info "Copied $cfg -> $SHARED_CONFIG_DIR/"
      fi
    done

    # Crates in dependency order (leaf → root)
    PUBLISH_CRATES=(
      "shared-lint-arwaky"
      "import-rules-lint-arwaky"
      "naming-rules-lint-arwaky"
      "code-analysis-lint-arwaky"
      "config-system-lint-arwaky"
      "external-lint-lint-arwaky"
      "orphan-detector-lint-arwaky"
      "role-rules-lint-arwaky"
      "auto-fix-lint-arwaky"
      "git-hooks-lint-arwaky"
      "project-setup-lint-arwaky"
      "maintenance-lint-arwaky"
      "cli-commands-lint-arwaky"
      "mcp-server-lint-arwaky"
      "file-watch-lint-arwaky"
      "tui-lint-arwaky"
      "lint_arwaky-arwaky"
    )

    PUBLISH_FAIL=0
    for crate in "${PUBLISH_CRATES[@]}"; do
      echo ""
      info "Publishing $crate..."
      if $DRY_RUN; then
        info "[DRY-RUN] Would run: cargo publish -p $crate"
      else
        if cargo publish -p "$crate" --allow-dirty --no-verify 2>&1; then
          pass "Published $crate"
          # crates.io needs a few seconds between publishes
          sleep 10
        else
          warn "Failed to publish $crate (may already exist or have errors)"
          PUBLISH_FAIL=1
        fi
      fi
    done

    if [ $PUBLISH_FAIL -eq 0 ]; then
      pass "All crates published to crates.io"
    else
      warn "Some crates failed to publish (check output above)"
    fi

    # Cleanup: remove copied config files from shared crate
    rm -f "$SHARED_CONFIG_DIR"/lint_arwaky.config.*.yaml
    info "Cleaned up copied config files"
  fi
fi

# ── Done ────────────────────────────────────────────────────────────────────────
echo ""
echo -e "${BOLD}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo -e "${GREEN}✅ Release script complete.${NC}"
echo ""
if command -v jj &>/dev/null; then
  echo "  Status: $(jj st 2>/dev/null | head -1 || echo '?')"
fi
echo ""
echo -e "${BOLD}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
