// PURPOSE: BypassChecker — IBypassCheckerProtocol for AES304: detect #[allow], noqa, unwrap, panic
// ALGORITHM:
//   1. Skip #[cfg(test)] blocks and static Lazy<Regex> multiline inits
//   2. For each line, classify forbidden tokens using word-boundary aware substring matching:
//        - Substring patterns (e.g. "noqa", "type: ignore", "eslint-disable", "ts-ignore",
//          "ts-expect-error", "pylint: disable") → BYPASS_COMMENT
//        - Word-pattern tokens (e.g. "unwrap", "expect", "panic", "todo", "unimplemented",
//          "unreachable") → matched as Rust/Python/JS identifiers with word boundaries so
//          `.unwrap_or_default`, `.expect("msg")`, `panic!`, `unreachable!` all fire.
//        - `#[allow(` / `#[expect(` → BYPASS_COMMENT (whole-line attribute).
//   3. Patterns are read from ArchitectureConfig.code_analysis.forbidden_bypass.values so
//      YAML config is honored (not hardcoded). A fallback default list applies if empty.
use shared::cli_commands::taxonomy_result_vo::LintResult;
use shared::cli_commands::taxonomy_severity_vo::Severity;
use shared::code_analysis::contract_bypass_checker_protocol::IBypassCheckerProtocol;
use shared::code_analysis::taxonomy_violation_code_analysis_vo::AesCodeAnalysisViolation;
use shared::common::taxonomy_common_vo::PatternList;

/// Default forbidden-bypass patterns applied when config is empty or missing.
/// These mirror the AES304 catalog (BypassComment, UnwrapExpect, Panic, Todo, Unimplemented).
const DEFAULT_FORBIDDEN_BYPASS: &[&str] = &[
    "#[allow(",
    "unwrap",
    "expect",
    "panic",
    "todo",
    "unimplemented",
    "unreachable",
    "noqa",
    "type: ignore",
    "eslint-disable",
    "ts-ignore",
    "ts-expect-error",
    "pylint: disable",
];

/// Identifiers treated as Rust-style word tokens (must match as a whole identifier).
/// This lets `.unwrap_or_default`, `.expect("msg")`, `panic!("..")`, `unreachable!()` all
/// fire even though their textual form differs from the bare pattern.
const WORD_PATTERN_TOKENS: &[&str] = &[
    "unwrap",
    "expect",
    "panic",
    "todo",
    "unimplemented",
    "unreachable",
];

pub struct BypassChecker {
    forbidden_bypass: Vec<String>,
}

impl Default for BypassChecker {
    fn default() -> Self {
        Self::new()
    }
}

impl BypassChecker {
    pub fn new() -> Self {
        Self {
            forbidden_bypass: DEFAULT_FORBIDDEN_BYPASS
                .iter()
                .map(|s| s.to_string())
                .collect(),
        }
    }

    /// Build a BypassChecker from an ArchitectureConfig-derived PatternList.
    /// Falls back to defaults if the pattern list is empty.
    pub fn from_patterns(patterns: &PatternList) -> Self {
        if patterns.values.is_empty() {
            return Self::new();
        }
        Self {
            forbidden_bypass: patterns.values.clone(),
        }
    }

    /// Returns true if `line` (already trimmed) contains `token` invoked as a method call
    /// or macro. Rejects bare identifier-name usage.
    ///
    /// Two flavors of match are supported per token:
    ///   * `requires_method_call`: token must be preceded by `.` (or be at start-of-line
    ///     immediately followed by `panic!`/`todo!`/etc macro syntax). Prevents
    ///     `unwrap_helper` from firing.
    ///   * Word-boundary match: token preceded by non-identifier-start char AND followed by
    ///     a non-identifier-start char (handles `panic!("..")`, `unreachable!()`).
    ///
    /// For method-call tokens (`unwrap`, `expect`) we follow the chain across `_segment_`
    /// boundaries (`unwrap_or_default`, `expect_err`) and require the chain to terminate
    /// in `(` (immediate call) or `!` (panic-style) — never bare identifier like
    /// `unwrap_helper`.
    fn matches_word_token(line: &str, token: &str, requires_method_call: bool) -> bool {
        if token.is_empty() {
            return false;
        }
        let bytes = line.as_bytes();
        let token_bytes = token.as_bytes();
        let tlen = token_bytes.len();
        if bytes.len() < tlen {
            return false;
        }
        let mut i = 0;
        while i + tlen <= bytes.len() {
            if &bytes[i..i + tlen] == token_bytes {
                let before_ok = i == 0 || !is_ident_start(bytes[i - 1]);
                if !before_ok {
                    i += 1;
                    continue;
                }
                // Method-call requirement: preceded by `.`
                if requires_method_call {
                    let preceded_by_dot = i > 0 && bytes[i - 1] == b'.';
                    if !preceded_by_dot {
                        i += 1;
                        continue;
                    }
                }
                // Walk the chain of `_segment` to find a terminating `(` or `!`.
                // Each iteration expects: optional `_` separator, then identifier segment,
                // then either `(`/`!` (match) or `_` (continue chain) or anything else
                // (reject). Bare `unwrap_helper` (no `_` after `helper`) is rejected.
                let mut j = i + tlen;
                loop {
                    if j >= bytes.len() {
                        return false;
                    }
                    let sep = bytes[j];
                    // We expect exactly `_` between segments.
                    if sep != b'_' {
                        // Could still be `(` / `!` immediately after the token.
                        if (sep == b'(' || sep == b'!') && j == i + tlen {
                            return true;
                        }
                        return false;
                    }
                    j += 1; // consume `_`
                    if j >= bytes.len() {
                        return false;
                    }
                    // Consume one identifier segment (must start with letter/_).
                    if !is_ident_start(bytes[j]) {
                        return false;
                    }
                    j += 1;
                    while j < bytes.len() && is_ident_continue(bytes[j]) {
                        j += 1;
                    }
                    // After segment: terminator or `_` to continue, otherwise reject.
                    if j >= bytes.len() {
                        return false;
                    }
                    let after_seg = bytes[j];
                    if after_seg == b'(' || after_seg == b'!' {
                        return true;
                    }
                    if after_seg != b'_' {
                        return false;
                    }
                    // Continue loop with j still on `_` so next iteration consumes it.
                }
            }
            i += 1;
        }
        false
    }
}

fn is_ident_continue(b: u8) -> bool {
    b.is_ascii_alphanumeric() || b == b'_'
}

fn is_ident_start(b: u8) -> bool {
    // Identifiers begin with [A-Za-z_] and continue with [A-Za-z0-9_].
    // We treat only alphabetic characters and `_` as identifier starters.
    b.is_ascii_alphabetic() || b == b'_'
}

impl IBypassCheckerProtocol for BypassChecker {
    fn check_bypass_comments(&self, file: &str, content: &str, violations: &mut Vec<LintResult>) {
        let mut in_test_module = false;
        let mut in_static_lazy = false;
        for (i, line) in content.lines().enumerate() {
            let t = line.trim();
            // Skip test modules — unwrap/panic is normal in tests
            if t.starts_with("#[cfg(test)]") {
                in_test_module = true;
                continue;
            }
            if in_test_module {
                continue;
            }
            // Skip static Lazy<Regex> initialization (multiline)
            if t.starts_with("static ") && t.contains("Lazy") {
                in_static_lazy = true;
                continue;
            }
            if in_static_lazy {
                if t.contains("});") {
                    in_static_lazy = false;
                }
                continue;
            }

            // Allow attribute: #[allow(...)] / #[expect(...)] → BYPASS_COMMENT (always).
            if t.starts_with("#[allow(") || t.starts_with("#[expect(") {
                violations.push(LintResult::new_arch(
                    file,
                    i + 1,
                    "AES304",
                    Severity::CRITICAL,
                    AesCodeAnalysisViolation::BypassComment { reason: None }.to_string(),
                ));
                continue;
            }

            // Match forbidden-bypass patterns from config (with sensible defaults).
            let mut bypass_hit: Option<&'static str> = None;
            for p in &self.forbidden_bypass {
                let p_str = p.as_str();
                if WORD_PATTERN_TOKENS.contains(&p_str) {
                    // `unwrap` and `expect` are method names — require `.` prefix.
                    // `panic`/`todo`/`unimplemented`/`unreachable` are macros — require
                    // `!` suffix (caught below) or word-boundary match.
                    let requires_dot = matches!(p_str, "unwrap" | "expect");
                    if Self::matches_word_token(t, p_str, requires_dot) {
                        bypass_hit = Some(classify_token(p_str));
                        break;
                    }
                } else if !p_str.is_empty() && t.to_lowercase().contains(&p_str.to_lowercase()) {
                    bypass_hit = Some("BypassComment");
                    break;
                }
            }

            if let Some(kind) = bypass_hit {
                let vo = match kind {
                    "UnwrapExpect" => AesCodeAnalysisViolation::UnwrapExpect { reason: None },
                    "Panic" => AesCodeAnalysisViolation::Panic { reason: None },
                    "Todo" => AesCodeAnalysisViolation::Todo { reason: None },
                    "Unimplemented" => AesCodeAnalysisViolation::Unimplemented { reason: None },
                    _ => AesCodeAnalysisViolation::BypassComment { reason: None },
                };
                violations.push(LintResult::new_arch(
                    file,
                    i + 1,
                    "AES304",
                    Severity::CRITICAL,
                    vo.to_string(),
                ));
                continue;
            }
        }
    }
}

/// Map a forbidden token to its Violation variant name.
fn classify_token(token: &str) -> &'static str {
    match token {
        "unwrap" | "expect" => "UnwrapExpect",
        "panic" => "Panic",
        "todo" => "Todo",
        "unimplemented" | "unreachable" => "Unimplemented",
        _ => "BypassComment",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn empty_violations() -> Vec<LintResult> {
        Vec::new()
    }

    fn count_code(violations: &[LintResult], code: &str) -> usize {
        violations.iter().filter(|v| v.code.code() == code).count()
    }

    #[test]
    fn detects_bare_unwrap() {
        let checker = BypassChecker::new();
        let mut v = empty_violations();
        checker.check_bypass_comments("f.rs", "let x = Some(5).unwrap();\n", &mut v);
        assert_eq!(count_code(&v, "AES304"), 1);
    }

    #[test]
    fn detects_unwrap_or_default() {
        let checker = BypassChecker::new();
        let mut v = empty_violations();
        checker.check_bypass_comments("f.rs", "let x = fs::read(p).unwrap_or_default();\n", &mut v);
        assert_eq!(
            count_code(&v, "AES304"),
            1,
            ".unwrap_or_default must be detected"
        );
    }

    #[test]
    fn detects_unwrap_or() {
        let checker = BypassChecker::new();
        let mut v = empty_violations();
        checker.check_bypass_comments("f.rs", "let x = opt.unwrap_or(0);\n", &mut v);
        assert_eq!(count_code(&v, "AES304"), 1);
    }

    #[test]
    fn detects_unwrap_or_else() {
        let checker = BypassChecker::new();
        let mut v = empty_violations();
        checker.check_bypass_comments("f.rs", "let x = opt.unwrap_or_else(|| 0);\n", &mut v);
        assert_eq!(count_code(&v, "AES304"), 1);
    }

    #[test]
    fn detects_bare_expect() {
        let checker = BypassChecker::new();
        let mut v = empty_violations();
        checker.check_bypass_comments("f.rs", "let x = Some(5).expect(\"msg\");\n", &mut v);
        assert_eq!(count_code(&v, "AES304"), 1);
    }

    #[test]
    fn detects_panic_macro() {
        let checker = BypassChecker::new();
        let mut v = empty_violations();
        checker.check_bypass_comments("f.rs", "panic!(\"oops\");\n", &mut v);
        assert_eq!(count_code(&v, "AES304"), 1);
    }

    #[test]
    fn detects_todo_macro() {
        let checker = BypassChecker::new();
        let mut v = empty_violations();
        checker.check_bypass_comments("f.rs", "todo!();\n", &mut v);
        assert_eq!(count_code(&v, "AES304"), 1);
    }

    #[test]
    fn detects_unimplemented_macro() {
        let checker = BypassChecker::new();
        let mut v = empty_violations();
        checker.check_bypass_comments("f.rs", "unimplemented!();\n", &mut v);
        assert_eq!(count_code(&v, "AES304"), 1);
    }

    #[test]
    fn detects_unreachable_macro() {
        let checker = BypassChecker::new();
        let mut v = empty_violations();
        checker.check_bypass_comments("f.rs", "unreachable!();\n", &mut v);
        assert_eq!(count_code(&v, "AES304"), 1);
    }

    #[test]
    fn detects_allow_attribute() {
        let checker = BypassChecker::new();
        let mut v = empty_violations();
        checker.check_bypass_comments("f.rs", "#[allow(unused)]\nfn x() {}\n", &mut v);
        assert_eq!(count_code(&v, "AES304"), 1);
    }

    #[test]
    fn detects_noqa_python() {
        let checker = BypassChecker::new();
        let mut v = empty_violations();
        checker.check_bypass_comments("f.py", "x = foo()  # noqa\n", &mut v);
        assert_eq!(count_code(&v, "AES304"), 1);
    }

    #[test]
    fn detects_eslint_disable_js() {
        let checker = BypassChecker::new();
        let mut v = empty_violations();
        checker.check_bypass_comments("f.js", "// eslint-disable-next-line\nvar x = 1;\n", &mut v);
        assert_eq!(count_code(&v, "AES304"), 1);
    }

    #[test]
    fn skips_test_modules() {
        let checker = BypassChecker::new();
        let mut v = empty_violations();
        let src = "#[cfg(test)]\nmod tests {\n    let x = Some(5).unwrap();\n}\n";
        checker.check_bypass_comments("f.rs", src, &mut v);
        assert_eq!(count_code(&v, "AES304"), 0);
    }

    #[test]
    fn does_not_match_substring_of_identifier() {
        // `expectation` should NOT fire — `expect` is only a substring, not a word.
        let checker = BypassChecker::new();
        let mut v = empty_violations();
        checker.check_bypass_comments("f.rs", "let expectation = 5;\n", &mut v);
        assert_eq!(
            count_code(&v, "AES304"),
            0,
            "identifier 'expectation' must not match 'expect'"
        );
    }

    #[test]
    fn does_not_match_unwrap_in_identifier_name() {
        // `unwrap_helper` is a local variable name, NOT a `.unwrap()` call.
        let checker = BypassChecker::new();
        let mut v = empty_violations();
        checker.check_bypass_comments(
            "f.rs",
            "let unwrap_helper = Some(5);\nlet _x = unwrap_helper;\n",
            &mut v,
        );
        assert_eq!(
            count_code(&v, "AES304"),
            0,
            "bare identifier 'unwrap_helper' must not fire"
        );
    }

    #[test]
    fn honors_config_patterns() {
        // Caller supplies only the patterns they want enforced.
        let patterns = PatternList::new(vec!["panic".to_string()]);
        let checker = BypassChecker::from_patterns(&patterns);
        let mut v = empty_violations();
        checker.check_bypass_comments(
            "f.rs",
            "let x = Some(5).unwrap();\npanic!(\"oops\");\n",
            &mut v,
        );
        // Only `panic!` should fire because config didn't list `unwrap`.
        assert_eq!(count_code(&v, "AES304"), 1);
    }

    #[test]
    fn empty_patterns_falls_back_to_defaults() {
        let checker = BypassChecker::from_patterns(&PatternList::default());
        let mut v = empty_violations();
        checker.check_bypass_comments("f.rs", "let x = Some(5).unwrap();\n", &mut v);
        assert_eq!(count_code(&v, "AES304"), 1);
    }
}
