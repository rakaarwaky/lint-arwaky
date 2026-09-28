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
        }
    }
}

/// The verb set the structure auditor supports today.
#[derive(Debug)]
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
