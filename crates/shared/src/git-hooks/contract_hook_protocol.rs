// PURPOSE: IHookProtocol — protocol for git hook management operations (business logic)
use crate::common::taxonomy_layer_vo::Identity;
use crate::git_hooks::taxonomy_hook_error::GitHookError;
use crate::mcp_server::taxonomy_job_vo::SuccessStatus;
use crate::source_parsing::taxonomy_path_vo::FilePath;
use async_trait::async_trait;

#[async_trait]
pub trait IHookProtocol: Send + Sync {
    /// Install pre-commit hook
    async fn install_pre_commit(
        &self,
        executable_path: &FilePath,
    ) -> Result<SuccessStatus, GitHookError>;

    /// Uninstall pre-commit hook
    async fn uninstall_pre_commit(&self) -> Result<SuccessStatus, GitHookError>;

    /// Get hook manager identity
    fn get_hook_manager_identity(&self) -> Identity;

    /// Initialize git hooks config
    async fn initialize_config(&self, path: &str) -> String;

    /// Update ignore rule
    fn update_ignore_rule(&self, rule: &str, remove: bool, config_path: &str) -> String;

    /// Get diff data between two paths
    async fn get_diff_data(
        &self,
        path1: &str,
        path2: &str,
    ) -> std::collections::HashMap<String, serde_json::Value>;

}
