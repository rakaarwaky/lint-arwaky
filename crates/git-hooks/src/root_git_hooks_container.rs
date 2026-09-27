// PURPOSE: GitContainer — composition root that wires Capabilities to Contract traits and bootstraps the git hooks subsystem (root layer)

use shared::common::FilePath;
use shared::filesystem::contract_filesystem_aggregate::IFilesystemAggregate;
use shared::filesystem::contract_filesystem_protocol::IFileSystemIOProtocol;
use shared::git_hooks::{IGitHooksAggregate, IHookInstallProtocol, IHookUninstallProtocol};

use std::sync::Arc;

pub struct GitContainer {
    aggregate: Arc<dyn IGitHooksAggregate>,
}

impl GitContainer {
    pub fn new(
        root_dir: FilePath,
        _filesystem: Arc<dyn IFilesystemAggregate>,
        io: Arc<dyn IFileSystemIOProtocol>,
    ) -> Self {
        let hook_installer: Arc<dyn IHookInstallProtocol> = Arc::new(
            crate::capabilities_hook_adapter::GitHookAdapter::new(root_dir.clone(), io.clone()),
        );
        let hook_uninstaller: Arc<dyn IHookUninstallProtocol> = Arc::new(
            crate::capabilities_hook_adapter::GitHookAdapter::new(root_dir, io.clone()),
        );
        let diff_checker = Arc::new(crate::capabilities_diff_checker::DiffChecker::new(
            io.clone(),
        ));
        let hook_manager = Arc::new(crate::capabilities_hook_manager::HookManager::new(
            hook_installer.clone(),
            hook_uninstaller.clone(),
            io,
        ));

        let aggregate: Arc<dyn IGitHooksAggregate> = Arc::new(
            crate::agent_git_hooks_orchestrator::GitHooksOrchestrator::new(
                diff_checker.clone(),
                diff_checker.clone(),
                hook_installer.clone(),
                hook_uninstaller.clone(),
                hook_manager.clone(),
                hook_manager.clone(),
                hook_manager.clone(),
            ),
        );

        Self { aggregate }
    }

    pub fn aggregate(&self) -> Arc<dyn IGitHooksAggregate> {
        self.aggregate.clone()
    }
}
