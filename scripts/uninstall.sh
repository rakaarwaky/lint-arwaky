#!/usr/bin/env bash
# uninstall.sh — Remove lint-arwaky installed files
set -euo pipefail

source "$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)/lib.sh"

BINARIES=(lint-arwaky-cli lint-arwaky-mcp lint-arwaky-tui)

usage() {
    echo "Usage: bash scripts/uninstall.sh [options]"
    echo ""
    echo "Removes installed binaries, config, and reports."
    echo ""
    echo "Options:"
    echo "  --local       Uninstall local (XDG) installation (default)"
    echo "  --global      Uninstall global system-wide installation"
    echo "  --all         Uninstall both local and global"
    echo "  --dry-run     Show what would be removed without removing"
    echo "  -h, --help    Show this help"
    exit 0
}

MODE="local"
DRY_RUN=false

while [[ $# -gt 0 ]]; do
    case "$1" in
        --local)   MODE="local"; shift ;;
        --global)  MODE="global"; shift ;;
        --all)     MODE="all"; shift ;;
        --dry-run) DRY_RUN=true; shift ;;
        -h|--help) usage ;;
        *) die "Unknown option: $1 (use -h for help)" ;;
    esac
done

remove_file() {
    local path="$1"
    if [ -f "$path" ] || [ -d "$path" ]; then
        if $DRY_RUN; then
            echo "  [dry-run] Would remove: $path"
        else
            rm -rf "$path"
            echo "  [removed] $path"
        fi
    fi
}

remove_binary() {
    local name="$1"
    local dir="$2"
    remove_file "$dir/$name"
}

echo -e "${BOLD}lint-arwaky uninstaller${NC}"
echo ""

# ── Local uninstall ────────────────────────────────────────────────────────────
uninstall_local() {
    echo "=== Local (XDG) Installation ==="
    local install_bin="${LINT_ARWAKY_INSTALL_BIN:-${XDG_BIN_HOME:-$HOME/.local/bin}}"
    local config_dir="${LINT_ARWAKY_CONFIG_DIR:-${XDG_CONFIG_HOME:-$HOME/.config}/lint-arwaky}"
    local report_dir="${LINT_ARWAKY_REPORT_DIR:-${XDG_DATA_HOME:-$HOME/.local/share}/lint-arwaky/reports}"

    echo "Binaries ($install_bin):"
    for bin in "${BINARIES[@]}"; do
        remove_binary "$bin" "$install_bin"
        if [ "$install_bin" != "$HOME/.cargo/bin" ] && [ -f "$HOME/.cargo/bin/$bin" ]; then
            remove_binary "$bin" "$HOME/.cargo/bin"
        fi
    done

    echo ""
    echo "Config ($config_dir):"
    remove_file "$config_dir"

    echo ""
    echo "Reports ($report_dir):"
    remove_file "$report_dir"
}

# ── Global uninstall ───────────────────────────────────────────────────────────
uninstall_global() {
    echo "=== Global Installation ==="
    local install_bin="${LINT_ARWAKY_INSTALL_BIN:-/usr/local/bin}"
    local config_dir="${LINT_ARWAKY_CONFIG_DIR:-/etc/lint-arwaky}"
    local report_dir="${LINT_ARWAKY_REPORT_DIR:-/var/lib/lint-arwaky/reports}"

    if [ "$(id -u)" -ne 0 ]; then
        warn "Global uninstall requires root. Re-running with sudo..."
        sudo bash "$0" --global ${DRY_RUN:+--dry-run}
        return
    fi

    echo "Binaries ($install_bin):"
    for bin in "${BINARIES[@]}"; do
        remove_binary "$bin" "$install_bin"
    done

    echo ""
    echo "Config ($config_dir):"
    remove_file "$config_dir"

    echo ""
    echo "Reports ($report_dir):"
    remove_file "$report_dir"

    echo ""
    echo "Systemd service:"
    if systemctl is-active --quiet lint-arwaky-security 2>/dev/null; then
        if $DRY_RUN; then
            echo "  [dry-run] Would stop and disable lint-arwaky-security"
        else
            sudo systemctl stop lint-arwaky-security
            sudo systemctl disable lint-arwaky-security
            sudo rm -f /etc/systemd/system/lint-arwaky-security.service
            sudo systemctl daemon-reload
            echo "  [removed] lint-arwaky-security service"
        fi
    fi
}

# ── Git hooks ──────────────────────────────────────────────────────────────────
uninstall_hooks() {
    echo ""
    echo "Git hooks:"
    local hooks_dir="$PROJECT_ROOT/.git/hooks"
    if [ -f "$hooks_dir/pre-commit" ]; then
        if grep -q "lint-arwaky" "$hooks_dir/pre-commit" 2>/dev/null; then
            if $DRY_RUN; then
                echo "  [dry-run] Would remove: $hooks_dir/pre-commit"
            else
                rm -f "$hooks_dir/pre-commit"
                echo "  [removed] $hooks_dir/pre-commit"
            fi
        fi
    fi
}

case "$MODE" in
    local)  uninstall_local ;;
    global) uninstall_global ;;
    all)    uninstall_local; echo ""; uninstall_global ;;
esac

uninstall_hooks

echo ""
if $DRY_RUN; then
    echo -e "${YELLOW}Dry run complete. No files were removed.${NC}"
else
    echo -e "${GREEN}Uninstall complete.${NC}"
fi
