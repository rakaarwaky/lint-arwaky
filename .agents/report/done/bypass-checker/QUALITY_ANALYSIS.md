
---

# Review Report: bypass-checker — Quality Analyst

## Summary

The `BypassChecker` module correctly targets AES304 and already follows the AES 3-block capability structure, implements the required protocol trait, and delegates several low-level operations to shared utility functions. However, the current implementation contains multiple functional correctness bugs and avoidable performance overhead. The most critical issues are: wrong line numbers for Python/JS/TS language-scoped violations, case-sensitive early-exit filtering that can silently skip entire files, missed comment-based bypasses such as `// eslint-disable` and `// @ts-ignore`, unsafe Cargo.toml comment handling, and repeated per-line lowercase allocations. The fixed code below corrects these issues without adding new product functionality. It also tightens token matching and reduces false negatives/positives around comments, strings, and case handling.

## Findings by Category

### Functional Bugs

| #  | Severity    | Issue                                                                                                                                                                                                                                                        | Location                                                                    | Recommendation                                                                                                                                                                                                         |
| -- | ----------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ | --------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| 1  | 🔴 CRITICAL | Language-scoped Python/JS/TS violations are emitted with the wrong line number. The code increments`i` before checking language phrase patterns, then reports `i + 1`, causing all such violations to point to the next line.                            | `check_bypass_comments`, language-scoped phrase patterns after `i += 1` | Capture`line_number = i + 1` before mutation and use it consistently for all diagnostics on the current line.                                                                                                        |
| 2  | 🔴 CRITICAL | Early-exit scan is case-sensitive for patterns. It lowercases the line but checks raw patterns, so default patterns like`FIXME`, `HACK`, `XXX`, or mixed-case YAML patterns can fail to trigger the detailed scan, causing whole-file false negatives. | `check_bypass_comments`, `has_bypass_token` closure                     | Precompute lowercased patterns before the early scan and compare against lowercased/ASCII-lowercased line content.                                                                                                     |
| 3  | 🔴 CRITICAL | Full-line comment skipping suppresses comment-based bypass detection for JS/TS. Lines such as`// eslint-disable`, `// @ts-ignore`, `// @ts-expect-error`, or `/* eslint-disable */` are skipped entirely, violating AES304 zero-bypass intent.       | `check_bypass_comments`, comment skip block                               | Distinguish code-only checks from bypass-comment checks. Word tokens like`unwrap`/`panic` should be checked only in code portions, while non-word bypass patterns should be checked against the full trimmed line. |
| 4  | 🔴 CRITICAL | Cargo.toml parsing does not handle TOML comments. Commented lines can create false positives, and trailing comments can create false negatives because value comparison is exact.                                                                            | `check_cargo_toml`                                                        | Skip full-line`#` comments, strip trailing TOML comments outside strings, and normalize inline table values such as `{ level = "allow" }`.                                                                         |
| 5  | 🟡 WARNING  | `usize::MAX` is passed to `is_inside_string_or_char` when a pattern is not found. This can produce incorrect string-state results and silently suppress valid diagnostics.                                                                               | `check_bypass_comments`, pattern position lookup                          | Use`match`/`let-else` and continue immediately when the pattern is absent.                                                                                                                                         |
| 6  | 🟡 WARNING  | `unwrap`/`expect` matching requires a preceding dot. Bare calls such as `unwrap()` or `expect()` can be missed, while FRD explicitly calls out `unwrap()`/`expect()` detection.                                                                  | `check_bypass_comments`, `requires_method_call` logic                   | Pass`false` for `requires_method_call` while preserving word-boundary and safe-variant checks.                                                                                                                     |
| 7  | 🟡 WARNING  | `has_safe_unwrap_variant` treats any `.unwrap_or*` prefix as safe. It can consider unknown variants such as `.unwrap_oracle()` safe.                                                                                                                   | `BypassChecker::has_safe_unwrap_variant`                                  | Match only known safe variants:`.unwrap_or(...)`, `.unwrap_or_else(...)`, `.unwrap_or_default(...)`.                                                                                                             |
| 8  | 🟡 WARNING  | `to_lowercase()` can change byte length for non-ASCII text. Pattern positions derived from lowercased strings may not align with the original line when passed to `is_inside_string_or_char`.                                                            | `check_bypass_comments`                                                   | Use`to_ascii_lowercase()` for ASCII bypass patterns so byte offsets remain stable.                                                                                                                                   |
| 9  | 🟡 WARNING  | `#[cfg(test)]` skipping only matches the exact literal prefix `#[cfg(test)]`. Variants such as `#[cfg(all(test, feature = "x"))]` are not skipped.                                                                                                     | `check_bypass_comments`, cfg skip                                         | Use a more robust cfg-test detector in a follow-up utility function.                                                                                                                                                   |
| 10 | 🟡 WARNING  | Block-comment handling is incomplete. Lines inside`/* ... */` blocks that do not start with `/*` or `*` may still be treated as code and produce false positives for word tokens.                                                                      | `check_bypass_comments`                                                   | Add a stateful comment scanner to`utility_bypass` and use it from the capability.                                                                                                                                    |

### Performance Bottlenecks

| # | Severity   | Issue                                                                                                                                                                                                   | Location                                           | Recommendation                                                                                                     |
| - | ---------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | -------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------ |
| 1 | 🟡 WARNING | The file is scanned once for early bailout and again for detailed diagnostics. When a bypass exists, every line is lowercased twice.                                                                    | `check_bypass_comments`                          | Keep the early bailout, but precompute lowered patterns once and avoid redundant lowercasing inside the main loop. |
| 2 | 🟡 WARNING | The main loop creates`line_lc = code_portion.to_lowercase()` even though `t_lower`/`code_lower` already exists.                                                                                   | `check_bypass_comments`, language phrase section | Reuse the already computed`code_lower`.                                                                          |
| 3 | 🟡 WARNING | `matches_word_token` is invoked for every word pattern on every non-skipped line, even when the token is absent.                                                                                      | `check_bypass_comments`, pattern loop            | Use`code_lower.find(token)` as a cheap guard before calling the more expensive boundary matcher.                 |
| 4 | 🟢 INFO    | `Vec<&str>` line collection is allocated for every file that passes the early scan. This is acceptable because block skipping requires random access, but it should only happen after the early scan. | `check_bypass_comments`                          | Preserve current behavior; it is already conditional on`has_bypass_token`.                                       |
| 5 | 🟢 INFO    | Cargo.toml inline-table normalization allocates a new string per candidate line. This is low impact because`Cargo.toml` files are small.                                                              | `cargo_value_is_allow`                           | Acceptable for now; optimize only if Cargo manifest scanning becomes hot.                                          |

### Edge Case Handling

| # | Severity   | Issue                                                                                                              | Location                                                      | Recommendation                                                          |
| - | ---------- | ------------------------------------------------------------------------------------------------------------------ | ------------------------------------------------------------- | ----------------------------------------------------------------------- |
| 1 | 🟡 WARNING | Block comments, raw strings, template literals, and Python triple-quoted strings are not fully understood.         | `check_bypass_comments`, `is_inside_string_or_char` usage | Move richer lexing into`utility_bypass` and reuse it across checkers. |
| 2 | 🟡 WARNING | TOML inline tables and arrays are only partially handled.                                                          | `check_cargo_toml`                                          | Normalize common inline table forms and add regression tests.           |
| 3 | 🟡 WARNING | Attributes appearing after code on the same line are not detected. Example:`let x = 1; #[allow(unused)]`.        | `starts_with_allow_attr` usage                              | Add attribute-aware scanning in utility if required by policy.          |
| 4 | 🟡 WARNING | `static Lazy` block skipping uses naive brace counting and can be confused by braces inside strings or comments. | `skip_brace_block` usage                                    | Harden utility block skipping with string/comment awareness.            |
| 5 | 🟢 INFO    | `cfg_attr(..., allow(...))` and other nested attribute forms are not detected.                                   | `check_bypass_comments`                                     | Add targeted regression tests before expanding detection.               |

### Output Correctness

| # | Severity    | Issue                                                                                                      | Location                  | Recommendation                                                                                  |
| - | ----------- | ---------------------------------------------------------------------------------------------------------- | ------------------------- | ----------------------------------------------------------------------------------------------- |
| 1 | 🔴 CRITICAL | Python/JS/TS phrase violations point to the wrong line.                                                    | `check_bypass_comments` | Use stable`line_number` captured before incrementing loop index.                              |
| 2 | 🔴 CRITICAL | Some bypass-rich files are skipped entirely because early detection is case-sensitive.                     | `has_bypass_token`      | Compare using lowercased patterns.                                                              |
| 3 | 🔴 CRITICAL | JS/TS comment bypasses are missed due to full-line comment skipping.                                       | `check_bypass_comments` | Check non-word bypass patterns against the full line while keeping word-token checks code-only. |
| 4 | 🟡 WARNING  | Cargo.toml diagnostics can be emitted for commented configuration or missed when comments trail the value. | `check_cargo_toml`      | Strip TOML comments before value inspection.                                                    |
| 5 | 🟡 WARNING  | Non-ASCII case folding can misalign string-literal checks.                                                 | `check_bypass_comments` | Use ASCII lowercase for ASCII bypass patterns.                                                  |

## Violations

1. **Magic constants in capability code**The module contains hardcoded rule codes, file names, default patterns, and phrase patterns. AES prefers reusable constants in a shared taxonomy constant file, for example `taxonomy_code_analysis_constant.rs`.
2. **Low-level parsing helpers inside capability**The fixed code introduces small private parsers for Python comments and TOML comments to make the module functionally correct. These are technical, stateless, domain-agnostic helpers and should be extracted to `utility_bypass.rs` to fully satisfy AES utility-delegation and DRY rules.
3. **Duplicated token list**
   The original `is_word_pattern_token` duplicates the intent of `WORD_PATTERN_TOKENS` from taxonomy. The fixed code uses the shared constant instead.

## Action Items

- [ ] 🔴 Apply the fixed `BypassChecker` code below.
- [ ] 🔴 Add regression tests for:
  - `// eslint-disable`
  - `// @ts-ignore`
  - `// @ts-expect-error`
  - Python `# noqa`
  - Python `# type: ignore`
  - Rust `#[allow(...)]`
  - Rust `.unwrap()`
  - Rust safe `.unwrap_or_else(...)`
  - Cargo.toml commented and inline-table allow values
  - Python/JS/TS language phrase line numbers
- [ ] 🟡 Extract `strip_python_comment`, `strip_toml_comment`, `is_comment_line`, and `code_portion_for_language` into `utility_bypass.rs`.
- [ ] 🟡 Extract default bypass patterns, rule code `AES304`, and language phrase patterns into shared taxonomy constants.
- [ ] 🟡 Implement a stateful block-comment scanner in utility to eliminate remaining block-comment false positives.
- [ ] 🟢 Improve `cfg(test)` detection to support `cfg(all(test, ...))` and similar forms.

## Fixed Code

Replace `crates/code-analysis/src/capabilities_check_bypass_checker.rs` with the following corrected implementation.

> Note: The private comment/TOML helpers are included here to keep the fix self-contained. They should be moved to `utility_bypass.rs` in the same PR to fully satisfy AES utility boundaries.

```rust
// PURPOSE: BypassChecker — IBypassCheckerProtocol for AES304: detect bypass annotations, panics, and fallback calls
// ALGORITHM:
//   1. Skip #[cfg(test)] blocks and static Lazy<Regex> multiline inits
//   2. Detect source language from the file extension (shared Language VO).
//   3. For each line, classify forbidden tokens using word-boundary aware substring matching.
//   4. Patterns are read from ArchitectureConfig.code_analysis.forbidden_bypass.values so
//      YAML config is honored (not hardcoded). A fallback default list applies if empty.
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
            let trimmed = line.trim();

            // TOML full-line comment.
            if trimmed.is_empty() || trimmed.starts_with('#') {
                continue;
            }

            // Strip trailing TOML comments outside strings before comparing values.
            let t = Self::strip_toml_comment(trimmed).trim();
            if t.is_empty() {
                continue;
            }

            if t.starts_with('[') {
                in_clippy_section =
                    t == "[workspace.lints.clippy]" || t == "[lints.clippy]";
                continue;
            }

            if !in_clippy_section {
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
        // ASCII lowercase is used because all default bypass patterns are ASCII and
        // byte offsets must remain stable for is_inside_string_or_char checks.
        let lowered_patterns: Vec<String> = effective_patterns
            .iter()
            .map(|p| p.to_ascii_lowercase())
            .collect();

        let language = code_analysis_language_from_file(file);

        // Early bailout scan.
        //
        // This intentionally checks the full lowered line for non-word bypass patterns
        // so comment-based bypasses such as `// eslint-disable` are not missed.
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

        while i < lines.len() {
            let t = lines[i].trim();
            let line_number = i + 1;

            // Skip test modules — unwrap/panic is normal in tests.
            if t.starts_with("#[cfg(test)]") {
                i = skip_cfg_test_block(&lines, i);
                continue;
            }

            // Skip static Lazy<Regex> multiline initialization blocks.
            if t.contains("static ") && t.contains("Lazy") {
                i = skip_brace_block(&lines, i);
                continue;
            }

            let is_comment_line = Self::is_comment_line(t, language);
            let code_portion = Self::code_portion_for_language(t, language);

            // Allow attribute: rustc annotation attributes → BYPASS_COMMENT.
            if !is_comment_line && starts_with_allow_attr(code_portion) {
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
            let code_lower = code_portion.to_ascii_lowercase();
            let mut matched = false;

            for lower_p in lowered_patterns.iter() {
                let token = lower_p.as_str();

                if Self::is_word_pattern_token(token) {
                    // Word tokens such as unwrap/panic/todo must not be reported from
                    // ordinary comment text. They are code-path violations.
                    if is_comment_line {
                        continue;
                    }

                    let pattern_pos = match code_lower.find(token) {
                        Some(pos) => pos,
                        None => continue,
                    };

                    if matches_word_token(code_lower.as_str(), token, false)
                        && !(token == "unwrap"
                            && Self::has_safe_unwrap_variant(code_lower.as_str()))
                        && !is_inside_string_or_char(code_portion, pattern_pos)
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
            // Fixed: use the stable `line_number` captured before index mutation.
            if !matched && !is_comment_line {
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
    /// Fixed to recognize only known safe variants.
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

                            // Unknown or unsafe variant.
                            return false;
                        }
                        _ => {
                            // Not a method call — just a variable/field name containing "unwrap".
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

    /// Returns the code portion of a line for language-sensitive checks.
    ///
    /// TODO(UTILITY): move to utility_bypass.rs.
    fn code_portion_for_language(line: &str, language: Language) -> &str {
        match language {
            Language::Python => Self::strip_python_comment(line),
            _ => strip_trailing_comment(line),
        }
    }

    /// Conservative full-line comment detection.
    ///
    /// TODO(UTILITY): move to utility_bypass.rs and replace with stateful block-comment scanning.
    fn is_comment_line(line: &str, language: Language) -> bool {
        match language {
            Language::Python => line.starts_with('#'),
            Language::Rust | Language::JavaScript | Language::TypeScript => {
                line.starts_with("//") || line.starts_with("/*") || line.starts_with('*')
            }
            _ => false,
        }
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
    fn cargo_value_is_allow(value: &str) -> bool {
        let value = value.trim();

        if value == "\"allow\"" || value == "'allow'" {
            return true;
        }

        let normalized: String = value.chars().filter(|c| !c.is_whitespace()).collect();

        normalized.contains("\"allow\"") || normalized.contains("'allow'")
    }
}
```
