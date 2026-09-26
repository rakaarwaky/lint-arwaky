// PURPOSE: RoleRequestVO / RoleResponseVO — aggregate request/response for role-rules
use crate::common::taxonomy_lint_result_vo::LintResult;
use crate::filesystem::taxonomy_filesystem_vo::FileEntry;

/// Consumer verb carried by the role aggregate's single entry point.
pub enum RoleRequest {
    /// Run AES401-406 role audits on pre-parsed file entries.
    RunAuditWithEntries { files: Vec<FileEntry> },
    /// Report the adapter name this orchestrator implements.
    Name,
}

impl RoleRequest {
    pub fn audit(files: &[FileEntry]) -> Self {
        Self::RunAuditWithEntries {
            files: files.to_vec(),
        }
    }
}

/// Result of a role aggregate request.
pub enum RoleResponse {
    /// Violations found by the audit.
    Audit { violations: Vec<LintResult> },
    /// Adapter name of the orchestrator.
    Name { name: String },
}

impl RoleResponse {
    pub fn into_violations(self) -> Vec<LintResult> {
        match self {
            Self::Audit { violations } => violations,
            Self::Name { .. } => Vec::new(),
        }
    }

    pub fn into_name(self) -> String {
        match self {
            Self::Name { name } => name,
            Self::Audit { .. } => String::new(),
        }
    }
}
