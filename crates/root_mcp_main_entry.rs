// PURPOSE: MCP binary entry point — wiring all dependencies
use std::sync::Arc;

fn main() {
    let filesystem: Arc<dyn shared::filesystem::contract_filesystem_aggregate::IFilesystemAggregate> =
        filesystem::root_filesystem_container::FilesystemContainer::new().orchestrator();

    let config_container =
        config_system::root_config_system_container::ConfigContainer::new(filesystem.clone());
    let config_orchestrator = config_container.orchestrator();

    let code_analysis_linter =
        quality_rules::root_quality_rules_container::CodeAnalysisContainer::from_orchestrator(
            &config_orchestrator,
            ".",
            filesystem.clone(),
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

    let ext_container = external_lint::root_external_lint_container::ExternalLintContainer::new(
        filesystem.clone(),
        config_container.parser(),
    );
    let external_lint = ext_container.aggregate();

    let role_container = role_rules::root_role_rules_container::RoleContainer::new_with_config(
        config_orchestrator.load_config_sync(
            &shared::common::taxonomy_path_vo::FilePath::new(".").unwrap_or_default(),
        ),
    );
    let role_orchestrator = role_container.orchestrator();

    let auto_fix_container =
        auto_fix::root_auto_fix_container::AutoFixContainer::new(code_analysis_linter.clone());
    let fix_orchestrator =
        auto_fix_container.orchestrator_with_filesystem(false, filesystem.clone());

    let git_container =
        git_hooks::root_git_hooks_container::GitContainer::new(
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

    // TODO: wire MCP server and start
    println!("MCP binary — wiring complete");
}
