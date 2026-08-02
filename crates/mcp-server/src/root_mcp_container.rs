// PURPOSE: McpContainer — DI wiring for MCP server aggregates
//
// Follows the project pattern: aggregate for cross-crate DI, protocol for intra-crate wiring.
// Each container is constructed with explicit DI parameters — no Default impls for production use.

use std::sync::Arc;

use crate::agent_mcp_server_orchestrator::{McpServerDependencies, McpServerOrchestrator};
use auto_fix::capabilities_file_adapter::FileAdapter;
use auto_fix::root_auto_fix_container::AutoFixContainer;
use git_hooks::root_git_hooks_container::GitContainer;
use maintenance::root_maintenance_container::MaintenanceContainer;
use project_setup::root_project_setup_container::SetupContainer;
use shared::auto_fix::LintFixOrchestratorAggregate;
use shared::common::LayerMapVO;
use shared::config_system::IConfigOrchestratorAggregate;
use shared::external_lint::IExternalLintAggregate;
use shared::filesystem::contract_filesystem_aggregate::IFilesystemAggregate;
use shared::git_hooks::GitHooksAggregate;
use shared::import_rules::IImportRunnerAggregate;
use shared::maintenance::MaintenanceCommandsAggregate;
use shared::naming_rules::INamingRunnerAggregate;
use shared::orphan_rules::IOrphanAggregate;
use shared::project_setup::SetupManagementAggregate;
use shared::quality_rules::ICodeAnalysisAggregate;
use shared::role_rules::IRoleRunnerAggregate;

pub struct McpContainer {
    pub code_analysis_linter: Arc<dyn ICodeAnalysisAggregate>,
    pub import_orchestrator: Arc<dyn IImportRunnerAggregate>,
    pub naming_orchestrator: Arc<dyn INamingRunnerAggregate>,
    pub orphan_orchestrator: Arc<dyn IOrphanAggregate>,
    pub external_lint: Arc<dyn IExternalLintAggregate>,
    pub role_orchestrator: Arc<dyn IRoleRunnerAggregate>,
    pub config_orchestrator: Arc<dyn IConfigOrchestratorAggregate>,
    pub fix_orchestrator: Arc<dyn LintFixOrchestratorAggregate>,
    pub git_hooks_aggregate: Arc<dyn GitHooksAggregate>,
    pub maintenance_orchestrator: Arc<dyn MaintenanceCommandsAggregate>,
    pub setup_orchestrator: Arc<dyn SetupManagementAggregate>,
    pub filesystem: Arc<dyn IFilesystemAggregate>,
}

impl McpContainer {
    /// Get the MCP server orchestrator with all dependencies wired.
    pub fn orchestrator(&self) -> McpServerOrchestrator {
        let deps = McpServerDependencies {
            code_analysis_linter: self.code_analysis_linter.clone(),
            fix_orchestrator: self.fix_orchestrator.clone(),
            orphan_orchestrator: self.orphan_orchestrator.clone(),
            maintenance_orchestrator: self.maintenance_orchestrator.clone(),
            git_hooks_aggregate: self.git_hooks_aggregate.clone(),
            setup_orchestrator: self.setup_orchestrator.clone(),
            config_orchestrator: self.config_orchestrator.clone(),
            external_lint: self.external_lint.clone(),
            import_orchestrator: self.import_orchestrator.clone(),
            naming_orchestrator: self.naming_orchestrator.clone(),
            role_orchestrator: self.role_orchestrator.clone(),
            filesystem: self.filesystem.clone(),
        };
        McpServerOrchestrator::new(deps)
    }

    /// Create McpContainer with all dependencies explicitly injected.
    ///
    /// # Arguments
    /// * `project_root` - Root directory of the project to lint
    pub fn new(project_root: &str) -> Self {
        // 1. Filesystem — shared across all containers
        let filesystem: Arc<dyn IFilesystemAggregate> =
            filesystem::root_filesystem_container::FilesystemContainer::new().orchestrator();

        // 2. Config — single source of truth for architecture config
        let config_container =
            config_system::root_config_system_container::ConfigContainer::new(filesystem.clone());
        let config_orchestrator = config_container.orchestrator();
        let config_parser = config_container.parser();

        // Load architecture config for containers that need it directly
        use shared::common::FilePath;
        let fp = FilePath::new(project_root.to_string()).unwrap_or_default();
        let arch_config = config_orchestrator.load_config_sync(&fp);
        let layer_map = Arc::new(LayerMapVO::new(arch_config.layers.clone()));

        // 3. Quality rules (code analysis)
        let code_analysis_linter =
            quality_rules::root_quality_rules_container::CodeAnalysisContainer::from_orchestrator(
                &config_orchestrator,
                project_root,
                filesystem.clone(),
            )
            .code_analysis_linter();

        // 4. Import rules
        let import_container =
            import_rules::root_import_rules_container::ImportContainer::from_orchestrator(
                &config_orchestrator,
                project_root,
                filesystem.clone(),
            );
        let import_orchestrator = import_container.orchestrator();

        // 5. Naming rules
        let naming_container = naming_rules::root_naming_rules_container::NamingContainer::new(
            Arc::new(arch_config.clone()),
            layer_map,
        );
        let naming_orchestrator = naming_container.orchestrator();

        // 6. Orphan rules
        let orphan_container =
            orphan_rules::root_orphan_detector_container::OrphanContainer::from_orchestrator(
                &config_orchestrator,
                project_root,
                filesystem.clone(),
            );
        let orphan_orchestrator = orphan_container.analyzer();

        // 7. External lint (needs filesystem + config_parser)
        let ext_container = external_lint::root_external_lint_container::ExternalLintContainer::new(
            filesystem.clone(),
            config_parser,
        );
        let external_lint = ext_container.aggregate();

        // 8. Role rules
        let role_container = role_rules::root_role_rules_container::RoleContainer::new_with_config(
            arch_config.clone(),
        );
        let role_orchestrator = role_container.orchestrator();

        // 9. File adapter for auto-fix
        let file_adapter: Arc<dyn shared::auto_fix::IFileAdapterProtocol> =
            Arc::new(FileAdapter::new(filesystem.clone()));

        // 10. Auto-fix orchestrator (uses same code analysis linter)
        let auto_fix_container = AutoFixContainer::new(code_analysis_linter.clone());
        let fix_orchestrator = auto_fix_container.orchestrator(false, file_adapter.clone());

        // 11. Git hooks — container owns adapter construction internally
        let git_container = GitContainer::new(fp.clone(), filesystem.clone());
        let git_hooks_aggregate = git_container.aggregate();

        // 12. Maintenance (doctor, security, dependencies)
        let maintenance_container = MaintenanceContainer::new(filesystem.clone());
        let maintenance_orchestrator = maintenance_container.orchestrator();

        // 13. Setup (init, install, mcp-config)
        let setup_container = SetupContainer::new(filesystem.clone());
        let setup_orchestrator = setup_container.aggregate();

        Self {
            code_analysis_linter,
            import_orchestrator,
            naming_orchestrator,
            orphan_orchestrator,
            external_lint,
            role_orchestrator,
            config_orchestrator,
            fix_orchestrator,
            git_hooks_aggregate,
            maintenance_orchestrator,
            setup_orchestrator,
            filesystem,
        }
    }
}
