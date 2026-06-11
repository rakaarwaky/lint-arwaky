// PURPOSE: HookOrchestratorAggregate — aggregate trait for hook orchestration
use git_hooks::contract_manager_port::IHookManagerPort;
use shared_common::taxonomy_layer_vo::Identity;

pub trait HookManagementOrchestratorAggregate: Send + Sync {
    fn get_hook_manager(&self) -> &dyn IHookManagerPort;
    fn get_hook_manager_identity(&self) -> Identity;
}
