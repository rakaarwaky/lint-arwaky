// PURPOSE: GitContainer — wiring for git-hooks feature (root layer, wiring only)
use std::sync::Arc;

use shared::git_hooks::contract_diff_protocol::IDiffProtocol;
use shared::git_hooks::contract_git_command_port::IGitCommandPort;
use shared::git_hooks::contract_git_file_check_port::IGitFileCheckPort;
use shared::git_hooks::contract_git_hooks_aggregate::GitHooksAggregate;
use shared::git_hooks::contract_hook_protocol::IHookProtocol;
use shared::git_hooks::contract_manager_port::IHookManagerPort;

// Block 1: struct Definition
pub struct GitContainer {
    aggregate: Arc<dyn GitHooksAggregate>,
}

// ─── Block 2: Public Contract ─────────────────────────────
// (No trait impl — root container is wiring only)

// Block 3: constructors & public API
impl GitContainer {
    pub fn aggregate(&self) -> Arc<dyn GitHooksAggregate> {
        self.aggregate.clone()
    }

    pub fn new(
        git_command: Arc<dyn IGitCommandPort>,
        file_check: Arc<dyn IGitFileCheckPort>,
        hook_adapter: Arc<dyn IHookManagerPort>,
    ) -> Self {
        let diff_protocol: Arc<dyn IDiffProtocol> = Arc::new(
            crate::capabilities_diff_checker::DiffChecker::new(git_command),
        );
        let hook_protocol: Arc<dyn IHookProtocol> = Arc::new(
            crate::capabilities_hook_manager::HookManager::new(hook_adapter.clone(), file_check),
        );

        let aggregate: Arc<dyn GitHooksAggregate> = Arc::new(
            crate::agent_git_hooks_orchestrator::GitHooksOrchestrator::new(
                diff_protocol,
                hook_protocol,
                hook_adapter,
            ),
        );

        Self { aggregate }
    }

    pub fn new_default() -> Self {
        let git_command: Arc<dyn IGitCommandPort> =
            Arc::new(crate::infrastructure_git_command_adapter::GitCommandAdapter::new());
        let file_check: Arc<dyn IGitFileCheckPort> = Arc::new(
            crate::infrastructure_file_system_check_adapter::FileSystemCheckAdapter::new(),
        );
        let hook_adapter: Arc<dyn IHookManagerPort> =
            Arc::new(crate::infrastructure_hook_adapter::GitHookAdapter::new(
                shared::common::taxonomy_path_vo::FilePath::new(".".to_string())
                    .unwrap_or_default(),
            ));
        Self::new(git_command, file_check, hook_adapter)
    }
}
