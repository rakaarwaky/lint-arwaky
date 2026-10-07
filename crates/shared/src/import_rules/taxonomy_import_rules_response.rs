// PURPOSE: ImportResponse — response payload for the import aggregate

use shared_common::taxonomy_adapter_error::ScanError;
use shared_common::taxonomy_lint_vo::LintResult;

pub enum ImportResponse {
    /// Audit result from a path-based run (may fail if the path is missing).
    Audit {
        result: Result<Vec<LintResult>, ScanError>,
    },
    /// Violations found by an entry-based audit.
    AuditEntries { violations: Vec<LintResult> },
    /// Adapter name of the orchestrator.
    Name { name: String },
}

impl ImportResponse {
    /// Take the path-based audit result, or Ok(empty) if a different verb was served.
    pub fn into_result(self) -> Result<Vec<LintResult>, ScanError> {
        match self {
            Self::Audit { result } => result,
            Self::AuditEntries { violations } => Ok(violations),
            Self::Name { .. } => Ok(Vec::new()),
        }
    }

    pub fn into_violations(self) -> Vec<LintResult> {
        match self {
            Self::AuditEntries { violations } => violations,
            Self::Audit { result } => result.unwrap_or_default(),
            Self::Name { .. } => Vec::new(),
        }
    }

    pub fn into_name(self) -> String {
        match self {
            Self::Name { name } => name,
            _ => String::new(),
        }
    }
}
