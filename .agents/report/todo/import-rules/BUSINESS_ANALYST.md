# Review Report: import-rules — Business Analyst

## Summary

The import-rules crate (import-rules-lint-arwaky) is **well-structured and largely complete**, with all 5 functional requirements (FR-001 through FR-005) mapped to their corresponding capability implementations and acceptance tests. The AES architecture follows the 7-layer pattern cleanly: root container wires 5 capabilities + 1 agent orchestrator, all using contract protocols. The main concerns are **overly ambitious non-functional claims** ("zero false positives", "1000 files in <2 seconds") that lack empirical validation, and a **regex-based parser limitation** acknowledged in the FRD but not mitigated with fallback strategies. Test coverage is adequate for happy paths but lacks negative test scenarios (e.g., multi-language edge cases, conditional imports, wildcard imports).

## Findings by Category

### Requirements Clarity & Completeness

| #   | Severity    | Issue                                                                                              | Location         | Recommendation                                                       |
| --- | ----------- | -------------------------------------------------------------------------------------------------- | ---------------- | -------------------------------------------------------------------- |
| 1   | 🟡 WARNING  | FRD claims "Import statements are parsed via regex (not AST)" — regex cannot handle all import patterns (e.g., conditional `#[cfg]` imports, macro-generated imports, multi-line imports) | FRD.md FR-001–FR-005 | Document known limitations and add AST-based parser as future enhancement; or add a "known limitation" section |
| 2   | 🟡 WARNING  | Non-functional requirement "Accuracy: Zero false positives for valid imports" is unmeasurable and unrealistic given regex parsing | FRD.md Non-functional Requirements | Revise to "Zero known false positives for supported import patterns" or define measurable threshold (e.g., 99.5%) |
| 3   | 🟡 WARNING  | Performance claim "Check 1000 files in < 2 seconds" has no benchmark evidence in the test suite | FRD.md Non-functional Requirements | Add benchmark test covering 1000-file scenario; or remove claim until validated |
| 4   | 🟢 INFO     | FR-004 (Dummy Import) scope is broader than other FRs — it checks dummy imports, dummy functions, dummy impls, taxonomy intent, AND surface logic. This may exceed the "import rules" crate scope | FRD.md FR-004 | Consider splitting into two crates: import-rules (AES201–203, 205) and quality-rules (AES204 dummy/quality checks) |
| 5   | 🟢 INFO     | Edge case "Conditional imports, feature flags" in FR-004 is mentioned but not addressed in implementation or tests | FRD.md FR-004 | Add test for `#[cfg(test)]` and `#[cfg(feature = "...")]` imports |

### Testability & Acceptance Criteria

| #   | Severity    | Issue                                                                                              | Location                        | Recommendation                                                       |
| --- | ----------- | -------------------------------------------------------------------------------------------------- | ------------------------------- | -------------------------------------------------------------------- |
| 6   | 🟡 WARNING  | acceptance_FR_003.rs (FR-003) tests only Rust imports — Python and JS/TS unused import detection is not tested | acceptance_FR_003.rs           | Add Python (`from foo import bar`) and JS/TS (`import { x } from 'y'`) test cases |
| 7   | 🟡 WARNING  | acceptance_FR_001.rs tests only Rust `use crate::` imports — cross-language import patterns (Python `import`, JS `require()`) are not validated | acceptance_FR_001.rs           | Add multi-language test fixtures for each supported language          |
| 8   | 🟢 INFO     | acceptance_FR_005.rs (FR-005) tests cycle detection at the graph level but NOT at the file/layer level — the orchestrator's `_scan` method which builds edges from actual files is not tested end-to-end | acceptance_FR_005.rs           | Add integration test that creates real files across layers and verifies cycle detection |
| 9   | 🟢 INFO     | No test for FR-002 exception handling — files listed in exceptions should skip mandatory import checks | acceptance_FR_002.rs (missing) | Add test case for exception list in FR-002                           |
| 10  | 🟢 INFO     | Bench test exists (bench_import_rules_throughput.rs) but no baseline data or performance assertions | bench_import_rules_throughput.rs | Add performance regression threshold (e.g., must not degrade >10% per release) |

### Scope & Dependencies

| #   | Severity    | Issue                                                                                              | Location                        | Recommendation                                                       |
| --- | ----------- | -------------------------------------------------------------------------------------------------- | ------------------------------- | -------------------------------------------------------------------- |
| 11  | 🟢 INFO     | Internal dependency on `config-system` (YAML rules) is well-defined but the FRD does not specify the YAML schema for layer hierarchy configuration | FRD.md Integration Points      | Add appendix with example YAML config showing layer definitions, exceptions, and rule configs |
| 12  | 🟢 INFO     | Internal dependency on `code-analysis` (file reading) is used but the FRD does not specify how file discovery works (recursive walk, ignore patterns, language detection) | FRD.md Integration Points      | Document the file collection algorithm used by `ImportOrchestrator::collect_files()` |
| 13  | 🟢 INFO     | No external dependencies — this is a self-contained linting crate. Good isolation.                    | Cargo.toml                      | No action needed                                                      |

### Traceability (FRD ↔ Code)

| #   | Severity    | Issue                                                                                              | Location                        | Recommendation                                                       |
| --- | ----------- | -------------------------------------------------------------------------------------------------- | ------------------------------- | -------------------------------------------------------------------- |
| 14  | 🟢 INFO     | FR-001 → `capabilities_import_forbidden_checker.rs` ✓ (ArchImportForbiddenChecker implements IImportForbiddenProtocol) | src/                            | Traceable — no action needed                                           |
| 15  | 🟢 INFO     | FR-002 → `capabilities_import_mandatory_checker.rs` ✓ (ArchImportMandatoryChecker implements IImportMandatoryProtocol) | src/                            | Traceable — no action needed                                           |
| 16  | 🟢 INFO     | FR-003 → `capabilities_import_unused_checker.rs` ✓ (UnusedImportRuleChecker implements IUnusedImportProtocol) | src/                            | Traceable — no action needed                                           |
| 17  | 🟢 INFO     | FR-004 → `capabilities_dummy_import_checker.rs` ✓ (DummyImportChecker implements IDummyImportCheckerProtocol) | src/                            | Traceable — no action needed                                           |
| 18  | 🟢 INFO     | FR-005 → `capabilities_cycle_import_analyzer.rs` ✓ (DependencyCycleAnalyzer implements ICycleImportProtocol) | src/                            | Traceable — no action needed                                           |
| 19  | 🟢 INFO     | Agent layer: `agent_import_orchestrator.rs` orchestrates all 5 capabilities with parallel execution for mandatory+forbidden | src/                            | Traceable — AES405 compliant (orchestrator calls multiple protocols) |
| 20  | 🟢 INFO     | Root layer: `root_import_rules_container.rs` wires all capabilities to protocols, follows AES container pattern | src/                            | Traceable — no action needed                                           |

## Violations (if any)

### AES Rule Compliance

| Code    | Severity | Description                                                                                             | Status  |
| ------- | -------- | ------------------------------------------------------------------------------------------------------- | ------- |
| AES101  | HIGH     | All filenames follow `prefix_concept_suffix` pattern (e.g., `capabilities_import_forbidden_checker.rs`) | ✅ Pass |
| AES102  | HIGH     | Suffixes match layer definitions: `_checker`, `_analyzer`, `_orchestrator`, `_container`                | ✅ Pass |
| AES201  | CRITICAL | Capabilities import only taxonomy, contract — no agent/infrastructure/surface/root imports              | ✅ Pass |
| AES202  | HIGH     | Capability files import their corresponding protocol (verified in source code)                          | ✅ Pass |
| AES303  | HIGH     | All capability files have non-empty struct definitions and impl blocks                                  | ✅ Pass |
| AES403  | HIGH     | All capabilities implement at least one protocol                                                        | ✅ Pass |
| AES405  | MEDIUM   | Orchestrator calls 5 subsystems (mandatory, forbidden, unused, cycle, dummy) — exceeds minimum of 2    | ✅ Pass |
| AES503  | MEDIUM   | All 5 capabilities are wired in ImportContainer — none are orphaned                                     | ✅ Pass |

### Potential Concerns (Not Violations Yet)

| Issue                                                                                             | Risk Level |
| -------------------------------------------------------------------------------------------------- | ---------- |
| DummyImportChecker contains business logic (surface logic detection, taxonomy intent analysis) that may belong in a separate quality-rules crate | Medium      |
| Regex-based parsing cannot handle all Rust/Python/JS import patterns (e.g., multi-line `use` statements with trailing commas, conditional imports) | Medium      |

## Action Items

- [ ] **HIGH** Revise non-functional requirements to replace "zero false positives" and "1000 files in <2 seconds" with measurable, evidence-based claims
- [ ] **HIGH** Add multi-language test fixtures (Python, JS/TS) for FR-001 through FR-004 acceptance tests
- [ ] **MEDIUM** Document known regex parsing limitations in FRD.md and consider adding AST-based parser as a future enhancement
- [ ] **MEDIUM** Add integration test for FR-005 that creates real files across layers and verifies cycle detection at the orchestrator level
- [ ] **LOW** Consider splitting FR-004 (Dummy Import) into a separate crate if scope continues to expand beyond import-specific checks
- [ ] **LOW** Add benchmark baseline and performance regression threshold

## Gap Analysis Table

| Current State                                                    | Issue                                      | Recommendation                                           | Priority |
| ---------------------------------------------------------------- | ------------------------------------------ | -------------------------------------------------------- | -------- |
| 5 acceptance tests covering happy paths only                     | No negative test scenarios (exceptions, edge cases) | Add exception handling tests for FR-001, FR-002          | HIGH     |
| Regex-based import parsing                                       | Cannot handle complex/multi-line imports   | Document limitation; plan AST parser enhancement         | MEDIUM   |
| Non-functional claims unverified                                 | "Zero false positives" and performance targets unvalidated | Run benchmarks, publish results, revise FRD              | HIGH     |
| Single-language test fixtures (Rust only)                        | Python/JS/TS import patterns not tested    | Add multi-language test files for each FR                | MEDIUM   |
| DummyImportChecker scope expanding beyond imports                | Checks dummy functions, impls, surface logic | Evaluate if scope fits "import-rules" or needs separate crate | LOW      |
| No YAML schema documentation                                     | Consumers cannot see expected config format | Add FRD appendix with example YAML config                | LOW      |

## Traceability Matrix (FRD ↔ Tests ↔ Code)

| FRD Requirement    | Acceptance Test              | Implementation File                              | Protocol Contract                          |
|--------------------|------------------------------|--------------------------------------------------|--------------------------------------------|
| FR-001: Layer Dependency Violation (AES201) | acceptance_FR_001.rs       | capabilities_import_forbidden_checker.rs         | IImportForbiddenProtocol                   |
| FR-002: Mandatory Layer Imports (AES202)    | acceptance_FR_002.rs       | capabilities_import_mandatory_checker.rs         | IImportMandatoryProtocol                   |
| FR-003: Unused Import Detection (AES203)    | acceptance_FR_003.rs       | capabilities_import_unused_checker.rs            | IUnusedImportProtocol                      |
| FR-004: Dummy or Forbidden Imports (AES204) | acceptance_FR_004.rs       | capabilities_dummy_import_checker.rs             | IDummyImportCheckerProtocol                |
| FR-005: Circular Dependency Detection (AES205)| acceptance_FR_005.rs     | capabilities_cycle_import_analyzer.rs            | ICycleImportProtocol                       |

**Orchestration:** agent_import_orchestrator.rs → IImportRunnerAggregate.run_audit()  
**Composition Root:** root_import_rules_container.rs → ImportContainer (wires all 5 capabilities)

## Overall Requirements Health Score: 🟡 7.5/10

**Strengths:**
- Clean AES architecture with proper layering (root → agent → capabilities → contract → taxonomy)
- All 5 FRs have corresponding implementations and acceptance tests
- Good protocol-based abstraction — capabilities are swappable
- Comprehensive dummy import detection (imports, functions, impls, taxonomy intent, surface logic)

**Areas for Improvement:**
- Multi-language test coverage (currently Rust-only)
- Non-functional requirements need empirical validation
- Regex parsing limitation should be documented with mitigation plan
- Acceptance tests lack negative scenarios and edge cases
