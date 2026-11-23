# Dispatcher — BACKLOG

FRD: —
Architecture: [ARCHITECTURE.md](../../ARCHITECTURE.md)
State: defined in root [ROADMAP.md](../../ROADMAP.md) — cited here, not restated
Last Updated: 2026-11-23

## Progress Overview

- Done: All 7 backlog rows reconciled to Done at HEAD; gate: `CARGO_INCREMENTAL=0 cargo nextest run --workspace --lib --tests` → Tests job passed at `e6cb89989215f435141aa68992a71dce44bc243c` (CI run `45261257668`, 2026-11-23). Per-row source and test evidence below.
- Done: `cargo test -p dispatcher --lib --tests` → 0 failures
- Done: single-file scan reaches the external adapters (`collect_single_file_external`)
- In Progress: None
- Blocked: None
- Next Action: —

## Backlog Status

### DS-DISP-001

| Aspect | Detail |
| -------------- | --------------------------------------------- |
| State | Done |
| Done | Outcome Vocabulary; source: `crates/dispatcher/src/surface_scan_cancel_action.rs`, `crates/dispatcher/src/surface_naming_action.rs`; tests: `crates/dispatcher/tests/regression_scan_cancel.rs`; Gate: `CARGO_INCREMENTAL=0 cargo nextest run --workspace --lib --tests` → Tests job passed at `e6cb89989215f435141aa68992a71dce44bc243c` (CI run `45261257668`, 2026-11-23).
| In Progress | None |
| Risk | None |
| Updated | 2026-11-23 |

### DS-DISP-002

| Aspect | Detail |
| -------------- | --------------------------------------------- |
| State | Done |
| Done | Kind; source: `crates/dispatcher/src/lib.rs`, `crates/dispatcher/src/surface_check_action.rs`; tests: `crates/dispatcher/tests/acceptance_dispatcher.rs`, `crates/dispatcher/tests/contract_dispatcher.rs`; Gate: `CARGO_INCREMENTAL=0 cargo nextest run --workspace --lib --tests` → Tests job passed at `e6cb89989215f435141aa68992a71dce44bc243c` (CI run `45261257668`, 2026-11-23).
| In Progress | None |
| Risk | None |
| Updated | 2026-11-23 |

### DS-DISP-003

| Aspect | Detail |
| -------------- | --------------------------------------------- |
| State | Done |
| Done | Request Shape; source: `crates/dispatcher/src/surface_check_scope_action.rs`, `crates/dispatcher/src/surface_scan_cancel_action.rs`; tests: `crates/dispatcher/tests/regression_panic_isolation.rs`, `crates/dispatcher/tests/regression_scan_latency.rs`, `crates/dispatcher/tests/unit_dispatcher_layer_scan_action.rs`; Gate: `CARGO_INCREMENTAL=0 cargo nextest run --workspace --lib --tests` → Tests job passed at `e6cb89989215f435141aa68992a71dce44bc243c` (CI run `45261257668`, 2026-11-23).
| In Progress | None |
| Risk | None |
| Updated | 2026-11-23 |

### DS-DISP-004

| Aspect | Detail |
| -------------- | --------------------------------------------- |
| State | Done |
| Done | Invariants; source: `crates/dispatcher/src/surface_docs_action.rs`, `crates/dispatcher/src/surface_check_scan_action.rs`; tests: `crates/dispatcher/tests/regression_scan_latency.rs`; Gate: `CARGO_INCREMENTAL=0 cargo nextest run --workspace --lib --tests` → Tests job passed at `e6cb89989215f435141aa68992a71dce44bc243c` (CI run `45261257668`, 2026-11-23).
| In Progress | None |
| Risk | None |
| Updated | 2026-11-23 |

### DS-DISP-005

| Aspect | Detail |
| -------------- | --------------------------------------------- |
| State | Done |
| Done | Change Checklist; source: `crates/dispatcher/src/surface_git_action.rs`, `crates/dispatcher/src/surface_fix_action.rs`; tests: `crates/dispatcher/tests/unit_dispatcher_plugins.rs`; Gate: `CARGO_INCREMENTAL=0 cargo nextest run --workspace --lib --tests` → Tests job passed at `e6cb89989215f435141aa68992a71dce44bc243c` (CI run `45261257668`, 2026-11-23).
| In Progress | None |
| Risk | None |
| Updated | 2026-11-23 |

### DS-DISP-006

| Aspect | Detail |
| -------------- | --------------------------------------------- |
| State | Done |
| Done | States; source: `crates/dispatcher/src/lib.rs`, `crates/dispatcher/src/surface_check_action.rs`; tests: `crates/dispatcher/tests/smoke_dispatcher.rs`, `crates/dispatcher/tests/integration_dispatcher.rs`; Gate: `CARGO_INCREMENTAL=0 cargo nextest run --workspace --lib --tests` → Tests job passed at `e6cb89989215f435141aa68992a71dce44bc243c` (CI run `45261257668`, 2026-11-23).
| In Progress | None |
| Risk | None |
| Updated | 2026-11-23 |

### DS-DISP-007

| Aspect | Detail |
| -------------- | --------------------------------------------- |
| State | Done |
| Done | Error States; source: `crates/dispatcher/src/surface_docs_action.rs`, `crates/dispatcher/src/surface_fix_action.rs`; tests: `crates/dispatcher/tests/unit_dispatcher_ci_action.rs`, `crates/dispatcher/tests/unit_dispatcher_fix_and_git.rs`, `crates/dispatcher/tests/unit_dispatcher_in_memory.rs`; Gate: `CARGO_INCREMENTAL=0 cargo nextest run --workspace --lib --tests` → Tests job passed at `e6cb89989215f435141aa68992a71dce44bc243c` (CI run `45261257668`, 2026-11-23).
| In Progress | None |
| Risk | None |
| Updated | 2026-11-23 |

## Reference

| Artifact | Purpose | Link |
| ---------- | --------- | ------ |
| PRD.md | Product requirements | [PRD.md](../../PRD.md) |
| FRD.md | Feature requirements | [FRD.md](FRD.md) |
| DESIGN.md | Surface design | [DESIGN.md](DESIGN.md) |
| ROADMAP.md | Roadmap status | [ROADMAP.md](../../ROADMAP.md) |
| ARCHITECTURE.md | Layer contract | [ARCHITECTURE.md](../../ARCHITECTURE.md) |
