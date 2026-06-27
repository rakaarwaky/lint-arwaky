use config_system_lint_arwaky::infrastructure_workspace_detector_provider::WorkspaceDetector;
use shared::common::taxonomy_path_vo::FilePath;
use shared::config_system::contract_workspace_detector_port::IWorkspaceDetectorPort;
use shared::config_system::contract_workspace_detector_port::WorkspaceType;
use std::io::Write;

fn temp_dir() -> std::path::PathBuf {
    let p = std::env::temp_dir().join(format!(
        "ws_test_{}",
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));
    std::fs::create_dir_all(&p).unwrap();
    p
}

// ─── WorkspaceType enum ─────────────────────────────────────────────────────

#[test]
fn test_workspace_detection_concept() {
    assert_eq!(WorkspaceType::Rust.as_str(), "rust");
    assert_eq!(WorkspaceType::TypeScript.as_str(), "typescript");
    assert_eq!(WorkspaceType::Python.as_str(), "python");
    assert_eq!(WorkspaceType::Unknown.as_str(), "unknown");
}

// ─── detect ─────────────────────────────────────────────────────────────────

#[test]
fn detects_rust_from_cargo_toml() {
    let dir = temp_dir();
    std::fs::File::create(dir.join("Cargo.toml")).unwrap();
    let fp = FilePath::new(dir.to_string_lossy().to_string()).unwrap();
    let detector = WorkspaceDetector::new();
    assert_eq!(detector.detect(&fp), WorkspaceType::Rust);
}

#[test]
fn detects_typescript_from_package_json() {
    let dir = temp_dir();
    std::fs::File::create(dir.join("package.json")).unwrap();
    let fp = FilePath::new(dir.to_string_lossy().to_string()).unwrap();
    let detector = WorkspaceDetector::new();
    assert_eq!(detector.detect(&fp), WorkspaceType::TypeScript);
}

#[test]
fn detects_python_from_pyproject_toml() {
    let dir = temp_dir();
    std::fs::File::create(dir.join("pyproject.toml")).unwrap();
    let fp = FilePath::new(dir.to_string_lossy().to_string()).unwrap();
    let detector = WorkspaceDetector::new();
    assert_eq!(detector.detect(&fp), WorkspaceType::Python);
}

#[test]
fn detects_python_from_setup_py() {
    let dir = temp_dir();
    std::fs::File::create(dir.join("setup.py")).unwrap();
    let fp = FilePath::new(dir.to_string_lossy().to_string()).unwrap();
    let detector = WorkspaceDetector::new();
    assert_eq!(detector.detect(&fp), WorkspaceType::Python);
}

#[test]
fn detects_python_from_requirements_txt() {
    let dir = temp_dir();
    std::fs::File::create(dir.join("requirements.txt")).unwrap();
    let fp = FilePath::new(dir.to_string_lossy().to_string()).unwrap();
    let detector = WorkspaceDetector::new();
    assert_eq!(detector.detect(&fp), WorkspaceType::Python);
}

#[test]
fn detects_rust_from_crates_parent() {
    let dir = temp_dir();
    let pkg_dir = dir.join("crates").join("some_crate");
    std::fs::create_dir_all(&pkg_dir).unwrap();
    let fp = FilePath::new(pkg_dir.to_string_lossy().to_string()).unwrap();
    let detector = WorkspaceDetector::new();
    assert_eq!(detector.detect(&fp), WorkspaceType::Rust);
}

#[test]
fn detects_typescript_from_packages_parent() {
    let dir = temp_dir();
    let pkg_dir = dir.join("packages").join("some_package");
    std::fs::create_dir_all(&pkg_dir).unwrap();
    let fp = FilePath::new(pkg_dir.to_string_lossy().to_string()).unwrap();
    let detector = WorkspaceDetector::new();
    assert_eq!(detector.detect(&fp), WorkspaceType::TypeScript);
}

#[test]
fn detects_python_from_modules_parent() {
    let dir = temp_dir();
    let mod_dir = dir.join("modules").join("some_module");
    std::fs::create_dir_all(&mod_dir).unwrap();
    let fp = FilePath::new(mod_dir.to_string_lossy().to_string()).unwrap();
    let detector = WorkspaceDetector::new();
    assert_eq!(detector.detect(&fp), WorkspaceType::Python);
}

#[test]
fn unknown_directory_returns_unknown() {
    let dir = temp_dir();
    // Create an empty directory with no workspace markers
    std::fs::create_dir_all(dir.join("empty")).unwrap();
    let fp = FilePath::new(dir.to_string_lossy().to_string()).unwrap();
    let detector = WorkspaceDetector::new();
    assert_eq!(detector.detect(&fp), WorkspaceType::Unknown);
}

#[test]
fn parent_walk_detects_cargo_toml_upward() {
    let dir = temp_dir();
    let subdir = dir.join("deeply/nested/path");
    std::fs::create_dir_all(&subdir).unwrap();
    // Place Cargo.toml in parent (not in subdir)
    std::fs::File::create(dir.join("Cargo.toml")).unwrap();
    let fp = FilePath::new(subdir.to_string_lossy().to_string()).unwrap();
    let detector = WorkspaceDetector::new();
    assert_eq!(detector.detect(&fp), WorkspaceType::Rust);
}

// ─── is_workspace ───────────────────────────────────────────────────────────

#[test]
fn is_workspace_true_when_has_crates_dir() {
    let dir = temp_dir();
    std::fs::create_dir_all(dir.join("crates")).unwrap();
    let fp = FilePath::new(dir.to_string_lossy().to_string()).unwrap();
    let detector = WorkspaceDetector::new();
    assert!(detector.is_workspace(&fp));
}

#[test]
fn is_workspace_true_when_has_packages_dir() {
    let dir = temp_dir();
    std::fs::create_dir_all(dir.join("packages")).unwrap();
    let fp = FilePath::new(dir.to_string_lossy().to_string()).unwrap();
    let detector = WorkspaceDetector::new();
    assert!(detector.is_workspace(&fp));
}

#[test]
fn is_workspace_true_when_has_modules_dir() {
    let dir = temp_dir();
    std::fs::create_dir_all(dir.join("modules")).unwrap();
    let fp = FilePath::new(dir.to_string_lossy().to_string()).unwrap();
    let detector = WorkspaceDetector::new();
    assert!(detector.is_workspace(&fp));
}

#[test]
fn is_workspace_false_for_empty_dir() {
    let dir = temp_dir();
    let fp = FilePath::new(dir.to_string_lossy().to_string()).unwrap();
    let detector = WorkspaceDetector::new();
    assert!(!detector.is_workspace(&fp));
}

#[test]
fn is_workspace_false_for_nonexistent_path() {
    let fp = FilePath::new("/nonexistent/path/xyz".to_string()).unwrap();
    let detector = WorkspaceDetector::new();
    assert!(!detector.is_workspace(&fp));
}
