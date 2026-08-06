#!/bin/bash
# Feature Manager for Graph Loop Pipeline (Revision 2.0)
# Handles feature claiming, releasing, dedup checks, and priority sorting
# FIX: Select next feature sorted by priority
# FIX: Sync features from config.yaml
# FIX: Idempotency includes DONE status check
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
CONFIG_READER="${SCRIPT_DIR}/config_reader.sh"
FEATURES_FILE="${SCRIPT_DIR}/features.json"
LOCKS_DIR="${SCRIPT_DIR}/locks"
LOG_FILE="${SCRIPT_DIR}/execution.log"

# ── Initialize Features File ────────────────────────────────────────
init_features_file() {
    if [ ! -f "$FEATURES_FILE" ]; then
        cat > "$FEATURES_FILE" << 'EOF'
{
  "version": "2.0",
  "features": {},
  "dedup_rules": {
    "cooldown_minutes": 60
  }
}
EOF
        log_event "init" "Features file created"
    fi
    mkdir -p "$LOCKS_DIR"
}

# ── Sync Features from Config ───────────────────────────────────────
# FIX: Populate features.json from config.yaml
sync_features_from_config() {
    local features_data
    features_data=$("$CONFIG_READER" features 2>/dev/null || echo "")

    if [ -z "$features_data" ]; then
        log_event "sync" "No features found in config"
        return 1
    fi

    while IFS='|' read -r priority name path description; do
        if ! feature_exists "$name"; then
            jq --arg name "$name" --arg path "$path" --argjson priority "$priority" --arg desc "$description" \
                '.features[$name] = {
                    "status": "PENDING",
                    "priority": $priority,
                    "path": $path,
                    "description": $desc,
                    "pipeline_id": null,
                    "claimed_at": null,
                    "locked_by": null,
                    "iteration": 0,
                    "history": []
                }' "$FEATURES_FILE" > "${FEATURES_FILE}.tmp" && mv "${FEATURES_FILE}.tmp" "$FEATURES_FILE"
            log_event "sync" "Feature synced from config: $name (priority $priority)"
        fi
    done <<< "$features_data"
}

# ── Check if Feature Exists ─────────────────────────────────────────
feature_exists() {
    local feature=$1
    jq -e ".features[\"$feature\"]" "$FEATURES_FILE" > /dev/null 2>&1
}

# ── Get Feature Status ──────────────────────────────────────────────
get_feature_status() {
    local feature=$1
    if ! feature_exists "$feature"; then
        echo "NOT_FOUND"
        return
    fi
    jq -r ".features[\"$feature\"].status" "$FEATURES_FILE"
}

# ── Get Feature Path ────────────────────────────────────────────────
get_feature_path() {
    local feature=$1
    if ! feature_exists "$feature"; then
        echo ""
        return
    fi
    jq -r ".features[\"$feature\"].path" "$FEATURES_FILE"
}

# ── Check Cooldown ──────────────────────────────────────────────────
check_cooldown() {
    local feature=$1
    local cooldown_minutes
    cooldown_minutes=$(jq -r '.dedup_rules.cooldown_minutes' "$FEATURES_FILE")

    if ! feature_exists "$feature"; then
        return 0
    fi

    local last_completed
    last_completed=$(jq -r ".features[\"$feature\"].history | map(select(.status == \"DONE\")) | last | .completed_at // empty" "$FEATURES_FILE")

    if [ -z "$last_completed" ]; then
        return 0
    fi

    local cooldown_until
    cooldown_until=$(date -d "$last_completed + ${cooldown_minutes} minutes" +%s 2>/dev/null || echo "0")
    local now=$(date +%s)

    if [ "$now" -lt "$cooldown_until" ]; then
        local remaining=$(( (cooldown_until - now) / 60 ))
        log_event "cooldown" "Feature $feature in cooldown (${remaining}m remaining)"
        return 1
    fi
    return 0
}

# ── Check Lock ──────────────────────────────────────────────────────
check_lock() {
    local feature=$1
    local lock_file="${LOCKS_DIR}/${feature}.lock"

    if [ -f "$lock_file" ]; then
        local locked_by locked_at
        locked_by=$(jq -r '.locked_by' "$lock_file" 2>/dev/null || echo "unknown")
        locked_at=$(jq -r '.locked_at' "$lock_file" 2>/dev/null || echo "unknown")
        log_event "locked" "Feature $feature is locked by $locked_by since $locked_at"
        return 1
    fi
    return 0
}

# ── Claim Feature ───────────────────────────────────────────────────
claim_feature() {
    local feature=$1
    local feature_path=$2
    local pipeline_id=${3:-"manual"}
    local timestamp=$(date -Iseconds)

    # Check if claimable
    local status
    status=$(get_feature_status "$feature")

    if [ "$status" = "LOCKED" ] || [ "$status" = "ACTIVE" ]; then
        log_event "claim_failed" "Feature $feature is $status — cannot claim"
        return 1
    fi

    # FIX: Also check DONE (idempotency)
    if [ "$status" = "DONE" ]; then
        if ! check_cooldown "$feature"; then
            return 1
        fi
    fi

    if ! check_cooldown "$feature"; then
        return 1
    fi

    if ! check_lock "$feature"; then
        return 1
    fi

    # Create lock file
    echo "{\"locked_by\": \"graph-engine\", \"locked_at\": \"$timestamp\"}" > "${LOCKS_DIR}/${feature}.lock"

    # Update features.json
    if ! feature_exists "$feature"; then
        jq --arg feature "$feature" --arg path "$feature_path" \
           --arg pid "$pipeline_id" --arg ts "$timestamp" \
            '.features[$feature] = {
                "status": "LOCKED",
                "pipeline_id": $pid,
                "path": $path,
                "claimed_at": $ts,
                "locked_by": "graph-engine",
                "iteration": 0,
                "history": []
            }' "$FEATURES_FILE" > "${FEATURES_FILE}.tmp" && mv "${FEATURES_FILE}.tmp" "$FEATURES_FILE"
    else
        jq --arg feature "$feature" --arg pid "$pipeline_id" --arg ts "$timestamp" \
            '.features[$feature].status = "LOCKED" |
             .features[$feature].pipeline_id = $pid |
             .features[$feature].claimed_at = $ts |
             .features[$feature].locked_by = "graph-engine"' "$FEATURES_FILE" > "${FEATURES_FILE}.tmp" && mv "${FEATURES_FILE}.tmp" "$FEATURES_FILE"
    fi

    log_event "claimed" "Feature $feature claimed for pipeline $pipeline_id"
    return 0
}

# ── Activate Feature ────────────────────────────────────────────────
activate_feature() {
    local feature=$1
    local timestamp=$(date -Iseconds)

    jq --arg feature "$feature" --arg ts "$timestamp" \
        '.features[$feature].status = "ACTIVE"' "$FEATURES_FILE" > "${FEATURES_FILE}.tmp" && mv "${FEATURES_FILE}.tmp" "$FEATURES_FILE"

    log_event "activated" "Feature $feature activated"
}

# ── Complete Feature ────────────────────────────────────────────────
complete_feature() {
    local feature=$1
    local pipeline_id=$2
    local timestamp=$(date -Iseconds)

    jq --arg feature "$feature" --arg pid "$pipeline_id" --arg ts "$timestamp" \
        '.features[$feature].status = "DONE" |
         .features[$feature].history += [{
            "status": "DONE",
            "pipeline_id": $pid,
            "completed_at": $ts
        }]' "$FEATURES_FILE" > "${FEATURES_FILE}.tmp" && mv "${FEATURES_FILE}.tmp" "$FEATURES_FILE"

    release_lock "$feature"
    log_event "completed" "Feature $feature completed (pipeline $pipeline_id)"
}

# ── Release Lock ────────────────────────────────────────────────────
release_lock() {
    local feature=$1
    local lock_file="${LOCKS_DIR}/${feature}.lock"

    if [ -f "$lock_file" ]; then
        rm -f "$lock_file"
        log_event "lock_released" "Lock released for feature $feature"
    fi
}

# ── Fail Feature ────────────────────────────────────────────────────
fail_feature() {
    local feature=$1
    local reason=$2
    local timestamp=$(date -Iseconds)

    jq --arg feature "$feature" --arg reason "$reason" --arg ts "$timestamp" \
        '.features[$feature].status = "FAILED" |
         .features[$feature].history += [{
            "status": "FAILED",
            "reason": $reason,
            "failed_at": $ts
        }]' "$FEATURES_FILE" > "${FEATURES_FILE}.tmp" && mv "${FEATURES_FILE}.tmp" "$FEATURES_FILE"

    release_lock "$feature"
    log_event "failed" "Feature $feature failed: $reason"
}

# ── Select Next Feature (sorted by priority) ───────────────────────
# FIX: Sort by priority before selection
select_next_feature() {
    local pending
    pending=$(jq -r '.features | to_entries[] | select(.value.status == "PENDING") | "\(.value.priority)|\(.key)"' "$FEATURES_FILE" 2>/dev/null | sort -t'|' -k1 -n)

    if [ -z "$pending" ]; then
        log_event "select" "No PENDING features found"
        return 1
    fi

    while IFS='|' read -r _priority feature; do
        if check_cooldown "$feature" && check_lock "$feature"; then
            echo "$feature"
            return 0
        fi
    done <<< "$pending"

    log_event "select" "No claimable PENDING features"
    return 1
}

# ── Log Event ───────────────────────────────────────────────────────
log_event() {
    local event_type=$1
    local message=$2
    local timestamp=$(date -Iseconds)
    echo "[$timestamp] [feature:$event_type] $message" >> "$LOG_FILE"
}

# ── Main ────────────────────────────────────────────────────────────
main() {
    init_features_file

    local command=${1:-}
    shift || true

    case $command in
        sync)           sync_features_from_config ;;
        status)         get_feature_status "$1" ;;
        path)           get_feature_path "$1" ;;
        check-cooldown) check_cooldown "$1" && echo "OK" || echo "COOLDOWN" ;;
        claim)          claim_feature "$1" "$2" "${3:-manual}" ;;
        activate)       activate_feature "$1" ;;
        complete)       complete_feature "$1" "$2" ;;
        fail)           fail_feature "$1" "$2" ;;
        release)        release_lock "$1" ;;
        select)         select_next_feature ;;
        *)
            echo "Usage: feature_manager.sh {sync|status|path|check-cooldown|claim|activate|complete|fail|release|select}"
            exit 1
            ;;
    esac
}

main "$@"
