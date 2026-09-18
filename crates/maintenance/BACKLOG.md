# Feature Backlog: Maintenance

FRD: [FRD.md](FRD.md)
Architecture: [ARCHITECTURE.md](../../ARCHITECTURE.md)
State / Health: defined in root [BACKLOG.md](../../BACKLOG.md) — cited here, not restated
Last Updated: 2026-09-17

## Current Condition

- Done: `cargo test -p maintenance --lib --tests` → 0 failures at `29c71083` (2026-09-17). Per-scenario evidence below.
- In Progress: None
- Blocked: None
- Next Action: Re-run `cargo test -p maintenance --lib --tests` after any code change to this crate

## Backlog

| ID | FRD Ref | Work Item | Priority | State | Actual Condition | Owner | Dependencies | Updated |
|---|---|---|---:|---|---|---|---|---|
| MAIN-01 | FR-001 | SCEN-001 - Doctor — 5 scenarios verified | P0 | Done | `cargo test -p maintenance --lib --tests` → 0 failures at `29c71083` (2026-09-17) | @raka | None | 2026-09-17 |
| MAIN-02 | FR-002 | SCEN-002 - Stats — 4 scenarios verified | P0 | Done | `cargo test -p maintenance --lib --tests` → 0 failures at `29c71083` (2026-09-17) | @raka | None | 2026-09-17 |
| MAIN-03 | FR-003 | SCEN-003 - Clean — 3 scenarios verified | P0 | Done | `cargo test -p maintenance --lib --tests` → 0 failures at `29c71083` (2026-09-17) | @raka | None | 2026-09-17 |
| MAIN-04 | FR-004 | SCEN-004 - Update — 2 scenarios verified | P0 | Done | `cargo test -p maintenance --lib --tests` → 0 failures at `29c71083` (2026-09-17) | @raka | None | 2026-09-17 |
| MAIN-05 | FR-005 | SCEN-005 - Diagnose — 4 scenarios verified | P0 | Done | `cargo test -p maintenance --lib --tests` → 0 failures at `29c71083` (2026-09-17) | @raka | None | 2026-09-17 |
| MAIN-06 | FR-006 | SCEN-006 - Security — 4 scenarios verified | P0 | Done | `cargo test -p maintenance --lib --tests` → 0 failures at `29c71083` (2026-09-17) | @raka | None | 2026-09-17 |
| MAIN-07 | FR-007 | SCEN-007 - Dependencies — 3 scenarios verified | P0 | Done | `cargo test -p maintenance --lib --tests` → 0 failures at `29c71083` (2026-09-17) | @raka | None | 2026-09-17 |
| MAIN-08 | FR-008 | SCEN-008 - Adapter Health Check — 3 scenarios verified | P0 | Done | `cargo test -p maintenance --lib --tests` → 0 failures at `29c71083` (2026-09-17) | @raka | None | 2026-09-17 |

## Scenario Evidence

| Scenario | Kind | Test file | Test name | Last verified |
|---|---|---|---|---|
| All required tools OK | healthy: true, all statuses "OK" | Automated | `tests/maintenance/` | cargo test -p maintenance | `29c71083` |
| Missing rustc (required) | healthy: false | Automated | `tests/maintenance/` | cargo test -p maintenance | `29c71083` |
| Missing ruff (optional) | Status "WARN" in adapter_statuses | Automated | `tests/maintenance/` | cargo test -p maintenance | `29c71083` |
| Language runtimes installed | Versions reported (rustc, python3, node) | Automated | `tests/maintenance/` | cargo test -p maintenance | `29c71083` |
| Language runtime missing | Version "NOT FOUND" | Automated | `tests/maintenance/` | cargo test -p maintenance | `29c71083` |
| Directory with mixed files | Per-language counts + overall totals | Automated | `tests/maintenance/` | cargo test -p maintenance | `29c71083` |
| Python project with test files | Correct test ratio | Automated | `tests/maintenance/` | cargo test -p maintenance | `29c71083` |
| Directory with no source files | All zeros, ratio 0.0 | Automated | `tests/maintenance/` | cargo test -p maintenance | `29c71083` |
| Empty directory | All zeros, ratio 0.0 | Automated | `tests/maintenance/` | cargo test -p maintenance | `29c71083` |
| Project with .pytest_cache, __pycache__ | Directories removed | Automated | `tests/maintenance/` | cargo test -p maintenance | `29c71083` |
| Project with target/ | Directory removed | Automated | `tests/maintenance/` | cargo test -p maintenance | `29c71083` |
| No cache directories | No-op | Automated | `tests/maintenance/` | cargo test -p maintenance | `29c71083` |
| Python tools upgrade | pip install --upgrade per tool | Automated | `tests/maintenance/` | cargo test -p maintenance | `29c71083` |
| pip not installed | Warning, no crash | Automated | `tests/maintenance/` | cargo test -p maintenance | `29c71083` |
| cargo + rustc installed | Status "OK" | Automated | `tests/maintenance/` | cargo test -p maintenance | `29c71083` |
| Missing clippy (required) | Status "FAIL" | Automated | `tests/maintenance/` | cargo test -p maintenance | `29c71083` |
| Missing mypy (optional) | Status "WARN" | Automated | `tests/maintenance/` | cargo test -p maintenance | `29c71083` |
| Missing eslint (optional) | Status "WARN" | Automated | `tests/maintenance/` | cargo test -p maintenance | `29c71083` |
| Rust project with Cargo.lock | Runs cargo-audit | Automated | `tests/maintenance/` | cargo test -p maintenance | `29c71083` |
| No Cargo.lock | tool_installed: false, empty findings | Automated | `tests/maintenance/` | cargo test -p maintenance | `29c71083` |
| cargo-audit not installed | tool_installed: false, empty findings | Automated | `tests/maintenance/` | cargo test -p maintenance | `29c71083` |
| No vulnerabilities | Empty findings, success | Automated | `tests/maintenance/` | cargo test -p maintenance | `29c71083` |
| Rust project with Cargo.lock | Parses all packages | Automated | `tests/maintenance/` | cargo test -p maintenance | `29c71083` |
| No Cargo.lock | Returns error | Automated | `tests/maintenance/` | cargo test -p maintenance | `29c71083` |
| Empty Cargo.lock | Empty dependency list | Automated | `tests/maintenance/` | cargo test -p maintenance | `29c71083` |
| All 9 adapters installed | All available: true | Automated | `tests/maintenance/` | cargo test -p maintenance | `29c71083` |
| Missing ruff | ruff available: false | Automated | `tests/maintenance/` | cargo test -p maintenance | `29c71083` |
| No adapters installed | All available: false | Automated | `tests/maintenance/` | cargo test -p maintenance | `29c71083` |

## Blockers

None

## Dependencies

None

## Release Readiness

| Area | Status | Notes |
|---|---|---|
| Tests | Done | `cargo test -p maintenance --lib --tests` → 0 failures at `29c71083` (2026-09-17) |
| Scenario evidence | Done | 28 scenarios mapped; all Automated via `cargo test -p maintenance` |
| Docs | Done | [FRD.md](FRD.md) is specification-only; status lives in this file |

## Deferred

None

## Change Log

| Date | Change | By |
|---|---|---|
| 2026-09-17 | Initial backlog created from FRD test-scenario mapping | @raka |
