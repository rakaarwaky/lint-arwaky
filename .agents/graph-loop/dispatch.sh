#!/bin/bash
# Dispatch Agent for Graph Loop Pipeline (Revision 2.0)
# FIX: FRD hash computed and injected
# FIX: Correlation ID injected into prompts
# FIX: Quality-Analysis receives all 6 inputs
# FIX: Skip Report passed to Architect
# FIX: Full names used
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROMPTS_DIR="${SCRIPT_DIR}/prompts"
TEMPLATES_DIR="${PROMPTS_DIR}/templates"
GENERATED_DIR="${PROMPTS_DIR}/generated"
CONFIG_READER="${SCRIPT_DIR}/config_reader.sh"
STATE_MANAGER="${SCRIPT_DIR}/state_manager.sh"
LOG_FILE="${SCRIPT_DIR}/execution.log"

# ── Log Event ───────────────────────────────────────────────────────
log_event() {
    local event_type=$1
    local message=$2
    local timestamp=$(date -Iseconds)
    echo "[$timestamp] [dispatch:$event_type] $message" >> "$LOG_FILE"
}

# ── Read Template File ──────────────────────────────────────────────
read_template() {
    local template_name=$1
    local template_file="${TEMPLATES_DIR}/${template_name}.txt"

    if [ ! -f "$template_file" ]; then
        echo "Template not found: $template_file" >&2
        return 1
    fi
    cat "$template_file"
}

# ── Replace Placeholder ─────────────────────────────────────────────
replace_placeholder() {
    local template=$1
    local placeholder=$2
    local value=$3
    echo "$template" | sed "s|${placeholder}|${value}|g"
}

# ── Get State Values ────────────────────────────────────────────────
get_correlation_id() {
    jq -r '.pipeline.correlation_id // "unknown"' "${SCRIPT_DIR}/state.json"
}

get_pipeline_iteration() {
    jq -r '.pipeline.pipeline_iteration_counter // 0' "${SCRIPT_DIR}/state.json"
}

get_rejection_loop() {
    jq -r '.pipeline.rejection_loop_counter // 0' "${SCRIPT_DIR}/state.json"
}

# ── Build Feature Context ──────────────────────────────────────────
# FIX: Includes FRD hash, correlation ID, counters
build_feature_context() {
    local feature_name=$1
    local correlation_id
    local pipeline_iteration
    local rejection_loop

    correlation_id=$(get_correlation_id)
    pipeline_iteration=$(get_pipeline_iteration)
    rejection_loop=$(get_rejection_loop)

    "$CONFIG_READER" feature-context "$feature_name" "$correlation_id" "$pipeline_iteration" "$rejection_loop"
}

# ── Dispatch Business-Analyst Agent ─────────────────────────────────
dispatch_business_analyst() {
    local feature=$1
    local feature_path=$2
    local frd_path=$3

    local context
    context=$(build_feature_context "$feature")

    local template
    template=$(read_template "business-analyst")

    template=$(replace_placeholder "$template" "{{FEATURE}}" "$feature")
    template=$(replace_placeholder "$template" "{{FEATURE_PATH}}" "$feature_path")
    template=$(replace_placeholder "$template" "{{FRD_PATH}}" "$frd_path")
    template=$(replace_placeholder "$template" "{{DATE}}" "$(date +%Y%m%d)")

    echo "${context}

${template}"
}

# ── Dispatch Tech-Lead Agent ────────────────────────────────────────
dispatch_tech_lead() {
    local feature=$1
    local feature_path=$2
    local frd_path=$3

    local context
    context=$(build_feature_context "$feature")

    local template
    template=$(read_template "tech-lead")

    template=$(replace_placeholder "$template" "{{FEATURE}}" "$feature")
    template=$(replace_placeholder "$template" "{{FEATURE_PATH}}" "$feature_path")
    template=$(replace_placeholder "$template" "{{FRD_PATH}}" "$frd_path")
    template=$(replace_placeholder "$template" "{{DATE}}" "$(date +%Y%m%d)")

    echo "${context}

${template}"
}

# ── Dispatch Architect Agent ────────────────────────────────────────
# FIX: Receives Skip Report as parameter
dispatch_architect() {
    local feature=$1
    local feature_path=$2
    local frd_path=$3
    local ba_report=$4
    local tl_report=$5
    local skip_report=${6:-""}

    local context
    context=$(build_feature_context "$feature")

    local correlation_id
    correlation_id=$(get_correlation_id)

    local template
    template=$(read_template "architect")

    template=$(replace_placeholder "$template" "{{FEATURE}}" "$feature")
    template=$(replace_placeholder "$template" "{{FEATURE_PATH}}" "$feature_path")
    template=$(replace_placeholder "$template" "{{FRD_PATH}}" "$frd_path")
    template=$(replace_placeholder "$template" "{{BA_REPORT}}" "$ba_report")
    template=$(replace_placeholder "$template" "{{TL_REPORT}}" "$tl_report")
    template=$(replace_placeholder "$template" "{{SKIP_REPORT}}" "$skip_report")
    template=$(replace_placeholder "$template" "{{CORRELATION_ID}}" "$correlation_id")
    template=$(replace_placeholder "$template" "{{DATE}}" "$(date +%Y%m%d)")

    echo "${context}

${template}"
}

# ── Dispatch Developer Agent ────────────────────────────────────────
dispatch_developer() {
    local feature=$1
    local feature_path=$2
    local frd_path=$3
    local merged_plan=$4

    local context
    context=$(build_feature_context "$feature")

    local template
    template=$(read_template "developer")

    template=$(replace_placeholder "$template" "{{FEATURE}}" "$feature")
    template=$(replace_placeholder "$template" "{{FEATURE_PATH}}" "$feature_path")
    template=$(replace_placeholder "$template" "{{FRD_PATH}}" "$frd_path")
    template=$(replace_placeholder "$template" "{{MERGED_PLAN}}" "$merged_plan")
    template=$(replace_placeholder "$template" "{{DATE}}" "$(date +%Y%m%d)")

    echo "${context}

${template}"
}

# ── Dispatch Quality-Analysis Agent ─────────────────────────────────
# FIX: Receives all 6 inputs per DESIGN.md
dispatch_quality_analysis() {
    local feature=$1
    local pr_number=$2
    local merged_plan=$3
    local frd_path=$4
    local ba_report=$5
    local tl_report=$6
    local dev_report=$7
    local qa_mode=${8:-"full-review"}

    local context
    context=$(build_feature_context "$feature")

    local template
    template=$(read_template "quality-analysis")

    template=$(replace_placeholder "$template" "{{FEATURE}}" "$feature")
    template=$(replace_placeholder "$template" "{{PR_NUMBER}}" "$pr_number")
    template=$(replace_placeholder "$template" "{{MERGED_PLAN}}" "$merged_plan")
    template=$(replace_placeholder "$template" "{{FRD_PATH}}" "$frd_path")
    template=$(replace_placeholder "$template" "{{BA_REPORT}}" "$ba_report")
    template=$(replace_placeholder "$template" "{{TL_REPORT}}" "$tl_report")
    template=$(replace_placeholder "$template" "{{DEV_REPORT}}" "$dev_report")
    template=$(replace_placeholder "$template" "{{QA_MODE}}" "$qa_mode")
    template=$(replace_placeholder "$template" "{{DATE}}" "$(date +%Y%m%d)")

    echo "${context}

${template}"
}

# ── Run Agent ───────────────────────────────────────────────────────
run_agent() {
    local node=$1
    local prompt=$2
    local output_file=$3
    local timeout_minutes=${4:-30}

    log_event "run" "Running agent: $node (timeout: ${timeout_minutes}m)"

    local project_root
    project_root=$("$CONFIG_READER" project-root)

    local timeout_seconds=$((timeout_minutes * 60))
    local start_time=$(date +%s)
    local exit_code=0

    (cd "$project_root" && timeout "$timeout_seconds" qwen -p "$prompt" -o text > "$output_file" 2>> "$LOG_FILE") || exit_code=$?

    local end_time=$(date +%s)
    local duration=$(( (end_time - start_time) / 60 ))

    if [ $exit_code -eq 124 ]; then
        log_event "timeout" "Agent $node timed out after ${timeout_minutes}m"
        return 124
    elif [ $exit_code -eq 0 ]; then
        log_event "complete" "Agent $node completed in ${duration}m"
    else
        log_event "error" "Agent $node failed with exit code $exit_code"
    fi

    return $exit_code
}

# ── Main ────────────────────────────────────────────────────────────
main() {
    local command=${1:-}
    shift || true

    case $command in
        business-analyst)
            dispatch_business_analyst "$1" "$2" "$3"
            ;;
        tech-lead)
            dispatch_tech_lead "$1" "$2" "$3"
            ;;
        architect)
            dispatch_architect "$1" "$2" "$3" "$4" "$5" "${6:-}"
            ;;
        developer)
            dispatch_developer "$1" "$2" "$3" "$4"
            ;;
        quality-analysis)
            dispatch_quality_analysis "$1" "$2" "$3" "$4" "$5" "$6" "$7" "${8:-full-review}"
            ;;
        run)
            run_agent "$1" "$2" "$3" "${4:-30}"
            ;;
        *)
            echo "Usage: dispatch.sh {business-analyst|tech-lead|architect|developer|quality-analysis|run}"
            exit 1
            ;;
    esac
}

main "$@"
