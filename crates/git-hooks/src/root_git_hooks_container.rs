// PURPOSE: GitContainer — wiring for git-hooks feature (root layer, wiring only)
// Wiring: HookManagementOrchestratorAggregate → GitHooksOrchestrator (agent layer)
// Wiring: IHookManagerProtocol → GitHookAdapter (capabilities layer)
use shared::filesystem::contract_filesystem_aggregate::IFilesystemAggregate;
use shared::git_hooks::{GitHooksAggregate, IDiffProtocol, IHookManagerProtocol, IHookProtocol};

use std::sync::Arc;
pub struct GitContainer {
    aggregate: Arc<dyn GitHooksAggregate>,
}

impl GitContainer {
    pub fn new(
        hook_adapter: Arc<dyn IHookManagerProtocol>,
        filesystem: Arc<dyn IFilesystemAggregate>,
    ) -> Self {
        let diff_protocol: Arc<dyn IDiffProtocol> = Arc::new(
            crate::capabilities_diff_checker::DiffChecker::new(filesystem.clone()),
        );
        let hook_adapter_clone = Arc::clone(&hook_adapter);
        let hook_protocol: Arc<dyn IHookProtocol> = Arc::new(
            crate::capabilities_hook_manager::HookManager::new(hook_adapter_clone),
        );

        let aggregate: Arc<dyn GitHooksAggregate> = Arc::new(
            crate::agent_git_hooks_orchestrator::GitHooksOrchestrator::new(
                diff_protocol,
                hook_protocol,
                hook_adapter,
            ),
        );

        Self { aggregate }
    }

    pub fn new_default() -> Self {
        let filesystem: Arc<dyn IFilesystemAggregate> =
            Arc::new(filesystem::FilesystemOrchestrator::new());
        let hook_adapter: Arc<dyn IHookManagerProtocol> =
            Arc::new(crate::capabilities_hook_adapter::GitHookAdapter::new(
                shared::common::taxonomy_path_vo::FilePath::new(".".to_string())
                    .unwrap_or_default(),
                filesystem.clone(),
            ));
        Self::new(hook_adapter, filesystem)
    }

    pub fn aggregate(&self) -> Arc<dyn GitHooksAggregate> {
        self.aggregate.clone()
    }
}
