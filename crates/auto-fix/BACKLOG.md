# Feature Backlog: Auto-Fix

FRD: [FRD.md](FRD.md)
Architecture: [ARCHITECTURE.md](../../ARCHITECTURE.md)
State: defined in root [ROADMAP.md](../../ROADMAP.md) — cited here, not restated
Last Updated: 2026-11-23

## Progress Overview

- Done: All 5 backlog rows reconciled to Done at HEAD; gate: `CARGO_INCREMENTAL=0 cargo nextest run --workspace --lib --tests` → Tests job passed at `e6cb89989215f435141aa68992a71dce44bc243c` (CI run `45261257668`, 2026-11-23). Per-row source and test evidence below.
- Done: `cargo nextest run -p auto-fix-lint-arwaky --lib --tests` → 47 passed, 0 skipped at `a09244f1` (2026-09-29). Per-scenario evidence below.
- In Progress: None
- Blocked: None
- Next Action: Re-run `cargo nextest run -p auto-fix-lint-arwaky --lib --tests` after any code change to this crate

## Backlog Status

### FR-AutoFix-001

| Aspect | Detail |
| -------------- | --------------------------------------------- |
| State | Done |
| Done | Unused Import Removal (`IUnusedImportFixProtocol`); source: `crates/auto-fix/src/capabilities_unused_import_fix.rs`, `crates/auto-fix/src/agent_fix_orchestrator.rs`; tests: `crates/auto-fix/tests/unit_auto_fix_fix_processor.rs`, `crates/auto-fix/tests/contract_auto_fix.rs`, `crates/auto-fix/tests/e2e_auto_fix_flow.rs`; Gate: `CARGO_INCREMENTAL=0 cargo nextest run --workspace --lib --tests` → Tests job passed at `e6cb89989215f435141aa68992a71dce44bc243c` (CI run `45261257668`, 2026-11-23).
| In Progress | None |
| Risk | None |
| Updated | 2026-11-23 |

### FR-AutoFix-002

| Aspect | Detail |
| -------------- | --------------------------------------------- |
| State | Done |
| Done | Bypass Fix (`IBypassFixProtocol`); source: `crates/auto-fix/src/capabilities_bypass_fix.rs`, `crates/auto-fix/src/agent_fix_orchestrator.rs`; tests: `crates/auto-fix/tests/unit_auto_fix_fix_processor.rs`, `crates/auto-fix/tests/contract_auto_fix.rs`, `crates/auto-fix/tests/e2e_auto_fix_flow.rs`; Gate: `CARGO_INCREMENTAL=0 cargo nextest run --workspace --lib --tests` → Tests job passed at `e6cb89989215f435141aa68992a71dce44bc243c` (CI run `45261257668`, 2026-11-23).
| In Progress | None |
| Risk | None |
| Updated | 2026-11-23 |

### FR-AutoFix-003

| Aspect | Detail |
| -------------- | --------------------------------------------- |
| State | Done |
| Done | Symbol Renaming (`ISymbolRenameProtocol`); source: `crates/auto-fix/src/capabilities_symbol_rename.rs`, `crates/auto-fix/src/capabilities_violation_report.rs`; tests: `crates/auto-fix/tests/unit_auto_fix_fix_processor.rs`, `crates/auto-fix/tests/contract_auto_fix.rs`, `crates/auto-fix/tests/acceptance_AES101_fix.rs`; Gate: `CARGO_INCREMENTAL=0 cargo nextest run --workspace --lib --tests` → Tests job passed at `e6cb89989215f435141aa68992a71dce44bc243c` (CI run `45261257668`, 2026-11-23).
| In Progress | None |
| Risk | None |
| Updated | 2026-11-23 |

### FR-AutoFix-004

| Aspect | Detail |
| -------------- | --------------------------------------------- |
| State | Done |
| Done | Violation Reporting (`IViolationReportProtocol`); source: `crates/auto-fix/src/capabilities_violation_report.rs`, `crates/auto-fix/src/agent_fix_orchestrator.rs`; tests: `crates/auto-fix/tests/e2e_auto_fix_flow.rs`, `crates/auto-fix/tests/acceptance_AES304_fix.rs`, `crates/auto-fix/tests/acceptance_AES201_fix.rs`; Gate: `CARGO_INCREMENTAL=0 cargo nextest run --workspace --lib --tests` → Tests job passed at `e6cb89989215f435141aa68992a71dce44bc243c` (CI run `45261257668`, 2026-11-23).
| In Progress | None |
| Risk | None |
| Updated | 2026-11-23 |

### FR-AutoFix-005

| Aspect | Detail |
| -------------- | --------------------------------------------- |
| State | Done |
| Done | Block Marker Fix (`IBlockMarkerFixProtocol, `IFixAggregate`); source: `crates/auto-fix/src/capabilities_block_marker_fix.rs`, `crates/auto-fix/src/agent_fix_orchestrator.rs`; tests: `crates/auto-fix/tests/unit_block_marker_fix.rs`, `crates/auto-fix/tests/integration_auto_fix.rs`, `crates/auto-fix/tests/e2e_auto_fix_flow.rs`; Gate: `CARGO_INCREMENTAL=0 cargo nextest run --workspace --lib --tests` → Tests job passed at `e6cb89989215f435141aa68992a71dce44bc243c` (CI run `45261257668`, 2026-11-23).
| In Progress | None |
| Risk | None |
| Updated | 2026-11-23 |

## Reference

| Artifact | Purpose | Link |
| ---------- | --------- | ------ |
| PRD.md | Product requirements | [PRD.md](../../PRD.md) |
| FRD.md | Feature requirements | [FRD.md](FRD.md) |
| ROADMAP.md | Roadmap status | [ROADMAP.md](../../ROADMAP.md) |
| ARCHITECTURE.md | Layer contract | [ARCHITECTURE.md](../../ARCHITECTURE.md) |
