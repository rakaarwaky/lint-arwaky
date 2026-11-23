# Feature Backlog: Orphan Rules

FRD: [FRD.md](FRD.md)
Architecture: [ARCHITECTURE.md](../../ARCHITECTURE.md)
State: defined in root [ROADMAP.md](../../ROADMAP.md) — cited here, not restated
Last Updated: 2026-11-23

## Progress Overview

- Done: All 7 backlog rows reconciled to Done at HEAD; gate: `CARGO_INCREMENTAL=0 cargo nextest run --workspace --lib --tests` → Tests job passed at `e6cb89989215f435141aa68992a71dce44bc243c` (CI run `45261257668`, 2026-11-23). Per-row source and test evidence below.
- Done: `cargo test -p orphan_rules --lib --tests` → 0 failures at `29c71083` (2026-09-17). Per-scenario evidence below.
- In Progress: None
- Blocked: None
- Next Action: Re-run `cargo test -p orphan_rules --lib --tests` after any code change to this crate

## Backlog Status

### FR-OrphanRules-001

| Aspect | Detail |
| -------------- | --------------------------------------------- |
| State | Done |
| Done | Build Graph Context, Entry Points, and Reachability (`IOrphanParserProtocol`); source: `crates/orphan-rules/src/agent_orphan_orchestrator.rs`; tests: `crates/orphan-rules/tests/e2e_orphan_flow.rs`, `crates/orphan-rules/tests/acceptance_AES503_DI.rs`, `crates/orphan-rules/tests/acceptance_AES503.rs`; Gate: `CARGO_INCREMENTAL=0 cargo nextest run --workspace --lib --tests` → Tests job passed at `e6cb89989215f435141aa68992a71dce44bc243c` (CI run `45261257668`, 2026-11-23).
| In Progress | None |
| Risk | None |
| Updated | 2026-11-23 |

### FR-OrphanRules-002

| Aspect | Detail |
| -------------- | --------------------------------------------- |
| State | Done |
| Done | Taxonomy Orphan Detection (AES501) (`ITaxonomyOrphanProtocol`); source: `crates/orphan-rules/src/capabilities_orphan_taxonomy_analyzer.rs`, `crates/orphan-rules/src/agent_orphan_orchestrator.rs`; tests: `crates/orphan-rules/tests/acceptance_AES501.rs`, `crates/orphan-rules/tests/integration_orphan_rules.rs`, `crates/orphan-rules/tests/unit_orphan_rules_taxonomy_analyzer.rs`; Gate: `CARGO_INCREMENTAL=0 cargo nextest run --workspace --lib --tests` → Tests job passed at `e6cb89989215f435141aa68992a71dce44bc243c` (CI run `45261257668`, 2026-11-23).
| In Progress | None |
| Risk | None |
| Updated | 2026-11-23 |

### FR-OrphanRules-003

| Aspect | Detail |
| -------------- | --------------------------------------------- |
| State | Done |
| Done | Contract Orphan Detection (AES502) (`IContractOrphanProtocol`); source: `crates/orphan-rules/src/capabilities_orphan_contract_analyzer.rs`, `crates/orphan-rules/src/agent_orphan_orchestrator.rs`; tests: `crates/orphan-rules/tests/acceptance_AES502.rs`, `crates/orphan-rules/tests/acceptance_AES503_DI.rs`, `crates/orphan-rules/tests/acceptance_AES503.rs`; Gate: `CARGO_INCREMENTAL=0 cargo nextest run --workspace --lib --tests` → Tests job passed at `e6cb89989215f435141aa68992a71dce44bc243c` (CI run `45261257668`, 2026-11-23).
| In Progress | None |
| Risk | None |
| Updated | 2026-11-23 |

### FR-OrphanRules-004

| Aspect | Detail |
| -------------- | --------------------------------------------- |
| State | Done |
| Done | Capabilities Orphan Detection (AES503) (`ICapabilitiesOrphanProtocol`); source: `crates/orphan-rules/src/capabilities_orphan_capabilities_analyzer.rs`, `crates/orphan-rules/src/agent_orphan_orchestrator.rs`; tests: `crates/orphan-rules/tests/acceptance_AES503.rs`, `crates/orphan-rules/tests/acceptance_AES503_DI.rs`, `crates/orphan-rules/tests/acceptance_AES502.rs`; Gate: `CARGO_INCREMENTAL=0 cargo nextest run --workspace --lib --tests` → Tests job passed at `e6cb89989215f435141aa68992a71dce44bc243c` (CI run `45261257668`, 2026-11-23).
| In Progress | None |
| Risk | None |
| Updated | 2026-11-23 |

### FR-OrphanRules-005

| Aspect | Detail |
| -------------- | --------------------------------------------- |
| State | Done |
| Done | Utility Orphan Detection (AES504) (`IUtilityOrphanProtocol`); source: `crates/orphan-rules/src/capabilities_orphan_utility_analyzer.rs`, `crates/orphan-rules/src/agent_orphan_orchestrator.rs`; tests: `crates/orphan-rules/tests/acceptance_AES504.rs`, `crates/orphan-rules/tests/acceptance_AES502.rs`, `crates/orphan-rules/tests/acceptance_AES503.rs`; Gate: `CARGO_INCREMENTAL=0 cargo nextest run --workspace --lib --tests` → Tests job passed at `e6cb89989215f435141aa68992a71dce44bc243c` (CI run `45261257668`, 2026-11-23).
| In Progress | None |
| Risk | None |
| Updated | 2026-11-23 |

### FR-OrphanRules-006

| Aspect | Detail |
| -------------- | --------------------------------------------- |
| State | Done |
| Done | Agent Orphan Detection (AES505) (`IAgentOrphanProtocol`); source: `crates/orphan-rules/src/agent_orphan_orchestrator.rs`, `crates/orphan-rules/src/capabilities_orphan_agent_analyzer.rs`; tests: `crates/orphan-rules/tests/acceptance_AES505.rs`, `crates/orphan-rules/tests/acceptance_AES503.rs`, `crates/orphan-rules/tests/acceptance_AES502.rs`; Gate: `CARGO_INCREMENTAL=0 cargo nextest run --workspace --lib --tests` → Tests job passed at `e6cb89989215f435141aa68992a71dce44bc243c` (CI run `45261257668`, 2026-11-23).
| In Progress | None |
| Risk | None |
| Updated | 2026-11-23 |

### FR-OrphanRules-007

| Aspect | Detail |
| -------------- | --------------------------------------------- |
| State | Done |
| Done | Surface Orphan Detection (AES506) (`ISurfacesOrphanProtocol, `IOrphanAggregate`); source: `crates/orphan-rules/src/agent_orphan_orchestrator.rs`, `crates/orphan-rules/src/root_orphan_detector_container.rs`; tests: `crates/orphan-rules/tests/acceptance_AES506.rs`, `crates/orphan-rules/tests/acceptance_AES503.rs`, `crates/orphan-rules/tests/contract_orphan_rules.rs`; Gate: `CARGO_INCREMENTAL=0 cargo nextest run --workspace --lib --tests` → Tests job passed at `e6cb89989215f435141aa68992a71dce44bc243c` (CI run `45261257668`, 2026-11-23).
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
