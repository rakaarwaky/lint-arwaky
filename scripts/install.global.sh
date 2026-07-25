#!/usr/bin/env bash
# install.global.sh — release build + global system-wide installation
set -euo pipefail

source "$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)/lib.sh"

CARGO_TOML="$PROJECT_ROOT/Cargo.toml"
RELEASE_DIR="$PROJECT_ROOT/target/release"
DIST_DIR="$PROJECT_ROOT/dist"

if [ "$(id -u)" -ne 0 ]; then
    echo "Warning: Global installation typically requires root privileges (e.g. sudo)."
fi

INSTALL_BIN="${LINT_ARWAKY_INSTALL_BIN:-/usr/local/bin}"
CONFIG_DIR="${LINT_ARWAKY_CONFIG_DIR:-/etc/lint-arwaky}"
REPORT_DIR="${LINT_ARWAKY_REPORT_DIR:-/var/lib/lint-arwaky/reports}"

BINARIES=(lint-arwaky-cli lint-arwaky-mcp lint-arwaky-tui)

usage() {
    echo "Usage: bash scripts/install.global.sh [options]"
    echo ""
    echo "Options:"
    echo "  -h, --help    Show this help"
    echo ""
    echo "Env vars:"
    echo "  LINT_ARWAKY_INSTALL_BIN   Install binaries to (default: /usr/local/bin)"
    echo "  LINT_ARWAKY_CONFIG_DIR    Config dir (default: /etc/lint-arwaky)"
    echo "  LINT_ARWAKY_REPORT_DIR    Reports dir (default: /var/lib/lint-arwaky/reports)"
    exit 0
}

while [[ $# -gt 0 ]]; do
    case "$1" in
        -h|--help) usage ;;
        *) die "Unknown option: $1 (use -h for help)" ;;
    esac
done

# 1. Cleanup & Install layout before build
if [ -d "$CONFIG_DIR" ]; then
    echo "Cleaning existing global config dir: $CONFIG_DIR"
    rm -rf "$CONFIG_DIR"
fi
if [ -d "$REPORT_DIR" ]; then
    echo "Cleaning existing global report dir: $REPORT_DIR"
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
RUST_MIN_STACK=33554432 cargo build --release

# 4. Checksums + install
pushd "$RELEASE_DIR" >/dev/null
sha256sum "${BINARIES[@]}" > "$DIST_DIR/SHA256SUMS.txt"
popd >/dev/null

for BIN in "${BINARIES[@]}"; do
    install -m 0755 "$RELEASE_DIR/$BIN" "$INSTALL_BIN/$BIN"
    echo "  -> $INSTALL_BIN/$BIN"
done

# 5. Install docs to global config
copy_docs_to_config "$CONFIG_DIR"

# 6. Copy .agents/ to global config
copy_agents_to_config "$CONFIG_DIR"

CURRENT_VERSION=$(get_project_version)
echo "Done (Global): $CURRENT_VERSION, config=$CONFIG_DIR, reports=$REPORT_DIR"
