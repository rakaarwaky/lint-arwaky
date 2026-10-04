// PURPOSE: StructureRequest — request payload for the structure-rules aggregate

use std::path::{Path, PathBuf};

/// A single structural invariant finding reported by the audit.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct StructureFinding {
    /// The invariant code that fired, e.g. `AES701`.
    pub code: String,
    /// A machine-parseable subtype so consumers can route by violation kind.
    pub violation_type: String,
    /// The path this finding points at (relative or absolute, per caller).
    pub file: String,
    /// Human-readable detail about what drifted.
    pub message: String,
    /// Why the invariant matters. Rendered on its own line in the report, so
    /// it holds the reason and not a restatement of `message`.
    pub why: String,
    /// What to do about it. Rendered on its own line under `why`.
    pub fix: String,
}

impl StructureFinding {
    pub fn new(
        code: impl Into<String>,
        violation_type: impl Into<String>,
        file: impl Into<String>,
        message: impl Into<String>,
    ) -> Self {
        Self {
            code: code.into(),
            violation_type: violation_type.into(),
            file: file.into(),
            message: message.into(),
            why: String::new(),
            fix: String::new(),
        }
    }

    /// Attach the reason and the remedy, for the report to print under the
    /// code. The 4-argument `new` keeps working for a caller that has no
    /// separate wording to offer.
    pub fn with_reason(
        mut self,
        why: impl Into<String>,
        fix: impl Into<String>,
    ) -> Self {
        self.why = why.into();
        self.fix = fix.into();
        self
    }
}

/// The verb set the structure auditor supports today.
#[derive(Debug, Clone)]
pub enum StructureRequest {
    /// Audit every folder under *root* that the structure rules recognize.
    AuditAll { root: PathBuf },
}

impl StructureRequest {
    /// Build the audit-all request.
    pub fn audit_all(root: impl AsRef<Path>) -> Self {
        Self::AuditAll {
            root: root.as_ref().to_path_buf(),
        }
    }
}
