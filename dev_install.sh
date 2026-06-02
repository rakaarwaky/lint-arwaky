#!/bin/bash
# Auto-Linter Developer Installer (Editable Source Mode)
# This script installs the auto-linter package directly from the current source directory.
# Any changes you make to the .py files will be reflected immediately without reinstalling.

set -e

BOLD='\033[1m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
BLUE='\033[0;34m'
NC='\033[0m'

echo -e "${BOLD}${BLUE}=== Auto-Linter Developer Installation ===${NC}"
echo -e "Installing from: ${YELLOW}$(pwd)${NC}\n"

# 1. Detect environment
if command -v uv &>/dev/null; then
    echo -e "${BOLD}[1/3] Using uv for installation...${NC}"
    echo -e "  - Syncing project dependencies..."
    uv sync
    
    echo -e "  - Installing CLI tools in editable mode..."
    uv tool install --editable . --force
    
    INSTALL_CMD="uv tool"
else
    echo -e "${BOLD}[1/3] Using pip for installation...${NC}"
    if [ -f "pyproject.toml" ]; then
        echo -e "  - Installing in editable mode..."
        # Try to detect if we're in a venv
        if [[ "$VIRTUAL_ENV" != "" ]]; then
            pip install -e .
        else
            echo -e "  ${YELLOW}Warning: Not in a virtual environment. Installing to user path.${NC}"
            pip install --user -e . --break-system-packages 2>/dev/null || pip install --user -e .
        fi
        INSTALL_CMD="pip"
    else
        echo -e "${RED}Error: pyproject.toml not found in current directory.${NC}"
        exit 1
    fi
fi

# 2. Verify installation
echo -e "\n${BOLD}[2/3] Verifying installation...${NC}"
if command -v auto-lint &>/dev/null; then
    VERSION=$(auto-lint version 2>/dev/null || echo "unknown")
    LOCATION=$(which auto-lint)
    echo -e "  ${GREEN}Success!${NC} auto-lint is available at: ${BLUE}$LOCATION${NC}"
    echo -e "  Current Version: ${BOLD}$VERSION${NC}"
else
    echo -e "  ${RED}Error: auto-lint command not found in PATH.${NC}"
    echo -e "  Please ensure your local bin directory is in your PATH."
    exit 1
fi

# 3. Initialize/Refresh Config
echo -e "\n${BOLD}[3/3] Refreshing configuration...${NC}"
auto-lint setup init

echo -e "\n${BOLD}${GREEN}=== Development Environment Ready ===${NC}"
echo -e "Mode: ${YELLOW}Editable (Source-based)${NC}"
echo -e "You can now edit the code and run 'auto-lint' to test changes immediately."
echo ""
echo -e "Quick Commands:"
echo -e "  ${BOLD}auto-lint check .${NC}         - Run full analysis"
echo -e "  ${BOLD}auto-lint setup doctor${NC}   - Check environment health"
