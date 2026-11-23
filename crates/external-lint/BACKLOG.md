# Feature Backlog: External Lint

FRD: [FRD.md](FRD.md)
Architecture: [ARCHITECTURE.md](../../ARCHITECTURE.md)
State: defined in root [ROADMAP.md](../../ROADMAP.md) — cited here, not restated
Last Updated: 2026-11-23

## Progress Overview

- Done: All 6 backlog rows reconciled to Done at HEAD; gate: `CARGO_INCREMENTAL=0 cargo nextest run --workspace --lib --tests` → Tests job passed at `e6cb89989215f435141aa68992a71dce44bc243c` (CI run `45261257668`, 2026-11-23). Per-row source and test evidence below.
- Done: `cargo test -p external-lint-lint-arwaky --lib --tests` → 10 passed at `cc63389a` (2026-09-29). Per-scenario evidence below.
- In Progress: None
- Blocked: None
- Next Action: Re-run `cargo test -p external-lint-lint-arwaky --lib --tests` after any code change to this crate

## Backlog Status

### FR-ExternalLint-001

| Aspect | Detail |
| -------------- | ------------------------------------------ |
| State | Done |
| Done | Select and Execute Adapters by Language (`IExternalLintSelectorProtocol`); source: `crates/external-lint/src/capabilities_rs_clippy_adapter.rs`, `crates/external-lint/src/agent_external_lint_orchestrator.rs`; tests: `crates/external-lint/tests/acceptance_adapters_selection.rs`, `crates/external-lint/tests/unit_external_lint_rs_clippy_adapter.rs`, `crates/external-lint/tests/e2e_external_lint_flow.rs`; Gate: `CARGO_INCREMENTAL=0 cargo nextest run --workspace --lib --tests` → Tests job passed at `e6cb89989215f435141aa68992a71dce44bc243c` (CI run `45261257668`, 2026-11-23).
| In Progress | None |
| Risk | None |
| Updated | 2026-11-23 |

### FR-ExternalLint-002

| Aspect | Detail |
| -------------- | ------------------------------------------ |
| State | Done |
| Done | Apply Auto-Fix via Adapters (`IJsToolResolutionProtocol`); source: `crates/external-lint/src/capabilities_js_tool_resolver.rs`, `crates/external-lint/src/capabilities_js_prettier_adapter.rs`; tests: `crates/external-lint/tests/e2e_external_lint_flow.rs`, `crates/external-lint/tests/contract_external_lint.rs`, `crates/external-lint/tests/smoke_external_lint.rs`; Gate: `CARGO_INCREMENTAL=0 cargo nextest run --workspace --lib --tests` → Tests job passed at `e6cb89989215f435141aa68992a71dce44bc243c` (CI run `45261257668`, 2026-11-23).
| In Progress | None |
| Risk | None |
| Updated | 2026-11-23 |

### FR-ExternalLint-003

| Aspect | Detail |
| -------------- | ------------------------------------------ |
| State | Done |
| Done | Normalize External Tool Output (`INormalizeProtocol`); source: `crates/external-lint/src/capabilities_output_normalizer.rs`, `crates/external-lint/src/capabilities_rs_clippy_adapter.rs`; tests: `crates/external-lint/tests/contract_external_lint.rs`, `crates/external-lint/tests/unit_external_lint_rs_clippy_adapter.rs`, `crates/external-lint/tests/unit_external_lint_py_ruff_adapter.rs`; Gate: `CARGO_INCREMENTAL=0 cargo nextest run --workspace --lib --tests` → Tests job passed at `e6cb89989215f435141aa68992a71dce44bc243c` (CI run `45261257668`, 2026-11-23).
| In Progress | None |
| Risk | None |
| Updated | 2026-11-23 |

### FR-ExternalLint-004

| Aspect | Detail |
| -------------- | ------------------------------------------ |
| State | Done |
| Done | Subprocess Execution (`ICommandExecutorProtocol`); source: `crates/external-lint/src/capabilities_command_executor.rs`, `crates/external-lint/src/root_external_lint_container.rs`; tests: `crates/external-lint/tests/unit_external_lint_py_ruff_adapter.rs`, `crates/external-lint/tests/unit_external_lint_py_bandit_adapter.rs`, `crates/external-lint/tests/contract_external_lint.rs`; Gate: `CARGO_INCREMENTAL=0 cargo nextest run --workspace --lib --tests` → Tests job passed at `e6cb89989215f435141aa68992a71dce44bc243c` (CI run `45261257668`, 2026-11-23).
| In Progress | None |
| Risk | None |
| Updated | 2026-11-23 |

### FR-ExternalLint-005

| Aspect | Detail |
| -------------- | ------------------------------------------ |
| State | Done |
| Done | JS Tool Path Resolution (`IJsToolResolutionProtocol`); source: `crates/external-lint/src/capabilities_js_tool_resolver.rs`, `crates/external-lint/src/capabilities_js_prettier_adapter.rs`; tests: `crates/external-lint/tests/contract_external_lint.rs`, `crates/external-lint/tests/unit_external_lint_utility_path_normalization.rs`, `crates/external-lint/tests/integration_external_lint.rs`; Gate: `CARGO_INCREMENTAL=0 cargo nextest run --workspace --lib --tests` → Tests job passed at `e6cb89989215f435141aa68992a71dce44bc243c` (CI run `45261257668`, 2026-11-23).
| In Progress | None |
| Risk | None |
| Updated | 2026-11-23 |

### FR-ExternalLint-006

| Aspect | Detail |
| -------------- | ------------------------------------------ |
| State | Done |
| Done | Rust Working Directory Resolution (`ICargoDirProtocol, `IExternalLintAggregate`); source: `crates/external-lint/src/root_external_lint_container.rs`, `crates/external-lint/src/capabilities_cargo_dir_resolver.rs`; tests: `crates/external-lint/tests/contract_external_lint.rs`, `crates/external-lint/tests/unit_external_lint_py_ruff_adapter.rs`, `crates/external-lint/tests/e2e_external_lint_flow.rs`; Gate: `CARGO_INCREMENTAL=0 cargo nextest run --workspace --lib --tests` → Tests job passed at `e6cb89989215f435141aa68992a71dce44bc243c` (CI run `45261257668`, 2026-11-23).
| In Progress | None |
| Risk | None |
| Updated | 2026-11-23 |

## Reference

| Artifact | Purpose | Link |
| -------------- | -------------------------------------------------------- | ------ |
| PRD | Business goals and scope for external lint adapters | [PRD.md](../../PRD.md) |
| FRD | Functional requirements for this feature | [FRD.md](FRD.md) |
| Roadmap | Timeline and dependencies | [ROADMAP.md](../../ROADMAP.md) |
| Architecture | System boundaries, tech stack, and infra | [ARCHITECTURE.md](../../ARCHITECTURE.md) |
