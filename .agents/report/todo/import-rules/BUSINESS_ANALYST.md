# Review Report: import-rules — Business Analyst

## Summary

The import-rules crate implements AES201–AES205 import compliance checks with a well-structured 7-layer architecture. The FRD is comprehensive with clear functional requirements, data models, and API contracts. However, several gaps exist between the FRD specification and actual implementation, particularly around scope-based rule matching, performance requirements, and test coverage specificity. The codebase follows AES conventions correctly with proper layer separation and naming.

## Findings by Category

### Requirements Clarity & Completeness

| # | Severity | Issue | Location | Recommendation |
|---|----------|-------|----------|----------------|
| 1 | 🟡 WARNING | FR-004 describes "Dummy or Forbidden Imports" as a single requirement, but implementation splits into two separate capabilities (DummyImportChecker + ArchImportForbiddenChecker) | FRD.md:43-52 | Split FR-004 into FR-004a (Dummy Imports) and FR-004b (Forbidden Imports) for clarity |
| 2 | 🟡 WARNING | FRD states "Test-only imports in production code are flagged" but no implementation detects test-only imports | FRD.md:50 | Add implementation for test-only import detection or remove from FRD |
| 3 | 🟢 INFO | FRD mentions "Conditional imports, feature flags" as edge cases for AES204 but no handling exists | FRD.md:51 | Document that conditional imports are out of scope or add detection logic |
| 4 | 🟡 WARNING | FRD API Contract lists `check_layer_violation()` for AES201 but implementation uses `check_forbidden_imports()` | FRD.md:89 | Update FRD API Contract to match actual method names |

### Testability & Acceptance Criteria

| # | Severity | Issue | Location | Recommendation |
|---|----------|-------|----------|----------------|
| 1 | 🟡 WARNING | Test scenarios lack specific pass/fail criteria (e.g., "Valid unidirectional import passes" — what defines valid?) | FRD.md:108-112 | Add concrete test cases with expected outputs |
| 2 | 🟡 WARNING | Performance requirement "Check 1000 files in < 2 seconds" has no benchmark test | FRD.md:102 | Add criterion benchmark in benches/ directory |
| 3 | 🟢 INFO | "Zero false positives for valid imports" is aspirational but untestable without a comprehensive valid-import corpus | FRD.md:104 | Define a test corpus of known-valid imports for regression testing |

### Scope & Dependencies

| # | Severity | Issue | Location | Recommendation |
|---|----------|-------|----------|----------------|
| 1 | 🟡 WARNING | FRD states "Import statements are parsed via regex (not AST)" but implementation uses utility functions that may use more sophisticated parsing | FRD.md:117 | Clarify parsing approach in FRD or confirm regex-only constraint |
| 2 | 🟢 INFO | Integration points mention "config-system (YAML rules), code-analysis (file reading)" but code-analysis is not a direct dependency | FRD.md:97-98 | Update integration points to reflect actual dependencies |

### Traceability (FRD ↔ Code)

| # | Severity | Issue | Location | Recommendation |
|---|----------|-------|----------|----------------|
| 1 | 🟡 WARNING | FR-001 (AES201) maps to `capabilities_import_forbidden_checker.rs` but FRD calls it "Layer Dependency Violation" | FRD.md:9 vs src/ | Align naming: FRD should say "Forbidden Import" to match code |
| 2 | 🟡 WARNING | FR-004 (AES204) maps to `capabilities_dummy_import_checker.rs` but FRD calls it "Dummy or Forbidden Imports" | FRD.md:43 vs src/ | Rename FR-004 to "Dummy Import Detection" since forbidden is FR-001 |
| 3 | 🟢 INFO | All 5 FRs have corresponding implementation files and test files | src/ + tests/ | Good traceability — maintain this pattern |

## Violations (if any)

No AES layer violations detected in the import-rules crate itself:

- **Naming (AES101/AES102)**: All files follow `layer_concern_role.rs` pattern
- **Import boundaries (AES201)**: Capabilities import only from Taxonomy, Contract, and Utility
- **Role compliance (AES403)**: All capabilities implement their respective protocols
- **Orphan detection**: All capabilities are wired in `root_import_rules_container.rs`

## Action Items

- [x] **HIGH** Update FRD API Contract table to match actual method names (`check_forbidden_imports` instead of `check_layer_violation`)
- [x] **HIGH** Split FR-004 into FR-004a (Dummy Imports) and FR-004b (Forbidden Imports) for implementation clarity
- [x] **MEDIUM** Add criterion benchmark test for "1000 files in < 2 seconds" performance requirement
- [x] **MEDIUM** Clarify parsing approach in FRD (regex vs utility functions)
- [x] **LOW** Add concrete test cases with expected outputs to FRD test scenarios
- [x] **LOW** Update integration points to reflect actual dependencies (remove code-analysis reference)

## Gap Analysis Table

| Current State | Issue | Recommendation | Priority |
|---------------|-------|----------------|----------|
| FRD describes 5 requirements | FR-004 conflates dummy and forbidden imports | Split into FR-004a and FR-004b | HIGH |
| API Contract lists `check_layer_violation()` | Implementation uses `check_forbidden_imports()` | Update FRD to match implementation | HIGH |
| Performance requirement exists | No benchmark test validates it | Add criterion benchmark | MEDIUM |
| Test scenarios listed | Lack specific pass/fail criteria | Add concrete test cases | LOW |
| Integration points defined | code-analysis not a direct dependency | Update integration points | LOW |
