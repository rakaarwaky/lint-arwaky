# justfile — Unified developer command runner for lint-arwaky
# Install just: cargo install just
# Usage: just <command>

# List available commands
default:
    @just --list

# ── Build ──────────────────────────────────────────────────────────────────────

# Build all crates (debug)
build:
    cargo build

# Build release binary
build-release:
    RUST_MIN_STACK=33554432 cargo build --release

# Build specific binary
build-bin target="lint-arwaky-cli":
    cargo build --bin {{target}}

# ── Quality Gates ──────────────────────────────────────────────────────────────

# Run all quality gates (fmt + clippy + self-lint + tests)
gates:
    bash scripts/gates.sh

# Check code formatting
fmt:
    cargo fmt --all -- --check

# Auto-format code
fmt-fix:
    cargo fmt --all

# Run clippy lints
clippy:
    cargo clippy --all-targets -- -D warnings

# Run self-lint (AES rules)
self-lint: build-bin
    ./target/debug/lint-arwaky-cli check . --filter AES

# ── Tests ──────────────────────────────────────────────────────────────────────

# Run all tests
test:
    cargo test --workspace

# Run tests with nextest (faster)
test-fast:
    cargo nextest run --workspace --lib --tests -j 2

# Run tests for specific crate
test-crate crate:
    cargo test -p {{crate}}

# Run benchmarks
bench:
    cargo bench --workspace

# ── Lint ───────────────────────────────────────────────────────────────────────

# Run lint-arwaky CLI scan on test workspaces
scan:
    cargo run --bin lint-arwaky-cli -- scan test-workspaces

# Self-scan the project
self-scan:
    cargo run --bin lint-arwaky-cli -- scan .

# ── External Linters ───────────────────────────────────────────────────────────

# Run eslint on TypeScript/JavaScript
lint-js:
    npx eslint . --ext .ts,.js,.tsx,.jsx || true

# Run prettier check
lint-prettier:
    npx prettier --check . || true

# Run ruff on Python
lint-py:
    ruff check . || true

# Run mypy type check
typecheck-py:
    mypy . || true

# Run bandit security check
security-py:
    bandit -r . || true

# Run all external linters
lint-all: lint-js lint-prettier lint-py typecheck-py security-py

# ── Install ────────────────────────────────────────────────────────────────────

# Full developer environment setup
install-dev:
    bash scripts/install.dev.sh

# Local user installation (XDG)
install-local:
    bash scripts/install.local.sh

# Global system-wide installation (requires root)
install-global:
    sudo bash scripts/install.global.sh

# Remote installation (crates.io / git)
install-remote:
    bash scripts/install.remote.sh

# ── Release ────────────────────────────────────────────────────────────────────

# Bump version (patch, minor, major, or X.Y.Z)
bump type:
    bash scripts/bump.sh {{type}}

# Dry-run version bump
bump-dry type:
    bash scripts/bump.sh --dry-run {{type}}

# Full release pipeline
release bump="" args="":
    bash scripts/release.sh {{args}} {{bump and "--bump " + bump or ""}}

# CI-only checks (no commit/tag/publish)
ci:
    bash scripts/release.sh --ci-only

# ── Git ────────────────────────────────────────────────────────────────────────

# Build + lint + commit + push
push msg="":
    bash scripts/push.sh {{msg and "-m " + quote(msg) or ""}}

# Fast push (skip fmt + clippy)
push-fast msg="":
    bash scripts/push.sh --fast {{msg and "-m " + quote(msg) or ""}}

# Sync main and develop with origin
sync:
    bash scripts/sync.sh

# ── Security ───────────────────────────────────────────────────────────────────

# Install security monitor (requires root)
security-install:
    sudo bash scripts/security.sh install

# Show security monitor status
security-status:
    sudo bash scripts/security.sh status

# Stop security monitor
security-stop:
    sudo bash scripts/security.sh stop

# ── MCP Server ─────────────────────────────────────────────────────────────────

# Run MCP server
mcp:
    cargo run --bin lint-arwaky-mcp

# Run TUI launcher
tui:
    cargo run --bin lint-arwaky-tui

# ── Dev Utilities ──────────────────────────────────────────────────────────────

# Watch for changes and rebuild
watch:
    cargo watch -x build

# Watch and run tests on change
watch-test:
    cargo watch -x test

# Watch and run clippy on change
watch-clippy:
    cargo watch -x clippy

# Run security audit
audit:
    cargo audit

# Export feature crate for AI analysis
export-feature:
    python3 scripts/export_feature.py

# Export single file for AI analysis
export-file:
    python3 scripts/export_file.py

# Export skill directory for AI analysis
export-skill:
    python3 scripts/export_skill.py

# Clean build artifacts
clean:
    cargo clean
    rm -rf dist/

# Show project version
version:
    @cargo metadata --no-deps --format-version 1 2>/dev/null | sed -n 's/.*"version":"\([^"]*\)".*/\1/p' | head -1

# Show workspace crate list
crates:
    @cargo metadata --no-deps --format-version 1 2>/dev/null | python3 -c "import sys,json; [print(p['name']) for p in json.load(sys.stdin)['packages']]"
