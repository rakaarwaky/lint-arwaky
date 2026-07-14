#!/usr/bin/env bash
# Lint Arwaky Remote Installer
# Clones from GitHub and builds from source.
# Usage: curl -sSL https://raw.githubusercontent.com/rakaarwaky/lint-arwaky/main/scripts/install.remote.sh | bash
#        or: bash scripts/install.remote.sh
set -euo pipefail

BOLD='\033[1m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m'

REPO="https://github.com/rakaarwaky/lint-arwaky.git"
BRANCH="${LINT_ARWAKY_BRANCH:-main}"
TMPDIR_BUILD="$(mktemp -d)"

cleanup() { rm -rf "$TMPDIR_BUILD"; }
trap cleanup EXIT

echo -e "${BOLD}"
echo "     _             _      _                               "
echo "    | |   (_)_ __   ___ | |    / \   _ __ __      ____ _  | | ___ _   _ "
echo "    | |   | | '_ \ / _ \| |   / _ \ | '__|\ \ /\ / / _\` || |/ / | | | |"
echo "    | |___| | | | | (_) | |___/ ___ \| |    \ V  V / (_| ||   <| |_| |"
echo "    |_____|_|_| |_|\___/|____/_/   \_\_|     \_/\_/ \__,_||_|\_\\\\__, |"
echo "                                                                |___/ "
echo "  Autonomous Code Quality and Architecture Enforcement"
echo -e "${NC}"

# Check prerequisites
echo -e "${BOLD}[1/5] Checking prerequisites...${NC}"

if ! command -v git &>/dev/null; then
    echo -e "  ${RED}git not found. Please install git.${NC}"
    exit 1
fi
echo -e "  ${GREEN}✓ git found${NC}"

if ! command -v cargo &>/dev/null; then
    echo -e "  ${RED}cargo not found. Please install Rust: curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh${NC}"
    exit 1
fi
echo -e "  ${GREEN}✓ cargo found${NC}"

# Check platform
OS="$(uname -s | tr '[:upper:]' '[:lower:]')"
if [ "$OS" != "linux" ]; then
    echo -e "${RED}Error: Only Linux is supported.${NC}"
    exit 1
fi
ARCH="$(uname -m)"
echo -e "  Platform: ${GREEN}linux ($ARCH)${NC}"

# Install directories
if [ "$(id -u)" -eq 0 ]; then
    INSTALL_BIN="${LINT_ARWAKY_INSTALL_BIN:-/usr/local/bin}"
    CONFIG_DIR="${LINT_ARWAKY_CONFIG_DIR:-/etc/lint-arwaky}"
else
    INSTALL_BIN="${LINT_ARWAKY_INSTALL_BIN:-$HOME/.cargo/bin}"
    CONFIG_DIR="${LINT_ARWAKY_CONFIG_DIR:-$HOME/.config/lint-arwaky}"
fi

# Clone
echo -e "\n${BOLD}[2/5] Cloning repository (branch: $BRANCH)...${NC}"
if git clone --depth 1 --branch "$BRANCH" "$REPO" "$TMPDIR_BUILD/lint-arwaky"; then
    echo -e "  ${GREEN}✓ Cloned successfully${NC}"
else
    echo -e "  ${RED}Failed to clone repository${NC}"
    exit 1
fi

PROJECT_ROOT="$TMPDIR_BUILD/lint-arwaky"

# Build
echo -e "\n${BOLD}[3/5] Building release binaries...${NC}"
cd "$PROJECT_ROOT"
if RUST_MIN_STACK=33554432 cargo build --release 2>&1 | tail -3; then
    echo -e "  ${GREEN}✓ Build successful${NC}"
else
    echo -e "  ${RED}Build failed${NC}"
    exit 1
fi

# Install
echo -e "\n${BOLD}[4/5] Installing binaries to $INSTALL_BIN...${NC}"
mkdir -p "$INSTALL_BIN" "$CONFIG_DIR"

BINARIES=(lint-arwaky-cli lint-arwaky-mcp lint-arwaky-tui)
RELEASE_DIR="$PROJECT_ROOT/target/release"

for BIN in "${BINARIES[@]}"; do
    if [ -f "$RELEASE_DIR/$BIN" ]; then
        install -m 0755 "$RELEASE_DIR/$BIN" "$INSTALL_BIN/$BIN"
        echo -e "  ${GREEN}✓ $BIN -> $INSTALL_BIN/$BIN${NC}"
    else
        echo -e "  ${YELLOW}⚠ $BIN not found, skipping${NC}"
    fi
done

# Install docs
DOCS=(SKILL.md ARCHITECTURE.md)
for DOC in "${DOCS[@]}"; do
    if [ -f "$PROJECT_ROOT/$DOC" ]; then
        cp "$PROJECT_ROOT/$DOC" "$CONFIG_DIR/$DOC"
    fi
done

# AES rules
RULES_SRC="$PROJECT_ROOT/.agents/rules/RULES_AES.md"
if [ -f "$RULES_SRC" ]; then
    cp "$RULES_SRC" "$CONFIG_DIR/RULES_AES.md"
fi

# Init config
echo -e "\n${BOLD}[5/5] Initializing configuration...${NC}"
if command -v lint-arwaky-cli &>/dev/null; then
    if lint-arwaky-cli init 2>/dev/null; then
        echo -e "  ${GREEN}✓ Config initialized${NC}"
    else
        echo -e "  ${YELLOW}Config may already exist, skipping${NC}"
    fi
fi

# Done
echo -e "\n${BOLD}Done!${NC}"
echo ""
echo -e "${GREEN}Lint Arwaky installed successfully.${NC}"
echo ""
echo "Binaries: $INSTALL_BIN"
echo "Config:   $CONFIG_DIR"
echo ""
echo "Quick start:"
echo "  lint-arwaky-cli check .            # run architecture check"
echo "  lint-arwaky-cli doctor             # run environment diagnostics"
echo "  lint-arwaky-cli mcp-config         # print MCP server config"
echo ""
echo "As MCP server:"
echo "  lint-arwaky-mcp                    # start MCP server (stdio)"
