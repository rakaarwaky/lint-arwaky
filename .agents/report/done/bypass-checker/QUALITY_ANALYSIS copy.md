QC result: **not clean yet**. The 7 fixes you listed are mostly correctly applied, but the updated module still has **critical bypass-detection gaps** and **block-comment handling bugs**.

---

# Review Report: bypass-checker — Quality Analyst

## Summary

The second QC pass confirms that the original line-number, Cargo.toml comment, `usize::MAX`, safe-unwrap, ASCII lowercase, `cfg(test)`, and initial block-comment fixes are present. However, the module is still not production-ready. The most serious remaining problem is that full-line comment bypasses such as `// eslint-disable`, `// @ts-ignore`, and `/* eslint-disable */` are still skipped before non-word bypass patterns are checked. The new block-comment tracker also skips entire lines when a block comment opens or closes on the same line, causing missed violations in code after the comment. There are also remaining AES hygiene issues: duplicated word-token constants, magic strings, and low-level comment parsers that should be moved to Utility.

## QC Verification of Applied Fixes

| #  | Claimed Fix                             | Status               | Note                                                                                                                                                                          |
| -- | --------------------------------------- | -------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| 1  | Stable`line_number` before increment  | ✅ Verified          | Language-scoped violations now use the correct line number.                                                                                                                   |
| 4  | Cargo.toml comment stripping            | ⚠️ Partially fixed | Comments are stripped, but`cargo_value_is_allow()` is too broad and can false-positive on unrelated quoted `"allow"` strings inside inline tables.                        |
| 5  | Avoid`usize::MAX` in string checks    | ✅ Verified          | Pattern position now uses`match`/`continue`.                                                                                                                              |
| 7  | Safe unwrap variants restricted         | ✅ Verified          | Only`or(`, `or_else(`, `or_default(` are treated as safe.                                                                                                               |
| 8  | ASCII lowercase for stable byte offsets | ✅ Verified          | `to_ascii_lowercase()` is used consistently.                                                                                                                                |
| 9  | `cfg(test)` variant detection         | ⚠️ Partially fixed | `cfg(all(...))` support exists, but current logic can still false-positive on `not(test)` because it only checks `starts_with("#[cfg(all(")` plus `contains("test")`. |
| 10 | Block-comment state tracking            | ❌ Incorrect         | The state tracker skips whole lines when a block comment opens/closes on the same line and does not handle mid-line block comments correctly.                                 |

## Findings by Category

### Functional Bugs

| #                                                                                                                                                                                                                                                 | Severity    | Issue                                                                                                                                                                                                                                     | Location                                              | Recommendation                                                                                                                                                                                                |
| ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ----------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ----------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| 1                                                                                                                                                                                                                                                 | 🔴 CRITICAL | Full-line comment bypasses are still missed. Lines starting with`//`, `/*`, or `*` are skipped before non-word bypass patterns are checked, so `// eslint-disable`, `// @ts-ignore`, `/* eslint-disable */` are not reported. | `check_bypass_comments`, early `continue` blocks  | Do not skip comment lines completely. Skip only code-only checks such as word tokens, attributes, and language phrase patterns. Non-word bypass-comment patterns must still be checked against the full line. |
| 2                                                                                                                                                                                                                                                 | 🔴 CRITICAL | Block-comment handling skips code after comment close. If a line is`/* comment */ unwrap();`, the current logic continues and never checks `unwrap();`. If a block comment closes mid-line, code after `*/` is ignored.             | `check_bypass_comments`, `in_block_comment` block | Replace the boolean-only check with a stateful comment stripper that returns the code portion outside comments while preserving code after`*/`.                                                             |
| 3                                                                                                                                                                                                                                                 | 🔴 CRITICAL | Mid-line block comments are not tracked. Example:`let x = 1; /* bypass */ unwrap();` or `foo(); /* eslint-disable */` are not handled correctly.                                                                                      | `check_bypass_comments`                             | Use a scanner that detects`/*` and `*/` anywhere in the line, outside string literals.                                                                                                                    |
| 4                                                                                                                                                                                                                                                 | 🟡 WARNING  | `cargo_value_is_allow()` can false-positive on inline tables containing unrelated `"allow"` strings, e.g. `{ level = "warn", note = "allow" }`.                                                                                     | `BypassChecker::cargo_value_is_allow`               | Restrict inline-table detection to`level = "allow"` or `level = 'allow'`.                                                                                                                                 |
| 5                                                                                                                                                                                                                                                 | 🟡 WARNING  | `is_cfg_test_block()` can false-positive on `#[cfg(all(not(test), ...))]` because it checks `starts_with("#[cfg(all(")` and `contains("test")`.                                                                                   | `BypassChecker::is_cfg_test                           |                                                                                                                                                                                                               |
| QC result:**not clean yet**. The 7 fixes you listed are mostly correctly applied, but the updated module still has **critical bypass-detection gaps** and **block-comment handling bugs**. Save the updated QC report below to: |             |                                                                                                                                                                                                                                           |                                                       |                                                                                                                                                                                                               |

```text
.agents/report/todo/bypass-checker/QUALITY_ANALYSIS.md
```

---

# Review Report: bypass-checker — Quality Analyst

## Summary

The second QC pass confirms that the original line-number, Cargo.toml comment, `usize::MAX`, safe-unwrap, ASCII lowercase, `cfg(test)`, and initial block-comment fixes are present. However, the module is still not production-ready. The most serious remaining problem is that full-line comment bypasses such as `// eslint-disable`, `// @ts-ignore`, and `/* eslint-disable */` are still skipped before non-word bypass patterns are checked. The new block-comment tracker also skips entire lines when a block comment opens or closes on the same line, causing missed violations in code after the comment. There are also remaining AES hygiene issues: duplicated word-token constants, magic strings, and low-level comment parsers that should be moved to Utility.

## QC Verification of Applied Fixes

| #  | Claimed Fix                             | Status               | Note                                                                                                                                                                          |
| -- | --------------------------------------- | -------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| 1  | Stable`line_number` before increment  | ✅ Verified          | Language-scoped violations now use the correct line number.                                                                                                                   |
| 4  | Cargo.toml comment stripping            | ⚠️ Partially fixed | Comments are stripped, but`cargo_value_is_allow()` is too broad and can false-positive on unrelated quoted `"allow"` strings inside inline tables.                        |
| 5  | Avoid`usize::MAX` in string checks    | ✅ Verified          | Pattern position now uses`match`/`continue`.                                                                                                                              |
| 7  | Safe unwrap variants restricted         | ✅ Verified          | Only`or(`, `or_else(`, `or_default(` are treated as safe.                                                                                                               |
| 8  | ASCII lowercase for stable byte offsets | ✅ Verified          | `to_ascii_lowercase()` is used consistently.                                                                                                                                |
| 9  | `cfg(test)` variant detection         | ⚠️ Partially fixed | `cfg(all(...))` support exists, but current logic can still false-positive on `not(test)` because it only checks `starts_with("#[cfg(all(")` plus `contains("test")`. |
| 10 | Block-comment state tracking            | ❌ Incorrect         | The state tracker skips whole lines when a block comment opens/closes on the same line and does not handle mid-line block comments correctly.                                 |

## Findings by Category

### Functional Bugs

| # | Severity    | Issue                                                                                                                                                                                                                                     | Location                                              | Recommendation                                                                                                                                                                                                |
| - | ----------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ----------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| 1 | 🔴 CRITICAL | Full-line comment bypasses are still missed. Lines starting with`//`, `/*`, or `*` are skipped before non-word bypass patterns are checked, so `// eslint-disable`, `// @ts-ignore`, `/* eslint-disable */` are not reported. | `check_bypass_comments`, early `continue` blocks  | Do not skip comment lines completely. Skip only code-only checks such as word tokens, attributes, and language phrase patterns. Non-word bypass-comment patterns must still be checked against the full line. |
| 2 | 🔴 CRITICAL | Block-comment handling skips code after comment close. If a line is`/* comment */ unwrap();`, the current logic continues and never checks `unwrap();`. If a block comment closes mid-line, code after `*/` is ignored.             | `check_bypass_comments`, `in_block_comment` block | Replace the boolean-only check with a stateful comment stripper that returns the code portion outside comments while preserving code after`*/`.                                                             |
| 3 | 🔴 CRITICAL | Mid-line block comments are not tracked. Example:`let x = 1; /* bypass */ unwrap();` or `foo(); /* eslint-disable */` are not handled correctly.                                                                                      | `check_bypass_comments`                             | Use a scanner that detects`/*` and `*/` anywhere in the line, outside string literals.                                                                                                                    |
| 4 | 🟡 WARNING  | `cargo_value_is_allow()` can false-positive on inline tables containing unrelated `"allow"` strings, e.g. `{ level = "warn", note = "allow" }`.                                                                                     | `BypassChecker::cargo_value_is_allow`               | Restrict inline-table detection to`level = "allow"` or `level = 'allow'`.                                                                                                                                 |
| 5 | 🟡 WARNING  | `is_cfg_test_block()` can false-positive on `#[cfg(all(not(test), ...))]` because it checks `starts_with("#[cfg(all(")` and `contains("test")`.                                                                                   | `BypassChecker::is_cfg_test_block`                  | Require`test` to appear as a positive predicate, e.g. `#[cfg(test)]`, `#[cfg(all(test, ...))]`, or `#[cfg(all(test))]`.                                                                               |
| 6 | 🟡 WARNING  | `starts_with("[workspace.lints.clippy]")` can match longer table names unintentionally.                                                                                                                                                 | `check_cargo_toml`                                  | Use exact comparison after comment stripping:`t == "[workspace.lints.clippy]"` or `t == "[lints.clippy]"`.                                                                                                |
| 7 | 🟡 WARNING  | `skip_cfg_test_block()` and `skip_brace_block()` can desynchronize block-comment state if skipped regions contain comment markers.                                                                                                    | `check_bypass_comments`                             | Reset or recompute block-comment state after block skips, or make the skip utilities comment-aware.                                                                                                           |
| 8 | 🟢 INFO     | `#[cfg_attr(..., allow(...))]`, attributes after code on the same line, and nested attributes are still not detected.                                                                                                                   | `starts_with_allow_attr` usage                      | Add regression tests first, then extend attribute detection if required by policy.                                                                                                                            |

### Performance Bottlenecks

| # | Severity   | Issue                                                                                                               | Location                                | Recommendation                                                                               |
| - | ---------- | ------------------------------------------------------------------------------------------------------------------- | --------------------------------------- | -------------------------------------------------------------------------------------------- |
| 1 | 🟡 WARNING | The file is scanned once for early bailout and again for detailed diagnostics. Lines are lowercased in both passes. | `check_bypass_comments`               | Keep the bailout, but avoid duplicate lowercase allocations in the main loop where possible. |
| 2 | 🟡 WARNING | `full_lower` and `code_lower` are allocated per scanned line even when `code_portion == t`.                   | `check_bypass_comments`               | Use`Cow<str>` or reuse the full lowered string when the code portion is unchanged.         |
| 3 | 🟢 INFO    | `cargo_value_is_allow()` allocates a normalized string for every candidate Cargo line.                            | `BypassChecker::cargo_value_is_allow` | Acceptable for`Cargo.toml`; optimize only if manifest scanning becomes hot.                |
| 4 | 🟢 INFO    | The corrected block-comment stripper allocates a`String` for lines containing comments.                           | proposed`code_without_comments`       | Use`Cow<str>` to avoid allocation for comment-free lines.                                  |

### Edge Case Handling

| # | Severity   | Issue                                                                                                                                                            | Location                                                                                            | Recommendation                                                                   |
| - | ---------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------------------------- | -------------------------------------------------------------------------------- |
| 1 | 🟡 WARNING | Raw strings, template literals, Python triple-quoted strings, and Rust lifetimes can confuse comment/string scanning.                                            | `strip_trailing_comment`, `strip_python_comment`, proposed `strip_c_like_comments_with_state` | Move comment/string scanning to Utility and add language-aware regression tests. |
| 2 | 🟡 WARNING | `#[cfg(test)] mod tests {}` written on one line may not be fully skipped because `skip_cfg_test_block()` expects the module declaration on a following line. | `skip_cfg_test_block` usage                                                                       | Handle same-line attribute + module forms in Utility.                            |
| 3 | 🟢 INFO    | Cargo lint arrays are not explicitly handled.                                                                                                                    | `check_cargo_toml`                                                                                | Add tests for array-style lint values if supported by Cargo configuration.       |
| 4 | 🟢 INFO    | Block-comment state is simplified and may desync after skipped test/Lazy blocks.                                                                                 | `check_bypass_comments`                                                                           | Make block skipping comment-aware.                                               |

### Output Correctness

| # | Severity    | Issue                                                                                      | Location                  | Recommendation                                                    |
| - | ----------- | ------------------------------------------------------------------------------------------ | ------------------------- | ----------------------------------------------------------------- |
| 1 | 🔴 CRITICAL | JS/TS comment bypasses are still not emitted.                                              | `check_bypass_comments` | Check non-word patterns before discarding comment-only lines.     |
| 2 | 🔴 CRITICAL | Violations after same-line block comments are missed.                                      | `check_bypass_comments` | Extract code outside comments instead of skipping the whole line. |
| 3 | 🟡 WARNING  | Cargo.toml may emit false positives for unrelated`"allow"` strings inside inline tables. | `cargo_value_is_allow`  | Match only`level = "allow"` or exact `"allow"` values.        |
| 4 | 🟡 WARNING  | Non-test cfg blocks containing`test` text can be skipped incorrectly.                    | `is_cfg_test_block`     | Use a stricter positive`test` predicate check.                  |

## Violations

1. **Duplicated constant logic**`is_word_pattern_token()` still duplicates the shared `WORD_PATTERN_TOKENS` constant from `taxonomy_violation_code_analysis_vo.rs`. It should consume the shared constant.
2. **Magic constants in capability code**Rule code `AES304`, `Cargo.toml`, default bypass patterns, Cargo section names, and JS throw patterns are still hardcoded in the capability. These should be extracted to a shared taxonomy constant file.
3. **Utility-boundary violation**
   `strip_python_comment`, `strip_toml_comment`, and the proposed C-like block-comment stripper are stateless, domain-agnostic parsing helpers. They should live in `utility_bypass.rs`, not inside the capability.

## Action Items

- [ ] 🔴 Fix full-line comment bypass detection.
- [ ] 🔴 Replace boolean block-comment skipping with a stateful code/comment extractor.
- [ ] 🔴 Add regression tests for `// eslint-disable`, `// @ts-ignore`, `/* eslint-disable */`, and code after `*/`.
- [ ] 🟡 Tighten `cargo_value_is_allow()` to avoid unrelated `"allow"` strings.
- [ ] 🟡 Tighten `is_cfg_test_block()` to avoid `not(test)` false positives.
- [ ] 🟡 Use shared `WORD_PATTERN_TOKENS` instead of duplicating the list.
- [ ] 🟡 Extract comment/TOML parsers to `utility_bypass.rs`.
- [ ] 🟢 Extract magic constants to `taxonomy_code_analysis_constant.rs`.

## Fixed Code

The following version fixes the remaining critical QC issues while preserving existing functionality. It still keeps the helper functions inside the capability for immediacy, but those helpers should be moved to `utility_bypass.rs` in the same PR to fully satisfy AES utility boundaries.

```rust
// PURPOSE: BypassChecker — IBypassCheckerProtocol for AES304: detect bypass annotations, panics, and fallback calls
// ALGORITHM:
//   1. Skip #[cfg(test)] blocks and static Lazy<Regex> multiline inits
//   2. Detect source language from the file extension (shared Language VO).
//   3. For each line, classify forbidden tokens using word-boundary aware substring matching.
//   4. Patterns are read from ArchitectureConfig.code_analysis.forbidden_bypass.values so
//      YAML config is honored (not hardcoded). A fallback default list applies if empty.
use std::borrow::Cow;

use shared::cli_commands::taxonomy_result_vo::LintResult;
use shared::cli_commands::taxonomy_severity_vo::Severity;
use shared::code_analysis::contract_bypass_checker_protocol::IBypassCheckerProtocol;
use shared::code_analysis::taxonomy_code_analysis_rule_vo::CodeAnalysisRuleVO;
use shared::code_analysis::taxonomy_violation_code_analysis_vo::{
    AesCodeAnalysisViolation, Language, ViolationKind, WORD_PATTERN_TOKENS,
};
use shared::code_analysis::utility_bypass::{
    is_inside_string_or_char, matches_word_token, skip_brace_block, skip_cfg_test_block,
    starts_with_allow_attr, strip_trailing_comment,
};
use shared::code_analysis::utility_language_mapper::code_analysis_language_from_file;
use shared::common::taxonomy_common_vo::PatternList;

// ─── Block 1: Struct Definition ───────────────────────────
pub struct BypassChecker {
    rule: CodeAnalysisRuleVO,
}

// ─── Block 2: Protocol Trait Implementation ───────────────
impl IBypassCheckerProtocol for BypassChecker {
    fn check_cargo_toml(&self, content: &str, violations: &mut Vec<LintResult>) {
        let mut in_clippy_section = false;

        for (i, line) in content.lines().enumerate() {
            let t = line.trim();

            // Skip empty lines and TOML full-line comments.
            if t.is_empty() || t.starts_with('#') {
                continue;
            }

            // Strip trailing TOML comments outside strings before comparing values.
            let t = Self::strip_toml_comment(t).trim();
            if t.is_empty() {
                continue;
            }

            // Exact section matching avoids accidental matches on longer table names.
            if t == "[workspace.lints.clippy]" || t == "[lints.clippy]" {
                in_clippy_section = true;
                continue;
            }

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

    fn check_bypass_comments(&self, file: &str, content: &str, violations: &mut Vec<LintResult>) {
        let patterns = &self.rule.forbidden_bypass;

        // P1.7 fix: use fallback default patterns when config is empty.
        let effective_patterns = if patterns.values.is_empty() {
            Self::default_forbidden_bypass()
        } else {
            PatternList {
                values: patterns.values.clone(),
            }
        };

        // P2.4 fix: precompute lowered patterns once per file scan.
        // ASCII lowercase keeps byte offsets stable for is_inside_string_or_char checks.
        let lowered_patterns: Vec<String> = effective_patterns
            .iter()
            .map(|p| p.to_ascii_lowercase())
            .collect();

        let language = code_analysis_language_from_file(file);

        // Early bailout scan.
        //
        // This intentionally checks the full lowered line for non-word bypass patterns
        // so comment-based bypasses like `// eslint-disable` are not missed.
        let has_bypass_token = content.lines().any(|line| {
            let trimmed = line.trim();
            if trimmed.is_empty() {
                return false;
            }

            let full_lower = trimmed.to_ascii_lowercase();
            if lowered_patterns
                .iter()
                .any(|p| full_lower.contains(p.as_str()))
            {
                return true;
            }

            let code_portion = Self::code_portion_for_language(trimmed, language);
            if starts_with_allow_attr(code_portion) {
                return true;
            }

            let code_lower = code_portion.to_ascii_lowercase();
            match language {
                Language::Python => {
                    code_lower.contains("raise notimplementederror")
                        || code_lower.contains("raise notimplemented")
                        || code_lower.contains("assert false")
                }
                Language::JavaScript | Language::TypeScript => code_lower.contains("throw new"),
                _ => false,
            }
        });

        if !has_bypass_token {
            return;
        }

        let lines: Vec<&str> = content.lines().collect();
        let mut i = 0;
        let mut in_block_comment = false;

        while i < lines.len() {
            let t = lines[i].trim();
            let line_number = i + 1;

            // Extract code outside comments.
            //
            // For C-like languages this tracks block comments across lines and preserves
            // code after a closing `*/` on the same line.
            let code_owned = Self::code_without_comments(t, language, &mut in_block_comment);
            let code_portion = code_owned.as_str();
            let code_trim = code_portion.trim();

            // Skip test modules — unwrap/panic is normal in tests.
            //
            // Only apply this when the attribute appears in actual code, not inside comments.
            if !code_trim.is_empty() && Self::is_cfg_test_block(code_trim) {
                i = skip_cfg_test_block(&lines, i);
                in_block_comment = false;
                continue;
            }

            // Skip static Lazy<Regex> multiline initialization blocks.
            if !code_trim.is_empty() && code_trim.contains("static ") && code_trim.contains("Lazy")
            {
                i = skip_brace_block(&lines, i);
                in_block_comment = false;
                continue;
            }

            // Allow attribute: rustc annotation attributes → BYPASS_COMMENT.
            if starts_with_allow_attr(code_trim) {
                violations.push(LintResult::new_arch(
                    file,
                    line_number,
                    "AES304",
                    Severity::CRITICAL,
                    AesCodeAnalysisViolation::BypassComment { reason: None }.to_string(),
                ));
                i += 1;
                continue;
            }

            let full_lower = t.to_ascii_lowercase();
            let code_lower = code_trim.to_ascii_lowercase();
            let mut matched = false;

            for lower_p in lowered_patterns.iter() {
                let token = lower_p.as_str();

                if Self::is_word_pattern_token(token) {
                    // Word tokens like unwrap/panic/todo must not be reported from comment text.
                    if code_trim.is_empty() {
                        continue;
                    }

                    let pattern_pos = match code_lower.find(token) {
                        Some(pos) => pos,
                        None => continue,
                    };

                    if matches_word_token(code_lower.as_str(), token, false)
                        && !(token == "unwrap"
                            && Self::has_safe_unwrap_variant(code_lower.as_str()))
                        && !is_inside_string_or_char(code_trim, pattern_pos)
                    {
                        let vo = match Self::classify_token(token) {
                            ViolationKind::UnwrapExpect => {
                                AesCodeAnalysisViolation::UnwrapExpect { reason: None }
                            }
                            ViolationKind::Panic => {
                                AesCodeAnalysisViolation::Panic { reason: None }
                            }
                            ViolationKind::Todo => AesCodeAnalysisViolation::Todo { reason: None },
                            ViolationKind::Unimplemented => {
                                AesCodeAnalysisViolation::Unimplemented { reason: None }
                            }
                            ViolationKind::BypassComment => {
                                AesCodeAnalysisViolation::BypassComment { reason: None }
                            }
                        };

                        violations.push(LintResult::new_arch(
                            file,
                            line_number,
                            "AES304",
                            Severity::CRITICAL,
                            vo.to_string(),
                        ));

                        matched = true;
                        break;
                    }
                } else if !token.is_empty() {
                    // Non-word patterns are bypass-comment patterns such as noqa,
                    // type: ignore, eslint-disable, ts-ignore, FIXME, HACK, XXX.
                    //
                    // These must be detected even when they appear inside comments.
                    let pattern_pos = match full_lower.find(token) {
                        Some(pos) => pos,
                        None => continue,
                    };

                    if !is_inside_string_or_char(t, pattern_pos) {
                        violations.push(LintResult::new_arch(
                            file,
                            line_number,
                            "AES304",
                            Severity::CRITICAL,
                            AesCodeAnalysisViolation::BypassComment { reason: None }.to_string(),
                        ));

                        matched = true;
                        break;
                    }
                }
            }

            // Language-scoped phrase patterns.
            //
            // These are code-path violations, so they must not be checked inside comments.
            if !matched && !code_trim.is_empty() {
                match language {
                    Language::Python => {
                        if code_lower.contains("raise notimplementederror")
                            || code_lower.contains("raise notimplemented")
                        {
                            violations.push(LintResult::new_arch(
                                file,
                                line_number,
                                "AES304",
                                Severity::CRITICAL,
                                AesCodeAnalysisViolation::Unimplemented { reason: None }
                                    .to_string(),
                            ));
                        } else if code_lower.contains("assert false") {
                            violations.push(LintResult::new_arch(
                                file,
                                line_number,
                                "AES304",
                                Severity::CRITICAL,
                                AesCodeAnalysisViolation::Panic { reason: None }.to_string(),
                            ));
                        }
                    }
                    Language::JavaScript | Language::TypeScript => {
                        let throw_patterns = [
                            "throw new error",
                            "throw new typeerror",
                            "throw new rangeerror",
                            "throw new referenceerror",
                            "throw new syntaxerror",
                        ];

                        if throw_patterns.iter().any(|p| code_lower.contains(p)) {
                            violations.push(LintResult::new_arch(
                                file,
                                line_number,
                                "AES304",
                                Severity::CRITICAL,
                                AesCodeAnalysisViolation::Panic { reason: None }.to_string(),
                            ));
                        }
                    }
                    _ => {} // Rust handled above via config patterns.
                }
            }

            i += 1;
        }
    }
}

// ─── Block 3: Constructors, Helpers, Private Methods ──────
impl Default for BypassChecker {
    fn default() -> Self {
        Self::new()
    }
}

impl BypassChecker {
    pub fn new() -> Self {
        Self {
            rule: CodeAnalysisRuleVO::default(),
        }
    }

    /// Build a BypassChecker from an ArchitectureConfig-derived CodeAnalysisRuleVO.
    pub fn from_rule(rule: CodeAnalysisRuleVO) -> Self {
        Self { rule }
    }

    /// Build a BypassChecker from a PatternList (forbidden_bypass patterns).
    pub fn from_patterns(patterns: &PatternList) -> Self {
        Self {
            rule: CodeAnalysisRuleVO {
                forbidden_bypass: patterns.clone(),
                ..CodeAnalysisRuleVO::default()
            },
        }
    }

    /// Map a forbidden token to its ViolationKind variant.
    fn classify_token(token: &str) -> ViolationKind {
        match token {
            "unwrap" | "expect" => ViolationKind::UnwrapExpect,
            "panic" => ViolationKind::Panic,
            "todo" => ViolationKind::Todo,
            "unimplemented" | "unreachable" => ViolationKind::Unimplemented,
            _ => ViolationKind::BypassComment,
        }
    }

    /// Tokens that require call-site style matching rather than plain contains.
    ///
    /// Uses the shared taxonomy constant instead of duplicating the token list.
    fn is_word_pattern_token(token: &str) -> bool {
        WORD_PATTERN_TOKENS.contains(&token)
    }

    /// Default fallback bypass patterns when config provides none.
    ///
    /// TODO(AES-CONSTANTS): extract to taxonomy_code_analysis_constant.rs.
    fn default_forbidden_bypass() -> PatternList {
        PatternList {
            values: vec![
                "unwrap".to_string(),
                "expect".to_string(),
                "panic".to_string(),
                "todo".to_string(),
                "unimplemented".to_string(),
                "unreachable".to_string(),
                // Python bypass patterns
                "type: ignore".to_string(),
                "noqa".to_string(),
                // JS/TS bypass patterns
                "@ts-ignore".to_string(),
                "@ts-expect-error".to_string(),
                "eslint-disable".to_string(),
                "lint-disable".to_string(),
                // Generic fallback
                "FIXME".to_string(),
                "HACK".to_string(),
                "XXX".to_string(),
            ],
        }
    }

    /// Returns true if the line has ONLY safe `.unwrap_or*()` variants and no unsafe `.unwrap()`.
    ///
    /// Matches only known safe variants: unwrap_or, unwrap_or_else, unwrap_or_default.
    fn has_safe_unwrap_variant(line: &str) -> bool {
        let bytes = line.as_bytes();
        let len = bytes.len();
        let mut i = 0;

        while i < len {
            if bytes[i..].starts_with(b".unwrap") {
                i += 7; // skip past ".unwrap"

                if i < len {
                    match bytes[i] {
                        b'(' | b'!' => {
                            // Unsafe .unwrap() or .unwrap! style call.
                            return false;
                        }
                        b'_' => {
                            i += 1;
                            let rest = &bytes[i..];

                            // Known safe variants only.
                            if rest.starts_with(b"or(")
                                || rest.starts_with(b"or_else(")
                                || rest.starts_with(b"or_default(")
                            {
                                continue;
                            }

                            // Unknown variant — treat as unsafe.
                            return false;
                        }
                        _ => {
                            i += 1;
                            continue;
                        }
                    }
                }
            }

            i += 1;
        }

        true
    }

    /// Detect cfg(test) blocks including positive `cfg(all(test, ...))` variants.
    ///
    /// This intentionally avoids false positives such as `#[cfg(all(not(test), ...))]`.
    fn is_cfg_test_block(line: &str) -> bool {
        if !line.starts_with("#[cfg(") {
            return false;
        }

        let compact: String = line.chars().filter(|c| !c.is_whitespace()).collect();

        compact.starts_with("#[cfg(test)]")
            || compact.starts_with("#[cfg(all(test)]")
            || compact.starts_with("#[cfg(all(test,")
    }

    /// Returns the code portion of a line for language-sensitive early-scan checks.
    ///
    /// TODO(UTILITY): move to utility_bypass.rs.
    fn code_portion_for_language(line: &str, language: Language) -> &str {
        match language {
            Language::Python => Self::strip_python_comment(line),
            _ => strip_trailing_comment(line),
        }
    }

    /// Returns code outside comments, tracking C-like block comments across lines.
    ///
    /// TODO(UTILITY): move to utility_bypass.rs.
    fn code_without_comments<'a>(
        line: &'a str,
        language: Language,
        in_block_comment: &mut bool,
    ) -> Cow<'a, str> {
        match language {
            Language::Python => Cow::Borrowed(Self::strip_python_comment(line)),
            _ => {
                // Fast path: no comment markers and not currently inside a block comment.
                if !*in_block_comment
                    && !line.contains("//")
                    && !line.contains("/*")
                    && !line.contains("*/")
                {
                    return Cow::Borrowed(line);
                }

                Cow::Owned(Self::strip_c_like_comments_with_state(
                    line,
                    in_block_comment,
                ))
            }
        }
    }

    /// Strip C-like `//` and `/* ... */` comments while preserving code after block comments.
    ///
    /// TODO(UTILITY): move to utility_bypass.rs and add richer language-aware lexing.
    fn strip_c_like_comments_with_state(line: &str, in_block_comment: &mut bool) -> String {
        let bytes = line.as_bytes();
        let len = bytes.len();

        let mut result = String::new();
        let mut segment_start = 0;
        let mut i = 0;

        let mut in_string = false;
        let mut string_quote: u8 = b'"';
        let mut in_char = false;

        while i < len {
            if *in_block_comment {
                if i + 1 < len && bytes[i] == b'*' && bytes[i + 1] == b'/' {
                    *in_block_comment = false;
                    i += 2;
                    segment_start = i;
                } else {
                    i += 1;
                }
                continue;
            }

            let b = bytes[i];

            if in_string {
                if b == b'\\' {
                    i += 2;
                    continue;
                }

                if b == string_quote {
                    in_string = false;
                }

                i += 1;
                continue;
            }

            if in_char {
                if b == b'\\' {
                    i += 2;
                    continue;
                }

                if b == b'\'' {
                    in_char = false;
                }

                i += 1;
                continue;
            }

            if b == b'"' {
                in_string = true;
                string_quote = b'"';
                i += 1;
                continue;
            }

            if b == b'\'' {
                in_char = true;
                i += 1;
                continue;
            }

            if b == b'/' && i + 1 < len {
                // Line comment: preserve code before the comment and stop.
                if bytes[i + 1] == b'/' {
                    result.push_str(&line[segment_start..i]);
                    return result;
                }

                // Block comment start: preserve code before the comment.
                if bytes[i + 1] == b'*' {
                    result.push_str(&line[segment_start..i]);
                    *in_block_comment = true;
                    i += 2;
                    segment_start = i;
                    continue;
                }
            }

            i += 1;
        }

        if !*in_block_comment && segment_start < len {
            result.push_str(&line[segment_start..len]);
        }

        result
    }

    /// Strip trailing Python `# ...` comments outside simple string literals.
    ///
    /// TODO(UTILITY): move to utility_bypass.rs and add triple-quote support.
    fn strip_python_comment(line: &str) -> &str {
        let bytes = line.as_bytes();
        let len = bytes.len();
        let mut i = 0;
        let mut in_string = false;
        let mut quote: u8 = b'"';

        while i < len {
            let b = bytes[i];

            if in_string {
                if b == quote && (i == 0 || bytes[i - 1] != b'\\') {
                    in_string = false;
                }
            } else if b == b'"' || b == b'\'' {
                in_string = true;
                quote = b;
            } else if b == b'#' {
                return &line[..i];
            }

            i += 1;
        }

        line
    }

    /// Strip trailing TOML `# ...` comments outside simple string literals.
    ///
    /// TODO(UTILITY): move to utility_bypass.rs.
    fn strip_toml_comment(line: &str) -> &str {
        let bytes = line.as_bytes();
        let len = bytes.len();
        let mut i = 0;
        let mut in_string = false;
        let mut quote: u8 = b'"';

        while i < len {
            let b = bytes[i];

            if in_string {
                if b == quote && (i == 0 || bytes[i - 1] != b'\\') {
                    in_string = false;
                }
            } else if b == b'"' || b == b'\'' {
                in_string = true;
                quote = b;
            } else if b == b'#' {
                return &line[..i];
            }

            i += 1;
        }

        line
    }

    /// Detect Cargo lint values that effectively silence lints.
    ///
    /// Handles:
    /// - `warnings = "allow"`
    /// - `warnings = 'allow'`
    /// - `warnings = { level = "allow" }`
    /// - `warnings = { level = 'allow', priority = -1 }`
    ///
    /// This avoids false positives such as `{ level = "warn", note = "allow" }`.
    fn cargo_value_is_allow(value: &str) -> bool {
        let value = value.trim();

        if value == "\"allow\"" || value == "'allow'" {
            return true;
        }

        let normalized: String = value.chars().filter(|c| !c.is_whitespace()).collect();

        normalized.contains("level=\"allow\"") || normalized.contains("level='allow'")
    }
}
```
