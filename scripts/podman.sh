#!/usr/bin/env bash
# PURPOSE: Build and run lint-arwaky inside Podman for isolated testing
# Usage: bash scripts/podman.sh [build|test|scan|shell|gates|clean] [args...]
# Keeps the host toolchain untouched so local stable lint stays intact.

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"

IMAGE_NAME="lint-arwaky-dev"
CONTAINER_NAME="lint-arwaky-podman"
# Mount source read-write so the container can build and write target/
VOLUME="${PROJECT_ROOT}:/app:z"

# Shared cargo registry inside the image to avoid re-downloading crates
CARGO_HOME_IN_IMAGE="/usr/local/cargo"

# ─── helpers ───────────────────────────────────────────────────────────────────
log()  { printf '\033[36m[podman]\033[0m %s\n' "$*"; }
fail() { printf '\033[31m[podman] ERROR:\033[0m %s\n' "$*" >&2; exit 1; }

podman_running() {
    podman ps --filter "name=^${CONTAINER_NAME}$" --format '{{.Names}}' 2>/dev/null | grep -q "${CONTAINER_NAME}"
}

stop_container() {
    if podman_running; then
        log "stopping container ${CONTAINER_NAME}..."
        podman stop "${CONTAINER_NAME}" >/dev/null
        podman rm "${CONTAINER_NAME}" >/dev/null 2>&1 || true
        log "container removed"
    fi
}

start_container() {
    stop_container
    log "starting container ${CONTAINER_NAME}..."
    # RUSTC_WRAPPER="" clears the host's sccache (not available in container);
    # mold IS installed in the image so the host cargo config's -fuse-ld=mold works.
    podman run -d \
        --name "${CONTAINER_NAME}" \
        -v "${VOLUME}" \
        -w /app \
        -e CARGO_INCREMENTAL=0 \
        -e RUSTC_WRAPPER= \
        --network=host \
        "${IMAGE_NAME}" \
        sleep infinity >/dev/null
    log "container ready"
}

exec_in_container() {
    start_container
    podman exec "${CONTAINER_NAME}" "$@"
}

# ─── image build ───────────────────────────────────────────────────────────────
build_image() {
    log "building image ${IMAGE_NAME}..."
    podman build -t "${IMAGE_NAME}" -f "${PROJECT_ROOT}/Containerfile.dev" "${PROJECT_ROOT}"
    log "image ready"
}

# ─── commands ──────────────────────────────────────────────────────────────────
cmd_build() {
    build_image
    log "building lint-arwaky-cli inside container..."
    exec_in_container bash -c "CARGO_INCREMENTAL=0 cargo build --release"
    log "binary at target/release/lint-arwaky-cli (host-visible)"
}

cmd_test() {
    exec_in_container bash -c "cargo test --workspace 2>&1 | tail -20"
}

cmd_gates() {
    exec_in_container bash -c "bash scripts/gates.sh 2>&1 | tail -15"
}

cmd_scan() {
    local target="${1:-.}"
    exec_in_container bash -c "./target/release/lint-arwaky-cli scan ${target}"
}

cmd_check() {
    local target="${1:-.}"
    exec_in_container bash -c "./target/release/lint-arwaky-cli check ${target}"
}

cmd_shell() {
    start_container
    log "entering interactive shell (exit to leave)"
    podman exec -it "${CONTAINER_NAME}" bash
}

cmd_clean() {
    stop_container
    log "removing image ${IMAGE_NAME}..."
    podman rmi "${IMAGE_NAME}" 2>/dev/null || true
    log "done"
}

cmd_help() {
    cat <<'EOF'
Usage: bash scripts/podman.sh <command> [args...]

Commands:
  build            Build Podman image + lint-arwaky binary inside it
  test             Run workspace tests inside container
  gates            Run full quality gates inside container
  scan <path>      Run lint-arwaky-cli scan on path (default: .)
  check <path>     Run lint-arwaky-cli check on path (default: .)
  shell            Open interactive bash shell inside container
  clean            Stop and remove container + image

Examples:
  bash scripts/podman.sh build
  bash scripts/podman.sh scan workspaces-good/crates
  bash scripts/podman.sh check crates/shared
EOF
}

# ─── main ──────────────────────────────────────────────────────────────────────
case "${1:-help}" in
    build)  build_image && cmd_build ;;
    test)   cmd_test ;;
    gates)  cmd_gates ;;
    scan)   shift; cmd_scan "${@}" ;;
    check)  shift; cmd_check "${@}" ;;
    shell)  cmd_shell ;;
    clean)  cmd_clean ;;
    help|-h|--help) cmd_help ;;
    *)      cmd_help; exit 1 ;;
esac
