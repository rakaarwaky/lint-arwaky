// PURPOSE: FixRequest — request payload for the fix aggregate

use crate::common::taxonomy_lint_result_vo::LintResult;
use crate::common::taxonomy_path_vo::FilePath;

pub enum FixRequest {
    /// Run linter + apply fixes.
    Execute { path: FilePath, dry_run: bool },
    /// FR-005: Report violations that require manual intervention.
    ManualReport { violations: Vec<LintResult> },
}

impl FixRequest {
    pub fn execute(path: &FilePath, dry_run: bool) -> Self {
        Self::Execute {
            path: path.clone(),
            dry_run,
        }
    }

    pub fn manual_report(violations: &[LintResult]) -> Self {
        Self::ManualReport {
            violations: violations.to_vec(),
        }
    }
}
