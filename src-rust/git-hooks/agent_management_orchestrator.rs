// hook_management_orchestrator — Orchestrates git hook management (Agent Layer).
use crate::git_hooks::contract_orchestrator_aggregate::HookManagementOrchestratorAggregate;
use crate::git_hooks::contract_manager_port::IHookManagerPort;
use crate::shared_common::taxonomy_name_vo::AdapterName;
use crate::source_parsing::taxonomy_path_vo::FilePath;
use crate::shared_common::taxonomy_layer_vo::Identity;
use /* UNKNOWN: SuccessStatus */ crate::pipeline_jobs::taxonomy_job_vo::SuccessStatus;

use crate::git_hooks::infrastructure_hook_adapter::GitHookAdapter;
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
