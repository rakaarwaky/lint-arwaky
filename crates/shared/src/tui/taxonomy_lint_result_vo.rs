#[derive(Debug, Clone)]
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
