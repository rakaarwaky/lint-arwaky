#!/bin/bash
# Dashboard for Graph Loop Pipeline (Revision 2.0)
# FIX: Full names used (Business-Analyst, Tech-Lead, Quality-Analysis)
# FIX: Counter display added (rejection loop + pipeline iteration per feature)
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
STATE_FILE="${SCRIPT_DIR}/state.json"
FEATURES_FILE="${SCRIPT_DIR}/features.json"
LOG_FILE="${SCRIPT_DIR}/execution.log"
HEALTH_LOG="${SCRIPT_DIR}/health.log"

# ── Colors ──────────────────────────────────────────────────────────
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
CYAN='\033[0;36m'
NC='\033[0m'

# ── Print Header ────────────────────────────────────────────────────
print_header() {
    echo ""
    echo -e "${CYAN}╔══════════════════════════════════════════════════════════════╗${NC}"
    echo -e "${CYAN}║         GRAPH LOOP PIPELINE DASHBOARD (v2.0)                ║${NC}"
    echo -e "${CYAN}╚══════════════════════════════════════════════════════════════╝${NC}"
    echo ""
}

# ── Print Pipeline Status ──────────────────────────────────────────
print_pipeline_status() {
    echo -e "${BLUE}━━━ Pipeline Status ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"

    if [ ! -f "$STATE_FILE" ]; then
        echo -e "${RED}State file not found${NC}"
        return
    fi

    local current_state feature pipeline_id started_at correlation_id
    current_state=$(jq -r '.pipeline.current_state' "$STATE_FILE")
    feature=$(jq -r '.pipeline.feature // "none"' "$STATE_FILE")
    pipeline_id=$(jq -r '.pipeline.id // "none"' "$STATE_FILE")
    started_at=$(jq -r '.pipeline.started_at // "none"' "$STATE_FILE")
    correlation_id=$(jq -r '.pipeline.correlation_id // "none"' "$STATE_FILE")

    echo -e "  State:         ${GREEN}$current_state${NC}"
    echo -e "  Feature:       $feature"
    echo -e "  Pipeline:      $pipeline_id"
    echo -e "  Correlation:   $correlation_id"
    echo -e "  Started:       $started_at"

    # FIX: Display counters
    local rejection_counter pipeline_counter
    rejection_counter=$(jq -r '.pipeline.rejection_loop_counter // 0' "$STATE_FILE")
    pipeline_counter=$(jq -r '.pipeline.pipeline_iteration_counter // 0' "$STATE_FILE")

    echo -e "  Rejection:     ${YELLOW}${rejection_counter}/3${NC}"
    echo -e "  Iteration:     ${YELLOW}${pipeline_counter}/5${NC}"
    echo ""
}

# ── Print Node Status ──────────────────────────────────────────────
print_node_status() {
    echo -e "${BLUE}━━━ Node Status ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"

    if [ ! -f "$STATE_FILE" ]; then
        echo -e "${RED}State file not found${NC}"
        return
    fi

    # Business-Analyst
    local ba_status ba_report
    ba_status=$(jq -r '.pipeline.parallel_nodes["business-analyst"].status' "$STATE_FILE")
    ba_report=$(jq -r '.pipeline.parallel_nodes["business-analyst"].report_file // "none"' "$STATE_FILE")

    if [ "$ba_status" = "completed" ]; then
        echo -e "  Business-Analyst:   ${GREEN}COMPLETED${NC}  → $ba_report"
    elif [ "$ba_status" = "running" ]; then
        echo -e "  Business-Analyst:   ${YELLOW}RUNNING${NC}"
    else
        echo -e "  Business-Analyst:   ${RED}IDLE${NC}"
    fi

    # Tech-Lead
    local tl_status tl_report
    tl_status=$(jq -r '.pipeline.parallel_nodes["tech-lead"].status' "$STATE_FILE")
    tl_report=$(jq -r '.pipeline.parallel_nodes["tech-lead"].report_file // "none"' "$STATE_FILE")

    if [ "$tl_status" = "completed" ]; then
        echo -e "  Tech-Lead:          ${GREEN}COMPLETED${NC}  → $tl_report"
    elif [ "$tl_status" = "running" ]; then
        echo -e "  Tech-Lead:          ${YELLOW}RUNNING${NC}"
    else
        echo -e "  Tech-Lead:          ${RED}IDLE${NC}"
    fi

    # Architect
    local current_state
    current_state=$(jq -r '.pipeline.current_state' "$STATE_FILE")

    if [ "$current_state" = "ARCHITECT" ]; then
        echo -e "  Architect:          ${YELLOW}MERGING${NC}"
    elif [[ "$current_state" == "DEVELOPER" || "$current_state" == "QUALITY-ANALYSIS" || "$current_state" == "MERGED" ]]; then
        echo -e "  Architect:          ${GREEN}DONE${NC}"
    else
        echo -e "  Architect:          ${RED}WAITING${NC}"
    fi

    # Developer
    if [ "$current_state" = "DEVELOPER" ]; then
        echo -e "  Developer:          ${YELLOW}EXECUTING${NC}"
    elif [[ "$current_state" == "QUALITY-ANALYSIS" || "$current_state" == "MERGED" ]]; then
        echo -e "  Developer:          ${GREEN}DONE${NC}"
    else
        echo -e "  Developer:          ${RED}WAITING${NC}"
    fi

    # Quality-Analysis
    if [ "$current_state" = "QUALITY-ANALYSIS" ]; then
        echo -e "  Quality-Analysis:   ${YELLOW}REVIEWING${NC}"
    elif [ "$current_state" = "MERGED" ]; then
        echo -e "  Quality-Analysis:   ${GREEN}APPROVED${NC}"
    else
        echo -e "  Quality-Analysis:   ${RED}WAITING${NC}"
    fi

    echo ""
}

# ── Print Feature Queue ────────────────────────────────────────────
print_feature_queue() {
    echo -e "${BLUE}━━━ Feature Queue ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"

    if [ ! -f "$FEATURES_FILE" ]; then
        echo -e "${RED}Features file not found${NC}"
        return
    fi

    local total
    total=$(jq '.features | length' "$FEATURES_FILE")
    echo -e "  Total features: $total"
    echo ""

    for status in PENDING LOCKED ACTIVE DONE FAILED BLOCKED; do
        local count
        count=$(jq -r "[.features[] | select(.status == \"$status\")] | length" "$FEATURES_FILE")
        if [ "$count" -gt 0 ]; then
            case "$status" in
                PENDING) echo -e "  ${YELLOW}PENDING:${NC} $count" ;;
                LOCKED)  echo -e "  ${BLUE}LOCKED:${NC}  $count" ;;
                ACTIVE)  echo -e "  ${GREEN}ACTIVE:${NC}  $count" ;;
                DONE)    echo -e "  ${GREEN}DONE:${NC}    $count" ;;
                FAILED)  echo -e "  ${RED}FAILED:${NC}  $count" ;;
                BLOCKED) echo -e "  ${RED}BLOCKED:${NC} $count" ;;
            esac
        fi
    done

    echo ""
}

# ── Print Recent Activity ──────────────────────────────────────────
print_recent_activity() {
    echo -e "${BLUE}━━━ Recent Activity (last 10 events) ━━━━━━━━━━━━━━━━━━━━━━━━${NC}"

    if [ ! -f "$LOG_FILE" ]; then
        echo -e "${RED}Log file not found${NC}"
        return
    fi

    tail -10 "$LOG_FILE" | while IFS= read -r line; do
        echo -e "  $line"
    done

    echo ""
}

# ── Print Metrics ──────────────────────────────────────────────────
print_metrics() {
    echo -e "${BLUE}━━━ Metrics ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"

    if [ ! -f "$FEATURES_FILE" ]; then
        return
    fi

    local completed failed total
    completed=$(jq '[.features[] | select(.status == "DONE")] | length' "$FEATURES_FILE")
    failed=$(jq '[.features[] | select(.status == "FAILED" or .status == "BLOCKED")] | length' "$FEATURES_FILE")
    total=$(jq '.features | length' "$FEATURES_FILE")

    echo -e "  Completed:    $completed"
    echo -e "  Failed:       $failed"
    echo -e "  Total:        $total"

    if [ "$total" -gt 0 ]; then
        local success_rate
        success_rate=$(echo "scale=1; $completed * 100 / $total" | bc 2>/dev/null || echo "0")
        echo -e "  Success rate: ${success_rate}%"
    fi

    echo ""
}

# ── Main ────────────────────────────────────────────────────────────
main() {
    local command=${1:-full}
    shift || true

    case $command in
        full)
            print_header
            print_pipeline_status
            print_node_status
            print_feature_queue
            print_recent_activity
            print_metrics
            ;;
        status)   print_pipeline_status ;;
        nodes)    print_node_status ;;
        features) print_feature_queue ;;
        activity) print_recent_activity ;;
        metrics)  print_metrics ;;
        *)
            echo "Usage: dashboard.sh {full|status|nodes|features|activity|metrics}"
            exit 1
            ;;
    esac
}

main "$@"
