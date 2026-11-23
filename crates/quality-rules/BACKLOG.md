# Feature Backlog: Quality Rules

FRD: [FRD.md](FRD.md)
Architecture: [ARCHITECTURE.md](../../ARCHITECTURE.md)
State: defined in root [ROADMAP.md](../../ROADMAP.md) — cited here, not restated
Last Updated: 2026-11-23

## Progress Overview

- Done: All 5 backlog rows reconciled to Done at HEAD; gate: `CARGO_INCREMENTAL=0 cargo nextest run --workspace --lib --tests` → Tests job passed at `e6cb89989215f435141aa68992a71dce44bc243c` (CI run `45261257668`, 2026-11-23). Per-row source and test evidence below.
- Done: `CARGO_INCREMENTAL=0 cargo nextest run -p quality-rules-lint-arwaky --lib --tests -j 2` → 149 passed, 0 skipped (2026-11-10). Includes the new empty-file, duplication-cardinality, Python-vocabulary, AES305-message, and AES304-kind-naming acceptance tests.
- In Progress: None
- Blocked: None
- Next Action: Re-run `CARGO_INCREMENTAL=0 cargo nextest run -p quality-rules-lint-arwaky --lib --tests -j 2` after any code change to this crate

## Backlog Status

### FR-QualityRules-001

| Aspect | Detail |
| -------------- | --------------------------------------------- |
| State | Done |
| Done | File Line Count Limits (AES301 & AES302) (`ILineCheckerProtocol`); source: `crates/quality-rules/src/capabilities_line_checker.rs`, `crates/quality-rules/src/agent_quality_orchestrator.rs`; tests: `crates/quality-rules/tests/acceptance_AES302.rs`, `crates/quality-rules/tests/integration_quality_rules.rs`, `crates/quality-rules/tests/unit_quality_rules_line_checker.rs`; Gate: `CARGO_INCREMENTAL=0 cargo nextest run --workspace --lib --tests` → Tests job passed at `e6cb89989215f435141aa68992a71dce44bc243c` (CI run `45261257668`, 2026-11-23).
| In Progress | None |
| Risk | None |
| Updated | 2026-11-23 |

### FR-QualityRules-002

| Aspect | Detail |
| -------------- | --------------------------------------------- |
| State | Done |
| Done | Mandatory Definitions & Dead Inheritance (AES303) (`IMandatoryClassProtocol, `IDeadInheritanceProtocol`); source: `crates/quality-rules/src/capabilities_mandatory_definition_checker.rs`, `crates/quality-rules/src/agent_quality_orchestrator.rs`; tests: `crates/quality-rules/tests/acceptance_AES303.rs`, `crates/quality-rules/tests/unit_quality_rules_mandatory_definition_checker.rs`, `crates/quality-rules/tests/contract_quality_rules.rs`; Gate: `CARGO_INCREMENTAL=0 cargo nextest run --workspace --lib --tests` → Tests job passed at `e6cb89989215f435141aa68992a71dce44bc243c` (CI run `45261257668`, 2026-11-23).
| In Progress | None |
| Risk | None |
| Updated | 2026-11-23 |

### FR-QualityRules-003

| Aspect | Detail |
| -------------- | --------------------------------------------- |
| State | Done |
| Done | Bypass Detection (AES304) (`IBypassCheckerProtocol`); source: `crates/quality-rules/src/capabilities_check_bypass_checker.rs`, `crates/quality-rules/src/agent_quality_orchestrator.rs`; tests: `crates/quality-rules/tests/integration_quality_rules.rs`, `crates/quality-rules/tests/acceptance_AES304.rs`, `crates/quality-rules/tests/e2e_quality_flow.rs`; Gate: `CARGO_INCREMENTAL=0 cargo nextest run --workspace --lib --tests` → Tests job passed at `e6cb89989215f435141aa68992a71dce44bc243c` (CI run `45261257668`, 2026-11-23).
| In Progress | None |
| Risk | None |
| Updated | 2026-11-23 |

### FR-QualityRules-004

| Aspect | Detail |
| -------------- | --------------------------------------------- |
| State | Done |
| Done | Duplicate Code Detection (AES305) (`ICodeMetricAnalyzerProtocol`); source: `crates/quality-rules/src/capabilities_code_duplication_analyzer.rs`, `crates/quality-rules/src/agent_quality_orchestrator.rs`; tests: `crates/quality-rules/tests/acceptance_AES305.rs`, `crates/quality-rules/tests/integration_quality_rules.rs`, `crates/quality-rules/tests/contract_quality_rules.rs`; Gate: `CARGO_INCREMENTAL=0 cargo nextest run --workspace --lib --tests` → Tests job passed at `e6cb89989215f435141aa68992a71dce44bc243c` (CI run `45261257668`, 2026-11-23).
| In Progress | None |
| Risk | None |
| Updated | 2026-11-23 |

### FR-QualityRules-005

| Aspect | Detail |
| -------------- | --------------------------------------------- |
| State | Done |
| Done | Per-Rule Enable Flags (UX-S5-002) (`ICodeAnalysisAggregate`); source: `crates/quality-rules/src/agent_quality_orchestrator.rs`, `crates/quality-rules/src/capabilities_check_bypass_checker.rs`; tests: `crates/quality-rules/tests/integration_quality_rules.rs`, `crates/quality-rules/tests/acceptance_AES304.rs`, `crates/quality-rules/tests/e2e_quality_flow.rs`; Gate: `CARGO_INCREMENTAL=0 cargo nextest run --workspace --lib --tests` → Tests job passed at `e6cb89989215f435141aa68992a71dce44bc243c` (CI run `45261257668`, 2026-11-23).
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
