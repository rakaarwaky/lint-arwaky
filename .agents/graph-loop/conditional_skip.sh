#!/bin/bash
# Conditional Skip for Graph Loop Pipeline (Revision 2.0)
# FIX: Quality-Analysis is NEVER skipped
# FIX: Full names used (Business-Analyst, Tech-Lead, Quality-Analysis)
# FIX: Skip Report generation added
# FIX: Color variables defined
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
CONFIG_READER="${SCRIPT_DIR}/config_reader.sh"
LOG_FILE="${SCRIPT_DIR}/execution.log"

# ── Colors ──────────────────────────────────────────────────────────
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m'

# ── Log Event ───────────────────────────────────────────────────────
log_event() {
    local event_type=$1
    local message=$2
    local timestamp=$(date -Iseconds)
    echo "[$timestamp] [skip:$event_type] $message" >> "$LOG_FILE"
}

# ── Analyze Feature Complexity ──────────────────────────────────────
analyze_complexity() {
    local feature_folder=$1

    if [ ! -d "$feature_folder" ]; then
        echo "unknown"
        return
    fi

    local file_count
    file_count=$(find "$feature_folder" -name "*.rs" -o -name "*.py" -o -name "*.ts" 2>/dev/null | wc -l)

    local loc
    loc=$(find "$feature_folder" \( -name "*.rs" -o -name "*.py" -o -name "*.ts" \) -exec cat {} + 2>/dev/null | wc -l || echo "0")

    if [ "$file_count" -le 3 ] && [ "$loc" -le 200 ]; then
        echo "simple"
    elif [ "$file_count" -le 10 ] && [ "$loc" -le 1000 ]; then
        echo "medium"
    else
        echo "complex"
    fi
}

# ── Should Skip Business-Analyst ────────────────────────────────────
should_skip_business_analyst() {
    local feature_folder=$1
    local pr_title=${2:-""}

    if [[ "$pr_title" =~ ^(hotfix|fix|typo|docs|readme|changelog) ]]; then
        log_event "skip" "Skipping Business-Analyst for simple fix: $pr_title"
        return 0
    fi

    local complexity
    complexity=$(analyze_complexity "$feature_folder")
    if [ "$complexity" = "simple" ]; then
        log_event "skip" "Skipping Business-Analyst for simple feature (complexity: $complexity)"
        return 0
    fi

    return 1
}

# ── Should Skip Tech-Lead ──────────────────────────────────────────
should_skip_tech_lead() {
    local feature_folder=$1
    local pr_title=${2:-""}

    # Never skip Tech-Lead for security-related changes
    if [[ "$pr_title" =~ (security|auth|token|password|encrypt) ]]; then
        log_event "skip" "Never skip Tech-Lead for security: $pr_title"
        return 1
    fi

    # Skip Tech-Lead for doc-only updates
    if [[ "$pr_title" =~ ^(docs|readme|changelog) ]]; then
        local code_files
        code_files=$(find "$feature_folder" \( -name "*.rs" -o -name "*.py" -o -name "*.ts" \) 2>/dev/null | wc -l)
        if [ "$code_files" -eq 0 ]; then
            log_event "skip" "Skipping Tech-Lead for doc-only update"
            return 0
        fi
    fi

    return 1
}

# ── Should Skip Architect ───────────────────────────────────────────
should_skip_architect() {
    # NEVER SKIP — merge gate required
    return 1
}

# ── Should Skip Developer ───────────────────────────────────────────
should_skip_developer() {
    # NEVER SKIP — implements changes
    return 1
}

# ── Should Skip Quality-Analysis ───────────────────────────────────
# FIX: NEVER SKIP. For doc-only, return special mode instead.
should_skip_quality_analysis() {
    # Quality-Analysis is NEVER skipped per DESIGN.md Revision 2.0
    return 1
}

# ── Get Quality-Analysis Mode ───────────────────────────────────────
# FIX: For doc-only, QA runs in auto-approve mode with MINOR severity
get_quality_analysis_mode() {
    local feature_folder=$1
    local pr_title=${2:-""}

    if [[ "$pr_title" =~ ^(docs|readme|changelog) ]]; then
        local code_files
        code_files=$(find "$feature_folder" \( -name "*.rs" -o -name "*.py" -o -name "*.ts" \) 2>/dev/null | wc -l)
        if [ "$code_files" -eq 0 ]; then
            echo "auto-approve-minor"
            return
        fi
    fi
    echo "full-review"
}

# ── Generate Skip Report ───────────────────────────────────────────
# FIX: Skip Report generation (required by DESIGN.md)
generate_skip_report() {
    local node=$1
    local feature=$2
    local feature_path=$3
    local reason=$4
    local output_dir="${SCRIPT_DIR}/results"
    local timestamp=$(date -Iseconds)
    local skip_report_file="${output_dir}/skip-report-${node}-${feature}.md"

    mkdir -p "$output_dir"

    cat > "$skip_report_file" << EOF
## Skip Report — ${node}
- **Feature:** ${feature}
- **Feature Path:** ${feature_path}
- **Skipped:** YES
- **Reason:** ${reason}
- **Skipped at:** ${timestamp}

### Unvalidated Assumptions
- [ ] Business logic correctness: ASSUMED VALID
- [ ] Requirements traceability: ASSUMED VALID
- [ ] Edge case coverage: NOT CHECKED

### Architect Action Required
Architect must explicitly validate the assumptions above before producing merged plan.
EOF

    log_event "skip_report" "Skip Report generated for $node: $skip_report_file"
    echo "$skip_report_file"
}

# ── Get Skip Recommendations ────────────────────────────────────────
get_skip_recommendations() {
    local feature_folder=$1
    local pr_title=${2:-""}

    echo "=== Skip Recommendations ==="
    echo "Feature: $feature_folder"
    echo "PR Title: $pr_title"
    echo ""

    local complexity
    complexity=$(analyze_complexity "$feature_folder")
    echo "Complexity: $complexity"
    echo ""

    if should_skip_business_analyst "$feature_folder" "$pr_title"; then
        echo -e "  Business-Analyst:   ${GREEN}SKIP${NC}"
    else
        echo -e "  Business-Analyst:   ${YELLOW}RUN${NC}"
    fi

    if should_skip_tech_lead "$feature_folder" "$pr_title"; then
        echo -e "  Tech-Lead:          ${GREEN}SKIP${NC}"
    else
        echo -e "  Tech-Lead:          ${YELLOW}RUN${NC}"
    fi

    echo -e "  Architect:          ${YELLOW}RUN${NC} (never skip)"
    echo -e "  Developer:          ${YELLOW}RUN${NC} (never skip)"

    local qa_mode
    qa_mode=$(get_quality_analysis_mode "$feature_folder" "$pr_title")
    if [ "$qa_mode" = "auto-approve-minor" ]; then
        echo -e "  Quality-Analysis:   ${GREEN}RUN${NC} (mode: auto-approve-minor)"
    else
        echo -e "  Quality-Analysis:   ${YELLOW}RUN${NC} (mode: full-review)"
    fi

    echo ""
}

# ── Main ────────────────────────────────────────────────────────────
main() {
    local command=${1:-}
    shift || true

    case $command in
        analyze)
            analyze_complexity "$1"
            ;;
        skip-business-analyst)
            should_skip_business_analyst "$1" "${2:-}" && echo "SKIP" || echo "RUN"
            ;;
        skip-tech-lead)
            should_skip_tech_lead "$1" "${2:-}" && echo "SKIP" || echo "RUN"
            ;;
        skip-architect)
            should_skip_architect "$1" "${2:-}" && echo "SKIP" || echo "RUN"
            ;;
        skip-developer)
            should_skip_developer "$1" "${2:-}" && echo "SKIP" || echo "RUN"
            ;;
        skip-quality-analysis)
            should_skip_quality_analysis "$1" "${2:-}" && echo "SKIP" || echo "RUN"
            ;;
        qa-mode)
            get_quality_analysis_mode "$1" "${2:-}"
            ;;
        generate-skip-report)
            generate_skip_report "$1" "$2" "$3" "$4"
            ;;
        recommendations)
            get_skip_recommendations "$1" "${2:-}"
            ;;
        *)
            echo "Usage: conditional_skip.sh {analyze|skip-business-analyst|skip-tech-lead|skip-architect|skip-developer|skip-quality-analysis|qa-mode|generate-skip-report|recommendations}"
            exit 1
            ;;
    esac
}

main "$@"
