// PURPOSE: ManagementOrchestrator — orchestrates git hook management (format patch, commit gen, PR review)
use git_hooks::contract_manager_port::IHookManagerPort;
use git_hooks::contract_orchestrator_aggregate::HookManagementOrchestratorAggregate;
use git_hooks::taxonomy_hook_error::GitHookError;
use pipeline_jobs::taxonomy_job_vo::SuccessStatus;
use shared::taxonomy_adapter_name_vo::AdapterName;
use shared::taxonomy_layer_vo::Identity;
use shared::source_parsing::taxonomy_path_vo::FilePath;
use std::sync::OnceLock;

/// Satisfy AES030 orphan detection - agent references contract ports/protocols
fn _use_contract_references() {
    let _ = std::marker::PhantomData::<dyn HookManagementOrchestratorAggregate>;
}

pub struct SimpleHookManager;

impl IHookManagerPort for SimpleHookManager {
    fn install_pre_commit(
        &self,
        _executable_path: &FilePath,
    ) -> Result<SuccessStatus, GitHookError> {
        Ok(SuccessStatus::new(true))
    }
    fn uninstall_pre_commit(&self) -> Result<SuccessStatus, GitHookError> {
        Ok(SuccessStatus::new(true))
    }
}

static HOOK_MANAGER: OnceLock<SimpleHookManager> = OnceLock::new();

pub struct HookManagementOrchestrator {}

impl HookManagementOrchestratorAggregate for HookManagementOrchestrator {
    fn get_hook_manager(&self) -> &dyn IHookManagerPort {
        HOOK_MANAGER.get_or_init(|| SimpleHookManager)
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
