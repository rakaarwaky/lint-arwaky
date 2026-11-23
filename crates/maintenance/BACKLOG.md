# Feature Backlog: Maintenance

FRD: [FRD.md](FRD.md)
Architecture: [ARCHITECTURE.md](../../ARCHITECTURE.md)
State: defined in root [ROADMAP.md](../../ROADMAP.md) — cited here, not restated
Last Updated: 2026-11-23

## Progress Overview

- Done: All 8 backlog rows reconciled to Done at HEAD; gate: `CARGO_INCREMENTAL=0 cargo nextest run --workspace --lib --tests` → Tests job passed at `e6cb89989215f435141aa68992a71dce44bc243c` (CI run `45261257668`, 2026-11-23). Per-row source and test evidence below.
- Done: `cargo test -p maintenance-lint-arwaky --lib --tests` → 54 passed, 0 failures at `c23e9c35` (2026-09-30). Per-scenario evidence below.
- In Progress: None
- Blocked: None
- Next Action: Re-run `cargo test -p maintenance-lint-arwaky --lib --tests` after any code change to this crate

## Backlog Status

### FR-Maintenance-001

| Aspect | Detail |
| -------------- | --------------------------------------------- |
| State | Done |
| Done | Environment Health Check (`IDoctorProtocol`); source: `crates/maintenance/src/agent_maintenance_orchestrator.rs`, `crates/maintenance/src/capabilities_doctor_checker.rs`; tests: `crates/maintenance/tests/acceptance_maintenance.rs`, `crates/maintenance/tests/e2e_maintenance_flow.rs`, `crates/maintenance/tests/unit_maintenance_checker.rs`; Gate: `CARGO_INCREMENTAL=0 cargo nextest run --workspace --lib --tests` → Tests job passed at `e6cb89989215f435141aa68992a71dce44bc243c` (CI run `45261257668`, 2026-11-23).
| In Progress | None |
| Risk | None |
| Updated | 2026-11-23 |

### FR-Maintenance-002

| Aspect | Detail |
| -------------- | --------------------------------------------- |
| State | Done |
| Done | Project Statistics (`IProjectStatsProtocol`); source: `crates/maintenance/src/agent_maintenance_orchestrator.rs`, `crates/maintenance/src/root_maintenance_container.rs`; tests: `crates/maintenance/tests/unit_maintenance_checker.rs`, `crates/maintenance/tests/contract_maintenance.rs`, `crates/maintenance/tests/acceptance_maintenance.rs`; Gate: `CARGO_INCREMENTAL=0 cargo nextest run --workspace --lib --tests` → Tests job passed at `e6cb89989215f435141aa68992a71dce44bc243c` (CI run `45261257668`, 2026-11-23).
| In Progress | None |
| Risk | None |
| Updated | 2026-11-23 |

### FR-Maintenance-003

| Aspect | Detail |
| -------------- | --------------------------------------------- |
| State | Done |
| Done | Cache Cleanup (`ICacheCleanupProtocol`); source: `crates/maintenance/src/agent_maintenance_orchestrator.rs`, `crates/maintenance/src/root_maintenance_container.rs`; tests: `crates/maintenance/tests/contract_maintenance.rs`, `crates/maintenance/tests/integration_maintenance.rs`, `crates/maintenance/tests/unit_maintenance_checker.rs`; Gate: `CARGO_INCREMENTAL=0 cargo nextest run --workspace --lib --tests` → Tests job passed at `e6cb89989215f435141aa68992a71dce44bc243c` (CI run `45261257668`, 2026-11-23).
| In Progress | None |
| Risk | None |
| Updated | 2026-11-23 |

### FR-Maintenance-004

| Aspect | Detail |
| -------------- | --------------------------------------------- |
| State | Done |
| Done | Tool Update (`IToolUpdateProtocol`); source: `crates/maintenance/src/capabilities_self_update_checker.rs`, `crates/maintenance/src/agent_maintenance_orchestrator.rs`; tests: `crates/maintenance/tests/unit_maintenance_checker.rs`, `crates/maintenance/tests/contract_maintenance.rs`, `crates/maintenance/tests/acceptance_maintenance.rs`; Gate: `CARGO_INCREMENTAL=0 cargo nextest run --workspace --lib --tests` → Tests job passed at `e6cb89989215f435141aa68992a71dce44bc243c` (CI run `45261257668`, 2026-11-23).
| In Progress | None |
| Risk | None |
| Updated | 2026-11-23 |

### FR-Maintenance-005

| Aspect | Detail |
| -------------- | --------------------------------------------- |
| State | Done |
| Done | Security Scan (`ISecurityScanProtocol`); source: `crates/maintenance/src/capabilities_security_scan_checker.rs`, `crates/maintenance/src/agent_maintenance_orchestrator.rs`; tests: `crates/maintenance/tests/unit_maintenance_checker.rs`, `crates/maintenance/tests/contract_maintenance.rs`, `crates/maintenance/tests/e2e_maintenance_flow.rs`; Gate: `CARGO_INCREMENTAL=0 cargo nextest run --workspace --lib --tests` → Tests job passed at `e6cb89989215f435141aa68992a71dce44bc243c` (CI run `45261257668`, 2026-11-23).
| In Progress | None |
| Risk | None |
| Updated | 2026-11-23 |

### FR-Maintenance-006

| Aspect | Detail |
| -------------- | --------------------------------------------- |
| State | Done |
| Done | Dependency Report (`IDependencyReportProtocol`); source: `crates/maintenance/src/capabilities_dependency_report_checker.rs`, `crates/maintenance/src/agent_maintenance_orchestrator.rs`; tests: `crates/maintenance/tests/unit_maintenance_checker.rs`, `crates/maintenance/tests/contract_maintenance.rs`, `crates/maintenance/tests/e2e_maintenance_flow.rs`; Gate: `CARGO_INCREMENTAL=0 cargo nextest run --workspace --lib --tests` → Tests job passed at `e6cb89989215f435141aa68992a71dce44bc243c` (CI run `45261257668`, 2026-11-23).
| In Progress | None |
| Risk | None |
| Updated | 2026-11-23 |

### FR-Maintenance-007

| Aspect | Detail |
| -------------- | --------------------------------------------- |
| State | Done |
| Done | Adapter Health Check (`IAdapterHealthProtocol`); source: `crates/maintenance/src/capabilities_adapter_health_checker.rs`, `crates/maintenance/src/agent_maintenance_orchestrator.rs`; tests: `crates/maintenance/tests/unit_maintenance_checker.rs`, `crates/maintenance/tests/contract_maintenance.rs`, `crates/maintenance/tests/acceptance_maintenance.rs`; Gate: `CARGO_INCREMENTAL=0 cargo nextest run --workspace --lib --tests` → Tests job passed at `e6cb89989215f435141aa68992a71dce44bc243c` (CI run `45261257668`, 2026-11-23).
| In Progress | None |
| Risk | None |
| Updated | 2026-11-23 |

### FR-Maintenance-008

| Aspect | Detail |
| -------------- | --------------------------------------------- |
| State | Done |
| Done | Self-Update (`ISelfUpdateProtocol, `IMaintenanceAggregate`); source: `crates/maintenance/src/agent_maintenance_orchestrator.rs`, `crates/maintenance/src/root_maintenance_container.rs`; tests: `crates/maintenance/tests/unit_maintenance_checker.rs`, `crates/maintenance/tests/contract_maintenance.rs`, `crates/maintenance/tests/integration_maintenance.rs`; Gate: `CARGO_INCREMENTAL=0 cargo nextest run --workspace --lib --tests` → Tests job passed at `e6cb89989215f435141aa68992a71dce44bc243c` (CI run `45261257668`, 2026-11-23).
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
