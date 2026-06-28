#!/usr/bin/env bash
# security.sh — 24/7 systemd security monitor + pre-commit hook protection
# Install: sudo bash scripts/security.sh
# Status:  sudo bash scripts/security.sh status
# Logs:    sudo bash scripts/security.sh logs
# Stop:    sudo bash scripts/security.sh stop

set -euo pipefail

REPO_ROOT="/home/raka/mcp-arwaky/lint-arwaky"
SERVICE_NAME="lint-arwaky-security"
SERVICE_FILE="/etc/systemd/system/${SERVICE_NAME}.service"
CHECKSUM_FILE="$REPO_ROOT/.security-checksums"
LOG_FILE="/var/log/lint-arwaky-security.log"

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m'

critical_files=(
    ".husky/pre-commit"
    "TEST.md"
    "AGENTS.md"
    "ARCHITECTURE.md"
    "RULES_AES.md"
    "CONTRIBUTING.md"
    "Cargo.toml"
    "lint_arwaky.config.rust.yaml"
    "lint_arwaky.config.python.yaml"
    "lint_arwaky.config.javascript.yaml"
)

save_checksums() {
    > "$CHECKSUM_FILE"
    for f in "${critical_files[@]}"; do
        [ -f "$REPO_ROOT/$f" ] && sha256sum "$REPO_ROOT/$f" >> "$CHECKSUM_FILE"
    done
    chmod 644 "$CHECKSUM_FILE"
}

verify_all() {
    while IFS=' ' read -r expected_hash filepath; do
        [ -z "$filepath" ] && continue
        [ ! -f "$filepath" ] && continue
        current=$(sha256sum "$filepath" | awk '{print $1}')
        if [ "$current" != "$expected_hash" ]; then
            return 1
        fi
    done < "$CHECKSUM_FILE"
    return 0
}

restore_file() {
    cd "$REPO_ROOT"
    git checkout -- "$1" 2>/dev/null || true
    echo "$(date '+%Y-%m-%d %H:%M:%S') RESTORED: $1" >> "$LOG_FILE"
}

daemon_loop() {
    echo "$(date '+%Y-%m-%d %H:%M:%S') Daemon started (PID $$)" >> "$LOG_FILE"
    while true; do
        sleep 3
        if ! verify_all; then
            echo "$(date '+%Y-%m-%d %H:%M:%S') ALERT: file modified — restoring" >> "$LOG_FILE"
            while IFS=' ' read -r expected_hash filepath; do
                [ -z "$filepath" ] && continue
                [ ! -f "$filepath" ] && continue
                current=$(sha256sum "$filepath" | awk '{print $1}')
                if [ "$current" != "$expected_hash" ]; then
                    restore_file "$filepath"
                fi
            done < "$CHECKSUM_FILE"
        fi
        if [ -f "$REPO_ROOT/.husky/pre-commit" ] && [ -f "$REPO_ROOT/.git/hooks/pre-commit" ]; then
            if ! diff -q "$REPO_ROOT/.husky/pre-commit" "$REPO_ROOT/.git/hooks/pre-commit" >/dev/null 2>&1; then
                cp "$REPO_ROOT/.husky/pre-commit" "$REPO_ROOT/.git/hooks/pre-commit"
                chmod +x "$REPO_ROOT/.git/hooks/pre-commit"
            fi
        fi
    done
}

cmd_install() {
    if [ "$EUID" -ne 0 ]; then
        echo -e "${RED}Run with sudo: sudo bash scripts/security.sh${NC}"
        exit 1
    fi
    echo -e "${YELLOW}Saving checksums...${NC}"
    save_checksums
    echo -e "${GREEN}  $(wc -l < "$CHECKSUM_FILE") files tracked${NC}"

    echo -e "${YELLOW}Creating systemd service...${NC}"
    cat > "$SERVICE_FILE" << EOF
[Unit]
Description=Lint Arwaky 24/7 Security Monitor
After=network.target

[Service]
Type=simple
ExecStart=/usr/bin/env bash -c 'REPO_ROOT=$REPO_ROOT exec bash $REPO_ROOT/scripts/security.sh _daemon'
Restart=always
RestartSec=3
StandardOutput=journal
StandardError=journal

[Install]
WantedBy=multi-user.target
EOF

    touch "$LOG_FILE"
    systemctl daemon-reload
    systemctl enable "$SERVICE_NAME"
    systemctl start "$SERVICE_NAME"

    if systemctl is-active --quiet "$SERVICE_NAME"; then
        echo -e "${GREEN}✅ Security ACTIVE — auto-starts on boot${NC}"
    else
        echo -e "${RED}❌ Failed${NC}"
        journalctl -u "$SERVICE_NAME" --no-pager -n 5
        exit 1
    fi
    echo "  Status:  sudo bash scripts/security.sh status"
    echo "  Logs:    sudo bash scripts/security.sh logs"
    echo "  Stop:    sudo bash scripts/security.sh stop"
}

case "${1:-install}" in
    install)   cmd_install ;;
    status)    systemctl status "$SERVICE_NAME" --no-pager 2>/dev/null || echo "Not running" ;;
    logs)      journalctl -u "$SERVICE_NAME" -f --no-pager ;;
    stop)      sudo systemctl stop "$SERVICE_NAME" && echo "Stopped" ;;
    start)     sudo systemctl start "$SERVICE_NAME" && echo "Started" ;;
    _daemon)   daemon_loop ;;
    *)         echo "Usage: $0 {install|status|logs|stop|start}" ;;
esac
