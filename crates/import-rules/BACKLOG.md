# Feature Backlog: Import Rules

FRD: [FRD.md](FRD.md)
Architecture: [ARCHITECTURE.md](../../ARCHITECTURE.md)
State: defined in root [ROADMAP.md](../../ROADMAP.md) — cited here, not restated
Last Updated: 2026-11-23

## Progress Overview

- Done: `CARGO_INCREMENTAL=0 cargo nextest run -p import-rules-lint-arwaky --lib --tests` → 112 tests, 0 failures at `54d2404d` (CI run `45203607718`, 2026-11-23). Per-FR source and test evidence below.
- In Progress: None
- Blocked: None
- Next Action: Re-run the import-rules test gate after any code change to this crate

## Backlog Status

### FR-ImportRules-001

| Aspect | Detail |
|--------|--------|
| State | Done |
| Done | AES201 forbidden-layer checker (`ArchImportForbiddenChecker` / `IImportForbiddenProtocol` in `crates/import-rules/src/capabilities_import_forbidden_checker.rs`), wired via `ImportOrchestrator`; exercised by `acceptance_AES201.rs` (22), `unit_import_rules_forbidden_checker.rs` (5), `e2e_import_flow.rs` (5), `integration_import_rules.rs` (8), `unit_agent_import_orchestrator.rs` (4). Gate: `CARGO_INCREMENTAL=0 cargo nextest run -p import-rules-lint-arwaky --lib --tests` → 112 tests, 0 failures at `54d2404d` (CI run `45203607718`). |
| In Progress | None |
| Risk | None |
| Updated | 2026-11-23 |

### FR-ImportRules-002

| Aspect | Detail |
|--------|--------|
| State | Done |
| Done | AES202 mandatory-import checker (`ArchImportMandatoryChecker` / `IImportMandatoryProtocol` in `crates/import-rules/src/capabilities_import_mandatory_checker.rs`), wired via `ImportOrchestrator`; exercised by `smoke_import_rules.rs` (11), `unit_agent_import_orchestrator.rs` (4), `contract_import_rules.rs` (6). Gate: `CARGO_INCREMENTAL=0 cargo nextest run -p import-rules-lint-arwaky --lib --tests` → 112 tests, 0 failures at `54d2404d` (CI run `45203607718`). |
| In Progress | None |
| Risk | None |
| Updated | 2026-11-23 |

### FR-ImportRules-003

| Aspect | Detail |
|--------|--------|
| State | Done |
| Done | AES203 unused-import checker (`UnusedImportRuleChecker` / `IUnusedImportProtocol` in `crates/import-rules/src/capabilities_import_unused_checker.rs`), wired via `ImportOrchestrator`; exercised by `acceptance_AES203.rs` (24), `unit_import_rules_unused_checker.rs` (10), `integration_import_rules.rs` (8), `unit_agent_import_orchestrator.rs` (4). Gate: `CARGO_INCREMENTAL=0 cargo nextest run -p import-rules-lint-arwaky --lib --tests` → 112 tests, 0 failures at `54d2404d` (CI run `45203607718`). |
| In Progress | None |
| Risk | None |
| Updated | 2026-11-23 |

### FR-ImportRules-004

| Aspect | Detail |
|--------|--------|
| State | Done |
| Done | AES204 dummy-import checker (`DummyImportChecker` / `IDummyImportCheckerProtocol` in `crates/import-rules/src/capabilities_dummy_import_checker.rs`), wired via `ImportOrchestrator`; exercised by `smoke_import_rules.rs` (11), `integration_import_rules.rs` (8), `unit_agent_import_orchestrator.rs` (4), `contract_import_rules.rs` (6). Gate: `CARGO_INCREMENTAL=0 cargo nextest run -p import-rules-lint-arwaky --lib --tests` → 112 tests, 0 failures at `54d2404d` (CI run `45203607718`). |
| In Progress | None |
| Risk | None |
| Updated | 2026-11-23 |

### FR-ImportRules-005

| Aspect | Detail |
|--------|--------|
| State | Done |
| Done | ImportRules |
| In Progress | None |
| Risk | None |
| Updated | 2026-11-10 |

## Reference

| Artifact | Purpose | Link |
|----------|---------|------|
| PRD.md | Product requirements | [PRD.md](../../PRD.md) |
| FRD.md | Feature requirements | [FRD.md](FRD.md) |
| ROADMAP.md | Roadmap status | [ROADMAP.md](../../ROADMAP.md) |
| ARCHITECTURE.md | Layer contract | [ARCHITECTURE.md](../../ARCHITECTURE.md) |
