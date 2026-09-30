# Feature Backlog: Maintenance

FRD: [FRD.md](FRD.md)
Architecture: [ARCHITECTURE.md](../../ARCHITECTURE.md)
State / Health: defined in root [ROADMAP.md](../../ROADMAP.md) — cited here, not restated
Last Updated: 2026-09-30

## Current Condition

- Done: `cargo test -p maintenance-lint-arwaky --lib --tests` → 54 passed, 0 failures at `c23e9c35` (2026-09-30). Per-scenario evidence below.
- In Progress: None
- Blocked: None
- Next Action: Re-run `cargo test -p maintenance-lint-arwaky --lib --tests` after any code change to this crate

## Backlog

| ID | Priority | State | Health | Dependencies | Next Action | Updated |
|---|---:|---|---|---|---|---|
| MAIN-01 | P0 | Done | On Track | None | — | 2026-09-30 |
| MAIN-02 | P0 | Done | On Track | None | — | 2026-09-30 |
| MAIN-03 | P0 | Done | On Track | None | — | 2026-09-30 |
| MAIN-04 | P0 | Done | On Track | None | — | 2026-09-30 |
| MAIN-05 | P0 | Done | On Track | None | — | 2026-09-30 |
| MAIN-06 | P0 | Done | On Track | None | — | 2026-09-30 |
| MAIN-07 | P0 | Done | On Track | None | — | 2026-09-30 |
| MAIN-08 | P0 | Done | On Track | None | — | 2026-09-30 |

## Scenario Evidence

| Scenario | Kind | Test file | Test name | Last verified |
|---|---|---|---|---|
| All required tools OK | healthy: true, all statuses "OK" | Automated | `tests/unit_maintenance_checker.rs` | cargo test -p maintenance-lint-arwaky | `ca070e66` |
| Missing rustc (required) | healthy: false | Automated | `tests/unit_maintenance_checker.rs` | cargo test -p maintenance-lint-arwaky | `ca070e66` |
| Missing ruff (optional) | Status "WARN" in adapter_statuses | Automated | `tests/unit_maintenance_checker.rs` | cargo test -p maintenance-lint-arwaky | `ca070e66` |
| Language runtimes installed | Versions reported (rustc, python3, node) | Automated | `tests/unit_maintenance_checker.rs` | cargo test -p maintenance-lint-arwaky | `ca070e66` |
| Language runtime missing | Version "NOT FOUND" | Automated | `tests/unit_maintenance_checker.rs` | cargo test -p maintenance-lint-arwaky | `ca070e66` |
| Directory with mixed files | Per-language counts + overall totals | Automated | `tests/unit_maintenance_checker.rs` | cargo test -p maintenance-lint-arwaky | `ca070e66` |
| Python project with test files | Correct test ratio | Automated | `tests/unit_maintenance_checker.rs` | cargo test -p maintenance-lint-arwaky | `ca070e66` |
| Directory with no source files | All zeros, ratio 0.0 | Automated | `tests/unit_maintenance_checker.rs` | cargo test -p maintenance-lint-arwaky | `ca070e66` |
| Empty directory | All zeros, ratio 0.0 | Automated | `tests/unit_maintenance_checker.rs` | cargo test -p maintenance-lint-arwaky | `ca070e66` |
| Project with .pytest_cache, __pycache__ | Directories removed | Automated | `tests/unit_maintenance_checker.rs` | cargo test -p maintenance-lint-arwaky | `ca070e66` |
| Project with target/ | Directory removed | Automated | `tests/unit_maintenance_checker.rs` | cargo test -p maintenance-lint-arwaky | `ca070e66` |
| No cache directories | No-op | Automated | `tests/unit_maintenance_checker.rs` | cargo test -p maintenance-lint-arwaky | `ca070e66` |
| Python tools upgrade | pip install --upgrade per tool | Automated | `tests/unit_maintenance_checker.rs` | cargo test -p maintenance-lint-arwaky | `ca070e66` |
| pip not installed | Warning, no crash | Automated | `tests/unit_maintenance_checker.rs` | cargo test -p maintenance-lint-arwaky | `ca070e66` |
| cargo + rustc installed | Status "OK" | Automated | `tests/unit_maintenance_checker.rs` | cargo test -p maintenance-lint-arwaky | `ca070e66` |
| Missing clippy (required) | Status "FAIL" | Automated | `tests/unit_maintenance_checker.rs` | cargo test -p maintenance-lint-arwaky | `ca070e66` |
| Missing mypy (optional) | Status "WARN" | Automated | `tests/unit_maintenance_checker.rs` | cargo test -p maintenance-lint-arwaky | `ca070e66` |
| Missing eslint (optional) | Status "WARN" | Automated | `tests/unit_maintenance_checker.rs` | cargo test -p maintenance-lint-arwaky | `ca070e66` |
| Rust project with Cargo.lock | Runs cargo-audit | Automated | `tests/unit_maintenance_checker.rs` | cargo test -p maintenance-lint-arwaky | `ca070e66` |
| No Cargo.lock | tool_installed: false, empty findings | Automated | `tests/unit_maintenance_checker.rs` | cargo test -p maintenance-lint-arwaky | `ca070e66` |
| cargo-audit not installed | tool_installed: false, empty findings | Automated | `tests/unit_maintenance_checker.rs` | cargo test -p maintenance-lint-arwaky | `ca070e66` |
| No vulnerabilities | Empty findings, success | Automated | `tests/unit_maintenance_checker.rs` | cargo test -p maintenance-lint-arwaky | `ca070e66` |
| Rust project with Cargo.lock | Parses all packages | Automated | `tests/unit_maintenance_checker.rs` | cargo test -p maintenance-lint-arwaky | `ca070e66` |
| No Cargo.lock | Returns error | Automated | `tests/unit_maintenance_checker.rs` | cargo test -p maintenance-lint-arwaky | `ca070e66` |
| Empty Cargo.lock | Empty dependency list | Automated | `tests/unit_maintenance_checker.rs` | cargo test -p maintenance-lint-arwaky | `ca070e66` |
| All 9 adapters installed | All available: true | Automated | `tests/unit_maintenance_checker.rs` | cargo test -p maintenance-lint-arwaky | `ca070e66` |
| Missing ruff | ruff available: false | Automated | `tests/unit_maintenance_checker.rs` | cargo test -p maintenance-lint-arwaky | `ca070e66` |
| No adapters installed | All available: false | Automated | `tests/unit_maintenance_checker.rs` | cargo test -p maintenance-lint-arwaky | `ca070e66` |
| Latest release is newer | `latest_version` = tag, `upgraded` = true | Automated | `tests/unit_maintenance_checker.rs` | cargo test -p maintenance-lint-arwaky | `ca070e66` |
| Latest release equals current | `already_up_to_date` = true, no install | Automated | `tests/unit_maintenance_version_helpers.rs` | cargo test -p maintenance-lint-arwaky | `ca070e66` |
| API unreachable | `latest_version` empty, status starts with `Error:` | Automated | `tests/unit_maintenance_checker.rs` | cargo test -p maintenance-lint-arwaky | `ca070e66` |
| `check_only` flag set | No download performed | Manual | `./lint-arwaky-cli update --check-only` | CLI smoke test | `2026-09-29` |
| Local version ahead of release | `already_up_to_date` = true, no install | Gap | — | — | — |

## Blockers

None

## Dependencies

None

## Release Readiness

| Area | Status | Notes |
|---|---|---|
| Tests | Done | `cargo test -p maintenance-lint-arwaky --lib --tests` → 45 passed, 0 failures at `ca070e66` (2026-09-29) |
| Scenario evidence | Done | 28 scenarios mapped; all Automated via `cargo test -p maintenance-lint-arwaky` |
| Docs | Done | [FRD.md](FRD.md) is specification-only; status lives in this file |

## Deferred

None

## Change Log

| Date | Change | By |
|---|---:|---|---|
| 2026-09-30 | Merged FR-Maintenance-005 (Diagnose Toolchain) into FR-Maintenance-001; renumbered FRs 006-009 to 005-008; removed ToolchainDiagnosticChecker; unified IDoctorProtocol implementation | @raka |
| 2026-09-27 | Added FR-MAINTENANCE-009: self-update — GitHub release query + binary install | @raka |
