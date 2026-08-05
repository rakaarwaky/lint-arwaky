// PURPOSE: Shared composition root — resolves all domain containers into service
// orchestrators. Used by CLI, MCP, and TUI entry points to avoid duplicating
// container wiring (AES305 duplicate-code consolidation).
use shared::auto_fix::LintFixOrchestratorAggregate;
use shared::common::taxonomy_path_vo::FilePath;
use shared::common::taxonomy_definition_vo::LayerMapVO;
use shared::config_system::contract_config_orchestrator_aggregate::IConfigOrchestratorAggregate;
use shared::external_lint::IExternalLintAggregate;
use shared::filesystem::contract_filesystem_aggregate::IFilesystemAggregate;
use shared::git_hooks::contract_git_hooks_aggregate::GitHooksAggregate;
use shared::import_rules::IImportRunnerAggregate;
use shared::maintenance::contract_maintenance_aggregate::MaintenanceCommandsAggregate;
use shared::naming_rules::INamingRunnerAggregate;
use shared::orphan_rules::IOrphanAggregate;
use shared::project_setup::contract_setup_aggregate::SetupManagementAggregate;
use shared::quality_rules::contract_code_analysis_aggregate::ICodeAnalysisAggregate;
use shared::role_rules::IRoleRunnerAggregate;
use std::sync::Arc;

// ─── Block 1: Struct Definition ───────────────────────────

pub struct RootLintServices {
    pub filesystem: Arc<dyn IFilesystemAggregate>,
    pub config_orchestrator: Arc<dyn IConfigOrchestratorAggregate>,
    pub code_analysis_linter: Arc<dyn ICodeAnalysisAggregate>,
    pub import_orchestrator: Arc<dyn IImportRunnerAggregate>,
    pub naming_orchestrator: Arc<dyn INamingRunnerAggregate>,
    pub orphan_orchestrator: Arc<dyn IOrphanAggregate>,
    pub external_lint: Arc<dyn IExternalLintAggregate>,
    pub role_orchestrator: Arc<dyn IRoleRunnerAggregate>,
    pub fix_orchestrator: Arc<dyn LintFixOrchestratorAggregate>,
    pub git_hooks_aggregate: Arc<dyn GitHooksAggregate>,
    pub maintenance_orchestrator: Arc<dyn MaintenanceCommandsAggregate>,
    pub setup_orchestrator: Arc<dyn SetupManagementAggregate>,
}

// ─── Block 2: Aggregate Trait Implementation ──────────────

impl RootLintServices {
    /// Resolve all domain containers into service orchestrators.
    /// `workspace_root` is the path passed to container constructors (typically `"."`).
    pub fn new(workspace_root: &str) -> Self {
        let filesystem: Arc<dyn IFilesystemAggregate> =
            filesystem::root_filesystem_container::FilesystemContainer::new().orchestrator();

        let config_container =
            config_system::root_config_system_container::ConfigContainer::new(filesystem.clone());
        let config_orchestrator = config_container.orchestrator();

        let code_analysis_linter =
            quality_rules::root_quality_rules_container::CodeAnalysisContainer::from_orchestrator(
                &config_orchestrator,
                workspace_root,
            )
            .code_analysis_linter();

        let import_container =
            import_rules::root_import_rules_container::ImportContainer::from_orchestrator(
                &config_orchestrator,
                workspace_root,
                filesystem.clone(),
            );
        let import_orchestrator = import_container.orchestrator();

        let root_path = FilePath::new(workspace_root.to_string()).unwrap_or_default();
        let naming_container = naming_rules::root_naming_rules_container::NamingContainer::new(
            Arc::new(config_orchestrator.load_config_sync(&root_path)),
            Arc::new(LayerMapVO::new(
                config_orchestrator
                    .load_config_sync(&root_path)
                    .layers
                    .clone(),
            )),
        );
        let naming_orchestrator = naming_container.orchestrator();

        let orphan_container =
            orphan_rules::root_orphan_detector_container::OrphanContainer::from_orchestrator(
                &config_orchestrator,
                workspace_root,
                filesystem.clone(),
            );
        let orphan_orchestrator = orphan_container.analyzer();

        let ext_container = external_lint::root_external_lint_container::ExternalLintContainer::new(
            filesystem.clone(),
        );
        let external_lint = ext_container.aggregate();

        let role_container =
            role_rules::root_role_rules_container::RoleContainer::new_with_config(
                config_orchestrator.load_config_sync(&root_path),
            );
        let role_orchestrator = role_container.orchestrator();

        let auto_fix_container =
            auto_fix::root_auto_fix_container::AutoFixContainer::new(code_analysis_linter.clone());
        let fix_orchestrator =
            auto_fix_container.orchestrator_with_filesystem(filesystem.clone());

        let git_container = git_hooks::root_git_hooks_container::GitContainer::new(
            root_path,
            filesystem.clone(),
        );
        let git_hooks_aggregate = git_container.aggregate();

        let maintenance_container =
            maintenance::root_maintenance_container::MaintenanceContainer::new(filesystem.clone());
        let maintenance_orchestrator = maintenance_container.orchestrator();

        let setup_container =
            project_setup::root_project_setup_container::SetupContainer::new(filesystem.clone());
        let setup_orchestrator = setup_container.aggregate();

        Self {
            filesystem,
            config_orchestrator,
            code_analysis_linter,
            import_orchestrator,
            naming_orchestrator,
            orphan_orchestrator,
            external_lint,
            role_orchestrator,
            fix_orchestrator,
            git_hooks_aggregate,
            maintenance_orchestrator,
            setup_orchestrator,
        }
    }
}
