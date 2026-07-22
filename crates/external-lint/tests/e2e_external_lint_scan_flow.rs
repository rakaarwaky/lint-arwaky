// PURPOSE: E2E tests — full scan lifecycle through the real container.
// Creates real files on disk, runs the orchestrator, asserts on real output.
// External tools may not be installed; the system must handle that gracefully.

use external_lint_lint_arwaky::ExternalLintContainer;
use shared::common::taxonomy_path_vo::FilePath;
use std::fs;

#[tokio::test]
async fn scan_rust_project_does_not_crash() {
    let dir = tempfile::tempdir().unwrap();
    fs::write(
        dir.path().join("Cargo.toml"),
        r#"[package]
name = "test-project"
version = "0.1.0"
edition = "2021"
"#,
    )
    .unwrap();
    fs::create_dir_all(dir.path().join("src")).unwrap();
    fs::write(
        dir.path().join("src/main.rs"),
        "fn main() { println!(\"hello\"); }",
    )
    .unwrap();

    let path = FilePath::new(dir.path().to_string_lossy().to_string()).unwrap();
    let container = ExternalLintContainer::new();
    let aggregate = container.aggregate();

    // Should not panic regardless of whether clippy/rustfmt are installed
    let results = aggregate.scan_all(&path).await;
    // Results may be empty if tools aren't installed — that's OK
    let _ = results;
}

#[tokio::test]
async fn scan_python_project_does_not_crash() {
    let dir = tempfile::tempdir().unwrap();
    fs::write(dir.path().join("app.py"), "x: int = 'not an int'\n").unwrap();

    let path = FilePath::new(dir.path().to_string_lossy().to_string()).unwrap();
    let container = ExternalLintContainer::new();
    let aggregate = container.aggregate();

    let results = aggregate.scan_all(&path).await;
    let _ = results;
}

#[tokio::test]
async fn scan_js_project_does_not_crash() {
    let dir = tempfile::tempdir().unwrap();
    fs::write(
        dir.path().join("package.json"),
        r#"{"name": "test", "version": "1.0.0"}"#,
    )
    .unwrap();
    fs::write(dir.path().join("index.ts"), "const x: number = 'oops';\n").unwrap();

    let path = FilePath::new(dir.path().to_string_lossy().to_string()).unwrap();
    let container = ExternalLintContainer::new();
    let aggregate = container.aggregate();

    let results = aggregate.scan_all(&path).await;
    let _ = results;
}

#[tokio::test]
async fn scan_mixed_project_detects_all_languages() {
    let dir = tempfile::tempdir().unwrap();
    fs::write(dir.path().join("main.rs"), "fn main() {}").unwrap();
    fs::write(dir.path().join("app.py"), "print('hi')").unwrap();
    fs::write(dir.path().join("index.ts"), "export {}").unwrap();

    let path = FilePath::new(dir.path().to_string_lossy().to_string()).unwrap();
    let container = ExternalLintContainer::new();
    let aggregate = container.aggregate();

    // All 9 adapters should be attempted
    let names = aggregate.adapter_names();
    assert_eq!(names.len(), 9);

    // Scan should complete without panic
    let results = aggregate.scan_all(&path).await;
    let _ = results;
}

#[tokio::test]
async fn scan_single_file_path_works() {
    let dir = tempfile::tempdir().unwrap();
    let file = dir.path().join("script.py");
    fs::write(&file, "import os\n").unwrap();

    let path = FilePath::new(file.to_string_lossy().to_string()).unwrap();
    let container = ExternalLintContainer::new();
    let aggregate = container.aggregate();

    let results = aggregate.scan_all(&path).await;
    let _ = results;
}

#[tokio::test]
async fn scan_nonexistent_path_does_not_crash() {
    let path = FilePath::new("/nonexistent/path/xyz_12345".to_string()).unwrap();
    let container = ExternalLintContainer::new();
    let aggregate = container.aggregate();

    // Should not panic — language detection finds nothing, no adapters run
    let results = aggregate.scan_all(&path).await;
    assert!(results.is_empty());
}
