// PURPOSE: FixResponse — response payload for the fix aggregate

use crate::auto_fix::taxonomy_fix_vo::FixResult;
use crate::common::taxonomy_message_vo::LintMessage;

pub enum FixResponse {
    Execute { result: FixResult },
    ManualReport { reports: Vec<LintMessage> },
}

impl FixResponse {
    /// Take the fix result. Returns an empty result if a different verb was served.
    pub fn into_fix_result(self) -> FixResult {
        match self {
            Self::Execute { result } => result,
            Self::ManualReport { .. } => FixResult::default(),
        }
    }

    pub fn into_manual_report(self) -> Vec<LintMessage> {
        match self {
            Self::ManualReport { reports } => reports,
            Self::Execute { .. } => Vec::new(),
        }
    }
}
