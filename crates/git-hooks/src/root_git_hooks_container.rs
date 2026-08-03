// PURPOSE: GitContainer — composition root that wires Capabilities to Contract traits and bootstraps the git hooks subsystem (root layer)

use shared::common::FilePath;
use shared::filesystem::contract_filesystem_aggregate::IFilesystemAggregate;
use shared::git_hooks::{GitHooksAggregate, IDiffProtocol, IHookManagerProtocol, IHookProtocol};

use std::sync::Arc;

pub struct GitContainer {
    aggregate: Arc<dyn GitHooksAggregate>,
}

impl GitContainer {
    pub fn new(root_dir: FilePath, filesystem: Arc<dyn IFilesystemAggregate>) -> Self {
        let hook_adapter: Arc<dyn IHookManagerProtocol> = Arc::new(
            crate::capabilities_hook_adapter::GitHookAdapter::new(root_dir, filesystem.clone()),
        );

        let diff_protocol: Arc<dyn IDiffProtocol> = Arc::new(
            crate::capabilities_diff_checker::DiffChecker::new(filesystem.clone()),
        );
        let hook_adapter_clone = Arc::clone(&hook_adapter);
        let hook_protocol: Arc<dyn IHookProtocol> =
            Arc::new(crate::capabilities_hook_manager::HookManager::new(
                hook_adapter_clone,
                filesystem.clone(),
            ));

        let aggregate: Arc<dyn GitHooksAggregate> = Arc::new(
            crate::agent_git_hooks_orchestrator::GitHooksOrchestrator::new(
                diff_protocol,
                hook_protocol,
                hook_adapter,
            ),
        );

        Self { aggregate }
    }

    pub fn aggregate(&self) -> Arc<dyn GitHooksAggregate> {
        self.aggregate.clone()
    }
}
