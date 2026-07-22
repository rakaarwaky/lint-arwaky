# Review Report: import-rules — Backend Developer

## Summary

The `import_rules-lint-arwaky` crate implements 5 AES import compliance checks (AES201–AES205) across 5 capabilities, 1 orchestrator agent, and 1 root container. The crate follows the AES 7-layer architecture well: capabilities are stateless structs implementing protocol traits, the agent orchestrates via `IImportRunnerAggregate`, and the root container wires everything together. Two pre-existing test failures in the dummy import checker (`unit_import_rules_dummy_checker`) were caused by (1) string literal false positives in symbol usage detection and (2) incomplete taxonomy import path patterns. Both have been fixed. All 80+ tests now pass, and clippy is clean.

## Findings by Category

### Architecture & Layer Compliance

| #   | Severity | Issue | Location | Recommendation |
| --- | -------- | ----- | -------- | -------------- |
| 1   | 🟡 WARNING | `Cargo.toml` package name uses dash (`import_rules-lint-arwaky`) instead of underscore convention | `crates/import-rules/Cargo.toml` | Rename to `import_rules_lint_arwaky` for workspace consistency; all other crates use underscores |
| 2   | 🟡 WARNING | Bench target filename `bench_import_rules_throughput.rs` exists in both `integration-test` and `bench` targets | `crates/import-rules/Cargo.toml` | Same as other crates — this is a workspace-wide issue, not import-rules specific |
| 3   | 🟢 INFO | `agent_import_orchestrator.rs` uses `tokio::join!` for parallel mandatory+forbidden checks but sequential loops for unused+dummy per-file checks | `src/agent_import_orchestrator.rs:68-100` | Consider extracting per-file checks into a separate capability or using `try_join!` for the 5 dummy sub-checks |
| 4   | 🟢 INFO | `_check_scope_forbidden_imports` and `_check_scope_mandatory_imports` share ~60 lines of nearly identical scope-matching logic | `capabilities_import_forbidden_checker.rs` / `capabilities_import_mandatory_checker.rs` | Extract shared scope-matching into a utility function in `utility_import_resolver` |

### Security

| #   | Severity | Issue | Location | Recommendation |
| --- | -------- | ----- | -------- | -------------- |
| 1   | 🟢 INFO | File content is read synchronously via `std::fs::read_to_string` inside the orchestrator's per-file loop | `agent_import_orchestrator.rs:87-88` | Consider using `tokio::fs::read_to_string` or keep `spawn_blocking` for consistency with async pattern |

### Performance

| #   | Severity | Issue | Location | Recommendation |
| --- | -------- | ----- | -------- | -------------- |
| 1   | 🟡 WARNING | `layer_keys` vector is rebuilt on every file in `_check_forbidden_imports` and `_check_mandatory_imports` | `capabilities_import_forbidden_checker.rs:67`, `capabilities_import_mandatory_checker.rs:54` | Pre-compute once per `run_audit` call and pass as parameter |
| 2   | 🟡 WARNING | `parse_import_lines_helper` is called per-file in both forbidden and mandatory checkers, with separate content reads | Cross-capability | Consider a shared import-cache in the orchestrator to avoid duplicate file reads |
| 3   | 🟢 INFO | `is_ignored` performs string operations (`strip_prefix`, `contains`) per file during directory walk | `agent_import_orchestrator.rs:158-172` | Build a `HashSet<String>` of ignored patterns for O(1) lookups |
| 4   | 🟢 INFO | `find_enclosing_string_start` scans from byte 0 to `pos` on every symbol occurrence | `utility_dummy_detector.rs:230-275` (new) | Use a forward-scanning approach or cache string boundaries for multi-pass analysis |

### Error Handling

| #   | Severity | Issue | Location | Recommendation |
| --- | -------- | ----- | -------- | -------------- |
| 1   | 🟢 INFO | `unwrap_or_default()` used silently on `FilePath::new()` failures in `collect_files` and `walk_dir` | `agent_import_orchestrator.rs:189-190, 200-201` | Consider logging or collecting parse errors for debug builds |

## Violations (Fixed)

### Bug Fix: String Literal False Positive in Symbol Usage Detection

**Root cause**: `symbol_used_real()` in `utility_dummy_detector.rs` used `contains_ident()` to check if an imported symbol appeared in the file body. When a symbol like `FilePath` appeared inside a string literal (e.g., `println!("no FilePath here")`), it was incorrectly treated as real usage. This caused `import_only_used_in_dummy_function_flagged` and `taxonomy_import_only_in_dummy_flagged` tests to fail.

**Fix**: Added `is_symbol_only_in_strings()` helper that scans for all occurrences of the symbol and verifies whether they fall exclusively inside double-quoted string literals or comments. If so, the symbol is correctly treated as unused.

### Bug Fix: Incomplete Taxonomy Import Path Pattern

**Root cause**: `_check_taxonomy_intent()` in `DummyImportChecker` only checked for `use shared::taxonomy_` patterns but missed `use shared::common::taxonomy_*` paths (which is where `FilePath`, `LineNumber`, etc. actually live). This caused the taxonomy intent check to never flag imports used only in dummy functions.

**Fix**: Added `use shared::common::taxonomy_` pattern to both taxonomy detection checks in `_check_taxonomy_intent()`.

## Action Items

- [ ] 🟡 **LOW** Fix `Cargo.toml` package name from `import_rules-lint-arwaky` to `import_rules_lint_arwaky` (underscore convention)
- [ ] 🟢 **LOW** Pre-compute `layer_keys` vector once per audit run instead of per-file
- [ ] 🟢 **LOW** Extract shared scope-matching logic from forbidden/mandatory checkers into a utility function

## Fixed Code

### Fix 1: String Literal Detection in `utility_dummy_detector.rs`

Added `is_symbol_only_in_strings()`, `find_enclosing_string_start()`, and `find_line_start()` functions to correctly identify when a symbol only appears inside string literals or comments (not real code usage):

```rust
/// Check if all occurrences of `needle` in `haystack` appear strictly inside
/// double-quoted string literals. Returns true when the symbol is never used
/// as a code identifier (only inside strings, comments, or doc lines).
pub fn is_symbol_only_in_strings(haystack: &str, needle: &str) -> bool { ... }

fn find_enclosing_string_start(haystack: &str, pos: usize) -> Option<usize> { ... }

fn find_line_start(haystack: &str, pos: usize) -> usize { ... }
```

And the guard in `symbol_used_real()`:
```rust
// If the symbol only appears inside string literals, it's not real usage
if is_symbol_only_in_strings(trimmed, symbol) {
    continue;
}
```

### Fix 2: Taxonomy Import Pattern in `capabilities_dummy_import_checker.rs`

Added `use shared::common::taxonomy_` pattern to both taxonomy detection checks:
```rust
LanguageVO::Rust => {
    t.contains("use shared::taxonomy_")
        || t.contains("use shared::common::taxonomy_")  // ← added
        || t.contains("use crate::common::taxonomy_")
        || t.contains("use crate::taxonomy_")
}
```

## Test Results

All 80+ tests pass across all test suites:
- `acceptance_FR_001` through `acceptance_FR_005`: 18 tests ✅
- `contract_import_rules`: 17 tests ✅
- `e2e_import_rules_audit_flow`: 3 tests ✅
- `integration_import_rules`: 7 tests ✅
- `smoke_import_rules`: 2 tests ✅
- `unit_import_rules_cycle_analyzer`: 9 tests ✅
- `unit_import_rules_dummy_checker`: 12 tests ✅ (was 10/12, now 12/12)
- `unit_import_rules_forbidden_checker`: 5 tests ✅
- `unit_import_rules_mandatory_checker`: 5 tests ✅
- `unit_import_rules_orchestrator`: 6 tests ✅
- `unit_import_rules_unused_checker`: 12 tests ✅

Clippy clean on both `shared-lint-arwaky` and `import_rules-lint-arwaky`.
