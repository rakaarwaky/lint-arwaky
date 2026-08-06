#!/bin/bash
# Config Reader for Graph Loop Pipeline (Revision 2.0)
# Reads from unified config.yaml — Single Source of Truth
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
CONFIG_FILE="${SCRIPT_DIR}/config.yaml"

# ── Check Dependencies ──────────────────────────────────────────────
check_deps() {
    if ! command -v yq &> /dev/null; then
        echo "Error: yq not installed. Install with: snap install yq" >&2
        exit 1
    fi
}

# ── Read Config Value ───────────────────────────────────────────────
read_config() {
    local path=$1
    local default=${2:-""}
    local value
    value=$(yq -r "$path" "$CONFIG_FILE" 2>/dev/null || echo "")
    if [ -z "$value" ] || [ "$value" = "null" ]; then
        echo "$default"
    else
        echo "$value"
    fi
}

# ── Read Node Config ────────────────────────────────────────────────
read_node_config() {
    local node=$1
    local field=$2
    local default=${3:-""}
    read_config ".nodes.${node}.${field}" "$default"
}

# ── Read Trigger Config ─────────────────────────────────────────────
read_trigger_config() {
    local trigger=$1
    local field=$2
    local default=${3:-""}
    read_config ".triggers.${trigger}.${field}" "$default"
}

# ── Read Skip Rules ─────────────────────────────────────────────────
read_skip_rules() {
    local node=$1
    local field=$2
    local default=${3:-""}
    read_config ".skip_rules.${node}.${field}" "$default"
}

# ── Read Feature Queue Config ───────────────────────────────────────
read_feature_queue_config() {
    local field=$1
    local default=${2:-""}
    read_config ".feature_queue.${field}" "$default"
}

# ── Read Notification Config ────────────────────────────────────────
read_notification_config() {
    local field=$1
    local default=${2:-""}
    read_config ".notifications.${field}" "$default"
}

# ── Read Health Check Config ────────────────────────────────────────
read_health_config() {
    local field=$1
    local default=${2:-""}
    read_config ".health_check.${field}" "$default"
}

# ── Read Paths Config ───────────────────────────────────────────────
read_paths_config() {
    local field=$1
    local default=${2:-""}
    read_config ".paths.${field}" "$default"
}

# ── Read Settings Config ────────────────────────────────────────────
read_settings_config() {
    local field=$1
    local default=${2:-""}
    read_config ".settings.${field}" "$default"
}

# ── Read Counter Config ─────────────────────────────────────────────
read_counter_config() {
    local field=$1
    local default=${2:-""}
    read_config ".counters.${field}" "$default"
}

# ── Read Recovery Config ────────────────────────────────────────────
read_recovery_config() {
    local field=$1
    local default=${2:-""}
    read_config ".recovery.${field}" "$default"
}

# ── Read Trigger Guards Config ──────────────────────────────────────
read_trigger_guards_config() {
    local field=$1
    local default=${2:-""}
    read_config ".trigger_guards.${field}" "$default"
}

# ── Get Project Root (top-level) ───────────────────────────────────
get_project_root() {
    read_config ".project_root" "/home/raka/mcp-arwaky/lint-arwaky"
}

# ── Get All Node Names ──────────────────────────────────────────────
get_node_names() {
    yq -r '.nodes | keys[]' "$CONFIG_FILE" 2>/dev/null || echo ""
}

# ── Get All Trigger Names ───────────────────────────────────────────
get_trigger_names() {
    yq -r '.triggers | keys[]' "$CONFIG_FILE" 2>/dev/null || echo ""
}

# ── Print Full Config ───────────────────────────────────────────────
print_config() {
    cat "$CONFIG_FILE"
}

# ── Read Feature List (sorted by priority) ──────────────────────────
get_features() {
    yq -r '.features | sort_by(.priority)[] | "\(.priority)|\(.name)|\(.path)|\(.description)"' "$CONFIG_FILE" 2>/dev/null || echo ""
}

# ── Get Feature by Priority ────────────────────────────────────────
get_feature_by_priority() {
    local priority=$1
    yq -r ".features[] | select(.priority == $priority)" "$CONFIG_FILE" 2>/dev/null || echo ""
}

# ── Get Feature by Name ────────────────────────────────────────────
get_feature_by_name() {
    local name=$1
    yq -r ".features[] | select(.name == \"$name\")" "$CONFIG_FILE" 2>/dev/null || echo ""
}

# ── Get Next Feature (sorted by priority) ──────────────────────────
get_next_feature() {
    local current_priority=${1:-0}
    yq -r ".features | sort_by(.priority) | map(select(.priority > $current_priority)) | .[0] // empty" "$CONFIG_FILE" 2>/dev/null || echo ""
}

# ── Compute FRD Hash ───────────────────────────────────────────────
compute_frd_hash() {
    local frd_path=$1
    if [ -f "$frd_path" ]; then
        sha256sum "$frd_path" | awk '{print "sha256:" $1}'
    else
        echo "sha256:NOT_FOUND"
    fi
}

# ── Build Feature Context for Prompt ───────────────────────────────
build_feature_context() {
    local feature_name=$1
    local correlation_id=${2:-"unknown"}
    local pipeline_iteration=${3:-1}
    local rejection_loop=${4:-0}

    local feature_data
    feature_data=$(get_feature_by_name "$feature_name")

    if [ -z "$feature_data" ]; then
        echo "Feature not found: $feature_name"
        return 1
    fi

    local feature_path
    local description
    feature_path=$(echo "$feature_data" | yq -r '.path')
    description=$(echo "$feature_data" | yq -r '.description')

    local frd_path="${feature_path}/FRD.md"
    local frd_hash
    frd_hash=$(compute_frd_hash "$frd_path")

    local project_root
    project_root=$(get_project_root)

    cat << EOF
## Feature Context
- **Feature:** ${feature_name}
- **Feature Path:** ${feature_path}
- **FRD Path:** ${frd_path}
- **FRD Hash:** ${frd_hash}
- **Project Root:** ${project_root}
- **Correlation ID:** ${correlation_id}
- **Pipeline Iteration:** ${pipeline_iteration}/5
- **Rejection Loop:** ${rejection_loop}/3
- **Rule:** Only analyze files within Feature Path. Do NOT touch files outside Feature Path.

## Shared Acceptance Criteria
- [ ] All findings must have evidence (file + line number)
- [ ] All recommendations must be actionable
- [ ] Report must follow the specified output schema
- [ ] FRD snapshot must be consistent with the recorded hash
EOF
}

# ── Main ────────────────────────────────────────────────────────────
main() {
    check_deps

    local command=${1:-}
    shift || true

    case $command in
        read)           read_config "$1" "${2:-}" ;;
        node)           read_node_config "$1" "$2" "${3:-}" ;;
        trigger)        read_trigger_config "$1" "$2" "${3:-}" ;;
        skip)           read_skip_rules "$1" "$2" "${3:-}" ;;
        feature-queue)  read_feature_queue_config "$1" "${2:-}" ;;
        notification)   read_notification_config "$1" "${2:-}" ;;
        health)         read_health_config "$1" "${2:-}" ;;
        paths)          read_paths_config "$1" "${2:-}" ;;
        settings)       read_settings_config "$1" "${2:-}" ;;
        counters)       read_counter_config "$1" "${2:-}" ;;
        recovery)       read_recovery_config "$1" "${2:-}" ;;
        trigger-guards) read_trigger_guards_config "$1" "${2:-}" ;;
        project-root)   get_project_root ;;
        features)       get_features ;;
        feature-by-priority) get_feature_by_priority "$1" ;;
        feature-by-name)     get_feature_by_name "$1" ;;
        feature-next)        get_next_feature "${1:-0}" ;;
        frd-hash)       compute_frd_hash "$1" ;;
        feature-context) build_feature_context "$1" "${2:-unknown}" "${3:-1}" "${4:-0}" ;;
        node-names)     get_node_names ;;
        trigger-names)  get_trigger_names ;;
        print)          print_config ;;
        *)
            echo "Usage: config_reader.sh {read|node|trigger|skip|feature-queue|notification|health|paths|settings|counters|recovery|trigger-guards|project-root|features|feature-by-priority|feature-by-name|feature-next|frd-hash|feature-context|node-names|trigger-names|print}"
            exit 1
            ;;
    esac
}

main "$@"
