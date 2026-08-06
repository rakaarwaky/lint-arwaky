# Plan: cli-commands — Architect (Merged Plan)

## Summary

The `cli-commands` crate is a well-structured Smart Surface with 11 source files implementing 14 FRDs. All handlers correctly delegate to `dispatcher::surface_*_action` functions — no business logic leaks into the surface layer. The crate follows AES406 conventions (surface_ prefix, `_command` suffix, zero direct lower-layer imports). However, a **critical FRD deviation** was found (`handle_check` routes to `collect_quality` instead of `collect_scan`, contradicting FR-001's 1:1 alias requirement), along with 6 code quality issues: duplicated formatting logic, inconsistent parameter patterns, unused Cargo dependencies, and redundant severity matching. The BA's claim of "4 AES406 violations" was **disagreed** — inline terminal output is legitimate smart surface presentation, not business logic.

## Merged Findings

### Layer Boundaries

| # | Severity | Issue | Location | Recommendation |
|---|----------|-------|----------|----------------|
| 1 | 🔴 CRITICAL | **FRD deviation: `handle_check` delegates to `collect_quality` instead of `collect_scan`** — FR-001 states "check and scan are 1:1 equivalent aliases that invoke the exact same analysis pipeline. Delegates to `dispatcher::surface_check_action::collect_scan`." Current code routes through `handle_quality → dispatcher::surface_quality_action::collect_quality` (single linter), not the full 6-group pipeline. | `surface_scan_command.rs:106-118` (`handle_check`) | Refactor `handle_check` to delegate to `dispatcher::surface_check_action::collect_scan` (same as `handle_scan`), making `check` a true 1:1 alias of `scan`. |

### Naming

| # | Severity | Issue | Location | Recommendation |
|---|----------|-------|----------|----------------|
| — | — | No naming violations found. All 10 source files follow `layer_concern_role.rs` pattern. Surface suffixes use `_command` (allowed). Utility suffix `_formatter` is flexible and not forbidden. `lib.rs` is an exception. | — | — |

### Orphan

| # | Severity | Issue | Location | Recommendation |
|---|----------|-------|----------|----------------|
| — | — | No orphan violations found. All surface files are imported by the binary entry point. `utility_output_text_formatter` is consumed by surface files. | — | — |

### Scalability

| # | Severity | Issue | Location | Recommendation |
|---|----------|-------|----------|----------------|
| 2 | 🟡 WARNING | **Inconsistent parameter patterns** — `handle_fix` takes 4 individual params, `handle_git_diff` takes 4 individual params, `handle_check` takes 6 individual params. All other handlers (`handle_scan`, `handle_import`, `handle_naming`, `handle_role`, `handle_orphan`, `handle_external`, `handle_ci`) use dedicated param structs. | `surface_fix_command.rs:12-17`, `surface_git_command.rs:10-14`, `surface_scan_command.rs:106-118` | Extract `FixCommandParams`, `GitDiffCommandParams`, and refactor `handle_check` to use `ScanCommandParams` (or a new `CheckCommandParams`). Consistent patterns reduce cognitive load and make adding new parameters non-breaking. |
| 3 | 🟡 WARNING | **Redundant `_config_orchestrator` param in `handle_check`** — parameter is accepted but never used (prefixed with `_`). Dead parameter adds noise to the signature. | `surface_scan_command.rs:111` | Remove `_config_orchestrator` from `handle_check` signature. If `handle_check` is refactored to use `ScanCommandParams` (finding #1), this resolves automatically. |

### Data Flow

| # | Severity | Issue | Location | Recommendation |
|---|----------|-------|----------|----------------|
| — | — | Data flow is correct — unidirectional bottom-up. All surface handlers receive aggregates via DI, delegate to dispatcher, and format output. No cycles detected. | — | — |

### Code Quality (Deduplication)

| # | Severity | Issue | Location | Recommendation |
|---|----------|-------|----------|----------------|
| 4 | 🟡 WARNING | **Duplicated location formatting** — identical `(line, column) → "file:line:col"` pattern appears in 3 places with identical match arms. | `surface_fix_command.rs:31-38`, `surface_git_command.rs:30-36`, `utility_output_text_formatter.rs:147-153` | Extract `format_location(file: &str, line: usize, column: usize) -> String` into `utility_output_text_formatter.rs` and call from all 3 sites. |
| 5 | 🟡 WARNING | **Redundant manual severity-to-string match** — `surface_git_command.rs` manually matches `Severity::CRITICAL => "CRITICAL"`, etc. But `Severity` already has a `Display` impl (outputs lowercase). The manual match can be simplified to `format!("{}", r.severity).to_uppercase()`. | `surface_git_command.rs:46-50` | Replace 5-arm match with `format!("{}", r.severity).to_uppercase()`. Note: Display outputs lowercase, git-diff intentionally uses uppercase — `.to_uppercase()` handles this. |

### Dependencies

| # | Severity | Issue | Location | Recommendation |
|---|----------|-------|----------|----------------|
| 6 | 🟡 WARNING | **Unused `rayon` dependency** — imported in Cargo.toml but zero usage in src/ or tests/. | `Cargo.toml:10` | Remove `rayon = { workspace = true }` from `[dependencies]`. |
| 7 | 🟡 WARNING | **Unused `regex` dependency** — imported in Cargo.toml but zero usage in src/ or tests/. | `Cargo.toml:11` | Remove `regex = { workspace = true }` from `[dependencies]`. |

### Maintainability

| # | Severity | Issue | Location | Recommendation |
|---|----------|-------|----------|----------------|
| 8 | 🟢 INFO | `utility_output_text_formatter.rs` is ~310 lines with `render_text` having 3 nested branches (70+ lines). Moderate complexity but not a violation — file is well-structured with clear section separators. | `utility_output_text_formatter.rs` | Consider extracting `render_text_member_detailed` helper to reduce nesting depth. Low priority — file is manageable. |

## BA Disagreements

| BA Finding | Architect Verdict | Rationale |
|---|---|---|
| **"4 AES406 violations from inline rendering logic"** in CI, fix, git-diff, formatter | **DISAGREED — Not AES406 violations** | AES406 checks for: (1) >15 functions, (2) active domain logic in passive surface, (3) role boundary violations. The inline terminal output in `surface_ci_command.rs` (~20 lines println), `surface_fix_command.rs` (~30 lines), `surface_git_command.rs` (~25 lines) is **legitimate smart surface presentation** — progress messages, headers, status output. This is NOT business logic. The formatter module (`utility_output_text_formatter.rs`) is correctly in the utility layer as a shared rendering utility. None of these exceed 15 functions or import forbidden layers. |
| **"FRD violates project conventions"** (file paths and signatures) | **OUT OF SCOPE** | This is a documentation concern, not an architectural code issue. FRD compliance is validated separately by the business analyst role. |
| **"Test suite is shallow"** | **OUT OF SCOPE** | Test coverage is a QA concern, not an architecture concern. 6 test files exist (contract, unit, integration, e2e, acceptance, smoke) + 1 benchmark. Adequate for architecture validation. |

## Validation

- [x] FRD compliance checked — 14 FRs mapped to handlers; FR-001 deviation identified (finding #1)
- [x] AES compliance checked — All 24 rules verified against code
- [x] Skip Report validated — No skips occurred (no skip report found)
- [x] Assumptions validated — `Severity` Display impl confirmed (outputs lowercase); manual match in git-diff outputs uppercase intentionally
- [x] Timestamp + Correlation ID signed — 2026-08-07T07:39:11+07:00, corr-20260807-cli-commands-414

## Action Items

| Priority | Item | Finding |
|----------|------|---------|
| 🔴 P0 | Refactor `handle_check` to delegate to `dispatcher::surface_check_action::collect_scan` — make check = scan per FR-001 | #1 |
| 🔴 P0 | Extract `format_location()` helper into `utility_output_text_formatter.rs`, call from 3 sites | #4 |
| 🔴 P0 | Replace manual severity match in `surface_git_command.rs` with `format!("{}", r.severity).to_uppercase()` | #5 |
| 🟡 P1 | Extract `FixCommandParams` and `GitDiffCommandParams` structs for consistency | #2 |
| 🟡 P1 | Remove unused `_config_orchestrator` from `handle_check` (resolves with #1 refactor) | #3 |
| 🟡 P1 | Remove unused `rayon` and `regex` from Cargo.toml | #6, #7 |
| 🟢 P2 | Consider extracting render helpers in `utility_output_text_formatter.rs` to reduce nesting | #8 |

## Fixed Code

### Finding #1: Refactor `handle_check` to use `collect_scan`

```rust
// surface_scan_command.rs — BEFORE:
/// `check` — quality scan (single linter).
pub fn handle_check(
    path: Option<FilePath>,
    format: Format,
    code_analysis_linter: Arc<dyn ICodeAnalysisAggregate>,
    filesystem: Arc<dyn IFilesystemAggregate>,
    _config_orchestrator: Option<Arc<dyn IConfigOrchestratorAggregate>>,
    filter: Option<String>,
) -> ExitCode {
    handle_quality(
        path,
        format,
        code_analysis_linter,
        filesystem,
        filter,
        Vec::new(),
    )
}

// AFTER:
/// `check` — 1:1 alias of `scan` per FR-001.
pub fn handle_check(params: ScanCommandParams) -> ExitCode {
    handle_scan(params)
}
```

Note: Callers of `handle_check` must be updated to construct `ScanCommandParams` instead of passing individual args. If the caller doesn't have `config_orchestrator`, pass `None`.

### Finding #4: Shared location helper

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

Then replace in `surface_fix_command.rs` (lines 31-38):

```rust
// BEFORE:
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

### Finding #5: Simplify severity match

```rust
// surface_git_command.rs — BEFORE (lines 46-50):
let sev = match r.severity {
    Severity::CRITICAL => "CRITICAL",
    Severity::HIGH => "HIGH",
    Severity::MEDIUM => "MEDIUM",
    Severity::LOW => "LOW",
    _ => "INFO",
};

// AFTER:
let sev = format!("{}", r.severity).to_uppercase();
```

Note: `_ => "INFO"` arm becomes `"INFO"` (lowercase → uppercase) via `.to_uppercase()`, which matches the original behavior.

### Findings #2: Consistent param structs

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
    pub project_path: Option<&'static str>,
    pub filter: Option<&'static str>,
}
```

### Findings #6 + #7: Remove unused dependencies

```toml
# Cargo.toml — remove these two lines:
rayon = { workspace = true }
regex = { workspace = true }
```
