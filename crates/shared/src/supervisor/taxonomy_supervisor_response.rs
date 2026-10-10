// PURPOSE: SupervisorResponse — response payload for the supervisor aggregate

use crate::taxonomy_supervisor_vo::{SupervisorCycleReport, SupervisorIssueOutcome};

/// The response the supervisor aggregate returns.
pub enum SupervisorResponse {
    /// The full per-issue report for one cycle.
    Cycle { report: SupervisorCycleReport },
    /// A single issue's outcome, for callers that want one row.
    One { outcome: SupervisorIssueOutcome },
}

impl SupervisorResponse {
    /// Extract the cycle report, or an empty report when the response carries
    /// a single-issue outcome only.
    pub fn into_cycle(self) -> SupervisorCycleReport {
        match self {
            Self::Cycle { report } => report,
            Self::One { outcome } => SupervisorCycleReport {
                outcomes: vec![outcome],
            },
        }
    }
}
