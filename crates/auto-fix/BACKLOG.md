# Feature Backlog: Auto-Fix

FRD: [FRD.md](FRD.md)
Architecture: [ARCHITECTURE.md](../../ARCHITECTURE.md)
State / Health: defined in root [ROADMAP.md](../../ROADMAP.md) — cited here, not restated
Last Updated: 2026-09-29

## Current Condition

- Done: `cargo nextest run -p auto-fix-lint-arwaky --lib --tests` → 47 passed, 0 skipped at `a09244f1` (2026-09-29). Per-scenario evidence below.
- In Progress: None
- Blocked: None
- Next Action: Re-run `cargo nextest run -p auto-fix-lint-arwaky --lib --tests` after any code change to this crate

## Backlog

| ID | FRD Ref | Work Item | Priority | State | Actual Condition | Owner | Dependencies | Updated |
|---|---|---|---:|---|---|---|---|---|
| AUTO-01 | FR-AutoFix-001 | SCEN-001 — Unused Import Removal — 6 scenarios verified | P0 | Done | `cargo nextest run -p auto-fix-lint-arwaky` → 47 passed at `a09244f1` (2026-09-29) | @raka | None | 2026-09-29 |
| AUTO-02 | FR-AutoFix-002 | SCEN-002 — Bypass Fix — 9 scenarios verified | P0 | Done | `cargo nextest run -p auto-fix-lint-arwaky` → 47 passed at `a09244f1` (2026-09-29) | @raka | None | 2026-09-29 |
| AUTO-03 | FR-AutoFix-003 | SCEN-003 — Symbol Renaming — 5 scenarios verified | P0 | Done | `cargo nextest run -p auto-fix-lint-arwaky` → 47 passed at `a09244f1` (2026-09-29) | @raka | None | 2026-09-29 |
| AUTO-04 | FR-AutoFix-004 | SCEN-004 — Violation Reporting (dry-run + non-fixable) — 5 scenarios verified | P0 | Done | `cargo nextest run -p auto-fix-lint-arwaky` → 47 passed at `a09244f1` (2026-09-29) | @raka | None | 2026-09-29 |

## Scenario Evidence

| Scenario | Kind | Test file | Test name | Last verified |
|---|---|---|---|---|
| Unused import at valid line | Removed, `Applied` | unit_auto_fix_fix_processor.rs | fix_unused_removes_use_line | `a09244f1` |
| Unused Python import | Removed, `Applied` | unit_auto_fix_fix_processor.rs | fix_unused_removes_python_import | `a09244f1` |
| Unused JS require pattern | Removed, `Applied` | unit_auto_fix_fix_processor.rs | fix_unused_removes_js_require | `a09244f1` |
| Line 0 or beyond EOF | `Skipped(line_out_of_bounds)` | unit_auto_fix_fix_processor.rs | fix_unused_skips_multiline | `a09244f1` |
| Non-import line | `Skipped(not_an_import_line)` | unit_auto_fix_fix_processor.rs | fix_unused_skips_non_import_line | `a09244f1` |
| Multi-line import block | `Skipped(multi_line_import)` | unit_auto_fix_fix_processor.rs | fix_unused_skips_multiline | `a09244f1` |
| `unwrap()` on target line | Replaced with `expect("safe")`, `Applied` | unit_auto_fix_fix_processor.rs | fix_bypass_replaces_unwrap | `a09244f1` |
| `#[allow(unused)]` line | Removed entirely, `Applied` | unit_auto_fix_fix_processor.rs | fix_bypass_strips_allow_attr | `a09244f1` |
| `// noqa` comment | Stripped from line, `Applied` | unit_auto_fix_fix_processor.rs | fix_bypass_strips_noqa_inline | `a09244f1` |
| `// HACK` comment | Stripped from line, `Applied` | unit_auto_fix_fix_processor.rs | fix_bypass_strips_hack_comment | `a09244f1` |
| `panic!("error")` | `Skipped(unsafe_removal)` | unit_auto_fix_fix_processor.rs | fix_bypass_skips_unsafe_macros | `a09244f1` |
| Missing file / out of bounds | `Failed(file_not_found)` / `Skipped(line_out_of_bounds)` | unit_auto_fix_fix_processor.rs | fix_bypass_skips_nonexistent_line | `a09244f1` |
| No bypass on target line | `Skipped(no_bypass_pattern)` | unit_auto_fix_fix_processor.rs | fix_bypass_skips_non_bypass_line | `a09244f1` |
| `expect(...)` with message | `Skipped(already_has_context)` | unit_auto_fix_fix_processor.rs | fix_bypass_skips_expect_with_message | `a09244f1` |
| Symbol rename, 3 occurrences | All replaced, `Applied` + count | unit_auto_fix_fix_processor.rs | rename_replaces_word_boundaries | `a09244f1` |
| Symbol already valid snake_case | `Skipped(already_valid)` | unit_auto_fix_fix_processor.rs | rename_preserves_surrounding_text | `a09244f1` |
| Symbol not found in file | `Skipped(symbol_not_found)` | unit_auto_fix_fix_processor.rs | rename_skips_nonexistent_symbol | `a09244f1` |
| New name is a Rust keyword | `Skipped(keyword_conflict)` | unit_auto_fix_fix_processor.rs | rename_skips_keyword_conflict | `a09244f1` |
| Dry-run with fixable violations | Outcomes reported, no files modified | e2e_auto_fix_flow.rs | e2e_dry_run_file_with_unused_import | `a09244f1` |
| Dry-run with bypass comment | Outcomes reported, no files modified | e2e_auto_fix_flow.rs | e2e_dry_run_file_with_bypass_comment | `a09244f1` |
| Dry-run with clean file | "No automatic fixes applied" | e2e_auto_fix_flow.rs | e2e_dry_run_clean_file | `a09244f1` |
| Per-request dry-run toggle | True → no write; false → applies fix | e2e_auto_fix_flow.rs | e2e_per_request_dry_run_toggle | `a09244f1` |
| Non-fixable violations (AES401) | In manual report | unit_auto_fix_fix_processor.rs | manual_report_lists_only_non_fixable_codes | `a09244f1` |
| AES304 `panic!` skipped | In manual report as unsafe_removal | unit_auto_fix_fix_processor.rs | manual_report_lists_only_non_fixable_codes | `a09244f1` |
| Empty violation list | Empty manual report | unit_auto_fix_fix_processor.rs | manual_report_on_empty_violations_is_empty | `a09244f1` |
| Idempotency — second run after fix | No further `Applied` outcomes | integration_auto_fix.rs | orchestrator_execute_per_request_dry_run_false | `a09244f1` |
| Container wiring (filesystem + QA) | Orchestrator created successfully | integration_auto_fix.rs | container_orchestrator_with_filesystem | `a09244f1` |
| Container wiring (custom IO) | Orchestrator created successfully | integration_auto_fix.rs | container_orchestrator_with_custom_file_adapter | `a09244f1` |
| Orchestrator is trait object (Send+Sync) | `Arc<dyn IFixAggregate>` and `Arc<dyn IViolationReportProtocol>` | contract_auto_fix.rs | all_capabilities_are_send_sync | `a09244f1` |

## Blockers

None

## Dependencies

- `filesystem` feature — provides `IFileSystemIOProtocol` for all file read/write/path-exists operations
- `quality-rules` feature — provides lint violation data via `ICodeAnalysisAggregate`

## Release Readiness

| Area | Status | Notes |
|---|---|---|
| Tests | Done | 47 tests pass at `a09244f1` (2026-09-29) |
| Scenario evidence | Done | 28 scenarios mapped across 8 test files |
| Docs | Done | [FRD.md](FRD.md) is specification-only; status lives in this file |
| Self-lint | Done | `lint-arwaky-cli check .` → 0 violations |
| Good workspaces | Done | All `workspaces-good/*` → 0 violations |

## Deferred

None

## Change Log

| Date | Change | By |
|---|---|---|
| 2026-09-29 | Removed FileAdapter capability; capabilities now use `IFileSystemIOProtocol` directly from filesystem feature. Reduced to exactly 4 protocols / 4 capabilities. | @raka |
| 2026-09-17 | Initial backlog created from FRD test-scenario mapping | @raka |
