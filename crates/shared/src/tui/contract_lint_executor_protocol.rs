use crate::tui::taxonomy_action_flags_vo::ActionFlags;
use crate::tui::taxonomy_lint_result_vo::LintExecutionResult;
use crate::tui::taxonomy_watch_message_vo::WatchMessage;
use std::sync::mpsc::Receiver;

pub trait ILintExecutorProtocol: Send + Sync {
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
    /// Start file-watch mode: run initial scan, then spawn background thread
    /// that lints changed files. Returns (initial_result, receiver_for_updates).
    fn watch(&self, path: &str) -> (LintExecutionResult, Receiver<WatchMessage>);
}

