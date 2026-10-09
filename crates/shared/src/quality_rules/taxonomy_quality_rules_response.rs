// PURPOSE: CodeAnalysisResponse — response payload for the code_analysis aggregate

use shared_common::taxonomy_lint_vo::LintResult;

pub enum CodeAnalysisResponse {
    Analysis { violations: Vec<LintResult> },
    Name { name: String },
}

impl CodeAnalysisResponse {
    pub fn into_violations(self) -> Vec<LintResult> {
        match self {
            Self::Analysis { violations } => violations,
            _ => Vec::new(),
        }
    }

    pub fn into_name(self) -> String {
        match self {
            Self::Name { name } => name,
            _ => String::new(),
        }
    }
}
