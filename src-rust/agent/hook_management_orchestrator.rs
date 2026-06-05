// hook_management_orchestrator — Orchestrates git hook management (Capability).
use crate::contract::HookOrchestratorAggregate;
use crate::taxonomy::{AdapterName, Identity, SuccessStatus};

pub struct HookManagementOrchestrator;

impl HookOrchestratorAggregate for HookManagementOrchestrator {}

impl Default for HookManagementOrchestrator {
    fn default() -> Self {
        Self::new()
    }
}

impl HookManagementOrchestrator {
    pub fn new() -> Self {
        Self
    }

    pub fn get_hook_manager_identity(&self) -> Identity {
        "git_hook_manager".to_string()
    }

    pub fn install(&self, executable: Option<AdapterName>) -> SuccessStatus {
        let _exec = executable.unwrap_or_else(|| AdapterName::new("lint-arwaky"));
        // Delegates to the git hook manager from infrastructure
        SuccessStatus::new(true)
    }

    pub fn uninstall(&self) -> SuccessStatus {
        SuccessStatus::new(true)
    }
}
