# Feature Backlog: Role Rules

FRD: [FRD.md](FRD.md)
Architecture: [ARCHITECTURE.md](../../ARCHITECTURE.md)
State: defined in root [ROADMAP.md](../../ROADMAP.md) — cited here, not restated
Last Updated: 2026-11-23

## Progress Overview

- Done: All 7 backlog rows reconciled to Done at HEAD; gate: `CARGO_INCREMENTAL=0 cargo nextest run --workspace --lib --tests` → Tests job passed at `e6cb89989215f435141aa68992a71dce44bc243c` (CI run `45261257668`, 2026-11-23). Per-row source and test evidence below.
- Done: `cargo nextest run -p role-rules-lint-arwaky` → 222 passed, 0 failed at `72d2d58b` (2026-09-28). Per-scenario evidence below.
- In Progress: None
- Blocked: None
- Next Action: Re-run `cargo nextest run -p role-rules-lint-arwaky` after any code change to this crate

## Backlog Status

### FR-RoleRules-001

| Aspect | Detail |
| -------------- | --------------------------------------------- |
| State | Done |
| Done | File Classification and Dispatch; source: `crates/role-rules/src/capabilities_taxonomy_role_auditor.rs`, `crates/role-rules/src/agent_role_orchestrator.rs`; tests: `crates/role-rules/tests/acceptance_AES406.rs`, `crates/role-rules/tests/acceptance_AES401.rs`, `crates/role-rules/tests/unit_role_rules_surface_role_auditor.rs`; Gate: `CARGO_INCREMENTAL=0 cargo nextest run --workspace --lib --tests` → Tests job passed at `e6cb89989215f435141aa68992a71dce44bc243c` (CI run `45261257668`, 2026-11-23).
| In Progress | None |
| Risk | None |
| Updated | 2026-11-23 |

### FR-RoleRules-002

| Aspect | Detail |
| -------------- | --------------------------------------------- |
| State | Done |
| Done | Taxonomy Purity and Primitive Restriction (AES401); source: `crates/role-rules/src/capabilities_taxonomy_role_auditor.rs`, `crates/role-rules/src/agent_role_orchestrator.rs`; tests: `crates/role-rules/tests/acceptance_AES401.rs`, `crates/role-rules/tests/e2e_role_flow.rs`, `crates/role-rules/tests/integration_role_rules.rs`; Gate: `CARGO_INCREMENTAL=0 cargo nextest run --workspace --lib --tests` → Tests job passed at `e6cb89989215f435141aa68992a71dce44bc243c` (CI run `45261257668`, 2026-11-23).
| In Progress | None |
| Risk | None |
| Updated | 2026-11-23 |

### FR-RoleRules-003

| Aspect | Detail |
| -------------- | --------------------------------------------- |
| State | Done |
| Done | Contract Primitive Restriction (AES402); source: `crates/role-rules/src/capabilities_contract_ts_role_auditor.rs`, `crates/role-rules/src/capabilities_contract_rust_role_auditor.rs`; tests: `crates/role-rules/tests/acceptance_AES402.rs`, `crates/role-rules/tests/unit_utility_signature_parser.rs`, `crates/role-rules/tests/unit_role_rules_agent_composition.rs`; Gate: `CARGO_INCREMENTAL=0 cargo nextest run --workspace --lib --tests` → Tests job passed at `e6cb89989215f435141aa68992a71dce44bc243c` (CI run `45261257668`, 2026-11-23).
| In Progress | None |
| Risk | None |
| Updated | 2026-11-23 |

### FR-RoleRules-004

| Aspect | Detail |
| -------------- | --------------------------------------------- |
| State | Done |
| Done | Capability Protocol Implementation (AES403); source: `crates/role-rules/src/capabilities_capabilities_rust_role_auditor.rs`, `crates/role-rules/src/capabilities_capabilities_ts_role_auditor.rs`; tests: `crates/role-rules/tests/acceptance_AES403.rs`, `crates/role-rules/tests/unit_role_rules_capabilities_role_auditor.rs`, `crates/role-rules/tests/unit_role_rules_capabilities_role_auditor_single_protocol.rs`; Gate: `CARGO_INCREMENTAL=0 cargo nextest run --workspace --lib --tests` → Tests job passed at `e6cb89989215f435141aa68992a71dce44bc243c` (CI run `45261257668`, 2026-11-23).
| In Progress | None |
| Risk | None |
| Updated | 2026-11-23 |

### FR-RoleRules-005

| Aspect | Detail |
| -------------- | --------------------------------------------- |
| State | Done |
| Done | Utility Purity (AES404); source: `crates/role-rules/src/capabilities_utility_ts_role_auditor.rs`, `crates/role-rules/src/agent_role_orchestrator.rs`; tests: `crates/role-rules/tests/acceptance_AES404.rs`, `crates/role-rules/tests/e2e_role_flow.rs`, `crates/role-rules/tests/integration_role_rules.rs`; Gate: `CARGO_INCREMENTAL=0 cargo nextest run --workspace --lib --tests` → Tests job passed at `e6cb89989215f435141aa68992a71dce44bc243c` (CI run `45261257668`, 2026-11-23).
| In Progress | None |
| Risk | None |
| Updated | 2026-11-23 |

### FR-RoleRules-006

| Aspect | Detail |
| -------------- | --------------------------------------------- |
| State | Done |
| Done | Agent Orchestrator Composition (AES405); source: `crates/role-rules/src/capabilities_agent_rust_role_auditor.rs`, `crates/role-rules/src/capabilities_agent_ts_role_auditor.rs`; tests: `crates/role-rules/tests/acceptance_AES405.rs`, `crates/role-rules/tests/unit_role_rules_agent_rust_role_auditor.rs`, `crates/role-rules/tests/unit_role_rules_agent_python_role_auditor.rs`; Gate: `CARGO_INCREMENTAL=0 cargo nextest run --workspace --lib --tests` → Tests job passed at `e6cb89989215f435141aa68992a71dce44bc243c` (CI run `45261257668`, 2026-11-23).
| In Progress | None |
| Risk | None |
| Updated | 2026-11-23 |

### FR-RoleRules-007

| Aspect | Detail |
| -------------- | --------------------------------------------- |
| State | Done |
| Done | Surface Passive Role (AES406); source: `crates/role-rules/src/capabilities_surface_role_auditor.rs`, `crates/role-rules/src/agent_role_orchestrator.rs`; tests: `crates/role-rules/tests/acceptance_AES406.rs`, `crates/role-rules/tests/unit_role_rules_surface_role_auditor.rs`, `crates/role-rules/tests/contract_role_rules.rs`; Gate: `CARGO_INCREMENTAL=0 cargo nextest run --workspace --lib --tests` → Tests job passed at `e6cb89989215f435141aa68992a71dce44bc243c` (CI run `45261257668`, 2026-11-23).
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
