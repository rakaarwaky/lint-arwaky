# Plan: cli-commands — Tech-Lead

## Summary

The `cli-commands` crate is a Smart Surface with 11 source files implementing 14 FRDs. The architecture is sound — all handlers delegate to `dispatcher::surface_*_action` functions and only format output for terminal display. The crate follows AES406 surface conventions and uses `Arc<dyn Aggregate>` for DI. However, 10 findings were identified across 6 dimensions: duplicated formatting logic across surface files, inconsistent parameter patterns that reduce maintainability, and unused Cargo dependencies.

## Findings

### Security

| # | Severity | Issue | Location | Recommendation |
|---|----------|-------|----------|----------------|
| — | — | No security issues found. Surface layer only formats output; no user input processing, file writes, or network calls. | — | — |

### Performance

| # | Severity | Issue | Location | Recommendation |
|---|----------|-------|----------|----------------|
| 1 | 🟢 INFO | `make_relative()` calls `canonicalize()` on every file path per violation in `render_text` | `utility_output_text_formatter.rs:287-289` | Cache canonicalized paths or pass pre-computed relative paths from dispatcher. Low impact unless scan returns thousands of violations. |

### Error Handling

| # | Severity | Issue | Location | Recommendation |
|---|----------|-------|----------|----------------|
| 2 | 🟢 INFO | `render_json` uses `.unwrap_or_default()` on `serde_json::to_string_pretty()` — serialisation failure silently returns empty string | `utility_output_text_formatter.rs:232` | Use `expect("json serialisation should not fail")` or propagate error. JSON serialisation of `ViolationItem` is infallible in practice. |

### SOLID

| # | Severity | Issue | Location | Recommendation |
|---|----------|-------|----------|----------------|
| 3 | 🟡 WARNING | `handle_check` takes `ICodeAnalysisAggregate` + `IFilesystemAggregate` + `Option<IConfigOrchestratorAggregate>` but ignores config orchestrator — inconsistent with other handlers that use param structs | `surface_scan_command.rs:106-118` | Refactor `handle_check` to take `ScanCommandParams` (like `handle_scan`) or a dedicated `CheckCommandParams`. The unused `_config_orchestrator` parameter is dead code. |
| 4 | 🟡 WARNING | `handle_fix` takes 4 individual parameters instead of a param struct, breaking the pattern used by scan/import/naming/role/orphan/external/ci | `surface_fix_command.rs:12-17` | Extract `FixCommandParams` struct for consistency with other handlers. |
| 5 | 🟡 WARNING | `handle_git_diff` takes 4 individual parameters instead of a param struct | `surface_git_command.rs:10-14` | Extract `GitDiffCommandParams` struct for consistency. |

### Code Quality

| # | Severity | Issue | Location | Recommendation |
|---|----------|-------|----------|----------------|
| 6 | 🟡 WARNING | **Duplicated location formatting** — identical `(line, column) → "file:line:col"` pattern appears in 3 places: `surface_fix_command.rs:31-38`, `surface_git_command.rs:30-36`, `utility_output_text_formatter.rs:147-153` | Multiple files | Extract `format_location(file, line, column) -> String` into `utility_output_text_formatter.rs` and call from all 3 sites. |
| 7 | 🟡 WARNING | **Duplicated Severity match** — `surface_git_command.rs:37-42` manually matches `Severity` to string, but `ViolationItem.severity` likely already has a Display impl (used in `render_sarif` via `format!("{}", v.severity)`) | `surface_git_command.rs:37-42` | Use `format!("{}", r.severity)` or call a shared `severity_label()` helper. |

### Maintainability

| # | Severity | Issue | Location | Recommendation |
|---|----------|-------|----------|----------------|
| 8 | 🟢 INFO | `utility_output_text_formatter.rs` is 310 lines — largest file in the crate. `render_text` alone is ~70 lines with 3 nested branches | `utility_output_text_formatter.rs` | Extract `render_text_member_compact` / `render_text_member_detailed` helpers to reduce nesting. Consider splitting renderers to a second file if the file continues to grow. |
| 9 | 🟢 INFO | `regex` crate imported in `Cargo.toml` but not used in any source file | `Cargo.toml:11` | Remove `regex` dependency — confirmed zero usage in src/ and tests/. |
| 10 | 🟢 INFO | `rayon` crate imported in `Cargo.toml` but not used in any source file | `Cargo.toml:10` | Remove `rayon` dependency — confirmed zero usage in src/ and tests/. |

## Action Items

| Priority | Item | Finding |
|----------|------|---------|
| 🔴 P0 | Extract `format_location()` helper into `utility_output_text_formatter.rs` — eliminates duplication across 3 files | #6 |
| 🔴 P0 | Use Display impl or shared helper for Severity-to-string in `surface_git_command.rs` | #7 |
| 🟡 P1 | Refactor `handle_check` to remove unused `_config_orchestrator` param | #3 |
| 🟡 P1 | Create `FixCommandParams` and `GitDiffCommandParams` structs for consistency | #4, #5 |
| 🟢 P2 | Remove unused `regex` and `rayon` dependencies from Cargo.toml | #9, #10 |
| 🟢 P2 | Consider caching canonicalized paths in `render_text` for large scans | #1 |

## Fixed Code

### Finding #6 + #7: Shared location/severity helpers

Add to `utility_output_text_formatter.rs`:

```rust
/// Format a violation location as "file:line:column".
pub fn format_location(file: &str, line: usize, column: usize) -> String {
    match (line, column) {
        (l, c) if l > 0 && c > 0 => format!("{}:{}:{}", file, l, c),
        (l, _) if l > 0 => format!("{}:{}", file, l),
        _ => file.to_string(),
    }
}
```

Then in `surface_fix_command.rs`, replace:

```rust
// BEFORE (lines 31-38):
let loc = match (r.line.value(), r.column.value()) {
    (l, c) if l > 0 && c > 0 => {
        format!("{}:{}:{}", r.file.value(), l, c)
    }
    (l, _) if l > 0 => format!("{}:{}", r.file.value(), l),
    _ => r.file.value().to_string(),
};

// AFTER:
use crate::utility_output_text_formatter::format_location;
let loc = format_location(r.file.value(), r.line.value(), r.column.value());
```

Same pattern for `surface_git_command.rs` lines 30-36.

### Finding #3: Remove unused param from handle_check

```rust
// BEFORE:
pub fn handle_check(
    path: Option<FilePath>,
    format: Format,
    code_analysis_linter: Arc<dyn ICodeAnalysisAggregate>,
    filesystem: Arc<dyn IFilesystemAggregate>,
    _config_orchestrator: Option<Arc<dyn IConfigOrchestratorAggregate>>,
    filter: Option<String>,
) -> ExitCode {

// AFTER:
pub fn handle_check(
    path: Option<FilePath>,
    format: Format,
    code_analysis_linter: Arc<dyn ICodeAnalysisAggregate>,
    filesystem: Arc<dyn IFilesystemAggregate>,
    filter: Option<String>,
) -> ExitCode {
```

### Findings #4 + #5: Consistent param structs

```rust
// surface_fix_command.rs — add:
pub struct FixCommandParams {
    pub path: Option<FilePath>,
    pub dry_run: bool,
    pub code_analysis_linter: Arc<dyn ICodeAnalysisAggregate>,
    pub fix_orchestrator_factory: Arc<
        dyn Fn(bool) -> Arc<dyn LintFixOrchestratorAggregate> + Send + Sync,
    >,
}

// surface_git_command.rs — add:
pub struct GitDiffCommandParams {
    pub code_analysis_linter: Arc<dyn ICodeAnalysisAggregate>,
    pub base: GitBranchName,
    pub project_path: Option<String>,
    pub filter: Option<String>,
}
```

### Finding #8: Extract render_text helpers

```rust
// In utility_output_text_formatter.rs — extract from render_text:

fn render_member_violations(
    results: &[&ViolationItem],
    member_name: &str,
    norm_target: &str,
) {
    let mut file_violations: BTreeMap<String, Vec<&&ViolationItem>> = BTreeMap::new();
    for r in results {
        let rel_path = make_relative(&r.file.value, norm_target);
        file_violations.entry(rel_path).or_default().push(r);
    }
    for (file_path, file_results) in &file_violations {
        println!("  {file_path}");
        for r in file_results {
            let loc = format_location(&r.file.value, r.line.value(), r.column.value());
            println!("    {} [{}] {}", loc, r.code.code(), r.message.value);
        }
    }
}
```

### Finding #9 + #10: Remove unused dependencies

```toml
# Cargo.toml — remove these two lines:
rayon = { workspace = true }
regex = { workspace = true }
```
