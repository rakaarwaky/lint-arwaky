// PURPOSE: GitContainer — composition root that wires Capabilities to Contract traits and bootstraps the git hooks subsystem (root layer)

use shared_common::FilePath;
use shared_filesystem::contract_filesystem_aggregate::IFilesystemAggregate;
use shared_filesystem::contract_filesystem_protocol::IFileSystemIOProtocol;
use shared_git_hooks::{IGitHooksAggregate, IHookInstallProtocol, IHookUninstallProtocol};
use shared_quality_rules::ICodeAnalysisAggregate;

use std::sync::Arc;

pub struct GitContainer {
    aggregate: Arc<dyn IGitHooksAggregate>,
}

impl GitContainer {
    pub fn new(
        root_dir: FilePath,
        _filesystem: Arc<dyn IFilesystemAggregate>,
        io: Arc<dyn IFileSystemIOProtocol>,
        code_analysis_linter: Arc<dyn ICodeAnalysisAggregate>,
    ) -> Self {
        let hook_installer: Arc<dyn IHookInstallProtocol> = Arc::new(
            crate::capabilities_hook_installer::HookInstaller::new(root_dir.clone(), io.clone()),
        );
        let hook_uninstaller: Arc<dyn IHookUninstallProtocol> =
            Arc::new(crate::capabilities_hook_uninstaller::HookUninstaller::new(
                root_dir.value.clone(),
                io.clone(),
            ));
        let diff_checker = Arc::new(crate::capabilities_diff_checker::DiffChecker::new(
            io.clone(),
            code_analysis_linter,
        ));
        let config_init = Arc::new(crate::capabilities_config_init::ConfigInit::new(io));

        let aggregate: Arc<dyn IGitHooksAggregate> = Arc::new(
            crate::agent_git_hooks_orchestrator::GitHooksOrchestrator::new(
                diff_checker.clone(),
                hook_installer.clone(),
                hook_uninstaller.clone(),
                config_init.clone(),
            ),
        );

        Self { aggregate }
    }

    pub fn aggregate(&self) -> Arc<dyn IGitHooksAggregate> {
        self.aggregate.clone()
    }
}
