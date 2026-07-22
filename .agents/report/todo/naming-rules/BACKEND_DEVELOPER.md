# Review Report: naming-rules — Backend Developer

## Summary

The `naming-rules` crate implements AES101 (Naming Convention) and AES102 (Suffix/Prefix Rules) enforcement for the lint-arwaky architecture scanner. The codebase follows the 7-layer AES pattern well with proper separation between agent (orchestration), capabilities (business logic), contract (protocols/aggregates), taxonomy (constants/VOs), and root (composition). No critical security vulnerabilities or data leaks exist — this is a pure linting tool with no I/O or network operations. The main concerns are a documentation/code discrepancy on minimum word count, minor dead code in the orchestrator, and duplicated helper methods across two capability files.

## Findings by Category

### Architecture & Layer Compliance

| #   | Severity | Issue | Location | Recommendation |
| --- | -------- | ----- | -------- | -------------- |
| 1   | 🟡 WARNING | Documentation/code mismatch — AGENTS.md states "min 2 words" for AES101, but code comment and regex enforce min 3 words | `capabilities_naming_convention_checker.rs` line 58, `FRD.md` | Align documentation with actual implementation (change FRD/AGENTS.md to say "min 3 words") or adjust the regex if 2 was intended |
| 2   | 🟡 WARNING | Dead code — `filter_source_files` in `agent_naming_orchestrator.rs` is defined but never called; `walk_recursive` already returns filtered results | `agent_naming_orchestrator.rs` lines 59–73 | Remove the unused `filter_source_files` method to reduce clutter |
| 3   | 🟢 INFO | Duplicated `_make_result` helper — both `NamingConventionChecker` and `SuffixPrefixChecker` implement identical `_make_result` methods | `capabilities_naming_convention_checker.rs`, `capabilities_suffix_prefix_checker.rs` | Extract into a shared utility function in `shared/naming-rules/utility_naming_checker.rs` to follow DRY principle |

### Security

| #   | Severity | Issue | Location | Recommendation |
| --- | -------- | ----- | -------- | -------------- |
| 1   | 🟢 INFO | No security vulnerabilities detected — this is a pure linting scanner with no I/O, network calls, or external data ingestion | All files | No action needed |

### Performance

| #   | Severity | Issue | Location | Recommendation |
| --- | -------- | ----- | -------- | -------------- |
| 1   | 🟢 INFO | Regex caching via `OnceLock` is correctly implemented in `naming_regex()` | `capabilities_naming_convention_checker.rs` lines 65–74 | No action needed — this is optimal |
| 2   | 🟢 INFO | File walking is performed once per audit run; filtering is O(n) on already-walked results | `agent_naming_orchestrator.rs` lines 30–31 | No optimization needed — linear scan on file list is appropriate for this scale |

### Error Handling

| #   | Severity | Issue | Location | Recommendation |
| --- | -------- | ----- | -------- | -------------- |
| 1   | 🟢 INFO | Proper `Result` types used throughout; no `unwrap()`, `expect()`, or `panic!()` in production code | All files | No action needed — error handling follows best practices |
| 2   | 🟡 WARNING | Hardcoded `LineNumber::new(1)` and `ColumnNumber::new(0)` in `_make_result` — intentional for file-level checks, but could be documented as such | Both checker files | Add a comment explaining this is intentional (file-level scan, not line-specific) |

## Violations (if any)

No AES layer violations detected:
- Agent (`agent_naming_orchestrator.rs`) correctly imports only `taxonomy` and `contract` layers — ✅ AES201 compliance
- Capabilities (`capabilities_*_checker.rs`) correctly import `taxonomy`, `contract` — ✅ AES201 compliance
- Root (`root_naming_rules_container.rs`) wires capabilities to contracts — ✅ AES201 compliance
- No circular imports — dependencies flow bottom-up — ✅ AES205 compliance
- All files have non-empty struct/enum/trait definitions — ✅ AES303 compliance

## Action Items

- [ ] **LOW** Update AGENTS.md and FRD.md to clarify minimum word count (currently says 2, code enforces 3)
- [ ] **LOW** Remove dead `filter_source_files` method from `agent_naming_orchestrator.rs`
- [ ] **LOW** Extract shared `_make_result` utility to reduce duplication across checker files

## Fixed Code

### Fix 1: Remove dead `filter_source_files` method

```rust
// In agent_naming_orchestrator.rs — REMOVE this unused method:

fn filter_source_files(
    files: &shared::common::taxonomy_paths_vo::FilePathList,
) -> shared::common::taxonomy_paths_vo::FilePathList {
    let filtered: Vec<FilePath> = files
        .values
        .iter()
        .filter(|f| {
            let path = Path::new(&f.value);
            path.extension()
                .and_then(|e| e.to_str())
                .map(|ext| SOURCE_EXTENSIONS.contains(&ext))
                .unwrap_or(false)
        })
        .cloned()
        .collect();
    shared::common::taxonomy_paths_vo::FilePathList::new(filtered)
}
```

Also remove the unused `use shared::naming_rules::taxonomy_naming_constant::SOURCE_EXTENSIONS;` import.

### Fix 2: Extract shared `_make_result` utility

Create `shared/naming-rules/utility_lint_result.rs`:

```rust
// PURPOSE: Shared helper to construct LintResult for naming-rule violations

use crate::cli_commands::taxonomy_result_vo::{LintResult, LintResultList};
use crate::taxonomy_adapter_name_vo::AdapterName;
use crate::taxonomy_common_vo::{ColumnNumber, LineNumber};
use crate::taxonomy_error_vo::ErrorCode;
use crate::taxonomy_lint_vo::{LocationList, ScopeRef};
use crate::taxonomy_message_vo::LintMessage;
use crate::taxonomy_path_vo::FilePath;
use crate::taxonomy_severity_vo::Severity;
use crate::taxonomy_suggestion_vo::DescriptionVO;

pub const ADAPTER_NAME: &str = "architecture";

pub fn make_file_result(
    file: &str,
    code: &str,
    msg: impl Into<String>,
    sev: Severity,
) -> LintResult {
    LintResult {
        file: FilePath::new(file).unwrap_or_default(),
        line: LineNumber::new(1),  // File-level check — not line-specific
        column: ColumnNumber::new(0),
        code: ErrorCode::raw(code),
        message: LintMessage::new(msg),
        source: Some(AdapterName::raw(ADAPTER_NAME)),
        severity: sev,
        enclosing_scope: Some(ScopeRef {
            name: DescriptionVO::new(String::new()),
            kind: DescriptionVO::new(String::new()),
            file: None,
            start_line: None,
            end_line: None,
        }),
        related_locations: LocationList::new(),
    }
}
```

Then replace both `_make_result` implementations with a call to `utility_lint_result::make_file_result()`.
