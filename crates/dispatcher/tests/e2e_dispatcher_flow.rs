// E2E tests — dispatcher scan flow.
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
fn e2e_check_action_full_flow() {
    let root = fixture_path("workspaces-good/crates/calculator");
    let path = FilePath::new(root.to_string_lossy().to_string()).unwrap();
    let c = filesystem::root_filesystem_container::FilesystemContainer::new();
    let opts = dispatcher_lint_arwaky::surface_check_action::ScanOptions {
        path: Some(path),
        multi_project_orchestrator: None,
        filter: None,
        member: None,
        filesystem: std::sync::Arc::new(
            dispatcher_lint_arwaky::surface_check_action::FilesystemSeam {
                workspace: c.workspace(),
                parser: c.parser(),
                aggregate: c.orchestrator(),
            },
        ),
        scan_aggregates: Some(common::build_scan_aggregates(
            root.to_string_lossy().as_ref(),
        )),
    };
    let result = dispatcher_lint_arwaky::surface_check_action::collect_scan(opts);
    assert!(result.is_ok());
}
