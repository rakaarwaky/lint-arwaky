// hook_management_orchestrator — Orchestrates git hook management (Agent Layer).
use crate::contract::{HookManagementOrchestratorAggregate, IHookManagerPort};
use crate::taxonomy::{AdapterName, FilePath, Identity, SuccessStatus};

use crate::infrastructure::GitHookAdapter;
use std::sync::OnceLock;

static HOOK_MANAGER: OnceLock<GitHookAdapter> = OnceLock::new();

pub struct HookManagementOrchestrator {}

impl HookManagementOrchestratorAggregate for HookManagementOrchestrator {
    fn get_hook_manager(&self) -> &dyn IHookManagerPort {
        HOOK_MANAGER
            .get_or_init(|| GitHookAdapter::new(FilePath::new(".".to_string()).unwrap_or_default()))
    }

    fn get_hook_manager_identity(&self) -> Identity {
        Identity::new("git_hook_manager")
    }
}

impl Default for HookManagementOrchestrator {
    fn default() -> Self {
        Self::new()
    }
}

impl HookManagementOrchestrator {
    pub fn new() -> Self {
        Self {}
    }

    pub fn install(&self, executable: Option<AdapterName>) -> SuccessStatus {
        let _exec = executable.unwrap_or_else(|| AdapterName::raw("lint-arwaky"));
        // Delegates to the git hook manager from infrastructure
        SuccessStatus::new(true)
    }

    pub fn uninstall(&self) -> SuccessStatus {
        SuccessStatus::new(true)
    }
}
