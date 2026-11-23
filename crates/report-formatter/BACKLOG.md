# Feature Backlog: Report Formatter

FRD: [FRD.md](FRD.md)
Architecture: [ARCHITECTURE.md](../../ARCHITECTURE.md)
State: defined in root [ROADMAP.md](../../ROADMAP.md) — cited here, not restated
Last Updated: 2026-11-23

## Progress Overview

- Done: All 4 backlog rows reconciled to Done at HEAD; gate: `CARGO_INCREMENTAL=0 cargo nextest run --workspace --lib --tests` → Tests job passed at `e6cb89989215f435141aa68992a71dce44bc243c` (CI run `45261257668`, 2026-11-23). Per-row source and test evidence below.
- Done: `cargo nextest run --workspace --lib --tests` → 2048 passed, 0 failed at `cc63389a` (2026-09-29). Per-scenario evidence below.
- In Progress: None
- Blocked: None
- Next Action: Re-run `cargo nextest run --workspace --lib --tests` after any code change to this crate

## Backlog Status

### FR-ReportFormatter-001

| Aspect | Detail |
| -------------- | --------------------------------------------- |
| State | Done |
| Done | Render Text Format Output (`ITextFormatProtocol`); source: `crates/report-formatter/src/capabilities_text_formatter.rs`, `crates/report-formatter/src/agent_report_formatter_orchestrator.rs`; tests: `crates/report-formatter/tests/unit_report_formatter_text.rs`, `crates/config-system/tests/acceptance_FR_001.rs`, `crates/report-formatter/tests/e2e_report_formatter_flow.rs`; Gate: `CARGO_INCREMENTAL=0 cargo nextest run --workspace --lib --tests` → Tests job passed at `e6cb89989215f435141aa68992a71dce44bc243c` (CI run `45261257668`, 2026-11-23).
| In Progress | None |
| Risk | None |
| Updated | 2026-11-23 |

### FR-ReportFormatter-002

| Aspect | Detail |
| -------------- | --------------------------------------------- |
| State | Done |
| Done | Render JSON Format Output (`IJsonFormatProtocol`); source: `crates/report-formatter/src/capabilities_json_formatter.rs`, `crates/report-formatter/src/agent_report_formatter_orchestrator.rs`; tests: `crates/report-formatter/tests/unit_report_formatter_json.rs`, `crates/report-formatter/tests/e2e_report_formatter_flow.rs`, `crates/report-formatter/tests/contract_report_formatter.rs`; Gate: `CARGO_INCREMENTAL=0 cargo nextest run --workspace --lib --tests` → Tests job passed at `e6cb89989215f435141aa68992a71dce44bc243c` (CI run `45261257668`, 2026-11-23).
| In Progress | None |
| Risk | None |
| Updated | 2026-11-23 |

### FR-ReportFormatter-003

| Aspect | Detail |
| -------------- | --------------------------------------------- |
| State | Done |
| Done | Render SARIF 2.1.0 Format Output (`ISarifFormatProtocol`); source: `crates/report-formatter/src/capabilities_sarif_formatter.rs`, `crates/report-formatter/src/agent_report_formatter_orchestrator.rs`; tests: `crates/report-formatter/tests/unit_report_formatter_sarif.rs`, `crates/config-system/tests/acceptance_FR_003.rs`, `crates/report-formatter/tests/e2e_report_formatter_flow.rs`; Gate: `CARGO_INCREMENTAL=0 cargo nextest run --workspace --lib --tests` → Tests job passed at `e6cb89989215f435141aa68992a71dce44bc243c` (CI run `45261257668`, 2026-11-23).
| In Progress | None |
| Risk | None |
| Updated | 2026-11-23 |

### FR-ReportFormatter-004

| Aspect | Detail |
| -------------- | --------------------------------------------- |
| State | Done |
| Done | Render JUnit XML Format Output (`IJUnitFormatProtocol, `IReportFormatterAggregate`); source: `crates/report-formatter/src/capabilities_junit_formatter.rs`, `crates/report-formatter/src/agent_report_formatter_orchestrator.rs`; tests: `crates/report-formatter/tests/unit_report_formatter_junit.rs`, `crates/report-formatter/tests/contract_report_formatter.rs`, `crates/report-formatter/tests/e2e_report_formatter_flow.rs`; Gate: `CARGO_INCREMENTAL=0 cargo nextest run --workspace --lib --tests` → Tests job passed at `e6cb89989215f435141aa68992a71dce44bc243c` (CI run `45261257668`, 2026-11-23).
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
