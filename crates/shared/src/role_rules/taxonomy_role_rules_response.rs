// PURPOSE: RoleResponse — response payload for the role aggregate

use crate::common::taxonomy_lint_result_vo::LintResult;

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
