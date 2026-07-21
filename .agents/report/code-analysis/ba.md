# Business Analyst Review: `code-analysis` Crate (v1.10.106)

## Executive Summary

The `code-analysis` crate implements a code-quality enforcement system (AES301–AES305) within the `lint-arwaky` workspace. While the Feature Requirements Document (FRD) establishes a clear strategic intent, the requirements documentation exhibits **significant gaps in testability, traceability, and edge-case specification** that increase implementation risk and reduce stakeholder confidence. Below is a structured assessment.

---

## 1. Requirement Clarity Assessment

### 1.1 Well-Defined Requirements ✅

| Rule                         | Clarity Rating     | Notes                                                                                  |
| ---------------------------- | ------------------ | -------------------------------------------------------------------------------------- |
| AES301 (Max File Line Count) | **Good**     | Clear threshold concept; configurable via YAML                                         |
| AES304 (Bypass Detection)    | **Good**     | Explicit enumeration of forbidden patterns (noqa, type: ignore, unwrap, expect, panic) |
| AES305 (Duplicate Code)      | **Moderate** | Algorithm described but threshold semantics ambiguous                                  |

### 1.2 Ambiguities Identified ⚠️

| ID               | Requirement                                                                              | Ambiguity                                                                                                                                                                                                      | Impact                                                                                 |
| ---------------- | ---------------------------------------------------------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | -------------------------------------------------------------------------------------- |
| **AMB-01** | AES301:*"500–800 lines depending on language/layer"*                                  | No explicit mapping table between language/layer → threshold. The FRD says "configured via YAML" but provides no default matrix.                                                                              | Developers cannot validate correct behavior without inspecting YAML configs.           |
| **AMB-02** | AES302:*"at least 10 lines"*                                                           | The parenthetical "(e.g., at least 10 lines)" uses "e.g." — is 10 the actual default or merely illustrative? Implementation uses`def.code_analysis.min_lines.value` which defaults to 0 (disabled).         | Contradiction between FRD narrative and implementation default.                        |
| **AMB-03** | AES305:*"identical/highly similar code segments"*                                      | "Highly similar" is undefined. Implementation uses normalized alphanumeric-only comparison (strips all punctuation/operators), which is a specific interpretation not stated in the FRD.                       | Normalization strategy is a critical business decision buried in code.                 |
| **AMB-04** | AES305:*"exceeding a configurable token/line threshold"*                               | FRD mentions "token/line threshold" but implementation uses a**sliding window of lines** with a **percentage-based file-level threshold**. These are fundamentally different detection strategies. | Stakeholders expecting token-level AST matching will receive line-level text matching. |
| **AMB-05** | AES303:*"at least one primary symbol (e.g., struct, enum, class, or interface/trait)"* | The word "primary" is undefined. Does a`type` alias count? A `const`? Implementation checks only `struct`, `enum`, `trait`, and `class` keywords.                                                  | Edge cases (type aliases, const generics) are unspecified.                             |
| **AMB-06** | AES304 Req 3:*"Flags fatal operations... such as .unwrap(), .expect(), and panic!"*    | The word "such as" implies a non-exhaustive list. Implementation also flags`todo!()`, `unimplemented!()`, and `unreachable!()` — none mentioned in the FRD.                                             | Scope creep without documented stakeholder approval.                                   |

---

## 2. Completeness Assessment

### 2.1 Missing Requirements

| Gap ID           | Missing Element                                                                                                                                                                                                                                                                  | Business Risk                                      |
| ---------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | -------------------------------------------------- |
| **GAP-01** | **No acceptance criteria per rule.** Success Indicators are checkbox-style but lack measurable pass/fail conditions (e.g., "Given a 501-line Rust file in the capabilities layer, When scanned, Then exactly 1 AES301 violation is reported at line 0").                   | QA cannot write deterministic test cases.          |
| **GAP-02** | **No error-handling requirements.** What happens when a file is unreadable? When YAML config is malformed? When a directory contains 100,000 files? Implementation silently skips unreadable files (`Err(_) => continue`) — this is a business decision not documented. | Silent failures erode user trust.                  |
| **GAP-03** | **No performance requirements.** AES305 performs O(n²) cross-file comparison with sliding windows. No SLA for scan time (e.g., "< 5s for 1,000 files").                                                                                                                   | Risk of unusable tool on large codebases.          |
| **GAP-04** | **No multi-language parity matrix.** FRD mentions Rust, Python, JS/TS but doesn't specify which rules apply to which languages. Implementation shows Python gets `raise NotImplementedError` checks, JS gets `throw new Error` checks — undocumented.                 | Inconsistent enforcement across polyglot teams.    |
| **GAP-05** | **No versioning/migration requirements.** What happens to existing violations when thresholds change? Is there a baseline/suppression mechanism for legacy code?                                                                                                           | Adoption barrier for existing projects.            |
| **GAP-06** | **No reporting format requirements.** FRD doesn't specify output formats (JSON, SARIF, table). Implementation has `format_report()` producing plain text only.                                                                                                           | Integration with CI/CD pipelines unclear.          |
| **GAP-07** | **No severity assignment rationale.** AES301/302/303 = HIGH, AES304 = CRITICAL, AES305 = HIGH. No documented justification for why bypass is CRITICAL but file size is HIGH.                                                                                               | Stakeholders may dispute severity classifications. |

### 2.2 Incomplete Traceability

```
FRD Requirement → Implementation Mapping (Partial)
─────────────────────────────────────────────────────
AES301 → capabilities_line_checker.rs (check_line_counts)     ✅ Traced
AES302 → capabilities_line_checker.rs (check_line_counts)     ✅ Traced
AES303 → capabilities_mandatory_definition_checker.rs         ✅ Traced
AES304 → capabilities_check_bypass_checker.rs                 ✅ Traced
AES305 → capabilities_code_duplication_analyzer.rs            ⚠️ Partial
         (FRD says "token/line threshold" but impl uses
          percentage-of-windows — semantic mismatch)
```

**Missing:** No requirement IDs embedded in code comments or test names. No traceability matrix document.

---

## 3. Testability Assessment

### 3.1 Current State: **POOR**

The FRD's Success Indicators are **not testable** in their current form:

| Indicator                                                            | Problem                                                                                                                             |
| -------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------- |
| *"Prevention of suppression hacks — absolute blockage"*           | "Absolute" is unfalsifiable. What about false positives in string literals containing "unwrap"?                                     |
| *"Code size discipline — strict enforcement of LOC limits"*       | No specific test scenario. What is "strict"? ±0 tolerance?                                                                         |
| *"DRY codebase — high-performance detection"*                     | "High-performance" has no benchmark. "Without lagging execution speeds" is subjective.                                              |
| *"Granular location info — reporting of line and column numbers"* | Implementation reports`column: 0` for all violations (see `LintResult::new_arch`). **This indicator is currently FALSE.** |
| *"Compliance — workspace self-check passes fully"*                | Circular — the tool checking itself is not an external validation.                                                                 |

### 3.2 Recommended Testable Criteria (Example for AES304)

```gherkin
Scenario: Detect .unwrap() call in non-test Rust file
  Given a file "capabilities_foo_checker.rs" containing:
    """
    let value = some_option.unwrap();
    """
  And the file is NOT inside a #[cfg(test)] block
  When the bypass checker scans the file
  Then exactly 1 violation is reported
  And violation.code == "AES304"
  And violation.severity == CRITICAL
  And violation.line == 1
  And violation.message contains "UNWRAP_EXPECT"

Scenario: Ignore .unwrap_or_default() (safe variant)
  Given a file containing "let x = opt.unwrap_or_default();"
  When the bypass checker scans the file
  Then 0 violations are reported
```

---

## 4. Conflicting Requirements

| Conflict ID      | Description                                                                                                                                                                                                                                                                                                                                                              | Resolution Needed                                                           |
| ---------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ | --------------------------------------------------------------------------- |
| **CON-01** | FRD AES302 says "at least 10 lines" but`CodeAnalysisRuleVO::default()` sets `min_lines: Count::default()` which is `0` (disabled).                                                                                                                                                                                                                                 | Clarify: Is AES302 enabled by default or opt-in?                            |
| **CON-02** | ARCHITECTURE.md §7 states*"Utility must not contain stateful objects"* but `utility_bypass.rs` uses `static PREFIXES: OnceLock<[String; 2]>` — a stateful lazy-initialized global.                                                                                                                                                                               | Either relax the architecture rule for lazy statics or refactor to a const. |
| **CON-03** | ARCHITECTURE.md §8 states*"Capabilities must not depend on or import other Capabilities"* but `root_code_analysis_container.rs` (Root layer) wires `MandatoryDefinitionChecker` as both `IDeadInheritanceProtocol` AND `IMandatoryClassProtocol`. This is correct architecturally, but the FRD doesn't document that one Capability implements two protocols. | Document multi-protocol implementations explicitly.                         |
| **CON-04** | FRD AES305 says*"Compares code blocks across files"* (implying block-level) but implementation compares **entire files** via sliding window percentage. The legacy `check_duplicates` method does block-level but is marked "prefer check_file_similarity."                                                                                                    | Update FRD to reflect the actual file-level similarity strategy.            |
| **CON-05** | The lint report flags`[clippy::collapsible_match]` in `capabilities_check_bypass_checker.rs`, but AES304 forbids `#[allow(...)]` annotations. The team cannot suppress this clippy warning without violating their own rule.                                                                                                                                       | Define an exception policy for self-referential lint violations.            |

---

## 5. Business Value Alignment

### 5.1 Value Proposition Assessment

| Business Goal                       | Alignment          | Evidence                                      |
| ----------------------------------- | ------------------ | --------------------------------------------- |
| Prevent technical debt accumulation | **Strong**   | AES301/302 enforce file size discipline       |
| Eliminate runtime crash vectors     | **Strong**   | AES304 blocks unwrap/panic/todo               |
| Enforce DRY principle               | **Moderate** | AES305 exists but threshold tuning is unclear |
| Support multi-language teams        | **Weak**     | No documented language parity matrix          |
| Enable CI/CD integration            | **Weak**     | No output format spec, no exit-code contract  |

### 5.2 Stakeholder Impact Matrix

| Stakeholder             | Primary Concern                 | Documentation Gap                                                  |
| ----------------------- | ------------------------------- | ------------------------------------------------------------------ |
| **Developers**    | "What exactly will be flagged?" | Pattern list is config-driven but no default list published in FRD |
| **Tech Leads**    | "Can I tune this per-team?"     | YAML config exists but no configuration guide                      |
| **QA Engineers**  | "How do I verify correctness?"  | No acceptance criteria, no test scenarios                          |
| **Product Owner** | "What's the ROI?"               | No metrics on violation reduction over time                        |
| **DevOps**        | "How does this integrate?"      | No CLI contract, no exit codes, no output format spec              |

---

## 6. Recommendations

### Priority 1 — Critical (Blocks Release Confidence)

| #            | Recommendation                                                                                                                                        | Rationale                                                             |
| ------------ | ----------------------------------------------------------------------------------------------------------------------------------------------------- | --------------------------------------------------------------------- |
| **R1** | Add a**Traceability Matrix** mapping each AES rule → source file → test case → config parameter.                                             | Eliminates ambiguity about what implements what.                      |
| **R2** | Replace "Success Indicators" with**Gherkin-style acceptance criteria** (Given/When/Then) for each AES rule.                                     | Makes requirements testable and removes subjective language.          |
| **R3** | Resolve**CON-01** (AES302 default) and **CON-04** (block vs. file-level duplication) by updating the FRD to match implementation reality. | Prevents stakeholder surprise during UAT.                             |
| **R4** | Document the**default forbidden_bypass pattern list** explicitly in the FRD (currently only in YAML config).                                    | Developers need to know what's enforced without reading config files. |

### Priority 2 — High (Reduces Rework Risk)

| #            | Recommendation                                                                                                                 | Rationale                                                                        |
| ------------ | ------------------------------------------------------------------------------------------------------------------------------ | -------------------------------------------------------------------------------- |
| **R5** | Add a**Language Parity Matrix** specifying which rules/patterns apply per language (Rust, Python, JS/TS).                | Prevents "why does Python get X but Rust doesn't?" disputes.                     |
| **R6** | Define**performance SLAs** for AES305 (e.g., "≤ 10s for 500 files, ≤ 60s for 5,000 files").                            | The O(n²) sliding-window algorithm needs bounded expectations.                  |
| **R7** | Specify**error-handling behavior** as a requirement: What is reported when files are unreadable? When config is invalid? | Silent`continue` on `Err(_)` is a business decision that should be explicit. |
| **R8** | Add a**Severity Rationale** section justifying CRITICAL vs. HIGH assignments.                                            | Prevents severity inflation/deflation debates.                                   |

### Priority 3 — Medium (Improves Maintainability)

| #             | Recommendation                                                                                                                                                                                | Rationale                                                               |
| ------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ----------------------------------------------------------------------- |
| **R9**  | Introduce**requirement IDs in code comments** (e.g., `// REQ: AES304-R2`) at each enforcement point.                                                                                  | Enables automated traceability checking.                                |
| **R10** | Document the**normalization strategy** for AES305 as a business rule: "Comparison strips all non-alphanumeric characters to detect structurally similar code regardless of formatting." | This is a critical design decision currently invisible to stakeholders. |
| **R11** | Add a**Configuration Guide** as a companion document showing all YAML knobs with examples.                                                                                              | Reduces support burden and empowers team leads.                         |
| **R12** | Define a**baseline/suppression mechanism** for legacy codebases adopting the tool.                                                                                                      | Without this, adoption friction will be high.                           |

### Priority 4 — Low (Polish)

| #             | Recommendation                                                                                                    | Rationale                                                        |
| ------------- | ----------------------------------------------------------------------------------------------------------------- | ---------------------------------------------------------------- |
| **R13** | Fix the**column number reporting** (currently always 0) or remove "column numbers" from Success Indicators. | The FRD promises granularity the implementation doesn't deliver. |
| **R14** | Add**sequence diagrams** for the orchestration flow (Agent → Capabilities → Utility).                     | Aids onboarding and architectural review.                        |
| **R15** | Version the FRD alongside the crate (currently FRD has no version/date).                                          | Enables change tracking and audit.                               |

---

## 7. Summary Scorecard

| Dimension                    | Score          | Notes                                                                     |
| ---------------------------- | -------------- | ------------------------------------------------------------------------- |
| **Clarity**            | 6/10           | Core intent clear; edge cases and defaults ambiguous                      |
| **Completeness**       | 5/10           | Missing acceptance criteria, error handling, performance, language matrix |
| **Testability**        | 3/10           | Success indicators are subjective; no Gherkin scenarios                   |
| **Traceability**       | 4/10           | Code maps to rules but no formal matrix; no req IDs in code               |
| **Consistency**        | 5/10           | 5 conflicts identified between FRD, architecture, and implementation      |
| **Business Alignment** | 7/10           | Strong technical value; weak CI/CD and multi-language story               |
| **Overall Maturity**   | **5/10** | Functional but not stakeholder-ready for enterprise adoption              |

---

## 8. Immediate Next Steps

1. **Schedule a requirements review workshop** with Tech Lead + QA + DevOps to resolve CON-01 through CON-05.
2. **Write acceptance criteria** for AES304 (highest severity, most complex logic) as a template for other rules.
3. **Publish the default pattern list** and language parity matrix as an appendix to the FRD.
4. **Create a traceability matrix** spreadsheet linking AES codes → files → tests → config keys.
5. **Resolve the clippy self-violation** (CON-05) by either fixing the collapsible match or documenting an explicit architectural exception.

---

*Prepared by: Business Analysis Review | Date: 2026-07-21 | Document Version: code-analysis v1.10.106*
