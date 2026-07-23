#!/usr/bin/env bash
# install.global.sh — release build + global system-wide installation
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
CARGO_TOML="$PROJECT_ROOT/Cargo.toml"
RELEASE_DIR="$PROJECT_ROOT/target/release"
DIST_DIR="$PROJECT_ROOT/dist"

if [ "$(id -u)" -ne 0 ]; then
    echo "Warning: Global installation typically requires root privileges (e.g. sudo)."
fi

INSTALL_BIN="${LINT_ARWAKY_INSTALL_BIN:-/usr/local/bin}"
CONFIG_DIR="${LINT_ARWAKY_CONFIG_DIR:-/etc/lint-arwaky}"
REPORT_DIR="${LINT_ARWAKY_REPORT_DIR:-/var/lib/lint-arwaky/reports}"

BINARIES=(lint-arwaky-cli lint-arwaky-mcp lint-arwaky-tui)

# 1. Pembersihan & Install XDG layout sebelum build
if [ -d "$CONFIG_DIR" ]; then
    echo "Cleaning existing global config dir: $CONFIG_DIR"
    rm -rf "$CONFIG_DIR"
fi
if [ -d "$REPORT_DIR" ]; then
    echo "Cleaning existing global report dir: $REPORT_DIR"
    rm -rf "$REPORT_DIR"
fi

mkdir -p "$CONFIG_DIR/rules" "$REPORT_DIR" "$DIST_DIR" "$INSTALL_BIN"

# 2. Build (increase stack size to prevent LLVM SIGSEGV during LTO)
RUST_MIN_STACK=33554432 cargo build --release

# 3. Checksums + install
pushd "$RELEASE_DIR" >/dev/null
sha256sum "${BINARIES[@]}" > "$DIST_DIR/SHA256SUMS.txt"
popd >/dev/null

for BIN in "${BINARIES[@]}"; do
    install -m 0755 "$RELEASE_DIR/$BIN" "$INSTALL_BIN/$BIN"
    echo "  -> $INSTALL_BIN/$BIN"
done

# 4. Install docs + SKILL.md to XDG config
Docs=(
    "SKILL.md"
    "ARCHITECTURE.md"
    "MIGRATION_RUST.md"
    "MIGRATION_PYTHON.md"
    "MIGRATION_TYPESCRIPT.md"
)
for DOC in "${Docs[@]}"; do
    SRC="$PROJECT_ROOT/$DOC"
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

# 5. Copy .agents/skills/, .agents/rules/, and .agents/prompts/ to target's .agents/ folder
AGENTS_SRC="$PROJECT_ROOT/.agents"
AGENTS_DST="$CONFIG_DIR/.agents"
if [ -d "$AGENTS_SRC" ]; then
    mkdir -p "$AGENTS_DST/skills" "$AGENTS_DST/rules" "$AGENTS_DST/prompts"

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

    # Copy prompts
    for PROMPT_FILE in "$AGENTS_SRC"/prompts/*; do
        if [ -f "$PROMPT_FILE" ]; then
            PROMPT_NAME=$(basename "$PROMPT_FILE")
            cp "$PROMPT_FILE" "$AGENTS_DST/prompts/$PROMPT_NAME"
            echo "  .agents/prompts/$PROMPT_NAME -> $CONFIG_DIR/.agents/prompts/$PROMPT_NAME"
        fi
    done
fi

CURRENT_VERSION=$(grep '^version' "$CARGO_TOML" | head -1 | sed 's/version = "\(.*\)"/\1/' | tr -d '\r')
echo "Done (Global): $CURRENT_VERSION, config=$CONFIG_DIR, reports=$REPORT_DIR"
