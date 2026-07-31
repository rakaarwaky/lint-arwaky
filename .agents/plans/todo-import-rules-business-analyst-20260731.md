# Review Plan: import-rules — Business Analyst

## Summary

The `import-rules` crate enforces AES201–AES205 import boundary rules across 7 layers. The architecture is well-structured: root wires, agent orchestrates, 5 capabilities implement protocols, utilities in shared. However, **22 tests are failing** — all positive-detection tests that expect violations to be caught. The root cause pattern: the forbidden/mandatory checkers are not detecting layer violations in test environments because the import resolver's layer detection from import paths is not matching. Additionally, the FRD describes v1.12.0 (AST-based) but Cargo.toml still reads v1.11.0, indicating an incomplete version bump. The implementation code does use the AST parser dispatch (v1.12 behavior) but the version was never updated.

## Findings by Category

### Requirements Clarity

| # | Severity | Issue | Location (File:Line) | Recommendation |
|---|----------|-------|----------------------|----------------|
| 1 | 🟡WARNING | FRD version (v1.12.0) mismatches Cargo.toml (v1.11.0) — creates confusion about which version of the migration is in effect | `Cargo.toml:2` vs `FRD.md:1` | Bump Cargo.toml to `1.12.0` to match FRD |
| 2 | 🟢INFO | FRD describes 10 FRs (FR-001 through FR-010) but only FR-001 through FR-005 have acceptance tests. FR-006 through FR-010 lack dedicated acceptance tests. | `tests/` directory | Create `acceptance_FR_006.rs` for AES205 full-cycle detection. FR-007/008/009/010 are utility/shared — integration coverage is acceptable. |
| 3 | 🟢INFO | FRD §API Contract lists 12 operations but not all are directly testable as public API (e.g., `parse_file`, `detect_cycle_edges` are internal) | `FRD.md:267-282` | Clarify which operations are public API vs internal implementation detail |

### Business Flow

| # | Severity | Issue | Location (File:Line) | Recommendation |
|---|----------|-------|----------------------|----------------|
| 4 | 🔴**CRITICAL** | **22 tests failing** — ALL positive-detection tests for AES201 (forbidden), AES202 (mandatory positive), AES203 (unused positive), integration, and e2e tests fail. The pattern: tests create temp files with forbidden imports but the orchestrator returns zero violations for those files. | `tests/acceptance_FR_001.rs` (5 failures), `tests/acceptance_FR_002.rs` (1 failure), `tests/acceptance_FR_003.rs` (2 failures), `tests/integration_import_rules.rs` (2 failures), `tests/e2e_import_rules_audit_flow.rs` (3 failures), `tests/unit_import_rules_dummy_checker.rs` (2 failures), `tests/unit_import_rules_forbidden_checker.rs` (2 failures), `tests/unit_import_rules_mandatory_checker.rs` (1 failure), `tests/unit_import_rules_unused_checker.rs` (2 failures) | Investigate root cause: the AST parser dispatch likely returns empty import lists for standalone test files (no workspace/crate context). The `parse_import_lines_helper` delegates to `utility_orphan_parser_dispatch::parse_file()` which may need a valid Rust file context to parse `use crate::` paths. |
| 5 | 🟡WARNING | The forbidden checker (`_check_forbidden_imports_with_lines`) uses `flat_map` on `files.values.iter()` instead of `par_iter()`. Meanwhile the orchestrator uses `par_iter` for file_violations. The inconsistency means AES201/AES202 are sequential while AES203/AES204 are parallelized — missing optimization opportunity. | `capabilities_import_forbidden_checker.rs:56` and `capabilities_import_mandatory_checker.rs:57` | Consider `par_iter` for consistency and performance |
| 6 | 🟡WARNING | `_check_forbidden_imports_with_lines` (line 32) calls `utility_path_normalizer::extract_layer_from_prefix("")` for no reason — dead code side-effect | `capabilities_import_forbidden_checker.rs:32` | Remove the unused call |

### Logic Implementation

| # | Severity | Issue | Location (File:Line) | Recommendation |
|---|----------|-------|----------------------|----------------|
| 7 | 🔴**CRITICAL** | **Root cause of 22 test failures**: The AST parser dispatch (`parse_file`) returns parsed results for the temp-dir test files, but `parse_import_lines_helper` only extracts imports from valid Rust AST — the test files contain `use crate::capabilities_checker::Checker;` which IS valid Rust syntax, but the parser may be returning an empty import list because the file content doesn't parse as a complete Rust module. Need to verify: is the file content being parsed as-is, or does it require a specific format? | `capabilities_import_forbidden_checker.rs:68` → `utility_import_resolver.rs:24-26` | Debug: add logging to `parse_import_lines_helper` to see what AST nodes are extracted from test file content. The test files are minimal snippets (e.g., `"use crate::capabilities_checker::Checker;\npub struct V;\n"`) — verify syn can parse these standalone. |
| 8 | 🟡WARNING | `_check_surface_logic` in `capabilities_dummy_import_checker.rs` constructs pattern strings using char arrays (`mk(&['l','i','n','t','_','p','a','t','h','('])`) to avoid false string matching — but this is unnecessarily verbose. A simple `const` or `&str` would be clearer. | `capabilities_dummy_import_checker.rs:383-395` | Use `const` string literals instead of `mk()` |
| 9 | 🟡WARNING | `_check_taxonomy_intent` hardcodes `source_layer: "surfaces"` regardless of the actual file layer | `capabilities_dummy_import_checker.rs:369` | Use `ctx.layer_name` instead of hardcoded `"surfaces"` |
| 10 | 🟡WARNING | `check_layer_contract_intent` is a no-op (always returns `Ok(Vec::new())`) — declared in protocol but never implemented | `capabilities_dummy_import_checker.rs:144-152` | Either implement the check or remove from the protocol. If it's a future placeholder, document it as FR-011 or similar. |
| 11 | 🟡WARNING | `DependencyCycleAnalyzer::normalize_to_layer` uses `unwrap_or(name)` for `split('_').next()` which will panic in debug builds on empty strings — not a runtime panic due to `unwrap_or` but `split('_').next()` always returns `Some`, so the `unwrap_or` is dead code | `capabilities_cycle_import_analyzer.rs:57` | Simplify to just `split('_').next().unwrap_or(name)` (already safe) or just `split('_').next().unwrap()` since `split` always yields at least one element |

### Testability & Acceptance Criteria

| # | Severity | Issue | Location (File:Line) | Recommendation |
|---|----------|-------|----------------------|----------------|
| 12 | 🔴**CRITICAL** | **Test coverage severely impacted by failures**: Out of ~130 total tests, 22 fail. The failures span ALL acceptance test files (FR-001 through FR-004 positive cases), integration, e2e, and unit tests. This means the core detection pipeline (forbidden, mandatory, unused, dummy) is NOT verified as working. | All `tests/` files | Priority 1: Fix the AST parser's handling of test-file content so positive detection tests pass |
| 13 | 🟡WARNING | `acceptance_FR_005.rs` test `fr005_self_edge_not_flagged_as_cross_layer` has no assertion — it calls `detect_cycle_edges` but `let _ = result;` suppresses the result. This means self-loop behavior is unverified. | `acceptance_FR_005.rs:120` | Add explicit assertion: either assert `result.is_empty()` (if self-loops are expected to be filtered at orchestrator level) or remove the test and document that self-loop filtering happens upstream |
| 14 | 🟡WARNING | Benchmark `perf_1000_files_under_2_seconds` and `bench_cycle_analyzer` both create temp files in a loop but `bench_cycle_analyzer` has unused `files` variable — the benchmark loop creates files but the `files` vec built from those paths is never moved into the benchmark closure properly | `benches/bench_import_rules_throughput.rs:251-261` | The `files` variable is computed but `group.bench_with_input` uses `&files` via `fls` — actually this looks correct, but the inner closure uses `analyzer.scan(&config, &layer_map, fls, &root)` which takes `&[FilePath]` so it should work |

### Traceability (FRD → Code)

| # | Severity | Issue | Location (File:Line) | Recommendation |
|---|----------|-------|----------------------|----------------|
| 15 | 🟡WARNING | FR-006 (AES205) has acceptance test `acceptance_FR_005.rs` but the test file is named FR_005, not FR_006. The FRD maps FR-006 to AES205 cycle detection. The naming mismatch breaks traceability. | `tests/acceptance_FR_005.rs` vs `FRD.md:172` | Rename to `acceptance_FR_006.rs` or update FRD to map FR-005 → AES205 |
| 16 | 🟢INFO | No acceptance tests for FR-007 (AST parser), FR-008 (barrel resolution), FR-009 (config suppression), FR-010 (macro handling). These are utility/shared functions tested indirectly through integration tests. | Missing test files | Low priority — coverage via integration tests is acceptable for utility functions |

## Violations

| Code | Rule | Location | Description |
|------|------|----------|-------------|
| AES304 | Bypass Comment | `capabilities_import_forbidden_checker.rs:32` | Dead code: `let _ = utility_path_normalizer::extract_layer_from_prefix("");` — side-effect call with no purpose |
| — | Version Mismatch | `Cargo.toml:2` | Version `1.11.0` but FRD specifies `1.12.0` |

## Action Items

- [ ] **P0 CRITICAL** Investigate and fix 22 test failures — likely AST parser dispatch not extracting imports from minimal test file snippets (temp dir, no workspace context)
- [ ] **P0 CRITICAL** Bump `Cargo.toml` version from `1.11.0` to `1.12.0` to match FRD
- [ ] **P1 WARNING** Remove dead code call at `capabilities_import_forbidden_checker.rs:32`
- [ ] **P1 WARNING** Remove no-op `check_layer_contract_intent` or document as future placeholder
- [ ] **P1 WARNING** Add assertion to `fr005_self_edge_not_flagged_as_cross_layer` or remove test
- [ ] **P1 WARNING** Fix hardcoded `"surfaces"` in `_check_taxonomy_intent` to use `ctx.layer_name`
- [ ] **P1 WARNING** Consider renaming `acceptance_FR_005.rs` → `acceptance_FR_006.rs` to match FRD mapping
- [ ] **P2 INFO** Clean up `mk()` char-array pattern in `_check_surface_logic` — use string literals
- [ ] **P2 INFO** Create `acceptance_FR_006.rs` for AES205 full-cycle detection if not already covered

## Fixed Code

### Fix 1: `Cargo.toml` — Version bump

```toml
[package]
name = "import-rules-lint-arwaky"
version = "1.12.0"
edition = "2021"
```

### Fix 2: `capabilities_import_forbidden_checker.rs` — Remove dead call

```rust
// BEFORE (line 31-33):
    fn rule_name(&self) -> Identity {
        let _ = utility_path_normalizer::extract_layer_from_prefix("");
        Identity::new("AES201")
    }

// AFTER:
    fn rule_name(&self) -> Identity {
        Identity::new("AES201")
    }
```

### Fix 3: `capabilities_dummy_import_checker.rs` — Use ctx.layer_name

```rust
// BEFORE (line 366-375):
                violations.push(LintResult::new_arch(file, dummy_function_line, "AES204", Severity::HIGH,
                    AesImportViolation::ImportIntentViolation {
                        source_layer: LayerNameVO::new("surfaces"),
                        import_type: SymbolName::new("taxonomy"),
                        intent: SymbolName::new("Use taxonomy Value Objects in function signatures instead of primitives"),
                        reason: Some(LintMessage::new(
                            "Taxonomy VOs encode domain concepts — using raw primitives defeats the purpose."
                        )),
                    }.to_string(),
                ));

// AFTER:
                violations.push(LintResult::new_arch(file, dummy_function_line, "AES204", Severity::HIGH,
                    AesImportViolation::ImportIntentViolation {
                        source_layer: LayerNameVO::new(ctx.layer_name.clone()),
                        import_type: SymbolName::new("taxonomy"),
                        intent: SymbolName::new("Use taxonomy Value Objects in function signatures instead of primitives"),
                        reason: Some(LintMessage::new(
                            "Taxonomy VOs encode domain concepts — using raw primitives defeats the purpose."
                        )),
                    }.to_string(),
                ));
```

### Fix 4: `capabilities_import_unused_checker.rs` — Inline fn for __future__ detection

The `unused_import_is_future_import` function at line 65-67 is duplicated between this file and `capabilities_dummy_import_checker.rs` (the `is_future_import` function at line 431-439). They do the same thing with slightly different APIs. Consider extracting to a shared utility in `utility_import_resolver`.

**Current duplicate in unused_checker.rs:**
```rust
fn unused_import_is_future_import(content: &str, alias: &str) -> bool {
    content.lines().any(|line| { let trimmed = line.trim(); trimmed.starts_with("from __future__ import ") && (trimmed == format!("from __future__ import {}", alias) || trimmed.contains(format!(", {}", alias).as_str()) || trimmed.contains(format!(" {},", alias).as_str())) })
}
```

**Current duplicate in dummy_checker.rs:**
```rust
fn is_future_import(lines: &[&str], symbol: &str) -> bool {
    lines.iter().any(|line| {
        let trimmed = line.trim();
        trimmed.starts_with("from __future__ import ")
            && (trimmed == format!("from __future__ import {}", symbol)
                || trimmed.contains(format!(", {}", symbol).as_str())
                || trimmed.contains(format!(" {},", symbol).as_str()))
    })
}
```

**Recommendation:** Extract to `utility_import_resolver::is_future_import(content: &str, symbol: &str) -> bool` and call from both files. This eliminates code duplication (AES305 concern).
