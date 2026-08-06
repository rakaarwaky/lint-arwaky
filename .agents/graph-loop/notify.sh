#!/bin/bash
# Notification System for Graph Loop Pipeline (Revision 2.0)
# FIX: Human alert mechanism added (webhook + desktop notification)
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
CONFIG_READER="${SCRIPT_DIR}/config_reader.sh"
LOG_FILE="${SCRIPT_DIR}/execution.log"
NOTIFY_LOG="${SCRIPT_DIR}/notifications.log"

# ── Log Notification ────────────────────────────────────────────────
log_notification() {
    local type=$1
    local message=$2
    local timestamp=$(date -Iseconds)
    echo "[$timestamp] [$type] $message" >> "$NOTIFY_LOG"
}

# ── Notify via Log ──────────────────────────────────────────────────
notify_log() {
    local event=$1
    local message=$2
    local timestamp=$(date -Iseconds)
    echo "[$timestamp] [$event] $message" >> "$LOG_FILE"
    log_notification "log" "$event: $message"
}

# ── Notify via PR Comment ───────────────────────────────────────────
notify_pr_comment() {
    local pr_number=$1
    local message=$2

    if [ -z "$pr_number" ]; then
        log_notification "error" "No PR number provided"
        return 1
    fi

    gh pr comment "$pr_number" --body "$message" 2>/dev/null
    log_notification "pr_comment" "PR #$pr_number: $message"
}

# ── Notify via Desktop Notification ─────────────────────────────────
# FIX: Desktop notification for human alerts
notify_desktop() {
    local title=$1
    local message=$2

    if command -v notify-send &> /dev/null; then
        notify-send "$title" "$message" 2>/dev/null || true
        log_notification "desktop" "$title: $message"
    fi
}

# ── Notify via Webhook (Slack/Discord) ──────────────────────────────
# FIX: Webhook for human alerts
notify_webhook() {
    local message=$1

    local webhook_url
    webhook_url=$("$CONFIG_READER" read ".notifications.channels[] | select(.type == \"webhook\") | .url" "" 2>/dev/null || echo "")

    local webhook_enabled
    webhook_enabled=$("$CONFIG_READER" read ".notifications.channels[] | select(.type == \"webhook\") | .enabled" "false" 2>/dev/null || echo "false")

    if [ "$webhook_enabled" = "true" ] && [ -n "$webhook_url" ]; then
        curl -s -X POST "$webhook_url" \
            -H "Content-Type: application/json" \
            -d "{\"text\": \"$message\"}" 2>/dev/null || true
        log_notification "webhook" "$message"
    fi
}

# ── Human Alert (combined) ──────────────────────────────────────────
# FIX: Human alert mechanism for escalation/timeout/error
notify_human_alert() {
    local severity=$1
    local message=$2

    notify_log "$severity" "$message"
    notify_desktop "Graph Loop [$severity]" "$message"
    notify_webhook "[$severity] $message"
}

# ── Notify Pipeline Started ─────────────────────────────────────────
notify_pipeline_started() {
    local feature=$1
    local pipeline_id=$2
    notify_log "pipeline_started" "Pipeline $pipeline_id started for feature: $feature"
}

# ── Notify Node Completed ───────────────────────────────────────────
notify_node_completed() {
    local node=$1
    local feature=$2
    local duration=$3
    notify_log "node_completed" "$node completed for $feature in ${duration}m"
}

# ── Notify Pipeline Completed ───────────────────────────────────────
notify_pipeline_completed() {
    local feature=$1
    local pipeline_id=$2
    local pr_number=$3

    notify_log "pipeline_completed" "Pipeline $pipeline_id completed for feature: $feature"

    if [ -n "$pr_number" ]; then
        notify_pr_comment "$pr_number" "✅ Pipeline completed for feature: $feature"
    fi
}

# ── Notify Quality-Analysis Approved ────────────────────────────────
notify_qa_approved() {
    local feature=$1
    local pr_number=$2

    notify_log "qa_approved" "Quality-Analysis approved PR #$pr_number for feature: $feature"

    if [ -n "$pr_number" ]; then
        notify_pr_comment "$pr_number" "✅ Quality-Analysis APPROVED — PR merged successfully"
    fi
}

# ── Notify Quality-Analysis Rejected ────────────────────────────────
notify_qa_rejected() {
    local feature=$1
    local pr_number=$2
    local reason=$3

    notify_log "qa_rejected" "Quality-Analysis rejected PR #$pr_number for feature: $feature — $reason"

    if [ -n "$pr_number" ]; then
        notify_pr_comment "$pr_number" "❌ Quality-Analysis REJECTED — $reason"
    fi
}

# ── Notify Timeout ──────────────────────────────────────────────────
notify_timeout() {
    local node=$1
    local feature=$2
    local timeout_minutes=$3

    notify_log "timeout" "$node timed out after ${timeout_minutes}m for feature: $feature"
    notify_human_alert "TIMEOUT" "$node timed out after ${timeout_minutes}m for feature: $feature"
}

# ── Notify Error ────────────────────────────────────────────────────
notify_error() {
    local component=$1
    local error=$2
    local feature=${3:-"unknown"}

    notify_log "error" "Error in $component for feature: $feature — $error"
    notify_human_alert "ERROR" "[$component] $feature: $error"
}

# ── Main ────────────────────────────────────────────────────────────
main() {
    local command=${1:-}
    shift || true

    case $command in
        log)                notify_log "$1" "$2" ;;
        pr-comment)         notify_pr_comment "$1" "$2" ;;
        desktop)            notify_desktop "$1" "$2" ;;
        webhook)            notify_webhook "$1" ;;
        human-alert)        notify_human_alert "$1" "$2" ;;
        pipeline-started)   notify_pipeline_started "$1" "$2" ;;
        node-completed)     notify_node_completed "$1" "$2" "$3" ;;
        pipeline-completed) notify_pipeline_completed "$1" "$2" "${3:-}" ;;
        qa-approved)        notify_qa_approved "$1" "$2" ;;
        qa-rejected)        notify_qa_rejected "$1" "$2" "$3" ;;
        timeout)            notify_timeout "$1" "$2" "$3" ;;
        error)              notify_error "$1" "$2" "${3:-}" ;;
        *)
            echo "Usage: notify.sh {log|pr-comment|desktop|webhook|human-alert|pipeline-started|node-completed|pipeline-completed|qa-approved|qa-rejected|timeout|error}"
            exit 1
            ;;
    esac
}

main "$@"
