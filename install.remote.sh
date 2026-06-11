#!/usr/bin/env bash
# Lint Arwaky Installer
# Cross-platform installation: Linux, macOS
# Usage: curl -sSL https://raw.githubusercontent.com/rakaarwaky/lint-arwaky/main/install.remote.sh | bash
#        or: bash install.remote.sh

set -euo pipefail

BOLD='\033[1m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m'

# Detect OS
detect_os() {
    local os_type
    os_type="$(uname -s | tr '[:upper:]' '[:lower:]')"
    case "$os_type" in
        linux*)   echo "linux" ;;
        darwin*)  echo "macos" ;;
        *)        echo "unknown" ;;
    esac
}

# Detect Arch
detect_arch() {
    local arch_type
    arch_type="$(uname -m)"
    case "$arch_type" in
        x86_64|amd64) echo "x86_64" ;;
        arm64|aarch64) echo "arm64" ;;
        *) echo "unknown" ;;
    esac
}

OS="$(detect_os)"
ARCH="$(detect_arch)"

if [ "$OS" != "linux" ]; then
    echo -e "${RED}Error: Lint Arwaky installer only supports Linux.${NC}"
    exit 1
fi

echo -e "${BOLD}"
echo " _     _             _      _                               "
echo "| |   (_)_ __   ___ | |    / \   _ __ __      ____ _ | | ___ _   _ "
echo "| |   | | '_ \ / _ \| |   / _ \ | '__|\ \ /\ / / _\` || |/ / | | | |"
echo "| |___| | | | | (_) | |___/ ___ \| |    \ V  V / (_| ||   <| |_| |"
echo "|_____|_|_| |_|\___/|____/_/   \_\_|     \_/\_/ \__,_||_|\_\\\\__, |"
echo "                                                            |___/ "
echo "  Autonomous Code Quality and Architecture Enforcement"
echo -e "${NC}"
echo -e "  Detected: ${GREEN}$OS ($ARCH)${NC}"

LOCAL_BIN="$HOME/.local/bin"
CARGO_BIN="$HOME/.cargo/bin"

# Ensure target directories exist
mkdir -p "$LOCAL_BIN"

# ── 1. Check Cargo / Rust ───────────────────────────────────────────
echo -e "\n${BOLD}[1/4] Checking installation method...${NC}"

INSTALLED=false

if command -v cargo &>/dev/null; then
    echo -e "  ${GREEN}Found Cargo. Installing via 'cargo install lint_arwaky'...${NC}"
    if cargo install lint_arwaky --force; then
        echo -e "  ${GREEN}✓ Successfully installed via Cargo!${NC}"
        INSTALLED=true
    else
        echo -e "  ${YELLOW}Cargo install failed. Trying to fall back to pre-built binaries...${NC}"
    fi
fi

# ── 2. Fallback to pre-built binaries from GitHub Releases ──────────
if [ "$INSTALLED" = false ]; then
    echo -e "\n${BOLD}[2/4] Downloading pre-built binaries from GitHub...${NC}"
    
    # Pre-built binaries are available for Linux x86_64 from GitHub Releases
    if [ "$ARCH" = "x86_64" ]; then
        CLI_URL="https://github.com/rakaarwaky/lint-arwaky/releases/latest/download/lint-arwaky-cli"
        MCP_URL="https://github.com/rakaarwaky/lint-arwaky/releases/latest/download/lint-arwaky-mcp"
        
        echo "  Downloading lint-arwaky-cli to $LOCAL_BIN..."
        if curl -sSL -o "$LOCAL_BIN/lint-arwaky-cli" "$CLI_URL"; then
            chmod +x "$LOCAL_BIN/lint-arwaky-cli"
            echo -e "  ${GREEN}✓ Downloaded lint-arwaky-cli${NC}"
            
            echo "  Downloading lint-arwaky-mcp to $LOCAL_BIN..."
            if curl -sSL -o "$LOCAL_BIN/lint-arwaky-mcp" "$MCP_URL"; then
                chmod +x "$LOCAL_BIN/lint-arwaky-mcp"
                echo -e "  ${GREEN}✓ Downloaded lint-arwaky-mcp${NC}"
                INSTALLED=true
            else
                echo -e "  ${RED}Failed to download lint-arwaky-mcp${NC}"
            fi
        else
            echo -e "  ${RED}Failed to download lint-arwaky-cli${NC}"
        fi
    else
        echo -e "  ${RED}Pre-built binaries are only provided for Linux x86_64.${NC}"
        echo -e "  Since you are on $OS ($ARCH), please install Rust and Cargo first to compile from source:"
        echo -e "  Run: ${YELLOW}curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh${NC}"
        exit 1
    fi
fi

if [ "$INSTALLED" = false ]; then
    echo -e "  ${RED}Failed to install Lint Arwaky.${NC}"
    echo "  Please install Rust/Cargo and run: cargo install lint_arwaky"
    exit 1
fi

# ── 3. Path Configuration ───────────────────────────────────────────
echo -e "\n${BOLD}[3/4] Checking PATH...${NC}"

# Find installed binary
LINT_CLI=""
for path in "$CARGO_BIN/lint-arwaky-cli" "$LOCAL_BIN/lint-arwaky-cli"; do
    if [ -x "$path" ]; then
        LINT_CLI="$path"
        break
    fi
done

if [ -z "$LINT_CLI" ] && command -v lint-arwaky-cli &>/dev/null; then
    LINT_CLI="$(command -v lint-arwaky-cli)"
fi

if [ -n "$LINT_CLI" ]; then
    echo -e "  ${GREEN}Found lint-arwaky-cli at: $LINT_CLI${NC}"
else
    echo -e "  ${YELLOW}Warning: lint-arwaky-cli not found in PATH.${NC}"
    echo -e "  Make sure either $CARGO_BIN or $LOCAL_BIN is in your PATH."
fi

# ── 4. Initialize Config ────────────────────────────────────────────
echo -e "\n${BOLD}[4/4] Initializing configuration...${NC}"
if [ -n "$LINT_CLI" ]; then
    if "$LINT_CLI" setup init; then
        echo -e "  ${GREEN}✓ Created lint_arwaky.config.yaml in the current directory${NC}"
    else
        echo -e "  ${YELLOW}Failed to run setup init automatically. Run manually: lint-arwaky-cli setup init${NC}"
    fi
else
    echo "  Could not find lint-arwaky-cli command to initialize config."
fi

# ── Done ────────────────────────────────────────────────────────────
echo -e "\n${BOLD}Done!${NC}"
echo ""
echo -e "${GREEN}Lint Arwaky is successfully installed and ready.${NC}"
echo ""
echo "Quick start:"
echo "  lint-arwaky-cli check .            # run architecture check"
echo "  lint-arwaky-cli setup doctor       # run environment diagnostics"
echo "  lint-arwaky-cli setup mcp-config   # print MCP server configuration"
echo ""
echo "As MCP server:"
echo "  lint-arwaky-mcp                    # start MCP server (stdio)"
echo ""
echo "For MCP clients (Claude, Hermes, VS Code):"
echo "  lint-arwaky-cli setup mcp-config --client claude"
echo "  lint-arwaky-cli setup mcp-config --client vscode"
echo "  lint-arwaky-cli setup mcp-config --client hermes"
echo ""
