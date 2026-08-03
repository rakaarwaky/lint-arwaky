// PURPOSE: UtilityRoleChecker — IUtilityRoleChecker for AES404: utility role boundary violations
//
// ALGORITHM:
//   Uses ParseMetadata when available to detect forbidden type definitions.
//   Falls back to comment-stripped line scanning.

use shared::common::Language;
use shared::common::LintResult;
use shared::common::Severity;
use shared::filesystem::taxonomy_filesystem_vo::{FileEntry, ParseMetadata};
use shared::role_rules::{AesRoleViolation, IUtilityRoleChecker};

// ─── Block 1: Struct Definition ───────────────────────────
pub struct UtilityRoleChecker {}

// ─── Block 2: Protocol Trait Implementation ───────────────
impl IUtilityRoleChecker for UtilityRoleChecker {
    fn check_utility_convention(&self, file: &FileEntry, violations: &mut Vec<LintResult>) {
        if let Some(meta) = &file.parse_metadata {
            self._check_with_metadata(file, meta, violations);
        } else {
            self._check_fallback(file, violations);
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

    fn _check_with_metadata(
        &self,
        file: &FileEntry,
        meta: &ParseMetadata,
        violations: &mut Vec<LintResult>,
    ) {
        let path_str = file.path.to_string_lossy();
        match meta {
            ParseMetadata::Rust(rust_meta) => {
                // Utility must not define structs, enums, traits, or type aliases
                if !rust_meta.struct_definitions.is_empty()
                    || !rust_meta.enum_definitions.is_empty()
                    || !rust_meta.trait_definitions.is_empty()
                {
                    let items: Vec<&str> = rust_meta
                        .struct_definitions
                        .iter()
                        .chain(rust_meta.enum_definitions.iter())
                        .chain(rust_meta.trait_definitions.iter())
                        .map(|s| s.as_str())
                        .collect();
                    violations.push(LintResult::new_arch(
                        &path_str,
                        0,
                        "AES404",
                        Severity::MEDIUM,
                        Self::fmt(&AesRoleViolation::UtilityRole {
                            reason: Some(
                                format!(
                                    "Utility files must not define structs or enums. Found: [{}]",
                                    items.join(", ")
                                )
                                .into(),
                            ),
                        }),
                    ));
                }
            }
            ParseMetadata::Python(py_meta) => {
                // Utility must not define classes
                if !py_meta.class_declarations.is_empty() {
                    let names: Vec<&str> = py_meta
                        .class_declarations
                        .iter()
                        .map(|c| c.name.as_str())
                        .collect();
                    violations.push(LintResult::new_arch(
                        &path_str,
                        0,
                        "AES404",
                        Severity::MEDIUM,
                        Self::fmt(&AesRoleViolation::UtilityRole {
                            reason: Some(
                                format!(
                                    "Utility files must not define classes. Found: [{}]",
                                    names.join(", ")
                                )
                                .into(),
                            ),
                        }),
                    ));
                }
            }
            ParseMetadata::TypeScript(ts_meta) | ParseMetadata::JavaScript(ts_meta) => {
                // Utility must not define classes, interfaces, enums, or type aliases
                let mut forbidden = Vec::new();
                for c in &ts_meta.class_declarations {
                    forbidden.push(format!("class '{}'", c.name));
                }
                for name in &ts_meta.interface_declarations {
                    forbidden.push(format!("interface '{}'", name));
                }
                for name in &ts_meta.type_alias_declarations {
                    forbidden.push(format!("type '{}'", name));
                }
                if !forbidden.is_empty() {
                    violations.push(LintResult::new_arch(
                        &path_str, 0, "AES404", Severity::MEDIUM,
                        Self::fmt(&AesRoleViolation::UtilityRole {
                            reason: Some(format!(
                                "Utility files must not define classes, interfaces, enums, or types. Found: [{}]",
                                forbidden.join(", ")
                            ).into()),
                        }),
                    ));
                }
            }
            _ => {} // ParseMetadata::Unknown — skip
        }
    }

    fn _check_fallback(&self, file: &FileEntry, violations: &mut Vec<LintResult>) {
        let path_str = file.path.to_string_lossy();
        let content = &file.content;
        let ext = file.language.as_str();

        if ext == "rust" || ext == "rs" {
            let stripped = Self::rust_strip_comments_macros(content);
            if stripped.contains("pub struct ") || stripped.contains("pub enum ") {
                violations.push(LintResult::new_arch(
                    &path_str,
                    0,
                    "AES404",
                    Severity::MEDIUM,
                    Self::fmt(&AesRoleViolation::UtilityRole {
                        reason: Some("Utility files must not define structs or enums.".into()),
                    }),
                ));
            }
        } else if ext == "typescript" || ext == "ts" || ext == "tsx" {
            let stripped = Self::ts_strip_comments(content);
            if stripped.contains("export class ")
                || stripped.contains("export interface ")
                || stripped.contains("export enum ")
                || stripped.contains("export type ")
            {
                violations.push(LintResult::new_arch(
                    &path_str,
                    0,
                    "AES404",
                    Severity::MEDIUM,
                    Self::fmt(&AesRoleViolation::UtilityRole {
                        reason: Some(
                            "Utility files must not define classes, interfaces, enums, or types."
                                .into(),
                        ),
                    }),
                ));
            }
        } else if ext == "python" || ext == "py" {
            let stripped = Self::python_strip_comments_docstrings(content);
            let has_forbidden = stripped.lines().any(|l| {
                let trimmed = l.trim();
                trimmed.starts_with("class ") || trimmed.starts_with("def ")
            });
            if has_forbidden {
                violations.push(LintResult::new_arch(
                    &path_str,
                    0,
                    "AES404",
                    Severity::MEDIUM,
                    format_role_violation(&AesRoleViolation::UtilityRole {
                        reason: Some("Utility files must not define classes or functions.".into()),
                    }, Language::Rust),
                ));
            }
        }
    }

    fn rust_strip_comments_macros(content: &str) -> String {
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
                let expected = "acro_rules!";
                let mut matched = true;
                for ch in expected.chars() {
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

    fn ts_strip_comments(content: &str) -> String {
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
                if c == '\n' {
                    in_template = false;
                    result.push(c);
                } else if c == '`' {
                    in_template = false;
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

    fn python_strip_comments_docstrings(content: &str) -> String {
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
}
