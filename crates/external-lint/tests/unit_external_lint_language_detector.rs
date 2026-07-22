// PURPOSE: Unit tests for ExternalLintLanguageDetectorAdapter — directory scanning
// to detect present programming languages.

use external_lint_lint_arwaky::capabilities_language_detector_adapter::ExternalLintLanguageDetectorAdapter;
use shared::common::taxonomy_path_vo::FilePath;
use shared::external_lint::contract_external_lint_language_detector_protocol::IExternalLintLanguageDetectorProtocol;
use std::fs;
use std::path::Path;

fn sut() -> ExternalLintLanguageDetectorAdapter {
    ExternalLintLanguageDetectorAdapter::new()
}

fn create_temp_dir_with_files(files: &[&str]) -> tempfile::TempDir {
    let dir = tempfile::tempdir().unwrap();
    for file in files {
        let path = dir.path().join(file);
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).unwrap();
        }
        fs::write(&path, "// dummy content").unwrap();
    }
    dir
}

// ─── Happy Path ───────────────────────────────────────────

#[tokio::test]
async fn detects_rust_files() {
    let dir = create_temp_dir_with_files(&["src/main.rs", "src/lib.rs"]);
    let path = FilePath::new(dir.path().to_string_lossy().to_string()).unwrap();

    let result = sut().detect_languages(&path).await;
    assert!(result.has_rs);
    assert!(!result.has_py);
    assert!(!result.has_js);
}

#[tokio::test]
async fn detects_python_files() {
    let dir = create_temp_dir_with_files(&["app.py", "utils/helper.py"]);
    let path = FilePath::new(dir.path().to_string_lossy().to_string()).unwrap();

    let result = sut().detect_languages(&path).await;
    assert!(!result.has_rs);
    assert!(result.has_py);
    assert!(!result.has_js);
}

#[tokio::test]
async fn detects_js_ts_files() {
    let dir = create_temp_dir_with_files(&["index.ts", "src/app.jsx", "lib/util.tsx"]);
    let path = FilePath::new(dir.path().to_string_lossy().to_string()).unwrap();

    let result = sut().detect_languages(&path).await;
    assert!(!result.has_rs);
    assert!(!result.has_py);
    assert!(result.has_js);
}

#[tokio::test]
async fn detects_all_three_languages() {
    let dir = create_temp_dir_with_files(&["main.rs", "app.py", "index.ts"]);
    let path = FilePath::new(dir.path().to_string_lossy().to_string()).unwrap();

    let result = sut().detect_languages(&path).await;
    assert!(result.has_rs);
    assert!(result.has_py);
    assert!(result.has_js);
}

// ─── Edge Cases ───────────────────────────────────────────

#[tokio::test]
async fn empty_directory_detects_nothing() {
    let dir = tempfile::tempdir().unwrap();
    let path = FilePath::new(dir.path().to_string_lossy().to_string()).unwrap();

    let result = sut().detect_languages(&path).await;
    assert!(!result.has_rs);
    assert!(!result.has_py);
    assert!(!result.has_js);
}

#[tokio::test]
async fn single_file_path_detects_language() {
    let dir = create_temp_dir_with_files(&["script.py"]);
    let file_path = dir.path().join("script.py");
    let path = FilePath::new(file_path.to_string_lossy().to_string()).unwrap();

    let result = sut().detect_languages(&path).await;
    assert!(!result.has_rs);
    assert!(result.has_py);
    assert!(!result.has_js);
}

#[tokio::test]
async fn skips_node_modules_directory() {
    let dir = create_temp_dir_with_files(&["src/main.rs", "node_modules/pkg/index.js"]);
    let path = FilePath::new(dir.path().to_string_lossy().to_string()).unwrap();

    let result = sut().detect_languages(&path).await;
    assert!(result.has_rs);
    assert!(!result.has_js); // node_modules skipped
}

#[tokio::test]
async fn skips_target_directory() {
    let dir = create_temp_dir_with_files(&["src/lib.py", "target/debug/build.rs"]);
    let path = FilePath::new(dir.path().to_string_lossy().to_string()).unwrap();

    let result = sut().detect_languages(&path).await;
    assert!(!result.has_rs); // target/ skipped
    assert!(result.has_py);
}

#[tokio::test]
async fn skips_git_directory() {
    let dir = create_temp_dir_with_files(&["app.ts", ".git/hooks/pre-commit.rs"]);
    let path = FilePath::new(dir.path().to_string_lossy().to_string()).unwrap();

    let result = sut().detect_languages(&path).await;
    assert!(!result.has_rs); // .git/ skipped
    assert!(result.has_js);
}

#[tokio::test]
async fn non_source_files_are_ignored() {
    let dir = create_temp_dir_with_files(&["README.md", "Cargo.lock", "data.json"]);
    let path = FilePath::new(dir.path().to_string_lossy().to_string()).unwrap();

    let result = sut().detect_languages(&path).await;
    assert!(!result.has_rs);
    assert!(!result.has_py);
    assert!(!result.has_js);
}

// ─── Default Constructor ──────────────────────────────────

#[test]
fn default_constructor_creates_instance() {
    let detector = ExternalLintLanguageDetectorAdapter::default();
    // Just verify it can be constructed via Default
    let _ = detector;
}
