#!/usr/bin/env bash
# install.vscode.sh — VS Code Extension Builder & Installer
# Builds the TypeScript packages, packages as .vsix, and installs into VS Code.
set -euo pipefail

source "$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)/lib.sh"

usage() {
    echo "Usage: bash scripts/install.vscode.sh [options]"
    echo ""
    echo "Options:"
    echo "  -h, --help     Show this help"
    echo "  -b, --build    Build only, skip install"
    echo "  --uninstall    Uninstall the extension"
    echo "  --open         Open VS Code with this workspace after install"
    echo ""
    echo "Requires: node/npm, @vscode/vsce (auto-installed if missing)"
    exit 0
}

BUILD_ONLY=false
DO_UNINSTALL=false
DO_OPEN=false

while [[ $# -gt 0 ]]; do
    case "$1" in
        -h|--help)    usage ;;
        -b|--build)   BUILD_ONLY=true; shift ;;
        --uninstall)  DO_UNINSTALL=true; shift ;;
        --open)       DO_OPEN=true; shift ;;
        *)            die "Unknown option: $1 (use -h for help)" ;;
    esac
done

echo -e "${BOLD}"
echo "  ╔═══════════════════════════════════════════╗"
echo "  ║  Lint Arwaky — VS Code Extension Installer ║"
echo "  ╚═══════════════════════════════════════════╝"
echo -e "${NC}"

# ── Pre-flight checks ──────────────────────────────────────────────────────────
echo -e "${BOLD}[1/5] Checking prerequisites...${NC}"

if ! command -v node &>/dev/null; then
    die "node is required. Run: bash scripts/install.dev.sh"
fi
info "node $(node --version)"

if ! command -v npm &>/dev/null; then
    die "npm is required. Run: bash scripts/install.dev.sh"
fi
info "npm $(npm --version)"

# Ensure vsce is available
if ! command -v vsce &>/dev/null && ! npx vsce --version &>/dev/null 2>&1; then
    info "Installing @vscode/vsce globally..."
    npm_install_global @vscode/vsce 2>/dev/null || npx @vscode/vsce --version >/dev/null 2>&1
fi

# ── Uninstall (if requested) ───────────────────────────────────────────────────
if [ "$DO_UNINSTALL" = true ]; then
    echo -e "\n${BOLD}Uninstalling lint-arwaky VS Code extension...${NC}"
    if code --uninstall-extension lint-arwaky.lint-arwaky 2>/dev/null; then
        pass "Extension uninstalled"
    else
        warn "Extension not installed or code CLI not available"
    fi
    exit 0
fi

# ── Install npm workspace dependencies ─────────────────────────────────────────
echo -e "\n${BOLD}[2/5] Installing npm workspace dependencies...${NC}"
cd "$PROJECT_ROOT"

# Remove stale node_modules from packages/
rm -rf "$PROJECT_ROOT/packages/node_modules"

npm install 2>&1 | tail -3
pass "Dependencies installed"

# ── TypeScript build ──────────────────────────────────────────────────────────
echo -e "\n${BOLD}[3/5] Building TypeScript packages...${NC}"

# Clean previous build
rm -rf "$PROJECT_ROOT/packages/dist"

npx tsc -p "$PROJECT_ROOT/tsconfig.json" 2>&1
if [ $? -eq 0 ]; then
    pass "TypeScript build succeeded"
else
    die "TypeScript build failed"
fi

# ── Package as .vsix ──────────────────────────────────────────────────────────
echo -e "\n${BOLD}[4/5] Packaging VSIX extension...${NC}"

VSIX_PATH="$PROJECT_ROOT/dist/lint-arwaky-$(get_project_version).vsix"
mkdir -p "$PROJECT_ROOT/dist"

# Use npx vsce to package (run from project root, provide base URL for README links)
cd "$PROJECT_ROOT"
if npx @vscode/vsce package --out "$VSIX_PATH" --no-dependencies \
    --baseContentUrl "https://github.com/lint-arwaky/lint-arwaky" \
    --allow-missing-repository 2>&1; then
    pass "VSIX created: $VSIX_PATH"
else
    die "VSIX packaging failed"
fi

# ── Build only mode ───────────────────────────────────────────────────────────
if [ "$BUILD_ONLY" = true ]; then
    echo -e "\n${BOLD}${GREEN}════════════════════════════════════════${NC}"
    echo -e "${BOLD}${GREEN} Build complete (skipping install)      ${NC}"
    echo -e "${BOLD}${GREEN} VSIX: $VSIX_PATH${NC}"
    echo -e "${BOLD}${GREEN}════════════════════════════════════════${NC}"
    exit 0
fi

# ── Install into VS Code ──────────────────────────────────────────────────────
echo -e "\n${BOLD}[5/5] Installing extension into VS Code...${NC}"

if command -v code &>/dev/null; then
    if code --install-extension "$VSIX_PATH" 2>&1; then
        pass "Extension installed successfully"
    else
        die "Extension install failed"
    fi
else
    die "VS Code CLI ('code') not found. Install VS Code or run with --build flag"
fi

# ── Open VS Code (if requested) ──────────────────────────────────────────────
if [ "$DO_OPEN" = true ]; then
    code "$PROJECT_ROOT" 2>/dev/null &
    pass "Opened VS Code"
fi

# ── Done ──────────────────────────────────────────────────────────────────────
echo -e "\n${BOLD}${GREEN}======================================================${NC}"
echo -e "${BOLD}${GREEN} VS Code extension installed! 🚀${NC}"
echo -e "${BOLD}${GREEN}   Open VS Code → Cmd/Ctrl+Shift+P → 'Lint Arwaky'${NC}"
echo -e "${BOLD}${GREEN}======================================================${NC}"
