#!/usr/bin/env bash
# lib.sh — Shared functions for all lint-arwaky scripts
# Source this file: source "$(dirname "${BASH_SOURCE[0]}")/lib.sh"
set -euo pipefail

# ── Colors ──────────────────────────────────────────────────────────────────────
if [ -t 1 ]; then
    GREEN='\033[0;32m'
    RED='\033[0;31m'
    YELLOW='\033[1;33m'
    CYAN='\033[0;36m'
    BOLD='\033[1m'
    NC='\033[0m'
else
    GREEN='' RED='' YELLOW='' CYAN='' BOLD='' NC=''
fi

pass() { echo -e " ${GREEN}✔${NC} $1"; }
info() { echo -e " ${CYAN}→${NC} $1"; }
warn() { echo -e " ${YELLOW}⚠${NC} $1"; }
fail() { echo -e " ${RED}✘${NC} $1"; }
die()  { fail "$1"; exit 1; }

# ── Project Root ────────────────────────────────────────────────────────────────
SCRIPTS_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPTS_DIR/.." && pwd)"

# ── Package Manager Detection ──────────────────────────────────────────────────
detect_pkg_mgr() {
    if command -v apt-get &>/dev/null; then
        PKG_MGR="apt"
    elif command -v dnf &>/dev/null; then
        PKG_MGR="dnf"
    elif command -v brew &>/dev/null; then
        PKG_MGR="brew"
    elif command -v pacman &>/dev/null; then
        PKG_MGR="pacman"
    else
        PKG_MGR="unknown"
    fi
}

# ── Install Helpers ────────────────────────────────────────────────────────────
npm_install() {
    case "${PKG_MGR:-unknown}" in
        apt)    curl -fsSL https://deb.nodesource.com/setup_lts.x | sudo -E bash - && sudo apt-get install -y nodejs ;;
        dnf)    curl -fsSL https://rpm.nodesource.com/setup_lts.x | sudo bash - && sudo dnf install -y nodejs ;;
        brew)   brew install node ;;
        pacman) sudo pacman -S --noconfirm nodejs npm ;;
        *)      warn "Unknown package manager. Install node/npm manually." ;;
    esac
}

npm_install_global() {
    if [ "$(id -u)" -eq 0 ]; then
        npm install -g "$1"
    else
        sudo npm install -g "$1" 2>/dev/null || npm install -g "$1"
    fi
}

pip_install() {
    local pkg="$1"
    if command -v pip3 &>/dev/null; then
        pip3 install --user "$pkg"
    elif command -v pip &>/dev/null; then
        pip install --user "$pkg"
    else
        warn "pip not found. Install $pkg manually."
    fi
}

install_if_missing() {
    local cmd="$1"
    local pkg="$2"
    local method="$3"
    if command -v "$cmd" &>/dev/null; then
        echo "  [skip] $cmd already installed"
    else
        echo "  [install] $pkg..."
        eval "$method"
    fi
}

# ── Doc/Agents Copy Helpers ────────────────────────────────────────────────────
copy_docs_to_config() {
    local config_dir="$1"
    local project_root="${2:-$PROJECT_ROOT}"
    local Docs=(
        "ARCHITECTURE.md"
        "MIGRATION_RUST.md"
        "MIGRATION_PYTHON.md"
        "MIGRATION_TYPESCRIPT.md"
    )
    for DOC in "${Docs[@]}"; do
        SRC="$project_root/$DOC"
        if [ -f "$SRC" ]; then
            cp "$SRC" "$config_dir/$DOC"
            echo "  $DOC -> $config_dir/$DOC"
        fi
    done

    RULES_SRC="$project_root/.agents/rules/RULES_AES.md"
    if [ -f "$RULES_SRC" ]; then
        cp "$RULES_SRC" "$config_dir/RULES_AES.md"
        echo "  RULES_AES.md -> $config_dir/RULES_AES.md"
    fi
}

copy_agents_to_config() {
    local config_dir="$1"
    local project_root="${2:-$PROJECT_ROOT}"
    local agents_src="$project_root/.agents"
    local agents_dst="$config_dir/.agents"

    if [ -d "$agents_src" ]; then
        mkdir -p "$agents_dst/skills" "$agents_dst/rules" "$agents_dst/prompts"

        for SKILL_DIR in "$agents_src"/skills/*; do
            if [ -d "$SKILL_DIR" ]; then
                SKILL_NAME=$(basename "$SKILL_DIR")
                cp -r "$SKILL_DIR" "$agents_dst/skills/$SKILL_NAME"
                echo "  .agents/skills/$SKILL_NAME -> $config_dir/.agents/skills/$SKILL_NAME"
            fi
        done

        for RULE_FILE in "$agents_src"/rules/*; do
            if [ -f "$RULE_FILE" ]; then
                RULE_NAME=$(basename "$RULE_FILE")
                cp "$RULE_FILE" "$agents_dst/rules/$RULE_NAME"
                echo "  .agents/rules/$RULE_NAME -> $config_dir/.agents/rules/$RULE_NAME"
            fi
        done

        for PROMPT_FILE in "$agents_src"/prompts/*; do
            if [ -f "$PROMPT_FILE" ]; then
                PROMPT_NAME=$(basename "$PROMPT_FILE")
                cp "$PROMPT_FILE" "$agents_dst/prompts/$PROMPT_NAME"
                echo "  .agents/prompts/$PROMPT_NAME -> $config_dir/.agents/prompts/$PROMPT_NAME"
            fi
        done
    fi
}

# ── Version Helpers ────────────────────────────────────────────────────────────
get_project_version() {
    local cargo_toml="${PROJECT_ROOT}/Cargo.toml"
    local ver
    ver=$(cargo metadata --no-deps --format-version 1 2>/dev/null | sed -n 's/.*"version":"\([^"]*\)".*/\1/p' | head -1 || true)
    if [ -z "$ver" ]; then
        ver=$(sed -nE 's/^[[:space:]]*version[[:space:]]*=[[:space:]]*"([^"]+)".*/\1/p' "$cargo_toml" | head -1)
    fi
    echo "$ver"
}

# ── Gate Runner ────────────────────────────────────────────────────────────────
ci_step() {
    local name="$1"
    shift
    echo ""
    echo "  === $name ==="
    if eval "$@"; then
        pass "$name"
        return 0
    else
        fail "$name"
        return 1
    fi
}
