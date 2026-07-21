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
use shared::code_analysis::taxonomy_violation_code_analysis_vo::{AesCodeAnalysisViolation, Language, ViolationKind};
use shared::code_analysis::utility_bypass::{matches_word_token, starts_with_allow_attr};
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
        // Early-exit: skip scan if file contains none of the bypass-related tokens
        let content_lower = content.to_lowercase();
        let has_bypass_token = patterns
            .values
            .iter()
            .any(|p| content_lower.contains(p.as_str()))
            || content_lower.contains("raise ")
            || content_lower.contains("throw new");
        if !has_bypass_token {
            return;
        }

        let language = Language::from_file(file);
        for (i, line) in content.lines().enumerate() {
            let t = line.trim();
            // Skip doc comments — documentation references to patterns are not runtime violations
            if t.starts_with("///") || t.starts_with("//!") {
                continue;
            }
            // Skip test modules — unwrap/panic is normal in tests
            if t.starts_with("#[cfg(test)]") {
                continue;
            }
            // Skip static Lazy<Regex> initialization (multiline)
            if t.contains("static ") && t.contains("Lazy") {
                let depth = t.matches('{').count() as i32 - t.matches('}').count() as i32;
                let mut d = depth;
                for subsequent_line in content.lines().skip(i + 1) {
                    let st = subsequent_line.trim();
                    d += st.matches('{').count() as i32 - st.matches('}').count() as i32;
                    if d <= 0 {
                        break;
                    }
                }
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
                continue;
            }

            // Match forbidden-bypass patterns from config.
            let t_lower = t.to_lowercase();
            for p in &patterns.values {
                let p_str = p.as_str();
                if matches!(p_str, "unwrap" | "expect" | "panic" | "todo" | "unimplemented" | "unreachable") {
                    if matches_word_token(t, p_str, matches!(p_str, "unwrap" | "expect")) {
                        // Safe .unwrap_or*() variants don't panic — skip.
                        if p_str == "unwrap" && t.contains(".unwrap_or") {
                            continue;
                        }
                        let vo = match Self::classify_token(p_str) {
                            ViolationKind::UnwrapExpect => {
                                AesCodeAnalysisViolation::UnwrapExpect { reason: None }
                            }
                            ViolationKind::Panic => {
                                AesCodeAnalysisViolation::Panic { reason: None }
                            }
                            ViolationKind::Todo => {
                                AesCodeAnalysisViolation::Todo { reason: None }
                            }
                            ViolationKind::Unimplemented => {
                                AesCodeAnalysisViolation::Unimplemented { reason: None }
                            }
                            ViolationKind::BypassComment => {
                                AesCodeAnalysisViolation::BypassComment { reason: None }
                            }
                        };
                        violations.push(LintResult::new_arch(
                            file, i + 1, "AES304", Severity::CRITICAL, vo.to_string(),
                        ));
                        break;
                    }
                } else if !p_str.is_empty() && t_lower.contains(&p_str.to_lowercase()) {
                    violations.push(LintResult::new_arch(
                        file, i + 1, "AES304", Severity::CRITICAL,
                        AesCodeAnalysisViolation::BypassComment { reason: None }.to_string(),
                    ));
                    break;
                }
            }

            // Language-scoped phrase patterns.
            let line_lc = t.to_lowercase();
            match language {
                Language::Python => {
                    if line_lc.contains("raise notimplementederror") || line_lc.contains("raise notimplemented") {
                        violations.push(LintResult::new_arch(
                            file, i + 1, "AES304", Severity::CRITICAL,
                            AesCodeAnalysisViolation::Unimplemented { reason: None }.to_string(),
                        ));
                    } else if line_lc.contains("assert false") {
                        violations.push(LintResult::new_arch(
                            file, i + 1, "AES304", Severity::CRITICAL,
                            AesCodeAnalysisViolation::Panic { reason: None }.to_string(),
                        ));
                    }
                }
                Language::JavaScript | Language::TypeScript => {
                    if line_lc.contains("throw new error") || line_lc.contains("throw new typeerror")
                        || line_lc.contains("throw new rangeerror") || line_lc.contains("throw new referenceerror")
                        || line_lc.contains("throw new syntaxerror") {
                        violations.push(LintResult::new_arch(
                            file, i + 1, "AES304", Severity::CRITICAL,
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
}      
 