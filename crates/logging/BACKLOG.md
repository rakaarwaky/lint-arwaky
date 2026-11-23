# BACKLOG — logging

---

## Progress Overview

The CLI and MCP entry points each carry their own `init_tracing()`
body, duplicating the `"warn,lint_arwaky::audit=info"` filter string.
No flag exists to raise the log level at runtime without exporting an
env var, so debugging a slow `la scan .` requires a rebuild or a
hard-coded filter change.

## Backlog Status

### FR-Logging-001

| Aspect | Detail |
| -------------- | --------------------------------------------- |
| State | Done |
| Done | Build the Filter Directives (`IFilterBuildProtocol`); source: `crates/logging/src/capabilities_filter_build.rs`; tests: `crates/logging/tests/contract_logging.rs`, `crates/logging/tests/acceptance_logging.rs`, `crates/logging/tests/unit_logging.rs`; Gate: `CARGO_INCREMENTAL=0 cargo nextest run --workspace --lib --tests` → Tests job passed at `e6cb89989215f435141aa68992a71dce44bc243c` (CI run `45261257668`, 2026-11-23).
| In Progress | None |
| Risk | None |
| Updated | 2026-11-23 |

### FR-Logging-002

| Aspect | Detail |
| -------------- | --------------------------------------------- |
| State | Done |
| Done | Install the Global Subscriber (`ISubscriberInstallProtocol`); source: `crates/logging/src/capabilities_subscriber_init.rs`, `crates/logging/src/agent_logging_orchestrator.rs`; tests: `crates/logging/tests/contract_logging.rs`, `crates/logging/tests/acceptance_logging.rs`, `crates/logging/tests/integration_logging.rs`; Gate: `CARGO_INCREMENTAL=0 cargo nextest run --workspace --lib --tests` → Tests job passed at `e6cb89989215f435141aa68992a71dce44bc243c` (CI run `45261257668`, 2026-11-23).
| In Progress | None |
| Risk | None |
| Updated | 2026-11-23 |

### FR-Logging-003

| Aspect | Detail |
| -------------- | --------------------------------------------- |
| State | Done |
| Done | Time a Scan Phase (`IPhaseTimerProtocol`); source: `crates/logging/src/capabilities_phase_timer.rs`, `crates/logging/src/agent_logging_orchestrator.rs`; tests: `crates/logging/tests/unit_logging.rs`, `crates/logging/tests/contract_logging.rs`, `crates/logging/tests/acceptance_logging.rs`; Gate: `CARGO_INCREMENTAL=0 cargo nextest run --workspace --lib --tests` → Tests job passed at `e6cb89989215f435141aa68992a71dce44bc243c` (CI run `45261257668`, 2026-11-23).
| In Progress | None |
| Risk | None |
| Updated | 2026-11-23 |

### FR-Logging-004

| Aspect | Detail |
| -------------- | --------------------------------------------- |
| State | Done |
| Done | Report Walker Progress (`IWalkerReportProtocol, `ILoggingAggregate`); source: `crates/logging/src/agent_logging_orchestrator.rs`, `crates/logging/src/root_logging_container.rs`; tests: `crates/logging/tests/integration_logging.rs`; Gate: `CARGO_INCREMENTAL=0 cargo nextest run --workspace --lib --tests` → Tests job passed at `e6cb89989215f435141aa68992a71dce44bc243c` (CI run `45261257668`, 2026-11-23).
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
