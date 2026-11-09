// Regression test — full-root scan detects violations under a subfolder
// target, but narrows the *reported* output to that subfolder (PR #3, plan 4ae3d58a).
//
// PR 2 made `collect_scan*` scan the full workspace root
// (`scan_root = root.clone()`) and narrow the *reported* output via
// `retain_scope(violations, &scope.filter_prefix())`. This file guards that
// behavior: a subfolder target (`crates/member_a`) must scan the whole member
// dir (so cross-file import evidence is seen) but report only violations
// under `member_a`.
//
// Fixtures:
//   workspaces-bad/crates/member_a/capabilities_foo.rs — capabilities layer
//   workspaces-bad/crates/member_b/agent_bar.rs — agent layer, clean.
//
// Drive the in-process path (same as production), mirroring
// `integration_in_process_aggregates.rs`. `path` is the workspace root
// (`workspaces-bad`), scanned in full; `member` narrows the *reported* output
// to that subfolder.

use std::collections::HashSet;
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
    let fs_orchestrator = fs_container.orchestrator();
    let fs_workspace = fs_container.workspace();
    let fs_parser = fs_container.parser();
    let fs_tool_resolution = fs_container.tool_resolution();

    let fs_seam = Arc::new(FilesystemSeam {
        workspace: fs_orchestrator.clone(),
        parser: fs_orchestrator.clone(),
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
            false,
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
        config_file_reader: config_container.file_reader(),
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

/// Drive `collect_scan` through the in-process aggregate path with a subfolder
/// target (`member`) under `root`. `root` is the workspace root scanned in
/// full; `member` narrows the *reported* output to that subfolder.
fn in_process_subfolder_scan(
    agg: &ScanAggregates,
    root: &str,
    member: &str,
) -> Result<Vec<ViolationItem>, String> {
    let full = fixture_path(root);
    let opts = ScanOptions {
        path: Some(FilePath::new(full.to_string_lossy().to_string()).expect("valid fixture path")),
        multi_project_orchestrator: None,
        filter: None,
        member: Some(member.to_string()),
        filesystem: agg.fs_seam.clone(),
        scan_aggregates: Some(agg.clone()),
    };
    collect_scan(opts)
}

// ── Tests ─────────────────────────────────────────────────────────────────

/// A subfolder target reports violations under `member_a` and reports nothing
/// under `member_b`. Asserts the scope contract from PR 2: full-root scan
/// sees cross-member imports, but `retain_scope` narrows reported output to
/// the subfolder path prefix.
#[test]
fn subfolder_scan_reports_only_member_a() {
    let root = fixture_path("workspaces-bad");
    let agg = build_scan_aggregates(root.to_string_lossy().as_ref());
    let violations = in_process_subfolder_scan(&agg, "workspaces-bad", "crates/member_a")
        .expect("subfolder scan of member_a must succeed");

    // The subfolder target must report something (cross-member evidence seen
    // during the full-root scan is surfaced under member_a).
    assert!(
        !violations.is_empty(),
        "subfolder scan of member_a must report violations, got none"
    );

    // Every reported violation must be under member_a.
    for v in &violations {
        assert!(
            v.file.value.contains("crates/member_a"),
            "reported violation outside member_a: {:?}",
            v.file.value
        );
    }

    // No violation may carry a path under member_b — that would mean
    // retain_scope failed to narrow to the subfolder prefix.
    let member_b_paths: HashSet<_> = violations
        .iter()
        .filter(|v| v.file.value.contains("crates/member_b"))
        .map(|v| v.file.value.clone())
        .collect();
    assert!(
        member_b_paths.is_empty(),
        "expected no violations under crates/member_b, got {member_b_paths:?}"
    );
}

/// The same scan applied to `member_b` reports only member_b paths — the
/// scope contract holds symmetrically for both members.
#[test]
fn subfolder_scan_reports_only_member_b() {
    let root = fixture_path("workspaces-bad");
    let agg = build_scan_aggregates(root.to_string_lossy().as_ref());
    let violations = in_process_subfolder_scan(&agg, "workspaces-bad", "crates/member_b")
        .expect("subfolder scan of member_b must succeed");

    for v in &violations {
        assert!(
            v.file.value.contains("crates/member_b"),
            "reported violation outside member_b: {:?}",
            v.file.value
        );
    }

    let member_a_paths: HashSet<_> = violations
        .iter()
        .filter(|v| v.file.value.contains("crates/member_a"))
        .map(|v| v.file.value.clone())
        .collect();
    assert!(
        member_a_paths.is_empty(),
        "expected no violations under crates/member_a, got {member_a_paths:?}"
    );
}
