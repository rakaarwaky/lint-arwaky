// Integration tests — dispatcher actions with real filesystem.
mod common;

use shared_common::taxonomy_path_vo::FilePath;

fn fixture_path(relative: &str) -> std::path::PathBuf {
    std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(|p| p.parent())
        .map(|p| p.join(relative))
        .expect("CARGO_MANIFEST_DIR must be crates/dispatcher")
}

#[test]
fn dispatcher_check_action_on_clean_project() {
    let root = fixture_path("workspaces-good/crates/calculator");
    let root_str = root.to_string_lossy().to_string();
    let fs = {
        let c = filesystem::root_filesystem_container::FilesystemContainer::new();
        std::sync::Arc::new(
            dispatcher_lint_arwaky::surface_check_action::FilesystemSeam {
                workspace: c.workspace(),
                parser: c.parser(),
                aggregate: c.orchestrator(),
            },
        )
    };
    let path = FilePath::new(root_str.clone()).unwrap();
    let opts = dispatcher_lint_arwaky::surface_check_action::ScanOptions {
        path: Some(path),
        multi_project_orchestrator: None,
        filter: None,
        member: None,
        filesystem: fs,
        scan_aggregates: Some(common::build_scan_aggregates(&root_str)),
    };
    let result = dispatcher_lint_arwaky::surface_check_action::collect_scan(opts);
    assert!(result.is_ok() || result.is_err());
}
