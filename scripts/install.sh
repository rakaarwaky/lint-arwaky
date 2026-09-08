#!/usr/bin/env bash
# install.sh — Unified installer for lint-arwaky (local, global, remote, dev)
set -euo pipefail

source "$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)/lib.sh"

# ── XDG Paths ──────────────────────────────────────────────────────────────────
DATA_DIR="${XDG_DATA_HOME:-$HOME/.local/share}/lint-arwaky"
CONFIG_DIR="${XDG_CONFIG_HOME:-$HOME/.config}/lint-arwaky"
CACHE_DIR="${XDG_CACHE_HOME:-$HOME/.cache}/lint-arwaky"
STATE_DIR="${XDG_STATE_HOME:-$HOME/.local/state}/lint-arwaky"
REPORT_DIR="$DATA_DIR/reports"

# ── Defaults ───────────────────────────────────────────────────────────────────
MODE="local"
DRY_RUN=false
REINSTALL=false

usage() {
    cat << 'EOF'
Usage: bash scripts/install.sh [options]

Installer for lint-arwaky — Architecture Linter for Rust, Python & TypeScript

Options:
  --local       Local XDG installation (default)
  --global      Global system-wide installation (requires root)
  --remote      Try pre-built binaries, fallback to cargo build
  --dev         Full developer setup (tools, hooks, build)
  --reinstall   Clean existing installation before installing
  --dry-run     Show what would be done without doing it
  -h, --help    Show this help

Examples:
  bash scripts/install.sh                # Local XDG install
  bash scripts/install.sh --remote       # Pre-built binary download
  bash scripts/install.sh --dev          # Full dev environment
  bash scripts/install.sh --global       # System-wide install (sudo)
EOF
    exit 0
}

while [[ $# -gt 0 ]]; do
    case "$1" in
        --local)     MODE="local"; shift ;;
        --global)    MODE="global"; shift ;;
        --remote)    MODE="remote"; shift ;;
        --dev)       MODE="dev"; shift ;;
        --reinstall) REINSTALL=true; shift ;;
        --dry-run)   DRY_RUN=true; shift ;;
        -h|--help)   usage ;;
        *)           die "Unknown option: $1 (use -h for help)" ;;
    esac
done

# ── Banner ─────────────────────────────────────────────────────────────────────
echo -e "${BOLD}"
echo "     _             _      _                               "
echo "    | |   (_)_ __   ___ | |    / \   _ __ __      ____ _  | | ___ _   _ "
echo "    | |   | | '_ \ / _ \| |   / _ \ | '__|\ \ /\ / / _\` || |/ / | | | |"
echo "    | |___| | | | | (_) | |___/ ___ \| |    \ V  V / (_| ||   <| |_| |"
echo "    |_____|_|_| |_|\___/|____/_/   \_\_|     \_/\_/ \__,_||_|\_\\\\__, |"
echo "                                                                |___/ "
echo -e "${NC}"

# ── Detect if running from source or remote ────────────────────────────────────
CARGO_TOML="$PROJECT_ROOT/Cargo.toml"
RELEASE_DIR="$PROJECT_ROOT/target/release"
DIST_DIR="$PROJECT_ROOT/dist"

IS_LOCAL_SOURCE=false
if [ -f "$CARGO_TOML" ]; then
    IS_LOCAL_SOURCE=true
fi

# ── Helper: run or dry-run ─────────────────────────────────────────────────────
run_cmd() {
    if $DRY_RUN; then
        echo "  [dry-run] $*"
    else
        eval "$@"
    fi
}

# ── Helper: install binary ─────────────────────────────────────────────────────
install_binary() {
    local src="$1"
    local dst="$2"
    if $DRY_RUN; then
        echo "  [dry-run] install -m 0755 $src $dst"
    else
        install -m 0755 "$src" "$dst"
        echo "  -> $dst"
    fi
}

# ── Helper: remove directory ──────────────────────────────────────────────────
remove_dir() {
    local dir="$1"
    if [ -d "$dir" ]; then
        if $DRY_RUN; then
            echo "  [dry-run] rm -rf $dir"
        else
            rm -rf "$dir"
            echo "  Removed $dir"
        fi
    fi
}

# ── Step: Setup XDG directories ───────────────────────────────────────────────
setup_xdg_dirs() {
    local install_bin="$1"
    local config_dir="$2"
    local report_dir="$3"

    echo -e "\n${BOLD}[1/5] Preparing directories...${NC}"

    if $REINSTALL; then
        info "Cleaning existing installation (--reinstall)..."
        remove_dir "$config_dir"
        remove_dir "$report_dir"
        remove_dir "$DATA_DIR"
        remove_dir "$CACHE_DIR"
        remove_dir "$STATE_DIR"
    fi

    if $DRY_RUN; then
        echo "  [dry-run] mkdir -p $config_dir/rules $report_dir $install_bin"
    else
        mkdir -p "$config_dir/rules" "$report_dir" "$install_bin"
    fi
}

# ── Step: Install dependencies ────────────────────────────────────────────────
install_dependencies() {
    local include_dev="${1:-false}"

    echo -e "\n${BOLD}[2/5] Checking dependencies...${NC}"
    detect_pkg_mgr

    # Core dependencies
    if [ "$MODE" = "dev" ] || [ "$include_dev" = "true" ]; then
        install_if_missing cargo "Rust/Cargo" 'curl --proto "=https" --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y && . "$HOME/.cargo/env"'
        install_if_missing sccache "sccache" "sccache_install"
        install_if_missing mold "mold" "mold_install"
    elif [ "$MODE" = "local" ] || [ "$MODE" = "global" ]; then
        install_if_missing cargo "Rust/Cargo" 'curl --proto "=https" --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y && . "$HOME/.cargo/env"'
    fi

    # External linters
    install_if_missing npm "npm" "npm_install"
    install_if_missing eslint "eslint" "npm_install_global eslint"
    install_if_missing tsc "typescript" "npm_install_global typescript"
    install_if_missing mypy "mypy" "pip_install mypy"
    install_if_missing ruff "ruff" "pip_install ruff"
    install_if_missing bandit "bandit" "pip_install bandit"

    # Dev-only dependencies
    if [ "$MODE" = "dev" ] || [ "$include_dev" = "true" ]; then
        install_if_missing prettier "prettier" "npm_install_global prettier"
        install_if_missing pytest "pytest" "pip_install pytest pytest-cov"
    fi
}

# ── Step: Build from source ───────────────────────────────────────────────────
build_from_source() {
    echo -e "\n${BOLD}[3/5] Building from source...${NC}"

    if ! command -v cargo &>/dev/null; then
        die "cargo not found. Install Rust first: curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
    fi

    if $DRY_RUN; then
        echo "  [dry-run] RUST_MIN_STACK=33554432 cargo build --release"
    else
        RUST_MIN_STACK=33554432 cargo build --release
    fi
}

# ── Step: Install pre-built binary ────────────────────────────────────────────
install_prebuilt() {
    local install_bin="$1"

    echo -e "\n${BOLD}[3/5] Downloading pre-built binary...${NC}"

    ARCH=$(uname -m)
    case "$ARCH" in
        x86_64)        ARCH="x86_64" ;;
        aarch64|arm64) ARCH="aarch64" ;;
        *)             warn "Unknown architecture $ARCH, defaulting to x86_64"; ARCH="x86_64" ;;
    esac

    DOWNLOAD_URL="https://github.com/rakaarwaky/lint-arwaky/releases/latest/download/lint-arwaky-latest-linux-${ARCH}.tar.gz"

    if $DRY_RUN; then
        echo "  [dry-run] curl -fsSL $DOWNLOAD_URL -o /tmp/lint-arwaky.tar.gz"
        echo "  [dry-run] tar xzf /tmp/lint-arwaky.tar.gz -C $install_bin"
        return 0
    fi

    if curl -fsSL "$DOWNLOAD_URL" -o /tmp/lint-arwaky.tar.gz 2>/dev/null; then
        tar xzf /tmp/lint-arwaky.tar.gz -C "$install_bin" lint-arwaky-cli lint-arwaky-mcp lint-arwaky-tui 2>/dev/null && {
            chmod +x "$install_bin"/lint-arwaky-*
            echo -e "  ${GREEN}✓ Installed pre-built binaries from GitHub Release${NC}"
            rm -f /tmp/lint-arwaky.tar.gz
            return 0
        }
    fi

    warn "Pre-built binary unavailable for $ARCH"
    info "Falling back to cargo build..."
    build_from_source
}

# ── Step: Install binaries to target ──────────────────────────────────────────
install_binaries() {
    local install_bin="$1"
    local binaries=("${@:2}")

    echo -e "\n${BOLD}[4/5] Installing binaries...${NC}"

    for BIN in "${binaries[@]}"; do
        if [ "$MODE" = "remote" ] && [ -f "$install_bin/$BIN" ]; then
            echo "  [skip] $BIN already installed"
        elif [ -f "$RELEASE_DIR/$BIN" ]; then
            install_binary "$RELEASE_DIR/$BIN" "$install_bin/$BIN"
        elif [ -f "$install_bin/$BIN" ]; then
            echo "  [skip] $BIN already installed"
        else
            warn "Binary $BIN not found, skipping"
        fi
    done
}

# ── Step: Install docs & config ───────────────────────────────────────────────
install_docs() {
    local config_dir="$1"
    local project_root="$2"

    echo -e "\n${BOLD}[5/5] Installing docs & configuration...${NC}"

    if [ -d "$project_root" ]; then
        copy_docs_to_config "$config_dir" "$project_root"
        copy_agents_to_config "$config_dir" "$project_root"
    else
        info "Source not available, skipping doc installation"
    fi
}

# ── Step: Setup shell aliases ─────────────────────────────────────────────────
setup_aliases() {
    for RC_FILE in "$HOME/.bashrc" "$HOME/.zshrc"; do
        if [ -f "$RC_FILE" ] && ! grep -q "alias la=" "$RC_FILE"; then
            if $DRY_RUN; then
                echo "  [dry-run] Add aliases to $RC_FILE"
            else
                echo "" >> "$RC_FILE"
                echo "# Lint Arwaky Aliases" >> "$RC_FILE"
                echo 'alias la="lint-arwaky"' >> "$RC_FILE"
                echo 'alias lac="lint-arwaky-cli"' >> "$RC_FILE"
                echo 'alias lat="lint-arwaky-tui"' >> "$RC_FILE"
                echo 'alias lam="lint-arwaky-mcp"' >> "$RC_FILE"
                echo -e "  ${GREEN}✓${NC} Shell aliases added to $RC_FILE"
            fi
        fi
    done
}

# ── Step: Setup git hooks ─────────────────────────────────────────────────────
setup_git_hooks() {
    local project_root="$1"
    local git_hooks_dir="$project_root/.git/hooks"

    if [ -d "$git_hooks_dir" ]; then
        if $DRY_RUN; then
            echo "  [dry-run] Install pre-commit hook"
        else
            cat << 'EOF' > "$git_hooks_dir/pre-commit"
#!/usr/bin/env bash
# Git pre-commit hook — runs quality gates before commit
set -e
PROJECT_ROOT="$(git rev-parse --show-toplevel)"
bash "$PROJECT_ROOT/scripts/gates.sh"
EOF
            chmod +x "$git_hooks_dir/pre-commit"
            echo -e "  ${GREEN}✓${NC} Installed .git/hooks/pre-commit -> scripts/gates.sh"
        fi
    fi
}

# ── Step: Verify installation ─────────────────────────────────────────────────
verify_installation() {
    local install_bin="$1"

    echo -e "\n${BOLD}Verifying installation...${NC}"

    for cmd in lint-arwaky-cli lint-arwaky-mcp lint-arwaky-tui; do
        if command -v "$cmd" &>/dev/null || [ -f "$install_bin/$cmd" ]; then
            echo -e "  ${GREEN}✓${NC} $cmd"
        else
            echo -e "  ${YELLOW}⚠${NC} $cmd not found"
        fi
    done
}

# ── MAIN: Local Mode ──────────────────────────────────────────────────────────
install_local() {
    local install_bin="${LINT_ARWAKY_INSTALL_BIN:-${XDG_BIN_HOME:-$HOME/.local/bin}}"
    local config_dir="$CONFIG_DIR"
    local report_dir="$REPORT_DIR"
    local binaries=(lint-arwaky la lint-arwaky-cli lint-arwaky-mcp lint-arwaky-tui)

    setup_xdg_dirs "$install_bin" "$config_dir" "$report_dir"
    install_dependencies false
    build_from_source
    install_binaries "$install_bin" "${binaries[@]}"
    install_docs "$config_dir" "$PROJECT_ROOT"
    setup_aliases
    verify_installation "$install_bin"

    local version
    version=$(get_project_version)
    echo -e "\n${BOLD}${GREEN}Done (Local): $version${NC}"
    echo "Config: $config_dir"
    echo "Reports: $report_dir"
}

# ── MAIN: Global Mode ─────────────────────────────────────────────────────────
install_global() {
    if [ "$(id -u)" -ne 0 ]; then
        warn "Global installation typically requires root. Re-running with sudo..."
        sudo bash "$0" --global ${REINSTALL:+--reinstall} ${DRY_RUN:+--dry-run}
        return
    fi

    local install_bin="${LINT_ARWAKY_INSTALL_BIN:-/usr/local/bin}"
    local config_dir="${LINT_ARWAKY_CONFIG_DIR:-/etc/lint-arwaky}"
    local report_dir="${LINT_ARWAKY_REPORT_DIR:-/var/lib/lint-arwaky/reports}"
    local binaries=(lint-arwaky-cli lint-arwaky-mcp lint-arwaky-tui)

    setup_xdg_dirs "$install_bin" "$config_dir" "$report_dir"
    install_dependencies false
    build_from_source
    install_binaries "$install_bin" "${binaries[@]}"
    install_docs "$config_dir" "$PROJECT_ROOT"

    local version
    version=$(get_project_version)
    echo -e "\n${BOLD}${GREEN}Done (Global): $version${NC}"
    echo "Config: $config_dir"
    echo "Reports: $report_dir"
}

# ── MAIN: Remote Mode ─────────────────────────────────────────────────────────
install_remote() {
    local install_bin="${LINT_ARWAKY_INSTALL_BIN:-${XDG_BIN_HOME:-$HOME/.local/bin}}"
    local config_dir="$CONFIG_DIR"
    local report_dir="$REPORT_DIR"
    local binaries=(lint-arwaky-cli lint-arwaky-mcp lint-arwaky-tui)

    setup_xdg_dirs "$install_bin" "$config_dir" "$report_dir"
    install_dependencies false
    install_prebuilt "$install_bin"
    install_binaries "$install_bin" "${binaries[@]}"
    install_docs "$config_dir" "$PROJECT_ROOT"
    setup_aliases
    verify_installation "$install_bin"

    echo -e "\n${BOLD}${GREEN}Done (Remote)!${NC}"
    echo "Config: $config_dir"
    echo "Reports: $report_dir"
}

# ── MAIN: Dev Mode ────────────────────────────────────────────────────────────
install_dev() {
    local install_bin="${LINT_ARWAKY_INSTALL_BIN:-${XDG_BIN_HOME:-$HOME/.local/bin}}"
    local config_dir="$CONFIG_DIR"
    local report_dir="$REPORT_DIR"
    local binaries=(lint-arwaky la lint-arwaky-cli lint-arwaky-mcp lint-arwaky-tui)

    # Extra dev tools
    echo -e "\n${BOLD}Setting up developer tools...${NC}"

    if ! command -v cargo &>/dev/null; then
        info "Installing Rust toolchain..."
        curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
        source "$HOME/.cargo/env" || true
    fi

    rustup component add clippy rustfmt 2>/dev/null || true

    # cargo extensions
    for tool in cargo-nextest cargo-audit cargo-watch; do
        if ! command -v "$tool" &>/dev/null; then
            info "Installing $tool..."
            cargo install "$tool" --locked || true
        fi
    done

    # mold linker
    if ! command -v mold &>/dev/null; then
        info "Attempting to install mold linker..."
        mold_install
    fi

    setup_xdg_dirs "$install_bin" "$config_dir" "$report_dir"
    install_dependencies true
    build_from_source
    install_binaries "$install_bin" "${binaries[@]}"
    install_docs "$config_dir" "$PROJECT_ROOT"
    setup_aliases
    setup_git_hooks "$PROJECT_ROOT"
    verify_installation "$install_bin"

    # Run quality gates
    if [ -f "$PROJECT_ROOT/scripts/gates.sh" ]; then
        echo -e "\n${BOLD}Running quality gates...${NC}"
        bash "$PROJECT_ROOT/scripts/gates.sh"
    fi

    echo -e "\n${BOLD}${GREEN}======================================================${NC}"
    echo -e "${BOLD}${GREEN} Developer environment setup complete! Happy coding! 🚀${NC}"
    echo -e "${BOLD}${GREEN}======================================================${NC}"
}

# ── Execute Mode ──────────────────────────────────────────────────────────────
case "$MODE" in
    local)  install_local ;;
    global) install_global ;;
    remote) install_remote ;;
    dev)    install_dev ;;
esac
