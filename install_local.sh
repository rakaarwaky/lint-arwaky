#!/bin/bash
# Lint Arwaky Developer Installer (Local Source Build — Rust/Cargo)
# This script builds and installs lint-arwaky directly from the current source directory.
# Re-run this script after making changes to recompile and reinstall.

set -e

BOLD='\033[1m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
BLUE='\033[0;34m'
NC='\033[0m'

echo -e "${BOLD}${BLUE}=== Lint Arwaky Developer Installation (Cargo) ===${NC}"
echo -e "Installing from: ${YELLOW}$(pwd)${NC}\n"

# ── Detect OS ─────────────────────────────────────────────────────────
detect_os() {
    if [ -n "$OSTYPE" ]; then
        case "$OSTYPE" in
            linux-gnu*)
                if [ -f /etc/os-release ]; then
                    . /etc/os-release
                    case "$ID" in
                        fedora|rhel|centos) echo "fedora" ;;
                        debian|ubuntu|linuxmint) echo "debian" ;;
                        *) echo "linux" ;;
                    esac
                else
                    echo "linux"
                fi
                ;;
            darwin*) echo "macos" ;;
            *) echo "unknown" ;;
        esac
    else
        echo "unknown"
    fi
}

OS=$(detect_os)

# ── 1. Verify Rust / Cargo ─────────────────────────────────────────────
echo -e "${BOLD}[1/4] Checking Rust toolchain...${NC}"

# Source cargo env in case it was installed but not in current shell PATH
[ -f "$HOME/.cargo/env" ] && . "$HOME/.cargo/env"
export PATH="$HOME/.cargo/bin:$PATH"

if ! command -v cargo &>/dev/null; then
    echo -e "  ${YELLOW}Rust/Cargo not found. Installing via rustup...${NC}"
    case "$OS" in
        fedora)
            sudo dnf install -y rust cargo rustup || {
                curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
                . "$HOME/.cargo/env"
            }
            ;;
        *)
            curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
            . "$HOME/.cargo/env"
            ;;
    esac
fi

if command -v cargo &>/dev/null; then
    echo -e "  ${GREEN}$(cargo --version)${NC}"
    echo -e "  ${GREEN}$(rustc --version)${NC}"
else
    echo -e "  ${RED}Error: cargo is still not available. Install Rust first.${NC}"
    exit 1
fi

# ── 2. Install build dependencies (Fedora / Debian) ───────────────────
echo -e "\n${BOLD}[2/4] Checking build dependencies...${NC}"

if [ "$OS" = "fedora" ]; then
    MISSING_DEPS=()
    rpm -q openssl-devel &>/dev/null || MISSING_DEPS+=("openssl-devel")
    rpm -q pkg-config    &>/dev/null || MISSING_DEPS+=("pkg-config")
    if [ ${#MISSING_DEPS[@]} -gt 0 ]; then
        echo -e "  ${YELLOW}Installing: ${MISSING_DEPS[*]}${NC}"
        sudo dnf install -y "${MISSING_DEPS[@]}" || {
            echo -e "  ${YELLOW}Warning: Could not install build deps. Build may fail.${NC}"
            echo "  Run manually: sudo dnf install openssl-devel pkg-config"
        }
    else
        echo -e "  ${GREEN}Build dependencies satisfied.${NC}"
    fi
elif [ "$OS" = "debian" ]; then
    if ! dpkg -l | grep -q libssl-dev 2>/dev/null; then
        echo -e "  ${YELLOW}Installing build deps...${NC}"
        sudo apt update && sudo apt install -y build-essential libssl-dev pkg-config || true
    else
        echo -e "  ${GREEN}Build dependencies satisfied.${NC}"
    fi
else
    echo -e "  ${GREEN}Skipping OS-specific build dep check.${NC}"
fi

# ── 3. Build and install from source ───────────────────────────────────
echo -e "\n${BOLD}[3/4] Building and installing from source...${NC}"
echo -e "  ${BLUE}cargo install --path . --force${NC}"

# Verify Cargo.toml exists
if [ ! -f "Cargo.toml" ]; then
    echo -e "  ${RED}Error: Cargo.toml not found in $(pwd).${NC}"
    echo "  Make sure you are running this script from the project root."
    exit 1
fi

cargo install --path . --force

# ── 4. Verify installation ─────────────────────────────────────────────
echo -e "\n${BOLD}[4/4] Verifying installation...${NC}"

if command -v lint-arwaky &>/dev/null; then
    LOCATION=$(which lint-arwaky)
    echo -e "  ${GREEN}Success!${NC} lint-arwaky is available at: ${BLUE}$LOCATION${NC}"
    lint-arwaky version 2>/dev/null || true
else
    CARGO_BIN="$HOME/.cargo/bin"
    if [ -f "$CARGO_BIN/lint-arwaky" ]; then
        echo -e "  ${YELLOW}Found at $CARGO_BIN/lint-arwaky but not in PATH.${NC}"
        echo -e "  ${YELLOW}Add to PATH: export PATH=\"$CARGO_BIN:\$PATH\"${NC}"
        export PATH="$CARGO_BIN:$PATH"
    else
        echo -e "  ${RED}Error: lint-arwaky binary not found.${NC}"
        echo "  Check build output above for errors."
        exit 1
    fi
fi

# ── Refresh config ─────────────────────────────────────────────────────
echo -e "\n${BOLD}Refreshing configuration...${NC}"
lint-arwaky setup init || {
    echo -e "  ${YELLOW}Setup init skipped. Run manually: lint-arwaky setup init${NC}"
}

echo -e "\n${BOLD}${GREEN}=== Development Environment Ready ===${NC}"
echo -e "Mode: ${YELLOW}Local source build (Cargo)${NC}"
echo -e "After editing source files, re-run this script or run:"
echo -e "  ${BOLD}cargo install --path . --force${NC}"
echo ""
echo -e "Quick Commands:"
echo -e "  ${BOLD}lint-arwaky check .${NC}          - Run full analysis"
echo -e "  ${BOLD}lint-arwaky setup doctor${NC}    - Check environment health"
echo -e "  ${BOLD}lint-arwaky-mcp${NC}             - Start MCP server (stdio)"
