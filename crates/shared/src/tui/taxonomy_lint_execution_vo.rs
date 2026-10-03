// PURPOSE: LintExecutionResult — result VO for background lint execution in the TUI.

/// The outcome of a background lint execution.
pub struct LintExecutionResult {
    pub output: String,
    pub violation_count: usize,
    pub success: bool,
}

impl LintExecutionResult {
    pub fn success(output: impl Into<String>, violations: usize) -> Self {
        Self {
            output: output.into(),
            violation_count: violations,
            success: true,
        }
    }

    pub fn failure(output: impl Into<String>) -> Self {
        Self {
            output: output.into(),
            violation_count: 0,
            success: false,
        }
    }
}
