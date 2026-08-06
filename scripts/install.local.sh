#!/usr/bin/env bash
# install.local.sh — release build + local user installation (XDG user layout)
set -euo pipefail

source "$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)/lib.sh"

CARGO_TOML="$PROJECT_ROOT/Cargo.toml"
RELEASE_DIR="$PROJECT_ROOT/target/release"
DIST_DIR="$PROJECT_ROOT/dist"

INSTALL_BIN="${LINT_ARWAKY_INSTALL_BIN:-$HOME/.cargo/bin}"
CONFIG_DIR="${LINT_ARWAKY_CONFIG_DIR:-$HOME/.config/lint-arwaky}"
REPORT_DIR="${LINT_ARWAKY_REPORT_DIR:-$HOME/.local/share/lint-arwaky/reports}"

BINARIES=(lint-arwaky-cli lint-arwaky-mcp lint-arwaky-tui)

usage() {
    echo "Usage: bash scripts/install.local.sh [options]"
    echo ""
    echo "Options:"
    echo "  -h, --help    Show this help"
    echo ""
    echo "Env vars:"
    echo "  LINT_ARWAKY_INSTALL_BIN   Install binaries to (default: ~/.cargo/bin)"
    echo "  LINT_ARWAKY_CONFIG_DIR    Config dir (default: ~/.config/lint-arwaky)"
    echo "  LINT_ARWAKY_REPORT_DIR    Reports dir (default: ~/.local/share/lint-arwaky/reports)"
    exit 0
}

while [[ $# -gt 0 ]]; do
    case "$1" in
        -h|--help) usage ;;
        *) die "Unknown option: $1 (use -h for help)" ;;
    esac
done

# 1. Cleanup & Install XDG layout before build
if [ -d "$CONFIG_DIR" ]; then
    echo "Cleaning existing XDG config dir: $CONFIG_DIR"
    rm -rf "$CONFIG_DIR"
fi
if [ -d "$REPORT_DIR" ]; then
    echo "Cleaning existing XDG report dir: $REPORT_DIR"
    rm -rf "$REPORT_DIR"
fi

mkdir -p "$CONFIG_DIR/rules" "$REPORT_DIR" "$DIST_DIR" "$INSTALL_BIN"

# 2. Install external dependencies (skip if already present)
echo "==> Checking external dependencies..."
detect_pkg_mgr

# Node/npm first, then eslint/tsc depend on it
install_if_missing cargo "Rust/Cargo" "curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y && . \"\$HOME/.cargo/env\""
install_if_missing npm "npm" "npm_install"
install_if_missing eslint "eslint" "npm_install_global eslint"
install_if_missing tsc "typescript" "npm_install_global typescript"
install_if_missing mypy "mypy" "pip_install mypy"
install_if_missing ruff "ruff" "pip_install ruff"
install_if_missing bandit "bandit" "pip_install bandit"

echo "==> External dependency check done."

# 3. Build (increase stack size to prevent LLVM SIGSEGV during LTO)
# Use nightly toolchain to avoid rustc stable ICE in release MIR optimization.
# Mold linker disabled in .cargo/config.toml — causes ICE in rustc 1.97.1 stable.
CARGO_INCREMENTAL=0 RUST_MIN_STACK=33554432 cargo +nightly build --release 2>&1

# 4. Checksums + install
pushd "$RELEASE_DIR" >/dev/null
sha256sum "${BINARIES[@]}" > "$DIST_DIR/SHA256SUMS.txt"
popd >/dev/null

for BIN in "${BINARIES[@]}"; do
    install -m 0755 "$RELEASE_DIR/$BIN" "$INSTALL_BIN/$BIN"
    echo "  -> $INSTALL_BIN/$BIN"
done

# 5. Install docs to XDG config
copy_docs_to_config "$CONFIG_DIR"

# 6. Copy .agents/ to XDG config
copy_agents_to_config "$CONFIG_DIR"

# 7. Setup shell aliases (lac, lat, lam) if missing
for RC_FILE in "$HOME/.bashrc" "$HOME/.zshrc"; do
    if [ -f "$RC_FILE" ] && ! grep -q "alias lac=" "$RC_FILE"; then
        echo "" >> "$RC_FILE"
        echo "# Lint Arwaky Aliases" >> "$RC_FILE"
        echo "alias lac=\"lint-arwaky-cli\"" >> "$RC_FILE"
        echo "alias lat=\"lint-arwaky-tui\"" >> "$RC_FILE"
        echo "alias lam=\"lint-arwaky-mcp\"" >> "$RC_FILE"
        echo "  -> Shell aliases added to $RC_FILE"
    fi
done

CURRENT_VERSION=$(get_project_version)
echo "Done (Local): $CURRENT_VERSION, config=$CONFIG_DIR, reports=$REPORT_DIR"
