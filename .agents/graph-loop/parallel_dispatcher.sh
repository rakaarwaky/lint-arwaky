#!/bin/bash
# Parallel Dispatcher for Graph Loop Pipeline (Revision 2.0)
# Spawns Business-Analyst and Tech-Lead agents simultaneously
# FIX: Race condition in PID file writing resolved
# FIX: Full names used
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
DISPATCHER="${SCRIPT_DIR}/dispatch.sh"
STATE_MANAGER="${SCRIPT_DIR}/state_manager.sh"
CONFIG_READER="${SCRIPT_DIR}/config_reader.sh"
LOG_FILE="${SCRIPT_DIR}/execution.log"
RESULTS_DIR="${SCRIPT_DIR}/results"
PROMPTS_DIR="${SCRIPT_DIR}/prompts"
GENERATED_DIR="${PROMPTS_DIR}/generated"

# ── Log Event ───────────────────────────────────────────────────────
log_event() {
    local event_type=$1
    local message=$2
    local timestamp=$(date -Iseconds)
    echo "[$timestamp] [parallel:$event_type] $message" >> "$LOG_FILE"
}

# ── Spawn Agent Background ──────────────────────────────────────────
# FIX: PID written correctly from parent shell, not inside subshell
spawn_agent() {
    local node=$1
    local feature=$2
    local feature_path=$3
    local frd_path=$4
    local prompt_file="${GENERATED_DIR}/${feature}-${node}.prompt"
    local result_file="${RESULTS_DIR}/${feature}-${node}.result"
    local pid_file="${RESULTS_DIR}/${feature}-${node}.pid"

    mkdir -p "$RESULTS_DIR" "$GENERATED_DIR"

    # Generate prompt based on node type
    case "$node" in
        business-analyst)
            "$DISPATCHER" business-analyst "$feature" "$feature_path" "$frd_path" > "$prompt_file"
            ;;
        tech-lead)
            "$DISPATCHER" tech-lead "$feature" "$feature_path" "$frd_path" > "$prompt_file"
            ;;
        *)
            log_event "error" "Unknown node for parallel dispatch: $node"
            return 1
            ;;
    esac

    local prompt
    prompt=$(cat "$prompt_file")

    local project_root
    project_root=$("$CONFIG_READER" project-root)

    # FIX: Run in background and capture PID from parent shell
    (cd "$project_root" && timeout 1800 qwen -p "$prompt" -o text > "$result_file" 2>> "$LOG_FILE") &
    local bg_pid=$!

    # Write PID file from parent (correct PID)
    echo "$bg_pid" > "$pid_file"

    log_event "spawn" "Agent $node spawned (PID: $bg_pid) for feature: $feature"
    log_event "spawn" "Prompt: $prompt_file"
    log_event "spawn" "Result: $result_file"
}

# ── Dispatch Parallel Analysis ──────────────────────────────────────
dispatch_parallel_analysis() {
    local feature=$1
    local feature_path=$2
    local frd_path=$3

    log_event "start" "Starting parallel analysis for: $feature"

    # Spawn both agents
    spawn_agent "business-analyst" "$feature" "$feature_path" "$frd_path"
    spawn_agent "tech-lead" "$feature" "$feature_path" "$frd_path"

    # Transition state to ANALYZING
    "$STATE_MANAGER" start-analyzing

    log_event "start" "Both Business-Analyst and Tech-Lead spawned"
}

# ── Wait for Parallel Completion ────────────────────────────────────
wait_for_parallel() {
    local feature=$1
    local ba_timeout_minutes=${2:-20}
    local tl_timeout_minutes=${3:-30}
    local check_interval=30
    local elapsed=0
    local max_seconds=$((tl_timeout_minutes * 60))
    local ba_done=false
    local tl_done=false

    log_event "wait" "Waiting for Business-Analyst (${ba_timeout_minutes}m) and Tech-Lead (${tl_timeout_minutes}m)..."

    while [ "$ba_done" = false ] || [ "$tl_done" = false ]; do
        # Check Business-Analyst
        if [ "$ba_done" = false ]; then
            local ba_report
            ba_report=$(ls "${SCRIPT_DIR}/results/business-analyst-${feature}.md" 2>/dev/null | head -1 || echo "")
            if [ -n "$ba_report" ]; then
                ba_done=true
                log_event "complete" "Business-Analyst completed: $ba_report"
                "$STATE_MANAGER" complete "business-analyst" "$ba_report"
            elif [ "$elapsed" -ge "$((ba_timeout_minutes * 60))" ]; then
                log_event "timeout" "Business-Analyst timed out after ${ba_timeout_minutes}m"
                "$STATE_MANAGER" complete "business-analyst" "TIMEOUT"
                ba_done=true
            fi
        fi

        # Check Tech-Lead
        if [ "$tl_done" = false ]; then
            local tl_report
            tl_report=$(ls "${SCRIPT_DIR}/results/tech-lead-${feature}.md" 2>/dev/null | head -1 || echo "")
            if [ -n "$tl_report" ]; then
                tl_done=true
                log_event "complete" "Tech-Lead completed: $tl_report"
                "$STATE_MANAGER" complete "tech-lead" "$tl_report"
            elif [ "$elapsed" -ge "$((tl_timeout_minutes * 60))" ]; then
                log_event "timeout" "Tech-Lead timed out after ${tl_timeout_minutes}m"
                "$STATE_MANAGER" complete "tech-lead" "TIMEOUT"
                tl_done=true
            fi
        fi

        if [ "$ba_done" = true ] && [ "$tl_done" = true ]; then
            break
        fi

        sleep "$check_interval"
        elapsed=$((elapsed + check_interval))

        if [ $((elapsed % 300)) -eq 0 ]; then
            log_event "wait" "Waiting... BA=$ba_done, TL=$tl_done (${elapsed}s elapsed)"
        fi
    done

    log_event "complete" "Both Business-Analyst and Tech-Lead completed — ready for Architect"
}

# ── Main ────────────────────────────────────────────────────────────
main() {
    mkdir -p "$PROMPTS_DIR" "$RESULTS_DIR"

    local command=${1:-}
    shift || true

    case $command in
        parallel-analysis)
            dispatch_parallel_analysis "$1" "$2" "$3"
            ;;
        wait)
            wait_for_parallel "$1" "${2:-20}" "${3:-30}"
            ;;
        spawn)
            spawn_agent "$1" "$2" "$3" "$4"
            ;;
        *)
            echo "Usage: parallel_dispatcher.sh {parallel-analysis|wait|spawn}"
            echo ""
            echo "Commands:"
            echo "  parallel-analysis <feature> <feature_path> <frd_path>"
            echo "  wait <feature> [ba_timeout_min] [tl_timeout_min]"
            echo "  spawn <node> <feature> <feature_path> <frd_path>"
            exit 1
            ;;
    esac
}

main "$@"
