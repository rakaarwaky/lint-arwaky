// PURPOSE: HookOrchestratorAggregate — aggregate trait for hook orchestration
use crate::common::taxonomy_layer_vo::Identity;
use crate::git_hooks::contract_manager_protocol::IHookManagerProtocol;

/// Aggregate that orchestrates Git hook execution.
///
/// Implementations provide access to the active hook manager and its
/// identity, allowing the surface layer to install, remove, or invoke
/// hooks through a single entry point.
pub trait HookManagementOrchestratorAggregate: Send + Sync {
    fn get_hook_manager(&self) -> &dyn IHookManagerProtocol;
    fn get_hook_manager_identity(&self) -> Identity;
}
