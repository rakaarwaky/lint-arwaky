// PURPOSE: Shared AES404 helpers for the three utility role auditors.
//
// Every language auditor (Rust / Python / TypeScript) answers the same
// question — does this utility file define a type or attach behaviour to a
// type? Only the syntax it recognises is language specific, so the pieces
// that are identical across languages — the LintResult shape, the rule code,
// and the comment strippers that keep the fallback scan from reading prose —
// live here and are shared by all three.

use shared::common::taxonomy_language_info_vo::LanguageInfo;
use shared::common::taxonomy_language_vo::Language;
use shared::common::taxonomy_lint_result_vo::LintResult;
use shared::common::taxonomy_path_vo::FilePath;
use shared::common::taxonomy_severity_vo::Severity;
use shared::filesystem::taxonomy_filesystem_vo::FileEntry;

/// The rule code every AES404 utility finding carries.
///
/// A single place so the three auditors cannot drift apart on the code they
/// report, and so the callers do not repeat the literal.
pub fn rule_code() -> &'static str {
    "AES404"
}

/// The single place an AES404 utility finding is constructed.
///
/// All three auditors build their messages the same way — a headline, the
/// reason naming the offending items, and a fix — so the three languages report
/// defects in one shape and a reader does not have to learn three formats.
pub fn build_violation(path: &str, why: &str, fix: &str, detail: &str) -> LintResult {
    LintResult::new_arch(
        path,
        0,
        rule_code(),
        Severity::MEDIUM,
        format!("{detail}\nWHY? {why}\nFIX: {fix}"),
    )
}

/// The shared headline for a utility file that defines a forbidden type.
pub fn type_definition_detail() -> &'static str {
    "AES404 UTILITY_ROLE: Utility file contains forbidden type definitions."
}

/// The fix that applies to every forbidden type definition.
pub fn type_definition_fix() -> &'static str {
    "Remove type definitions; use stateless functions only."
}

/// Resolve the language flags for a file.
///
/// An auditor calls this to confirm the file is a language it can scan before
/// walking its syntax. Returns `None` when the extension is not one this
/// linter parses, so the caller skips instead of scanning nothing useful.
pub fn detect_language(file: &FileEntry) -> Option<LanguageInfo> {
    let path_str = file.path.to_string_lossy();
    // The FilePath validates the path shape; a malformed one cannot be scanned.
    FilePath::new(path_str.to_string()).ok()?;
    let lang = match file
        .path
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or_default()
    {
        "rs" => Language::Rust,
        "py" => Language::Python,
        "js" | "jsx" | "mjs" | "cjs" | "ts" | "tsx" | "mts" | "cts" => {
            if file.language == Language::JavaScript {
                Language::JavaScript
            } else {
                Language::TypeScript
            }
        }
        _ => return None,
    };
    Some(LanguageInfo::new(
        lang == Language::Rust,
        lang == Language::Python,
        lang == Language::TypeScript || lang == Language::JavaScript,
        lang,
    ))
}

/// Strip Rust line comments, block comments, and `macro_rules!` bodies.
///
/// A macro body is token soup rather than items, so an `impl` written inside
/// `macro_rules!` is not a utility defect — it produces the type only where the
/// macro is invoked, and the invoking file is scanned on its own. Dropping the
/// body keeps the fallback scan from reporting it.
pub fn strip_rust_comments_and_macros(content: &str) -> String {
    let mut result = String::with_capacity(content.len());
    let mut in_line_comment = false;
    let mut in_block_comment = false;
    let mut in_macro = false;
    let mut brace_depth: usize = 0;
    let mut chars = content.chars().peekable();
    while let Some(c) = chars.next() {
        if in_block_comment {
            if c == '*' && chars.peek() == Some(&'/') {
                chars.next();
                in_block_comment = false;
            }
            continue;
        }
        if in_line_comment {
            if c == '\n' {
                in_line_comment = false;
                result.push(c);
            }
            continue;
        }
        if in_macro {
            if c == '{' {
                brace_depth += 1;
            } else if c == '}' {
                brace_depth = brace_depth.saturating_sub(1);
                if brace_depth == 0 {
                    in_macro = false;
                }
            }
            continue;
        }
        if c == '/' && chars.peek() == Some(&'/') {
            in_line_comment = true;
            chars.next();
            continue;
        }
        if c == '/' && chars.peek() == Some(&'*') {
            in_block_comment = true;
            chars.next();
            continue;
        }
        if c == 'm' {
            let mut temp = chars.clone();
            let mut matched = true;
            for ch in "acro_rules!".chars() {
                match temp.next() {
                    Some(a) if a == ch => {}
                    _ => {
                        matched = false;
                        break;
                    }
                }
            }
            if matched {
                for _ in 0..12 {
                    chars.next();
                }
                while let Some(&nc) = chars.peek() {
                    if nc == '{' {
                        break;
                    }
                    chars.next();
                }
                if let Some(&'{') = chars.peek() {
                    in_macro = true;
                    brace_depth = 1;
                    chars.next();
                }
                continue;
            }
        }
        result.push(c);
    }
    result
}

/// True when the comment-stripped Rust content defines a type or impl block at
/// the start of a line.
///
/// The check is anchored to line-start so an `impl` nested inside a function
/// body or a const block is not mistaken for a top-level item; the
/// `macro_rules!` bodies are already gone by the time this runs.
pub fn rust_has_forbidden_item(stripped: &str) -> bool {
    stripped.lines().any(|l| {
        let trimmed = l.trim_start();
        trimmed.starts_with("pub struct ")
            || trimmed.starts_with("struct ")
            || trimmed.starts_with("pub enum ")
            || trimmed.starts_with("enum ")
            || trimmed.starts_with("pub trait ")
            || trimmed.starts_with("trait ")
            || trimmed.starts_with("pub type ")
            || trimmed.starts_with("type ")
            || trimmed.starts_with("impl ")
    })
}

/// Strip TypeScript line comments, block comments, and template literals.
///
/// A template literal can hold braces and the word `class`, so it is dropped
/// alongside the comments; the fallback scan reads what remains as code.
pub fn strip_ts_comments(content: &str) -> String {
    let mut result = String::with_capacity(content.len());
    let mut in_line = false;
    let mut in_block = false;
    let mut in_template = false;
    let mut chars = content.chars().peekable();
    while let Some(c) = chars.next() {
        if in_block {
            if c == '*' && chars.peek() == Some(&'/') {
                chars.next();
                in_block = false;
            }
            continue;
        }
        if in_line {
            if c == '\n' {
                in_line = false;
                result.push(c);
            }
            continue;
        }
        if in_template {
            if c == '\n' || c == '`' {
                in_template = false;
                result.push(c);
            }
            continue;
        }
        if c == '/' && chars.peek() == Some(&'/') {
            in_line = true;
            chars.next();
            continue;
        }
        if c == '/' && chars.peek() == Some(&'*') {
            in_block = true;
            chars.next();
            continue;
        }
        if c == '`' {
            in_template = true;
            continue;
        }
        result.push(c);
    }
    result
}

/// True when the comment-stripped TypeScript content exports a forbidden type.
pub fn ts_has_forbidden_item(stripped: &str) -> bool {
    stripped.contains("export class ")
        || stripped.contains("export interface ")
        || stripped.contains("export enum ")
        || stripped.contains("export type ")
}

/// Strip Python line comments and triple-quoted docstrings.
pub fn strip_python_comments_and_docstrings(content: &str) -> String {
    let mut result = String::with_capacity(content.len());
    let mut in_line = false;
    let mut in_docstring = false;
    let mut chars = content.chars().peekable();
    while let Some(c) = chars.next() {
        if in_line {
            if c == '\n' {
                in_line = false;
                result.push(c);
            }
            continue;
        }
        if in_docstring {
            let is_q = c == '"' || c == '\'';
            if is_q && chars.peek() == Some(&c) {
                chars.next();
                if chars.peek() == Some(&c) {
                    chars.next();
                    in_docstring = false;
                }
            }
            continue;
        }
        if c == '#' {
            in_line = true;
            continue;
        }
        if c == '"' || c == '\'' {
            let q = c;
            let first_two: String = chars.clone().take(2).collect();
            if first_two.len() == 2
                && first_two.starts_with(q)
                && first_two.chars().all(|ch| ch == q)
            {
                in_docstring = true;
                for _ in 0..2 {
                    chars.next();
                }
                continue;
            }
        }
        result.push(c);
    }
    result
}

/// True when the comment-stripped Python content declares a class.
///
/// Only `class` counts: a module-level `def` is the body a utility file is
/// supposed to have, so flagging it would report every correct utility file.
pub fn python_has_forbidden_item(stripped: &str) -> bool {
    stripped.lines().any(|l| l.trim().starts_with("class "))
}
