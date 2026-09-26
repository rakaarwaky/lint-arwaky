#!/usr/bin/env bash
# ==============================================================================
# install.sh — unified installer for lint-arwaky
#
# Merges the previous install.local.sh, install.global.sh and install.remote.sh
# into one script. The three installation targets are selected with --mode.
#
# Usage:
#   bash scripts/install.sh                      # local  (default)
#   bash scripts/install.sh --mode global        # system-wide
#   bash scripts/install.sh --mode remote        # pre-built binary, no cargo build
#
# Remote / curl usage:
#   curl -sSL .../scripts/install.sh | bash -s -- --mode remote
#
# Run with -h for the full flag reference.
# ==============================================================================
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" 2>/dev/null && pwd || echo "")"

# ── Project root: only when running from inside the lint-arwaky repo ──────────
PROJECT_ROOT=""
if [ -n "$SCRIPT_DIR" ] && [ -f "$SCRIPT_DIR/../Cargo.toml" ]; then
    PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
fi

# ── Shared library: source when available, else define fallbacks inline so the
#    script still works when piped from curl (lib.sh is not fetched). ──────────
if [ -n "$SCRIPT_DIR" ] && [ -f "$SCRIPT_DIR/lib.sh" ]; then
    # shellcheck source=lib.sh
    source "$SCRIPT_DIR/lib.sh"
else
    if [ -t 1 ]; then
        GREEN='\033[0;32m'; RED='\033[0;31m'; YELLOW='\033[1;33m'
        CYAN='\033[0;36m'; BOLD='\033[1m'; NC='\033[0m'
    else
        GREEN=''; RED=''; YELLOW=''; CYAN=''; BOLD=''; NC=''
    fi

    pass() { echo -e " ${GREEN}✔${NC} $1"; }
    info() { echo -e " ${CYAN}→${NC} $1"; }
    warn() { echo -e " ${YELLOW}⚠${NC} $1"; }
    fail() { echo -e " ${RED}✘${NC} $1"; }
    die()  { fail "$1"; exit 1; }

    detect_pkg_mgr() {
        if command -v apt-get &>/dev/null;   then PKG_MGR="apt"
        elif command -v dnf &>/dev/null;     then PKG_MGR="dnf"
        elif command -v brew &>/dev/null;    then PKG_MGR="brew"
        elif command -v pacman &>/dev/null;  then PKG_MGR="pacman"
        else                                      PKG_MGR="unknown"
        fi
    }

    npm_install() {
        case "${PKG_MGR:-unknown}" in
            apt)    curl -fsSL https://deb.nodesource.com/setup_lts.x | sudo -E bash - && sudo apt-get install -y nodejs ;;
            dnf)    curl -fsSL https://rpm.nodesource.com/setup_lts.x | sudo bash - && sudo dnf install -y nodejs ;;
            brew)   brew install node ;;
            pacman) sudo pacman -S --noconfirm nodejs npm ;;
            *)      warn "Unknown package manager. Install node/npm manually." ;;
        esac
    }

    npm_install_global() {
        if [ "$(id -u)" -eq 0 ]; then npm install -g "$1"
        else sudo npm install -g "$1" 2>/dev/null || npm install -g "$1"; fi
    }

    pip_install() {
        if command -v pip3 &>/dev/null; then pip3 install --user "$1"
        elif command -v pip &>/dev/null; then pip install --user "$1"
        else warn "pip not found. Install $1 manually."; fi
    }

    install_if_missing() {
        local cmd="$1" pkg="$2" method="$3"
        if command -v "$cmd" &>/dev/null; then
            echo "  [skip] $cmd already installed"
        else
            echo "  [install] $pkg..."
            eval "$method" || warn "Automatic install of $pkg failed. Install it manually."
        fi
    }
fi

# ── Mode ───────────────────────────────────────────────────────────────────────
MODE="${LINT_ARWAKY_INSTALL_MODE:-local}"
EXPLAIN=true

# ── Binaries and build paths ───────────────────────────────────────────────────
BINARIES=(lint-arwaky-cli lint-arwaky-mcp lint-arwaky-tui)
RELEASE_DIR="${PROJECT_ROOT:+$PROJECT_ROOT/target/release}"
DIST_DIR="${PROJECT_ROOT:+$PROJECT_ROOT/dist}"

REPO_SLUG="${LINT_ARWAKY_REPO_SLUG:-rakaarwaky/lint-arwaky}"
RAW_BASE="https://raw.githubusercontent.com/$REPO_SLUG/main"

# ── Usage ──────────────────────────────────────────────────────────────────────
usage() {
    cat <<'USAGE'
Usage: bash scripts/install.sh [options]

Options:
  --mode <local|global|remote|dev>  Installation target
                                     (default: $LINT_ARWAKY_INSTALL_MODE, else local)
  --no-explain                       Skip the post-install explanation block
  --no-gates                         With --mode dev, skip the quality gates run at the end
  -h, --help                         Show this help

Installation modes:
  local    Build from source with cargo, install into XDG user directories.
           Binaries  ~/.cargo/bin
           Config    ~/.config/lint-arwaky
           Reports   ~/.local/share/lint-arwaky/reports
           Adds the aliases lac / lat / lam to ~/.bashrc and ~/.zshrc.

  global   Build from source with cargo, install system-wide. Re-executes
           itself under sudo when not already root.
           Binaries  /usr/local/bin
           Config    /etc/lint-arwaky
           Reports   /var/lib/lint-arwaky/reports
           Available to every user on the machine. No shell aliases.

  remote   Download a pre-built release tarball, so no Rust toolchain and no
           source build are required. Falls back to cargo install from
           crates.io, then to a local build, then to cargo install from git.
           Layout matches local mode.

  dev      Full developer onboarding. Installs the Rust toolchain, the dev
           extensions (cargo-nextest, cargo-audit, cargo-watch, mold), the
           extra linter adapters (prettier, pytest), and a git pre-commit
           hook. Then performs a local install, installs the shell aliases,
           and runs scripts/gates.sh to confirm the workspace passes.
           Requires the lint-arwaky source tree and takes considerably
           longer than the other modes.

Environment variables:
  LINT_ARWAKY_INSTALL_MODE      Default mode when --mode is omitted
  LINT_ARWAKY_INSTALL_BIN       Override the binary install directory
  LINT_ARWAKY_CONFIG_DIR        Override the config directory
  LINT_ARWAKY_REPORT_DIR        Override the reports directory
  LINT_ARWAKY_REPO_SLUG         Owner/repo for remote downloads
                                (default: rakaarwaky/lint-arwaky)
  LINT_ARWAKY_RELEASE_URL       Full tarball URL, overrides remote detection

Examples:
  bash scripts/install.sh
  bash scripts/install.sh --mode global
  bash scripts/install.sh --mode remote
  bash scripts/install.sh --mode dev                # onboarding + gates
  bash scripts/install.sh --mode dev --no-gates     # skip gate run
  LINT_ARWAKY_INSTALL_BIN=~/.local/bin bash scripts/install.sh
  curl -sSL <raw-url>/scripts/install.sh | bash -s -- --mode remote
USAGE
    exit 0
}

# ── Flag parsing ───────────────────────────────────────────────────────────────
# dev-only flags
SKIP_GATES=false

while [[ $# -gt 0 ]]; do
    case "$1" in
        --mode)
            [[ $# -ge 2 ]] || die "--mode requires a value: local | global | remote | dev"
            MODE="$2"
            shift 2
            ;;
        --mode=*)
            MODE="${1#*=}"
            shift
            ;;
        --no-explain)
            EXPLAIN=false
            shift
            ;;
        --no-gates)
            SKIP_GATES=true
            shift
            ;;
        -h|--help) usage ;;
        *) die "Unknown option: $1 (use -h for help)" ;;
    esac
done

case "$MODE" in
    local|global|remote|dev) ;;
    *) die "Invalid mode '$MODE' (allowed: local | global | remote | dev)" ;;
esac

# ── Resolve install paths for the selected mode ────────────────────────────────
INSTALL_BIN="${LINT_ARWAKY_INSTALL_BIN:-}"
CONFIG_DIR="${LINT_ARWAKY_CONFIG_DIR:-}"
REPORT_DIR="${LINT_ARWAKY_REPORT_DIR:-}"

if [ -z "$INSTALL_BIN" ] || [ -z "$CONFIG_DIR" ] || [ -z "$REPORT_DIR" ]; then
    case "$MODE" in
        global)
            [ -n "$INSTALL_BIN" ] || INSTALL_BIN="/usr/local/bin"
            [ -n "$CONFIG_DIR" ]  || CONFIG_DIR="/etc/lint-arwaky"
            [ -n "$REPORT_DIR" ]  || REPORT_DIR="/var/lib/lint-arwaky/reports"
            ;;
        *)
            [ -n "$INSTALL_BIN" ] || INSTALL_BIN="$HOME/.cargo/bin"
            [ -n "$CONFIG_DIR" ]  || CONFIG_DIR="$HOME/.config/lint-arwaky"
            [ -n "$REPORT_DIR" ]  || REPORT_DIR="$HOME/.local/share/lint-arwaky/reports"
            ;;
    esac
fi

# ── Banner ─────────────────────────────────────────────────────────────────────
print_banner() {
    if [ "$MODE" = "remote" ]; then
        cat <<'BANNER'
     _             _      _
    | |   (_)_ __   ___ | |    / \   _ __ __      ____ _  | | ___ _   _
    | |   | | '_ \ / _ \| |   / _ \ | '__|\ \ /\ / / _` || |/ / | | | |
    | |___| | | | | (_) | |___/ ___ \| |    \ V  V / (_| ||   <| |_| |
    |_____|_|_| |_|\___/|____/_/   \_\_|     \_/\_/ \__,_||_|\_\\__, |
                                                                |___/
  Autonomous Code Quality and Architecture Enforcement
BANNER
    elif [ "$MODE" = "dev" ]; then
        cat <<'DEV_BANNER'
  _     _       _         _                   _
 | |   (_)_ __ | |_      / \   _ __ __      ____ _| |_   _
 | |   | | '_ \| __|    / _ \ | '__|\ \ /\ / / _` | / / | | | |
 | |___| | | | | |_    / ___ \| |    \ V  V / (_| |   < | |_| |
 |_____|_|_| |_|\__|  /_/   \_\_|     \_/\_/ \__,_|_|\_\ \__, |
                                                         |___/
  Developer Environment Setup & Onboarding
DEV_BANNER
    fi
}

# ── Shared: layout preparation ─────────────────────────────────────────────────
prepare_layout() {
    local label="$1"
    if [ -d "$CONFIG_DIR" ]; then
        echo "Cleaning existing $label config dir: $CONFIG_DIR"
        rm -rf "$CONFIG_DIR"
    fi
    if [ -d "$REPORT_DIR" ]; then
        echo "Cleaning existing $label report dir: $REPORT_DIR"
        rm -rf "$REPORT_DIR"
    fi
    mkdir -p "$CONFIG_DIR/rules" "$REPORT_DIR" "$INSTALL_BIN"
    if [ -n "$DIST_DIR" ]; then
        mkdir -p "$DIST_DIR"
    fi
}

# ── Shared: external dependency check ──────────────────────────────────────────
check_external_deps() {
    local with_cargo="${1:-yes}"
    echo ""
    echo "==> Checking external dependencies..."
    detect_pkg_mgr

    if [ "$with_cargo" = "yes" ]; then
        install_if_missing cargo "Rust/Cargo" \
            'curl --proto "=https" --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y && . "$HOME/.cargo/env"'
    fi

    # Node/npm first: eslint and tsc depend on it.
    install_if_missing npm "npm" "npm_install"
    install_if_missing eslint "eslint" "npm_install_global eslint"
    install_if_missing tsc "typescript" "npm_install_global typescript"
    install_if_missing mypy "mypy" "pip_install mypy"
    install_if_missing ruff "ruff" "pip_install ruff"
    install_if_missing bandit "bandit" "pip_install bandit"

    echo "==> External dependency check done."
}

# ── Shared: rust toolchain ─────────────────────────────────────────────────────
ensure_cargo() {
    if ! command -v cargo &>/dev/null; then
        echo "  [info] cargo not found. Installing Rust via rustup..."
        install_if_missing cargo "Rust/Cargo" \
            'curl --proto "=https" --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y && . "$HOME/.cargo/env"'
    fi
}

# ── Shared: build from the local repository ────────────────────────────────────
build_from_source() {
    [ -n "$PROJECT_ROOT" ] && [ -f "$PROJECT_ROOT/Cargo.toml" ] \
        || die "No lint-arwaky source tree found. Use --mode remote to install a pre-built binary."
    # RUST_MIN_STACK avoids an LLVM SIGSEGV during LTO linking.
    RUST_MIN_STACK=33554432 cargo build --release
}

# ── Shared: install binaries from the release build dir ────────────────────────
install_from_release_dir() {
    local release_dir="$1"
    mkdir -p "$INSTALL_BIN"

    if [ -n "$DIST_DIR" ]; then
        ( cd "$release_dir" && sha256sum "${BINARIES[@]}" ) > "$DIST_DIR/SHA256SUMS.txt"
    fi

    for bin in "${BINARIES[@]}"; do
        install -m 0755 "$release_dir/$bin" "$INSTALL_BIN/$bin"
        echo "  -> $INSTALL_BIN/$bin"
    done
}

# ── Shared: provision docs, skills and agent rules ─────────────────────────────
DOCS=(ARCHITECTURE.md RULES_AES.md)

fetch_or_copy() {
    # Local repo wins; otherwise pull from raw.githubusercontent.
    local rel="$1" dest="$2"
    if [ -n "$PROJECT_ROOT" ] && [ -f "$PROJECT_ROOT/$rel" ]; then
        cp "$PROJECT_ROOT/$rel" "$dest"
        return 0
    fi
    curl -fsSL "$RAW_BASE/$rel" -o "$dest" 2>/dev/null
}

provision_docs() {
    for doc in "${DOCS[@]}"; do
        if fetch_or_copy "$doc" "$CONFIG_DIR/$doc"; then
            echo "  $doc -> $CONFIG_DIR/$doc"
        else
            warn "Could not install $doc"
        fi
    done

    if fetch_or_copy ".agents/rules/RULES_AES.md" "$CONFIG_DIR/RULES_AES.md"; then
        echo "  RULES_AES.md -> $CONFIG_DIR/RULES_AES.md"
    else
        warn "Could not install RULES_AES.md"
    fi
}

provision_skills() {
    local skills_dst="$CONFIG_DIR/.agents/skills"
    if [ -z "$PROJECT_ROOT" ] || [ ! -d "$PROJECT_ROOT/skills" ]; then
        warn "No local skills/ folder — skipping skill provisioning"
        warn "Run 'lint-arwaky-cli init' in a local clone to provision skills there"
        return 0
    fi

    mkdir -p "$skills_dst"
    local copied=0 skill_dir skill_name
    for skill_dir in "$PROJECT_ROOT"/skills/*; do
        [ -d "$skill_dir" ] || continue
        skill_name="$(basename "$skill_dir")"
        rm -rf "${skills_dst:?}/$skill_name"
        cp -r "$skill_dir" "$skills_dst/$skill_name"
        copied=$((copied + 1))
    done

    if [ "$copied" -eq 0 ]; then
        warn "skills/ is empty — add a <skill-name>/SKILL.md to provision"
    else
        pass "$copied skill(s) provisioned to $skills_dst"
    fi
}

provision_agents() {
    [ -n "$PROJECT_ROOT" ] && [ -d "$PROJECT_ROOT/.agents" ] || {
        warn "No local .agents/ folder — skipping agent rules"
        return 0
    }

    local agents_dst="$CONFIG_DIR/.agents"
    mkdir -p "$agents_dst/skills"

    local group src file
    for group in rules prompts research; do
        [ -d "$PROJECT_ROOT/.agents/$group" ] || continue
        mkdir -p "$agents_dst/$group"
        for src in "$PROJECT_ROOT"/.agents/"$group"/*; do
            [ -f "$src" ] || continue
            file="$(basename "$src")"
            cp "$src" "$agents_dst/$group/$file"
            echo "  .agents/$group/$file -> $agents_dst/$group/$file"
        done
    done
}

provision_config() {
    echo ""
    echo "==> Installing docs to $CONFIG_DIR..."
    provision_docs

    echo ""
    echo "==> Provisioning agent skills to $CONFIG_DIR/.agents/skills..."
    provision_skills

    echo ""
    echo "==> Copying .agents rules and research to $CONFIG_DIR/.agents..."
    provision_agents
}

# ── Shared: shell aliases ─────────────────────────────────────────────────────
add_shell_aliases() {
    local rc_file
    for rc_file in "$HOME/.bashrc" "$HOME/.zshrc"; do
        if [ -f "$rc_file" ] && ! grep -q 'alias lac=' "$rc_file"; then
            {
                echo ""
                echo "# Lint Arwaky Aliases"
                echo 'alias lac="lint-arwaky-cli"'
                echo 'alias lat="lint-arwaky-tui"'
                echo 'alias lam="lint-arwaky-mcp"'
            } >> "$rc_file"
            echo "  -> Shell aliases added to $rc_file"
        fi
    done
}

# ── Shared: verification ──────────────────────────────────────────────────────
verify_installation() {
    echo ""
    echo "==> Verifying installation..."
    local bin found=false
    for bin in "${BINARIES[@]}"; do
        if command -v "$bin" &>/dev/null; then
            pass "$bin -> $(command -v "$bin")"
            found=true
        elif [ -x "$INSTALL_BIN/$bin" ]; then
            warn "$bin installed at $INSTALL_BIN/$bin but that directory is not in PATH"
        else
            warn "$bin not found"
        fi
    done

    if command -v lint-arwaky-cli &>/dev/null; then
        echo ""
        info "lint-arwaky-cli version: $(lint-arwaky-cli version 2>/dev/null | head -1)"
    fi
    if [ "$found" = true ]; then
        echo ""
        info "If a binary was not detected, add this to your PATH:"
        info "  export PATH=\"$INSTALL_BIN:\$PATH\""
    fi
}

# ── Post-install explanation ───────────────────────────────────────────────────
explain() {
    local version="unknown"
    if [ -n "$PROJECT_ROOT" ] && [ -f "$PROJECT_ROOT/Cargo.toml" ]; then
        version="$(sed -nE 's/^[[:space:]]*version[[:space:]]*=[[:space:]]*"([^"]+)".*/\1/p' \
            "$PROJECT_ROOT/Cargo.toml" | head -1)"
        [ -n "$version" ] || version="unknown"
    elif command -v lint-arwaky-cli &>/dev/null; then
        version="$(lint-arwaky-cli version 2>/dev/null | head -1)"
    fi

    echo ""
    echo -e "${BOLD}============================================================${NC}"
    echo -e "${BOLD} Lint Arwaky installed  ·  mode=$MODE  ·  $version${NC}"
    echo -e "${BOLD}============================================================${NC}"
    echo ""
    echo -e "  ${BOLD}Layout${NC}"
    echo -e "    binaries   $INSTALL_BIN"
    echo -e "    config     $CONFIG_DIR"
    echo -e "    reports    $REPORT_DIR"
    if [ -n "$DIST_DIR" ]; then
        echo -e "    checksums  $DIST_DIR/SHA256SUMS.txt"
    fi
    echo ""
    echo -e "  ${BOLD}What got installed${NC}"
    echo "    lint-arwaky-cli   command line linter (scan, check, fix, ci, doctor)"
    echo "    lint-arwaky-mcp   MCP server over stdin/stdout JSON-RPC 2.0"
    echo "    lint-arwaky-tui   interactive terminal file browser"
    echo ""
    echo -e "  ${BOLD}What the config directory holds${NC}"
    echo "    ARCHITECTURE.md, MIGRATION_*.md, RULES_AES.md   spec and migration guides"
    echo "    .agents/skills/<skill>/SKILL.md                  skills for 'lint-arwaky-cli init'"
    echo "    .agents/rules, .agents/prompts, .agents/research agent rule and prompt files"
    echo ""
    echo -e "  ${BOLD}First commands${NC}"
    echo "    lint-arwaky-cli doctor        check toolchain and binary health"
    echo "    lint-arwaky-cli scan .        run the full rule set on a codebase"
    echo "    lint-arwaky-cli check .       architecture-only check"
    echo "    lint-arwaky-cli mcp-config    print the MCP server config snippet"
    echo "    lint-arwaky-tui               browse a project interactively"
    echo ""
    if [ "$MODE" = "local" ] || [ "$MODE" = "remote" ]; then
        echo -e "  ${BOLD}Aliases${NC}"
        echo "    lac, lat, lam  were appended to ~/.bashrc and ~/.zshrc."
        echo "    Run 'source ~/.bashrc' (or open a new terminal) to pick them up."
        echo ""
    fi
    echo -e "  ${BOLD}Next steps${NC}"
    echo "    In an existing project:  cd <project> && lint-arwaky-cli init"
    echo "    In the lint-arwaky repo: bash scripts/gates.sh"
    echo "    To remove:               bash scripts/uninstall.sh --${MODE}"
    echo ""
    if [ "$MODE" = "global" ]; then
        echo "  Every user on this machine can now run lint-arwaky-cli."
        echo "  Uninstall requires root: sudo bash scripts/uninstall.sh --global"
        echo ""
    fi
}

# ── Mode: local ────────────────────────────────────────────────────────────────
run_local() {
    echo -e "${BOLD}== Lint Arwaky — local install (XDG user layout) ==${NC}"
    echo ""

    echo "==> Preparing directories..."
    prepare_layout "local"

    check_external_deps yes

    echo ""
    echo "==> Building release binaries from source..."
    build_from_source

    echo ""
    echo "==> Installing binaries..."
    install_from_release_dir "$RELEASE_DIR"

    provision_config
    add_shell_aliases
    verify_installation
}

# ── Mode: global ───────────────────────────────────────────────────────────────
run_global() {
    if [ "$(id -u)" -ne 0 ]; then
        echo "Global installation writes to $INSTALL_BIN and $CONFIG_DIR."
        echo "Re-running this script under sudo..."
        sudo env \
            LINT_ARWAKY_INSTALL_MODE=global \
            LINT_ARWAKY_INSTALL_BIN="$INSTALL_BIN" \
            LINT_ARWAKY_CONFIG_DIR="$CONFIG_DIR" \
            LINT_ARWAKY_REPORT_DIR="$REPORT_DIR" \
            bash "$SCRIPT_DIR/install.sh" --no-explain
        return $?
    fi

    echo -e "${BOLD}== Lint Arwaky — global install (system-wide) ==${NC}"
    echo ""

    echo "==> Preparing directories..."
    prepare_layout "global"

    check_external_deps yes

    echo ""
    echo "==> Building release binaries from source..."
    build_from_source

    echo ""
    echo "==> Installing binaries..."
    install_from_release_dir "$RELEASE_DIR"

    provision_config
    verify_installation
    # Global install is shared by every user, so no per-user shell aliases.
}

# ── Mode: remote ───────────────────────────────────────────────────────────────
release_url_for() {
    local arch="$1"
    if [ -n "${LINT_ARWAKY_RELEASE_URL:-}" ]; then
        echo "$LINT_ARWAKY_RELEASE_URL"
        return
    fi
    local version
    version="$(curl -fsSL "https://api.github.com/repos/$REPO_SLUG/releases/latest" 2>/dev/null \
        | sed -n 's/.*"tag_name":[[:space:]]*"v\?\([^"]*\)".*/\1/p' | head -1)"
    if [ -n "$version" ]; then
        echo "https://github.com/$REPO_SLUG/releases/download/v$version/lint-arwaky-v$version-linux-$arch.tar.gz"
    else
        echo ""
    fi
}

download_prebuilt() {
    local arch
    case "$(uname -m)" in
        x86_64|amd64)     arch="x86_64" ;;
        aarch64|arm64)    arch="aarch64" ;;
        *)
            warn "No pre-built binary published for $(uname -m)"
            return 1
            ;;
    esac

    local url
    url="$(release_url_for "$arch")"
    [ -n "$url" ] || { warn "Could not resolve the latest release for $REPO_SLUG"; return 1; }

    info "Downloading $url"
    local tmp
    tmp="$(mktemp -t lint-arwaky-XXXXXX.tar.gz)"
    if ! curl -fsSL "$url" -o "$tmp" 2>/dev/null; then
        rm -f "$tmp"
        warn "Pre-built binary unavailable for linux-$arch"
        return 1
    fi

    local extract_dir
    extract_dir="$(mktemp -d -t lint-arwaky-XXXXXX)"
    if ! tar xzf "$tmp" -C "$extract_dir" 2>/dev/null; then
        rm -rf "$tmp" "$extract_dir"
        warn "Could not extract the release tarball"
        return 1
    fi
    rm -f "$tmp"

    mkdir -p "$INSTALL_BIN"
    local bin found=false
    for bin in "${BINARIES[@]}"; do
        if [ -f "$extract_dir/$bin" ]; then
            install -m 0755 "$extract_dir/$bin" "$INSTALL_BIN/$bin"
            echo "  -> $INSTALL_BIN/$bin"
            found=true
        elif [ -f "$extract_dir/lint-arwaky-$bin" ]; then
            install -m 0755 "$extract_dir/lint-arwaky-$bin" "$INSTALL_BIN/$bin"
            echo "  -> $INSTALL_BIN/$bin"
            found=true
        fi
    done
    rm -rf "$extract_dir"

    if [ "$found" != true ]; then
        warn "Release tarball did not contain the expected binaries"
        return 1
    fi

    if [ -n "$DIST_DIR" ] && [ -d "$INSTALL_BIN" ]; then
        mkdir -p "$DIST_DIR"
        ( cd "$INSTALL_BIN" && sha256sum "${BINARIES[@]}" 2>/dev/null ) > "$DIST_DIR/SHA256SUMS.txt" || true
    fi

    pass "Installed pre-built binaries from GitHub Release"
    return 0
}

install_via_cargo() {
    ensure_cargo
    command -v cargo &>/dev/null \
        || die "cargo is unavailable and no pre-built binary was published.
Install Rust from https://rustup.rs, or download a release from
https://github.com/$REPO_SLUG/releases"

    if cargo install lint_arwaky-arwaky --force 2>/dev/null; then
        pass "Installed lint_arwaky-arwaky from crates.io"
        return 0
    fi

    if [ -n "$PROJECT_ROOT" ] && [ -f "$PROJECT_ROOT/Cargo.toml" ]; then
        info "Building from the local repository..."
        build_from_source
        install_from_release_dir "$RELEASE_DIR"
        return 0
    fi

    info "Installing from the git repository..."
    cargo install --git "https://github.com/$REPO_SLUG.git" --force
}

run_remote() {
    print_banner
    echo ""

    echo "==> Preparing directories..."
    prepare_layout "remote"

    check_external_deps no

    echo ""
    if ! download_prebuilt; then
        echo ""
        echo "==> Falling back to cargo install..."
        install_via_cargo
    fi

    provision_config
    add_shell_aliases
    verify_installation
}

# ── Mode: dev ──────────────────────────────────────────────────────────────────
install_cargo_dev_tool() {
    local cmd="$1"
    local crate="$2"
    local desc="$3"
    if command -v "$cmd" &>/dev/null; then
        echo "  [skip] $desc already installed"
    else
        echo "  [install] $desc..."
        cargo install "$crate" --locked 2>&1 | tail -3 || warn "Installation of $desc failed (optional)"
    fi
}

install_mold_linker() {
    if command -v mold &>/dev/null; then
        echo "  [skip] mold linker already installed"
        return 0
    fi
    echo "  [optional] Installing mold linker..."
    case "${PKG_MGR:-unknown}" in
        apt)    sudo apt-get install -y mold 2>/dev/null || warn "apt install of mold failed" ;;
        dnf)    sudo dnf install -y mold 2>/dev/null || warn "dnf install of mold failed" ;;
        pacman) sudo pacman -S --noconfirm mold 2>/dev/null || warn "pacman install of mold failed" ;;
        brew)   brew install mold 2>/dev/null || warn "brew install of mold failed" ;;
        *)      warn "No known package manager for mold. Install it manually." ;;
    esac
}

install_git_hook() {
    [ -n "$PROJECT_ROOT" ] || {
        warn "No lint-arwaky source tree found — skipping git hook installation"
        return 0
    }
    local hooks_dir
    hooks_dir="$(git -C "$PROJECT_ROOT" rev-parse --git-path hooks 2>/dev/null)" || hooks_dir=""
    if [ -z "$hooks_dir" ] || [ ! -d "$hooks_dir" ]; then
        warn "Not a git workspace or .git/hooks missing — skipping pre-commit hook"
        return 0
    fi
    local hook_file="$hooks_dir/pre-commit"
    if [ -f "$hook_file" ] && grep -q "gates.sh" "$hook_file" 2>/dev/null; then
        echo "  [skip] pre-commit hook already installed"
        return 0
    fi
    cat > "$hook_file" <<'EOF'
#!/usr/bin/env bash
# Git pre-commit hook — runs quality gates before commit
set -e
PROJECT_ROOT="$(git rev-parse --show-toplevel)"
bash "$PROJECT_ROOT/scripts/gates.sh"
EOF
    chmod +x "$hook_file"
    pass "Installed .git/hooks/pre-commit -> scripts/gates.sh"
}

run_dev() {
    print_banner
    echo ""

    # 1. Platform detection
    echo -e "${BOLD}[1/6] Detecting platform and package manager...${NC}"
    detect_pkg_mgr
    echo "  Platform: $(uname -s), Package Manager: $PKG_MGR"

    # 2. Rust toolchain & dev extensions
    echo -e "\n${BOLD}[2/6] Setting up Rust toolchain & developer tools...${NC}"
    if ! command -v cargo &>/dev/null; then
        echo "  [install] Rust toolchain..."
        curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
        # shellcheck source=/dev/null
        . "$HOME/.cargo/env" 2>/dev/null || true
    else
        echo "  [skip] Rust/Cargo already installed ($(cargo --version))"
    fi
    rustup component add clippy rustfmt 2>/dev/null || true

    install_cargo_dev_tool cargo-nextest "cargo-nextest" \
        "cargo-nextest (parallel test runner)"
    install_cargo_dev_tool cargo-audit "cargo-audit" \
        "cargo-audit (security vulnerability scanner)"
    install_cargo_dev_tool cargo-watch "cargo-watch" \
        "cargo-watch (auto-rebuild on file changes)"
    install_mold_linker

    # 3. External linter adapters — extra tools for dev mode
    echo -e "\n${BOLD}[3/6] Installing external linter adapters...${NC}"
    check_external_deps yes
    install_if_missing prettier "prettier" "npm_install_global prettier"
    install_if_missing pytest "pytest" "pip_install pytest pytest-cov"

    # 4. Local install + build
    echo -e "\n${BOLD}[4/6] Building project & installing local XDG config...${NC}"
    run_local

    # 5. Git hooks
    echo -e "\n${BOLD}[5/6] Setting up Git hooks...${NC}"
    install_git_hook

    # 6. Quality gates
    if [ "$SKIP_GATES" = true ]; then
        echo -e "\n${BOLD}[6/6] Quality gates skipped (--no-gates flag).${NC}"
    else
        echo -e "\n${BOLD}[6/6] Running quality gates...${NC}"
        if [ -n "$PROJECT_ROOT" ] && [ -f "$PROJECT_ROOT/scripts/gates.sh" ]; then
            bash "$PROJECT_ROOT/scripts/gates.sh"
        else
            warn "scripts/gates.sh not found at $PROJECT_ROOT — skipping gates"
        fi
    fi

    echo -e "\n${BOLD}${GREEN}======================================================${NC}"
    echo -e "${BOLD}${GREEN} Developer environment setup complete! Happy coding!${NC}"
    echo -e "${BOLD}${GREEN}======================================================${NC}"
}

# ── Dispatch ───────────────────────────────────────────────────────────────────
case "$MODE" in
    local)  run_local ;;
    global) run_global ;;
    remote) run_remote ;;
    dev)    run_dev ;;
esac

if [ "$EXPLAIN" = true ] && [ "$MODE" != "dev" ]; then
    explain
fi
