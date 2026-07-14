#!/usr/bin/env bash
# Lint Arwaky Remote Installer
# Install from crates.io (pre-built, no local build needed).
# Usage: curl -sSL https://raw.githubusercontent.com/rakaarwaky/lint-arwaky/main/scripts/install.remote.sh | bash
#        or: bash scripts/install.remote.sh
set -euo pipefail

BOLD='\033[1m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m'

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
echo -e "${BOLD}[1/3] Checking prerequisites...${NC}"

if ! command -v cargo &>/dev/null; then
    echo -e "  ${RED}cargo not found.${NC}"
    echo -e "  Install Rust first: curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
    exit 1
fi
echo -e "  ${GREEN}✓ cargo found${NC}"

# Detect platform
OS="$(uname -s | tr '[:upper:]' '[:lower:]')"
if [ "$OS" != "linux" ]; then
    echo -e "${RED}Error: Only Linux is supported.${NC}"
    exit 1
fi
echo -e "  Platform: ${GREEN}linux ($(uname -m))${NC}"

# Install
echo -e "\n${BOLD}[2/3] Installing lint_arwaky from crates.io...${NC}"
echo "  This downloads pre-built binaries, no local compilation needed."

if cargo install lint_arwaky --force; then
    echo -e "  ${GREEN}✓ Installed successfully${NC}"
else
    echo -e "  ${RED}Failed to install from crates.io${NC}"
    echo "  Try: cargo install lint_arwaky --force"
    exit 1
fi

# Verify
echo -e "\n${BOLD}[3/3] Verifying installation...${NC}"

if command -v lint-arwaky-cli &>/dev/null; then
    VERSION=$(lint-arwaky-cli version 2>/dev/null | grep -oP '"version":\s*"\K[^"]+' || echo "unknown")
    echo -e "  ${GREEN}✓ lint-arwaky-cli v${VERSION}${NC}"
else
    echo -e "  ${YELLOW}⚠ lint-arwaky-cli not in PATH. Restart your shell or add ~/.cargo/bin to PATH.${NC}"
fi

if command -v lint-arwaky-mcp &>/dev/null; then
    echo -e "  ${GREEN}✓ lint-arwaky-mcp found${NC}"
fi

# Done
echo -e "\n${BOLD}Done!${NC}"
echo ""
echo -e "${GREEN}Lint Arwaky installed from crates.io.${NC}"
echo ""
echo "Quick start:"
echo "  lint-arwaky-cli check .            # run architecture check"
echo "  lint-arwaky-cli doctor             # run environment diagnostics"
echo "  lint-arwaky-cli mcp-config         # print MCP server config"
echo ""
echo "As MCP server:"
echo "  lint-arwaky-mcp                    # start MCP server (stdio)"
