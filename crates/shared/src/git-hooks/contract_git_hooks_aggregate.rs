// PURPOSE: GitHooksAggregate — unified aggregate trait for git hooks orchestration
use crate::git_hooks::contract_diff_protocol::IDiffProtocol;
use crate::git_hooks::contract_hook_protocol::IHookProtocol;
use async_trait::async_trait;

#[async_trait]
pub trait GitHooksAggregate: Send + Sync {
    /// Access to diff protocol (read operations)
    fn diff_protocol(&self) -> &dyn IDiffProtocol;

    /// Access to hook protocol (write/management operations)
    fn hook_protocol(&self) -> &dyn IHookProtocol;

    /// Run full git hooks check on a path
    async fn run_git_hooks_check(&self, path: &crate::source_parsing::taxonomy_path_vo::FilePath) -> crate::output_report::taxonomy_result_vo::LintResultList {
        self.diff_protocol().run_git_diff_check(path).await
    }

    /// Install pre-commit hook
    async fn install_hook(&self, executable_path: &crate::source_parsing::taxonomy_path_vo::FilePath) -> Result<crate::pipeline_jobs::taxonomy_job_vo::SuccessStatus, crate::git_hooks::taxonomy_hook_error::GitHookError> {
        self.hook_protocol().install_pre_commit(executable_path).await
    }

    /// Uninstall pre-commit hook
    async fn uninstall_hook(&self) -> Result<crate::pipeline_jobs::taxonomy_job_vo::SuccessStatus, crate::git_hooks::taxonomy_hook_error::GitHookError> {
        self.hook_protocol().uninstall_pre_commit().await
    }
}