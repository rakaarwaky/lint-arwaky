#!/bin/bash
# State Manager for Graph Loop Pipeline (Revision 2.0)
# Handles state read, write, transitions, and counters
# FIX: Uses "QUALITY-ANALYSIS" state name (not "QA")
# FIX: Proper counter management with separate rejection/pipeline counters
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
STATE_FILE="${SCRIPT_DIR}/state.json"
LOG_FILE="${SCRIPT_DIR}/execution.log"

# ── Log Event ───────────────────────────────────────────────────────
log_event() {
    local event_type=$1
    local message=$2
    local timestamp=$(date -Iseconds)
    echo "[$timestamp] [state:$event_type] $message" >> "$LOG_FILE"
}

# ── Read State ──────────────────────────────────────────────────────
read_state() {
    local path="${1:-.}"
    if [ "$path" = "." ]; then
        cat "$STATE_FILE"
    else
        jq -r "$path" "$STATE_FILE"
    fi
}

# ── Write State ─────────────────────────────────────────────────────
write_state() {
    local tmp_file="${STATE_FILE}.tmp"
    cat > "$tmp_file"
    # SAFETY: Never overwrite state.json with an empty file
    if [ -s "$tmp_file" ]; then
        mv "$tmp_file" "$STATE_FILE"
    else
        echo "[state_manager] WARN: refusing to write empty state" >&2
        rm -f "$tmp_file"
        return 1
    fi
}

# ── Get Current State ───────────────────────────────────────────────
get_current_state() {
    jq -r '.pipeline.current_state' "$STATE_FILE"
}

# ── Transition State ────────────────────────────────────────────────
transition_state() {
    local new_state=$1
    local timestamp=$(date -Iseconds)

    jq --arg state "$new_state" --arg ts "$timestamp" \
        '.pipeline.current_state = $state' "$STATE_FILE" | write_state

    log_event "transition" "State changed to: $new_state"
}

# ── Increment Rejection Loop Counter ────────────────────────────────
increment_rejection() {
    jq '.pipeline.rejection_loop_counter = (.pipeline.rejection_loop_counter + 1)' \
        "$STATE_FILE" | write_state

    local counter
    counter=$(jq -r '.pipeline.rejection_loop_counter' "$STATE_FILE")
    log_event "counter" "Rejection loop counter incremented to: $counter"
}

# ── Increment Pipeline Iteration Counter ────────────────────────────
increment_pipeline() {
    jq '.pipeline.pipeline_iteration_counter = (.pipeline.pipeline_iteration_counter + 1)' \
        "$STATE_FILE" | write_state

    local counter
    counter=$(jq -r '.pipeline.pipeline_iteration_counter' "$STATE_FILE")
    log_event "counter" "Pipeline iteration counter incremented to: $counter"
}

# ── Reset Rejection Counter (new pipeline run) ─────────────────────
reset_rejection() {
    jq '.pipeline.rejection_loop_counter = 0' "$STATE_FILE" | write_state
    log_event "counter" "Rejection loop counter reset to 0"
}

# ── Start Pipeline ──────────────────────────────────────────────────
# FIX: Reset pipeline_iteration_counter to 1 for new feature
# FIX: Reset rejection_loop_counter to 0
start_pipeline() {
    local feature=$1
    local pipeline_id="pipeline-$(date +%Y%m%d)-$(shuf -i 100-999 -n 1)"
    local correlation_id="corr-$(date +%Y%m%d)-${feature}-$(shuf -i 100-999 -n 1)"
    local timestamp=$(date -Iseconds)

    jq --arg feature "$feature" --arg pid "$pipeline_id" --arg cid "$correlation_id" --arg ts "$timestamp" \
        '.pipeline.id = $pid |
         .pipeline.feature = $feature |
         .pipeline.current_state = "DISPATCHING" |
         .pipeline.started_at = $ts |
         .pipeline.correlation_id = $cid |
         .pipeline.iteration = 1 |
         .pipeline.pipeline_iteration_counter = 1 |
         .pipeline.rejection_loop_counter = 0 |
         .pipeline.parallel_nodes["business-analyst"].status = "pending" |
         .pipeline.parallel_nodes["business-analyst"].started_at = null |
         .pipeline.parallel_nodes["business-analyst"].completed_at = null |
         .pipeline.parallel_nodes["business-analyst"].report_file = null |
         .pipeline.parallel_nodes["tech-lead"].status = "pending" |
         .pipeline.parallel_nodes["tech-lead"].started_at = null |
         .pipeline.parallel_nodes["tech-lead"].completed_at = null |
         .pipeline.parallel_nodes["tech-lead"].report_file = null |
         .pipeline.pending_merge = [] |
         .pipeline.failure = null |
         .pipeline.escalation = null' "$STATE_FILE" | write_state

    log_event "start" "Pipeline started for feature: $feature (ID: $pipeline_id, correlation: $correlation_id)"
    echo "$pipeline_id|$correlation_id"
}

# ── Transition to ANALYZING (after dispatch) ────────────────────────
start_analyzing() {
    local timestamp=$(date -Iseconds)

    jq --arg ts "$timestamp" \
        '.pipeline.current_state = "ANALYZING" |
         .pipeline.parallel_nodes["business-analyst"].status = "running" |
         .pipeline.parallel_nodes["business-analyst"].started_at = $ts |
         .pipeline.parallel_nodes["tech-lead"].status = "running" |
         .pipeline.parallel_nodes["tech-lead"].started_at = $ts' "$STATE_FILE" | write_state

    log_event "analyzing" "Parallel analysis started"
}

# ── Mark Node Complete ──────────────────────────────────────────────
mark_node_complete() {
    local node=$1
    local report_file=$2
    local timestamp=$(date -Iseconds)

    jq --arg node "$node" --arg report "$report_file" --arg ts "$timestamp" \
        ".pipeline.parallel_nodes[\"$node\"].status = \"completed\" |
         .pipeline.parallel_nodes[\"$node\"].report_file = \$report |
         .pipeline.parallel_nodes[\"$node\"].completed_at = \$ts" "$STATE_FILE" | write_state

    log_event "node_complete" "Node $node completed: $report_file"

    # Check if both parallel nodes are done
    check_parallel_completion
}

# ── Check Parallel Completion ───────────────────────────────────────
check_parallel_completion() {
    local ba_status tl_status
    ba_status=$(jq -r '.pipeline.parallel_nodes["business-analyst"].status' "$STATE_FILE")
    tl_status=$(jq -r '.pipeline.parallel_nodes["tech-lead"].status' "$STATE_FILE")

    if [ "$ba_status" = "completed" ] && [ "$tl_status" = "completed" ]; then
        transition_state "ARCHITECT"
        log_event "parallel_complete" "Both Business-Analyst and Tech-Lead completed — transitioning to ARCHITECT"
    fi
}

# ── Mark Architect Merge Complete ───────────────────────────────────
mark_merge_complete() {
    local merged_plan=$1
    local timestamp=$(date -Iseconds)

    jq --arg plan "$merged_plan" --arg ts "$timestamp" \
        '.pipeline.current_state = "DEVELOPER" |
         .pipeline.pending_merge = []' "$STATE_FILE" | write_state

    log_event "merge_complete" "Architect merge complete: $merged_plan"
}

# ── Mark Developer Complete ─────────────────────────────────────────
# FIX: Use "QUALITY-ANALYSIS" state name
mark_developer_complete() {
    local pr_number=$1
    local timestamp=$(date -Iseconds)

    jq --arg pr "$pr_number" --arg ts "$timestamp" \
        '.pipeline.current_state = "QUALITY-ANALYSIS"' "$STATE_FILE" | write_state

    log_event "developer_complete" "Developer completed — PR #$pr_number created — transitioning to QUALITY-ANALYSIS"
}

# ── Mark Quality-Analysis Approved ──────────────────────────────────
mark_qa_approved() {
    local timestamp=$(date -Iseconds)

    jq --arg ts "$timestamp" \
        '.pipeline.current_state = "MERGED"' "$STATE_FILE" | write_state

    log_event "qa_approved" "Quality-Analysis approved — pipeline complete"
}

# ── Mark Quality-Analysis Rejected ──────────────────────────────────
mark_qa_rejected() {
    local reason=$1
    local timestamp=$(date -Iseconds)

    jq --arg reason "$reason" --arg ts "$timestamp" \
        '.pipeline.current_state = "ARCHITECT"' "$STATE_FILE" | write_state

    log_event "qa_rejected" "Quality-Analysis rejected: $reason — re-merge needed"
}

# ── Reset to Idle ───────────────────────────────────────────────────
reset_to_idle() {
    local timestamp=$(date -Iseconds)

    jq '.pipeline.current_state = "IDLE" |
        .pipeline.id = null |
        .pipeline.feature = null |
        .pipeline.started_at = null |
        .pipeline.iteration = 0 |
        .pipeline.correlation_id = null |
        .pipeline.rejection_loop_counter = 0 |
        .pipeline.pipeline_iteration_counter = 0 |
        .pipeline.parallel_nodes["business-analyst"].status = "idle" |
        .pipeline.parallel_nodes["business-analyst"].task_id = null |
        .pipeline.parallel_nodes["business-analyst"].report_file = null |
        .pipeline.parallel_nodes["business-analyst"].started_at = null |
        .pipeline.parallel_nodes["business-analyst"].completed_at = null |
        .pipeline.parallel_nodes["tech-lead"].status = "idle" |
        .pipeline.parallel_nodes["tech-lead"].task_id = null |
        .pipeline.parallel_nodes["tech-lead"].report_file = null |
        .pipeline.parallel_nodes["tech-lead"].started_at = null |
        .pipeline.parallel_nodes["tech-lead"].completed_at = null |
        .pipeline.pending_merge = [] |
        .pipeline.failure = null |
        .pipeline.escalation = null' "$STATE_FILE" | write_state

    log_event "reset" "Pipeline reset to IDLE"
}

# ── Mark Failed ─────────────────────────────────────────────────────
mark_failed() {
    local reason=$1
    local timestamp=$(date -Iseconds)

    jq --arg reason "$reason" --arg ts "$timestamp" \
        '.pipeline.current_state = "FAILED" |
         .pipeline.failure = {"reason": $reason, "at": $ts}' "$STATE_FILE" | write_state

    log_event "failed" "Pipeline marked FAILED: $reason"
}

# ── Mark Blocked ────────────────────────────────────────────────────
mark_blocked() {
    local reason=$1
    local timestamp=$(date -Iseconds)

    jq --arg reason "$reason" --arg ts "$timestamp" \
        '.pipeline.current_state = "BLOCKED" |
         .pipeline.failure = {"reason": $reason, "at": $ts}' "$STATE_FILE" | write_state

    log_event "blocked" "Pipeline marked BLOCKED: $reason"
}

# ── Mark Escalated ──────────────────────────────────────────────────
mark_escalated() {
    local reason=$1
    local timestamp=$(date -Iseconds)

    jq --arg reason "$reason" --arg ts "$timestamp" \
        '.pipeline.current_state = "ESCALATED" |
         .pipeline.escalation = {"reason": $reason, "at": $ts}' "$STATE_FILE" | write_state

    log_event "escalated" "Pipeline marked ESCALATED: $reason"
}

# ── Mark Waiting Human ──────────────────────────────────────────────
mark_waiting_human() {
    local reason=$1
    local timestamp=$(date -Iseconds)

    jq --arg reason "$reason" --arg ts "$timestamp" \
        '.pipeline.current_state = "WAITING_HUMAN" |
         .pipeline.escalation = {"reason": $reason, "at": $ts}' "$STATE_FILE" | write_state

    log_event "waiting_human" "Pipeline marked WAITING_HUMAN: $reason"
}

# ── Main ────────────────────────────────────────────────────────────
main() {
    local command=${1:-}
    shift || true

    case $command in
        read)               read_state "${1:-.}" ;;
        state)              get_current_state ;;
        start)              start_pipeline "$1" ;;
        start-analyzing)    start_analyzing ;;
        complete)           mark_node_complete "$1" "$2" ;;
        merge-complete)     mark_merge_complete "$1" ;;
        dev-complete)       mark_developer_complete "$1" ;;
        qa-approved)        mark_qa_approved ;;
        qa-rejected)        mark_qa_rejected "$1" ;;
        reset)              reset_to_idle ;;
        failed)             mark_failed "$1" ;;
        blocked)            mark_blocked "$1" ;;
        escalated)          mark_escalated "$1" ;;
        waiting-human)      mark_waiting_human "$1" ;;
        increment-rejection) increment_rejection ;;
        increment-pipeline)  increment_pipeline ;;
        reset-rejection)     reset_rejection ;;
        transition-state)    transition_state "$1" ;;
        *)
            echo "Usage: state_manager.sh {read|state|start|start-analyzing|complete|merge-complete|dev-complete|qa-approved|qa-rejected|reset|failed|blocked|escalated|waiting-human|increment-rejection|increment-pipeline|reset-rejection|transition-state}"
            exit 1
            ;;
    esac
}

main "$@"
