// PURPOSE: GitHooksOrchestrator — orchestrates git hooks operations by delegating to protocols/ports only (agent layer)
use shared::git_hooks::contract_commands_aggregate::GitCommandsAggregate;
use shared::git_hooks::contract_diff_protocol::IDiffProtocol;
use shared::git_hooks::contract_git_hooks_aggregate::GitHooksAggregate;
use shared::git_hooks::contract_hook_protocol::IHookProtocol;
use shared::git_hooks::contract_manager_port::IHookManagerPort;
use shared::git_hooks::contract_orchestrator_aggregate::HookManagementOrchestratorAggregate;
use shared::git_hooks::taxonomy_diff_result_vo::GitDiffResultVO;
use shared::git_hooks::taxonomy_hook_error::GitHookError;
use shared::output_report::taxonomy_result_vo::LintResultList;
use shared::pipeline_jobs::taxonomy_job_vo::SuccessStatus;
use shared::source_parsing::taxonomy_path_vo::FilePath;
use std::sync::Arc;
use async_trait::async_trait;

pub struct GitHooksOrchestrator {
    diff_protocol: Arc<dyn IDiffProtocol>,
    hook_protocol: Arc<dyn IHookProtocol>,
    hook_manager: Arc<dyn IHookManagerPort>,
}

impl GitHooksOrchestrator {
    pub fn new(
        diff_protocol: Arc<dyn IDiffProtocol>,
        hook_protocol: Arc<dyn IHookProtocol>,
        hook_manager: Arc<dyn IHookManagerPort>,
    ) -> Self {
        Self { diff_protocol, hook_protocol, hook_manager }
    }
}

#[async_trait::async_trait]
impl GitHooksAggregate for GitHooksOrchestrator {
    fn diff_protocol(&self) -> &dyn IDiffProtocol {
        self.diff_protocol.as_ref()
    }

    fn hook_protocol(&self) -> &dyn IHookProtocol {
        self.hook_protocol.as_ref()
    }

    async fn run_git_hooks_check(&self, path: &FilePath) -> LintResultList {
        self.diff_protocol().run_git_diff_check(path).await
    }

    async fn install_hook(&self, executable_path: &FilePath) -> Result<SuccessStatus, GitHookError> {
        self.hook_protocol().install_pre_commit(executable_path).await
    }

    async fn uninstall_hook(&self) -> Result<SuccessStatus, GitHookError> {
        self.hook_protocol().uninstall_pre_commit().await
    }
}

#[async_trait::async_trait]
impl GitCommandsAggregate for GitHooksOrchestrator {
    async fn run_git_diff_check(&self, path: &FilePath) -> LintResultList {
        self.diff_protocol().run_git_diff_check(path).await
    }

    async fn get_diff(&self, path: &FilePath) -> GitDiffResultVO {
        self.diff_protocol().get_diff(path).await
    }
}

#[async_trait::async_trait]
impl HookManagementOrchestratorAggregate for GitHooksOrchestrator {
    fn get_hook_manager(&self) -> &dyn IHookManagerPort {
        self.hook_manager.as_ref()
    }

    fn get_hook_manager_identity(&self) -> shared::taxonomy_layer_vo::Identity {
        self.hook_manager.get_hook_manager_identity()
    }
}
