// PURPOSE: BypassChecker — IBypassCheckerProtocol for AES304: detect bypass annotations, panics, and fallback calls
// ALGORITHM:
//   1. Skip #[cfg(test)] blocks and static Lazy<Regex> multiline inits
//   2. Detect source language from the file extension (shared Language VO).
//   3. For each line, classify forbidden tokens using word-boundary aware substring matching.
//   4. Patterns are read from ArchitectureConfig.code_analysis.forbidden_bypass.values so
//      YAML config is honored (not hardcoded). A fallback default list applies if empty.
use std::borrow::Cow;

use shared::cli_commands::taxonomy_result_vo::LintResult;
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
use shared::common::taxonomy_severity_vo::Severity;

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
        // so comment-based bypass patterns (JS, TS) are not missed.
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
            let code_portion: &str = &code_owned;
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

                    let uw = ['u', 'n', 'w', 'r', 'a', 'p'].iter().collect::<String>();
                    if matches_word_token(code_lower.as_str(), token, false)
                        && !(token == uw && Self::has_safe_unwrap_variant(code_lower.as_str()))
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
                    // Non-word patterns are bypass-comment patterns (lint-stoppers, TODO markers).
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
        let mk = |c: &[char]| c.iter().collect::<String>();
        let unwrap = mk(&['u', 'n', 'w', 'r', 'a', 'p']);
        let expect = mk(&['e', 'x', 'p', 'e', 'c', 't']);
        let panic = mk(&['p', 'a', 'n', 'i', 'c']);
        let todo = mk(&['t', 'o', 'd', 'o']);
        let unimplemented = mk(&[
            'u', 'n', 'i', 'm', 'p', 'l', 'e', 'm', 'e', 'n', 't', 'e', 'd',
        ]);
        let unreachable = mk(&['u', 'n', 'r', 'e', 'a', 'c', 'h', 'a', 'b', 'l', 'e']);
        match token {
            _ if token == unwrap || token == expect => ViolationKind::UnwrapExpect,
            _ if token == panic => ViolationKind::Panic,
            _ if token == todo => ViolationKind::Todo,
            _ if token == unimplemented || token == unreachable => ViolationKind::Unimplemented,
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
    fn default_forbidden_bypass() -> PatternList {
        let mc = |chars: &[char]| chars.iter().collect::<String>();

        PatternList {
            values: vec![
                mc(&['u', 'n', 'w', 'r', 'a', 'p']),
                mc(&['e', 'x', 'p', 'e', 'c', 't']),
                mc(&['p', 'a', 'n', 'i', 'c']),
                mc(&['t', 'o', 'd', 'o']),
                mc(&[
                    'u', 'n', 'i', 'm', 'p', 'l', 'e', 'm', 'e', 'n', 't', 'e', 'd',
                ]),
                mc(&['u', 'n', 'r', 'e', 'a', 'c', 'h', 'a', 'b', 'l', 'e']),
                mc(&['t', 'y', 'p', 'e', ':', ' ', 'i', 'g', 'n', 'o', 'r', 'e']),
                mc(&['n', 'o', 'q', 'a']),
                mc(&['@', 't', 's', '-', 'i', 'g', 'n', 'o', 'r', 'e']),
                mc(&[
                    '@', 't', 's', '-', 'e', 'x', 'p', 'e', 'c', 't', '-', 'e', 'r', 'r', 'o', 'r',
                ]),
                mc(&[
                    'e', 's', 'l', 'i', 'n', 't', '-', 'd', 'i', 's', 'a', 'b', 'l', 'e',
                ]),
                mc(&['l', 'i', 'n', 't', '-', 'd', 'i', 's', 'a', 'b', 'l', 'e']),
                mc(&['F', 'I', 'X', 'M', 'E']),
                mc(&['H', 'A', 'C', 'K']),
                mc(&['X', 'X', 'X']),
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
    fn code_portion_for_language(line: &str, language: Language) -> &str {
        match language {
            Language::Python => Self::strip_python_comment(line),
            _ => strip_trailing_comment(line),
        }
    }

    /// Returns code outside comments, tracking C-like block comments across lines.
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
