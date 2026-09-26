// PURPOSE: FixRequest / FixResponse — aggregate request/response VOs for auto-fix
use crate::auto_fix::taxonomy_fix_vo::FixResult;
use crate::common::taxonomy_lint_result_vo::LintResult;
use crate::common::taxonomy_message_vo::LintMessage;
use crate::common::taxonomy_path_vo::FilePath;

/// Consumer verb carried by the fix aggregate's single entry point.
/// FR-004 §9: `dry_run` is selectable per request.
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

/// Result of a fix aggregate request.
pub enum FixResponse {
    Execute { result: FixResult },
    ManualReport { reports: Vec<LintMessage> },
}

impl FixResponse {
    /// Take the fix result. Panics if a different verb was served.
    pub fn into_fix_result(self) -> FixResult {
        match self {
            Self::Execute { result } => result,
            Self::ManualReport { .. } => panic!("expected an Execute response"),
        }
    }

    pub fn into_manual_report(self) -> Vec<LintMessage> {
        match self {
            Self::ManualReport { reports } => reports,
            Self::Execute { .. } => Vec::new(),
        }
    }
}
