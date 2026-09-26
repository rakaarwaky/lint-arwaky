// PURPOSE: NamingRequestVO / NamingResponseVO — aggregate request/response for naming-rules
use crate::common::taxonomy_lint_result_vo::LintResult;
use crate::filesystem::taxonomy_filesystem_vo::FileEntry;

/// Consumer verb carried by the naming aggregate's single entry point.
pub enum NamingRequest {
    /// Run AES101/AES102 audit on pre-parsed file entries.
    RunAuditWithEntries { files: Vec<FileEntry> },
    /// Report the adapter name this orchestrator implements.
    Name,
}

impl NamingRequest {
    /// Build an audit request from a borrowed entry slice.
    pub fn audit(files: &[FileEntry]) -> Self {
        Self::RunAuditWithEntries {
            files: files.to_vec(),
        }
    }
}

/// Result of a naming aggregate request.
pub enum NamingResponse {
    /// Violations found by the audit.
    Audit { violations: Vec<LintResult> },
    /// Adapter name of the orchestrator.
    Name { name: String },
}

impl NamingResponse {
    /// Take the audit violations, or an empty list if a different verb was served.
    pub fn into_violations(self) -> Vec<LintResult> {
        match self {
            Self::Audit { violations } => violations,
            Self::Name { .. } => Vec::new(),
        }
    }

    /// Take the adapter name, or an empty string if a different verb was served.
    pub fn into_name(self) -> String {
        match self {
            Self::Name { name } => name,
            Self::Audit { .. } => String::new(),
        }
    }
}
