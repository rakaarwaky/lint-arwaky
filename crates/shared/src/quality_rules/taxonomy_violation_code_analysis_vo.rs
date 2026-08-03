// PURPOSE: AesCodeAnalysisViolation — data container for code quality rule violations (AES301-305)
// Messages are formatted by `format_code_analysis_violation()`, not by Display.
use std::fmt;

pub use crate::common::taxonomy_language_vo::Language;

use crate::common::taxonomy_message_vo::LintMessage;

/// Identifiers treated as Rust-style word tokens (must match as a whole identifier).
pub const WORD_PATTERN_TOKENS: &[&str] = &[
    "unwrap",
    "expect",
    "panic",
    "todo",
    "unimplemented",
    "unreachable",
];

/// Internal violation kind for classification during scanning.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ViolationKind {
    UnwrapExpect,
    Panic,
    Todo,
    Unimplemented,
    BypassComment,
}

#[derive(Debug, Clone)]
pub enum AesCodeAnalysisViolation {
    // AES301 — File size
    FileTooLarge { reason: Option<LintMessage> },
    FileTooShort { reason: Option<LintMessage> },
    // AES303 — Mandatory class/struct definition
    MandatoryClassDefinition { reason: Option<LintMessage> },
    // AES304 — Bypass comments (Rust only)
    BypassComment { reason: Option<LintMessage> },
    UnwrapExpect { reason: Option<LintMessage> },
    Panic { reason: Option<LintMessage> },
    Todo { reason: Option<LintMessage> },
    Unimplemented { reason: Option<LintMessage> },
    // AES305 — Duplicate/dead code (empty impl blocks)
    DeadInheritance { reason: Option<LintMessage> },
    CodeDuplication { reason: Option<LintMessage> },
}

pub fn format_code_analysis_violation(v: &AesCodeAnalysisViolation) -> String {
    match v {
        AesCodeAnalysisViolation::FileTooLarge { reason } => {
            let default_why =
                "Large files violate the Single Responsibility Principle.".to_string();
            let why = match reason {
                Some(r) => r.to_string(),
                None => default_why,
            };
            format!(
                "AES301 FILE_TOO_LARGE: File exceeds the maximum allowed line count.\n\
                    WHY? {}\n\
                    FIX: Split the module into smaller, more focused files.",
                why
            )
        }
        AesCodeAnalysisViolation::FileTooShort { reason } => {
            let default_why = "Excessively small files clutter the project structure.".to_string();
            let why = match reason {
                Some(r) => r.to_string(),
                None => default_why,
            };
            format!(
                "AES302 FILE_TOO_SHORT: File contains fewer than the required minimum lines.\n\
                    WHY? {}\n\
                    FIX: Expand the component or merge this logic into a related module.",
                why
            )
        }
        AesCodeAnalysisViolation::BypassComment { reason } => {
            let default_why =
                "Bypassing code checks hides issues and risks architectural regressions."
                    .to_string();
            let why = match reason {
                Some(r) => r.to_string(),
                None => default_why,
            };
            format!(
                "AES304 BYPASS_COMMENT: Forbidden bypass comment or annotation detected.\n\
                    WHY? {}\n\
                    FIX: Remove the bypass comment and resolve the issue properly.",
                why
            )
        }
        AesCodeAnalysisViolation::UnwrapExpect { reason } => {
            let default_why =
                "Using unwrap or expect results in runtime errors and bypasses proper error propagation."
                    .to_string();
            let why = match reason {
                Some(r) => r.to_string(),
                None => default_why,
            };
            format!(
                "AES304 UNWRAP_EXPECT: Forbidden unwrap or expect call detected.\n\
                    WHY? {}\n\
                    FIX: Replace the unwrap/expect call with structured error handling (Option/Result pattern matching or '?').",
                why
            )
        }
        AesCodeAnalysisViolation::Panic { reason } => {
            let default_why =
                "Manual panic calls crash the program unexpectedly instead of using structured error recovery."
                    .to_string();
            let why = match reason {
                Some(r) => r.to_string(),
                None => default_why,
            };
            format!(
                "AES304 PANIC: Forbidden panic call detected.\n\
                    WHY? {}\n\
                    FIX: Return a Result or handle the failure case gracefully without panicking.",
                why
            )
        }
        AesCodeAnalysisViolation::Todo { reason } => {
            let default_why =
                "todo!() placeholders represent incomplete code paths that can crash at runtime if reached unexpectedly."
                    .to_string();
            let why = match reason {
                Some(r) => r.to_string(),
                None => default_why,
            };
            format!(
                "AES304 TODO: Forbidden todo!() call detected.\n\
                    WHY? {}\n\
                    FIX: Implement the function body with real logic, or return a meaningful default/error instead of leaving a todo!() placeholder.",
                why
            )
        }
        AesCodeAnalysisViolation::Unimplemented { reason } => {
            let default_why =
                "unimplemented!() claims a code path is unreachable, but when reached it crashes — violating the principle of fail-fast with clear error messages."
                    .to_string();
            let why = match reason {
                Some(r) => r.to_string(),
                None => default_why,
            };
            format!(
                "AES304 UNIMPLEMENTED: Forbidden unimplemented!() call detected.\n\
                    WHY? {}\n\
                    FIX: Either implement the missing logic or return a Result::Err with a descriptive error message.",
                why
            )
        }
        AesCodeAnalysisViolation::MandatoryClassDefinition { reason } => {
            let lang = Language::Rust;
            let default_why = format!(
                "Encapsulation in {} is required for proper modularization and contract adherence.",
                lang.struct_keyword()
            );
            let why = match reason {
                Some(r) => r.to_string(),
                None => default_why,
            };
            format!(
                "AES303 MANDATORY_DEFINITION: File is missing a {}, {}, or {} definition.\n\
                    WHY? {}\n\
                    FIX: Group functions into a {} or implement a {} that defines the module interface.",
                lang.struct_keyword(),
                lang.interface_kw(),
                lang.type_kw(),
                why,
                lang.struct_keyword(),
                lang.interface_kw()
            )
        }
        AesCodeAnalysisViolation::DeadInheritance { reason } => {
            let lang = Language::Rust;
            let default_why = format!(
                "Empty {} implementation blocks do not add behavior and indicate dead or incomplete code.",
                lang.inherits_kw()
            );
            let why = match reason {
                Some(r) => r.to_string(),
                None => default_why,
            };
            format!(
                "AES305 DEAD_INHERITANCE: Empty {}, class, or {} implementation block detected.\n\
                    WHY? {}\n\
                    FIX: Implement the necessary methods/fields or remove the empty definition block.",
                lang.struct_keyword(),
                lang.interface_kw(),
                why
            )
        }
        AesCodeAnalysisViolation::CodeDuplication { reason } => {
            let default_why = "Duplicate code blocks increase maintenance burden and indicate missing abstraction.".to_string();
            let why = match reason {
                Some(r) => r.to_string(),
                None => default_why,
            };
            format!(
                "AES305 CODE_DUPLICATION: Duplicate code block detected.\n\
                    WHY? {}\n\
                    FIX: Extract the duplicated logic into a shared function or module.",
                why
            )
        }
    }
}
