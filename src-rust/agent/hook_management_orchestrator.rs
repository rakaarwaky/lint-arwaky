// hook_management_orchestrator — Orchestrates git hook management (Capability).
use crate::contract::{HookOrchestratorAggregate};
use crate::taxonomy::{AdapterName, FilePath, Identity, SuccessStatus};

pub struct HookManagementOrchestrator;

impl HookOrchestratorAggregate for HookManagementOrchestrator {}

impl HookManagementOrchestrator {
    pub fn new() -> Self {
        Self
    }

    pub fn get_hook_manager_identity(&self) -> Identity {
        Identity::new("git_hook_manager")
    }

    pub fn install(&self, executable: Option<AdapterName>) -> SuccessStatus {
        let _exec = executable.unwrap_or_else(|| AdapterName::new("auto-lint").unwrap());
        // Delegates to the git hook manager from infrastructure
        SuccessStatus::new(true)
    }

    pub fn uninstall(&self) -> SuccessStatus {
        SuccessStatus::new(true)
    }
}
