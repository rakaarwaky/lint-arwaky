// Unit tests — DiffChecker: lintable filter, get_diff, run_git_diff_check.

use git_hooks_lint_arwaky::capabilities_diff_checker::is_lintable_file;
use shared::common::FilePath;

// ─── Lintable filter (FR-001) ─────────────────────────────

#[test]
fn lintable_rs_is_lintable() {
    assert!(is_lintable_file(&FilePath::new("src/main.rs".to_string()).unwrap()));
}

#[test]
fn lintable_py_is_lintable() {
    assert!(is_lintable_file(&FilePath::new("app.py".to_string()).unwrap()));
}

#[test]
fn lintable_ts_is_lintable() {
    assert!(is_lintable_file(&FilePath::new("index.ts".to_string()).unwrap()));
}

#[test]
fn lintable_js_is_lintable() {
    assert!(is_lintable_file(&FilePath::new("script.js".to_string()).unwrap()));
}

#[test]
fn lintable_jsx_is_lintable() {
    assert!(is_lintable_file(&FilePath::new("App.jsx".to_string()).unwrap()));
}

#[test]
fn lintable_tsx_is_lintable() {
    assert!(is_lintable_file(&FilePath::new("App.tsx".to_string()).unwrap()));
}

#[test]
fn non_lintable_md_not_lintable() {
    assert!(!is_lintable_file(&FilePath::new("README.md".to_string()).unwrap()));
}

#[test]
fn non_lintable_toml_not_lintable() {
    assert!(!is_lintable_file(&FilePath::new("Cargo.toml".to_string()).unwrap()));
}

#[test]
fn non_lintable_json_not_lintable() {
    assert!(!is_lintable_file(&FilePath::new("package.json".to_string()).unwrap()));
}

#[test]
fn non_lintable_yaml_not_lintable() {
    assert!(!is_lintable_file(&FilePath::new("config.yaml".to_string()).unwrap()));
}

#[test]
fn non_lintable_lock_not_lintable() {
    assert!(!is_lintable_file(&FilePath::new("Cargo.lock".to_string()).unwrap()));
}

#[test]
fn non_lintable_png_not_lintable() {
    assert!(!is_lintable_file(&FilePath::new("image.png".to_string()).unwrap()));
}

#[test]
fn non_lintable_empty_ext_not_lintable() {
    assert!(!is_lintable_file(&FilePath::new("Makefile".to_string()).unwrap()));
}

#[test]
fn lintable_nested_path_rs() {
    assert!(is_lintable_file(&FilePath::new("crates/shared/src/lib.rs".to_string()).unwrap()));
}

#[test]
fn lintable_nested_path_tsx() {
    assert!(is_lintable_file(
        &FilePath::new("src/components/App.tsx".to_string()).unwrap()
    ));
}
