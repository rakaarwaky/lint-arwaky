#!/bin/bash
# Graph Loop Execution Engine (Revision 2.0)
# FIX: Uses "QUALITY-ANALYSIS" state name
# FIX: Pipeline iteration counter incremented on re-merge loop
# FIX: Recovery policy 3-tier (60m/180m)
# FIX: Exponential backoff for retries
# FIX: Quality-Analysis receives all 6 inputs
# FIX: Skip Report handling
# FIX: Full names used
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
STATE_MANAGER="${SCRIPT_DIR}/state_manager.sh"
FEATURE_MANAGER="${SCRIPT_DIR}/feature_manager.sh"
SCANNER="${SCRIPT_DIR}/trigger_scanner.sh"
DISPATCHER="${SCRIPT_DIR}/dispatch.sh"
CONDITIONAL_SKIP="${SCRIPT_DIR}/conditional_skip.sh"
CONFIG_READER="${SCRIPT_DIR}/config_reader.sh"
NOTIFY="${SCRIPT_DIR}/notify.sh"
STATE_FILE="${SCRIPT_DIR}/state.json"
FEATURES_FILE="${SCRIPT_DIR}/features.json"
LOG_FILE="${SCRIPT_DIR}/execution.log"
RESULTS_DIR="${SCRIPT_DIR}/results"
PLANS_DIR="${SCRIPT_DIR}/plans"

# ── Read Config Values ──────────────────────────────────────────────
POLL_INTERVAL=$("$CONFIG_READER" settings "poll_interval_seconds" "30")
GLOBAL_TIMEOUT_MINUTES=$("$CONFIG_READER" settings "global_timeout_minutes" "180")
MAX_REJECTION_LOOPS=$("$CONFIG_READER" counters "max_rejection_loops" "3")
MAX_PIPELINE_ITERATIONS=$("$CONFIG_READER" counters "max_pipeline_iterations" "5")
RESUME_STALE_MINUTES=$("$CONFIG_READER" recovery "resume_if_stale_minutes" "60")
BACKOFF_INITIAL=$("$CONFIG_READER" recovery "exponential_backoff.initial_minutes" "2")
BACKOFF_MAX=$("$CONFIG_READER" recovery "exponential_backoff.max_minutes" "30")

# ── Log Event ───────────────────────────────────────────────────────
log_event() {
    local event_type=$1
    local message=$2
    local timestamp=$(date -Iseconds)
    echo "[$timestamp] [engine:$event_type] $message" >> "$LOG_FILE"
}

# ── State Accessors ─────────────────────────────────────────────────
get_state() { jq -r '.pipeline.current_state' "$STATE_FILE"; }
get_feature() { jq -r '.pipeline.feature' "$STATE_FILE"; }
get_pipeline_id() { jq -r '.pipeline.id' "$STATE_FILE"; }
get_correlation_id() { jq -r '.pipeline.correlation_id // "unknown"' "$STATE_FILE"; }
get_rejection_counter() { jq -r '.pipeline.rejection_loop_counter' "$STATE_FILE"; }
get_pipeline_counter() { jq -r '.pipeline.pipeline_iteration_counter' "$STATE_FILE"; }

# ── Check Node Timeout ──────────────────────────────────────────────
check_node_timeout() {
    local node=$1
    local timeout_minutes
    timeout_minutes=$("$CONFIG_READER" node "$node" "timeout_minutes" "30")

    local started_at
    started_at=$(jq -r ".pipeline.parallel_nodes.\"${node}\".started_at // empty" "$STATE_FILE")

    if [ -z "$started_at" ]; then
        return 1
    fi

    local started_epoch
    started_epoch=$(date -d "$started_at" +%s 2>/dev/null || echo "0")
    local now=$(date +%s)
    local elapsed=$(( (now - started_epoch) / 60 ))

    if [ "$elapsed" -ge "$timeout_minutes" ]; then
        log_event "timeout" "Node $node timed out after ${elapsed}m (limit: ${timeout_minutes}m)"
        return 0
    fi
    return 1
}

# ── Recovery Policy on Restart ──────────────────────────────────────
# FIX: 3-tier recovery (< 60m resume, 60-180m resume with warning, >= 180m BLOCKED)
handle_recovery() {
    if [ ! -f "$STATE_FILE" ]; then
        log_event "recovery" "No state.json found — starting fresh"
        "$STATE_MANAGER" reset
        return
    fi

    local current_state
    current_state=$(get_state)

    if [ "$current_state" = "IDLE" ]; then
        log_event "recovery" "State is IDLE — no recovery needed"
        return
    fi

    local started_at
    started_at=$(jq -r '.pipeline.started_at // empty' "$STATE_FILE")

    if [ -z "$started_at" ] || [ "$started_at" = "null" ]; then
        log_event "recovery" "No started_at — marking FAILED"
        "$STATE_MANAGER" failed "Missing started_at on restart"
        "$FEATURE_MANAGER" fail "$(get_feature)" "Missing started_at on restart"
        "$STATE_MANAGER" reset
        return
    fi

    local started_epoch
    started_epoch=$(date -d "$started_at" +%s 2>/dev/null || echo "0")
    local now=$(date +%s)
    local elapsed_minutes=$(( (now - started_epoch) / 60 ))

    log_event "recovery" "State=$current_state, elapsed=${elapsed_minutes}m"

    # Policy 1: WAITING_HUMAN — stay and wait
    if [ "$current_state" = "WAITING_HUMAN" ]; then
        log_event "recovery" "State is WAITING_HUMAN — staying (waiting for human)"
        return
    fi

    # Policy 2: Within resume threshold (< 60m) — auto-resume
    if [ "$elapsed_minutes" -lt "$RESUME_STALE_MINUTES" ]; then
        log_event "recovery" "Elapsed < ${RESUME_STALE_MINUTES}m — auto-resuming from $current_state"
        return
    fi

    # Policy 3: Between resume and max (60m-180m) — resume with warning
    if [ "$elapsed_minutes" -lt "$GLOBAL_TIMEOUT_MINUTES" ]; then
        log_event "recovery" "WARNING: Elapsed ${elapsed_minutes}m (stale) — resuming from $current_state with caution"
        "$NOTIFY" error "engine" "Pipeline state is stale (${elapsed_minutes}m old) — resuming with caution" "$(get_feature)"
        return
    fi

    # Policy 4: Exceeded global timeout (>= 180m) — mark BLOCKED
    log_event "recovery" "Elapsed >= ${GLOBAL_TIMEOUT_MINUTES}m — marking BLOCKED"
    "$STATE_MANAGER" blocked "Global timeout on restart (${elapsed_minutes}m)"
    "$FEATURE_MANAGER" fail "$(get_feature)" "Global timeout on restart (${elapsed_minutes}m)"
    "$STATE_MANAGER" reset
}

# ── Handle IDLE State ───────────────────────────────────────────────
handle_idle() {
    log_event "state" "State: IDLE — scanning for triggers..."

    # Sync features from config if features.json is empty
    local feature_count
    feature_count=$(jq '.features | length' "$FEATURES_FILE" 2>/dev/null || echo "0")
    if [ "$feature_count" -eq 0 ]; then
        "$FEATURE_MANAGER" sync
    fi

    # Scan GitHub PRs
    local prs
    prs=$("$SCANNER" scan-prs 2>/dev/null || echo "")

    if [ -n "$prs" ]; then
        echo "$prs" | head -1 | while IFS='|' read -r pr_number pr_title pr_branch; do
            log_event "state" "Found PR #${pr_number}: ${pr_title}"
            "$SCANNER" handle-pr "$pr_number" "$pr_title" "$pr_branch"
        done
        return 0
    fi

    # Check for pending features (sorted by priority)
    local feature
    feature=$("$FEATURE_MANAGER" select 2>/dev/null || echo "")

    if [ -n "$feature" ]; then
        log_event "state" "Found pending feature: ${feature}"
        local feature_path
        feature_path=$("$FEATURE_MANAGER" path "$feature")

        local pipeline_info
        pipeline_info=$("$STATE_MANAGER" start "$feature")
        local pipeline_id
        pipeline_id=$(echo "$pipeline_info" | cut -d'|' -f1)

        "$FEATURE_MANAGER" claim "$feature" "$feature_path" "$pipeline_id"
        return 0
    fi

    log_event "state" "No triggers found — staying IDLE"
}

# ── Handle DISPATCHING State ────────────────────────────────────────
handle_dispatching() {
    local feature
    feature=$(get_feature)

    log_event "state" "State: DISPATCHING — dispatching agents for: ${feature}"

    local feature_path
    feature_path=$("$FEATURE_MANAGER" path "$feature")
    local frd_path="${feature_path}/FRD.md"

    # Dispatch parallel analysis
    "$SCRIPT_DIR/parallel_dispatcher.sh" parallel-analysis "$feature" "$feature_path" "$frd_path"
}

# ── Handle ANALYZING State ──────────────────────────────────────────
handle_analyzing() {
    local feature
    feature=$(get_feature)

    log_event "state" "State: ANALYZING — feature: ${feature}"

    local ba_status tl_status
    ba_status=$(jq -r '.pipeline.parallel_nodes["business-analyst"].status' "$STATE_FILE")
    tl_status=$(jq -r '.pipeline.parallel_nodes["tech-lead"].status' "$STATE_FILE")

    # Check timeouts
    if [ "$ba_status" = "running" ] && check_node_timeout "business-analyst"; then
        log_event "state" "Business-Analyst timed out — marking as failed"
        "$STATE_MANAGER" failed "Business-Analyst timeout"
        "$FEATURE_MANAGER" fail "$feature" "Business-Analyst timeout"
        "$STATE_MANAGER" reset
        return
    fi

    if [ "$tl_status" = "running" ] && check_node_timeout "tech-lead"; then
        log_event "state" "Tech-Lead timed out — marking as failed"
        "$STATE_MANAGER" failed "Tech-Lead timeout"
        "$FEATURE_MANAGER" fail "$feature" "Tech-Lead timeout"
        "$STATE_MANAGER" reset
        return
    fi

    # Check if both completed
    if [ "$ba_status" = "completed" ] && [ "$tl_status" = "completed" ]; then
        log_event "state" "Both Business-Analyst and Tech-Lead completed — transitioning to ARCHITECT"
        "$STATE_MANAGER" transition-state "ARCHITECT"
        return
    fi

    # Check for report files (filesystem trigger)
    local ba_report tl_report
    ba_report=$(ls "${RESULTS_DIR}/${feature}-business-analyst.result" 2>/dev/null | head -1 || echo "")
    tl_report=$(ls "${RESULTS_DIR}/${feature}-tech-lead.result" 2>/dev/null | head -1 || echo "")

    if [ -n "$ba_report" ] && [ "$ba_status" = "running" ]; then
        log_event "state" "Business-Analyst report found: ${ba_report}"
        "$STATE_MANAGER" complete "business-analyst" "$ba_report"
    fi

    if [ -n "$tl_report" ] && [ "$tl_status" = "running" ]; then
        log_event "state" "Tech-Lead report found: ${tl_report}"
        "$STATE_MANAGER" complete "tech-lead" "$tl_report"
    fi
}

# ── Handle ARCHITECT State ──────────────────────────────────────────
handle_architect() {
    local feature
    feature=$(get_feature)
    local pipeline_id
    pipeline_id=$(get_pipeline_id)
    local correlation_id
    correlation_id=$(get_correlation_id)

    log_event "state" "State: ARCHITECT — merging reports for: ${feature}"

    local feature_path
    feature_path=$("$FEATURE_MANAGER" path "$feature")
    local frd_path="${feature_path}/FRD.md"

    # Find BA and TL reports
    local ba_report tl_report
    ba_report=$(ls "${RESULTS_DIR}/${feature}-business-analyst.result" 2>/dev/null | head -1 || echo "")
    tl_report=$(ls "${RESULTS_DIR}/${feature}-tech-lead.result" 2>/dev/null | head -1 || echo "")

    # FIX: Handle Skip Reports
    local skip_report=""
    if [ -z "$ba_report" ]; then
        local skip_reason="Business-Analyst skipped (simple fix or low complexity)"
        skip_report=$("$CONDITIONAL_SKIP" generate-skip-report "Business-Analyst" "$feature" "$feature_path" "$skip_reason")
        ba_report="$skip_report"
        log_event "state" "Business-Analyst skipped — Skip Report generated"
    fi

    if [ -z "$tl_report" ]; then
        local skip_reason="Tech-Lead skipped (doc-only update)"
        local tl_skip_report
        tl_skip_report=$("$CONDITIONAL_SKIP" generate-skip-report "Tech-Lead" "$feature" "$feature_path" "$skip_reason")
        if [ -n "$skip_report" ]; then
            skip_report="${skip_report}\n${tl_skip_report}"
        else
            skip_report="$tl_skip_report"
        fi
        tl_report="$tl_skip_report"
        log_event "state" "Tech-Lead skipped — Skip Report generated"
    fi

    # Dispatch architect
    log_event "state" "Dispatching Architect to merge reports"

    local prompt
    prompt=$("$DISPATCHER" architect "$feature" "$feature_path" "$frd_path" "$ba_report" "$tl_report" "$skip_report")

    local project_root
    project_root=$("$CONFIG_READER" project-root)

    local result_file="${RESULTS_DIR}/architect-${feature}.result"
    mkdir -p "$RESULTS_DIR" "$PLANS_DIR"

    log_event "state" "Running Architect agent..."
    (cd "$project_root" && timeout 1800 qwen -p "$prompt" -o text > "$result_file" 2>> "$LOG_FILE") &
    local arch_pid=$!

    # Wait for architect to complete (check for merged plan)
    # FIX: Look in correct plans directory
    local merged_plan=""
    local wait_count=0
    local max_wait=60

    while [ -z "$merged_plan" ] && [ "$wait_count" -lt "$max_wait" ]; do
        sleep 30
        merged_plan=$(ls "${PLANS_DIR}/merged-${feature}-${correlation_id}.md" 2>/dev/null | head -1 || echo "")
        if [ -z "$merged_plan" ]; then
            merged_plan=$(ls "${PLANS_DIR}/merged-${feature}-"*.md 2>/dev/null | head -1 || echo "")
        fi
        wait_count=$((wait_count + 1))
        echo "[engine] $(date -Iseconds) Waiting for Architect... (${wait_count}/${max_wait})"
    done

    if [ -z "$merged_plan" ]; then
        kill "$arch_pid" 2>/dev/null || true
        log_event "state" "Architect timed out — marking FAILED"
        "$STATE_MANAGER" failed "Architect timeout"
        "$FEATURE_MANAGER" fail "$feature" "Architect timeout"
        "$STATE_MANAGER" reset
        return
    fi

    log_event "state" "Architect completed: ${merged_plan}"
    "$STATE_MANAGER" merge-complete "$merged_plan"
}

# ── Handle DEVELOPER State ──────────────────────────────────────────
handle_developer() {
    local feature
    feature=$(get_feature)
    local pipeline_id
    pipeline_id=$(get_pipeline_id)
    local correlation_id
    correlation_id=$(get_correlation_id)

    log_event "state" "State: DEVELOPER — implementing: ${feature}"

    local feature_path
    feature_path=$("$FEATURE_MANAGER" path "$feature")
    local frd_path="${feature_path}/FRD.md"

    # Find merged plan
    local merged_plan
    merged_plan=$(ls "${PLANS_DIR}/merged-${feature}-"*.md 2>/dev/null | head -1 || echo "")

    if [ -z "$merged_plan" ]; then
        log_event "state" "Merged plan not found"
        return
    fi

    # Dispatch developer
    log_event "state" "Dispatching Developer"

    local prompt
    prompt=$("$DISPATCHER" developer "$feature" "$feature_path" "$frd_path" "$merged_plan")

    local project_root
    project_root=$("$CONFIG_READER" project-root)

    local result_file="${RESULTS_DIR}/developer-${feature}.result"
    mkdir -p "$RESULTS_DIR"

    log_event "state" "Running Developer agent..."
    (cd "$project_root" && timeout 3600 qwen -p "$prompt" -o text > "$result_file" 2>> "$LOG_FILE") &
    local dev_pid=$!

    # Wait for developer to complete
    local dev_report=""
    local wait_count=0
    local max_wait=120

    while [ -z "$dev_report" ] && [ "$wait_count" -lt "$max_wait" ]; do
        sleep 30
        dev_report=$(ls "${SCRIPT_DIR}/reports/done-${feature}-"*.md 2>/dev/null | head -1 || echo "")
        wait_count=$((wait_count + 1))
        echo "[engine] $(date -Iseconds) Waiting for Developer... (${wait_count}/${max_wait})"
    done

    if [ -z "$dev_report" ]; then
        kill "$dev_pid" 2>/dev/null || true
        log_event "state" "Developer timed out — marking FAILED"
        "$STATE_MANAGER" failed "Developer timeout"
        "$FEATURE_MANAGER" fail "$feature" "Developer timeout"
        "$STATE_MANAGER" reset
        return
    fi

    log_event "state" "Developer completed: ${dev_report}"
    "$STATE_MANAGER" dev-complete "PR"
}

# ── Handle QUALITY-ANALYSIS State ───────────────────────────────────
# FIX: State name is "QUALITY-ANALYSIS" (not "QA")
# FIX: All 6 inputs passed to Quality-Analysis
# FIX: Pipeline iteration counter incremented on re-merge
handle_quality_analysis() {
    local feature
    feature=$(get_feature)
    local correlation_id
    correlation_id=$(get_correlation_id)

    log_event "state" "State: QUALITY-ANALYSIS — reviewing: ${feature}"

    local feature_path
    feature_path=$("$FEATURE_MANAGER" path "$feature")
    local frd_path="${feature_path}/FRD.md"

    # Find all required inputs
    local dev_report
    dev_report=$(ls "${SCRIPT_DIR}/reports/done-${feature}-"*.md 2>/dev/null | head -1 || echo "")

    if [ -z "$dev_report" ]; then
        log_event "state" "Developer report not found"
        return
    fi

    local pr_number
    pr_number=$(grep -oP 'PR.*#\K[0-9]+' "$dev_report" 2>/dev/null || echo "")

    if [ -z "$pr_number" ]; then
        log_event "state" "PR number not found in report"
        return
    fi

    local merged_plan
    merged_plan=$(ls "${PLANS_DIR}/merged-${feature}-"*.md 2>/dev/null | head -1 || echo "")

    local ba_report
    ba_report=$(ls "${RESULTS_DIR}/${feature}-business-analyst.result" 2>/dev/null | head -1 || echo "")

    local tl_report
    tl_report=$(ls "${RESULTS_DIR}/${feature}-tech-lead.result" 2>/dev/null | head -1 || echo "")

    # Determine QA mode
    local qa_mode
    qa_mode=$("$CONDITIONAL_SKIP" qa-mode "$feature_path" "" 2>/dev/null || echo "full-review")

    # Dispatch Quality-Analysis with all 6 inputs
    log_event "state" "Dispatching Quality-Analysis for PR #${pr_number} (mode: $qa_mode)"

    local prompt
    prompt=$("$DISPATCHER" quality-analysis "$feature" "$pr_number" "$merged_plan" "$frd_path" "$ba_report" "$tl_report" "$dev_report" "$qa_mode")

    local project_root
    project_root=$("$CONFIG_READER" project-root)

    local result_file="${RESULTS_DIR}/quality-analysis-${feature}.result"
    mkdir -p "$RESULTS_DIR"

    log_event "state" "Running Quality-Analysis agent..."
    (cd "$project_root" && timeout 1800 qwen -p "$prompt" -o text > "$result_file" 2>> "$LOG_FILE") &
    local qa_pid=$!

    # Wait for Quality-Analysis to complete
    local qa_verdict=""
    local wait_count=0
    local max_wait=60

    while [ -z "$qa_verdict" ] && [ "$wait_count" -lt "$max_wait" ]; do
        sleep 30

        # Check for rejection plan
        local rejection_plan
        rejection_plan=$(ls "${PLANS_DIR}/"*"quality-analysis-${feature}"*.md 2>/dev/null | head -1 || echo "")
        if [ -n "$rejection_plan" ]; then
            qa_verdict="REJECTED"
            break
        fi

        # Check if PR was merged (APPROVED)
        local pr_state
        pr_state=$(gh pr view "$pr_number" --json state --jq '.state' 2>/dev/null || echo "")
        if [ "$pr_state" = "MERGED" ]; then
            qa_verdict="APPROVED"
            break
        fi

        wait_count=$((wait_count + 1))
    done

    if [ "$qa_verdict" = "APPROVED" ]; then
        log_event "state" "Quality-Analysis APPROVED — pipeline complete"
        "$STATE_MANAGER" qa-approved
        "$FEATURE_MANAGER" complete "$feature" "$(get_pipeline_id)"
        "$NOTIFY" qa-approved "$feature" "$pr_number"

    elif [ "$qa_verdict" = "REJECTED" ]; then
        log_event "state" "Quality-Analysis REJECTED — checking rejection counter"

        local counter
        counter=$(get_rejection_counter)
        counter=$((counter + 1))

        if [ "$counter" -ge "$MAX_REJECTION_LOOPS" ]; then
            log_event "state" "Rejection counter reached max ($counter/$MAX_REJECTION_LOOPS) — ESCALATED"
            "$STATE_MANAGER" increment-rejection
            "$STATE_MANAGER" escalated "Max rejection loops reached ($counter/$MAX_REJECTION_LOOPS)"
            "$STATE_MANAGER" waiting-human "Max rejection loops reached — human intervention required"
            "$NOTIFY" error "quality-analysis" "Max rejection loops reached for feature: $feature" "$feature"
        else
            log_event "state" "Rejection counter: $counter/$MAX_REJECTION_LOOPS — re-merge"
            "$STATE_MANAGER" increment-rejection
            # FIX: Increment pipeline iteration counter on each re-merge loop
            "$STATE_MANAGER" increment-pipeline
            "$STATE_MANAGER" qa-rejected "Quality-Analysis rejected PR (rejection $counter/$MAX_REJECTION_LOOPS)"
            "$NOTIFY" qa-rejected "$feature" "$pr_number" "Rejection $counter/$MAX_REJECTION_LOOPS"
        fi
    else
        # Timeout
        kill "$qa_pid" 2>/dev/null || true
        log_event "state" "Quality-Analysis timed out — marking FAILED"
        "$STATE_MANAGER" failed "Quality-Analysis timeout"
        "$FEATURE_MANAGER" fail "$feature" "Quality-Analysis timeout"
        "$STATE_MANAGER" reset
    fi
}

# ── Handle FAILED State ─────────────────────────────────────────────
# FIX: Exponential backoff implemented
handle_failed() {
    local feature
    feature=$(get_feature)

    log_event "state" "State: FAILED — feature: ${feature}"

    local iteration
    iteration=$(get_pipeline_counter)

    if [ "$iteration" -ge "$MAX_PIPELINE_ITERATIONS" ]; then
        log_event "state" "Max pipeline iterations reached ($iteration/$MAX_PIPELINE_ITERATIONS) — BLOCKED"
        "$STATE_MANAGER" blocked "Max pipeline iterations reached"
        return
    fi

    # FIX: Exponential backoff
    local retry_count=$iteration
    local backoff_minutes=$((BACKOFF_INITIAL * (2 ** (retry_count - 1))))
    if [ "$backoff_minutes" -gt "$BACKOFF_MAX" ]; then
        backoff_minutes=$BACKOFF_MAX
    fi

    log_event "state" "Retrying after ${backoff_minutes}m backoff (iteration $iteration/$MAX_PIPELINE_ITERATIONS)"

    # Cleanup partial results before retry
    cleanup_results "$feature"

    sleep $((backoff_minutes * 60))

    # Retry: go back to ANALYZING
    "$STATE_MANAGER" transition-state "ANALYZING"
}

# ── Handle BLOCKED State ───────────────────────────────────────────
handle_blocked() {
    local feature
    feature=$(get_feature)

    log_event "state" "State: BLOCKED — skipping feature: ${feature}"
    "$FEATURE_MANAGER" fail "$feature" "BLOCKED — max retries exceeded"
    "$NOTIFY" error "engine" "Feature BLOCKED: $feature — skipping to next" "$feature"

    # Cleanup results for blocked feature
    cleanup_results "$feature"

    "$STATE_MANAGER" reset
}

# ── Handle TIMEOUT State ────────────────────────────────────────────
handle_timeout() {
    local feature
    feature=$(get_feature)

    log_event "state" "State: TIMEOUT — feature: ${feature}"
    "$FEATURE_MANAGER" fail "$feature" "Global timeout"
    "$NOTIFY" timeout "pipeline" "$feature" "$GLOBAL_TIMEOUT_MINUTES"
    "$STATE_MANAGER" blocked "Global timeout"
}

# ── Handle WAITING_HUMAN State ─────────────────────────────────────
handle_waiting_human() {
    local feature
    feature=$(get_feature)

    log_event "state" "State: WAITING_HUMAN — waiting for human intervention on: ${feature}"
    # Just wait — no action until human changes state
}

# ── Handle ESCALATED State ─────────────────────────────────────────
handle_escalated() {
    local feature
    feature=$(get_feature)

    log_event "state" "State: ESCALATED — escalating feature: ${feature} to human"
    "$STATE_MANAGER" waiting-human "Escalated by Quality-Analysis — critical issue"
    "$NOTIFY" error "quality-analysis" "CRITICAL escalation for feature: $feature" "$feature"
}

# ── Cleanup Results ─────────────────────────────────────────────────
cleanup_results() {
    local feature=$1
    local prompts_dir="${SCRIPT_DIR}/prompts"
    local generated_dir="${prompts_dir}/generated"
    log_event "cleanup" "Cleaning up results for feature: ${feature}"

    # Remove result files for this feature (both naming conventions)
    rm -f "${RESULTS_DIR}/${feature}-"*.result 2>/dev/null
    rm -f "${RESULTS_DIR}/"*"-${feature}.result" 2>/dev/null
    rm -f "${RESULTS_DIR}/${feature}-"*.pid 2>/dev/null

    # Remove generated prompts for this feature
    rm -f "${generated_dir}/${feature}-"*.prompt 2>/dev/null

    # Remove old naming convention files (feature-role.md)
    rm -f "${RESULTS_DIR}/business-analyst-${feature}.md" 2>/dev/null
    rm -f "${RESULTS_DIR}/tech-lead-${feature}.md" 2>/dev/null
    rm -f "${RESULTS_DIR}/architect-${feature}.md" 2>/dev/null

    # Remove stale .pid files older than 1 hour
    find "${RESULTS_DIR}" -name "*.pid" -mmin +60 -delete 2>/dev/null

    # Remove stale generated prompts older than 30 minutes
    find "${generated_dir}" -name "*.prompt" -mmin +30 -delete 2>/dev/null

    log_event "cleanup" "Results cleaned for feature: ${feature}"
}

# ── Handle MERGED State ─────────────────────────────────────────────
handle_merged() {
    local feature
    feature=$(get_feature)

    log_event "state" "State: MERGED — pipeline complete for: ${feature}"
    "$FEATURE_MANAGER" complete "$feature" "$(get_pipeline_id)"
    "$NOTIFY" pipeline-completed "$feature" "$(get_pipeline_id)" ""

    # Cleanup results for completed feature
    cleanup_results "$feature"

    "$STATE_MANAGER" reset
}

# ── Handle RESUMED State ───────────────────────────────────────────
handle_resumed() {
    local feature
    feature=$(get_feature)

    log_event "state" "State: RESUMED — resuming from Architect for: ${feature}"
    "$STATE_MANAGER" transition-state "ARCHITECT"
}

# ── Main Loop ───────────────────────────────────────────────────────
main_loop() {
    log_event "engine" "Graph Loop Engine started (PID: $$)"
    log_event "engine" "Poll interval: ${POLL_INTERVAL}s"

    # Write PID file
    echo $$ > "${SCRIPT_DIR}/engine.pid"

    # Run recovery policy on startup
    handle_recovery

    # Cleanup stale files from previous runs
    log_event "cleanup" "Startup cleanup — removing stale files"
    local generated_dir="${SCRIPT_DIR}/prompts/generated"
    # Remove dead PID files
    for pid_file in "${RESULTS_DIR}"/*.pid; do
        [ -f "$pid_file" ] || continue
        pid=$(cat "$pid_file" 2>/dev/null)
        if ! kill -0 "$pid" 2>/dev/null; then
            rm -f "$pid_file"
        fi
    done
    # Remove old naming convention files
    rm -f "${RESULTS_DIR}"/business-analyst-*.md 2>/dev/null
    rm -f "${RESULTS_DIR}"/tech-lead-*.md 2>/dev/null
    rm -f "${RESULTS_DIR}"/architect-*.md 2>/dev/null
    # Remove result files for completed features (both naming conventions)
    for feature in $(jq -r '.features | to_entries[] | select(.value.status == "DONE") | .key' "$FEATURES_FILE" 2>/dev/null); do
        rm -f "${RESULTS_DIR}/${feature}-"*.result 2>/dev/null
        rm -f "${RESULTS_DIR}/"*"-${feature}.result" 2>/dev/null
        rm -f "${generated_dir}/${feature}-"*.prompt 2>/dev/null
    done
    # Remove stale generated prompts older than 30 minutes
    find "${generated_dir}" -name "*.prompt" -mmin +30 -delete 2>/dev/null
    log_event "cleanup" "Startup cleanup complete"

    while true; do
        local current_state
        current_state=$(get_state)
        echo "[engine] $(date -Iseconds) State: $current_state"

        case "$current_state" in
            IDLE)                handle_idle ;;
            DISPATCHING)         handle_dispatching ;;
            ANALYZING)           handle_analyzing ;;
            ARCHITECT)           handle_architect ;;
            DEVELOPER)           handle_developer ;;
            QUALITY-ANALYSIS)    handle_quality_analysis ;;
            MERGED)              handle_merged ;;
            FAILED)              handle_failed ;;
            BLOCKED)             handle_blocked ;;
            TIMEOUT)             handle_timeout ;;
            WAITING_HUMAN)       handle_waiting_human ;;
            ESCALATED)           handle_escalated ;;
            SKIPPED)             handle_blocked ;;
            RESUMED)             handle_resumed ;;
            *)
                log_event "engine" "Unknown state: ${current_state}"
                "$STATE_MANAGER" reset
                ;;
        esac

        sleep "$POLL_INTERVAL"
    done
}

# ── Signal Handlers ─────────────────────────────────────────────────
cleanup() {
    log_event "engine" "Engine stopping (PID: $$)"
    rm -f "${SCRIPT_DIR}/engine.pid"
    exit 0
}

trap cleanup SIGINT SIGTERM

# ── Main ────────────────────────────────────────────────────────────
main() {
    local command=${1:-}
    shift || true

    case $command in
        start)
            main_loop
            ;;
        once)
            handle_recovery
            local current_state
            current_state=$(get_state)
            log_event "engine" "Single cycle — state: ${current_state}"
            case "$current_state" in
                IDLE)             handle_idle ;;
                DISPATCHING)      handle_dispatching ;;
                ANALYZING)        handle_analyzing ;;
                ARCHITECT)        handle_architect ;;
                DEVELOPER)        handle_developer ;;
                QUALITY-ANALYSIS) handle_quality_analysis ;;
                MERGED)           handle_merged ;;
                FAILED)           handle_failed ;;
                BLOCKED)          handle_blocked ;;
                TIMEOUT)          handle_timeout ;;
                WAITING_HUMAN)    handle_waiting_human ;;
                ESCALATED)        handle_escalated ;;
                RESUMED)          handle_resumed ;;
            esac
            ;;
        recover)
            handle_recovery
            ;;
        status)
            echo "State: $(get_state)"
            echo "Feature: $(get_feature)"
            echo "Pipeline: $(get_pipeline_id)"
            echo "Correlation: $(get_correlation_id)"
            echo "Rejection loop: $(get_rejection_counter)/$MAX_REJECTION_LOOPS"
            echo "Pipeline iteration: $(get_pipeline_counter)/$MAX_PIPELINE_ITERATIONS"
            ;;
        *)
            echo "Usage: engine.sh {start|once|recover|status}"
            exit 1
            ;;
    esac
}

main "$@"
