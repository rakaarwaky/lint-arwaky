#!/bin/bash
# Health Check for Graph Loop Pipeline (Revision 2.0)
# FIX: Checks actual agent processes (not just PID file)
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
STATE_FILE="${SCRIPT_DIR}/state.json"
FEATURES_FILE="${SCRIPT_DIR}/features.json"
LOG_FILE="${SCRIPT_DIR}/execution.log"
HEALTH_LOG="${SCRIPT_DIR}/health.log"
RESULTS_DIR="${SCRIPT_DIR}/results"

# ── Log Health Event ────────────────────────────────────────────────
log_health() {
    local status=$1
    local message=$2
    local timestamp=$(date -Iseconds)
    echo "[$timestamp] [$status] $message" >> "$HEALTH_LOG"
}

# ── Check Engine Process ────────────────────────────────────────────
check_engine_process() {
    local pid_file="${SCRIPT_DIR}/engine.pid"

    if [ -f "$pid_file" ]; then
        local pid
        pid=$(cat "$pid_file")
        if kill -0 "$pid" 2>/dev/null; then
            log_health "OK" "Engine process running (PID: $pid)"
            return 0
        else
            log_health "WARN" "Engine process not running (stale PID: $pid)"
            return 1
        fi
    else
        # FIX: Also check systemd service
        local service_status
        service_status=$(systemctl is-active graph-loop.service 2>/dev/null || echo "inactive")
        if [ "$service_status" = "active" ]; then
            log_health "OK" "Engine running via systemd"
            return 0
        fi
        log_health "WARN" "No engine PID file or active service found"
        return 1
    fi
}

# ── Check Agent Processes ───────────────────────────────────────────
# FIX: Verify actual qwen agent processes
check_agent_processes() {
    local running_agents
    running_agents=$(pgrep -f "qwen -p" 2>/dev/null | wc -l)

    if [ "$running_agents" -gt 0 ]; then
        log_health "OK" "Active agent processes: $running_agents"
    else
        log_health "INFO" "No active agent processes"
    fi

    # Check PID files in results dir
    if [ -d "$RESULTS_DIR" ]; then
        local pid_files
        pid_files=$(find "$RESULTS_DIR" -name "*.pid" 2>/dev/null | wc -l)
        if [ "$pid_files" -gt 0 ]; then
            local stale_pids=0
            for pf in "$RESULTS_DIR"/*.pid; do
                local pid
                pid=$(cat "$pf" 2>/dev/null || echo "0")
                if ! kill -0 "$pid" 2>/dev/null; then
                    stale_pids=$((stale_pids + 1))
                fi
            done
            if [ "$stale_pids" -gt 0 ]; then
                log_health "WARN" "Stale agent PID files: $stale_pids"
                return 1
            fi
        fi
    fi

    return 0
}

# ── Check State File ────────────────────────────────────────────────
check_state_file() {
    if [ ! -f "$STATE_FILE" ]; then
        log_health "ERROR" "State file not found: $STATE_FILE"
        return 1
    fi

    local current_state
    current_state=$(jq -r '.pipeline.current_state' "$STATE_FILE")

    case "$current_state" in
        IDLE|MERGED)
            log_health "OK" "State: $current_state (healthy)"
            return 0
            ;;
        ANALYZING|ARCHITECT|DEVELOPER|QUALITY-ANALYSIS|DISPATCHING)
            local started_at
            started_at=$(jq -r '.pipeline.started_at' "$STATE_FILE")
            if [ "$started_at" != "null" ] && [ -n "$started_at" ]; then
                local started_epoch
                started_epoch=$(date -d "$started_at" +%s 2>/dev/null || echo "0")
                local now=$(date +%s)
                local elapsed_hours=$(( (now - started_epoch) / 3600 ))
                if [ "$elapsed_hours" -ge 2 ]; then
                    log_health "WARN" "State $current_state for ${elapsed_hours}h (may be stuck)"
                    return 1
                fi
            fi
            log_health "OK" "State: $current_state (active)"
            return 0
            ;;
        WAITING_HUMAN|ESCALATED)
            log_health "WARN" "State: $current_state (requires attention)"
            return 1
            ;;
        FAILED|BLOCKED|TIMEOUT)
            log_health "ERROR" "State: $current_state (failure)"
            return 1
            ;;
        *)
            log_health "ERROR" "Unknown state: $current_state"
            return 1
            ;;
    esac
}

# ── Check Disk Space ────────────────────────────────────────────────
check_disk_space() {
    local usage
    usage=$(df -h "$SCRIPT_DIR" | awk 'NR==2 {print $5}' | tr -d '%')

    if [ "$usage" -ge 90 ]; then
        log_health "ERROR" "Disk usage critical: ${usage}%"
        return 1
    elif [ "$usage" -ge 80 ]; then
        log_health "WARN" "Disk usage high: ${usage}%"
        return 1
    fi

    log_health "OK" "Disk usage: ${usage}%"
    return 0
}

# ── Check Log File Size ─────────────────────────────────────────────
check_log_size() {
    if [ ! -f "$LOG_FILE" ]; then
        return 0
    fi

    local size_kb
    size_kb=$(du -k "$LOG_FILE" | cut -f1)

    if [ "$size_kb" -ge 10240 ]; then
        log_health "WARN" "Log file large: ${size_kb}KB (consider rotation)"
        return 1
    fi

    log_health "OK" "Log file size: ${size_kb}KB"
    return 0
}

# ── Check Lock Files ────────────────────────────────────────────────
check_lock_files() {
    local locks_dir="${SCRIPT_DIR}/locks"

    if [ ! -d "$locks_dir" ]; then
        return 0
    fi

    local lock_count
    lock_count=$(find "$locks_dir" -name "*.lock" 2>/dev/null | wc -l)

    if [ "$lock_count" -gt 1 ]; then
        log_health "WARN" "Multiple lock files: $lock_count (expected: 0-1)"
        return 1
    fi

    log_health "OK" "Lock files: $lock_count"
    return 0
}

# ── Generate Health Report ──────────────────────────────────────────
generate_report() {
    echo "=== Graph Loop Health Report (v2.0) ==="
    echo "Timestamp: $(date -Iseconds)"
    echo ""

    local issues=0

    check_engine_process || issues=$((issues + 1))
    check_agent_processes || issues=$((issues + 1))
    check_state_file || issues=$((issues + 1))
    check_disk_space || issues=$((issues + 1))
    check_log_size || issues=$((issues + 1))
    check_lock_files || issues=$((issues + 1))

    echo ""
    if [ "$issues" -eq 0 ]; then
        echo "Status: HEALTHY"
    elif [ "$issues" -le 2 ]; then
        echo "Status: WARNING ($issues issues)"
    else
        echo "Status: CRITICAL ($issues issues)"
    fi
}

# ── Main ────────────────────────────────────────────────────────────
main() {
    local command=${1:-report}
    shift || true

    case $command in
        report)          generate_report ;;
        check-engine)    check_engine_process ;;
        check-agents)    check_agent_processes ;;
        check-state)     check_state_file ;;
        check-disk)      check_disk_space ;;
        check-logs)      check_log_size ;;
        check-locks)     check_lock_files ;;
        *)
            echo "Usage: health_check.sh {report|check-engine|check-agents|check-state|check-disk|check-logs|check-locks}"
            exit 1
            ;;
    esac
}

main "$@"
