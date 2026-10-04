// PURPOSE: DocRequest — request payload for the doc-rules aggregate

use shared_common::taxonomy_severity_vo::Severity;
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
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct DocFinding {
    /// The invariant code that fired, e.g. `AES601`.
    pub code: &'static str,
    /// A machine-parseable subtype so consumers can route by violation kind.
    pub violation_type: &'static str,
    /// The document this finding belongs to, relative to the audit root when
    /// possible (used by the CLI to group output by file).
    pub doc: String,
    /// Human-readable detail about what drifted.
    pub message: String,
    /// 1-based line in the document the finding anchors to.
    /// Document-level findings that have no concrete line use `0` or `1`.
    pub line: usize,
    /// Severity of the finding. All AES6xx rules are HIGH per RULES_AES.
    pub severity: Severity,
}

impl DocFinding {
    /// Create a new finding anchored to a specific 1-based line.
    /// Document-level findings that have no concrete line pass `0` or `1`.
    pub fn new_with_line(
        doc: impl Into<String>,
        line: usize,
        code: &'static str,
        violation_type: &'static str,
        message: impl Into<String>,
    ) -> Self {
        Self {
            code,
            violation_type,
            doc: doc.into(),
            message: message.into(),
            line,
            severity: Severity::HIGH,
        }
    }

    /// Convert this finding to a shared `ViolationItem` for SARIF/JSON output.
    ///
    /// The finding's `doc` field is stored relative to the audit root; joining
    /// it to *root* yields the absolute path the SARIF renderer expects.
    /// When the path cannot be constructed the item falls back to a default
    /// file path so the mapping is total.
    pub fn to_violation_item(&self, root: &Path) -> shared_common::ViolationItem {
        use shared_common::{ErrorCode, FilePath, LintMessage, ViolationItem};
        let file = if self.doc.is_empty() {
            FilePath::default()
        } else {
            let candidate = if root == Path::new("") {
                self.doc.clone()
            } else {
                root.join(&self.doc).to_string_lossy().into_owned()
            };
            FilePath::new(candidate).unwrap_or_default()
        };
        ViolationItem {
            code: ErrorCode::raw(self.code),
            file,
            line: shared_common::LineNumber::new(self.line as i64),
            column: shared_common::ColumnNumber::new(1),
            message: LintMessage::new(self.message.clone()),
            severity: self.severity.clone(),
            violation_name: self.violation_type.to_string(),
            why: String::new(),
            fix: String::new(),
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
