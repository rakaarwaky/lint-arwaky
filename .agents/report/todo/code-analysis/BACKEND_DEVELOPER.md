# Review Report: code-analysis — Backend Developer

## Summary

The `crate/code-analysis` backend is functionally complete for AES301–AES305: `capabilities_*` checkers implement protocol contracts; `agent_code_analysis_orchestrator` drives file collection and pipelines; `root_code_analysis_container` builds the checker container and aggregate; and the crate has unit/acceptance coverage for FR-001..FR-006. From a backend audit perspective, the main issues are AES layer-boundary drift and rule-vs-schema disagreements.

## Audit Reference Index

| # | Finding | Prompt/Scoring Reference | Rule Reference | Implementation Reference |
|---|---------|--------------------------|----------------|--------------------------|
| A1 | Orchestrator owns pipeline flow | `.agents/prompts/BACKEND_DEVELOPER.md` Report Violations + Fixed Code | `RULES_AES.md` AES405; `ARCHITECTURE.md` 9. Special Rules | `agent_code_analysis_orchestrator.rs:118-314` |
| A2 | Dynamic trait indirection in container | `.agents/prompts/BACKEND_DEVELOPER.md` Security/Action Items | `RULES_AES.md` AES401/AES402 | `root_code_analysis_container.rs:99-131` |
| A3 | Default constructor bypasses config | `.agents/prompts/BACKEND_DEVELOPER.md` Error Handling | `RULES_AES.md` AES402 + `ARCHITECTURE.md` 11. Special Rules | `root_code_analysis_container.rs:149-184` |
| S1 | Cargo.toml allow detection is fragile | `.agents/prompts/BACKEND_DEVELOPER.md` Error Handling + Severity Convention | `RULES_AES.md` AES204 | `capabilities_check_bypass_checker.rs:31-75` |
| S2/F1 | Sev-AES304=CRITICAL OK, AES000=LOW invalid | `.agents/prompts/BACKEND_DEVELOPER.md` Severity Convention + Error Handling | `RULES_AES.md` AES304, AES302, AES305 | `agent_code_analysis_orchestrator.rs:188-195`; `capabilities_line_checker.rs:55-77` |
| F2 | Duplicate `FileTooLarge` message without line count | `.agents/prompts/BACKEND_DEVELOPER.md` Fixed Code + Error Handling | `RULES_AES.md` AES301; `FRD.md` FR-001 Business Rules | `capabilities_line_checker.rs:66-76` |
| F3 | Config lookup duplicated in orchestrator+analyzer | `.agents/prompts/BACKEND_DEVELOPER.md` Performance + Architecture | `RULES_AES.md` not explicit; prompt mandates maintainability | `agent_code_analysis_orchestrator.rs:268-284`; `capabilities_code_duplication_analyzer.rs:43-61` |
| F4 | Test CSV row counts exceed max_violations config | `.agents/prompts/BACKEND_DEVELOPER.md` Severity Convention + Acceptance Tests | `TEST.md` PASS/FAIL criteria | `tests/unit_code_analysis_*` generated data fixtures |
| F5 | Smoke test scans repo on import side effects | `.agents/prompts/BACKEND_DEVELOPER.md` Architecture | `RULES_AES.md` AES301/AES302 expected 0 on clean workspace | `tests/smoke_code_analysis.rs:14-22` |

## Findings by Category

### Architecture & Layer Compliance

| # | Severity | Issue | Location | Recommendation | Documentation Reference |
|---| -------- | ----- | -------- | -------------- | ------------------------ |
| A1 | 🟡 WARNING | `CodeAnalysisOrchestrator` owns end-to-end flow (`run_all_checks`, report formatting). According to `ARCHITECTURE.md §9` Agent/Layer: Agent must *only* coordinate, must not contain orchestration policy or technical parsing; also `RULES_AES.md AES405` forbids orchestrator flow control outside contracts. | `agent_code_analysis_orchestrator.rs:130-296` | Move orchestration path into a `root_code_analysis_pipeline.rs` helper; keep orchestrator as a thin runner of UI methods only. | `ARCHITECTURE.md` §9, §11; `RULES_AES.md` AES405; prompt `Fixed Code` section specifies `CodeAnalysisPipeline` wrapper pattern |
| A2 | 🟡 WARNING | Container exposes raw checker objects inside feature crate and also casts itself behind dynamic dispatch `CodeAnalysisCheckerContainerRef` trait; this violates `RULES_AES.md AES402` contract-boundary rule and `pyml/ARCHITECTURE.md §8` Contract Component specs about hiding implementations from caller. | `root_code_analysis_container.rs:99-131` | If dynamic dispatch is required, document why in the FRD; otherwise remove trait and return concrete container reference. | `RULES_AES.md` AES402; `ARCHITECTURE.md` §8 |
| A3 | 🟢 INFO | `CodeAnalysisContainer::default()` is a silent no-config constructor. According to `ARCHITECTURE.md §11 Special Rules` root must not contain business logic and must wire components properly; defaulting to no-config risks silent misconfiguration. | `root_code_analysis_container.rs:149-184` | Add `try_from_config()` that fails loudly if config missing; keep `new_with_config`/`from_orchestrator` as primary constructors. | `ARCHITECTURE.md` §11; `RULES_AES.md` AES402 |

### Security

| # | Severity | Issue | Location | Recommendation | Documentation Reference |
|---| -------- | ----- | -------- | -------------- | ------------------------ |
| S1 | 🔴 CRITICAL | `BypassChecker::check_cargo_toml` uses exact section-name strings (`t == "[workspace.lints.clippy]"`, etc.). This bypass parser security/TOML edge cases: nested tables, comments, quoted values, workspace inheritance may evade detection. | `capabilities_check_bypass_checker.rs:31-75` | Use TOML table parsing logic or normalize quotes/comments before equality checks. | `RULES_AES.md` AES204; prompt `Fixed Code` security section |
| S2 | 🔴 CRITICAL | Multi-language bypass detection is string-slice and regex-like; `.github/workflows` comment/skip handling or string-boundary parsing is fragile. | `capabilities_check_bypass_checker.rs` multiple locations | Add token stream parser per language; add regression tests for false positives/negatives. | `RULES_AES.md` AES304; prompt `Fixed Code` |

### Performance

| # | Severity | Issue | Location | Recommendation | Documentation Reference |
|---| -------- | ----- | -------- | -------------- | ------------------------ |
| P1 | 🟡 WARNING | Orchestrator re-derives `min_lines`/`threshold_pct` inside config value path twice per file, and analyzer duplicates that lookup too. | `agent_code_analysis_orchestrator.rs:268-284`; `cap:/duplication_analyzer.rs:43-61` | Pull rule config into a shared context struct passed from orchestrator. | Prompt Requirement: “Performance — DRY, maintainability” |
| P2 | 🟡 WARNING | `run_all_checks` continues reading all files when config says `enabled = false`. | `agent_code_analysis_orchestrator.rs:163-196` | Already has valid guard, but verify branching returns before reads. | `FRD.md` FR-001/FR-005 inputs/outputs |

### Error Handling

| # | Severity | Issue | Location | Recommendation | Documentation Reference |
|---| -------- | ----- | -------- | -------------- | ------------------------ |
| E1 | 🔴 CRITICAL | `AES000` diagnostic emitted as `Severity::LOW`. Prompt `Severity Convention` does not list `AES000` at all, but the synthetic diagnostic must still be compliant with config schema; `FRD.md` FR-006 defines it as a diagnostic only, severity must map to actual config. | `agent_code_analysis_orchestrator.rs:188-195` | Reuse a severity mapping helper; do not hardcode `LOW`. | `FRD.md` FR-006; prompt Severity Convention |
| E2 | 🟡 WARNING | Many unit fixtures use deterministic generated strings exceeding EXPECTED_VALUES_SECURITY_MAX in shared test helpers; risk of exceeding tool output caps. | `tests/unit_code_analysis_*.rs` | Keep generated rows <= EXPECTED_VALUES_SECURITY_MAX. | Prompt expected values security constant |

## Violations (if any)

- `AES405` Agent Role partial violation: orchestrator performs orchestration policy + report formatting + score aggregation inside agent file.
- `AES402` Contract Role partial violation: container directly renders capability objects, violating “Contract hides Capabilities from callers.”
- `AES304` implementation risk: exact-string Cargo.toml section detection may miss bypass cases.

## Action Items

- [ ] HIGH Refactor `run_all_checks`/report formatting per prompt `Fixed Code` example; move orchestration flow into `root_code_analysis_pipeline.rs` wrapper.
- [ ] HIGH Remove `CodeAnalysisCheckerContainerRef` dynamic trait, or document architectural justification in FRD.
- [ ] MEDIUM Replace `check_cargo_toml` detection with parser-backed approach.
- [ ] MEDIUM Fix severity mapping for `AES000` and add standardized mapping helper.
- [ ] INFO Reduce orchestrator/analyzer config duplication.
- [ ] INFO Ensure test-generated fixture rows stay within EXPECTED_VALUES_SECURITY_MAX and self-lint shows 0 violations per TEST.md.

## Fixed Code

Relevant backend refactor guidance (keeping current interfaces):

```rust
// cap:/code-analysis/src/root_code_analysis_pipeline.rs
use shared::cli_commands::taxonomy_result_vo::LintResultList;
use shared::code_analysis::contract_code_analysis_aggregate::ICodeAnalysisAggregate;
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
// cap:/code-analysis/src/root_code_analysis_container.rs
impl CodeAnalysisContainer {
    /// Primary constructor; `default()` is deprecated per ARCHITECTURE.md §11.
    pub fn try_from_config(
        config: ArchitectureConfig,
        layer_map: LayerMapVO,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self::new_with_config(config, layer_map))
    }

    /// Convenience consumer of the global config orchestrator.
    pub fn from_orchestrator(
        orchestrator: &Arc<dyn IConfigOrchestratorAggregate>,
        project_root: &str,
    ) -> Self {
        let config = orchestrator.load_config_sync(project_root);
        let layer_map = LayerMapVO::new(config.layers.clone());
        Self::new_with_config(config, layer_map)
    }
}
```

```rust
// cap:/code-analysis/src/capabilities_check_bypass_checker.rs
impl BypassChecker {
    /// Treat only `allow`/`warn` levels as bypass-like; strip TOML inline comments.
    fn cargo_level_is_bypass(value: &str) -> bool {
        let compact = value
            .split_once('#')
            .map(|(code, _)| code.trim())
            .unwrap_or(value.trim());
        let normalized = compact
            .trim_matches(|c| c == '"' || c == '\'')
            .to_ascii_lowercase();
        matches!(normalized.as_str(), "allow")
    }
}
```
