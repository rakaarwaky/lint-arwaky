#!/bin/bash
# Auto-Linter Installer
# Cross-platform installation: Linux (Debian/Ubuntu), Fedora, macOS, Windows (PowerShell)
# Usage: curl -sSL https://raw.githubusercontent.com/rakaarwaky/auto-linter/main/install.sh | bash
#        or: bash install.sh

set -e

BOLD='\033[1m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m'

# Detect OS
detect_os() {
    if [ -n "$OSTYPE" ]; then
        case "$OSTYPE" in
            linux-gnu*)
                if [ -f /etc/os-release ]; then
                    . /etc/os-release
                    case "$ID" in
                        debian|ubuntu|linuxmint)
                            echo "debian"
                            ;;
                        fedora|rhel|centos)
                            echo "fedora"
                            ;;
                        *)
                            echo "linux"
                            ;;
                    esac
                else
                    echo "linux"
                fi
                ;;
            darwin*)
                echo "macos"
                ;;
            *)
                echo "unknown"
                ;;
        esac
    else
        echo "unknown"
    fi
}

OS=$(detect_os)

echo -e "${BOLD}"
echo "  _         _        _     _                        "
echo " /_\  _   _| |_ ___ | |   (_)_ __  _   ___  __     "
echo "//_\| | | | __/ _ \| |   | | '_ \| | | \ \/ /    "
echo "/  _ \ |_| | || (_) | |___| | | | | |_| |>  <     "
echo "\_/ \_/\__,_|\__\___/|_____|_|_| |_|\__,_/_/\_\    "
echo ""
echo "  Lint Architecture for Python & JavaScript/TypeScript"
echo -e "${NC}"
echo -e "  Detected: ${GREEN}$OS${NC}"

# ── Check Python ────────────────────────────────────────────────────
echo -e "${BOLD}[1/5] Checking Python...${NC}"

PYTHON=""
for cmd in python3.13 python3.12 python3; do
    if command -v "$cmd" &>/dev/null; then
        ver=$($cmd -c "import sys; print(f'{sys.version_info.major}.{sys.version_info.minor}')")
        major=$(echo "$ver" | cut -d. -f1)
        minor=$(echo "$ver" | cut -d. -f2)
        if [ "$major" -ge 3 ] && [ "$minor" -ge 12 ]; then
            PYTHON="$cmd"
            echo -e "  ${GREEN}Found: $cmd ($ver)${NC}"
            break
        fi
    fi
done

if [ -z "$PYTHON" ]; then
    echo -e "  ${RED}Python >= 3.12 not found!${NC}"
    echo "  Install Python 3.12+ first:"
    case "$OS" in
        debian)
            echo "    Debian/Ubuntu: sudo apt update && sudo apt install python3.12 python3.12-venv python3-pip"
            echo "    Or use pyenv: curl https://pyenv.run | bash"
            ;;
        fedora)
            echo "    Fedora: sudo dnf install python3.12 python3-pip"
            ;;
        macos)
            echo "    macOS: brew install python3.12"
            echo "    Or use pyenv: brew install pyenv && pyenv install 3.12.0"
            ;;
        *)
            echo "    Install Python 3.12+ from https://python.org/downloads"
            ;;
    esac
    exit 1
fi

# ── Choose install method ───────────────────────────────────────────
echo -e "\n${BOLD}[2/5] Install method:${NC}"

INSTALL_METHOD=""
if command -v uv &>/dev/null; then
    INSTALL_METHOD="uv"
    echo -e "  ${GREEN}Using uv (recommended)${NC}"
elif command -v pip3 &>/dev/null; then
    INSTALL_METHOD="pip"
    echo -e "  Using pip3"
elif command -v pip &>/dev/null; then
    INSTALL_METHOD="pip"
    echo -e "  Using pip"
else
    echo -e "  ${RED}No pip or uv found!${NC}"
    exit 1
fi

# ── Install ─────────────────────────────────────────────────────────
echo -e "\n${BOLD}[3/5] Installing auto-linter...${NC}"

if [ "$INSTALL_METHOD" = "uv" ]; then
    uv tool install auto-linter || uv pip install auto-linter
else
    $PYTHON -m pip install --user auto-linter
fi

# Verify installation
if command -v auto-lint &>/dev/null; then
    echo -e "  ${GREEN}Installed: $(which auto-lint)${NC}"
elif command -v auto-linter &>/dev/null; then
    echo -e "  ${GREEN}Installed: $(which auto-linter)${NC}"
else
    # Try to find LOCAL_BIN dynamically
    if [ "$OS" = "macos" ]; then
        LOCAL_BIN=$($PYTHON -m site --user-base 2>/dev/null)/bin
    else
        LOCAL_BIN="$HOME/.local/bin"
    fi
    
    if [ -f "$LOCAL_BIN/auto-lint" ]; then
        echo -e "  ${YELLOW}auto-lint is at $LOCAL_BIN/auto-lint${NC}"
        echo -e "  ${YELLOW}Add to PATH: export PATH=\"$LOCAL_BIN:\$PATH\"${NC}"
        export PATH="$LOCAL_BIN:$PATH"
    fi
fi

# ── Init config ─────────────────────────────────────────────────────
echo -e "\n${BOLD}[4/5] Initializing configuration...${NC}"

# Find auto-lint command
AUTO_LINT=""
for cmd in auto-lint "$HOME/.local/bin/auto-lint"; do
    if command -v "$cmd" &>/dev/null || [ -x "$cmd" ]; then
        AUTO_LINT="$cmd"
        break
    fi
done

if [ -n "$AUTO_LINT" ]; then
    $AUTO_LINT setup init
else
    echo -e "  ${YELLOW}Could not find auto-lint command for init${NC}"
    echo "  Run manually: auto-lint setup init"
fi

# ── Done ────────────────────────────────────────────────────────────
echo -e "\n${BOLD}[5/5] Done!${NC}"
echo ""
echo -e "${GREEN}Auto-Linter is ready.${NC}"
echo ""
echo "Quick start:"
echo "  auto-lint check ./src/           # lint your code"
echo "  auto-lint setup doctor           # diagnose issues"
echo "  auto-lint setup mcp-config       # get MCP server config"
echo ""
echo "As MCP server:"
echo "  auto-linter                      # start MCP server (stdio)"
echo ""
echo "For MCP clients (Claude, Hermes, VS Code):"
echo "  auto-lint setup mcp-config --client claude"
echo "  auto-lint setup mcp-config --client hermes"
