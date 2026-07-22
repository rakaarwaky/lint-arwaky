# Review Report: naming-rules — Business Analyst

## Summary

The `naming-rules` crate enforces AES101 and AES102 naming conventions (file stem pattern validation and suffix/prefix layer alignment). The FRD is clear with 2 functional requirements mapped to testable business rules. The crate follows the 7-layer AES architecture well — contract layer defines protocols and aggregates, capabilities implement single responsibilities, agent orchestrates both checkers, root wires everything together via DI, and shared taxonomy types are cleanly separated. Test coverage is comprehensive: 11 test files including acceptance tests for both FR-001 and FR-002, unit tests for both checkers, integration tests for the container and orchestrator, and a throughput benchmark. The implementation is solid but there are a few gaps between FRD promises and code reality — notably the FRD API contract table references methods that don't exist in the actual capabilities layer (different method signatures), and the `filter_source_files` helper in the agent layer is stateless freestanding logic that could be extracted to shared utility per project feedback conventions.

---

## Findings by Category

### Requirements Clarity & Completeness

| #   | Severity | Issue | Location | Recommendation |
| --- | -------- | ----- | -------- | -------------- |
| 1   | 🟢 INFO | **FRD is well-structured** — Both FR-001 (AES101) and FR-002 (AES102) have clear descriptions, inputs, outputs, business rules, edge cases, and error handling. The FRD follows the project template faithfully. | `crates/naming-rules/FRD.md` | No action needed — this is a positive finding. |
| 2   | 🟢 INFO | **FRD API contract table is outdated** — The FRD lists functions like `check_naming_convention()` and `check_layer_alignment()` but the actual capabilities layer uses `NamingConventionChecker::check_file_naming()` and `SuffixPrefixChecker::check_domain_suffixes()`. The FRD API table doesn't match the implementation. | `crates/naming-rules/FRD.md` | Update the FRD API contract table to reflect actual method signatures, or add a CI step that validates FRD contracts against code. |
| 3   | 🟢 INFO | **FRD non-functional requirements are minimal** — The FRD lists performance (1000 files in < 1s), memory (O(1) per file), and accuracy (zero false positives) but provides no concrete benchmarks or measurement methodology. The benchmark test `bench_naming_rules_throughput.rs` exists but isn't integrated into CI. | `crates/naming-rules/FRD.md` | Add the benchmark to CI with a performance regression threshold. Document the measured throughput in the FRD. |
| 4   | 🟢 INFO | **PRD alignment** — The naming-rules crate directly implements P0 features from the PRD: multi-language scanning (supports rs, py, js, ts, jsx, tsx via `SOURCE_EXTENSIONS`) and 24 AES rules enforcement (AES101 + AES102 are part of the 24). | `PRD.md`, `crates/naming-rules/FRD.md` | No action needed — this is a positive finding. |

### Testability & Acceptance Criteria

| #   | Severity | Issue | Location | Recommendation |
| --- | -------- | ----- | -------- | -------------- |
| 5   | 🟢 INFO | **Acceptance tests map 1:1 to FRD** — `acceptance_FRD_001.rs` and `acceptance_FRD_002.rs` cover every FRD business rule with explicit FR-XXX test functions. Each test is named to match the FRD requirement (e.g., `fr001_valid_snake_case_passes`, `fr002_taxonomy_vo_passes`). | `crates/naming-rules/tests/` | No action needed — this is a positive finding. |
| 6   | 🟢 INFO | **Unit tests cover edge cases** — `unit_naming_convention_checker.rs` tests valid names, invalid names (uppercase, camelCase, hyphen separator), unknown prefixes, barrel files, entry points, dotfiles, multi-dot files, and empty inputs. Unit tests for suffix checker also exist. | `crates/naming-rules/tests/` | No action needed — this is a positive finding. |
| 7   | 🟡 WARNING | **Integration tests focus on container wiring** — `integration_naming_rules_container.rs` and `integration_naming_rules.rs` test the DI container and orchestrator wiring but don't validate end-to-end audit runs against real file trees. Adding an integration test that walks a mock project structure would close the coverage gap. | `crates/naming-rules/tests/` | Add an integration test that creates a tempdir with mixed valid/invalid files and verifies the full pipeline produces correct results. |
| 8   | 🟡 WARNING | **Benchmark test has duplicate target conflict** — `bench_naming_rules_throughput.rs` is declared under both `[test]` (integration-test) and `[bench]` targets in Cargo.toml, causing a duplicate file-to-target warning. This conflicts with the workspace-wide issue but is specifically visible in this crate. | `crates/naming-rules/Cargo.toml` | Move the benchmark to `tests/bench/` directory and declare it only under `[dev-dependencies]` with `required-features = ["bench"]` or use `--bench` exclusively. |
| 9   | 🟢 INFO | **FRD test scenarios checklist is complete** — All 6 FRD test scenarios (valid snake_case, non-snake_case fails, < 3 words fails, correct suffix passes, wrong suffix fails, exception files pass) are covered by acceptance tests. | `crates/naming-rules/FRD.md` | No action needed — this is a positive finding. |

### Scope & Dependencies

| #   | Severity | Issue | Location | Recommendation |
| --- | -------- | ----- | -------- | -------------- |
| 10  | 🟢 INFO | **Clean layer boundaries** — The crate follows AES conventions perfectly: `contract_*` traits define protocols, `capabilities_*` implement them, `agent_*` orchestrates via contracts, `root_*` wires everything. No cross-layer violations. | `crates/naming-rules/src/` | No action needed — this is a positive finding. |
| 11  | 🟢 INFO | **Shared layer support is well-organized** — The shared crate has 9 naming-rules-specific files: 2 contracts, 1 aggregate, 1 constant, 1 rule VO, 1 violation VO, and 3 utility modules (naming, checker, filesystem). Utilities are properly separated from capabilities. | `crates/shared/src/naming-rules/` | No action needed — this is a positive finding. |
| 12  | 🟡 WARNING | **Agent layer has stateless freestanding function** — `NamingOrchestrator::filter_source_files()` in `agent_naming_orchestrator.rs` is a stateless function (no `&self`) that filters files by extension. Per project feedback memory "extract freestanding functions", this should be extracted to a shared utility module (`shared::naming_rules::utility_*`). | `crates/naming-rules/src/agent_naming_orchestrator.rs` | Extract `filter_source_files` to `shared::naming_rules::utility_file_filter.rs` as a standalone function. |
| 13  | 🟢 INFO | **Configuration-driven design** — Both checkers read from `ArchitectureConfig` and `LayerMapVO`, making rules configurable without code changes. The `min_words_from_config()` helper in the convention checker respects user-defined word counts with a sensible default of 3. | `crates/naming-rules/src/` | No action needed — this is a positive finding. |
| 14  | 🟢 INFO | **Regex caching for performance** — `NamingConventionChecker::naming_regex()` uses `OnceLock<Option<Regex>>` to cache compiled regex patterns based on `min_words`, avoiding recompilation per file. This aligns with the O(1) per file memory claim in the FRD. | `crates/naming-rules/src/capabilities_naming_convention_checker.rs` | No action needed — this is a positive finding. |

### Traceability (FRD ↔ Code)

| #   | Severity | Issue | Location | Recommendation |
| --- | -------- | ----- | -------- | -------------- |
| 15  | 🟡 WARNING | **FRD API table does not match code** — FRD lists `check_naming_convention()` / `check_layer_alignment()` but actual implementations are `NamingConventionChecker::check_file_naming()` and `SuffixPrefixChecker::check_domain_suffixes()`. The FRD API contract section is outdated. | `crates/naming-rules/FRD.md` | Update the FRD API table with actual method signatures from the capabilities layer. |
| 16  | 🟢 INFO | **FRD ↔ test traceability is excellent** — Every FR-XXX requirement has a corresponding `fr001_*` / `fr002_*` acceptance test function. The naming convention is explicit and consistent. | `crates/naming-rules/tests/` | No action needed — this is a positive finding. |
| 17  | 🟢 INFO | **FRD ↔ code traceability** — FR-001 (AES101) maps to `capabilities_naming_convention_checker.rs` + `acceptance_FRD_001.rs` + unit tests. FR-002 (AES102) maps to `capabilities_suffix_prefix_checker.rs` + `acceptance_FRD_002.rs`. | All files | No action needed — this is a positive finding. |
| 18  | 🟢 INFO | **Shared types are properly versioned** — Rule codes (`RULE_CODE_NAMING_CONVENTION`, `RULE_CODE_SUFFIX_PREFIX`) are constants in `taxonomy_naming_constant.rs`, making them discoverable and centralized. | `crates/shared/src/naming-rules/taxonomy_naming_constant.rs` | No action needed — this is a positive finding. |

### AES Architecture Compliance (Self-Detected)

| #   | Severity | Issue | Location | Recommendation |
| --- | -------- | ----- | -------- | -------------- |
| 19  | 🟢 INFO | **AES101 — Naming convention compliance** — All files in this crate follow the `layer_concern_role` naming pattern correctly: `agent_naming_orchestrator.rs`, `capabilities_naming_convention_checker.rs`, `capabilities_suffix_prefix_checker.rs`, `root_naming_rules_container.rs`. | `crates/naming-rules/src/` | No action needed — this is a positive finding. |
| 20  | 🟢 INFO | **AES102 — Suffix/prefix alignment** — All files have correct layer prefixes and role suffixes: agent_..._orchestrator, capabilities_..._checker, root_..._container. No violations detected. | `crates/naming-rules/src/` | No action needed — this is a positive finding. |
| 21  | 🟢 INFO | **AES201 — Forbidden import compliance** — Capabilities only import taxonomy, contract, config-system, and shared utilities. Agent imports contracts and shared types. No cross-layer violations detected. | All `src/*.rs` | No action needed — this is a positive finding. |
| 22  | 🟢 INFO | **AES403 — Capabilities implement protocol** — `NamingConventionChecker` implements `INamingConventionChecker`, `SuffixPrefixChecker` implements `ISuffixPrefixChecker`. Both protocols are consumed by the agent orchestrator. | `crates/naming-rules/src/` | No action needed — this is a positive finding. |

---

## Violations

### Minor (Non-Blocking)

1. **FRD API table is outdated** — Method signatures in the FRD don't match the actual capabilities implementation. This doesn't affect functionality but creates documentation debt.
2. **Benchmark test target conflict** — `bench_naming_rules_throughput.rs` is declared under both `[test]` and `[bench]` targets, causing Cargo warnings.
3. **Freestanding function in agent layer** — `filter_source_files()` is stateless and should be extracted to shared utility per project conventions.

---

## Action Items

- [ ] **MEDIUM** Update FRD API contract table to match actual method signatures (`check_file_naming`, `check_domain_suffixes`)
- [ ] **LOW** Extract `filter_source_files()` from agent layer to `shared::naming_rules::utility_file_filter.rs` (stateless per project feedback)
- [ ] **LOW** Resolve benchmark target conflict — move `bench_naming_rules_throughput.rs` to dedicated bench target
- [ ] **LOW** Add integration test that walks a mock project structure with mixed valid/invalid files end-to-end
- [ ] **INFO** Document measured benchmark throughput in FRD non-functional requirements section

---

## Gap Analysis Table

| Current State | Issue | Recommendation | Priority |
| ------------- | ----- | -------------- | -------- |
| FRD API table references non-existent methods | FRD lists `check_naming_convention()` / `check_layer_alignment()` but actual methods are `check_file_naming()` / `check_domain_suffixes()` | Update FRD with correct signatures | P2 |
| Benchmark test has duplicate Cargo.toml target | `bench_naming_rules_throughput.rs` declared under both `[test]` and `[bench]` targets | Split into dedicated bench directory | P3 |
| Agent layer contains stateless freestanding function | `filter_source_files()` in `agent_naming_orchestrator.rs` has no `&self` — should be in shared utility | Extract to `shared::naming_rules::utility_file_filter.rs` | P3 |
| No end-to-end integration test for full pipeline | Integration tests only cover container wiring, not real file trees | Add integration test with tempdir + mixed files | P3 |
| Benchmark throughput not measured in CI | FRD claims "1000 files in < 1s" but no CI gate enforces this | Add benchmark to CI with regression threshold | P3 |
