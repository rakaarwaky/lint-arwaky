#!/usr/bin/env bash
# install.remote.sh — remote installation from crates.io / git with full dependency checks & XDG layout
# Usage: curl -sSL https://raw.githubusercontent.com/rakaarwaky/lint-arwaky/main/scripts/install.remote.sh | bash
#        or: bash scripts/install.remote.sh
set -euo pipefail

BOLD='\033[1m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m'

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" 2>/dev/null && pwd || echo "")"
PROJECT_ROOT=""
if [ -n "$SCRIPT_DIR" ] && [ -f "$SCRIPT_DIR/../Cargo.toml" ]; then
    PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
fi

INSTALL_BIN="${LINT_ARWAKY_INSTALL_BIN:-$HOME/.cargo/bin}"
CONFIG_DIR="${LINT_ARWAKY_CONFIG_DIR:-$HOME/.config/lint-arwaky}"
REPORT_DIR="${LINT_ARWAKY_REPORT_DIR:-$HOME/.local/share/lint-arwaky/reports}"

BINARIES=(lint-arwaky-cli lint-arwaky-mcp lint-arwaky-tui)

echo -e "${BOLD}"
echo "     _             _      _                               "
echo "    | |   (_)_ __   ___ | |    / \   _ __ __      ____ _  | | ___ _   _ "
echo "    | |   | | '_ \ / _ \| |   / _ \ | '__|\ \ /\ / / _\` || |/ / | | | |"
echo "    | |___| | | | | (_) | |___/ ___ \| |    \ V  V / (_| ||   <| |_| |"
echo "    |_____|_|_| |_|\___/|____/_/   \_\_|     \_/\_/ \__,_||_|\_\\\\__, |"
echo "                                                                |___/ "
echo "  Autonomous Code Quality and Architecture Enforcement"
echo -e "${NC}"

# 1. Cleanup & Setup XDG layout
echo -e "${BOLD}[1/6] Preparing XDG directories...${NC}"
if [ -d "$CONFIG_DIR" ]; then
    echo "  Cleaning existing config dir: $CONFIG_DIR"
    rm -rf "$CONFIG_DIR"
fi
if [ -d "$REPORT_DIR" ]; then
    echo "  Cleaning existing report dir: $REPORT_DIR"
    rm -rf "$REPORT_DIR"
fi

mkdir -p "$CONFIG_DIR/rules" "$REPORT_DIR" "$INSTALL_BIN"

# 2. Check & install external dependencies
echo -e "\n${BOLD}[2/6] Checking external dependencies...${NC}"

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
detect_pkg_mgr

npm_install() {
    case "$PKG_MGR" in
        apt)    curl -fsSL https://deb.nodesource.com/setup_lts.x | sudo -E bash - && sudo apt-get install -y nodejs ;;
        dnf)    curl -fsSL https://rpm.nodesource.com/setup_lts.x | sudo bash - && sudo dnf install -y nodejs ;;
        brew)   brew install node ;;
        pacman) sudo pacman -S --noconfirm nodejs npm ;;
        *)      echo "  [warn] Unknown package manager. Install node/npm manually." ;;
    esac
}

pip_install() {
    local pkg="$1"
    if command -v pip3 &>/dev/null; then
        pip3 install --user "$pkg"
    elif command -v pip &>/dev/null; then
        pip install --user "$pkg"
    else
        echo "  [warn] pip not found. Install $pkg manually."
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

install_if_missing cargo "Rust/Cargo" "curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y && . \"\$HOME/.cargo/env\""
install_if_missing npm "npm" "npm_install"
install_if_missing eslint "eslint" "sudo npm install -g eslint || npm install -g eslint"
install_if_missing tsc "typescript" "sudo npm install -g typescript || npm install -g typescript"
install_if_missing mypy "mypy" "pip_install mypy"
install_if_missing ruff "ruff" "pip_install ruff"
install_if_missing bandit "bandit" "pip_install bandit"

# 3. Install binaries from crates.io / git
echo -e "\n${BOLD}[3/6] Installing binaries...${NC}"
if cargo install lint_arwaky-arwaky --force 2>/dev/null; then
    echo -e "  ${GREEN}✓ Installed lint_arwaky-arwaky from crates.io${NC}"
elif [ -n "$PROJECT_ROOT" ] && [ -f "$PROJECT_ROOT/Cargo.toml" ]; then
    echo "  Building from local repository..."
    RUST_MIN_STACK=33554432 cargo build --release --manifest-path "$PROJECT_ROOT/Cargo.toml"
    for BIN in "${BINARIES[@]}"; do
        install -m 0755 "$PROJECT_ROOT/target/release/$BIN" "$INSTALL_BIN/$BIN"
        echo "  -> $INSTALL_BIN/$BIN"
    done
else
    echo "  Installing from git repository..."
    cargo install --git https://github.com/rakaarwaky/lint-arwaky.git --force
fi

# 4. Install documentation & SKILL.md
echo -e "\n${BOLD}[4/6] Installing docs & configuration...${NC}"
Docs=(
    "SKILL.md"
    "ARCHITECTURE.md"
    "MIGRATION_RUST.md"
    "MIGRATION_PYTHON.md"
    "MIGRATION_TYPESCRIPT.md"
)

RAW_BASE="https://raw.githubusercontent.com/rakaarwaky/lint-arwaky/main"

for DOC in "${Docs[@]}"; do
    if [ -n "$PROJECT_ROOT" ] && [ -f "$PROJECT_ROOT/$DOC" ]; then
        cp "$PROJECT_ROOT/$DOC" "$CONFIG_DIR/$DOC"
        echo "  $DOC -> $CONFIG_DIR/$DOC"
    else
        curl -fsSL "$RAW_BASE/$DOC" -o "$CONFIG_DIR/$DOC" 2>/dev/null && echo "  $DOC -> $CONFIG_DIR/$DOC" || true
    fi
done

# RULES_AES.md
if [ -n "$PROJECT_ROOT" ] && [ -f "$PROJECT_ROOT/.agents/rules/RULES_AES.md" ]; then
    cp "$PROJECT_ROOT/.agents/rules/RULES_AES.md" "$CONFIG_DIR/RULES_AES.md"
    echo "  RULES_AES.md -> $CONFIG_DIR/RULES_AES.md"
else
    curl -fsSL "$RAW_BASE/.agents/rules/RULES_AES.md" -o "$CONFIG_DIR/RULES_AES.md" 2>/dev/null && echo "  RULES_AES.md -> $CONFIG_DIR/RULES_AES.md" || true
fi

# 5. Copy .agents/ skills, rules, and prompts
echo -e "\n${BOLD}[5/6] Syncing agent skills, rules & prompts...${NC}"
AGENTS_DST="$CONFIG_DIR/.agents"
mkdir -p "$AGENTS_DST/skills" "$AGENTS_DST/rules" "$AGENTS_DST/prompts"

if [ -n "$PROJECT_ROOT" ] && [ -d "$PROJECT_ROOT/.agents" ]; then
    cp -r "$PROJECT_ROOT/.agents/"* "$AGENTS_DST/"
    echo "  Synced .agents/ directory from local source"
fi

# 6. Verification & Summary
echo -e "\n${BOLD}[6/6] Verifying installation...${NC}"

if command -v lint-arwaky-cli &>/dev/null; then
    VERSION=$(lint-arwaky-cli version 2>/dev/null | grep -oP '"version":\s*"\K[^"]+' || echo "installed")
    echo -e "  ${GREEN}✓ lint-arwaky-cli (${VERSION})${NC}"
else
    echo -e "  ${YELLOW}⚠ lint-arwaky-cli not in PATH. Ensure $INSTALL_BIN is in your PATH.${NC}"
fi

if command -v lint-arwaky-mcp &>/dev/null; then
    echo -e "  ${GREEN}✓ lint-arwaky-mcp found${NC}"
fi
if command -v lint-arwaky-tui &>/dev/null; then
    echo -e "  ${GREEN}✓ lint-arwaky-tui found${NC}"
fi

echo -e "\n${BOLD}Done (Remote)!${NC}"
echo -e "${GREEN}Lint Arwaky configuration: $CONFIG_DIR, reports: $REPORT_DIR${NC}"
echo ""
echo "Quick start:"
echo "  lint-arwaky-cli check .            # run architecture check"
echo "  lint-arwaky-cli doctor             # run environment diagnostics"
echo "  lint-arwaky-cli mcp-config         # print MCP server config"
echo "  lint-arwaky-tui                    # launch interactive TUI"
