#!/bin/bash
# Trigger Scanner for Graph Loop Pipeline (Revision 2.0)
# FIX: Bot event filter implemented
# FIX: Correlation ID check implemented
# FIX: Idempotency includes DONE status
# FIX: Absolute paths used
# FIX: Full names used
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
STATE_MANAGER="${SCRIPT_DIR}/state_manager.sh"
FEATURE_MANAGER="${SCRIPT_DIR}/feature_manager.sh"
CONFIG_READER="${SCRIPT_DIR}/config_reader.sh"
CONDITIONAL_SKIP="${SCRIPT_DIR}/conditional_skip.sh"
PARALLEL_DISPATCHER="${SCRIPT_DIR}/parallel_dispatcher.sh"
STATE_FILE="${SCRIPT_DIR}/state.json"
LOG_FILE="${SCRIPT_DIR}/execution.log"
DEBOUNCE_FILE="${SCRIPT_DIR}/locks/.debounce"

# ── Log Event ───────────────────────────────────────────────────────
log_event() {
    local event_type=$1
    local message=$2
    local timestamp=$(date -Iseconds)
    echo "[$timestamp] [scanner:$event_type] $message" >> "$LOG_FILE"
}

# ── Get Debounce Seconds from Config ────────────────────────────────
get_debounce_seconds() {
    "$CONFIG_READER" trigger-guards "debounce_seconds" "30"
}

# ── Check if Sender is Bot ─────────────────────────────────────────
# FIX: Bot event filter implemented
is_bot_author() {
    local pr_number=$1
    local ignore_bots
    ignore_bots=$("$CONFIG_READER" trigger-guards "ignore_bot_events" "true")

    if [ "$ignore_bots" != "true" ]; then
        return 1
    fi

    local author
    author=$(gh pr view "$pr_number" --json author --jq '.author.login' 2>/dev/null || echo "")

    # List of known bot patterns
    local bot_patterns=("github-actions" "dependabot" "renovate" "greenkeeper" "snyk-bot" "codecov")
    for pattern in "${bot_patterns[@]}"; do
        if [[ "$author" == *"$pattern"* ]]; then
            log_event "bot_filter" "PR #$pr_number authored by bot ($author) — ignoring"
            return 0
        fi
    done

    # Check if author type is Bot via GitHub API
    local author_type
    author_type=$(gh api "/users/$author" --jq '.type' 2>/dev/null || echo "User")
    if [ "$author_type" = "Bot" ]; then
        log_event "bot_filter" "PR #$pr_number authored by Bot type ($author) — ignoring"
        return 0
    fi

    return 1
}

# ── Check Correlation ID Guard ─────────────────────────────────────
# FIX: Correlation ID check implemented
check_correlation_guard() {
    local current_state
    current_state=$(jq -r '.pipeline.current_state' "$STATE_FILE")

    if [ "$current_state" != "IDLE" ]; then
        log_event "guard" "State is $current_state (not IDLE) — queuing trigger"
        return 1
    fi

    return 0
}

# ── Trigger Debounce ────────────────────────────────────────────────
check_debounce() {
    local trigger_key=$1
    local debounce_seconds
    debounce_seconds=$(get_debounce_seconds)

    mkdir -p "$(dirname "$DEBOUNCE_FILE")"

    if [ -f "$DEBOUNCE_FILE" ]; then
        local last_time
        last_time=$(jq -r ".[\"$trigger_key\"] // 0" "$DEBOUNCE_FILE" 2>/dev/null || echo "0")
        local now=$(date +%s)
        local elapsed=$((now - last_time))

        if [ "$elapsed" -lt "$debounce_seconds" ]; then
            log_event "debounce" "Trigger '$trigger_key' debounced (${elapsed}s < ${debounce_seconds}s)"
            return 1
        fi
    fi

    # Record trigger time
    local now=$(date +%s)
    if [ -f "$DEBOUNCE_FILE" ]; then
        jq --arg key "$trigger_key" --argjson ts "$now" '.[$key] = $ts' "$DEBOUNCE_FILE" > "${DEBOUNCE_FILE}.tmp" && mv "${DEBOUNCE_FILE}.tmp" "$DEBOUNCE_FILE"
    else
        echo "{\"$trigger_key\": $now}" > "$DEBOUNCE_FILE"
    fi

    return 0
}

# ── Scan GitHub PRs ─────────────────────────────────────────────────
scan_github_prs() {
    log_event "scan" "Scanning GitHub for PRs with 'need review' label..."

    local prs
    prs=$(gh pr list --label "need review" --json number,title,headRefName 2>/dev/null || echo "[]")

    if [ "$prs" = "[]" ] || [ -z "$prs" ]; then
        log_event "scan" "No PRs found with 'need review' label"
        return 1
    fi

    echo "$prs" | jq -r '.[] | "\(.number)|\(.title)|\(.headRefName)"'
}

# ── Extract Feature from PR ─────────────────────────────────────────
extract_feature_from_pr() {
    local pr_number=$1
    local pr_title=$2
    local pr_branch=$3
    local feature=""

    if [[ "$pr_branch" =~ ^worktree-(.+)$ ]]; then
        feature="${BASH_REMATCH[1]}"
    elif [[ "$pr_branch" =~ ^(feature|fix|hotfix)/(.+)$ ]]; then
        feature="${BASH_REMATCH[2]}"
    else
        feature=$(echo "$pr_title" | tr '[:upper:]' '[:lower:]' | sed 's/[^a-z0-9-]/-/g' | sed 's/--*/-/g' | sed 's/^-//;s/-$//')
    fi

    echo "$feature"
}

# ── Get Feature Folder (absolute path) ─────────────────────────────
# FIX: Use absolute paths
get_feature_folder() {
    local feature=$1
    local project_root
    project_root=$("$CONFIG_READER" project-root)

    if [ -d "${project_root}/crates/${feature}" ]; then
        echo "${project_root}/crates/${feature}"
    elif [ -d "${project_root}/modules/${feature}" ]; then
        echo "${project_root}/modules/${feature}"
    elif [ -d "${project_root}/packages/${feature}" ]; then
        echo "${project_root}/packages/${feature}"
    else
        echo ""
    fi
}

# ── Check if Feature Already Claimed ────────────────────────────────
# FIX: Includes DONE check for idempotency
is_feature_claimed() {
    local feature=$1
    local status
    status=$("$FEATURE_MANAGER" status "$feature" 2>/dev/null || echo "NOT_FOUND")

    case "$status" in
        LOCKED|ACTIVE)
            return 0  # Already claimed
            ;;
        DONE)
            # Check cooldown for idempotency
            "$FEATURE_MANAGER" check-cooldown "$feature" > /dev/null 2>&1
            if [ $? -ne 0 ]; then
                return 0  # Still in cooldown
            fi
            return 1  # Cooldown passed, can re-process
            ;;
        *)
            return 1
            ;;
    esac
}

# ── Handle New PR Trigger ───────────────────────────────────────────
handle_pr_created() {
    local pr_number=$1
    local pr_title=$2
    local pr_branch=$3

    log_event "trigger" "Handling new PR: #${pr_number} - ${pr_title}"

    # Guard 1: Debounce
    local debounce_key="pr-${pr_number}"
    if ! check_debounce "$debounce_key"; then
        return 0
    fi

    # Guard 2: Bot filter
    if is_bot_author "$pr_number"; then
        return 0
    fi

    # Guard 3: State guard (correlation ID)
    if ! check_correlation_guard; then
        return 0
    fi

    # Extract feature name
    local feature
    feature=$(extract_feature_from_pr "$pr_number" "$pr_title" "$pr_branch")

    if [ -z "$feature" ]; then
        log_event "trigger" "Could not extract feature name from PR #${pr_number}"
        return 1
    fi

    # Guard 4: Idempotency check
    if is_feature_claimed "$feature"; then
        log_event "trigger" "Feature ${feature} already claimed or in cooldown — skipping"
        return 1
    fi

    # Get feature folder (absolute path)
    local feature_folder
    feature_folder=$(get_feature_folder "$feature")

    if [ -z "$feature_folder" ]; then
        log_event "trigger" "Feature folder not found for: ${feature}"
        return 1
    fi

    # Get FRD path
    local frd_path="${feature_folder}/FRD.md"
    if [ ! -f "$frd_path" ]; then
        log_event "trigger" "FRD not found: ${frd_path}"
        return 1
    fi

    # Claim feature
    local pipeline_info
    pipeline_info=$("$STATE_MANAGER" start "$feature")
    local pipeline_id correlation_id
    pipeline_id=$(echo "$pipeline_info" | cut -d'|' -f1)
    correlation_id=$(echo "$pipeline_info" | cut -d'|' -f2)

    "$FEATURE_MANAGER" claim "$feature" "$feature_folder" "$pipeline_id"

    # FIX: Track correlation ID on PR (label + comment)
    gh pr edit "$pr_number" --add-label "corr:${correlation_id}" 2>/dev/null || true
    gh pr comment "$pr_number" --body "🔗 Correlation ID: \`${correlation_id}\` | Pipeline: \`${pipeline_id}\`" 2>/dev/null || true

    # Dispatch parallel analysis
    log_event "trigger" "Dispatching Business-Analyst and Tech-Lead for feature: ${feature}"
    "$PARALLEL_DISPATCHER" parallel-analysis "$feature" "$feature_folder" "$frd_path"

    log_event "trigger" "Pipeline started: $pipeline_id (correlation: $correlation_id)"
}

# ── Check Parallel Completion ───────────────────────────────────────
check_parallel_completion() {
    log_event "scan" "Checking parallel completion..."

    local ba_status tl_status
    ba_status=$(jq -r '.pipeline.parallel_nodes["business-analyst"].status' "$STATE_FILE")
    tl_status=$(jq -r '.pipeline.parallel_nodes["tech-lead"].status' "$STATE_FILE")

    if [ "$ba_status" = "completed" ] && [ "$tl_status" = "completed" ]; then
        log_event "scan" "Both Business-Analyst and Tech-Lead completed — transitioning to ARCHITECT"
        "$STATE_MANAGER" transition-state "ARCHITECT"
        return 0
    fi

    log_event "scan" "Waiting: Business-Analyst=$ba_status, Tech-Lead=$tl_status"
    return 1
}

# ── Handle Report Written ──────────────────────────────────────────
handle_report_written() {
    local feature=$1
    local role=$2
    local report_file=$3

    log_event "trigger" "Report written: ${role} for ${feature}"

    "$STATE_MANAGER" complete "$role" "$report_file"
    check_parallel_completion
}

# ── Main ────────────────────────────────────────────────────────────
main() {
    mkdir -p "${SCRIPT_DIR}/locks"

    local command=${1:-}
    shift || true

    case $command in
        scan-prs)
            scan_github_prs
            ;;
        handle-pr)
            handle_pr_created "$1" "$2" "$3"
            ;;
        check-parallel)
            check_parallel_completion
            ;;
        report-written)
            handle_report_written "$1" "$2" "$3"
            ;;
        *)
            echo "Usage: trigger_scanner.sh {scan-prs|handle-pr|check-parallel|report-written}"
            exit 1
            ;;
    esac
}

main "$@"
