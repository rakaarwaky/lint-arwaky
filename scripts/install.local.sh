#!/usr/bin/env bash
# install.local.sh — bump patch + release build + XDG-aware install + checksums
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
CARGO_TOML="$PROJECT_ROOT/Cargo.toml"
RELEASE_DIR="$PROJECT_ROOT/target/release"
DIST_DIR="$PROJECT_ROOT/dist"

if [ "$(id -u)" -eq 0 ]; then
    INSTALL_BIN="${LINT_ARWAKY_INSTALL_BIN:-/usr/local/bin}"
    CONFIG_DIR="${LINT_ARWAKY_CONFIG_DIR:-/etc/lint-arwaky}"
    REPORT_DIR="${LINT_ARWAKY_REPORT_DIR:-/var/lib/lint-arwaky/reports}"
else
    INSTALL_BIN="${LINT_ARWAKY_INSTALL_BIN:-$HOME/.cargo/bin}"
    CONFIG_DIR="${LINT_ARWAKY_CONFIG_DIR:-$HOME/.config/lint-arwaky}"
    REPORT_DIR="${LINT_ARWAKY_REPORT_DIR:-$HOME/.local/share/lint-arwaky/reports}"
fi

BINARIES=(lint-arwaky-cli lint-arwaky-mcp lint-arwaky-tui)

# 1. Instal XDG layout sebelum build
mkdir -p "$CONFIG_DIR/rules" "$REPORT_DIR" "$DIST_DIR" "$INSTALL_BIN"

# 2. Bump patch version
OLD_VERSION=$(grep '^version' "$CARGO_TOML" | head -1 | sed 's/version = "\(.*\)"/\1/' | tr -d '\r')
IFS='.' read -r MAJOR MINOR PATCH <<< "$OLD_VERSION"
PATCH=$(( PATCH + 1 ))
NEW_VERSION="$MAJOR.$MINOR.$PATCH"
sed -i "0,/^version = \"$OLD_VERSION\"/s/^version = \"$OLD_VERSION\"/version = \"$NEW_VERSION\"/" "$CARGO_TOML"
echo "  $OLD_VERSION  ->  $NEW_VERSION"

# 3. Build (increase stack size to prevent LLVM SIGSEGV during LTO)
RUST_MIN_STACK=33554432 cargo build --release

# 4. Checksums + install
pushd "$RELEASE_DIR" >/dev/null
sha256sum "${BINARIES[@]}" > "$DIST_DIR/SHA256SUMS.txt"
popd >/dev/null

for BIN in "${BINARIES[@]}"; do
    install -m 0755 "$RELEASE_DIR/$BIN" "$INSTALL_BIN/$BIN"
    echo "  -> $INSTALL_BIN/$BIN"
done

# 5. Install docs + SKILL.md to XDG config
Docs=(
    "SKILL.md"
    "ARCHITECTURE.md"
    "MIGRATION_RUST.md"
    "MIGRATION_PYTHON.md"
    "MIGRATION_TYPESCRIPT.md"
)
for DOC in "${Docs[@]}"; do
    SRC="$PROJECT 2_ROOT/$DOC"
    if [ -f "$SRC" ]; then
        cp "$SRC" "$CONFIG_DIR/$DOC"
        echo "  $DOC -> $CONFIG_DIR/$DOC"
    fi
done

# RULES_AES.md: source in .agents/rules/, target at XDG root
RULES_SRC="$PROJECT_ROOT/.agents/rules/RULES_AES.md"
if [ -f "$RULES_SRC" ]; then
    cp "$RULES_SRC" "$CONFIG_DIR/RULES_AES.md"
    echo "  RULES_AES.md -> $CONFIG_DIR/RULES_AES.md"
fi

# 6. Copy .agents/skills/ and .agents/rules/ to target's .agents/ folder
AGENTS_SRC="$PROJECT_ROOT/.agents"
AGENTS_DST="$CONFIG_DIR/.agents"
if [ -d "$AGENTS_SRC" ]; then
    mkdir -p "$AGENTS_DST/skills" "$AGENTS_DST/rules"

    # Copy skills (each subdirectory = one skill)
    for SKILL_DIR in "$AGENTS_SRC"/skills/*; do
        if [ -d "$SKILL_DIR" ]; then
            SKILL_NAME=$(basename "$SKILL_DIR")
            cp -r "$SKILL_DIR" "$AGENTS_DST/skills/$SKILL_NAME"
            echo "  .agents/skills/$SKILL_NAME -> $CONFIG_DIR/.agents/skills/$SKILL_NAME"
        fi
    done

    # Copy rules
    for RULE_FILE in "$AGENTS_SRC"/rules/*; do
        if [ -f "$RULE_FILE" ]; then
            RULE_NAME=$(basename "$RULE_FILE")
            cp "$RULE_FILE" "$AGENTS_DST/rules/$RULE_NAME"
            echo "  .agents/rules/$RULE_NAME -> $CONFIG_DIR/.agents/rules/$RULE_NAME"
        fi
    done
fi

echo "Done: $NEW_VERSION, config=$CONFIG_DIR, reports=$REPORT_DIR"
