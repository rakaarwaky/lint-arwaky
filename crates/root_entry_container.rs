// PURPOSE: Common dependency wiring shared across CLI, MCP, and TUI entry points.
// Root layer — thin composition that constructs all container orchestrators.
use std::sync::Arc;

use dispatcher::surface_orphan_action::OrphanFactory;
use shared::auto_fix::contract_fix_aggregate::LintFixOrchestratorAggregate;
use shared::config_system::contract_config_orchestrator_aggregate::IConfigOrchestratorAggregate;
use shared::external_lint::contract_external_lint_aggregate::IExternalLintAggregate;
use shared::filesystem::contract_filesystem_aggregate::IFilesystemAggregate;
use shared::git_hooks::contract_git_hooks_aggregate::GitHooksAggregate;
use shared::import_rules::contract_import_runner_aggregate::IImportRunnerAggregate;
use shared::maintenance::contract_maintenance_aggregate::MaintenanceCommandsAggregate;
use shared::naming_rules::contract_naming_runner_aggregate::INamingRunnerAggregate;
use shared::orphan_rules::contract_orphan_aggregate::IOrphanAggregate;
use shared::project_setup::contract_setup_aggregate::SetupManagementAggregate;
use shared::quality_rules::contract_code_analysis_aggregate::ICodeAnalysisAggregate;
use shared::role_rules::contract_role_runner_aggregate::IRoleRunnerAggregate;

/// All shared dependencies constructed once and consumed by entry points.
pub struct CommonDeps {
    pub filesystem: Arc<dyn IFilesystemAggregate>,
    pub config_orchestrator: Arc<dyn IConfigOrchestratorAggregate>,
    pub code_analysis_linter: Arc<dyn ICodeAnalysisAggregate>,
    pub import_orchestrator: Arc<dyn IImportRunnerAggregate>,
    pub naming_orchestrator: Arc<dyn INamingRunnerAggregate>,
    pub orphan_orchestrator: Arc<dyn IOrphanAggregate>,
    pub external_lint: Arc<dyn IExternalLintAggregate>,
    pub role_orchestrator: Arc<dyn IRoleRunnerAggregate>,
    pub maintenance_orchestrator: Arc<dyn MaintenanceCommandsAggregate>,
    pub setup_orchestrator: Arc<dyn SetupManagementAggregate>,
    pub git_hooks_aggregate: Arc<dyn GitHooksAggregate>,
    pub fix_orchestrator_factory:
        Arc<dyn Fn(bool) -> Arc<dyn LintFixOrchestratorAggregate> + Send + Sync>,
    pub fs_factory: Arc<dyn Fn() -> Arc<dyn IFilesystemAggregate> + Send + Sync>,
    pub orphan_factory: Arc<OrphanFactory>,
}

impl CommonDeps {
    /// Build every container orchestrator with a single "." project root.
    pub fn build() -> Self {
        let filesystem: Arc<dyn IFilesystemAggregate> =
            filesystem::root_filesystem_container::FilesystemContainer::new().orchestrator();

        let config_container =
            config_system::root_config_system_container::ConfigContainer::new(filesystem.clone());
        let config_orchestrator = config_container.orchestrator();

        let code_analysis_linter =
            quality_rules::root_quality_rules_container::CodeAnalysisContainer::from_orchestrator(
                &config_orchestrator,
                ".",
            )
            .code_analysis_linter();

        let import_container =
            import_rules::root_import_rules_container::ImportContainer::from_orchestrator(
                &config_orchestrator,
                ".",
                filesystem.clone(),
            );
        let import_orchestrator = import_container.orchestrator();

        let naming_container = naming_rules::root_naming_rules_container::NamingContainer::new(
            Arc::new(config_orchestrator.load_config_sync(
                &shared::common::taxonomy_path_vo::FilePath::new(".").unwrap_or_default(),
            )),
            Arc::new(shared::common::LayerMapVO::new(
                config_orchestrator
                    .load_config_sync(
                        &shared::common::taxonomy_path_vo::FilePath::new(".").unwrap_or_default(),
                    )
                    .layers
                    .clone(),
            )),
        );
        let naming_orchestrator = naming_container.orchestrator();

        let orphan_container =
            orphan_rules::root_orphan_detector_container::OrphanContainer::from_orchestrator(
                &config_orchestrator,
                ".",
                filesystem.clone(),
            );
        let orphan_orchestrator = orphan_container.analyzer();

        let ext_container =
            external_lint::root_external_lint_container::ExternalLintContainer::new(
                filesystem.clone(),
            );
        let external_lint = ext_container.aggregate();

        let role_container =
            role_rules::root_role_rules_container::RoleContainer::new_with_config(
                config_orchestrator.load_config_sync(
                    &shared::common::taxonomy_path_vo::FilePath::new(".").unwrap_or_default(),
                ),
            );
        let role_orchestrator = role_container.orchestrator();

        let auto_fix_container =
            auto_fix::root_auto_fix_container::AutoFixContainer::new(code_analysis_linter.clone());
        // BF-1: dry_run is now per-request via execute(path, dry_run), not baked into orchestrator.
        let fix_orchestrator_factory: Arc<
            dyn Fn(bool) -> Arc<dyn LintFixOrchestratorAggregate> + Send + Sync,
        > = {
            let container = auto_fix_container;
            let fs_for_factory = filesystem.clone();
            Arc::new(move |_dry| container.orchestrator_with_filesystem(fs_for_factory.clone()))
        };

        let fs_factory: Arc<dyn Fn() -> Arc<dyn IFilesystemAggregate> + Send + Sync> =
            Arc::new(|| filesystem::root_filesystem_container::FilesystemContainer::new().orchestrator());
        let orphan_factory: Arc<OrphanFactory> = Arc::new(|config, fs| {
            orphan_rules::root_orphan_detector_container::OrphanContainer::new_with_config(config, fs)
                .analyzer()
        });

        let git_container = git_hooks::root_git_hooks_container::GitContainer::new(
            shared::common::taxonomy_path_vo::FilePath::new(".").unwrap_or_default(),
            filesystem.clone(),
        );
        let git_hooks_aggregate = git_container.aggregate();

        let maintenance_container =
            maintenance::root_maintenance_container::MaintenanceContainer::new(filesystem.clone());
        let maintenance_orchestrator = maintenance_container.orchestrator();

        let setup_container =
            project_setup::root_project_setup_container::SetupContainer::new(filesystem.clone());
        let setup_orchestrator = setup_container.aggregate();

        CommonDeps {
            filesystem,
            config_orchestrator,
            code_analysis_linter,
            import_orchestrator,
            naming_orchestrator,
            orphan_orchestrator,
            external_lint,
            role_orchestrator,
            maintenance_orchestrator,
            setup_orchestrator,
            git_hooks_aggregate,
            fix_orchestrator_factory,
            fs_factory,
            orphan_factory,
        }
    }
}
