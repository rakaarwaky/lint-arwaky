// PURPOSE: LintExecutionResult — result VO for background lint execution in the TUI.

/// Three-state outcome of a background lint execution.
/// `success: bool` alone cannot express the capability-absent "unavailable"
/// state, which must render as "Unavailable in TUI — see CLI" rather than "Done".
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LintOutcome {
    Success,
    Unavailable,
    Failure,
}

/// The outcome of a background lint execution.
pub struct LintExecutionResult {
    pub output: String,
    pub violation_count: usize,
    pub outcome: LintOutcome,
    /// Derived flag kept for consumer compatibility.
    /// `true` only when `outcome == LintOutcome::Success`.
    pub success: bool,
    /// Set true when a cooperative cancel token stopped the scan early,
    /// before every linter phase ran.
    pub cancelled: bool,
}

impl LintExecutionResult {
    pub fn success(output: impl Into<String>, violations: usize) -> Self {
        Self {
            output: output.into(),
            violation_count: violations,
            outcome: LintOutcome::Success,
            success: true,
            cancelled: false,
        }
    }

    /// A scan that was cut short by the user cancelling it.
    pub fn success_cancelled(
        output: impl Into<String>,
        violations: usize,
        cancelled: bool,
    ) -> Self {
        Self {
            output: output.into(),
            violation_count: violations,
            outcome: LintOutcome::Success,
            success: true,
            cancelled,
        }
    }

    /// Capability absent in the TUI — the action can only be completed via the CLI.
    /// Distinct from both success and failure: nothing ran, nothing failed.
    pub fn unavailable(output: impl Into<String>) -> Self {
        Self {
            output: output.into(),
            violation_count: 0,
            outcome: LintOutcome::Unavailable,
            success: false,
            cancelled: false,
        }
    }

    pub fn failure(output: impl Into<String>) -> Self {
        Self {
            output: output.into(),
            violation_count: 0,
            outcome: LintOutcome::Failure,
            success: false,
            cancelled: false,
        }
    }

    /// True when the scan was genuinely stopped early by the cancel token.
    pub fn cancelled_early(&self) -> bool {
        self.cancelled
    }
}

/// Render the status-bar prefix for a lint outcome.
/// Failure and unavailable both name the path and give a next step, so the
/// user never sees the bare word "Error" (#368 defect #2, #566).
pub fn outcome_label(outcome: LintOutcome, path: &str) -> String {
    match outcome {
        LintOutcome::Success => "Done".to_string(),
        LintOutcome::Unavailable => {
            format!("Unavailable in TUI on {path} — run the CLI command instead")
        }
        LintOutcome::Failure => {
            format!("Error on {path} — press the same key to retry (see panel for details)")
        }
    }
}
