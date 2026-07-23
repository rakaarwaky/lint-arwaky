// PURPOSE: IHookProtocol — protocol for git hook management operations (business logic)
// AES402: All primitive types in this contract have been replaced with taxonomy VOs.
//   * `String` returns → `DescriptionVO` (semantic description text)
//   * `HashMap<String, serde_json::Value>` → `GitDiffDataVO` (strongly-typed diff)
//   * `&str file_path` params → kept as `&str` (idiomatic borrow, AES402 allows)
//   * `bool remove` → kept (semantic toggle, AES402 allows)
use crate::common::taxonomy_job_vo::SuccessStatus;
use crate::common::taxonomy_layer_vo::Identity;
use crate::common::taxonomy_path_vo::FilePath;
use crate::common::taxonomy_suggestion_vo::DescriptionVO;
use crate::git_hooks::taxonomy_git_diff_data_vo::{GitDiffDataVO, HookIgnoreUpdateVO};
use crate::git_hooks::taxonomy_hook_error::GitHookError;
use async_trait::async_trait;

#[async_trait]
pub trait IHookProtocol: Send + Sync {
    /// Install pre-commit hook.
    async fn install_pre_commit(
        &self,
        executable_path: &FilePath,
    ) -> Result<SuccessStatus, GitHookError>;

    /// Uninstall pre-commit hook.
    async fn uninstall_pre_commit(&self) -> Result<SuccessStatus, GitHookError>;

    /// Get hook manager identity.
    fn get_hook_manager_identity(&self) -> Identity;

    /// Initialize git hooks config at the given project path.
    /// Returns a description of the result (e.g. "ALREADY_EXISTS:..." or
    /// "Initialized ..."). The description is a description VO so callers can
    /// introspect, translate, or log it without parsing strings.
    async fn initialize_config(&self, path: &str) -> DescriptionVO;

    /// Update the ignore list: add or remove a single rule.
    /// Returns a description of the operation.
    fn update_ignore_rule(&self, request: HookIgnoreUpdateVO) -> DescriptionVO;

    /// Get diff data between two file paths. Returns a strongly-typed VO;
    /// no raw JSON in the contract surface.
    async fn get_diff_data(&self, path1: &str, path2: &str) -> GitDiffDataVO;
}
