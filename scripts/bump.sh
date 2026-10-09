#!/usr/bin/env bash
# bump.sh — semantic version bumper for the lint-arwaky workspace
#
# One command moves the whole multi-crate release in lockstep:
#
#   - Cargo.toml      [workspace.package] version AND the root [package]
#                     version (member crates inherit the workspace value
#                     via `version.workspace = true`, so no per-crate edit)
#   - VERSION         root file read by crates/shared/src/common/build.rs
#                     to stamp RELEASE_VERSION into every binary
#   - Cargo.lock      every path-dep workspace entry moves to the new
#                     version; the rest of the locked dependency graph is
#                     untouched (CI already tested it)
#   - CHANGELOG.md    new "## <version> (<date>)" entry grouped from the
#                     conventional commits since the last v* tag
#   - README.md       version badge, heading, and install tag
#
# Usage:
#   bash scripts/bump.sh patch          # 3.8.2 → 3.8.3
#   bash scripts/bump.sh minor          # 3.8.2 → 3.9.0
#   bash scripts/bump.sh major          # 3.8.2 → 4.0.0
#   bash scripts/bump.sh 3.10.0         # explicit version
#   bash scripts/bump.sh --dry-run patch # show what would happen
#
# Options:
#   --dry-run      Show changes without applying
#   --no-commit    Skip git commit after bump
#   --no-changelog Skip the CHANGELOG.md entry (version files only)
#   -y, --yes      Auto-confirm (no prompts)
#   -h, --help     Show this help
set -euo pipefail
shopt -s nullglob

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
cd "$PROJECT_ROOT"

CARGO_TOML="Cargo.toml"
VERSION_FILE="VERSION"
CARGO_LOCK="Cargo.lock"
CHANGELOG="CHANGELOG.md"
README="README.md"

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
NO_CHANGELOG=false
AUTO=false
BUMP_TYPE=""
NEW_VERSION=""

# ── Parse arguments ─────────────────────────────────────────────────────────────
usage() {
  sed -n '3,27p' "$0" | sed 's/^#//; s/^ //'
  echo ""
  echo "Usage: bash scripts/bump.sh <patch|minor|major|X.Y.Z> [--dry-run] [--no-commit] [--no-changelog] [-y]"
  exit 0
}

while [[ $# -gt 0 ]]; do
  case "$1" in
    -h|--help) usage ;;
    --dry-run) DRY_RUN=true; shift ;;
    --no-commit) NO_COMMIT=true; shift ;;
    --no-changelog) NO_CHANGELOG=true; shift ;;
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
  grep -m1 '^version = ' "$CARGO_TOML" | sed -E 's/version = "([^"]+)".*/\1/'
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

# Newest v* tag reachable from HEAD, or empty.
latest_tag() {
  git tag --merged HEAD --sort=-v:refname 2>/dev/null | grep '^v' | head -1
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
echo ""

if $DRY_RUN; then
  info "[DRY-RUN] Would update: $CARGO_TOML, $VERSION_FILE, $CARGO_LOCK (all path-dep entries)"
  [[ -f "$README" ]] && info "[DRY-RUN] Would update: $README (version refs, if any)"
  if ! $NO_CHANGELOG; then
    info "[DRY-RUN] Would insert: $CHANGELOG (new ## $CALCULATED_VERSION entry)"
  fi
  exit 0
fi

# ── Confirm ─────────────────────────────────────────────────────────────────────
if [ "$AUTO" = false ]; then
  read -rp "Bump version to $CALCULATED_VERSION? [Y/n] " ans
  [[ "$ans" =~ ^[nN] ]] && die "Aborted by user"
fi

# ── Apply ───────────────────────────────────────────────────────────────────────
# 1. Root Cargo.toml — both `version = ` lines ([workspace.package] and
#    [package]). Member crates carry `version.workspace = true`, so the
#    workspace value above is the single source of truth they inherit.
sed -i -E "s/^version = \"[^\"]+\"/version = \"${CALCULATED_VERSION}\"/" "$CARGO_TOML"
pass "Updated $CARGO_TOML (workspace.package + package): $CALCULATED_VERSION"

# 2. VERSION file — crates/shared/src/common/build.rs reads it to stamp
#    RELEASE_VERSION into every binary; it must stay in step with
#    Cargo.toml or the shipped binary announces the previous release.
if [[ -f "$VERSION_FILE" ]]; then
  echo "$CALCULATED_VERSION" > "$VERSION_FILE"
  pass "Updated $VERSION_FILE: $CALCULATED_VERSION"
else
  warn "$VERSION_FILE missing — skipped (build script expects it)"
fi

# 3. Cargo.lock — every workspace member publishes under a distinct
#    package name, so each carries its own `version` entry. Bump all of
#    them to the new release version; registry-sourced dependencies keep
#    their pinned versions untouched.
if [[ -f "$CARGO_LOCK" ]]; then
  python3 - "$CARGO_LOCK" "$CALCULATED_VERSION" <<'PYEOF'
import re, sys

lockfile, new_version = sys.argv[1], sys.argv[2]
with open(lockfile) as f:
    lines = f.readlines()

out = []
in_ours = False
touched = 0
for line in lines:
    stripped = line.strip()
    name_m = re.match(r'name = "([^"]+)"', stripped)
    if name_m:
        in_ours = ("lint-arwaky" in name_m.group(1)) or ("lint_arwaky" in name_m.group(1))
        out.append(line)
        continue
    if stripped.startswith("[[package]]"):
        in_ours = False
        out.append(line)
        continue
    if in_ours and stripped.startswith("version = "):
        out.append('version = "' + new_version + '"\n')
        touched += 1
        continue
    out.append(line)

if touched < 2:
    sys.exit("only %d version entries rewritten in %s — expected ~40" % (touched, lockfile))

with open(lockfile, "w") as f:
    f.writelines(out)
print("rewrote %d workspace entries in %s" % (touched, lockfile))
PYEOF
  pass "Updated $CARGO_LOCK: all path-dep entries → $CALCULATED_VERSION"
else
  die "$CARGO_LOCK missing — cannot guarantee a consistent lock"
fi

# 4. README version references (badge, heading, install tag) — no-op when
#    the file carries none.
if [[ -f "$README" ]]; then
  sed -i -E "s/version-[0-9]+\.[0-9]+\.[0-9]+-blue/version-${CALCULATED_VERSION}-blue/" "$README"
  sed -i -E "s/^# Lint Arwaky v[0-9]+\.[0-9]+\.[0-9]+$/# Lint Arwaky v${CALCULATED_VERSION}/" "$README"
  sed -i -E "s/--tag v[0-9]+\.[0-9]+\.[0-9]+/--tag v${CALCULATED_VERSION}/" "$README"
  pass "Checked $README for version references"
fi

# 5. CHANGELOG.md — insert an entry for the bumped version, grouped from
#    the conventional commits since the previous release, with the same
#    shape auto-release.yml writes.
if ! $NO_CHANGELOG; then
  if [[ ! -f "$CHANGELOG" ]]; then
    die "$CHANGELOG missing — cannot insert release entry"
  fi

  LAST_TAG="$(latest_tag)"
  if [[ -n "$LAST_TAG" ]]; then
    RANGE="${LAST_TAG}..HEAD"
  else
    RANGE="HEAD"
    warn "No v* tag found — changelog covers all history"
  fi

  # Dedupe: a squash-merge leaves the subject once as a PR-merge commit and
  # once as the original commit; auto-release.yml has the same quirk.
  DATE="$(date +%Y-%m-%d)"
  FEATS="$(git log "$RANGE" --pretty=format:'- %s' --grep='^feat(\([^)]+\))?:' -E 2>/dev/null | head -20 | awk '!seen[$0]++' || true)"
  FIXES="$(git log "$RANGE" --pretty=format:'- %s' --grep='^fix(\([^)]+\))?:' -E 2>/dev/null | head -20 | awk '!seen[$0]++' || true)"
  CHORES="$(git log "$RANGE" --pretty=format:'- %s' --grep='^(chore|refactor|docs|test|perf)(\([^)]+\))?:' -E 2>/dev/null | head -20 | awk '!seen[$0]++' || true)"

  python3 - "$CHANGELOG" "$CALCULATED_VERSION" "$DATE" "$FEATS" "$FIXES" "$CHORES" <<'PYEOF'
import sys
changelog, version, date = sys.argv[1], sys.argv[2], sys.argv[3]
feats, fixes, chores = sys.argv[4], sys.argv[5], sys.argv[6]
sections = []
if feats.strip():
    sections.append("### Features\n\n" + feats.strip())
if fixes.strip():
    sections.append("### Bug Fixes\n\n" + fixes.strip())
if chores.strip():
    sections.append("### Maintenance\n\n" + chores.strip())
if not sections:
    sections.append("- No significant changes")
body = "\n\n".join(sections)
entry = "## %s (%s)\n\n%s\n\n" % (version, date, body)
with open(changelog) as f:
    content = f.read()
first_nl = content.index('\n') + 1
with open(changelog, 'w') as f:
    f.write(content[:first_nl] + entry + content[first_nl:])
PYEOF
  pass "Updated $CHANGELOG (entry: ## $CALCULATED_VERSION ($DATE))"
fi

# 6. Sanity — the workspace still resolves with the new version. If the
#    lock drifted from the manifests, this fails instead of shipping a
#    release that lies about its own version.
if command -v cargo &>/dev/null; then
  cargo metadata --no-deps --format-version 1 >/dev/null 2>&1 \
    || die "cargo metadata failed after bump — manifest/lock drift, release would lie"
  pass "cargo metadata: workspace resolves with $CALCULATED_VERSION"
else
  die "cargo not found — cannot verify the bump"
fi

pass "Version bumped: $CURRENT_VERSION → $CALCULATED_VERSION"

# ── Commit ──────────────────────────────────────────────────────────────────────
if [ "$NO_COMMIT" = false ]; then
  echo ""
  info "Committing version bump..."

  if command -v git &>/dev/null; then
    for f in "$CARGO_TOML" "$VERSION_FILE" "$CARGO_LOCK" "$CHANGELOG" "$README"; do
      [[ -f "$f" ]] && git add -- "$f"
    done
    if git diff --cached --quiet; then
      info "No version files changed — nothing to commit"
    else
      git commit -m "chore: bump version to $CALCULATED_VERSION" \
        && pass "Committed via git" \
        || die "git commit failed"
    fi
  else
    warn "No git found — skipping commit"
  fi
fi

# ── Done ────────────────────────────────────────────────────────────────────────
echo ""
echo -e "${BOLD}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo -e "${GREEN}✅ Done: $CALCULATED_VERSION${NC}"
echo -e "${BOLD}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
