// PURPOSE: NamingResponse — response payload for the naming aggregate

use shared_common::taxonomy_lint_result_vo::LintResult;

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
