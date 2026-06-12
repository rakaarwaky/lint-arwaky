#!/usr/bin/env bash
set -euo pipefail

# ============================================================================
# release.sh — Bump version, build, test, tag, push (GitHub Release), publish to crates.io
# ============================================================================
# Usage:
#   ./release.sh              # interactive: prompts for version bump type
#   ./release.sh patch        # auto bump patch (1.10.9 -> 1.10.10)
#   ./release.sh minor        # auto bump minor (1.10.9 -> 1.11.0)
#   ./release.sh major        # auto bump major (1.10.9 -> 1.12.0)
#   ./release.sh 1.11.0       # explicit version
# ============================================================================

CARGO_TOML="Cargo.toml"
REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$REPO_ROOT"

# --- helpers ----------------------------------------------------------------
current_version() {
  grep '^version = ' "$CARGO_TOML" | head -1 | sed -E 's/version = "([^"]+)"/\1/'
}

bump_version() {
  local current="$1"
  local bump_type="$2"
  # Strip any non-digit/dot chars (defense against hidden chars)
  current="${current//[^0-9.]/}"
  IFS='.' read -r major minor patch <<< "$current"
  # Ensure numeric
  major="${major//[^0-9]/}"; minor="${minor//[^0-9]/}"; patch="${patch//[^0-9]/}"
  case "$bump_type" in
    patch) patch=$((patch + 1)) ;;
    minor) minor=$((minor + 1)); patch=0 ;;
    major) major=$((major + 1)); minor=0; patch=0 ;;
    *) echo "Invalid bump type: $bump_type" >&2; exit 1 ;;
  esac
  echo "${major}.${minor}.${patch}"
}

update_cargo_version() {
  local new_version="$1"
  sed -i -E "s/^version = \"[^\"]+\"/version = \"$new_version\"/" "$CARGO_TOML"
  echo "Updated $CARGO_TOML: version = \"$new_version\""
}

check_cargo_publish_token() {
  if [[ -z "${CARGO_REGISTRY_TOKEN:-}" ]]; then
    echo "⚠️  CARGO_REGISTRY_TOKEN not set — will skip crates.io publish"
    echo "   Set it via: export CARGO_REGISTRY_TOKEN=your_token"
    return 1
  fi
  return 0
}

# --- main -------------------------------------------------------------------
CURRENT=$(current_version)
echo "Current version: $CURRENT"

# Parse flags
AUTO_CONFIRM=false
for arg in "$@"; do
  case "$arg" in
    -y|--yes) AUTO_CONFIRM=true ;;
  esac
done
# Also check env var
[[ "${RELEASE_AUTO_CONFIRM:-}" == "1" ]] && AUTO_CONFIRM=true

# Determine new version
if [[ $# -eq 0 ]] || [[ "$1" == "-y" ]] || [[ "$1" == "--yes" ]]; then
  echo "Select bump type:"
  echo "  1) patch  ($CURRENT -> $(bump_version "$CURRENT" patch))"
  echo "  2) minor  ($CURRENT -> $(bump_version "$CURRENT" minor))"
  echo "  3) major  ($CURRENT -> $(bump_version "$CURRENT" major))"
  echo "  4) custom version"
  read -rp "Choice [1-4]: " choice
  case "$choice" in
    1) NEW_VERSION=$(bump_version "$CURRENT" patch) ;;
    2) NEW_VERSION=$(bump_version "$CURRENT" minor) ;;
    3) NEW_VERSION=$(bump_version "$CURRENT" major) ;;
    4) read -rp "Enter version (e.g. 1.11.0): " NEW_VERSION ;;
    *) echo "Invalid choice"; exit 1 ;;
  esac
else
  ARG="$1"
  if [[ "$ARG" =~ ^(patch|minor|major)$ ]]; then
    NEW_VERSION=$(bump_version "$CURRENT" "$ARG")
  else
    NEW_VERSION="$ARG"
  fi
fi

# Validate semver
if ! [[ "$NEW_VERSION" =~ ^[0-9]+\.[0-9]+\.[0-9]+(-[a-zA-Z0-9.-]+)?$ ]]; then
  echo "Invalid version format: $NEW_VERSION (expected semver like 1.11.0 or 1.11.0-rc1)"
  exit 1
fi

echo "New version: $NEW_VERSION"
if [[ "$AUTO_CONFIRM" == true ]]; then
  echo "Auto-confirm enabled, proceeding..."
else
  read -rp "Proceed? [y/N]: " confirm
  [[ "$confirm" =~ ^[Yy]$ ]] || { echo "Aborted"; exit 1; }
fi

# 1. Update Cargo.toml
update_cargo_version "$NEW_VERSION"

# 2. Build release
echo "🔨 Building release..."
cargo build --release --all-targets

# 3. Run tests
echo "🧪 Running tests..."
cargo test --workspace

# 4. Check clippy
echo "🔍 Running clippy..."
cargo clippy --all-targets -- -D warnings

# 5. Check fmt
echo "🎨 Checking format..."
cargo fmt --all -- --check

# 6. Self-lint (AES)
echo "🔬 Self-lint (AES)..."
cargo run --bin lint-arwaky-cli -- check .

# 7. Commit version bump (jj)
echo "📝 Creating jj change with version bump..."
jj commit -m "chore: release v$NEW_VERSION"

# 8. Create tag
TAG="v$NEW_VERSION"
echo "🏷️  Creating tag: $TAG"
jj bookmark set "$TAG"

# 9. Push tag (triggers GitHub Actions Release workflow)
echo "🚀 Pushing tag to GitHub (triggers Release workflow)..."
jj git push --bookmark "$TAG"

echo ""
echo "✅ GitHub Release workflow triggered."
echo "   Check: https://github.com/rakaarwaky/lint-arwaky/actions"
echo "   Release will appear at: https://github.com/rakaarwaky/lint-arwaky/releases/tag/$TAG"
echo ""

# 10. Publish to crates.io (optional)
if check_cargo_publish_token; then
  if [[ "$AUTO_CONFIRM" == true ]]; then
    echo "📦 Publishing to crates.io (auto-confirm)..."
    cargo publish --token "$CARGO_REGISTRY_TOKEN"
    echo "✅ Published to crates.io"
  else
    read -rp "Publish to crates.io? [y/N]: " publish_confirm
    if [[ "$publish_confirm" =~ ^[Yy]$ ]]; then
      echo "📦 Publishing to crates.io..."
      cargo publish --token "$CARGO_REGISTRY_TOKEN"
      echo "✅ Published to crates.io"
    else
      echo "Skipped crates.io publish. Run later with: cargo publish --token \$CARGO_REGISTRY_TOKEN"
    fi
  fi
else
  echo "Skipped crates.io publish (no CARGO_REGISTRY_TOKEN)."
fi

echo ""
echo "🎉 Release v$NEW_VERSION complete!"
echo "   Tag: $TAG"
echo "   GitHub Release: https://github.com/rakaarwaky/lint-arwaky/releases/tag/$TAG"