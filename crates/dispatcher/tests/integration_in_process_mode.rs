// Integration tests — in-process aggregate dispatch mode (issue #573).
//
// Production always passes `ScanOptions.scan_aggregates: Some(..)` (CLI, MCP,
// TUI), yet the dispatcher suite only exercised the subprocess fallback. These
// tests build REAL feature aggregates exactly like the root container does and
// drive `collect_scan`/`collect_ci` through the in-process path, then assert
// parity with the subprocess mode on the same fixture.

use dispatcher_lint_arwaky::orchestrator_check_pipeline::{
    FilesystemSeam, ScanAggregates, ScanOptions, collect_scan,
};
use dispatcher_lint_arwaky::orchestrator_ci_pipeline::{CiScanDeps, collect_ci};
use shared_common::Threshold;
use shared_common::taxonomy_path_vo::FilePath;
use std::process::Command;
use std::sync::Arc;
use structure_rules::root_structure_rules_container::RootStructureRulesContainer;

/// Resolve workspace root from CARGO_MANIFEST_DIR (crates/<name>/ → project root).
fn workspace_root() -> std::path::PathBuf {
    std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(|p| p.parent())
        .map(|p| p.to_path_buf())
        .unwrap_or_else(|| std::path::PathBuf::from("."))
}

/// Build the real aggregate set, mirroring `root_entry_container::CommonDeps`.
fn real_aggregates() -> ScanAggregates {
    let root = workspace_root();
    let root_str = root.to_string_lossy().to_string();

    let fs_container = filesystem::root_filesystem_container::FilesystemContainer::new();
    let filesystem = fs_container.orchestrator();
    let filesystem_io = fs_container.io();
    let filesystem_workspace = fs_container.workspace();
    let filesystem_parser = fs_container.parser();
    let filesystem_tool_resolution = fs_container.tool_resolution();
    let fs_seam = Arc::new(FilesystemSeam {
        io: filesystem_io.clone(),
        workspace: filesystem_workspace.clone(),
        parser: filesystem_parser.clone(),
        aggregate: filesystem.clone(),
    });

    let config_container = config_system::root_config_system_container::ConfigContainer::new(
        filesystem.clone(),
        filesystem_io.clone(),
    );
    let config_orchestrator = config_container.orchestrator();

    let quality =
        quality_rules::root_quality_rules_container::CodeAnalysisContainer::from_orchestrator(
            &config_orchestrator,
            &root_str,
        )
        .code_analysis_linter();

    let import = import_rules::root_import_rules_container::ImportContainer::from_orchestrator(
        &config_orchestrator,
        &root_str,
        filesystem.clone(),
        filesystem_io.clone(),
        filesystem_workspace.clone(),
        filesystem_parser.clone(),
    )
    .orchestrator();

    let sync_config = || {
        config_orchestrator
            .execute(shared_config_system::ConfigRequest::load_sync(
                &FilePath::new(root_str.clone()).unwrap_or_default(),
            ))
            .into_sync_config()
    };
    let naming = naming_rules::root_naming_rules_container::NamingContainer::new(
        Arc::new(sync_config()),
        Arc::new(shared_common::LayerMapVO::new(sync_config().layers.clone())),
    )
    .orchestrator();

    let orphan =
        orphan_rules::root_orphan_detector_container::OrphanContainer::from_orchestrator(
            &config_orchestrator,
            &root_str,
            filesystem.clone(),
            filesystem_workspace.clone(),
        )
        .analyzer();

    let external_lint =
        external_lint::root_external_lint_container::ExternalLintContainer::new(
            filesystem.clone(),
            filesystem_io.clone(),
            filesystem_tool_resolution.clone(),
        )
        .aggregate();

    let role = role_rules::root_role_rules_container::RoleContainer::new_with_config(sync_config())
        .orchestrator();

    let doc = doc_rules::root_doc_rules_container::RootDocRulesContainer::orchestrator();

    let structure = RootStructureRulesContainer::orchestrator();

    ScanAggregates {
        quality,
        role,
        import,
        naming,
        external: external_lint,
        orphan,
        config: config_orchestrator,
        structure,
        doc,
        fs_seam,
    }
}

/// In-process scan over a fixture, exactly like the production entry points.
fn in_process_scan(path: &str) -> Vec<shared_common::ViolationItem> {
    let full_path = workspace_root().join(path);
    let aggregates = real_aggregates();
    let seam = aggregates.fs_seam.clone();
    let opts = ScanOptions {
        path: Some(FilePath::new(full_path.to_string_lossy().to_string()).unwrap()),
        multi_project_orchestrator: None,
        filter: None,
        member: None,
        filesystem: seam,
        scan_aggregates: Some(aggregates),
    };
    collect_scan(opts).unwrap_or_default()
}

/// Resolve the release CLI binary the subprocess mode spawns.
fn release_cli() -> std::path::PathBuf {
    std::env::current_exe()
        .ok()
        .and_then(|p| {
            p.parent()
                .and_then(|p| p.parent())
                .and_then(|p| p.parent())
                .map(|p| p.join("release/lint-arwaky-cli"))
        })
        .unwrap_or_else(|| std::path::PathBuf::from("target/release/lint-arwaky-cli"))
}

/// Subprocess-mode scan codes via the release binary (`--format json`).
fn subprocess_codes(path: &str) -> Vec<String> {
    let full_path = workspace_root().join(path);
    let output = Command::new(release_cli())
        .args([
            "scan",
            full_path.to_str().unwrap_or(path),
            "--format",
            "json",
        ])
        .output()
        .expect("release lint-arwaky-cli binary; run `cargo build --release` first");
    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let report: serde_json::Value = serde_json::from_str(&stdout).unwrap_or_default();
    report
        .get("results")
        .and_then(|r| r.as_array())
        .map(|results| {
            results
                .iter()
                .filter_map(|r| r.get("code").and_then(|c| c.as_str()).map(String::from))
                .collect()
        })
        .unwrap_or_default()
}

fn sorted_codes(violations: &[shared_common::ViolationItem]) -> Vec<String> {
    let mut codes: Vec<String> = violations.iter().map(|v| v.code.code().to_string()).collect();
    codes.sort();
    codes
}

#[test]
fn in_process_scan_reports_fixture_violations() {
    let violations = in_process_scan("workspaces-bad/crates");
    assert!(
        !violations.is_empty(),
        "in-process mode must surface the fixture's violations"
    );
    assert!(
        violations.iter().all(|v| v.code.code().starts_with("AES")),
        "every finding must carry an AES code"
    );
}

#[test]
fn in_process_single_file_scan_reports_violations() {
    let files = std::fs::read_dir(workspace_root().join("workspaces-bad/crates"))
        .expect("workspaces-bad/crates fixture");
    let some_file = files
        .filter_map(|e| e.ok())
        .find(|e| e.path().extension().is_some_and(|ext| ext == "rs"));
    let Some(entry) = some_file else {
        panic!("fixture has no .rs file to scan");
    };
    let rel = entry
        .path()
        .strip_prefix(workspace_root())
        .unwrap_or(&entry.path())
        .to_string_lossy()
        .to_string();
    let violations = in_process_scan(&rel);
    assert!(!violations.is_empty(), "single-file scan of {rel} must report");
}

#[test]
fn in_process_mode_matches_subprocess_mode() {
    let fixture = "workspaces-bad/crates";
    let in_process = in_process_scan(fixture);
    let in_process_codes = sorted_codes(&in_process);
    let mut subprocess = subprocess_codes(fixture);
    subprocess.sort();

    assert_eq!(
        in_process_codes, subprocess,
        "in-process and subprocess scan modes drifted on {fixture}"
    );
}

#[test]
fn collect_ci_in_process_mode_evaluates_the_fixture() {
    let aggregates = real_aggregates();
    let deps = CiScanDeps {
        code_analysis_linter: aggregates.quality,
        import_orchestrator: aggregates.import,
        naming_orchestrator: aggregates.naming,
        config_orchestrator: aggregates.config,
        orphan_orchestrator: aggregates.orphan,
        filesystem: aggregates.fs_seam.aggregate.clone(),
        filesystem_io: aggregates.fs_seam.io.clone(),
    };
    let path = FilePath::new(workspace_root().join("workspaces-bad").to_string_lossy().to_string())
        .unwrap();
    let threshold = Threshold::try_new(80).expect("valid threshold");
    let report = collect_ci(deps, Some(path), threshold).expect("ci evaluation");
    assert!(report.score <= 100.0);
    assert!(
        report.total_violations > 0,
        "the bad workspace must not score a clean CI report"
    );
}
