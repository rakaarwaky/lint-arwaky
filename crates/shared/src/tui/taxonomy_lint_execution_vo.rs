// PURPOSE: LintExecutionResult — result VO for background lint execution in the TUI.

/// The outcome of a background lint execution.
pub struct LintExecutionResult {
    pub output: String,
    pub violation_count: usize,
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
            success: true,
            cancelled,
        }
    }

    pub fn failure(output: impl Into<String>) -> Self {
        Self {
            output: output.into(),
            violation_count: 0,
            success: false,
            cancelled: false,
        }
    }

    /// True when the scan was genuinely stopped early by the cancel token.
    pub fn cancelled_early(&self) -> bool {
        self.cancelled
    }
}
