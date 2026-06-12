// PURPOSE: GitContainer — wiring for git-hooks feature (root layer, wiring only)
use std::sync::Arc;
use shared::git_hooks::contract_commands_aggregate::GitCommandsAggregate;
use shared::git_hooks::contract_orchestrator_aggregate::HookManagementOrchestratorAggregate;

pub struct GitContainer {
    commands_aggregate: Arc<dyn GitCommandsAggregate>,
    orchestrator_aggregate: Arc<dyn HookManagementOrchestratorAggregate>,
}

impl GitContainer {
    pub fn new() -> Self {
        Self {
            commands_aggregate: Arc::new(
                crate::agent_commands_orchestrator::GitCommandsOrchestrator::new(),
            ),
            orchestrator_aggregate: Arc::new(
                crate::agent_management_orchestrator::HookManagementOrchestrator::new(),
            ),
        }
    }

    pub fn commands_aggregate(&self) -> Arc<dyn GitCommandsAggregate> {
        self.commands_aggregate.clone()
    }

    pub fn orchestrator_aggregate(&self) -> Arc<dyn HookManagementOrchestratorAggregate> {
        self.orchestrator_aggregate.clone()
    }
}
impl Default for GitContainer {
    fn default() -> Self {
        Self::new()
    }
}

