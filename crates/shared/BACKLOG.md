# shared — BACKLOG

FRD: —
Architecture: [ARCHITECTURE.md](../ARCHITECTURE.md)
State: defined in root [ROADMAP.md](../ROADMAP.md) — cited here, not restated
Last Updated: 2026-11-23

## Progress Overview

- Done: All 7 backlog rows reconciled to Done at HEAD; gate: `CARGO_INCREMENTAL=0 cargo nextest run --workspace --lib --tests` → Tests job passed at `e6cb89989215f435141aa68992a71dce44bc243c` (CI run `45261257668`, 2026-11-23). Per-row source and test evidence below.
- Done: `cargo test -p shared --lib --tests` → 0 failures
- In Progress: None
- Blocked: None
- Next Action: —

## Backlog Status

### DT-SHARED-001

| Aspect | Detail |
| -------------- | -------- |
| State | Done |
| Done | Severity; source: `crates/shared/src/common/taxonomy_severity_vo.rs`, `crates/shared/src/common/taxonomy_violation_item_vo.rs`; tests: `crates/shared/tests/unit_shared_common_vo.rs`, `crates/shared/tests/contract_shared_traits.rs`; Gate: `CARGO_INCREMENTAL=0 cargo nextest run --workspace --lib --tests` → Tests job passed at `e6cb89989215f435141aa68992a71dce44bc243c` (CI run `45261257668`, 2026-11-23).
| In Progress | None |
| Risk | None |
| Updated | 2026-11-23 |

### DT-SHARED-002

| Aspect | Detail |
| -------------- | -------- |
| State | Done |
| Done | ViolationItem; source: `crates/shared/src/common/taxonomy_violation_item_vo.rs`, `crates/shared/src/cli_commands/utility_output_text_formatter.rs`; tests: `crates/shared/tests/contract_shared.rs`, `crates/shared/tests/integration_shared.rs`; Gate: `CARGO_INCREMENTAL=0 cargo nextest run --workspace --lib --tests` → Tests job passed at `e6cb89989215f435141aa68992a71dce44bc243c` (CI run `45261257668`, 2026-11-23).
| In Progress | None |
| Risk | None |
| Updated | 2026-11-23 |

### DT-SHARED-003

| Aspect | Detail |
| -------------- | -------- |
| State | Done |
| Done | StructureFinding; source: `crates/shared/src/structure_rules/taxonomy_structure_rules_request.rs`, `crates/shared/src/structure_rules/taxonomy_structure_rules_response.rs`; tests: `crates/shared/tests/contract_shared_traits.rs`, `crates/shared/tests/integration_shared.rs`; Gate: `CARGO_INCREMENTAL=0 cargo nextest run --workspace --lib --tests` → Tests job passed at `e6cb89989215f435141aa68992a71dce44bc243c` (CI run `45261257668`, 2026-11-23).
| In Progress | None |
| Risk | None |
| Updated | 2026-11-23 |

### DT-SHARED-004

| Aspect | Detail |
| -------------- | -------- |
| State | Done |
| Done | DocFinding; source: `crates/shared/src/doc_rules/contract_doc_protocol.rs`, `crates/shared/src/doc_rules/taxonomy_doc_rules_defect_vo.rs`; tests: `crates/shared/tests/contract_shared_traits.rs`, `crates/shared/tests/integration_shared.rs`; Gate: `CARGO_INCREMENTAL=0 cargo nextest run --workspace --lib --tests` → Tests job passed at `e6cb89989215f435141aa68992a71dce44bc243c` (CI run `45261257668`, 2026-11-23).
| In Progress | None |
| Risk | None |
| Updated | 2026-11-23 |

### DT-SHARED-005

| Aspect | Detail |
| -------------- | -------- |
| State | Done |
| Done | ReportSnapshot; source: `crates/shared/src/common/taxonomy_report_snapshot_vo.rs`, `crates/shared/src/cli_commands/utility_report_snapshot.rs`; tests: `crates/shared/tests/contract_shared.rs`, `crates/shared/tests/unit_shared_common_vo.rs`; Gate: `CARGO_INCREMENTAL=0 cargo nextest run --workspace --lib --tests` → Tests job passed at `e6cb89989215f435141aa68992a71dce44bc243c` (CI run `45261257668`, 2026-11-23).
| In Progress | None |
| Risk | None |
| Updated | 2026-11-23 |

### DT-SHARED-006

| Aspect | Detail |
| -------------- | -------- |
| State | Done |
| Done | SnapshotStore; source: `crates/shared/src/cli_commands/utility_report_snapshot.rs`, `crates/shared/src/report_formatter/mod.rs`; tests: `crates/shared/tests/integration_shared.rs`, `crates/shared/tests/contract_shared.rs`; Gate: `CARGO_INCREMENTAL=0 cargo nextest run --workspace --lib --tests` → Tests job passed at `e6cb89989215f435141aa68992a71dce44bc243c` (CI run `45261257668`, 2026-11-23).
| In Progress | None |
| Risk | None |
| Updated | 2026-11-23 |

### DT-SHARED-007

| Aspect | Detail |
| -------------- | -------- |
| State | Done |
| Done | MemberDelta; source: `crates/shared/src/cli_commands/utility_report_delta.rs`, `crates/shared/src/common/taxonomy_report_snapshot_vo.rs`; tests: `crates/shared/tests/integration_shared.rs`, `crates/shared/tests/contract_shared.rs`; Gate: `CARGO_INCREMENTAL=0 cargo nextest run --workspace --lib --tests` → Tests job passed at `e6cb89989215f435141aa68992a71dce44bc243c` (CI run `45261257668`, 2026-11-23).
| In Progress | None |
| Risk | None |
| Updated | 2026-11-23 |

## Reference

| Artifact | Purpose | Link |
| ---------- | --------- | ------ |
| PRD.md | Product requirements | [PRD.md](../PRD.md) |
| DATA.md | Data contracts | [DATA.md](DATA.md) |
| ROADMAP.md | Roadmap status | [ROADMAP.md](../ROADMAP.md) |
| ARCHITECTURE.md | Layer contract | [ARCHITECTURE.md](../ARCHITECTURE.md) |
