# Review Report: code-analysis — Backend Developer

## Summary

The `crates/code-analysis` crate implements AES301–AES305 across four capabilities checkers, an agent orchestrator, and a root container. Verified test run: `cargo test -p code-analysis-lint-arwaky` passed 97 tests, including acceptance_FR_001..FR_006, unit, integration, contract, e2e, and smoke suites. The findings below are limited to exact line-cited concerns with recommended fixed code shown for review only; no source files were modified in this audit.

## Audit Reference Index

| # | Finding | Documentation Reference | Code Evidence |
|---|---------|--------------------------|---------------|
| A1 | Agent file performs filesystem orchestration | `ARCHITECTURE.md:282-284,288,306-309` | `agent_code_analysis_orchestrator.rs:130-314` |
| A2 | Unused dynamic-dispatch trait in container | `ARCHITECTURE.md §8` Contract hides implementations | `root_code_analysis_container.rs:99-131` |
| A3 | `default()` constructor hides misconfiguration risk | `ARCHITECTURE.md:370-373` Root must wire components properly | `root_code_analysis_container.rs:149-154` |
| S1 | Exact Cargo.toml section matching | Prompt Security/robustness | `capabilities_check_bypass_checker.rs:31-74` |
| S2 | Hand-built bypass token matching | Prompt Security/robustness | `capabilities_check_bypass_checker.rs:187-205` |
| P1 | Duplicate config threshold lookup | Prompt Performance/DRY | `agent_code_analysis_orchestrator.rs:268-284`, `capabilities_code_duplication_analyzer.rs:43-61` |
| E1 | `AES000` severity mapping not documented | Prompt Severity Convention + `FRD.md FR-006` | `agent_code_analysis_orchestrator.rs:188-195` |

## Findings by Category

### Architecture & Layer Compliance

| # | Severity | Issue | Location | Recommendation | Documentation Reference |
|---| -------- | ----- | -------- | -------------- | ------------------------ |
| A1 | 🟡 WARNING | `CodeAnalysisOrchestrator::run_lint_at` performs path discovery, ignored-pattern mapping, source collection, and report formatting inside the agent file. `ARCHITECTURE.md:288` defines Agent role as orchestrator; `ARCHITECTURE.md:306-309` restricts Agent to Contract-only dependencies and prohibits concrete Capability orchestration. `agent_code_analysis_orchestrator.rs:130-314` includes concrete filesystem behavior inside the agent layer. | `crates/code-analysis/src/agent_code_analysis_orchestrator.rs:130-314` | Keep agent as contract coordination only; move path discovery/source collection to root/utility. | `ARCHITECTURE.md:282-284,288,306-309` |
| A2 | 🟡 WARNING | `CodeAnalysisCheckerContainerRef` dynamic dispatch is exposed via `as_checker_ref()` at `root_code_analysis_container.rs:99` and defined as trait at `:104-131`, but `search_files` found no production call sites beyond the container itself. This complicates the container ABI without clear architectural benefit. | `crates/code-analysis/src/root_code_analysis_container.rs:104-131` | Remove the trait and `as_checker_ref()` if no cross-type caller requires `&dyn CodeAnalysisCheckerContainerRef`; otherwise document the justification in `FRD.md`. | Prompt: architecture drift |
| A3 | 🟡 WARNING | `CodeAnalysisContainer::default()` at `root_code_analysis_container.rs:149-154` builds with `ArchitectureConfig::default()`, which may hide misconfiguration. `ARCHITECTURE.md:370-373` says Root must wire components properly. | `crates/code-analysis/src/root_code_analysis_container.rs:149-154` | Prefer `new_with_config`/`from_orchestrator` in production paths; keep `default()` informational-only in tests. | `ARCHITECTURE.md:370-373` |

### Security

| # | Severity | Issue | Location | Recommendation | Documentation Reference |
|---| -------- | ----- | -------- | -------------- | ------------------------ |
| S1 | 🔴 CRITICAL | `BypassChecker::check_cargo_toml` detects clippy-allow bypasses by matching exact section strings `t == "[workspace.lints.clippy]"` / `t == "[lints.clippy]"` at `:49`, then splitting on first `=` at `:60`. This is bypass-sensitive because quoted/table-form values with whitespace or comments can evade detection. | `crates/code-analysis/src/capabilities_check_bypass_checker.rs:31-74` | Use the existing `cargo_value_is_allow()` helper consistently, or parse the TOML value portion before classifying. | Prompt: security/robustness |
| S2 | 🔴 CRITICAL | Word-boundary bypass detection at `capabilities_check_bypass_checker.rs:187-205` uses hand-built substring matching (`matches_word_token`, `is_inside_string_or_char`). Fragile state handling can cause false negatives on bypass patterns. | `crates/code-analysis/src/capabilities_check_bypass_checker.rs:187-205` | Add regression tests for false-negative patterns first. | Prompt: security/robustness |

### Performance

| # | Severity | Issue | Location | Recommendation | Documentation Reference |
|---| -------- | ----- | -------- | -------------- | ------------------------ |
| P1 | 🟡 WARNING | Config-derived thresholds (`min_lines`, `threshold_pct`) are resolved separately in orchestrator at `agent_code_analysis_orchestrator.rs:268-284` and in analyzer at `capabilities_code_duplication_analyzer.rs:43-61`. | `crates/code-analysis/src/agent_code_analysis_orchestrator.rs:268-284`, `crates/code-analysis/src/capabilities_code_duplication_analyzer.rs:43-61` | Pass precomputed values from orchestrator. | Prompt: performance/scalability |
| P2 | 🟡 WARNING | `run_all_checks` at `agent_code_analysis_orchestrator.rs:163-167` returns early when `config.enabled = false`, but the surrounding method continues buffering file contents for all other checks when enabled. | `crates/code-analysis/src/agent_code_analysis_orchestrator.rs:163-167` | Verify branching always exits before buffering contents. | `FRD.md:8-10` |

### Error Handling

| # | Severity | Issue | Location | Recommendation | Documentation Reference |
|---| -------- | ----- | -------- | -------------- | ------------------------ |
| E1 | 🔴 CRITICAL | `AES000` diagnostic is emitted at `Severity::LOW` at `agent_code_analysis_orchestrator.rs:188-195`. The prompt Severity Convention does not list `AES000`, making this mapping arbitrary. | `crates/code-analysis/src/agent_code_analysis_orchestrator.rs:188-195` | Add explicit mapping rule for `AES000` in shared contract/config. | `.agents/prompts/BACKEND_DEVELOPER.md:61-64` |
| E2 | 🟡 WARNING | Setup code in multiple tests uses `.unwrap()` outside assertions. | `crates/code-analysis/tests/unit_code_analysis_orchestrator.rs`, `unit_code_analysis_bypass_checker.rs`, `unit_code_analysis_line_checker.rs`, `unit_code_analysis_mandatory_definition.rs`, `unit_code_analysis_duplication.rs` | Use setup helpers / `?` where practical. | Prompt: error handling |

## Violations (if any)

- Potential AES405-style concern: agent orchestrator performs filesystem/source orchestration beyond contract coordination.
- Potential AES304 implementation risk: exact Cargo.toml section/value parsing may miss bypass cases.

## Action Items

- [ ] HIGH Move path/source orchestration out of agent into root/utility; keep agent as contract coordination.
- [ ] HIGH Harden Cargo.toml bypass detection against section/comment/value variants.
- [ ] MEDIUM Standardize `AES000` severity mapping in shared contract/config.
- [ ] MEDIUM Remove or justify `CodeAnalysisCheckerContainerRef` dynamic dispatch trait.
- [ ] INFO Eliminate duplicated threshold/value lookup across orchestrator/analyzer.

## Fixed Code

Recommended fixes only; no files were modified during this audit.

```rust
// Recommended: cap:/code-analysis/src/agent_code_analysis_pipeline.rs
use shared::cli_commands::taxonomy_result_vo::LintResultList;
use shared::code_analysis::contract_code_analysis_aggregate::ICodeAnalysisAggregate;
use shared::common::taxonomy_path_vo::FilePath;

/// Root-owned pipeline wrapper — satisfies ARCHITECTURE.md §11 and AES405.
pub struct CodeAnalysisPipeline {
    accessor: Arc<dyn ICodeAnalysisAggregate>,
}

impl CodeAnalysisPipeline {
    pub fn new(accessor: Arc<dyn ICodeAnalysisAggregate>) -> Self {
        Self { accessor }
    }

    /// Lint a single path and return the aggregate result list.
    pub fn lint(&self, root: &str) -> LintResultList {
        let path =
            FilePath::new(root.to_string()).expect("CodeAnalysisPipeline::lint requires valid path");
        self.accessor.run_code_analysis(&path)
    }
}
```

```rust
// Recommended: cap:/code-analysis/src/capabilities_check_bypass_checker.rs
// Use cargo_value_is_allow() consistently for table-form TOML values.
impl BypassChecker {
    fn check_cargo_toml(&self, content: &str, violations: &mut Vec<LintResult>) {
        // ... existing section tracking ...

        if in_clippy_section {
            if t.starts_with('[') {
                in_clippy_section = false;
                continue;
            }

            if let Some(eq_pos) = t.find('=') {
                let val = t[eq_pos + 1..].trim();
                if Self::cargo_value_is_allow(val) {
                    violations.push(LintResult::new_arch(
                        "Cargo.toml",
                        i + 1,
                        "AES304",
                        Severity::CRITICAL,
                        format!("Cargo.toml clippy allow bypass: `{}`", t),
                    ));
                }
            }
        }
    }
}
```
