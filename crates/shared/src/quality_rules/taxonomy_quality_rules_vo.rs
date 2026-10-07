// PURPOSE: Orphan-specific analysis VOs + re-exports of graph types from filesystem.
// Re-export LintResultList so code_analysis contracts stay within their own domain.
pub use shared_common::taxonomy_lint_vo::LintResultList;

// ── Re-export graph types from filesystem (canonical home) ──
pub use shared_filesystem::taxonomy_filesystem_vo::GraphAnalysisContext;
pub use shared_filesystem::taxonomy_filesystem_vo::ImportGraph;
pub use shared_filesystem::taxonomy_filesystem_vo::InboundLinkMap;
pub use shared_filesystem::taxonomy_filesystem_vo::InheritanceMap;

use serde::{Deserialize, Serialize};
use shared_common::taxonomy_path_vo::FilePath;
use shared_common::taxonomy_severity_vo::Severity;
use std::collections::HashSet;

/// A set of file paths.
pub type FilePathSet = HashSet<FilePath>;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct OrphanIndicatorResult {
    pub is_orphan: bool,
    pub reason: String,
    pub severity: Severity,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ReachabilityResult {
    pub paths: FilePathSet,
}

impl ReachabilityResult {
    pub fn new(value: FilePathSet) -> Self {
        Self { paths: value }
    }
}

impl OrphanIndicatorResult {
    pub fn new(is_orphan: bool, reason: String, severity: Severity) -> Self {
        Self {
            is_orphan,
            reason,
            severity,
        }
    }
}

// PURPOSE: AesCodeAnalysisViolation — data container for code quality rule violations (AES301-305)
// Messages are written inline in each checker, not here.
pub use shared_common::taxonomy_language_vo::Language;

use std::path::PathBuf;

use shared_common::taxonomy_message_vo::LintMessage;

pub const WORD_PATTERN_TOKENS: &[&str] = &[
    "unwrap",
    "expect",
    "panic",
    "todo",
    "unimplemented",
    "unreachable",
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

/// Intermediate bookkeeping structure for duplicate block detection.
/// Tracks how many times a normalized code block was seen and where.
#[derive(Debug, Default)]
pub struct BlockHits {
    pub count: usize,
    pub locations: Vec<(PathBuf, usize)>,
}
