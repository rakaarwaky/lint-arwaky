// PURPOSE: McpContainer — DI wiring for MCP server aggregates
use std::sync::Arc;

use crate::agent_mcp_server_orchestrator::{McpServerDependencies, McpServerOrchestrator};
use shared::auto_fix::LintFixOrchestratorAggregate;
use shared::quality_rules::ICodeAnalysisAggregate;
use shared::common::taxonomy_path_vo::FilePath;
use shared::config_system::IConfigOrchestratorAggregate;
use shared::external_lint::IExternalLintAggregate;
use shared::filesystem::contract_filesystem_aggregate::IFilesystemAggregate;
use shared::git_hooks::GitHooksAggregate;
use shared::import_rules::IImportRunnerAggregate;
use shared::maintenance::MaintenanceCommandsAggregate;
use shared::naming_rules::INamingRunnerAggregate;
use shared::orphan_rules::IOrphanAggregate;
use shared::project_setup::SetupManagementAggregate;
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

    pub fn new_default() -> Self {
        // Filesystem orchestrator — shared across all containers
        let filesystem: Arc<dyn IFilesystemAggregate> =
            filesystem::root_filesystem_container::FilesystemContainer::new().orchestrator();

        // Create config orchestrator — single source of truth for config
        let config_container =
            config_system::root_config_system_container::ConfigContainer::new(filesystem.clone());
        let orchestrator = config_container.orchestrator();

        // Default project root for container initialization
        let default_root = FilePath::new(".").unwrap_or_default();
        let default_config = orchestrator.load_config_sync(&default_root);

        // Quality rules (code analysis)
        let code_analysis_linter =
            quality_rules::root_quality_rules_container::CodeAnalysisContainer::new(
                filesystem.clone(),
            )
            .code_analysis_linter();

        // Import rules
        let import_container =
            import_rules::root_import_rules_container::ImportContainer::new_with_config(
                default_config.clone(),
                filesystem.clone(),
            );
        let import_orchestrator = import_container.orchestrator();

        // Naming rules
        let naming_container = naming_rules::root_naming_rules_container::NamingContainer::new(
            Arc::new(default_config.clone()),
            Arc::new(shared::common::taxonomy_definition_vo::LayerMapVO::new(
                default_config.layers.clone(),
            )),
        );
        let naming_orchestrator = naming_container.orchestrator();

        // Orphan detector
        let orphan_container =
            orphan_rules::root_orphan_detector_container::OrphanContainer::new(filesystem.clone());
        let orphan_orchestrator = orphan_container.analyzer();

        // External linters
        let ext_container =
            external_lint::root_external_lint_container::ExternalLintContainer::new_default();
        let external_lint = ext_container.aggregate();

        // Role rules
        let role_container = role_rules::root_role_rules_container::RoleContainer::new_with_config(
            default_config.clone(),
        );
        let role_orchestrator = role_container.orchestrator();

        // Auto-fix orchestrator (uses same code analysis linter)
        let auto_fix_container =
            auto_fix::root_auto_fix_container::AutoFixContainer::new(code_analysis_linter.clone());
        let fix_orchestrator = auto_fix_container.orchestrator(false);

        // Git hooks aggregate
        let git_hooks_container = git_hooks::root_git_hooks_container::GitContainer::new_default();
        let git_hooks_aggregate = git_hooks_container.aggregate();

        // Maintenance orchestrator (doctor, security, dependencies)
        let maintenance_container =
            maintenance::root_maintenance_container::MaintenanceContainer::new();
        let maintenance_orchestrator = maintenance_container.orchestrator();

        // Setup orchestrator (init, install, mcp-config)
        let setup_container = project_setup::root_project_setup_container::SetupContainer::new();
        let setup_orchestrator = setup_container.aggregate();

        Self {
            code_analysis_linter,
            import_orchestrator,
            naming_orchestrator,
            orphan_orchestrator,
            external_lint,
            role_orchestrator,
            config_orchestrator: orchestrator,
            fix_orchestrator,
            git_hooks_aggregate,
            maintenance_orchestrator,
            setup_orchestrator,
            filesystem,
        }
    }
}
