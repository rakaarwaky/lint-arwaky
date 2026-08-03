// PURPOSE: Test path normalization utility functions from shared::common::utility_path_normalization

use shared_lint_arwaky::common::taxonomy_path_vo::FilePath;
use shared_lint_arwaky::common::utility_path_normalization::{
    normalize_path, resolve_capabilities_path,
};

// ── normalize_path ──────────────────────────────────────────

#[test]
fn normalize_path_returns_path_unchanged() {
    let fp = FilePath::new("src/main.rs").unwrap();
    let result = normalize_path(fp.clone());
    assert_eq!(result.value(), "src/main.rs");
}

#[test]
fn normalize_path_preserves_absolute_path() {
    let fp = FilePath::new("/home/user/project/src/main.rs").unwrap();
    let result = normalize_path(fp);
    assert_eq!(result.value(), "/home/user/project/src/main.rs");
}

#[test]
fn normalize_path_preserves_relative_path() {
    let fp = FilePath::new("./src/main.rs").unwrap();
    let result = normalize_path(fp);
    assert_eq!(result.value(), "./src/main.rs");
}

#[test]
fn normalize_path_preserves_trailing_component() {
    let fp = FilePath::new("src/utils/helpers.rs").unwrap();
    let result = normalize_path(fp);
    assert_eq!(result.value(), "src/utils/helpers.rs");
}

// ── resolve_capabilities_path ───────────────────────────────

#[test]
fn resolve_capabilities_path_returns_path_unchanged() {
    let path = FilePath::new("capabilities/scan.rs").unwrap();
    let result = resolve_capabilities_path(path.clone(), None);
    assert_eq!(result.value(), "capabilities/scan.rs");
}

#[test]
fn resolve_capabilities_path_ignores_context() {
    let path = FilePath::new("capabilities/fix.rs").unwrap();
    let context = FilePath::new("/some/context/file.rs").unwrap();
    let result = resolve_capabilities_path(path, Some(context));
    assert_eq!(result.value(), "capabilities/fix.rs");
}

#[test]
fn resolve_capabilities_path_none_context() {
    let path = FilePath::new("mod.rs").unwrap();
    let result = resolve_capabilities_path(path, None);
    assert_eq!(result.value(), "mod.rs");
}

#[test]
fn resolve_capabilities_path_preserves_complex_path() {
    let path = FilePath::new("src/layers/surface/cli/scan.rs").unwrap();
    let result = resolve_capabilities_path(path, None);
    assert_eq!(result.value(), "src/layers/surface/cli/scan.rs");
}
