//! Test helpers for `dispatcher` integration tests.

use shared_common::taxonomy_path_vo::FilePath;

#[allow(dead_code)]
pub fn fp(path: &str) -> FilePath {
    FilePath::new(path.to_string()).expect("valid file path in test")
}

/// Build a `ScanAggregates` bundle for a target root, mirroring the
/// container wiring in `crates/root_cli_main_entry.rs`. Used by tests that
/// previously relied on the `scan_aggregates: None` subprocess fallback,
/// which is now forbidden (current_exe self-invocation is an architecture
/// violation).
#[allow(dead_code)]
pub fn build_scan_aggregates(
    root: &str,
) -> dispatcher_lint_arwaky::surface_check_action::ScanAggregates {
    let fs_container = filesystem::root_filesystem_container::FilesystemContainer::new();
    let filesystem = fs_container.orchestrator();
    let fs_io = fs_container.io();
    let fs_workspace = fs_container.workspace();
    let fs_parser = fs_container.parser();
    let fs_tool_resolution = fs_container.tool_resolution();

    let fs_seam = std::sync::Arc::new(
        dispatcher_lint_arwaky::surface_check_action::FilesystemSeam {
            workspace: fs_workspace.clone(),
            parser: fs_parser.clone(),
            aggregate: filesystem.clone(),
        },
    );

    let config_container = config_system::root_config_system_container::ConfigContainer::new(
        filesystem.clone(),
        fs_io.clone(),
    );
    let config_orchestrator = config_container.orchestrator();

    let quality_container =
        quality_rules::root_quality_rules_container::CodeAnalysisContainer::from_orchestrator(
            &config_orchestrator,
            root,
        );
    let quality = quality_container.code_analysis_linter();

    let sync_config = config_orchestrator
        .execute(shared_config_system::ConfigRequest::load_sync(
            &FilePath::new(root.to_string()).unwrap_or_default(),
        ))
        .into_sync_config();

    let import_container =
        import_rules::root_import_rules_container::ImportContainer::new_with_config(
            sync_config.clone(),
            filesystem.clone(),
            fs_io.clone(),
            fs_workspace.clone(),
            fs_parser.clone(),
        );
    let import = import_container.orchestrator();

    let naming_container = naming_rules::root_naming_rules_container::NamingContainer::new(
        std::sync::Arc::new(sync_config.clone()),
        std::sync::Arc::new(shared_common::LayerMapVO::new(sync_config.layers.clone())),
    );
    let naming = naming_container.orchestrator();

    let orphan_container =
        orphan_rules::root_orphan_detector_container::OrphanContainer::from_orchestrator(
            &config_orchestrator,
            root,
            filesystem.clone(),
            fs_workspace.clone(),
        );
    let orphan = orphan_container.analyzer();

    let external_container =
        external_lint::root_external_lint_container::ExternalLintContainer::new(
            filesystem.clone(),
            fs_io.clone(),
            fs_tool_resolution.clone(),
        );
    let external = external_container.aggregate();

    let role_container =
        role_rules::root_role_rules_container::RoleContainer::new_with_config(sync_config);
    let role = role_container.orchestrator();

    let structure =
        structure_rules::root_structure_rules_container::RootStructureRulesContainer::orchestrator(
        );
    let doc = doc_rules::root_doc_rules_container::RootDocRulesContainer::orchestrator();

    dispatcher_lint_arwaky::surface_check_action::ScanAggregates {
        quality,
        role,
        import,
        naming,
        external,
        orphan,
        config: config_orchestrator,
        structure,
        doc,
        fs_seam,
    }
}
