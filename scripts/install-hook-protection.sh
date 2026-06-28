#!/usr/bin/env bash
# Install hook integrity protection — requires sudo
# This creates a systemd service that monitors the pre-commit hook

set -euo pipefail

REPO_ROOT="/home/raka/mcp-arwaky/lint-arwaky"
SERVICE_NAME="hook-integrity-lint-arwaky"
SERVICE_FILE="/etc/systemd/system/${SERVICE_NAME}.service"
CHECKER_SCRIPT="$REPO_ROOT/scripts/hook-integrity-check.sh"
CHECKSUM_FILE="$REPO_ROOT/.hook-checksum"

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m'

echo -e "${YELLOW}Hook Integrity Protection Installer${NC}"
echo "=================================="

# Check if running as root
if [ "$EUID" -ne 0 ]; then
    echo -e "${RED}Error: Run with sudo${NC}"
    echo "Usage: sudo bash scripts/install-hook-protection.sh"
    exit 1
fi

# Step 1: Save current checksum
echo -e "${YELLOW}Step 1: Saving hook checksum...${NC}"
sha256sum "$REPO_ROOT/.husky/pre-commit" | awk '{print $1}' > "$CHECKSUM_FILE"
chmod 644 "$CHECKSUM_FILE"
echo -e "${GREEN}  Checksum: $(cat "$CHECKSUM_FILE")${NC}"

# Step 2: Make checker executable
echo -e "${YELLOW}Step 2: Making checker script executable...${NC}"
chmod +x "$CHECKER_SCRIPT"
echo -e "${GREEN}  Done${NC}"

# Step 3: Create systemd service
echo -e "${YELLOW}Step 3: Creating systemd service...${NC}"
cat > "$SERVICE_FILE" << EOF
[Unit]
Description=Hook Integrity Monitor for lint-arwaky
After=network.target

[Service]
Type=simple
ExecStart=/usr/bin/env bash $CHECKER_SCRIPT
Restart=always
RestartSec=10
StandardOutput=journal
StandardError=journal

[Install]
WantedBy=multi-user.target
EOF

echo -e "${GREEN}  Service created: $SERVICE_FILE${NC}"

# Step 4: Enable and start service
echo -e "${YELLOW}Step 4: Enabling and starting service...${NC}"
systemctl daemon-reload
systemctl enable "$SERVICE_NAME"
systemctl start "$SERVICE_NAME"

echo -e "${GREEN}  Service started${NC}"

# Step 5: Verify
echo -e "${YELLOW}Step 5: Verifying...${NC}"
if systemctl is-active --quiet "$SERVICE_NAME"; then
    echo -e "${GREEN}✅ Hook integrity protection is ACTIVE${NC}"
else
    echo -e "${RED}❌ Service failed to start${NC}"
    journalctl -u "$SERVICE_NAME" --no-pager -n 10
fi

echo ""
echo "Commands:"
echo "  sudo systemctl status $SERVICE_NAME   # Check status"
echo "  sudo systemctl stop $SERVICE_NAME      # Stop monitoring"
echo "  sudo systemctl restart $SERVICE_NAME   # Restart monitoring"
echo "  sudo journalctl -u $SERVICE_NAME -f    # View logs"
