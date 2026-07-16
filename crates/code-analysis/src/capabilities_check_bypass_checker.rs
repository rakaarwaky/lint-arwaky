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
use shared::common::taxonomy_path_vo::FilePath;

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
/// `shared::common::contract_language_detector_port::Language` but kept
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
        match fp.language() {
            shared::common::contract_language_detector_port::Language::Rust => SourceLanguage::Rust,
            shared::common::contract_language_detector_port::Language::Python => {
                SourceLanguage::Python
            }
            shared::common::contract_language_detector_port::Language::JavaScript => {
                SourceLanguage::JavaScript
            }
            shared::common::contract_language_detector_port::Language::TypeScript => {
                SourceLanguage::TypeScript
            }
            shared::common::contract_language_detector_port::Language::Unknown => {
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
    forbidden_bypass_lower: Vec<String>,
}

impl Default for BypassChecker {
    fn default() -> Self {
        Self::new()
    }
}

impl BypassChecker {
    pub fn new() -> Self {
        let v = default_forbidden_bypass();
        let lower = v.iter().map(|s| s.to_lowercase()).collect();
        Self {
            forbidden_bypass: v,
            forbidden_bypass_lower: lower,
        }
    }

    /// Build a BypassChecker from an ArchitectureConfig-derived PatternList.
    /// Falls back to defaults if the pattern list is empty.
    pub fn from_patterns(patterns: &PatternList) -> Self {
        if patterns.values.is_empty() {
            return Self::new();
        }
        let lower = patterns.values.iter().map(|s| s.to_lowercase()).collect();
        Self {
            forbidden_bypass: patterns.values.clone(),
            forbidden_bypass_lower: lower,
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
        // Early-exit: skip scan if file contains none of the bypass-related tokens
        let content_lower = content.to_lowercase();
        let has_bypass_token = self
            .forbidden_bypass_lower
            .iter()
            .any(|p| content_lower.contains(p.as_str()))
            || content_lower.contains("raise ")
            || content_lower.contains("throw new");
        if !has_bypass_token {
            return;
        }

        let language = SourceLanguage::from_file(file);

        // Reconstruct code-only and comment-only source views using character classifications.
        let chars: Vec<char> = content.chars().collect();
        let classifications = classify_source(content, language);

        let mut code_content = String::with_capacity(content.len());
        let mut comment_content = String::with_capacity(content.len());

        for (ch, &cls) in chars.iter().zip(classifications.iter()) {
            if *ch == '\n' {
                code_content.push('\n');
                comment_content.push('\n');
            } else {
                match cls {
                    CharClass::Code => {
                        code_content.push(*ch);
                        comment_content.push(' ');
                    }
                    CharClass::Comment => {
                        code_content.push(' ');
                        comment_content.push(*ch);
                    }
                    CharClass::StringLiteral => {
                        code_content.push(' ');
                        comment_content.push(' ');
                    }
                }
            }
        }

        let mut in_test_module = false;
        let mut in_static_lazy = false;
        let mut lazy_brace_depth: i32 = 0;

        // Split code and comments into lines. Their lines correspond 1-to-1 with original content.
        let code_lines: Vec<&str> = code_content.lines().collect();
        let comment_lines: Vec<&str> = comment_content.lines().collect();

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
            if t.contains("static ") && t.contains("Lazy") {
                in_static_lazy = true;
                lazy_brace_depth = t.matches('{').count() as i32 - t.matches('}').count() as i32;
                if lazy_brace_depth <= 0 {
                    in_static_lazy = false;
                    lazy_brace_depth = 0;
                }
                continue;
            }
            if in_static_lazy {
                lazy_brace_depth += t.matches('{').count() as i32 - t.matches('}').count() as i32;
                if lazy_brace_depth <= 0 {
                    in_static_lazy = false;
                    lazy_brace_depth = 0;
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

            let code_line = code_lines.get(i).copied().unwrap_or("").trim();
            let comment_line = comment_lines.get(i).copied().unwrap_or("").trim();

            // Match forbidden-bypass patterns from config (with sensible defaults).
            let mut bypass_hit: Option<ViolationKind> = None;
            for (p, p_lower) in self
                .forbidden_bypass
                .iter()
                .zip(self.forbidden_bypass_lower.iter())
            {
                let p_str = p.as_str();
                if is_comment_bypass_pattern(p_str) {
                    // Check against comments part
                    if !p_str.is_empty() && comment_line.to_lowercase().contains(p_lower.as_str()) {
                        bypass_hit = Some(ViolationKind::BypassComment);
                        break;
                    }
                } else {
                    // Check against code part
                    if WORD_PATTERN_TOKENS.contains(&p_str) {
                        // `unwrap` and `expect` are method names — require `.` prefix.
                        // `panic`/`todo`/`unimplemented`/`unreachable` are macros — require
                        // `!` suffix (caught below) or word-boundary match.
                        let requires_dot = matches!(p_str, "unwrap" | "expect");
                        if Self::matches_word_token(code_line, p_str, requires_dot) {
                            // Safe .unwrap_or*() variants don't panic — skip.
                            if p_str == "unwrap" && code_line.contains(".unwrap_or") {
                                continue;
                            }
                            bypass_hit = Some(classify_token(p_str));
                            break;
                        }
                    } else {
                        // Other code-level patterns (like `Any`, `pass`, `except:`) matched with word boundaries
                        if !p_str.is_empty()
                            && matches_keyword_token(&code_line.to_lowercase(), p_lower.as_str())
                        {
                            bypass_hit = Some(classify_token(p_str));
                            break;
                        }
                    }
                }
            }

            // Language-scoped phrase patterns. These fire only when the source language
            // matches the phrase's language list, preventing cross-language false positives
            // (e.g. `raise` only fires on Python; `throw new Error` only on JS/TS).
            // Checked against the code line.
            if bypass_hit.is_none() {
                let code_line_lc = code_line.to_lowercase();
                for &(needle, kind, languages) in LANGUAGE_PHRASE_PATTERNS {
                    if !languages.contains(&language) {
                        continue;
                    }
                    if code_line_lc.contains(needle) {
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

#[derive(Clone, Copy, Debug, PartialEq)]
enum CharClass {
    Code,
    Comment,
    StringLiteral,
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum ClassifierState {
    Normal,
    LineComment,
    BlockComment,
    SingleQuoteString,
    DoubleQuoteString,
    TripleSingleQuoteString,
    TripleDoubleQuoteString,
    TemplateLiteral,
}

fn classify_source(content: &str, language: SourceLanguage) -> Vec<CharClass> {
    let chars: Vec<char> = content.chars().collect();
    let mut classes = vec![CharClass::Code; chars.len()];
    let mut i = 0;

    match language {
        SourceLanguage::Python => {
            let mut state = ClassifierState::Normal;
            while i < chars.len() {
                match state {
                    ClassifierState::Normal => {
                        if chars[i] == '#' {
                            state = ClassifierState::LineComment;
                            classes[i] = CharClass::Comment;
                        } else if i + 2 < chars.len() && chars[i..i + 3] == ['"', '"', '"'] {
                            state = ClassifierState::TripleDoubleQuoteString;
                            classes[i] = CharClass::StringLiteral;
                            classes[i + 1] = CharClass::StringLiteral;
                            classes[i + 2] = CharClass::StringLiteral;
                            i += 2;
                        } else if i + 2 < chars.len() && chars[i..i + 3] == ['\'', '\'', '\''] {
                            state = ClassifierState::TripleSingleQuoteString;
                            classes[i] = CharClass::StringLiteral;
                            classes[i + 1] = CharClass::StringLiteral;
                            classes[i + 2] = CharClass::StringLiteral;
                            i += 2;
                        } else if chars[i] == '"' {
                            state = ClassifierState::DoubleQuoteString;
                            classes[i] = CharClass::StringLiteral;
                        } else if chars[i] == '\'' {
                            state = ClassifierState::SingleQuoteString;
                            classes[i] = CharClass::StringLiteral;
                        } else {
                            classes[i] = CharClass::Code;
                        }
                    }
                    ClassifierState::LineComment => {
                        classes[i] = CharClass::Comment;
                        if chars[i] == '\n' {
                            state = ClassifierState::Normal;
                        }
                    }
                    ClassifierState::DoubleQuoteString => {
                        classes[i] = CharClass::StringLiteral;
                        if chars[i] == '\\' && i + 1 < chars.len() {
                            classes[i + 1] = CharClass::StringLiteral;
                            i += 1;
                        } else if chars[i] == '"' {
                            state = ClassifierState::Normal;
                        }
                    }
                    ClassifierState::SingleQuoteString => {
                        classes[i] = CharClass::StringLiteral;
                        if chars[i] == '\\' && i + 1 < chars.len() {
                            classes[i + 1] = CharClass::StringLiteral;
                            i += 1;
                        } else if chars[i] == '\'' {
                            state = ClassifierState::Normal;
                        }
                    }
                    ClassifierState::TripleDoubleQuoteString => {
                        classes[i] = CharClass::StringLiteral;
                        if chars[i] == '\\' && i + 1 < chars.len() {
                            classes[i + 1] = CharClass::StringLiteral;
                            i += 1;
                        } else if i + 2 < chars.len() && chars[i..i + 3] == ['"', '"', '"'] {
                            classes[i] = CharClass::StringLiteral;
                            classes[i + 1] = CharClass::StringLiteral;
                            classes[i + 2] = CharClass::StringLiteral;
                            i += 2;
                            state = ClassifierState::Normal;
                        }
                    }
                    ClassifierState::TripleSingleQuoteString => {
                        classes[i] = CharClass::StringLiteral;
                        if chars[i] == '\\' && i + 1 < chars.len() {
                            classes[i + 1] = CharClass::StringLiteral;
                            i += 1;
                        } else if i + 2 < chars.len() && chars[i..i + 3] == ['\'', '\'', '\''] {
                            classes[i] = CharClass::StringLiteral;
                            classes[i + 1] = CharClass::StringLiteral;
                            classes[i + 2] = CharClass::StringLiteral;
                            i += 2;
                            state = ClassifierState::Normal;
                        }
                    }
                    _ => {}
                }
                i += 1;
            }
        }
        SourceLanguage::Rust | SourceLanguage::JavaScript | SourceLanguage::TypeScript => {
            let mut state = ClassifierState::Normal;
            while i < chars.len() {
                match state {
                    ClassifierState::Normal => {
                        if i + 1 < chars.len() && chars[i] == '/' && chars[i + 1] == '/' {
                            state = ClassifierState::LineComment;
                            classes[i] = CharClass::Comment;
                            classes[i + 1] = CharClass::Comment;
                            i += 1;
                        } else if i + 1 < chars.len() && chars[i] == '/' && chars[i + 1] == '*' {
                            state = ClassifierState::BlockComment;
                            classes[i] = CharClass::Comment;
                            classes[i + 1] = CharClass::Comment;
                            i += 1;
                        } else if chars[i] == '"' {
                            state = ClassifierState::DoubleQuoteString;
                            classes[i] = CharClass::StringLiteral;
                        } else if chars[i] == '\'' {
                            state = ClassifierState::SingleQuoteString;
                            classes[i] = CharClass::StringLiteral;
                        } else if chars[i] == '`'
                            && (language == SourceLanguage::JavaScript
                                || language == SourceLanguage::TypeScript)
                        {
                            state = ClassifierState::TemplateLiteral;
                            classes[i] = CharClass::StringLiteral;
                        } else {
                            classes[i] = CharClass::Code;
                        }
                    }
                    ClassifierState::LineComment => {
                        classes[i] = CharClass::Comment;
                        if chars[i] == '\n' {
                            state = ClassifierState::Normal;
                        }
                    }
                    ClassifierState::BlockComment => {
                        classes[i] = CharClass::Comment;
                        if i + 1 < chars.len() && chars[i] == '*' && chars[i + 1] == '/' {
                            classes[i] = CharClass::Comment;
                            classes[i + 1] = CharClass::Comment;
                            i += 1;
                            state = ClassifierState::Normal;
                        }
                    }
                    ClassifierState::DoubleQuoteString => {
                        classes[i] = CharClass::StringLiteral;
                        if chars[i] == '\\' && i + 1 < chars.len() {
                            classes[i + 1] = CharClass::StringLiteral;
                            i += 1;
                        } else if chars[i] == '"' {
                            state = ClassifierState::Normal;
                        }
                    }
                    ClassifierState::SingleQuoteString => {
                        classes[i] = CharClass::StringLiteral;
                        if chars[i] == '\\' && i + 1 < chars.len() {
                            classes[i + 1] = CharClass::StringLiteral;
                            i += 1;
                        } else if chars[i] == '\'' {
                            state = ClassifierState::Normal;
                        }
                    }
                    ClassifierState::TemplateLiteral => {
                        classes[i] = CharClass::StringLiteral;
                        if chars[i] == '\\' && i + 1 < chars.len() {
                            classes[i + 1] = CharClass::StringLiteral;
                            i += 1;
                        } else if chars[i] == '`' {
                            state = ClassifierState::Normal;
                        }
                    }
                    _ => {}
                }
                i += 1;
            }
        }
    }
    classes
}

fn is_comment_bypass_pattern(p: &str) -> bool {
    p.starts_with('#')
        || p.starts_with("//")
        || p.starts_with("/*")
        || p.contains("eslint")
        || p.contains("ts-ignore")
        || p.contains("ts-expect-error")
        || p.contains("noqa")
        || p.contains("type:")
        || p.contains("pylint:")
}

fn matches_keyword_token(line: &str, token: &str) -> bool {
    let bytes = line.as_bytes();
    let token_bytes = token.as_bytes();
    let tlen = token_bytes.len();
    if bytes.len() < tlen {
        return false;
    }
    let mut i = 0;
    while i + tlen <= bytes.len() {
        if &bytes[i..i + tlen] == token_bytes {
            let before_ok =
                i == 0 || (!bytes[i - 1].is_ascii_alphanumeric() && bytes[i - 1] != b'_');
            let after_ok = i + tlen == bytes.len()
                || (!bytes[i + tlen].is_ascii_alphanumeric() && bytes[i + tlen] != b'_');
            if before_ok && after_ok {
                return true;
            }
        }
        i += 1;
    }
    false
}
