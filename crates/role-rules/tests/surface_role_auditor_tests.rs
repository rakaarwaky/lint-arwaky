use role_rules_lint_arwaky::capabilities_surface_role_auditor::{is_in_surfaces, is_init};
use shared::common::taxonomy_path_vo::FilePath;

#[test]
fn test_is_in_surfaces() {
    let f = FilePath::new("src/surfaces/surface_handler.py").unwrap_or_else(|_| FilePath::new(".").unwrap_or_default());
    assert!(is_in_surfaces(&f));

    let f = FilePath::new("src/capabilities/capabilities_not_checker.py").unwrap_or_else(|_| FilePath::new(".").unwrap_or_default());
    assert!(!is_in_surfaces(&f));

    let f = FilePath::new("src/cli-commands/surface_check_command.rs").unwrap_or_else(|_| FilePath::new(".").unwrap_or_default());
    assert!(is_in_surfaces(&f));
}

#[test]
fn test_is_init() {
    let f = FilePath::new("src/surfaces/__init__.py").unwrap_or_else(|_| FilePath::new(".").unwrap_or_default());
    assert!(is_init(&f));

    let f = FilePath::new("src/surfaces/handler.py").unwrap_or_else(|_| FilePath::new(".").unwrap_or_default());
    assert!(!is_init(&f));
}
