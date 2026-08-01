# Plan: Fix Cross-Feature Contract Violations

## Problem
1. **tui has dead `IReportFormatterProtocol`** — defined in `tui/` but never implemented. Real one is in `report-formatter/`.
2. **tui uses internal protocols instead of aggregates** — should use aggregates when consuming other features.

---

## Violation 1: Dead IReportFormatterProtocol in tui/

**File**: `crates/shared/src/tui/contract_report_formatter_protocol.rs`

This trait defines:
```rust
pub trait IReportFormatterProtocol: Send + Sync {
    fn format_results(&self, results: &LintResultList) -> DisplayContent;
    fn format_doctor_report(&self, diagnostics: &ToolchainDiagnostics) -> LintExecutionResult;
    fn format_dependency_report(&self, path: &str, report: &DependencyReport) -> LintExecutionResult;
    fn format_config_result(&self, result: &ConfigResult) -> LintExecutionResult;
}
```

**Status**: NEVER IMPLEMENTED. The real `IReportFormatterProtocol` is in `report-formatter/` with different methods (`format`, `supported_format`).

tui's `utility_report_formatter.rs` uses standalone functions, NOT this trait.

### Fix
1. Delete `crates/shared/src/tui/contract_report_formatter_protocol.rs`
2. Remove `pub mod contract_report_formatter_protocol;` from `crates/shared/src/tui/mod.rs`
3. Remove `pub use contract_report_formatter_protocol::IReportFormatterProtocol;` from `crates/shared/src/tui/mod.rs`

---

## Violation 2: tui uses IHookManagerProtocol instead of GitHooksAggregate

**File**: `crates/tui/src/capabilities_lint_executor.rs`

Current:
```rust
use shared::git_hooks::IHookManagerProtocol;
// ...
hook_port: Option<Arc<dyn IHookManagerProtocol>>,
```

**Should be**:
```rust
use shared::git_hooks::GitHooksAggregate;
// ...
hook_port: Option<Arc<dyn GitHooksAggregate>>,
```

### Fix
1. In `capabilities_lint_executor.rs`: replace `IHookManagerProtocol` with `GitHooksAggregate`
2. In `root_tui_container.rs`: pass `GitHooksAggregate` instead of `IHookManagerProtocol`

---

## Violation 3: tui uses IWatchProviderProtocol/IChangeAnalyzerProtocol instead of IWatchAggregate

**File**: `crates/tui/src/capabilities_lint_executor.rs`

Current:
```rust
use shared::file_watch::{IChangeAnalyzerProtocol, IWatchProviderProtocol};
// ...
watch_provider: Option<Arc<dyn IWatchProviderProtocol>>,
```

**Should be**:
```rust
use shared::file_watch::IWatchAggregate;
// ...
watch_provider: Option<Arc<dyn IWatchAggregate>>,
```

### Fix
1. In `capabilities_lint_executor.rs`: replace with `IWatchAggregate`
2. In `root_tui_container.rs`: pass `IWatchAggregate` instead of individual protocols

---

## Files Modified
1. `crates/shared/src/tui/contract_report_formatter_protocol.rs` — DELETE
2. `crates/shared/src/tui/mod.rs` — remove dead exports
3. `crates/tui/src/capabilities_lint_executor.rs` — use aggregates
4. `crates/tui/src/root_tui_container.rs` — pass aggregates

## Verification
1. `CARGO_INCREMENTAL=0 cargo check -p tui-lint-arwaky`
2. `CARGO_INCREMENTAL=0 cargo check --workspace`
3. `grep -r 'IHookManagerProtocol' crates/tui/src/` — zero (use aggregate instead)
4. `grep -r 'IWatchProviderProtocol' crates/tui/src/` — zero (use aggregate instead)
5. `grep -r 'contract_report_formatter_protocol' crates/shared/src/tui/` — zero
