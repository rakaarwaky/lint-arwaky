// PURPOSE: DocRequest — request payload for the doc-rules aggregate

use std::path::{Path, PathBuf};

/// A Markdown document the checker inspects.
#[derive(Clone, Debug)]
pub struct DocSource {
    /// Absolute path to the Markdown file.
    pub path: PathBuf,
    /// Raw UTF-8 text of the file.
    pub text: String,
}

/// An invariant finding reported by the doc checker.
#[derive(Clone, Debug)]
pub struct DocFinding {
    /// The invariant code that fired, e.g. `AES601`.
    pub code: &'static str,
    /// A machine-parseable subtype so consumers can route by violation kind.
    pub violation_type: &'static str,
    /// Human-readable detail about what drifted.
    pub message: String,
}

impl DocFinding {
    pub fn new(
        code: &'static str,
        violation_type: &'static str,
        message: impl Into<String>,
    ) -> Self {
        Self {
            code,
            violation_type,
            message: message.into(),
        }
    }
}

/// The verb set the document chain supports today.
pub enum DocRequest {
    /// Audit every document under *root* that the document chain recognizes.
    /// Returns an array of findings across all inspected documents.
    AuditAll { root: PathBuf },
}

impl DocRequest {
    /// Build the audit-all request.
    pub fn audit_all(root: impl AsRef<Path>) -> Self {
        Self::AuditAll {
            root: root.as_ref().to_path_buf(),
        }
    }
}
