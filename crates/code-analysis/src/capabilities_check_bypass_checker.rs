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
    AesCodeAnalysisViolation, Language, ViolationKind,
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

            // Skip empty lines and TOML full-line comments
            if t.is_empty() || t.starts_with('#') {
                continue;
            }

            // Strip trailing TOML comments outside strings before comparing values
            let t = Self::strip_toml_comment(t).trim();
            if t.is_empty() {
                continue;
            }

            if t.starts_with("[workspace.lints.clippy]") || t.starts_with("[lints.clippy]") {
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
        // P1.7 fix: use fallback default patterns when config is empty
        let effective_patterns = if patterns.values.is_empty() {
            Self::default_forbidden_bypass()
        } else {
            PatternList {
                values: patterns.values.clone(),
            }
        };

        // P2.4 fix: precompute lowered patterns once per file scan
        // Use to_ascii_lowercase for stable byte offsets (fix #8)
        let lowered_patterns: Vec<String> = effective_patterns
            .iter()
            .map(|p| p.to_ascii_lowercase())
            .collect();

        let language = code_analysis_language_from_file(file);

        // Early bailout scan — checks full lowered line for non-word bypass patterns
        // so comment-based bypasses like `// eslint-disable` are not missed (fix #3)
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
        // Fix #10: track block comment state across lines
        let mut in_block_comment = false;

        while i < lines.len() {
            let t = lines[i].trim();
            let line_number = i + 1;

            // Fix #10: handle block comment state
            if in_block_comment {
                if t.contains("*/") {
                    in_block_comment = false;
                }
                i += 1;
                continue;
            }

            // Skip line comments — documentation references to patterns are not runtime violations
            if t.starts_with("//") {
                i += 1;
                continue;
            }

            // Fix #10: detect start of block comment
            if t.starts_with("/*") {
                if !t.contains("*/") || t.find("*/").unwrap_or(0) < t.find("/*").unwrap_or(0) {
                    in_block_comment = true;
                }
                i += 1;
                continue;
            }

            // Skip doc comment continuation lines
            if t.starts_with('*') {
                i += 1;
                continue;
            }

            // Fix #9: skip test modules — support cfg(all(test, ...)) and similar
            if Self::is_cfg_test_block(t) {
                i = skip_cfg_test_block(&lines, i);
                continue;
            }
            // Skip static Lazy<Regex> multiline initialization blocks
            if t.contains("static ") && t.contains("Lazy") {
                i = skip_brace_block(&lines, i);
                continue;
            }

            let is_comment_line = Self::is_comment_line(t, language);
            let code_portion = Self::code_portion_for_language(t, language);

            // Allow attribute: rustc annotation attributes → BYPASS_COMMENT (always).
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
                    // Word tokens like unwrap/panic must not be reported from comment text
                    if is_comment_line {
                        continue;
                    }

                    // Fix #5: skip if pattern not found
                    let pattern_pos = match code_lower.find(token) {
                        Some(pos) => pos,
                        None => continue,
                    };

                    // Fix #6: pass false for requires_method_call to detect bare unwrap()
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
                    // Non-word patterns: noqa, type: ignore, eslint-disable, etc.
                    // These must be detected even inside comments
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

            // Fix #1: use stable line_number captured before i += 1
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
                    _ => {}
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
    fn is_word_pattern_token(token: &str) -> bool {
        matches!(
            token,
            "unwrap" | "expect" | "panic" | "todo" | "unimplemented" | "unreachable"
        )
    }

    /// Default fallback bypass patterns when config provides none.
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

    /// Fix #7: Check if the line has ONLY safe `.unwrap_or*()` variants.
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
                            return false;
                        }
                        b'_' => {
                            i += 1;
                            let rest = &bytes[i..];
                            // Fix #7: match only known safe variants
                            if rest.starts_with(b"or(")
                                || rest.starts_with(b"or_else(")
                                || rest.starts_with(b"or_default(")
                            {
                                continue;
                            }
                            // Unknown variant — treat as unsafe
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

    /// Fix #9: Detect cfg(test) blocks including cfg(all(test, ...)) variants.
    fn is_cfg_test_block(line: &str) -> bool {
        if line.starts_with("#[cfg(test)]") {
            return true;
        }
        // Support #[cfg(all(test, ...))] and similar patterns
        if line.starts_with("#[cfg(all(") && line.contains("test") {
            return true;
        }
        false
    }

    /// Returns the code portion of a line for language-sensitive checks.
    fn code_portion_for_language(line: &str, language: Language) -> &str {
        match language {
            Language::Python => Self::strip_python_comment(line),
            _ => strip_trailing_comment(line),
        }
    }

    /// Conservative full-line comment detection.
    fn is_comment_line(line: &str, language: Language) -> bool {
        match language {
            Language::Python => line.starts_with('#'),
            Language::Rust | Language::JavaScript | Language::TypeScript => {
                line.starts_with("//") || line.starts_with("/*") || line.starts_with('*')
            }
        }
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

    /// Fix #4: Strip trailing TOML `# ...` comments outside simple string literals.
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

    /// Fix #4: Detect Cargo lint values that effectively silence lints.
    /// Handles: "allow", 'allow', { level = "allow" }, { level = 'allow', priority = -1 }
    fn cargo_value_is_allow(value: &str) -> bool {
        let value = value.trim();
        if value == "\"allow\"" || value == "'allow'" {
            return true;
        }
        let normalized: String = value.chars().filter(|c| !c.is_whitespace()).collect();
        normalized.contains("\"allow\"") || normalized.contains("'allow'")
    }
}
