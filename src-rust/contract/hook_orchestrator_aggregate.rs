use crate::contract::IHookManagerPort;
use crate::taxonomy::Identity;

pub trait HookManagementOrchestratorAggregate: Send + Sync {
    fn get_hook_manager(&self) -> &dyn IHookManagerPort;
    fn get_hook_manager_identity(&self) -> Identity;
}
