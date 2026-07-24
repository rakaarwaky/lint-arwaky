#!/usr/bin/env bash
# install.dev.sh — Developer Environment Installer & Setup
# Sets up all developer tools, adapters, linkers, git hooks, and builds the project.
set -euo pipefail

source "$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)/lib.sh"

export CARGO_BUILD_JOBS="${CARGO_BUILD_JOBS:-4}"
export RUST_MIN_STACK="${RUST_MIN_STACK:-268435456}"
export CARGO_INCREMENTAL=0

usage() {
    echo "Usage: bash scripts/install.dev.sh [options]"
    echo ""
    echo "Options:"
    echo "  -h, --help    Show this help"
    echo ""
    echo "Sets up: Rust toolchain, cargo-nextest, cargo-audit, cargo-watch,"
    echo "  mold linker, eslint, prettier, typescript, mypy, ruff, bandit,"
    echo "  pytest, git hooks, and builds the project."
    exit 0
}

while [[ $# -gt 0 ]]; do
    case "$1" in
        -h|--help) usage ;;
        *) die "Unknown option: $1 (use -h for help)" ;;
    esac
done

echo -e "${BOLD}"
echo "  _     _       _         _                   _                 "
echo " | |   (_)_ __ | |_      / \   _ __ __      ____ _| |_   _ "
echo " | |   | | '_ \| __|    / _ \ | '__|\ \ /\ / / _\` | / / | | | |"
echo " | |___| | | | | |_    / ___ \| |    \ V  V / (_| |   < | |_| |"
echo " |_____|_|_| |_|\__|  /_/   \_\_|     \_/\_/ \__,_|_|\_\ \__, |"
echo "                                                         |___/ "
echo "  Developer Environment Setup & Onboarding"
echo -e "${NC}"

# 1. System & Package Manager Detection
echo -e "${BOLD}[1/6] Detecting platform and package manager...${NC}"
detect_pkg_mgr
echo "  Platform: $(uname -s), Package Manager: $PKG_MGR"

# 2. Rust Toolchain & Dev Extensions
echo -e "\n${BOLD}[2/6] Setting up Rust toolchain & developer tools...${NC}"

if ! command -v cargo &>/dev/null; then
    echo "  [install] Rust toolchain..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source "$HOME/.cargo/env" || true
else
    echo "  [skip] Rust/Cargo already installed ($(cargo --version))"
fi

rustup component add clippy rustfmt 2>/dev/null || true

# Install cargo-nextest for fast parallel testing
if ! command -v cargo-nextest &>/dev/null; then
    echo "  [install] cargo-nextest runner..."
    curl -LsSf https://get.nexte.st/latest/linux | tar zxf - -C "$HOME/.cargo/bin" 2>/dev/null || cargo install cargo-nextest --locked || true
else
    echo "  [skip] cargo-nextest already installed"
fi

# Install cargo-audit for security vulnerability scanning
if ! command -v cargo-audit &>/dev/null; then
    echo "  [install] cargo-audit..."
    cargo install cargo-audit --locked || true
else
    echo "  [skip] cargo-audit already installed"
fi

# Install cargo-watch for auto-rebuild on file changes
if ! command -v cargo-watch &>/dev/null; then
    echo "  [install] cargo-watch..."
    cargo install cargo-watch --locked || true
else
    echo "  [skip] cargo-watch already installed"
fi

# Install mold linker if available for faster linking
if ! command -v mold &>/dev/null; then
    echo "  [optional] Attempting to install mold linker..."
    case "$PKG_MGR" in
        apt)    sudo apt-get install -y mold 2>/dev/null || true ;;
        dnf)    sudo dnf install -y mold 2>/dev/null || true ;;
        pacman) sudo pacman -S --noconfirm mold 2>/dev/null || true ;;
        brew)   brew install mold 2>/dev/null || true ;;
        *)      echo "  [skip] mold linker not installed" ;;
    esac
else
    echo "  [skip] mold linker already installed"
fi

# 3. External Linters & Adapters
echo -e "\n${BOLD}[3/6] Installing external linter adapters...${NC}"

install_if_missing npm "npm" "npm_install"
install_if_missing eslint "eslint" "npm_install_global eslint"
install_if_missing tsc "typescript" "npm_install_global typescript"
install_if_missing prettier "prettier" "npm_install_global prettier"
install_if_missing mypy "mypy" "pip_install mypy"
install_if_missing ruff "ruff" "pip_install ruff"
install_if_missing bandit "bandit" "pip_install bandit"
install_if_missing pytest "pytest" "pip_install pytest pytest-cov"

# 4. Local Installation & Build
echo -e "\n${BOLD}[4/6] Building project & installing local XDG config...${NC}"
bash "$PROJECT_ROOT/scripts/install.local.sh"

# 5. Git Hooks Setup
echo -e "\n${BOLD}[5/6] Setting up Git hooks...${NC}"
GIT_HOOKS_DIR="$PROJECT_ROOT/.git/hooks"
if [ -d "$GIT_HOOKS_DIR" ]; then
    cat << 'EOF' > "$GIT_HOOKS_DIR/pre-commit"
#!/usr/bin/env bash
# Git pre-commit hook — runs quality gates before commit
set -e
PROJECT_ROOT="$(git rev-parse --show-toplevel)"
bash "$PROJECT_ROOT/scripts/gates.sh"
EOF
    chmod +x "$GIT_HOOKS_DIR/pre-commit"
    echo -e "  ${GREEN}✓ Installed .git/hooks/pre-commit -> scripts/gates.sh${NC}"
else
    echo "  [skip] Not a git workspace or .git/hooks missing"
fi

# 6. Verification
echo -e "\n${BOLD}[6/6] Running quality gates verification...${NC}"
bash "$PROJECT_ROOT/scripts/gates.sh"

echo -e "\n${BOLD}${GREEN}======================================================${NC}"
echo -e "${BOLD}${GREEN} Developer environment setup complete! Happy coding! 🚀${NC}"
echo -e "${BOLD}${GREEN}======================================================${NC}"
