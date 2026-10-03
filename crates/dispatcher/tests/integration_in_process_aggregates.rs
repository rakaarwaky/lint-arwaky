// Integration tests — in-process aggregate dispatch (W10 path) for collect_scan.
//
// Every production entry point (CLI, MCP server, TUI) constructs a real
// `ScanAggregates` and passes `Some(…)` to `ScanOptions.scan_aggregates`,
// which routes all linters through their in-process aggregate entry points.
// The subprocess fallback (`scan_aggregates: None`) is covered by other
// dispatcher test files; this file drives the in-process path against the
// same fixture workspaces so the production path has real coverage.
//
// Fixtures:
//   workspaces-bad  — known violations, e.g. AES102 via `naming_violations`
//   workspaces-good — clean files, expected to produce 0 violations

use std::sync::Arc;

use dispatcher_lint_arwaky::surface_check_action::{
    FilesystemSeam, ScanAggregates, ScanOptions, collect_scan,
};
use shared_common::ViolationItem;
use shared_common::taxonomy_path_vo::FilePath;

// ── Container wiring — mirrors crates/root_cli_main_entry.rs ──────────────

fn build_scan_aggregates(root: &str) -> ScanAggregates {
    let fs_container = filesystem::root_filesystem_container::FilesystemContainer::new();
    let filesystem = fs_container.orchestrator();
    let fs_io = fs_container.io();
    let fs_workspace = fs_container.workspace();
    let fs_parser = fs_container.parser();
    let fs_tool_resolution = fs_container.tool_resolution();

    let fs_seam = Arc::new(FilesystemSeam {
        io: fs_io.clone(),
        workspace: fs_workspace.clone(),
        parser: fs_parser.clone(),
        aggregate: filesystem.clone(),
    });

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

    let import_container =
        import_rules::root_import_rules_container::ImportContainer::from_orchestrator(
            &config_orchestrator,
            root,
            filesystem.clone(),
            fs_io.clone(),
            fs_workspace.clone(),
            fs_parser.clone(),
        );
    let import = import_container.orchestrator();

    let sync_config = config_orchestrator
        .execute(shared_config_system::ConfigRequest::load_sync(
            &FilePath::new(root.to_string()).unwrap_or_default(),
        ))
        .into_sync_config();
    let layer_map = shared_common::LayerMapVO::new(sync_config.layers.clone());
    let naming_container = naming_rules::root_naming_rules_container::NamingContainer::new(
        Arc::new(sync_config.clone()),
        Arc::new(layer_map),
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

    ScanAggregates {
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

// ── Test helpers ──────────────────────────────────────────────────────────

/// Resolve a fixture path relative to the workspace root (parent of
/// `crates/dispatcher`).
fn fixture_path(relative: &str) -> std::path::PathBuf {
    std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(|p| p.parent())
        .map(|p| p.join(relative))
        .expect("CARGO_MANIFEST_DIR must be crates/dispatcher")
}

/// Drive `collect_scan` through the in-process aggregate path.
fn in_process_scan(agg: &ScanAggregates, target: &str) -> Result<Vec<ViolationItem>, String> {
    let full = fixture_path(target);
    let opts = ScanOptions {
        path: Some(FilePath::new(full.to_string_lossy().to_string()).expect("valid fixture path")),
        multi_project_orchestrator: None,
        filter: None,
        member: None,
        filesystem: agg.fs_seam.clone(),
        scan_aggregates: Some(agg.clone()),
    };
    collect_scan(opts)
}

/// Same as `in_process_scan` but with an unreadable path to exercise the
/// contract error path.
fn in_process_scan_missing(agg: &ScanAggregates) -> Result<Vec<ViolationItem>, String> {
    let missing = fixture_path("definitely/does/not/exist/anywhere.rs");
    let opts = ScanOptions {
        path: Some(
            FilePath::new(missing.to_string_lossy().to_string()).expect("valid string → FilePath"),
        ),
        multi_project_orchestrator: None,
        filter: None,
        member: None,
        filesystem: agg.fs_seam.clone(),
        scan_aggregates: Some(agg.clone()),
    };
    collect_scan(opts)
}

// ── Tests ─────────────────────────────────────────────────────────────────

/// Known bad fixture: `naming_violations` triggers AES102 through the
/// in-process path. The subprocess test in regression_scan_modes.rs asserts
/// the same code; this confirms both dispatch modes agree.
#[test]
fn in_process_bad_rust_fixture_yields_aes102() {
    let root = fixture_path("workspaces-bad/crates/naming_violations")
        .to_string_lossy()
        .to_string();
    let agg = build_scan_aggregates(&root);
    let violations = in_process_scan(
        &agg,
        "workspaces-bad/crates/naming_violations/src/capabilities_user_vo.rs",
    )
    .expect("in-process scan of valid fixture must succeed");
    let codes: Vec<&str> = violations.iter().map(|v| v.code.code()).collect();
    assert!(
        codes.iter().any(|c| c.starts_with("AES102")),
        "expected AES102 in in-process scan, got: {codes:?}"
    );
}

/// Subfolder scope: the whole `naming_violations` crate produces at least
/// 20 violations in-process (same threshold as the subprocess test).
#[test]
fn in_process_bad_rust_subfolder_has_many_violations() {
    let root = fixture_path("workspaces-bad/crates/naming_violations")
        .to_string_lossy()
        .to_string();
    let agg = build_scan_aggregates(&root);
    let violations = in_process_scan(&agg, "workspaces-bad/crates/naming_violations")
        .expect("in-process subfolder scan must succeed");
    assert!(
        violations.len() >= 20,
        "expected >= 20 violations in in-process subfolder scan, got {}",
        violations.len()
    );
}

/// Clean fixture: `workspaces-good/crates/calculator` must produce 0
/// violations through the in-process path (no false positives).
#[test]
fn in_process_clean_fixture_yields_zero_violations() {
    let root = fixture_path("workspaces-good/crates/calculator")
        .to_string_lossy()
        .to_string();
    let agg = build_scan_aggregates(&root);
    let violations = in_process_scan(&agg, "workspaces-good/crates/calculator")
        .expect("in-process scan of clean fixture must succeed");
    assert!(
        violations.is_empty(),
        "clean fixture must produce 0 in-process violations, got: {violations:?}",
    );
}

/// Error contract: an unreadable path must return `Err` with a user-facing
/// message — the same contract the subprocess fallback honours.
#[test]
fn in_process_missing_path_returns_error() {
    let root = fixture_path(".").to_string_lossy().to_string();
    let agg = build_scan_aggregates(&root);
    let result = in_process_scan_missing(&agg);
    assert!(
        result.is_err(),
        "in-process scan of nonexistent path must return Err, got {:?}",
        result.as_ref().map(|v| v.len()),
    );
    let err = result.unwrap_err();
    assert!(
        err.contains("does not exist"),
        "error message must mention the missing path, got: {err}"
    );
}
