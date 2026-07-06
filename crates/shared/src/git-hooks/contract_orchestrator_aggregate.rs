// PURPOSE: HookOrchestratorAggregate — aggregate trait for hook orchestration
use crate::common::taxonomy_layer_vo::Identity;
use crate::git_hooks::contract_manager_port::IHookManagerPort;

pub trait HookManagementOrchestratorAggregate: Send + Sync {
    fn get_hook_manager(&self) -> &dyn IHookManagerPort;
    fn get_hook_manager_identity(&self) -> Identity;
}
