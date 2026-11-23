# Cli Commands — BACKLOG

FRD: —
Architecture: [ARCHITECTURE.md](../../ARCHITECTURE.md)
State: defined in root [ROADMAP.md](../../ROADMAP.md) — cited here, not restated
Last Updated: 2026-11-23

## Progress Overview

- Done: All 10 backlog rows reconciled to Done at HEAD; gate: `CARGO_INCREMENTAL=0 cargo nextest run --workspace --lib --tests` → Tests job passed at `e6cb89989215f435141aa68992a71dce44bc243c` (CI run `45261257668`, 2026-11-23). Per-row source and test evidence below.
- Done: `cargo test -p cli_commands --lib --tests` → 0 failures
- In Progress: None
- Blocked: None
- Next Action: —

## Backlog Status

### DS-CLIC-001

| Aspect | Detail |
| -------------- | --------------------------------------------- |
| State | Done |
| Done | Outcome Vocabulary; source: `crates/cli-commands/src/surface_scan_command.rs`; tests: `crates/cli-commands/tests/acceptance_cli_commands.rs`, `crates/cli-commands/tests/contract_cli_commands.rs`; Gate: `CARGO_INCREMENTAL=0 cargo nextest run --workspace --lib --tests` → Tests job passed at `e6cb89989215f435141aa68992a71dce44bc243c` (CI run `45261257668`, 2026-11-23).
| In Progress | None |
| Risk | None |
| Updated | 2026-11-23 |

### DS-CLIC-002

| Aspect | Detail |
| -------------- | --------------------------------------------- |
| State | Done |
| Done | Path Semantics; source: `crates/cli-commands/src/surface_scan_command.rs`, `crates/cli-commands/src/surface_report_command.rs`; tests: `crates/cli-commands/tests/unit_cli_report_delta.rs`, `crates/cli-commands/tests/unit_cli_formatting.rs`, `crates/cli-commands/tests/dogfood_cli_commands.rs`; Gate: `CARGO_INCREMENTAL=0 cargo nextest run --workspace --lib --tests` → Tests job passed at `e6cb89989215f435141aa68992a71dce44bc243c` (CI run `45261257668`, 2026-11-23).
| In Progress | None |
| Risk | None |
| Updated | 2026-11-23 |

### DS-CLIC-003

| Aspect | Detail |
| -------------- | --------------------------------------------- |
| State | Done |
| Done | Output Shapes; source: `crates/cli-commands/src/surface_watch_command.rs`, `crates/cli-commands/src/surface_scan_command.rs`; tests: `crates/cli-commands/tests/dogfood_cli_commands.rs`, `crates/cli-commands/tests/smoke_cli_commands.rs`, `crates/cli-commands/tests/unit_cli_watch_output.rs`; Gate: `CARGO_INCREMENTAL=0 cargo nextest run --workspace --lib --tests` → Tests job passed at `e6cb89989215f435141aa68992a71dce44bc243c` (CI run `45261257668`, 2026-11-23).
| In Progress | None |
| Risk | None |
| Updated | 2026-11-23 |

### DS-CLIC-004

| Aspect | Detail |
| -------------- | --------------------------------------------- |
| State | Done |
| Done | Report; source: `crates/cli-commands/src/surface_report_command.rs`, `crates/cli-commands/src/surface_scan_command.rs`; tests: `crates/cli-commands/tests/unit_cli_report_delta.rs`, `crates/cli-commands/tests/unit_cli_formatting.rs`, `crates/cli-commands/tests/acceptance_cli_commands.rs`; Gate: `CARGO_INCREMENTAL=0 cargo nextest run --workspace --lib --tests` → Tests job passed at `e6cb89989215f435141aa68992a71dce44bc243c` (CI run `45261257668`, 2026-11-23).
| In Progress | None |
| Risk | None |
| Updated | 2026-11-23 |

### DS-CLIC-005

| Aspect | Detail |
| -------------- | --------------------------------------------- |
| State | Done |
| Done | Kind; source: `crates/cli-commands/src/lib.rs`, `crates/cli-commands/src/surface_report_command.rs`; tests: `crates/cli-commands/tests/acceptance_cli_commands.rs`, `crates/cli-commands/tests/contract_cli_commands.rs`; Gate: `CARGO_INCREMENTAL=0 cargo nextest run --workspace --lib --tests` → Tests job passed at `e6cb89989215f435141aa68992a71dce44bc243c` (CI run `45261257668`, 2026-11-23).
| In Progress | None |
| Risk | None |
| Updated | 2026-11-23 |

### DS-CLIC-006

| Aspect | Detail |
| -------------- | --------------------------------------------- |
| State | Done |
| Done | Request Shape; source: `crates/cli-commands/src/surface_report_command.rs`; tests: `crates/cli-commands/tests/unit_rules_aes_parity.rs`, `crates/cli-commands/tests/unit_cli_formatting.rs`, `crates/cli-commands/tests/unit_cli_report_delta.rs`; Gate: `CARGO_INCREMENTAL=0 cargo nextest run --workspace --lib --tests` → Tests job passed at `e6cb89989215f435141aa68992a71dce44bc243c` (CI run `45261257668`, 2026-11-23).
| In Progress | None |
| Risk | None |
| Updated | 2026-11-23 |

### DS-CLIC-007

| Aspect | Detail |
| -------------- | --------------------------------------------- |
| State | Done |
| Done | Invariants; source: `crates/cli-commands/src/lib.rs`, `crates/cli-commands/src/surface_scan_command.rs`; tests: `crates/cli-commands/tests/smoke_cli_commands.rs`, `crates/cli-commands/tests/dogfood_cli_commands.rs`; Gate: `CARGO_INCREMENTAL=0 cargo nextest run --workspace --lib --tests` → Tests job passed at `e6cb89989215f435141aa68992a71dce44bc243c` (CI run `45261257668`, 2026-11-23).
| In Progress | None |
| Risk | None |
| Updated | 2026-11-23 |

### DS-CLIC-008

| Aspect | Detail |
| -------------- | --------------------------------------------- |
| State | Done |
| Done | Change Checklist; source: `crates/cli-commands/src/surface_git_command.rs`, `crates/cli-commands/src/surface_report_command.rs`; tests: `crates/cli-commands/tests/unit_cli_report_delta.rs`, `crates/cli-commands/tests/unit_cli_watch_output.rs`; Gate: `CARGO_INCREMENTAL=0 cargo nextest run --workspace --lib --tests` → Tests job passed at `e6cb89989215f435141aa68992a71dce44bc243c` (CI run `45261257668`, 2026-11-23).
| In Progress | None |
| Risk | None |
| Updated | 2026-11-23 |

### DS-CLIC-009

| Aspect | Detail |
| -------------- | --------------------------------------------- |
| State | Done |
| Done | States; source: `crates/cli-commands/src/lib.rs`, `crates/cli-commands/src/surface_watch_command.rs`; tests: `crates/cli-commands/tests/unit_cli_watch_output.rs`, `crates/cli-commands/tests/acceptance_cli_commands.rs`; Gate: `CARGO_INCREMENTAL=0 cargo nextest run --workspace --lib --tests` → Tests job passed at `e6cb89989215f435141aa68992a71dce44bc243c` (CI run `45261257668`, 2026-11-23).
| In Progress | None |
| Risk | None |
| Updated | 2026-11-23 |

### DS-CLIC-010

| Aspect | Detail |
| -------------- | --------------------------------------------- |
| State | Done |
| Done | Error States; source: `crates/cli-commands/src/surface_scan_command.rs`, `crates/cli-commands/src/surface_maintenance_command.rs`; tests: `crates/cli-commands/tests/unit_cli_watch_output.rs`, `crates/cli-commands/tests/dogfood_cli_commands.rs`; Gate: `CARGO_INCREMENTAL=0 cargo nextest run --workspace --lib --tests` → Tests job passed at `e6cb89989215f435141aa68992a71dce44bc243c` (CI run `45261257668`, 2026-11-23).
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
