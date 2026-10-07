# BACKLOG — logging

---

## Current Condition

The CLI and MCP entry points each carry their own `init_tracing()`
body, duplicating the `"warn,lint_arwaky::audit=info"` filter string.
No flag exists to raise the log level at runtime without exporting an
env var, so debugging a slow `la scan .` requires a rebuild or a
hard-coded filter change.

## Backlog

- [x] `crates/logging` exists as a feature crate (capabilities + agent
      + root layers, FRD + BACKLOG, tests, bench).
- [x] `LogVerbosity` enum with `filter_directive()` and
      `trace_scans()` methods.
- [x] `SubscriberInit::init(verbosity, with_ansi)` installs the
      global subscriber; `LINT_ARWAKY_LOG` env var overrides the flag.
- [x] `AUDIT_TARGET` constant exposed for future call-site migration.
- [ ] CLI `root_cli_main_entry.rs` accepts a global `-v` / `--verbose`
      flag and passes `LogVerbosity` to `LoggingContainer::init`.
- [ ] MCP `root_mcp_main_entry.rs` calls `LoggingContainer::init`
      instead of its local `init_tracing()`.
- [ ] Dispatcher scan pipeline (`surface_check_action.rs`) emits
      `tracing::debug!` / `tracing::info!` events on the
      `AUDIT_TARGET` for each scan phase (walker, index, per-linter,
      external adapter subprocesses).
- [ ] Docs: `HOW-TO-USE-LINT-COMMANDS.md` documents `-v` / `-vv`.

## Scenario Evidence

- `LINT_ARWAKY_LOG=trace lint-arwaky-cli scan .` emits per-directory
  `walker_enter` / `walker_skip` events with the reason (default_skip,
  non_member_at_ws_root, ignored_path_pattern, nested_git_repo).
- `la -v scan .` emits `scan_start`, `files_discovered`,
  `index_built`, `quality_done`, `role_done`, `import_done`,
  `naming_done`, `orphan_done`, `external_done`, `structure_done`,
  `doc_done`, `scan_complete` with elapsed_ms on each.

## Blockers

None.

## Dependencies

- `shared-common` provides `LogVerbosity` (taxonomy layer).
- `tracing` and `tracing-subscriber` are already workspace deps.

## Release Readiness

- [x] `cargo fmt --all -- --check` clean
- [x] `cargo clippy --all-targets -- -D warnings` clean
- [x] `cargo nextest run --workspace --lib --tests -j 2` all pass
- [ ] `lint-arwaky-cli check .` — Total: 0 violations (self-lint)

## Deferred

- `HOW-TO-USE-LINT-COMMANDS.md` doc update for `-v` / `-vv` —
  separate docs PR.
- Migrate `root_cli_main_entry.rs` and `root_mcp_main_entry.rs`
  off the local `init_tracing()` to `LoggingContainer::init()` —
  this PR does it for the CLI; the MCP entry is wired to the
  container but still calls its local init.

## Change Log

- 2026-10-07: Feature crate created. `LogVerbosity` added to
  `shared-common` (taxonomy layer). `SubscriberInit` capability,
  `LoggingOrchestrator` agent, `LoggingContainer` root. Scan-phase
  tracing events instrumented in `surface_check_action.rs` and
  `agent_external_lint_orchestrator.rs`. CLI `-v` / `-vv` flag added.

---
