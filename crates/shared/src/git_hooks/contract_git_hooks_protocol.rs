// PURPOSE: git-hooks-domain capability contracts (AES102 `_protocol`).
//
// One file for the git-hooks feature. Each trait below is one capability
// seam: a trait carries every method that capability implements, with one
// concrete return type each, so a capability implements its trait outright
// and never carries unimplemented stubs. One trait per FR-GitHooks-001..007.

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

/// FR-GitHooks-001: identify files changed between HEAD and the default branch
/// using git diff, keeping only lintable source files.
pub trait IDiffDetectionProtocol: Send + Sync {
    /// Get detailed diff result for a path.
    fn get_diff(&self, path: &FilePath) -> GitDiffResultVO;

    /// Get list of changed files against an explicit base branch.
    fn get_changed_files(&self, path: &FilePath, base: &GitBranchName) -> FilePathList;

    /// Get default branch name for a repository.
    fn get_default_branch(&self, path: &FilePath) -> GitBranchName;
}

/// FR-GitHooks-002: install the pre-commit hook script into `.git/hooks/`.
pub trait IHookInstallProtocol: Send + Sync {
    /// Install the pre-commit hook script.
    fn install_pre_commit(&self, executable_path: &FilePath)
    -> Result<SuccessStatus, GitHookError>;

    /// Identity of the component that owns the hook lifecycle.
    fn get_hook_manager_identity(&self) -> Identity;
}

/// FR-GitHooks-003: remove the pre-commit hook script from `.git/hooks/`.
pub trait IHookUninstallProtocol: Send + Sync {
    /// Remove the pre-commit hook script. Idempotent.
    fn uninstall_pre_commit(&self) -> Result<SuccessStatus, GitHookError>;
}

/// FR-GitHooks-004: run the diff check and lint pipeline over changed files.
pub trait IHookCheckProtocol: Send + Sync {
    /// Run the lint check over the files changed since the default branch.
    fn run_git_diff_check(&self, path: &FilePath) -> LintResultList;
}

/// FR-GitHooks-005: compare two file paths and score their content difference.
pub trait IDiffDataProtocol: Send + Sync {
    /// Get diff data between two file paths. Returns a strongly-typed VO;
    /// no raw JSON in the contract surface.
    fn get_diff_data(&self, path1: &str, path2: &str) -> GitDiffDataVO;
}

/// FR-GitHooks-006: manage `ignored_paths` entries in the lint-arwaky config.
pub trait IIgnoreRuleProtocol: Send + Sync {
    /// Update the ignore list: add or remove a single rule.
    /// Returns a description of the operation.
    fn update_ignore_rule(&self, request: HookIgnoreUpdateVO) -> DescriptionVO;
}

/// FR-GitHooks-007: create the default config file when none exists.
pub trait IConfigInitProtocol: Send + Sync {
    /// Initialize git hooks config at the given project path.
    /// Returns a description of the result (e.g. "ALREADY_EXISTS:..." or
    /// "Initialized ..."). The description is a description VO so callers can
    /// introspect, translate, or log it without parsing strings.
    fn initialize_config(&self, path: &str) -> DescriptionVO;
}
