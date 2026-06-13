# Original User Request

## Initial Request — 2026-06-13T13:00:25+07:00

Execute the production readiness fixes in the `lint-arwaky` workspace as detailed in [2026-06-13-production-readiness-fixes.md](file:///home/raka/mcp-arwaky/lint-arwaky/docs/compose/plans/2026-06-13-production-readiness-fixes.md). The main goal is to fix critical runtime issues, todo/panic stubs, duplicates, version mismatches, and stub implementations.

Working directory: `/home/raka/mcp-arwaky/lint-arwaky`
Integrity mode: development

## Requirements

### R1. Cycle Detection Real Implementation
Replace `todo!()` in `crates/shared/src/code-analysis/contract_cycle_protocol.rs` with a working cycle detection algorithm that detects circular dependencies between files and adds violations to `results`.

### R2. Replace default `todo!()`
Remove `todo!()` from `SourceParserOrchestrator::default()` in `crates/source-parsing/src/infrastructure_parser_adapter.rs` and panic with a clear message indicating DI is required.

### R3. Remove `panic!()` in PlaceholderAnalyzer
Implement a null-object pattern in `crates/code-analysis/src/root_code_analysis_container.rs` to replace panic stubs for `fs()` and `parser()` methods.

### R4. Mutex poisoned handle
Fix mutex `.unwrap()` in `crates/root_mcp_main_entry.rs` to handle poisoned mutexes gracefully.

### R5. Remove Duplicate Metrics Provider
Delete `crates/metrics-service/src/infrastructure_py_metrics_adapter.rs`, remove the module from `lib.rs`, and update container references to use `RustMetricsProvider`.

### R6. Workspace version synchronization
Sync versions to `1.10.13` in all 22 `Cargo.toml` files under `crates/`.

### R7. Real Plugin Commands Orchestrator
Replace print stubs in `crates/plugin-system/src/agent_commands_orchestrator.rs` with actual discovery of language adapters and plugins.

### R8. Real Report Commands Orchestrator
Replace print stubs in `crates/output-report/src/agent_commands_orchestrator.rs` with actual serialization/output formatting for json, sarif, and junit formats.

### R9. MCP server println! removal
Clean up print/println stubs in MCP server files (`surface_tools_controller.rs` and `surface_server_controller.rs`).

### R10. Setup Command Home Fallback
Avoid hardcoded `/home/user` path in setup command (`crates/cli-commands/src/surface_setup_command.rs`), using a platform-specific fallback instead.

### R11. Clean test data in Rust Scanner
Remove hardcoded `MyTrait`/`MyStruct` test data from `crates/source-parsing/src/infrastructure_rust_scanner.rs`.

### R12. Orphan surfaces analyzer TODO
Update `check_surfaces_orphan` in `crates/orphan-detector/src/capabilities_orphan_surfaces_analyzer.rs` to accept the actual files list and update all callers.

## Acceptance Criteria

### Compilation
- [ ] `cargo check --workspace` passes with no compilation errors.

### Testing
- [ ] `cargo test --workspace` runs and passes all tests.

### Code Quality and Standards
- [ ] `cargo clippy --all-targets -- -D warnings` runs cleanly.
- [ ] `cargo run --bin lint-arwaky-cli -- check .` runs successfully and reports 0 CRITICAL violations.
- [ ] Crate versions across the workspace are synchronized (all version fields in `crates/*/Cargo.toml` must be set to `1.10.13`).
