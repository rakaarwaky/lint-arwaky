# Feature Backlog: Project Setup

FRD: [FRD.md](FRD.md)
Architecture: [ARCHITECTURE.md](../../ARCHITECTURE.md)
State: defined in root [ROADMAP.md](../../ROADMAP.md) — cited here, not restated
Last Updated: 2026-11-23

## Progress Overview

- Done: All 8 backlog rows reconciled to Done at HEAD; gate: `CARGO_INCREMENTAL=0 cargo nextest run --workspace --lib --tests` → Tests job passed at `e6cb89989215f435141aa68992a71dce44bc243c` (CI run `45261257668`, 2026-11-23). Per-row source and test evidence below.
- Done: `cargo nextest run --workspace --lib --tests` → 2035 passed at `72d98f50` (2026-09-29). Per-scenario evidence below.
- In Progress: None
- Blocked: None
- Next Action: Re-run `cargo nextest run --workspace --lib --tests` after any code change to this crate

## Backlog Status

### FR-ProjectSetup-001

| Aspect | Detail |
| -------------- | --------------------------------------------- |
| State | Done |
| Done | Generate MCP Configuration (`IMcpConfigGenerationProtocol`); source: `crates/project-setup/src/capabilities_mcp_config_generator.rs`, `crates/project-setup/src/agent_setup_orchestrator.rs`; tests: `crates/config-system/tests/acceptance_FR_001.rs`, `crates/project-setup/tests/unit_project_setup_processor.rs`, `crates/project-setup/tests/e2e_project_setup_flow.rs`; Gate: `CARGO_INCREMENTAL=0 cargo nextest run --workspace --lib --tests` → Tests job passed at `e6cb89989215f435141aa68992a71dce44bc243c` (CI run `45261257668`, 2026-11-23).
| In Progress | None |
| Risk | None |
| Updated | 2026-11-23 |

### FR-ProjectSetup-002

| Aspect | Detail |
| -------------- | --------------------------------------------- |
| State | Done |
| Done | Generate Environment File (`IEnvGenerationProtocol`); source: `crates/project-setup/src/capabilities_env_generator.rs`, `crates/project-setup/src/agent_setup_orchestrator.rs`; tests: `crates/project-setup/tests/unit_project_setup_processor.rs`, `crates/config-system/tests/acceptance_FR_002.rs`, `crates/project-setup/tests/e2e_project_setup_flow.rs`; Gate: `CARGO_INCREMENTAL=0 cargo nextest run --workspace --lib --tests` → Tests job passed at `e6cb89989215f435141aa68992a71dce44bc243c` (CI run `45261257668`, 2026-11-23).
| In Progress | None |
| Risk | None |
| Updated | 2026-11-23 |

### FR-ProjectSetup-003

| Aspect | Detail |
| -------------- | --------------------------------------------- |
| State | Done |
| Done | Detect Languages (`ILanguageDetectionProtocol`); source: `crates/project-setup/src/capabilities_language_detector.rs`, `crates/project-setup/src/agent_setup_orchestrator.rs`; tests: `crates/project-setup/tests/unit_project_setup_processor.rs`, `crates/config-system/tests/acceptance_FR_003.rs`, `crates/project-setup/tests/e2e_project_setup_flow.rs`; Gate: `CARGO_INCREMENTAL=0 cargo nextest run --workspace --lib --tests` → Tests job passed at `e6cb89989215f435141aa68992a71dce44bc243c` (CI run `45261257668`, 2026-11-23).
| In Progress | None |
| Risk | None |
| Updated | 2026-11-23 |

### FR-ProjectSetup-004

| Aspect | Detail |
| -------------- | --------------------------------------------- |
| State | Done |
| Done | Install Adapters (`IAdapterInstallationProtocol, `InstallPythonAdapters`); source: `crates/project-setup/src/capabilities_setup_installer_adapter.rs`, `crates/project-setup/src/root_project_setup_container.rs`; tests: `crates/project-setup/tests/unit_project_setup_installer_adapter.rs`, `crates/project-setup/tests/contract_project_setup.rs`, `crates/project-setup/tests/integration_project_setup.rs`; Gate: `CARGO_INCREMENTAL=0 cargo nextest run --workspace --lib --tests` → Tests job passed at `e6cb89989215f435141aa68992a71dce44bc243c` (CI run `45261257668`, 2026-11-23).
| In Progress | None |
| Risk | None |
| Updated | 2026-11-23 |

### FR-ProjectSetup-005

| Aspect | Detail |
| -------------- | --------------------------------------------- |
| State | Done |
| Done | Load Config Templates (`IConfigTemplateProtocol`); source: `crates/project-setup/src/agent_setup_orchestrator.rs`; tests: `crates/config-system/tests/acceptance_FR_001.rs`, `crates/config-system/tests/acceptance_FR_003.rs`, `crates/project-setup/tests/unit_project_setup_processor.rs`; Gate: `CARGO_INCREMENTAL=0 cargo nextest run --workspace --lib --tests` → Tests job passed at `e6cb89989215f435141aa68992a71dce44bc243c` (CI run `45261257668`, 2026-11-23).
| In Progress | None |
| Risk | None |
| Updated | 2026-11-23 |

### FR-ProjectSetup-006

| Aspect | Detail |
| -------------- | --------------------------------------------- |
| State | Done |
| Done | Write Config Files (`IConfigWritingProtocol`); source: `crates/project-setup/src/agent_setup_orchestrator.rs`; tests: `crates/config-system/tests/acceptance_FR_001.rs`, `crates/config-system/tests/acceptance_FR_004.rs`, `crates/project-setup/tests/e2e_project_setup_flow.rs`; Gate: `CARGO_INCREMENTAL=0 cargo nextest run --workspace --lib --tests` → Tests job passed at `e6cb89989215f435141aa68992a71dce44bc243c` (CI run `45261257668`, 2026-11-23).
| In Progress | None |
| Risk | None |
| Updated | 2026-11-23 |

### FR-ProjectSetup-007

| Aspect | Detail |
| -------------- | --------------------------------------------- |
| State | Done |
| Done | Run Pre-flight Check (`IPreFlightProtocol`); source: `crates/project-setup/src/agent_setup_orchestrator.rs`; tests: `crates/config-system/tests/acceptance_FR_005.rs`; Gate: `CARGO_INCREMENTAL=0 cargo nextest run --workspace --lib --tests` → Tests job passed at `e6cb89989215f435141aa68992a71dce44bc243c` (CI run `45261257668`, 2026-11-23).
| In Progress | None |
| Risk | None |
| Updated | 2026-11-23 |

### FR-ProjectSetup-008

| Aspect | Detail |
| -------------- | --------------------------------------------- |
| State | Done |
| Done | Probe File Existence (`IFilePathExistenceProtocol, `ISetupAggregate`); source: `crates/project-setup/src/agent_setup_orchestrator.rs`, `crates/project-setup/src/root_project_setup_container.rs`; tests: `crates/config-system/tests/acceptance_FR_001.rs`, `crates/project-setup/tests/e2e_project_setup_flow.rs`, `crates/config-system/tests/acceptance_FR_005.rs`; Gate: `CARGO_INCREMENTAL=0 cargo nextest run --workspace --lib --tests` → Tests job passed at `e6cb89989215f435141aa68992a71dce44bc243c` (CI run `45261257668`, 2026-11-23).
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
