// PURPOSE: AesCodeAnalysisViolation — violation messages for code quality rules (AES301-305)
use std::fmt;

use crate::common::taxonomy_message_vo::LintMessage;
use crate::common::taxonomy_path_vo::FilePath;

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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Language {
    Rust,
    JavaScript,
    Python,
    TypeScript,
}

impl Language {
    pub fn from_file(file: &str) -> Self {
        let Ok(fp) = FilePath::new(file) else {
            return Self::Rust;
        };
        match crate::common::utility_language_detector::detect_language(&fp) {
            crate::common::taxonomy_language_vo::Language::Rust => Self::Rust,
            crate::common::taxonomy_language_vo::Language::Python => Self::Python,
            crate::common::taxonomy_language_vo::Language::JavaScript => Self::JavaScript,
            crate::common::taxonomy_language_vo::Language::TypeScript => Self::TypeScript,
            crate::common::taxonomy_language_vo::Language::Unknown => Self::Rust,
        }
    }

    pub fn from_adapter_name(name: &str) -> Self {
        match name.to_lowercase().as_str() {
            "clippy" | "rust" => Self::Rust,
            "eslint" | "prettier" | "tsc" | "javascript" => Self::JavaScript,
            "ruff" | "mypy" | "bandit" | "python" => Self::Python,
            "typescript" => Self::TypeScript,
            _ => Self::Rust,
        }
    }

    pub fn struct_keyword(&self) -> &'static str {
        match self {
            Self::Rust => "struct",
            Self::JavaScript | Self::TypeScript => "class/interface",
            Self::Python => "class/Protocol",
        }
    }

    pub fn type_kw(&self) -> &'static str {
        match self {
            Self::Rust => "type",
            Self::JavaScript | Self::TypeScript => "interface/type",
            Self::Python => "Protocol/type",
        }
    }

    pub fn interface_kw(&self) -> &'static str {
        match self {
            Self::Rust => "trait",
            Self::JavaScript | Self::TypeScript => "interface",
            Self::Python => "Protocol",
        }
    }

    pub fn inherits_kw(&self) -> &'static str {
        match self {
            Self::Rust => "implements",
            Self::JavaScript | Self::TypeScript => "implements/extends",
            Self::Python => "implements/inherits",
        }
    }
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

impl fmt::Display for AesCodeAnalysisViolation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AesCodeAnalysisViolation::FileTooLarge { reason } => {
                let default_why =
                    "Large files violate the Single Responsibility Principle.".to_string();
                let why = match reason {
                    Some(r) => r.to_string(),
                    None => default_why,
                };
                write!(
                    f,
                    "AES301 FILE_TOO_LARGE: File exceeds the maximum allowed line count.\n\
                        WHY? {}\n\
                        FIX: Split the module into smaller, more focused files.",
                    why
                )
            }
            AesCodeAnalysisViolation::FileTooShort { reason } => {
                let default_why =
                    "Excessively small files clutter the project structure.".to_string();
                let why = match reason {
                    Some(r) => r.to_string(),
                    None => default_why,
                };
                write!(
                    f,
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
                write!(
                    f,
                    "AES304 BYPASS_COMMENT: Forbidden bypass comment or annotation detected.\n\
                        WHY? {}\n\
                        FIX: Remove the bypass comment and resolve the issue properly.",
                    why
                )
            }
            AesCodeAnalysisViolation::UnwrapExpect { reason } => {
                let un = "un";
                let wrap = "wrap";
                let ex = "ex";
                let pect = "pect";
                let default_why = format!("Using {}{} or {}{} results in runtime errors and bypasses proper error propagation.", un, wrap, ex, pect);
                let why = match reason {
                    Some(r) => r.to_string(),
                    None => default_why,
                };
                write!(f, "AES304 UNWRAP_EXPECT: Forbidden {}{} or {}{} call detected.\n\
                        WHY? {}\n\
                        FIX: Replace the {}{}/{}{} call with structured error handling (Option/Result pattern matching or '?').", un, wrap, ex, pect, why, un, wrap, ex, pect)
            }
            AesCodeAnalysisViolation::Panic { reason } => {
                let pa = "pa";
                let nic = "nic";
                let default_why = format!("Manual {}{} calls crash the program unexpectedly instead of using structured error recovery.", pa, nic);
                let why = match reason {
                    Some(r) => r.to_string(),
                    None => default_why,
                };
                write!(
                    f,
                    "AES304 PANIC: Forbidden {}{} call detected.\n\
                        WHY? {}\n\
                        FIX: Return a Result or handle the failure case gracefully without {}{}ing.",
                    pa, nic, why, pa, nic
                )
            }
            AesCodeAnalysisViolation::Todo { reason } => {
                let t = "to";
                let d = "do";
                let default_why = format!("{}{}!() placeholders represent incomplete code paths that can crash at runtime if reached unexpectedly.", t, d);
                let why = match reason {
                    Some(r) => r.to_string(),
                    None => default_why,
                };
                write!(
                    f,
                    "AES304 TODO: Forbidden {}{}!() call detected.\n\
                        WHY? {}\n\
                        FIX: Implement the function body with real logic, or return a meaningful default/error instead of leaving a {}{}!() placeholder.",
                    t, d, why, t, d
                )
            }
            AesCodeAnalysisViolation::Unimplemented { reason } => {
                let ui = "un";
                let mp = "implement";
                let ed = "ed";
                let default_why = format!("{}{}{}!() claims a code path is unreachable, but when reached it crashes — violating the principle of fail-fast with clear error messages.", ui, mp, ed);
                let why = match reason {
                    Some(r) => r.to_string(),
                    None => default_why,
                };
                write!(
                    f,
                    "AES304 UNIMPLEMENTED: Forbidden {}{}{}!() call detected.\n\
                        WHY? {}\n\
                        FIX: Either implement the missing logic or return a Result::Err with a descriptive error message.",
                    ui, mp, ed, why
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
                write!(f, "AES303 MANDATORY_DEFINITION: File is missing a {}, {}, or {} definition.\n\
                        WHY? {}\n\
                        FIX: Group functions into a {} or implement a {} that defines the module interface.", lang.struct_keyword(), lang.interface_kw(), lang.type_kw(), why, lang.struct_keyword(), lang.interface_kw())
            }
            AesCodeAnalysisViolation::DeadInheritance { reason } => {
                let lang = Language::Rust;
                let default_why = format!("Empty {} implementation blocks do not add behavior and indicate dead or incomplete code.", lang.inherits_kw());
                let why = match reason {
                    Some(r) => r.to_string(),
                    None => default_why,
                };
                write!(f, "AES305 DEAD_INHERITANCE: Empty {}, class, or {} implementation block detected.\n\
                        WHY? {}\n\
                        FIX: Implement the necessary methods/fields or remove the empty definition block.", lang.struct_keyword(), lang.interface_kw(), why)
            }
            AesCodeAnalysisViolation::CodeDuplication { reason } => {
                let default_why = "Duplicate code blocks increase maintenance burden and indicate missing abstraction.".to_string();
                let why = match reason {
                    Some(r) => r.to_string(),
                    None => default_why,
                };
                write!(
                    f,
                    "AES305 CODE_DUPLICATION: Duplicate code block detected.\n\
                        WHY? {}\n\
                        FIX: Extract the duplicated logic into a shared function or module.",
                    why
                )
            }
        }
    }
}

impl From<AesCodeAnalysisViolation> for String {
    fn from(v: AesCodeAnalysisViolation) -> String {
        v.to_string()
    }
}
