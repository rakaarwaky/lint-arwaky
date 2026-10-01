// PURPOSE: git-hooks-domain capability contracts (AES102 `_protocol`).
//
// One file for the git-hooks feature. Four traits, four business FRs:
// FR-001 = IDiffDetectionProtocol (diff + check), FR-002 = IHookInstallProtocol,
// FR-003 = IHookUninstallProtocol, FR-004 = IConfigInitProtocol.
// Utility-only protocols (IDiffDataProtocol, IIgnoreRuleProtocol, IHookCheckProtocol)
// have been removed — they were not business capabilities.

use crate::taxonomy_git_hooks_error::GitHookError;
use crate::taxonomy_git_hooks_vo::HookIgnoreUpdateVO;
use shared_common::taxonomy_git_vo::GitBranchName;
use shared_common::taxonomy_job_vo::SuccessStatus;
use shared_common::taxonomy_layer_vo::Identity;
use shared_common::taxonomy_lint_result_vo::LintResultList;
use shared_common::taxonomy_path_vo::FilePath;
use shared_common::taxonomy_paths_vo::FilePathList;
use shared_common::taxonomy_suggestion_vo::DescriptionVO;
use shared_file_watch::taxonomy_file_watch_vo::GitDiffResultVO;

/// FR-GitHooks-001: identify changed files via git diff and run the lint
/// pipeline over them. One seam that owns both detection and execution.
pub trait IDiffDetectionProtocol: Send + Sync {
    /// Get detailed diff result for a path.
    fn get_diff(&self, path: &FilePath) -> GitDiffResultVO;

    /// Get list of changed files against an explicit base branch.
    fn get_changed_files(&self, path: &FilePath, base: &GitBranchName) -> FilePathList;

    /// Get default branch name for a repository.
    fn get_default_branch(&self, path: &FilePath) -> GitBranchName;

    /// Run the lint check over the files changed since the default branch.
    /// Returns lint results; empty when no lintable files changed.
    fn run_git_diff_check(&self, path: &FilePath) -> LintResultList;
}

/// FR-GitHooks-001 (lint half): run the per-file lint pipeline over an
/// explicit list of changed files. Split out of IDiffDetectionProtocol so the
/// diff checker only detects and this seam executes — the checker no longer
/// stubs the check away (issue #582).
pub trait IChangedFilesLintProtocol: Send + Sync {
    /// Lint the given files with the per-file rule groups (quality, role,
    /// import, naming). Returns one result per violation; files that cannot
    /// be read produce an E902 marker result instead of being skipped.
    fn lint_changed_files(&self, files: &FilePathList) -> LintResultList;
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

/// FR-GitHooks-004: initialize and manage the project config for git hooks.
/// Covers config file creation and ignore-rule management.
pub trait IConfigInitProtocol: Send + Sync {
    /// Initialize git hooks config at the given project path.
    /// Returns a description of the result (e.g. "ALREADY_EXISTS:..." or
    /// "Initialized ...").
    fn initialize_config(&self, path: &str) -> DescriptionVO;

    /// Update the ignore list: add or remove a single rule.
    /// Returns a description of the operation.
    fn update_ignore_rule(&self, request: HookIgnoreUpdateVO) -> DescriptionVO;
}
