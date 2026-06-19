// PURPOSE: IDiffProtocol — protocol for git diff analysis operations (business logic)
use crate::cli_commands::taxonomy_result_vo::LintResultList;
use crate::git_hooks::taxonomy_diff_result_vo::GitDiffResultVO;
use crate::source_parsing::taxonomy_path_vo::FilePath;
use crate::source_parsing::taxonomy_paths_vo::FilePathList;
use async_trait::async_trait;

#[async_trait]
pub trait IDiffProtocol: Send + Sync {
    /// Run lint check on git diff changes
    async fn run_git_diff_check(&self, path: &FilePath) -> LintResultList;

    /// Get detailed diff result for a path
    async fn get_diff(&self, path: &FilePath) -> GitDiffResultVO;

    /// Get list of changed files from git diff
    async fn get_changed_files(&self, path: &FilePath) -> FilePathList;

    /// Get default branch name for a repository
    async fn get_default_branch(&self, path: &FilePath) -> String;
}
