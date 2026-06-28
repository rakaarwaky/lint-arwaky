#!/usr/bin/env bash
# Hook Integrity Checker — monitors pre-commit hook for unauthorized changes
# Run with: sudo bash scripts/hook-integrity-check.sh

set -euo pipefail

REPO_ROOT="/home/raka/mcp-arwaky/lint-arwaky"
HOOK_FILE="$REPO_ROOT/.husky/pre-commit"
CHECKSUM_FILE="$REPO_ROOT/.hook-checksum"
LOG_FILE="/var/log/hook-integrity.log"

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m'

log() {
    echo -e "$(date '+%Y-%m-%d %H:%M:%S') $1" | tee -a "$LOG_FILE"
}

get_checksum() {
    sha256sum "$HOOK_FILE" | awk '{print $1}'
}

verify_checksum() {
    local current=$(get_checksum)
    local expected=$(cat "$CHECKSUM_FILE" 2>/dev/null || echo "")

    if [ "$current" = "$expected" ]; then
        return 0
    else
        return 1
    fi
}

save_checksum() {
    get_checksum > "$CHECKSUM_FILE"
    chmod 644 "$CHECKSUM_FILE"
}

# Initialize
if [ ! -f "$CHECKSUM_FILE" ]; then
    log "${YELLOW}Initializing checksum...${NC}"
    save_checksum
    log "${GREEN}Checksum saved: $(cat "$CHECKSUM_FILE")${NC}"
fi

# Verify on startup
if verify_checksum; then
    log "${GREEN}✅ Hook integrity verified on startup${NC}"
else
    log "${RED}❌ CRITICAL: Hook has been modified since last save!${NC}"
    log "${RED}   Expected: $(cat "$CHECKSUM_FILE")${NC}"
    log "${RED}   Current:  $(get_checksum)${NC}"
    log "${YELLOW}   Restoring original hook...${NC}"

    # Restore from git
    cd "$REPO_ROOT"
    git checkout .husky/pre-commit 2>/dev/null || true

    if verify_checksum; then
        log "${GREEN}✅ Hook restored successfully${NC}"
    else
        log "${RED}❌ Failed to restore hook — manual intervention required${NC}"
    fi
fi

# Watch loop (simple polling since inotifywait not available)
log "${YELLOW}Watching hook for changes... (Ctrl+C to stop)${NC}"
log "Monitoring: $HOOK_FILE"

while true; do
    sleep 5

    if ! verify_checksum; then
        log "${RED}$(date '+%Y-%m-%d %H:%M:%S') ❌ Hook modification detected!${NC}"
        log "${RED}   Expected: $(cat "$CHECKSUM_FILE")${NC}"
        log "${RED}   Current:  $(get_checksum)${NC}"

        # Restore
        cd "$REPO_ROOT"
        git checkout .husky/pre-commit 2>/dev/null || true

        if verify_checksum; then
            log "${GREEN}   ✅ Hook auto-restored${NC}"
        else
            log "${RED}   ❌ Auto-restore failed${NC}"
        fi
    fi
done
