// PURPOSE: Unit tests for ExternalLintUtilityAdapter — path, language, JS, Cargo,
// and command protocol implementations.

use external_lint_lint_arwaky::capabilities_external_lint_adapter::ExternalLintUtilityAdapter;
use shared::common::taxonomy_common_vo::PatternList;
use shared::common::taxonomy_path_vo::{DirectoryPath, FilePath};
use shared::external_lint::contract_external_lint_utility_protocol::{
    IExternalLintCargoProtocol, IExternalLintCommandProtocol, IExternalLintJsProtocol,
    IExternalLintLanguageProtocol, IExternalLintPathProtocol,
};
use std::fs;

fn sut() -> ExternalLintUtilityAdapter {
    ExternalLintUtilityAdapter::new()
}

// ─── IExternalLintPathProtocol ────────────────────────────

#[test]
fn canonicalize_path_returns_valid_path() {
    let adapter = sut();
    let result = adapter.canonicalize_path("/tmp");
    assert!(!result.value().is_empty());
}

#[test]
fn canonicalize_path_nonexistent_returns_original() {
    let adapter = sut();
    let result = adapter.canonicalize_path("/nonexistent/path/xyz");
    assert_eq!(result.value(), "/nonexistent/path/xyz");
}

#[test]
fn default_working_dir_returns_dot() {
    let adapter = sut();
    let path = FilePath::new("/some/path".to_string()).unwrap();
    let result = adapter.default_working_dir(&path);
    assert_eq!(result.value(), ".");
}

// ─── IExternalLintLanguageProtocol ────────────────────────

#[test]
fn has_python_files_detects_py_extension() {
    let dir = tempfile::tempdir().unwrap();
    fs::write(dir.path().join("script.py"), "print('hi')").unwrap();

    let adapter = sut();
    let path = FilePath::new(dir.path().to_string_lossy().to_string()).unwrap();
    assert!(adapter.has_python_files(&path).value());
}

#[test]
fn has_python_files_false_for_rs_only() {
    let dir = tempfile::tempdir().unwrap();
    fs::write(dir.path().join("main.rs"), "fn main() {}").unwrap();

    let adapter = sut();
    let path = FilePath::new(dir.path().to_string_lossy().to_string()).unwrap();
    assert!(!adapter.has_python_files(&path).value());
}

#[test]
fn has_python_files_single_py_file() {
    let dir = tempfile::tempdir().unwrap();
    let file = dir.path().join("app.py");
    fs::write(&file, "x = 1").unwrap();

    let adapter = sut();
    let path = FilePath::new(file.to_string_lossy().to_string()).unwrap();
    assert!(adapter.has_python_files(&path).value());
}

#[test]
fn is_in_path_finds_echo() {
    let adapter = sut();
    // 'echo' should be in PATH on any Unix system
    assert!(adapter.is_in_path("echo").value());
}

#[test]
fn is_in_path_false_for_nonexistent() {
    let adapter = sut();
    assert!(!adapter.is_in_path("nonexistent_tool_xyz_99999").value());
}

// ─── IExternalLintJsProtocol ──────────────────────────────

#[test]
fn resolve_js_cmd_uses_executable_name() {
    let adapter = sut();
    let wd = FilePath::new("/tmp".to_string()).unwrap();
    let args = PatternList::new(vec!["--version".to_string()]);
    let result = adapter.resolve_js_cmd("eslint", args, &wd);
    // Should contain eslint somewhere in the command
    assert!(result.values().iter().any(|s| s.contains("eslint")));
}

#[test]
fn resolve_js_working_dir_returns_path() {
    let adapter = sut();
    let path = FilePath::new("/tmp".to_string()).unwrap();
    let result = adapter.resolve_js_working_dir(&path);
    assert!(!result.value().is_empty());
}

// ─── IExternalLintCargoProtocol ───────────────────────────

#[test]
fn resolve_cargo_working_dir_with_cargo_toml() {
    let dir = tempfile::tempdir().unwrap();
    fs::write(dir.path().join("Cargo.toml"), "[package]\nname=\"test\"").unwrap();

    let adapter = sut();
    let path = FilePath::new(dir.path().to_string_lossy().to_string()).unwrap();
    let result = adapter.resolve_cargo_working_dir(&path);
    assert!(result.value().contains("Cargo.toml") || !result.value().is_empty());
}

#[test]
fn resolve_cargo_working_dir_empty_path_returns_input() {
    let adapter = sut();
    let path = FilePath::new("/some/path".to_string()).unwrap();
    // When no Cargo.toml found, returns a fallback
    let result = adapter.resolve_cargo_working_dir(&path);
    assert!(!result.value().is_empty());
}

#[test]
fn resolve_cargo_lock_working_dir_with_cargo_lock() {
    let dir = tempfile::tempdir().unwrap();
    fs::write(dir.path().join("Cargo.lock"), "# lock").unwrap();

    let adapter = sut();
    let path = FilePath::new(dir.path().to_string_lossy().to_string()).unwrap();
    let result = adapter.resolve_cargo_lock_working_dir(&path);
    assert!(!result.value().is_empty());
}

// ─── IExternalLintCommandProtocol ─────────────────────────

#[tokio::test]
async fn noop_apply_fix_returns_false() {
    let adapter = sut();
    let result = adapter.noop_apply_fix().await.unwrap();
    assert!(!result.value());
}

// ─── Default / Constructor ────────────────────────────────

#[test]
fn default_creates_instance() {
    let adapter = ExternalLintUtilityAdapter::default();
    let _ = adapter;
}

#[test]
fn new_creates_instance() {
    let adapter = ExternalLintUtilityAdapter::new();
    let _ = adapter;
}
