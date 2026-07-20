// PURPOSE: GitContainer — wiring for git-hooks feature (root layer, wiring only)
// Wiring: HookManagementOrchestratorAggregate → GitHooksOrchestrator (agent layer)
// Wiring: IHookManagerProtocol → GitHookAdapter (infrastructure layer)
use shared::git_hooks::contract_diff_protocol::IDiffProtocol;
use shared::git_hooks::contract_git_hooks_aggregate::GitHooksAggregate;
use shared::git_hooks::contract_hook_protocol::IHookProtocol;
use shared::git_hooks::contract_manager_protocol::IHookManagerProtocol;
use std::sync::Arc;

pub struct GitContainer {
    aggregate: Arc<dyn GitHooksAggregate>,
}

impl GitContainer {
    pub fn new(hook_adapter: Arc<dyn IHookManagerProtocol>) -> Self {
        let diff_protocol: Arc<dyn IDiffProtocol> =
            Arc::new(crate::capabilities_diff_checker::DiffChecker::new());
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
        let hook_adapter: Arc<dyn IHookManagerProtocol> =
            Arc::new(crate::infrastructure_hook_adapter::GitHookAdapter::new(
                shared::common::taxonomy_path_vo::FilePath::new(".".to_string())
                    .unwrap_or_default(),
            ));
        Self::new(hook_adapter)
    }

    pub fn aggregate(&self) -> Arc<dyn GitHooksAggregate> {
        self.aggregate.clone()
    }
}
