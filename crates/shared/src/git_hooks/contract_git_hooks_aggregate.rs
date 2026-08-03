// PURPOSE: GitHooksAggregate — unified aggregate trait for git hooks orchestration
use crate::cli_commands::taxonomy_result_vo::LintResultList;
use crate::common::taxonomy_job_vo::SuccessStatus;
use crate::common::taxonomy_path_vo::FilePath;
use crate::common::taxonomy_suggestion_vo::DescriptionVO;
use crate::git_hooks::contract_diff_protocol::IDiffProtocol;
use crate::git_hooks::contract_hook_protocol::IHookProtocol;
use crate::git_hooks::taxonomy_git_diff_data_vo::{GitDiffDataVO, HookIgnoreUpdateVO};
use crate::git_hooks::taxonomy_hook_error::GitHookError;

pub trait GitHooksAggregate: Send + Sync {
    /// Access to diff protocol (read operations)
    fn diff_protocol(&self) -> &dyn IDiffProtocol;

    /// Access to hook protocol (write/management operations)
    fn hook_protocol(&self) -> &dyn IHookProtocol;

    /// Run full git hooks check on a path
    fn run_git_hooks_check(&self, path: &FilePath) -> LintResultList {
        self.diff_protocol().run_git_diff_check(path)
    }

    /// Install pre-commit hook
    fn install_hook(&self, executable_path: &FilePath) -> Result<SuccessStatus, GitHookError> {
        self.hook_protocol().install_pre_commit(executable_path)
    }

    /// Uninstall pre-commit hook
    fn uninstall_hook(&self) -> Result<SuccessStatus, GitHookError> {
        self.hook_protocol().uninstall_pre_commit()
    }

    /// Initialize config at project path.
    fn initialize_config(&self, path: &str) -> DescriptionVO {
        self.hook_protocol().initialize_config(path)
    }

    /// Add or remove an ignore rule in config.
    fn update_ignore_rule(&self, request: HookIgnoreUpdateVO) -> DescriptionVO {
        self.hook_protocol().update_ignore_rule(request)
    }

    /// Compare two file paths for diff data.
    fn get_diff_data(&self, path1: &str, path2: &str) -> GitDiffDataVO {
        self.hook_protocol().get_diff_data(path1, path2)
    }
}
