# Review Report: code-analysis — Backend Developer

## Summary

The `crates/code-analysis` backend follows the crate layout prescribed in `.agents/prompts/BACKEND_DEVELOPER.md` and `ARCHITECTURE.md`: checker implementations live in `capabilities_*`, orchestration lives in `agent_code_analysis_orchestrator.rs`, and wiring lives in `root_code_analysis_container.rs`. The main concerns are **agent-layer behavior that includes concrete technical orchestration**, **suspicious TODO/fragile parsing paths**, and **contract/test-helper divergence on rule severity/value mapping**.

## Findings by Category

### Architecture & Layer Compliance

| # | Severity | Issue | Location | Recommendation | Documentation Reference |
|---| -------- | ----- | -------- | -------------- | ------------------------ |
| A1 | 🟡 WARNING | `CodeAnalysisOrchestrator::run_lint_at` performs path discovery, ignored-pattern mapping, source collection, and report formatting inside the agent file. `ARCHITECTURE.md:288` says the Agent role is orchestrator; `ARCHITECTURE.md:306-309` says Agent must depend on Contract, not concrete Capabilities implementations. `agent_code_analysis_orchestrator.rs:130-314` includes concrete filesystem behavior inside the agent layer. | `crates/code-analysis/src/agent_code_analysis_orchestrator.rs:130-314` | Keep agent as contract coordination only; move path discovery/source collection to root/utility. | `ARCHITECTURE.md:288,306-309` |
| A2 | 🟡 WARNING | `CodeAnalysisCheckerContainerRef` dynamic dispatch is used here without clear caller need; `root_code_analysis_container.rs:99-131` wraps the container in a trait and casts back to `&dyn ...`. | `crates/code-analysis/src/root_code_analysis_container.rs:104-131` | Remove trait unless a cross-type caller requires `&dyn`. | Prompt: architecture drift |
| A3 | 🟡 WARNING | `CodeAnalysisContainer::default()` builds with default config, which may hide misconfiguration. | `crates/code-analysis/src/root_code_analysis_container.rs:149-154` | Prefer `new_with_config`/`from_orchestrator`; keep `default()` informational-only. | `ARCHITECTURE.md:370-373` |

### Security

| # | Severity | Issue | Location | Recommendation | Documentation Reference |
|---| -------- | ----- | -------- | -------------- | ------------------------ |
| S1 | 🔴 CRITICAL | `BypassChecker::check_cargo_toml` detects only exact section tokens, `t == "[workspace.lints.clippy]"` / `t == "[lints.clippy]"`, then parses Key=Value by naive split. This is bypass-sensitive because TOML variation can evade detection. | `crates/code-analysis/src/capabilities_check_bypass_checker.rs:31-74` | Replace exact matching with parsed/lexed TOML sections or broaden section detection before equating values. | Prompt: security / robustness |
| S2 | 🔴 CRITICAL | Word-boundary bypass logic uses hand-built substring/state checks. Fragile comment/skip handling can cause false negatives on bypass patterns. | `crates/code-analysis/src/capabilities_check_bypass_checker.rs:187-205` | Add regression tests for false-negative patterns first. | Prompt: security / robustness |

### Performance

| # | Severity | Issue | Location | Recommendation | Documentation Reference |
|---| -------- | ----- | -------- | -------------- | ------------------------ |
| P1 | 🟡 WARNING | Duplication-related config values (`min_lines`, `threshold_pct`) are resolved separately in orchestrator and analyzer. | `crates/code-analysis/src/agent_code_analysis_orchestrator.rs:268-284`, `crates/code-analysis/src/capabilities_code_duplication_analyzer.rs:43-61` | Pass precomputed values from orchestrator. | Prompt: performance/scalability |
| P2 | 🟡 WARNING | `run_all_checks` reads all file contents even though it returns early when `config.enabled = false`. | `crates/code-analysis/src/agent_code_analysis_orchestrator.rs:163-167` | Verify branching always exits before buffering contents. | `FRD.md:8-10` |

### Error Handling

| # | Severity | Issue | Location | Recommendation | Documentation Reference |
|---| -------- | ----- | -------- | -------------- | ------------------------ |
| E1 | 🔴 CRITICAL | `AES000` diagnostic is emitted at `Severity::LOW`. The prompt Severity Convention does not include `AES000`, so this is inconsistent. | `crates/code-analysis/src/agent_code_analysis_orchestrator.rs:188-195` | Add explicit mapping rule for `AES000`. | `.agents/prompts/BACKEND_DEVELOPER.md:61-64` |
| E2 | 🟡 WARNING | Setup code in tests uses `.unwrap()` outside assertions. | `crates/code-analysis/tests/unit_code_analysis_orchestrator.rs`, `unit_code_analysis_bypass_checker.rs`, `unit_code_analysis_line_checker.rs`, `unit_code_analysis_mandatory_definition.rs`, `unit_code_analysis_duplication.rs` | Use setup helpers / `?` where practical. | Prompt: error handling |

## Violations (if any)

- Potential AES405-style concern: agent orchestrator performs filesystem/source orchestration beyond contract coordination.
- Potential AES304 implementation risk: naive Cargo.toml section/value parsing may miss bypass cases.

## Action Items

- [ ] HIGH Move path/source orchestration out of agent into root/utility; keep agent as contract coordination.
- [ ] HIGH Harden Cargo.toml bypass detection against section/comment/value variants.
- [ ] MEDIUM Standardize `AES000` severity mapping in shared contract/config.
- [ ] MEDIUM Remove or justify `CodeAnalysisCheckerContainerRef` dynamic dispatch trait.
- [ ] INFO Eliminate duplicated threshold/value lookup across orchestrator/analyzer.

## Fixed Code

Relevant guidance, keeping current interfaces:

```rust
// cap:/code-analysis/src/capabilities_check_bypass_checker.rs
impl BypassChecker {
    fn cargo_level_is_bypass(value: &str) -> bool {
        let compact = value.split_once('#').map(|(code, _)| code).unwrap_or(value);
        let normalized = compact.trim().trim_matches(|c| c == '"' || c == '\'').to_ascii_lowercase();
        matches!(normalized.as_str(), "allow")
    }
}
```
