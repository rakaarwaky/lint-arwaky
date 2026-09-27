// PURPOSE: git-hooks-domain capability contracts (AES102 `_protocol`).
//
// One file for the git-hooks feature. Each trait below is one capability
// seam: a trait carries every method that capability implements, with one
// concrete return type each, so a capability implements its trait outright
// and never carries unimplemented stubs.

use crate::common::taxonomy_git_vo::GitBranchName;
use crate::common::taxonomy_job_vo::SuccessStatus;
use crate::common::taxonomy_layer_vo::Identity;
use crate::common::taxonomy_lint_result_vo::LintResultList;
use crate::common::taxonomy_path_vo::FilePath;
use crate::common::taxonomy_paths_vo::FilePathList;
use crate::common::taxonomy_suggestion_vo::DescriptionVO;
use crate::file_watch::taxonomy_watch_config_vo::GitDiffResultVO;
use crate::git_hooks::taxonomy_git_diff_data_vo::{GitDiffDataVO, HookIgnoreUpdateVO};
use crate::git_hooks::taxonomy_hook_error::GitHookError;

pub trait IDiffProtocol: Send + Sync {
    /// Run lint check on git diff changes
    fn run_git_diff_check(&self, path: &FilePath) -> LintResultList;

    /// Get detailed diff result for a path
    fn get_diff(&self, path: &FilePath) -> GitDiffResultVO;

    /// Get list of changed files from git diff
    fn get_changed_files(&self, path: &FilePath, base: &GitBranchName) -> FilePathList;

    /// Get default branch name for a repository
    fn get_default_branch(&self, path: &FilePath) -> GitBranchName;
}

pub trait IHookProtocol: Send + Sync {
    /// Install pre-commit hook.
    fn install_pre_commit(&self, executable_path: &FilePath)
    -> Result<SuccessStatus, GitHookError>;

    /// Uninstall pre-commit hook.
    fn uninstall_pre_commit(&self) -> Result<SuccessStatus, GitHookError>;

    /// Get hook manager identity.
    fn get_hook_manager_identity(&self) -> Identity;

    /// Initialize git hooks config at the given project path.
    /// Returns a description of the result (e.g. "ALREADY_EXISTS:..." or
    /// "Initialized ..."). The description is a description VO so callers can
    /// introspect, translate, or log it without parsing strings.
    fn initialize_config(&self, path: &str) -> DescriptionVO;

    /// Update the ignore list: add or remove a single rule.
    /// Returns a description of the operation.
    fn update_ignore_rule(&self, request: HookIgnoreUpdateVO) -> DescriptionVO;

    /// Get diff data between two file paths. Returns a strongly-typed VO;
    /// no raw JSON in the contract surface.
    fn get_diff_data(&self, path1: &str, path2: &str) -> GitDiffDataVO;
}

pub trait IHookManagerProtocol: Send + Sync {
    fn install_pre_commit(&self, executable_path: &FilePath)
    -> Result<SuccessStatus, GitHookError>;
    fn uninstall_pre_commit(&self) -> Result<SuccessStatus, GitHookError>;
}
