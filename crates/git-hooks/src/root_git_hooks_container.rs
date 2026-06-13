// PURPOSE: GitContainer — wiring for git-hooks feature (root layer, wiring only)
use shared::git_hooks::contract_commands_aggregate::GitCommandsAggregate;
use shared::git_hooks::contract_diff_protocol::IDiffProtocol;
use shared::git_hooks::contract_git_hooks_aggregate::GitHooksAggregate;
use shared::git_hooks::contract_hook_protocol::IHookProtocol;
use shared::git_hooks::contract_manager_port::IHookManagerPort;
use shared::git_hooks::contract_orchestrator_aggregate::HookManagementOrchestratorAggregate;
use shared::source_parsing::contract_scanner_provider_port::IScannerProviderPort;
use std::sync::Arc;

pub struct GitContainer {
    aggregate: Arc<dyn GitHooksAggregate>,
}

impl GitContainer {
    pub fn new(
        scanner: Arc<dyn IScannerProviderPort>,
        hook_adapter: Arc<dyn IHookManagerPort>,
    ) -> Self {
        let diff_protocol: Arc<dyn IDiffProtocol> =
            Arc::new(crate::capabilities_diff_checker::DiffChecker::new(scanner));
        let hook_protocol: Arc<dyn IHookProtocol> =
            Arc::new(crate::capabilities_hook_manager::HookManager::new(hook_adapter));

        let aggregate: Arc<dyn GitHooksAggregate> = Arc::new(
            crate::agent_git_hooks_orchestrator::GitHooksOrchestrator::new(
                diff_protocol,
                hook_protocol,
                crate::agent_git_hooks_orchestrator::SimpleHookManager::static_instance(),
            ),
        );

        Self { aggregate }
    }

    pub fn aggregate(&self) -> Arc<dyn GitHooksAggregate> {
        self.aggregate.clone()
    }
}