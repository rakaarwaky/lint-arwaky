# Feature Backlog: Doc Rules

FRD: [FRD.md](FRD.md)
Architecture: [ARCHITECTURE.md](../../ARCHITECTURE.md)
State: defined in root [ROADMAP.md](../../ROADMAP.md) — cited here, not restated
Last Updated: 2026-11-23

## Progress Overview

- Done: All 10 backlog rows reconciled to Done at HEAD; gate: `CARGO_INCREMENTAL=0 cargo nextest run --workspace --lib --tests` → Tests job passed at `e6cb89989215f435141aa68992a71dce44bc243c` (CI run `45261257668`, 2026-11-23). Per-row source and test evidence below.
- Done: `cargo nextest run -p doc-rules-lint-arwaky` → 149 passed, 0 failed. Per-scenario evidence below.
- In Progress: None
- Blocked: None
- Next Action: Re-run `cargo nextest run -p doc-rules-lint-arwaky` after any code change to this crate

## Backlog Status

### FR-DOC-001

| Aspect | Detail |
| -------------- | --------------------------------------------- |
| State | Done |
| Done | Audit AGENTS.md (AES601) (`IAgentsDocProtocol`); source: `crates/doc-rules/src/capabilities_doc_agents.rs`, `crates/doc-rules/src/capabilities_doc_design.rs`; tests: `crates/doc-rules/tests/unit_doc_rules.rs`; Gate: `CARGO_INCREMENTAL=0 cargo nextest run --workspace --lib --tests` → Tests job passed at `e6cb89989215f435141aa68992a71dce44bc243c` (CI run `45261257668`, 2026-11-23).
| In Progress | None |
| Risk | None |
| Updated | 2026-11-23 |

### FR-DOC-002

| Aspect | Detail |
| -------------- | --------------------------------------------- |
| State | Done |
| Done | Audit README.md (AES602) (`IReadmeDocProtocol`); source: `crates/doc-rules/src/capabilities_doc_readme.rs`, `crates/doc-rules/src/capabilities_doc_frd.rs`; Gate: `CARGO_INCREMENTAL=0 cargo nextest run --workspace --lib --tests` → Tests job passed at `e6cb89989215f435141aa68992a71dce44bc243c` (CI run `45261257668`, 2026-11-23).
| In Progress | None |
| Risk | None |
| Updated | 2026-11-23 |

### FR-DOC-003

| Aspect | Detail |
| -------------- | --------------------------------------------- |
| State | Done |
| Done | Audit ARCHITECTURE.md (AES603) (`IArchitectureDocProtocol`); source: `crates/doc-rules/src/capabilities_doc_architecture.rs`, `crates/doc-rules/src/agent_doc_orchestrator.rs`; Gate: `CARGO_INCREMENTAL=0 cargo nextest run --workspace --lib --tests` → Tests job passed at `e6cb89989215f435141aa68992a71dce44bc243c` (CI run `45261257668`, 2026-11-23).
| In Progress | None |
| Risk | None |
| Updated | 2026-11-23 |

### FR-DOC-004

| Aspect | Detail |
| -------------- | --------------------------------------------- |
| State | Done |
| Done | Audit CONTRIBUTING.md (AES604) (`IContributingDocProtocol`); source: `crates/doc-rules/src/capabilities_doc_contributing.rs`, `crates/doc-rules/src/agent_doc_orchestrator.rs`; tests: `crates/doc-rules/tests/unit_doc_rules.rs`; Gate: `CARGO_INCREMENTAL=0 cargo nextest run --workspace --lib --tests` → Tests job passed at `e6cb89989215f435141aa68992a71dce44bc243c` (CI run `45261257668`, 2026-11-23).
| In Progress | None |
| Risk | None |
| Updated | 2026-11-23 |

### FR-DOC-005

| Aspect | Detail |
| -------------- | --------------------------------------------- |
| State | Done |
| Done | Audit PRD.md (AES605) (`IPrdDocProtocol`); source: `crates/doc-rules/src/capabilities_doc_prd.rs`, `crates/doc-rules/src/capabilities_doc_frd.rs`; tests: `crates/doc-rules/tests/unit_doc_rules.rs`; Gate: `CARGO_INCREMENTAL=0 cargo nextest run --workspace --lib --tests` → Tests job passed at `e6cb89989215f435141aa68992a71dce44bc243c` (CI run `45261257668`, 2026-11-23).
| In Progress | None |
| Risk | None |
| Updated | 2026-11-23 |

### FR-DOC-006

| Aspect | Detail |
| -------------- | --------------------------------------------- |
| State | Done |
| Done | Audit ROADMAP.md (AES606) (`IRoadmapDocProtocol`); source: `crates/doc-rules/src/capabilities_doc_roadmap.rs`, `crates/doc-rules/src/agent_doc_orchestrator.rs`; Gate: `CARGO_INCREMENTAL=0 cargo nextest run --workspace --lib --tests` → Tests job passed at `e6cb89989215f435141aa68992a71dce44bc243c` (CI run `45261257668`, 2026-11-23).
| In Progress | None |
| Risk | None |
| Updated | 2026-11-23 |

### FR-DOC-007

| Aspect | Detail |
| -------------- | --------------------------------------------- |
| State | Done |
| Done | Audit BACKLOG.md (AES607) (`IBacklogDocProtocol`); source: `crates/doc-rules/src/capabilities_doc_backlog.rs`, `crates/doc-rules/src/agent_doc_orchestrator.rs`; Gate: `CARGO_INCREMENTAL=0 cargo nextest run --workspace --lib --tests` → Tests job passed at `e6cb89989215f435141aa68992a71dce44bc243c` (CI run `45261257668`, 2026-11-23).
| In Progress | None |
| Risk | None |
| Updated | 2026-11-23 |

### FR-DOC-008

| Aspect | Detail |
| -------------- | --------------------------------------------- |
| State | Done |
| Done | Audit DATA.md (AES608) (`IDataDocProtocol, `ID_MISSING_FEATURE_PREFIX`); source: `crates/doc-rules/src/capabilities_doc_data.rs`, `crates/doc-rules/src/agent_doc_orchestrator.rs`; Gate: `CARGO_INCREMENTAL=0 cargo nextest run --workspace --lib --tests` → Tests job passed at `e6cb89989215f435141aa68992a71dce44bc243c` (CI run `45261257668`, 2026-11-23).
| In Progress | None |
| Risk | None |
| Updated | 2026-11-23 |

### FR-DOC-009

| Aspect | Detail |
| -------------- | --------------------------------------------- |
| State | Done |
| Done | Audit FRD.md (AES609) (`IFrdDocProtocol, `ID_MISSING_FEATURE_PREFIX`); source: `crates/doc-rules/src/capabilities_doc_frd.rs`, `crates/doc-rules/src/agent_doc_orchestrator.rs`; Gate: `CARGO_INCREMENTAL=0 cargo nextest run --workspace --lib --tests` → Tests job passed at `e6cb89989215f435141aa68992a71dce44bc243c` (CI run `45261257668`, 2026-11-23).
| In Progress | None |
| Risk | None |
| Updated | 2026-11-23 |

### FR-DOC-010

| Aspect | Detail |
| -------------- | --------------------------------------------- |
| State | Done |
| Done | Audit DESIGN.md (AES610) (`IDesignDocProtocol, `IDocRunnerAggregate`); source: `crates/doc-rules/src/capabilities_doc_design.rs`, `crates/doc-rules/src/agent_doc_orchestrator.rs`; tests: `crates/doc-rules/tests/unit_doc_rules.rs`; Gate: `CARGO_INCREMENTAL=0 cargo nextest run --workspace --lib --tests` → Tests job passed at `e6cb89989215f435141aa68992a71dce44bc243c` (CI run `45261257668`, 2026-11-23).
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
