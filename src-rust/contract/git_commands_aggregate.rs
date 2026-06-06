use crate::taxonomy::FilePath;
use crate::taxonomy::LintResultList;
use async_trait::async_trait;
use crate::contract::GitDiffResultAggregate;

#[async_trait]
pub trait GitCommandsAggregate: Send + Sync {
    async fn run_git_diff_check(&self, path: &FilePath) -> LintResultList;
    async fn get_diff(&self, path: &FilePath) -> GitDiffResultAggregate;
}
