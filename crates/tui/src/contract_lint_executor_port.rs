use crate::taxonomy_action_flags_vo::ActionFlags;

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

pub trait ILintExecutorPort: Send + Sync {
    fn check(&self, path: &str, flags: &ActionFlags) -> LintExecutionResult;
    fn scan(&self, path: &str) -> LintExecutionResult;
    fn fix(&self, path: &str, flags: &ActionFlags) -> LintExecutionResult;
    fn ci(&self, path: &str, flags: &ActionFlags) -> LintExecutionResult;
    fn orphan(&self, path: &str) -> LintExecutionResult;
    fn security(&self, path: &str) -> LintExecutionResult;
    fn duplicates(&self, path: &str) -> LintExecutionResult;
    fn dependencies(&self, path: &str) -> LintExecutionResult;
    fn doctor(&self) -> LintExecutionResult;
    fn init(&self, flags: &ActionFlags) -> LintExecutionResult;
    fn install(&self, flags: &ActionFlags) -> LintExecutionResult;
    fn mcp_config(&self, flags: &ActionFlags) -> LintExecutionResult;
    fn config_show(&self) -> LintExecutionResult;
    fn install_hook(&self) -> LintExecutionResult;
    fn uninstall_hook(&self) -> LintExecutionResult;
    fn adapters(&self) -> LintExecutionResult;
    fn version(&self) -> LintExecutionResult;
}
