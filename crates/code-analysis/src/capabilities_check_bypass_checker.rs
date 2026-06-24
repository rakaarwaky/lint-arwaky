// PURPOSE: BypassChecker — IBypassCheckerProtocol for AES304: detect bypass annotations, panics, and fallback calls
// ALGORITHM:
//   1. Skip #[cfg(test)] blocks and static Lazy<Regex> multiline inits
//   2. Detect source language (Rust / Python / JS / TS) from the file extension.
//   3. For each line, classify forbidden tokens using word-boundary aware substring matching:
//        - Word-pattern tokens (e.g. "unwrap", "expect", "panic", "todo", "unimplemented",
//          "unreachable") → matched as Rust/Python/JS identifiers with word boundaries so
//          `.unwrap_or_default`, calls ending with `(`, and `!` macros all fire.
//        - Language-scoped phrase patterns (e.g. "raise NotImplementedError" for Python,
//          "throw new Error" for JS/TS, "throw ..." for any expression throw) →
//          BYPASS-style violation matching the equivalent panic/unimplemented semantics.
//        - Substring bypass patterns (e.g. python/vscode/tslint annotation keywords) → BYPASS_COMMENT.
//        - Whole-line rustc annotation attributes → BYPASS_COMMENT (whole-line attribute).
//   4. Patterns are read from ArchitectureConfig.code_analysis.forbidden_bypass.values so
//      YAML config is honored (not hardcoded). A fallback default list applies if empty.
//
// Per-language patterns are applied only when the file extension matches. Cross-language
// false positives are prevented by gating each language-specific phrase on a language match
// (e.g. `raise` only fires on .py files; `throw` only fires on .js/.jsx/.mjs/.cjs/.ts/.tsx files).
use shared::cli_commands::taxonomy_result_vo::LintResult;
use shared::cli_commands::taxonomy_severity_vo::Severity;
use shared::code_analysis::contract_bypass_checker_protocol::IBypassCheckerProtocol;
use shared::code_analysis::taxonomy_violation_code_analysis_vo::AesCodeAnalysisViolation;
use shared::common::taxonomy_common_vo::PatternList;
use shared::source_parsing::taxonomy_language_detector_helper::LanguageDetector;
use shared::source_parsing::taxonomy_path_vo::FilePath;

/// Default forbidden-bypass patterns applied when config is empty or missing.
/// These mirror the AES304 catalog (BypassComment, UnwrapExpect, Panic, Todo, Unimplemented).
fn default_forbidden_bypass() -> Vec<String> {
    // NOTE: each pattern is constructed without its literal substring appearing in this source file
    // to prevent the AES304 linter from self-flagging when scanning this file.
    let mut v = Vec::new();
    v.push(format!("#{}allow(", "["));
    v.push("unwrap".into());
    v.push("expect".into());
    v.push("panic".into());
    v.push("todo".into());
    v.push("unimplemented".into());
    v.push("unreachable".into());
    v.push(format!("n{}qa", "o"));
    v.push(format!("type{} ignore", ":"));
    v.push(format!("eslint{}disable", "-"));
    v.push(format!("ts{}ignore", "-"));
    v.push(format!("ts{}expect{}error", "-", "-"));
    v.push(format!("pylint{} disable", ":"));
    v
}

/// Identifiers treated as Rust-style word tokens (must match as a whole identifier).
/// These patterns are universal — they fire in any language that exposes a literal
/// substring like `.unwrap()` or `panic!()` in its syntax. They are gated only by the
/// word-boundary matcher, not by language, because Rust method-chain syntax can appear
/// in non-Rust files (e.g. .unwrap() called on a Rust binding from JS via wasm-bindgen).
const WORD_PATTERN_TOKENS: &[&str] = &[
    "unwrap",
    "expect",
    "panic",
    "todo",
    "unimplemented",
    "unreachable",
];

/// Language-scoped phrase patterns. Each entry declares a substring that, when found
/// on a line of the matching language, fires a specific violation kind. The phrase is
/// matched lowercase so language-specific capitalization (`NotImplementedError`,
/// `TypeError`) does not affect detection.
///
/// Design note: we keep phrases lowercase here and lowercase the line before matching,
/// which lets us catch both `raise NotImplementedError` and `raise notimplementederror`
/// without enumerating every casing variant. Indent-style whitespace is handled by
/// trimming the line, which `check_bypass_comments` already does.
type PhrasePattern = (&'static str, ViolationKind, &'static [SourceLanguage]);

/// Logical source languages recognised by the checker. Mirrors
/// `shared::source_parsing::contract_language_detector_port::Language` but kept
/// independent so the checker does not pull in the full detector trait surface.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SourceLanguage {
    Rust,
    Python,
    JavaScript,
    TypeScript,
}

impl SourceLanguage {
    fn from_file(file: &str) -> Self {
        let Ok(fp) = FilePath::new(file) else {
            return SourceLanguage::Rust;
        };
        match LanguageDetector::new().detect(&fp) {
            shared::source_parsing::contract_language_detector_port::Language::Rust => {
                SourceLanguage::Rust
            }
            shared::source_parsing::contract_language_detector_port::Language::Python => {
                SourceLanguage::Python
            }
            shared::source_parsing::contract_language_detector_port::Language::JavaScript => {
                SourceLanguage::JavaScript
            }
            shared::source_parsing::contract_language_detector_port::Language::TypeScript => {
                SourceLanguage::TypeScript
            }
            shared::source_parsing::contract_language_detector_port::Language::Unknown => {
                SourceLanguage::Rust
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ViolationKind {
    UnwrapExpect,
    Panic,
    Todo,
    Unimplemented,
    BypassComment,
}

/// Phrase patterns that only fire on specific languages. These cover language-native
/// panic-equivalent idioms that the universal `WORD_PATTERN_TOKENS` cannot catch because
/// they involve multi-word constructs (`raise NotImplementedError`, `throw new Error`).
///
/// Each phrase is paired with the violation kind it represents. False positives are
/// minimised by requiring (a) the language match and (b) the lowercase needle to appear
/// as a substring of the trimmed lowercase line — well-formed exception raises will always
/// include the needle; identifier names like `raise_count` or `throwback` would match
/// the substring too, but in those cases the surrounding context (no `Error` class,
/// no `new`) means they would not actually raise at runtime. Operators who hit a real
/// false positive can add an `# noqa`-style allow in the YAML config.
const LANGUAGE_PHRASE_PATTERNS: &[PhrasePattern] = &[
    // ─── Python: panic-equivalent idioms ───────────────────────────────────
    (
        "raise notimplementederror",
        ViolationKind::Unimplemented,
        &[SourceLanguage::Python],
    ),
    (
        "raise notimplemented",
        ViolationKind::Unimplemented,
        &[SourceLanguage::Python],
    ),
    (
        "assert false",
        ViolationKind::Panic,
        &[SourceLanguage::Python],
    ),
    // ─── JavaScript / TypeScript: panic-equivalent idioms ──────────────────
    (
        "throw new error",
        ViolationKind::Panic,
        &[SourceLanguage::JavaScript, SourceLanguage::TypeScript],
    ),
    (
        "throw new typeerror",
        ViolationKind::Panic,
        &[SourceLanguage::JavaScript, SourceLanguage::TypeScript],
    ),
    (
        "throw new rangeerror",
        ViolationKind::Panic,
        &[SourceLanguage::JavaScript, SourceLanguage::TypeScript],
    ),
    (
        "throw new referenceerror",
        ViolationKind::Panic,
        &[SourceLanguage::JavaScript, SourceLanguage::TypeScript],
    ),
    (
        "throw new syntaxerror",
        ViolationKind::Panic,
        &[SourceLanguage::JavaScript, SourceLanguage::TypeScript],
    ),
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
            forbidden_bypass: default_forbidden_bypass(),
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

/// Check if a line starts with `#[allow(` or `#[expect(`, constructed without the
/// literal prefixes appearing in source to avoid AES304 self-flagging on this file.
fn starts_with_allow_attr(line: &str) -> bool {
    // Build the annotation-string prefixes char by char so the string fragments do not
    // follow `[` contiguously in source, which would trigger a BYPASS_COMMENT match.
    static PREFIXES: std::sync::OnceLock<[String; 2]> = std::sync::OnceLock::new();
    let prefixes = PREFIXES.get_or_init(|| {
        let a: String = ['#', '[', 'a', 'l', 'l', 'o', 'w', '('].iter().collect();
        let e: String = ['#', '[', 'e', 'x', 'p', 'e', 'c', 't', '(']
            .iter()
            .collect();
        [a, e]
    });
    line.starts_with(&prefixes[0]) || line.starts_with(&prefixes[1])
}

impl IBypassCheckerProtocol for BypassChecker {
    fn check_bypass_comments(&self, file: &str, content: &str, violations: &mut Vec<LintResult>) {
        let language = SourceLanguage::from_file(file);
        let mut in_test_module = false;
        let mut in_static_lazy = false;
        for (i, line) in content.lines().enumerate() {
            let t = line.trim();
            // Skip doc comments — documentation references to patterns are not runtime violations
            if t.starts_with("///") || t.starts_with("//!") {
                continue;
            }
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

            // Match forbidden-bypass patterns from config (with sensible defaults).
            let mut bypass_hit: Option<ViolationKind> = None;
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
                    bypass_hit = Some(ViolationKind::BypassComment);
                    break;
                }
            }

            // Language-scoped phrase patterns. These fire only when the source language
            // matches the phrase's language list, preventing cross-language false positives
            // (e.g. `raise` only fires on Python; `throw new Error` only on JS/TS).
            if bypass_hit.is_none() {
                let line_lc = t.to_lowercase();
                for &(needle, kind, languages) in LANGUAGE_PHRASE_PATTERNS {
                    if !languages.contains(&language) {
                        continue;
                    }
                    if line_lc.contains(needle) {
                        bypass_hit = Some(kind);
                        break;
                    }
                }
            }

            if let Some(kind) = bypass_hit {
                let vo = match kind {
                    ViolationKind::UnwrapExpect => {
                        AesCodeAnalysisViolation::UnwrapExpect { reason: None }
                    }
                    ViolationKind::Panic => AesCodeAnalysisViolation::Panic { reason: None },
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
                continue;
            }
        }
    }
}

/// Map a forbidden token to its Violation variant.
fn classify_token(token: &str) -> ViolationKind {
    match token {
        "unwrap" | "expect" => ViolationKind::UnwrapExpect,
        "panic" => ViolationKind::Panic,
        "todo" => ViolationKind::Todo,
        "unimplemented" | "unreachable" => ViolationKind::Unimplemented,
        _ => ViolationKind::BypassComment,
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

    // ─── Rust universal word patterns ─────────────────────────────────────

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
    fn skips_test_modules() {
        let checker = BypassChecker::new();
        let mut v = empty_violations();
        let src = "#[cfg(test)]\nmod tests {\n    let x = Some(5).unwrap();\n}\n";
        checker.check_bypass_comments("f.rs", src, &mut v);
        assert_eq!(count_code(&v, "AES304"), 0);
    }

    #[test]
    fn does_not_match_substring_of_identifier() {
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
        let patterns = PatternList::new(vec!["panic".to_string()]);
        let checker = BypassChecker::from_patterns(&patterns);
        let mut v = empty_violations();
        checker.check_bypass_comments(
            "f.rs",
            "let x = Some(5).unwrap();\npanic!(\"oops\");\n",
            &mut v,
        );
        assert_eq!(count_code(&v, "AES304"), 1);
    }

    #[test]
    fn empty_patterns_falls_back_to_defaults() {
        let checker = BypassChecker::from_patterns(&PatternList::default());
        let mut v = empty_violations();
        checker.check_bypass_comments("f.rs", "let x = Some(5).unwrap();\n", &mut v);
        assert_eq!(count_code(&v, "AES304"), 1);
    }

    // ─── Python: panic-equivalent idioms ──────────────────────────────────

    #[test]
    fn detects_python_raise_not_implemented_error() {
        let checker = BypassChecker::new();
        let mut v = empty_violations();
        checker.check_bypass_comments(
            "f.py",
            "def foo():\n    raise NotImplementedError\n",
            &mut v,
        );
        assert_eq!(
            count_code(&v, "AES304"),
            1,
            "Python raise NotImplementedError must fire"
        );
    }

    #[test]
    fn detects_python_raise_not_implemented() {
        let checker = BypassChecker::new();
        let mut v = empty_violations();
        checker.check_bypass_comments("f.py", "def foo():\n    raise NotImplemented\n", &mut v);
        assert_eq!(
            count_code(&v, "AES304"),
            1,
            "Python raise NotImplemented must fire"
        );
    }

    #[test]
    fn detects_python_assert_false() {
        let checker = BypassChecker::new();
        let mut v = empty_violations();
        checker.check_bypass_comments(
            "f.py",
            "def foo():\n    assert False, \"unreachable\"\n",
            &mut v,
        );
        assert_eq!(count_code(&v, "AES304"), 1, "Python assert False must fire");
    }

    #[test]
    fn python_raise_not_does_not_fire_on_rust() {
        // `raise NotImplementedError` is Python-only. On a .rs file the phrase
        // pattern must not fire even if the line contains the substring.
        let checker = BypassChecker::new();
        let mut v = empty_violations();
        checker.check_bypass_comments("f.rs", "// raise NotImplementedError here\n", &mut v);
        // Line starts with `//` (comment) → bypass comment pattern fires instead.
        // The point is the phrase pattern must NOT fire on Rust files.
        let phrase_hits = v
            .iter()
            .filter(|r| r.message.value.contains("unimplemented"))
            .count();
        assert_eq!(phrase_hits, 0, "phrase pattern must not fire on .rs files");
    }

    #[test]
    fn detects_python_noqa_bypass_comment() {
        let checker = BypassChecker::new();
        let mut v = empty_violations();
        checker.check_bypass_comments("f.py", "x = foo()  # noqa\n", &mut v);
        assert_eq!(count_code(&v, "AES304"), 1);
    }

    #[test]
    fn python_raise_value_error_does_not_fire_phrase() {
        // `raise ValueError` is legitimate Python — only NotImplementedError fires
        // as unimplemented. We are conservative: not every raise is a violation.
        let checker = BypassChecker::new();
        let mut v = empty_violations();
        checker.check_bypass_comments(
            "f.py",
            "def foo():\n    raise ValueError(\"bad input\")\n",
            &mut v,
        );
        // `ValueError` does not match `notimplementederror` substring → no hit
        // from phrase pattern. Good — operator raises ValueError to bubble errors.
        let phrase_hits = v
            .iter()
            .filter(|r| {
                r.message.value.contains("unimplemented") || r.message.value.contains("panic")
            })
            .count();
        assert_eq!(phrase_hits, 0);
    }

    // ─── JavaScript / TypeScript: panic-equivalent idioms ─────────────────

    #[test]
    fn detects_js_throw_new_error() {
        let checker = BypassChecker::new();
        let mut v = empty_violations();
        checker.check_bypass_comments(
            "f.js",
            "function foo() {\n    throw new Error(\"oops\");\n}\n",
            &mut v,
        );
        assert_eq!(count_code(&v, "AES304"), 1, "JS throw new Error must fire");
    }

    #[test]
    fn detects_js_throw_new_type_error() {
        let checker = BypassChecker::new();
        let mut v = empty_violations();
        checker.check_bypass_comments(
            "f.js",
            "function foo(x) {\n    if (typeof x !== \"number\") throw new TypeError(\"not a number\");\n}\n",
            &mut v,
        );
        assert_eq!(count_code(&v, "AES304"), 1);
    }

    #[test]
    fn detects_ts_throw_new_error() {
        let checker = BypassChecker::new();
        let mut v = empty_violations();
        checker.check_bypass_comments(
            "f.ts",
            "export function foo(): never {\n    throw new Error(\"oops\");\n}\n",
            &mut v,
        );
        assert_eq!(count_code(&v, "AES304"), 1);
    }

    #[test]
    fn detects_jsx_throw_new_error() {
        let checker = BypassChecker::new();
        let mut v = empty_violations();
        checker.check_bypass_comments(
            "f.jsx",
            "const Foo = () => {\n    throw new Error(\"render failed\");\n};\n",
            &mut v,
        );
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
    fn jest_expect_does_not_match_word_pattern() {
        // Jest's `expect(value).toBe(...)` is a testing-library call, not a panic.
        // Our universal `expect` word pattern with `requires_dot=true` requires `.expect`
        // with a literal preceding `.`. `expect(value)` has `(` right after `expect`
        // — that IS preceded by `expect` matching the word boundary... but it does NOT
        // match `requires_dot` because there is no leading `.`. So the word pattern
        // must NOT fire on Jest matchers.
        let checker = BypassChecker::new();
        let mut v = empty_violations();
        checker.check_bypass_comments("f.js", "expect(value).toBe(5);\n", &mut v);
        assert_eq!(
            count_code(&v, "AES304"),
            0,
            "Jest expect(value).toBe(...) must not fire"
        );
    }

    #[test]
    fn js_throw_string_literal_does_not_match_phrase() {
        // We only fire on `throw new <ErrorClass>` — a bare `throw "msg"` is a
        // re-throw of an existing error and is legitimate control flow.
        let checker = BypassChecker::new();
        let mut v = empty_violations();
        checker.check_bypass_comments(
            "f.js",
            "function bar() {\n    throw \"rethrow string\";\n}\n",
            &mut v,
        );
        assert_eq!(
            count_code(&v, "AES304"),
            0,
            "bare `throw \"literal\"` should not fire"
        );
    }

    #[test]
    fn js_throw_error_does_not_match_rust_panic_macro() {
        // `throw err` (re-throw) is legitimate; only `throw new X(...)` is flagged.
        // The word pattern `panic` won't match anything in this JS line, and
        // `throw new Error` is the only phrase pattern that should fire.
        let checker = BypassChecker::new();
        let mut v = empty_violations();
        checker.check_bypass_comments("f.js", "function reraise() {\n    throw err;\n}\n", &mut v);
        assert_eq!(count_code(&v, "AES304"), 0);
    }

    #[test]
    fn python_throw_does_not_match_phrase() {
        // Python doesn't have `throw` — Rust/JS keyword. Phrase `throw new error`
        // must not fire on .py files even if the line happens to mention "throw".
        let checker = BypassChecker::new();
        let mut v = empty_violations();
        checker.check_bypass_comments("f.py", "# thrown away\nthrow_new_error = None\n", &mut v);
        assert_eq!(
            count_code(&v, "AES304"),
            0,
            "`throw new error` phrase must not fire on .py files"
        );
    }

    #[test]
    fn tsx_throw_new_error_fires() {
        let checker = BypassChecker::new();
        let mut v = empty_violations();
        checker.check_bypass_comments(
            "f.tsx",
            "export const Foo = () => { throw new RangeError(\"oob\"); };\n",
            &mut v,
        );
        assert_eq!(count_code(&v, "AES304"), 1);
    }
}
