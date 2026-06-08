use crate::git_hooks::contract_result_aggregate::GitDiffResultAggregate;
use crate::source_parsing::taxonomy_path_vo::FilePath;
use crate::output_report::taxonomy_result_vo::LintResultList;
use async_trait::async_trait;

#[async_trait]
pub trait GitCommandsAggregate: Send + Sync {
    async fn run_git_diff_check(&self, path: &FilePath) -> LintResultList;
    async fn get_diff(&self, path: &FilePath) -> GitDiffResultAggregate;
}
