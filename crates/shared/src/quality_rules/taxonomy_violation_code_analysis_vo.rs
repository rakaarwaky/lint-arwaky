// PURPOSE: AesCodeAnalysisViolation — data container for code quality rule violations (AES301-305)
// Messages are written inline in each checker, not here.
pub use crate::common::taxonomy_language_vo::Language;

use crate::common::taxonomy_message_vo::LintMessage;

pub const WORD_PATTERN_TOKENS: &[&str] = &[
    "unwrap", "expect", "panic", "todo", "unimplemented", "unreachable",
];

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
    FileTooLarge { reason: Option<LintMessage> },
    FileTooShort { reason: Option<LintMessage> },
    MandatoryClassDefinition { reason: Option<LintMessage> },
    BypassComment { reason: Option<LintMessage> },
    UnwrapExpect { reason: Option<LintMessage> },
    Panic { reason: Option<LintMessage> },
    Todo { reason: Option<LintMessage> },
    Unimplemented { reason: Option<LintMessage> },
    DeadInheritance { reason: Option<LintMessage> },
    CodeDuplication { reason: Option<LintMessage> },
}
