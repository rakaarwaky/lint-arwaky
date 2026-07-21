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
    matches_word_token, skip_brace_block, skip_cfg_test_block, starts_with_allow_attr,
};
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
                    if val == "\"allow\"" || val == "'allow'" {
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
            PatternList { values: patterns.values.clone() }
        };

        // P2.5 fix: line-by-line scan instead of allocating full lowercase copy
        let has_bypass_token = content.lines().any(|line| {
            let lc = line.to_lowercase();
            effective_patterns
                .iter()
                .any(|p| lc.contains(p.as_str()))
                || lc.contains("raise ")
                || lc.contains("throw new")
        });
        if !has_bypass_token {
            return;
        }

        // P2.4 fix: precompute lowered patterns once per file scan
        let lowered_patterns: Vec<String> =
            effective_patterns.iter().map(|p| p.to_lowercase()).collect();

        let language = Language::from_file(file);
        let lines: Vec<&str> = content.lines().collect();
        let mut i = 0;
        while i < lines.len() {
            let t = lines[i].trim();

            // Skip doc comments — documentation references to patterns are not runtime violations
            if t.starts_with("///") || t.starts_with("//!") {
                i += 1;
                continue;
            }
            // Skip test modules — unwrap/panic is normal in tests
            if t.starts_with("#[cfg(test)]") {
                i = skip_cfg_test_block(&lines, i);
                continue;
            }
            // Skip static Lazy<Regex> multiline initialization blocks
            if t.contains("static ") && t.contains("Lazy") {
                i = skip_brace_block(&lines, i);
                continue;
            }

            // Allow attribute: rustc annotation attributes → BYPASS_COMMENT (always).
            if starts_with_allow_attr(t) {
                violations.push(LintResult::new_arch(
                    file,
                    i + 1,
                    "AES304",
                    Severity::CRITICAL,
                    AesCodeAnalysisViolation::BypassComment { reason: None }.to_string(),
                ));
                i += 1;
                continue;
            }

            // Use guarded match arms (P1.2 fix: eliminates clippy::collapsible_match)
            let t_lower = t.to_lowercase();
            for (idx, _p) in lowered_patterns.iter().enumerate() {
                let p_str = &effective_patterns.values[idx];
                let requires_method_call = matches!(p_str.as_str(), "unwrap" | "expect");

                match p_str.as_str() {
                    p if Self::is_word_pattern_token(p)
                        && matches_word_token(t, p, requires_method_call)
                        && !(p == "unwrap" && Self::has_safe_unwrap_variant(t)) =>
                    {
                        let vo = match Self::classify_token(p) {
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
                            i + 1,
                            "AES304",
                            Severity::CRITICAL,
                            vo.to_string(),
                        ));
                        break;
                    }

                    p if !Self::is_word_pattern_token(p)
                        && !p.is_empty()
                        && t_lower.contains(p) =>
                    {
                        violations.push(LintResult::new_arch(
                            file,
                            i + 1,
                            "AES304",
                            Severity::CRITICAL,
                            AesCodeAnalysisViolation::BypassComment { reason: None }.to_string(),
                        ));
                        break;
                    }

                    _ => {}
                }
            }
            i += 1;

            // Language-scoped phrase patterns (P1.9: only after main pattern match).
            let line_lc = t.to_lowercase();
            match language {
                Language::Python => {
                    if line_lc.contains("raise notimplementederror")
                        || line_lc.contains("raise notimplemented")
                    {
                        violations.push(LintResult::new_arch(
                            file,
                            i + 1,
                            "AES304",
                            Severity::CRITICAL,
                            AesCodeAnalysisViolation::Unimplemented { reason: None }.to_string(),
                        ));
                    } else if line_lc.contains("assert false") {
                        violations.push(LintResult::new_arch(
                            file,
                            i + 1,
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
                    if throw_patterns
                        .iter()
                        .any(|p| line_lc.contains(p))
                    {
                        violations.push(LintResult::new_arch(
                            file,
                            i + 1,
                            "AES304",
                            Severity::CRITICAL,
                            AesCodeAnalysisViolation::Panic { reason: None }.to_string(),
                        ));
                    }
                }
                _ => {} // Rust handled above via config patterns
            }
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

    /// P1.2 fix: Tokens that require call-site style matching rather than plain contains.
    fn is_word_pattern_token(token: &str) -> bool {
        matches!(
            token,
            "unwrap" | "expect" | "panic" | "todo" | "unimplemented" | "unreachable"
        )
    }

    /// P1.7 fix: Default fallback bypass patterns when config provides none.
    fn default_forbidden_bypass() -> PatternList {
        use shared::common::taxonomy_common_vo::PatternList;
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

    /// P1.9 fix: Check if the line has ONLY safe `.unwrap_or*()` variants (no unsafe `.unwrap(`).
    /// Returns true if all `.unwrap` calls on this line are safe variants like `.unwrap_or()`,
    /// `.unwrap_or_else()`, `.unwrap_or_default()`. Returns false if unsafe `.unwrap(` is found.
    fn has_safe_unwrap_variant(line: &str) -> bool {
        let bytes = line.as_bytes();
        let len = bytes.len();
        let mut i = 0;
        while i < len {
            // Find ".unwrap" occurrences
            if bytes[i..].starts_with(b".unwrap") && (i == 0 || !b_is_ident(bytes[i - 1])) {
                i += 7; // skip past ".unwrap"
                // Check if followed by '(', '!', or '_' (method call)
                if i < len {
                    match bytes[i] {
                        b'(' | b'!' => {
                            // This is an unsafe .unwrap() call — return false
                            return false;
                        }
                        b'_' => {
                            // Check if it's a safe variant: unwrap_or, unwrap_or_else, unwrap_or_default
                            i += 1;
                            let rest = &bytes[i..];
                            if rest.starts_with(b"or") || rest.starts_with(b"Or") {
                                // Safe variant — continue checking for more .unwrap calls
                                continue;
                            }
                            // Unknown variant — treat as unsafe
                            return false;
                        }
                        _ => {
                            // Not a method call — just a variable/field name containing "unwrap"
                            i += 1;
                            continue;
                        }
                    }
                }
            }
            i += 1;
        }
        // No unsafe .unwrap() found — all are safe variants
        true
    }
}

fn b_is_ident(b: u8) -> bool {
    b.is_ascii_alphanumeric() || b == b'_'
}
