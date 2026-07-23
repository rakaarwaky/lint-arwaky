// PURPOSE: UtilityRoleChecker — IUtilityRoleChecker for AES404: utility role boundary violations
//
// ALGORITHM:
//   check_utility_convention:
//     1. Detect language from file extension (.rs → Rust, .ts/.tsx → TypeScript, .py → Python)
//     2. Strip comments and language-specific noise (macros for Rust, template literals for TS,
//        docstrings for Python)
//     3. Check for forbidden type definitions:
//        - Rust: pub struct, pub enum
//        - TS: export class, export interface, export enum, export type
//        - Python: class ClassName(, def function_name(
//     4. Flag violation if any found

use shared::cli_commands::taxonomy_result_vo::LintResult;
use shared::common::taxonomy_severity_vo::Severity;
use shared::role_rules::contract_utility_role_protocol::IUtilityRoleChecker;
use shared::role_rules::taxonomy_violation_role_vo::AesRoleViolation;
use shared::taxonomy_source_vo::SourceContentVO;

// ─── Block 1: Struct Definition ───────────────────────────

pub struct UtilityRoleChecker {}

// ─── Block 2: Protocol Trait Implementation ───────────────

impl IUtilityRoleChecker for UtilityRoleChecker {
    fn check_utility_convention(&self, source: &SourceContentVO, violations: &mut Vec<LintResult>) {
        let content = source.content.value();
        let file = source.file_path.value();
        let ext = source.language.as_str();

        // Detect language from extension and apply appropriate checks
        if ext == "rs" || ext == "rust" {
            self._check_rust_utility(&content, &file, violations);
        } else if ext == "ts" || ext == "tsx" {
            self._check_ts_utility(&content, &file, violations);
        } else if ext == "py" {
            self._check_python_utility(&content, &file, violations);
        }
    }
}

// ─── Block 3: Constructors, Helpers, Private Methods ──────

impl Default for UtilityRoleChecker {
    fn default() -> Self {
        Self::new()
    }
}

impl UtilityRoleChecker {
    pub fn new() -> Self {
        Self {}
    }

    // ─── Rust ──────────────────────────────────────────────

    fn _check_rust_utility(&self, content: &str, file: &str, violations: &mut Vec<LintResult>) {
        let stripped = Self::rust_strip_comments_macros(content);

        if stripped.contains("pub struct ") || stripped.contains("pub enum ") {
            violations.push(LintResult::new_arch(
                file,
                0,
                "AES404",
                Severity::MEDIUM,
                AesRoleViolation::UtilityRole {
                    reason: Some("Utility files must not define structs or enums.".into()),
                }
                .to_string(),
            ));
        }
    }

    /// Strip Rust comments (line //, block /* */) and macro_rules! bodies.
    fn rust_strip_comments_macros(content: &str) -> String {
        let mut result = String::with_capacity(content.len());
        let mut in_line_comment = false;
        let mut in_block_comment = false;
        let mut in_macro = false;
        let mut brace_depth: usize = 0;
        let mut chars = content.chars().peekable();

        while let Some(c) = chars.next() {
            // Handle block comments (/* ... */)
            if in_block_comment {
                if c == '*' && chars.peek() == Some(&'/') {
                    chars.next();
                    in_block_comment = false;
                }
                continue;
            }

            // Handle line comments (// ...)
            if in_line_comment {
                if c == '\n' {
                    in_line_comment = false;
                    result.push(c);
                }
                continue;
            }

            // Handle macro bodies (macro_rules! { ... })
            if in_macro {
                if c == '{' {
                    brace_depth += 1;
                } else if c == '}' {
                    if brace_depth > 0 {
                        brace_depth -= 1;
                    }
                    if brace_depth == 0 {
                        in_macro = false;
                    }
                }
                continue;
            }

            // Detect comment start: //
            if c == '/' && chars.peek() == Some(&'/') {
                in_line_comment = true;
                chars.next();
                continue;
            }

            // Detect block comment start: /*
            if c == '/' && chars.peek() == Some(&'*') {
                in_block_comment = true;
                chars.next();
                continue;
            }

            // Detect macro_rules! start — check if content starting from here matches "macro_rules!"
            if c == 'm' {
                let mut temp_iter = chars.clone();
                let expected = "acro_rules!";
                let mut matched = true;
                for ch in expected.chars() {
                    match temp_iter.next() {
                        Some(actual) if actual == ch => {} // continue
                        _ => { matched = false; break; }
                    }
                }
                if matched {
                    // Skip "macro_rules!" (12 chars) and current 'm'
                    for _ in 0..12 {
                        chars.next();
                    }
                    // Skip everything until opening brace (name, whitespace, etc.)
                    while let Some(&next_c) = chars.peek() {
                        if next_c == '{' {
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

    // ─── TypeScript ────────────────────────────────────────

    fn _check_ts_utility(&self, content: &str, file: &str, violations: &mut Vec<LintResult>) {
        let stripped = Self::ts_strip_comments(content);

        if stripped.contains("export class ")
            || stripped.contains("export interface ")
            || stripped.contains("export enum ")
            || stripped.contains("export type ")
        {
            violations.push(LintResult::new_arch(
                file,
                0,
                "AES404",
                Severity::MEDIUM,
                AesRoleViolation::UtilityRole {
                    reason: Some("Utility files must not define classes, interfaces, enums, or types.".into()),
                }
                .to_string(),
            ));
        }
    }

    /// Strip TypeScript comments (line //, block /* */) and template literals.
    fn ts_strip_comments(content: &str) -> String {
        let mut result = String::with_capacity(content.len());
        let mut in_line_comment = false;
        let mut in_block_comment = false;
        let mut in_template = false;
        let mut chars = content.chars().peekable();

        while let Some(c) = chars.next() {
            // Handle block comments (/* ... */)
            if in_block_comment {
                if c == '*' && chars.peek() == Some(&'/') {
                    chars.next();
                    in_block_comment = false;
                }
                continue;
            }

            // Handle line comments (// ...)
            if in_line_comment {
                if c == '\n' {
                    in_line_comment = false;
                    result.push(c);
                }
                continue;
            }

            // Handle template literals (` ... `) — strip content but keep newline
            if in_template {
                if c == '\n' {
                    in_template = false;
                    result.push(c);
                } else if c == '\\' && chars.peek().is_some() {
                    // Escaped character inside template — skip both
                    let next = chars.next();
                    if next.is_some() {
                        // consumed escaped char
                    }
                } else if c == '`' {
                    in_template = false;
                }
                continue;
            }

            // Detect comment start: //
            if c == '/' && chars.peek() == Some(&'/') {
                in_line_comment = true;
                chars.next();
                continue;
            }

            // Detect block comment start: /*
            if c == '/' && chars.peek() == Some(&'*') {
                in_block_comment = true;
                chars.next();
                continue;
            }

            // Detect template literal start: `
            if c == '`' {
                in_template = true;
                continue;
            }

            result.push(c);
        }

        result
    }

    // ─── Python ────────────────────────────────────────────

    fn _check_python_utility(&self, content: &str, file: &str, violations: &mut Vec<LintResult>) {
        let stripped = Self::python_strip_comments_docstrings(content);

        if stripped.contains("class ") || stripped.contains("def ") {
            violations.push(LintResult::new_arch(
                file,
                0,
                "AES404",
                Severity::MEDIUM,
                AesRoleViolation::UtilityRole {
                    reason: Some("Utility files must not define classes or functions.".into()),
                }
                .to_string(),
            ));
        }
    }

    /// Strip Python comments (# ...) and docstrings (""" ... """ or ''' ... ''').
    fn python_strip_comments_docstrings(content: &str) -> String {
        let mut result = String::with_capacity(content.len());
        let mut in_line_comment = false;
        let mut in_docstring = false;
        let mut chars = content.chars().peekable();

        while let Some(c) = chars.next() {
            // Handle line comments (# ...)
            if in_line_comment {
                if c == '\n' {
                    in_line_comment = false;
                    result.push(c);
                }
                continue;
            }

            // Handle docstrings (""" ... """ or ''' ... ''')
            if in_docstring {
                // Check for closing delimiter (3 quote chars)
                let is_quote = c == '"' || c == '\'';
                if is_quote && chars.peek() == Some(&c) {
                    // peek second char matches — consume it
                    chars.next();
                    // peek third char matches — consume it and close
                    if chars.peek() == Some(&c) {
                        chars.next();
                        in_docstring = false;
                    }
                }
                continue;
            }

            // Detect line comment: #
            if c == '#' {
                in_line_comment = true;
                continue;
            }

            // Detect docstring start: """ or ''' (3 quote chars)
            if c == '"' || c == '\'' {
                let q = c;
                let first_two: String = chars.clone().take(2).collect();
                if first_two.len() == 2 && first_two.starts_with(q) && first_two.chars().all(|ch| ch == q) {
                    in_docstring = true;
                    // Skip the 3 quote chars (current + 2 peeked)
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
}
