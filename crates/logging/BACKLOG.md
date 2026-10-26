# BACKLOG — logging

---

## Current Condition

The CLI and MCP entry points each carry their own `init_tracing()`
body, duplicating the `"warn,lint_arwaky::audit=info"` filter string.
No flag exists to raise the log level at runtime without exporting an
env var, so debugging a slow `la scan .` requires a rebuild or a
hard-coded filter change.

## Backlog

| ID | Priority | State | Dependencies | Next Action | Updated |
|---|---:|---|---|---|---|
| FR-Logging-001 | P0 | Done | None | — | 2026-10-07 |
| FR-Logging-002 | P0 | Done | None | — | 2026-10-07 |
| FR-Logging-003 | P0 | Done | None | — | 2026-10-07 |
| FR-Logging-004 | P0 | Done | None | — | 2026-10-07 |

## Scenario Evidence

| Scenario | Kind | Test file | Test name | Last verified |
|---|---|---|---|---|
| `LINT_ARWAKY_LOG=trace lint-arwaky-cli scan .` emits per-directory `walker_enter`/`walker_skip` events | Manual | `tests/integration_logging_scan_events.rs` | `trace_emits_per_directory_events` | 2026-10-07 |
| `la -v scan .` emits `scan_start` through `scan_complete` with `elapsed_ms` | Manual | `tests/integration_logging_verbose.rs` | `verbose_flag_emits_scan_events` | 2026-10-07 |

## Blockers

None.

## Dependencies

- `shared-common` provides `LogVerbosity` (taxonomy layer).
- `tracing` and `tracing-subscriber` are already workspace deps.

## Release Readiness

| Area | Status | Notes |
|---|---|---|
| Tests | Done | `cargo nextest run --workspace --lib --tests -j 2` → all pass at `dc477f2d` |
| Format | Done | `cargo fmt --all -- --check` clean at `dc477f2d` |
| Clippy | Done | `cargo clippy --all-targets -- -D warnings` clean at `dc477f2d` |
| Self-lint | In Progress | `lint-arwaky-cli check .` — Total: 0 violations pending AES606 merge |

## Deferred

- `HOW-TO-USE-LINT-COMMANDS.md` doc update for `-v`/`-vv` — separate docs PR.
- Migrate `root_cli_main_entry.rs` and `root_mcp_main_entry.rs` off the local `init_tracing()` to `LoggingContainer::init()` — CLI done; MCP entry still calls its local init.

## Change Log

| Date | Change | By |
|---|---|---|
| 2026-10-26 | Aligned BACKLOG ID column to FRD FR-IDs (AES601) | @raka |
| 2026-10-07 | Feature crate created. `LogVerbosity` added to `shared-common` (taxonomy layer). `SubscriberInit` capability, `LoggingOrchestrator` agent, `LoggingContainer` root. Scan-phase tracing events instrumented in `surface_check_action.rs` and `agent_external_lint_orchestrator.rs`. CLI `-v`/`-vv` flag added | @raka |

---
