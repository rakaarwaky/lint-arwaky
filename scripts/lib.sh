#!/usr/bin/env bash
# lib.sh — Shared functions for all lint-arwaky scripts
# Source this file: source "$(dirname "${BASH_SOURCE[0]}")/lib.sh"
set -euo pipefail

# ── Colors ──────────────────────────────────────────────────────────────────────
if [ -t 1 ]; then
    GREEN='\033[0;32m'
    RED='\033[0;31m'
    YELLOW='\033[1;33m'
    CYAN='\033[0;36m'
    BOLD='\033[1m'
    NC='\033[0m'
else
    GREEN='' RED='' YELLOW='' CYAN='' BOLD='' NC=''
fi

pass() { echo -e " ${GREEN}✔${NC} $1"; }
info() { echo -e " ${CYAN}→${NC} $1"; }
warn() { echo -e " ${YELLOW}⚠${NC} $1"; }
fail() { echo -e " ${RED}✘${NC} $1"; }
die()  { fail "$1"; exit 1; }

# ── Project Root ────────────────────────────────────────────────────────────────
SCRIPTS_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPTS_DIR/.." && pwd)"

# ── Install Bookkeeping ─────────────────────────────────────────────────────────
# A silent skip must never look like a success, so every helper records its
# outcome and install_summary() reports all three lists at the end of a run.
INSTALL_DONE=()
INSTALL_SKIPPED=()
INSTALL_MANUAL=()

record_installed() {
    if ! is_dry_run; then
        local existing
        for existing in "${INSTALL_DONE[@]}"; do
            [ "$existing" = "$1" ] && return 0
        done
        INSTALL_DONE+=("$1")
    fi
}
record_skipped()   {
    local entry
    for entry in ${INSTALL_SKIPPED[@]+"${INSTALL_SKIPPED[@]}"}; do
        [ "$entry" = "$1" ] && return 0
    done
    INSTALL_SKIPPED+=("$1")
}
record_manual() {
    if ! is_dry_run; then
        local key="${1%%|*}"
        local existing
        for existing in "${INSTALL_MANUAL[@]}"; do
            [ "${existing%%|*}" = "$key" ] && return 0
        done
        INSTALL_MANUAL+=("$1")
    fi
}

is_dry_run() { [ "${DRY_RUN:-false}" = "true" ]; }

# ── PATH Helpers ────────────────────────────────────────────────────────────────
# rustup and `cargo install` write into $HOME/.cargo/bin, which this shell only
# learns about if we export it: install_if_missing cargo runs rustup inside an
# eval, so its PATH changes never reach the installer.
ensure_cargo_bin_on_path() {
    if [ -d "$HOME/.cargo/bin" ]; then
        case ":$PATH:" in
            *":$HOME/.cargo/bin:"*) ;;
            *) export PATH="$HOME/.cargo/bin:$PATH" ;;
        esac
    fi
}

RUSTUP_CMD='curl --proto "=https" --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y'

install_rust_if_missing() {
    ensure_cargo_bin_on_path
    if command -v cargo &>/dev/null; then
        return 0
    fi
    if is_dry_run; then
        echo "  [dry-run] $RUSTUP_CMD"
        return 0
    fi
    info "Installing Rust toolchain..."
    eval "$RUSTUP_CMD" || true
    ensure_cargo_bin_on_path
    if ! command -v cargo &>/dev/null; then
        die "cargo still missing after rustup, run: $RUSTUP_CMD"
    fi
}

# ── Package Manager Detection ──────────────────────────────────────────────────
detect_pkg_mgr() {
    if command -v apt-get &>/dev/null; then
        PKG_MGR="apt"
    elif command -v dnf &>/dev/null; then
        PKG_MGR="dnf"
    elif command -v brew &>/dev/null; then
        PKG_MGR="brew"
    elif command -v pacman &>/dev/null; then
        PKG_MGR="pacman"
    else
        PKG_MGR="unknown"
    fi
}

# ── Install Helpers ────────────────────────────────────────────────────────────
# Every helper honours $DRY_RUN: it prints "[dry-run] <command>" and runs nothing.
# npm_remedy returns the remedy string for the current PKG_MGR. Both
# npm_install's dry-run branch and install.sh's remedy string call this,
# so the package-manager arms live in one place.
npm_remedy() {
    case "${PKG_MGR:-unknown}" in
        apt)    echo "sudo apt-get install -y nodejs npm" ;;
        dnf)    echo "sudo dnf install -y nodejs npm" ;;
        pacman) echo "sudo pacman -S --noconfirm nodejs npm" ;;
        brew)   echo "brew install node" ;;
        *)      echo "install node/npm with your package manager" ;;
    esac
}

# mold_remedy returns the remedy string for the current PKG_MGR.
mold_remedy() {
    case "${PKG_MGR:-unknown}" in
        apt)    echo "sudo apt-get install -y mold" ;;
        dnf)    echo "sudo dnf install -y mold" ;;
        pacman) echo "sudo pacman -S --noconfirm mold" ;;
        brew)   echo "brew install mold" ;;
        *)      echo "install mold with your package manager" ;;
    esac
}

npm_install() {
    if is_dry_run; then
        echo "  [dry-run] $(npm_remedy)"
        return 0
    fi
    case "${PKG_MGR:-unknown}" in
        apt)    curl -fsSL https://deb.nodesource.com/setup_lts.x | sudo -E bash - && sudo apt-get install -y nodejs ;;
        dnf)    curl -fsSL https://rpm.nodesource.com/setup_lts.x | sudo bash - && sudo dnf install -y nodejs ;;
        brew)   brew install node ;;
        pacman) sudo pacman -S --noconfirm nodejs npm ;;
        *)      warn "Unknown package manager. Install node/npm manually." ;;
    esac
}

# npm_install_cmd is intentionally not kept as a separate helper: the remedy
# string is derived from npm_remedy() so the package-manager arms live in one
# place; the dry-run branch of npm_install and install.sh both call it.

npm_install_global() {
    if is_dry_run; then
        echo "  [dry-run] npm install -g $1"
        return 0
    fi
    if [ "$(id -u)" -eq 0 ]; then
        npm install -g "$1"
    else
        sudo npm install -g "$1" 2>/dev/null || npm install -g "$1"
    fi
}

# Debian-family images ship python3 without pip and without ensurepip, so the
# remedy names the distro package that provides pip.
pip_install_cmd() {
    case "${PKG_MGR:-unknown}" in
        apt)    echo "sudo apt-get install -y python3-pip" ;;
        dnf)    echo "sudo dnf install -y python3-pip" ;;
        pacman) echo "sudo pacman -S --noconfirm python-pip" ;;
        brew)   echo "brew install python" ;;
        *)      echo "python3 -m ensurepip --user" ;;
    esac
}

# Probe capabilities, not binary names: pip often exists only as `python3 -m pip`.
# Returns 1 after printing a remedy, so the caller can report a skip.
pip_install() {
    local pkgs=("$@")

    # Under dry-run nothing is executed (R5), so the probe must not run.
    if is_dry_run; then
        # No probe under dry-run: print a static line, best-effort fidelity.
        echo "  [dry-run] python3 -m pip install --user ${pkgs[*]}"
        return 0
    fi

    # Probe only in real mode.
    local -a pip_cmd=()
    if python3 -m pip --version &>/dev/null; then
        pip_cmd=(python3 -m pip)
    elif command -v pip3 &>/dev/null; then
        pip_cmd=(pip3)
    elif command -v pip &>/dev/null; then
        pip_cmd=(pip)
    fi

    if [ ${#pip_cmd[@]} -eq 0 ]; then
        local remedy
        remedy=$(pip_install_cmd)
        warn "No pip for python3, install it with: $remedy"
        warn "Then install manually: python3 -m pip install --user ${pkgs[*]}"
        record_manual "${pkgs[*]}|$remedy && python3 -m pip install --user ${pkgs[*]}"
        return 1
    fi

    local out
    if out=$("${pip_cmd[@]}" install --user "${pkgs[@]}" 2>&1); then
        return 0
    fi

    if grep -qi 'externally-managed-environment' <<<"$out"; then
        info "System Python is externally managed (PEP 668), retrying with --break-system-packages."
        if out=$("${pip_cmd[@]}" install --user --break-system-packages "${pkgs[@]}" 2>&1); then
            return 0
        fi
        warn "pip install failed even with --break-system-packages, use a venv: python3 -m venv .venv && .venv/bin/pip install ${pkgs[*]}"
        record_manual "${pkgs[*]}|python3 -m venv .venv && .venv/bin/pip install ${pkgs[*]}"
        return 1
    fi

    warn "pip install failed for ${pkgs[*]}, run: ${pip_cmd[*]} install --user ${pkgs[*]}"
    record_manual "${pkgs[*]}|${pip_cmd[*]} install --user ${pkgs[*]}"
    return 1
}

sccache_distro_cmd() {
    case "${PKG_MGR:-unknown}" in
        pacman) echo "sudo pacman -S --noconfirm sccache" ;;
        apt)    echo "sudo apt-get install -y sccache" ;;
        dnf)    echo "sudo dnf install -y sccache" ;;
        brew)   echo "brew install sccache" ;;
        *)      echo "" ;;
    esac
}

# Prints the commands it attempted when the install fails, so the user is never
# left guessing which package manager or cargo invocation went wrong.
sccache_install() {
    local distro_cmd
    distro_cmd=$(sccache_distro_cmd)
    local -a attempted=()

    if [ -n "$distro_cmd" ]; then
        attempted+=("$distro_cmd")
        if is_dry_run; then
            echo "  [dry-run] $distro_cmd"
            echo "  [dry-run] cargo install sccache --locked  (fallback)"
            return 0
        fi
        if eval "$distro_cmd" >/dev/null 2>&1; then
            ensure_cargo_bin_on_path
            if command -v sccache &>/dev/null; then
                return 0
            fi
        fi
    elif is_dry_run; then
        echo "  [dry-run] cargo install sccache --locked"
        return 0
    fi

    info "sccache is not packaged for this system, compiling it, this takes a few minutes..."
    attempted+=("cargo install sccache --locked")
    ensure_cargo_bin_on_path
    if ! command -v cargo &>/dev/null; then
        fail "sccache needs cargo, run: $RUSTUP_CMD"
        return 1
    fi
    cargo install sccache --locked >/dev/null 2>&1 || true
    ensure_cargo_bin_on_path
    if command -v sccache &>/dev/null; then
        return 0
    fi

    fail "sccache install failed, commands attempted:"
    local cmd
    for cmd in "${attempted[@]}"; do
        echo "    $cmd"
    done
    return 1
}

# sccache is not optional: .cargo/config.toml sets rustc-wrapper = "sccache",
# so every cargo invocation dies with "could not execute process" without it.
ensure_sccache() {
    if ! cargo_uses_sccache; then
        return 0
    fi

    if command -v sccache &>/dev/null; then
        pass "sccache already installed"
        record_skipped sccache
        return 0
    fi

    info "sccache is required by .cargo/config.toml (rustc-wrapper = \"sccache\"), cargo builds fail without it."
    if is_dry_run; then
        sccache_install
        return 0
    fi

    sccache_install || true
    ensure_cargo_bin_on_path
    if command -v sccache &>/dev/null && sccache --version >/dev/null 2>&1; then
        pass "sccache installed"
        record_installed sccache
        return 0
    fi

    die "sccache is missing, run: cargo install sccache --locked (or delete rustc-wrapper from .cargo/config.toml)"
}

# 0 when the effective cargo config sets rustc-wrapper = "sccache". Cargo merges
# config files closest-first, so the first file that defines the key decides.
cargo_uses_sccache() {
    local root="${1:-$PROJECT_ROOT}"
    local cargo_home="${CARGO_HOME:-$HOME/.cargo}"
    local cfg line

    # When CARGO_HOME is unset the third path repeats the second; keep it so
    # a caller that exports CARGO_HOME but does not set HOME still gets the
    # user-level fallback, and add a comment for the redundancy.
    for cfg in "$root/.cargo/config.toml" "$cargo_home/config.toml" "$HOME/.cargo/config.toml"; do
        [ -f "$cfg" ] || continue
        line=$(grep -E '^[[:space:]]*rustc-wrapper[[:space:]]*=' "$cfg" | tail -1) || true
        [ -n "$line" ] || continue
        line=${line#*=}
        line=${line%%#*}
        line=${line//[[:space:]]/}
        line=${line#\"}
        line=${line%\"}
        line=${line#\'}
        line=${line%\'}
        if [ "$line" = "sccache" ]; then
            return 0
        fi
        return 1
    done
    return 1
}

# mold_remedy is defined above npm_remedy; both are shared by the install
# helpers and install.sh so the package-manager arms live in one place.

mold_install() {
    if is_dry_run; then
        echo "  [dry-run] $(mold_remedy)"
        return 0
    fi
    case "${PKG_MGR:-unknown}" in
        pacman) sudo pacman -S --noconfirm mold 2>/dev/null || true ;;
        apt)    sudo apt-get install -y mold 2>/dev/null || true ;;
        dnf)    sudo dnf install -y mold 2>/dev/null || true ;;
        brew)   brew install mold 2>/dev/null || true ;;
        *)      true ;;
    esac
}

# Optional tools: a failed helper is a skip with a remedy, never a fatal error.
install_if_missing() {
    local cmd="$1"
    local pkg="$2"
    local method="$3"
    local remedy="${4:-$3}"

    if command -v "$cmd" &>/dev/null; then
        echo "  [skip] $cmd already installed"
        record_skipped "$cmd"
        return 0
    fi

    # Under dry-run nothing is actually installed, so suppress the
    # [install] banner and the "still missing" warning; only show
    # [dry-run] lines.
    if is_dry_run; then
        echo "  [dry-run] $method"
        return 0
    fi

    echo "  [install] $pkg..."
    if eval "$method"; then
        if command -v "$cmd" &>/dev/null; then
            record_installed "$cmd"
        else
            warn "$cmd is still missing, install it with: $remedy"
            record_manual "$cmd|$remedy"
        fi
        return 0
    fi

    warn "Skipped $pkg, install it with: $remedy"
    record_manual "$cmd|$remedy"
    return 0
}

# ── Run Summary ────────────────────────────────────────────────────────────────
install_summary() {
    local install_bin="${1:-}"
    local installed="none"
    local skipped="none"
    local entry

    if [ ${#INSTALL_DONE[@]} -gt 0 ]; then
        installed="${INSTALL_DONE[*]}"
    fi
    if [ ${#INSTALL_SKIPPED[@]} -gt 0 ]; then
        skipped="${INSTALL_SKIPPED[*]}"
    fi

    echo ""
    echo -e "${BOLD}Install summary${NC}"
    echo "  installed: $installed"
    echo "  skipped (already present): $skipped"
    if [ ${#INSTALL_MANUAL[@]} -eq 0 ]; then
        echo "  needs manual action: none"
    else
        for entry in "${INSTALL_MANUAL[@]}"; do
            echo "  needs manual action: ${entry%%|*} -> ${entry#*|}"
        done
    fi
    if [ -n "$install_bin" ]; then
        case ":$PATH:" in
            *":$install_bin:"*) ;;
            *) echo "  PATH: $install_bin is missing, run: export PATH=\"$install_bin:\$PATH\"" ;;
        esac
    fi
    case ":$PATH:" in
        *":$HOME/.cargo/bin:"*) ;;
        *) echo "  PATH: $HOME/.cargo/bin is missing, run: export PATH=\"\$HOME/.cargo/bin:\$PATH\"" ;;
    esac
}

# ── Doc/Agents Copy Helpers ────────────────────────────────────────────────────
copy_docs_to_config() {
    local config_dir="$1"
    local project_root="${2:-$PROJECT_ROOT}"
    local Docs=(
        "ARCHITECTURE.md"
    )
    for DOC in "${Docs[@]}"; do
        SRC="$project_root/$DOC"
        if [ -f "$SRC" ]; then
            if is_dry_run; then
                echo "  [dry-run] cp $SRC $config_dir/$DOC"
            else
                cp "$SRC" "$config_dir/$DOC"
                echo "  $DOC -> $config_dir/$DOC"
            fi
        fi
    done
}

copy_agents_to_config() {
    local config_dir="$1"
    local project_root="${2:-$PROJECT_ROOT}"
    local agents_src="$project_root/.agents"
    local agents_dst="$config_dir/.agents"

    if is_dry_run; then
        # The rm -rf / mkdir -p are destructive; only echo them under dry-run.
        echo "  [dry-run] rm -rf $agents_dst/prompts"
        if [ -d "$agents_src" ]; then
            echo "  [dry-run] mkdir -p $agents_dst/rules"
        fi
        return 0
    fi

    # Prompts are intentionally NOT distributed: `init`'s copy_dir_all would
    # leak the whole .agents/prompts/ tree into target projects.
    # Stale prompt files from older installs are removed on every run.
    rm -rf "$agents_dst/prompts"
    if [ -d "$agents_src" ]; then
        mkdir -p "$agents_dst/rules"
    fi
}

# ── Version Helpers ────────────────────────────────────────────────────────────
get_project_version() {
    local cargo_toml="${PROJECT_ROOT}/Cargo.toml"
    local ver
    ver=$(cargo metadata --no-deps --format-version 1 2>/dev/null | sed -n 's/.*"version":"\([^"]*\)".*/\1/p' | head -1 || true)
    if [ -z "$ver" ]; then
        ver=$(sed -nE 's/^[[:space:]]*version[[:space:]]*=[[:space:]]*"([^"]+)".*/\1/p' "$cargo_toml" | head -1)
    fi
    echo "$ver"
}

# ── Gate Runner ────────────────────────────────────────────────────────────────
ci_step() {
    local name="$1"
    shift
    echo ""
    echo "  === $name ==="
    if eval "$@"; then
        pass "$name"
        return 0
    else
        fail "$name"
        return 1
    fi
}
