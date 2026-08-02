// PURPOSE: FixOutcome — reason-coded outcome for every fix attempt
//
// FRD requires: "Every fix attempt MUST return a reason-coded outcome
// (Applied / Skipped(reason) / Failed(reason)), not a bare boolean."
// This enum replaces bare `bool` returns on IFixProtocol methods.

use serde::{Deserialize, Serialize};

/// Reason why a fix was skipped — not fixable by auto-fix.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SkipReason {
    /// Target line is part of a multi-line import block.
    MultiLineImport,
    /// Line number is 0 or exceeds file length.
    LineOutOfBounds,
    /// Target line is not an import statement.
    NotAnImportLine,
    /// Line number does not contain a recognised bypass pattern.
    NoBypassPattern,
    /// Bypass pattern requires semantic understanding (panic!, todo!, etc.).
    UnsafeRemoval,
    /// Pattern already has a context message (e.g. expect("...")).
    AlreadyHasContext,
    /// Symbol name is already valid snake_case.
    AlreadyValid,
    /// Symbol name was not found in file content.
    SymbolNotFound,
    /// Symbol name conflicts with a language keyword.
    KeywordConflict,
}

/// Reason why a fix failed — hard error, not a policy skip.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum FailReason {
    /// Target file does not exist on disk.
    FileNotFound,
    /// File could not be read (I/O error).
    ReadError,
    /// File could not be written (I/O error).
    WriteError,
}

/// Reason-coded outcome for every fix attempt.
///
/// The FRD mandates this shape for all individual fix operations
/// (fix_bypass_comments, fix_unused_import, rename_symbol).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum FixOutcome {
    /// Fix was applied and the file was modified.
    Applied { changes: usize },
    /// Fix was not applied — see reason.
    Skipped(SkipReason),
    /// Fix could not be applied — see reason.
    Failed(FailReason),
}

impl FixOutcome {
    pub fn applied(changes: usize) -> Self {
        Self::Applied { changes }
    }

    pub fn skipped(reason: SkipReason) -> Self {
        Self::Skipped(reason)
    }

    pub fn failed(reason: FailReason) -> Self {
        Self::Failed(reason)
    }

    /// Whether this outcome represents a successful fix.
    pub fn is_applied(&self) -> bool {
        matches!(self, Self::Applied { .. })
    }
}

impl std::fmt::Display for FixOutcome {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Applied { changes } => write!(f, "Applied ({changes} change(s))"),
            Self::Skipped(reason) => write!(f, "Skipped({reason:?})"),
            Self::Failed(reason) => write!(f, "Failed({reason:?})"),
        }
    }
}
