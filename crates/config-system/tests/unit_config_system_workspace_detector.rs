// Unit tests for WorkspaceDetector — language detection and workspace discovery.
use config_system_lint_arwaky::capabilities_workspace_detector::WorkspaceDetector;
use shared::common::FilePath;
use shared::config_system::{IWorkspaceDetectorProtocol, WorkspaceType};
use std::fs;
use tempfile::TempDir;

fn make_detector() -> WorkspaceDetector {
    WorkspaceDetector::new()
}
fn create_file(dir: &std::path::Path, name: &str) {
    fs::write(dir.join(name), "").unwrap();
}

#[test]
fn detect_rust_workspace_by_cargo_toml() {
    let tmp = TempDir::new().unwrap();
    create_file(tmp.path(), "Cargo.toml");
    let fp = FilePath::new(tmp.path().to_string_lossy().to_string()).unwrap();
    assert_eq!(make_detector().detect(&fp), WorkspaceType::Rust);
}

#[test]
fn detect_typescript_workspace_by_package_json() {
    let tmp = TempDir::new().unwrap();
    create_file(tmp.path(), "package.json");
    let fp = FilePath::new(tmp.path().to_string_lossy().to_string()).unwrap();
    assert_eq!(make_detector().detect(&fp), WorkspaceType::TypeScript);
}

#[test]
fn detect_python_workspace_by_pyproject_toml() {
    let tmp = TempDir::new().unwrap();
    create_file(tmp.path(), "pyproject.toml");
    let fp = FilePath::new(tmp.path().to_string_lossy().to_string()).unwrap();
    assert_eq!(make_detector().detect(&fp), WorkspaceType::Python);
}

#[test]
fn detect_python_workspace_by_setup_py() {
    let tmp = TempDir::new().unwrap();
    create_file(tmp.path(), "setup.py");
    let fp = FilePath::new(tmp.path().to_string_lossy().to_string()).unwrap();
    assert_eq!(make_detector().detect(&fp), WorkspaceType::Python);
}

#[test]
fn detect_python_workspace_by_requirements_txt() {
    let tmp = TempDir::new().unwrap();
    create_file(tmp.path(), "requirements.txt");
    let fp = FilePath::new(tmp.path().to_string_lossy().to_string()).unwrap();
    assert_eq!(make_detector().detect(&fp), WorkspaceType::Python);
}

#[test]
fn detect_rust_takes_priority_over_typescript() {
    let tmp = TempDir::new().unwrap();
    create_file(tmp.path(), "Cargo.toml");
    create_file(tmp.path(), "package.json");
    let fp = FilePath::new(tmp.path().to_string_lossy().to_string()).unwrap();
    assert_eq!(make_detector().detect(&fp), WorkspaceType::Rust);
}

#[test]
fn detect_unknown_when_no_markers() {
    let tmp = TempDir::new().unwrap();
    let fp = FilePath::new(tmp.path().to_string_lossy().to_string()).unwrap();
    assert_eq!(make_detector().detect(&fp), WorkspaceType::Unknown);
}

#[test]
fn detect_by_parent_directory_name_crates() {
    let tmp = TempDir::new().unwrap();
    let member = tmp.path().join("crates").join("my-crate");
    fs::create_dir_all(&member).unwrap();
    let fp = FilePath::new(member.to_string_lossy().to_string()).unwrap();
    assert_eq!(make_detector().detect(&fp), WorkspaceType::Rust);
}

#[test]
fn detect_by_parent_directory_name_packages() {
    let tmp = TempDir::new().unwrap();
    let member = tmp.path().join("packages").join("my-pkg");
    fs::create_dir_all(&member).unwrap();
    let fp = FilePath::new(member.to_string_lossy().to_string()).unwrap();
    assert_eq!(make_detector().detect(&fp), WorkspaceType::TypeScript);
}

#[test]
fn detect_by_parent_directory_name_modules() {
    let tmp = TempDir::new().unwrap();
    let member = tmp.path().join("modules").join("my-mod");
    fs::create_dir_all(&member).unwrap();
    let fp = FilePath::new(member.to_string_lossy().to_string()).unwrap();
    assert_eq!(make_detector().detect(&fp), WorkspaceType::Python);
}

#[test]
fn is_workspace_true_when_crates_dir_exists() {
    let tmp = TempDir::new().unwrap();
    fs::create_dir(tmp.path().join("crates")).unwrap();
    let fp = FilePath::new(tmp.path().to_string_lossy().to_string()).unwrap();
    assert!(make_detector().is_workspace(&fp));
}

#[test]
fn is_workspace_true_when_packages_dir_exists() {
    let tmp = TempDir::new().unwrap();
    fs::create_dir(tmp.path().join("packages")).unwrap();
    let fp = FilePath::new(tmp.path().to_string_lossy().to_string()).unwrap();
    assert!(make_detector().is_workspace(&fp));
}

#[test]
fn is_workspace_true_when_modules_dir_exists() {
    let tmp = TempDir::new().unwrap();
    fs::create_dir(tmp.path().join("modules")).unwrap();
    let fp = FilePath::new(tmp.path().to_string_lossy().to_string()).unwrap();
    assert!(make_detector().is_workspace(&fp));
}

#[test]
fn is_workspace_false_when_no_workspace_dirs() {
    let tmp = TempDir::new().unwrap();
    let fp = FilePath::new(tmp.path().to_string_lossy().to_string()).unwrap();
    assert!(!make_detector().is_workspace(&fp));
}

#[tokio::test]
async fn discover_members_under_crates_dir() {
    let tmp = TempDir::new().unwrap();
    let crates = tmp.path().join("crates");
    fs::create_dir_all(crates.join("alpha")).unwrap();
    fs::create_dir_all(crates.join("beta")).unwrap();
    fs::write(crates.join("README.md"), "# readme").unwrap();
    let fp = FilePath::new(tmp.path().to_string_lossy().to_string()).unwrap();
    let members = make_detector().discover_workspace_members(&fp).await;
    assert_eq!(members.len(), 2);
    let names: Vec<String> = members.iter().map(|m| m.basename()).collect();
    assert!(names.contains(&"alpha".to_string()));
    assert!(names.contains(&"beta".to_string()));
}

#[tokio::test]
async fn discover_members_under_packages_dir() {
    let tmp = TempDir::new().unwrap();
    let packages = tmp.path().join("packages");
    fs::create_dir_all(packages.join("ui")).unwrap();
    fs::create_dir_all(packages.join("api")).unwrap();
    let fp = FilePath::new(tmp.path().to_string_lossy().to_string()).unwrap();
    let members = make_detector().discover_workspace_members(&fp).await;
    assert_eq!(members.len(), 2);
}

#[tokio::test]
async fn discover_members_under_modules_dir() {
    let tmp = TempDir::new().unwrap();
    fs::create_dir_all(tmp.path().join("modules").join("core")).unwrap();
    let fp = FilePath::new(tmp.path().to_string_lossy().to_string()).unwrap();
    let members = make_detector().discover_workspace_members(&fp).await;
    assert_eq!(members.len(), 1);
    assert_eq!(members[0].basename(), "core");
}

#[tokio::test]
async fn discover_members_returns_empty_when_no_workspace_dirs() {
    let tmp = TempDir::new().unwrap();
    let fp = FilePath::new(tmp.path().to_string_lossy().to_string()).unwrap();
    assert!(
        make_detector()
            .discover_workspace_members(&fp)
            .await
            .is_empty()
    );
}

#[tokio::test]
async fn discover_members_from_within_workspace_dir() {
    let tmp = TempDir::new().unwrap();
    let crates = tmp.path().join("crates");
    fs::create_dir_all(crates.join("one")).unwrap();
    fs::create_dir_all(crates.join("two")).unwrap();
    let fp = FilePath::new(crates.to_string_lossy().to_string()).unwrap();
    let members = make_detector().discover_workspace_members(&fp).await;
    assert_eq!(members.len(), 2);
}

// ─── Regression: Phase 3.4 — File paths deep inside modules/packages/crates ─────

/// A file nested under `modules/<member>/src/` should still detect Python.
/// This was broken before the fix because the parent-chain walk didn't check
/// for workspace markers, and the depth limit was too shallow.
#[test]
fn detect_python_from_file_deep_in_modules_src() {
    let tmp = TempDir::new().unwrap();
    let file = tmp
        .path()
        .join("modules")
        .join("security")
        .join("src")
        .join("capabilities_archive_guard.py");
    fs::create_dir_all(file.parent().unwrap()).unwrap();
    fs::write(&file, "# test\n").unwrap();
    let fp = FilePath::new(file.to_string_lossy().to_string()).unwrap();
    assert_eq!(make_detector().detect(&fp), WorkspaceType::Python);
}

/// A file nested under `crates/<member>/src/` should detect Rust.
#[test]
fn detect_rust_from_file_deep_in_crates_src() {
    let tmp = TempDir::new().unwrap();
    let file = tmp
        .path()
        .join("crates")
        .join("my-crate")
        .join("src")
        .join("lib.rs");
    fs::create_dir_all(file.parent().unwrap()).unwrap();
    fs::write(&file, "fn main() {}\n").unwrap();
    let fp = FilePath::new(file.to_string_lossy().to_string()).unwrap();
    assert_eq!(make_detector().detect(&fp), WorkspaceType::Rust);
}

/// A file nested under `packages/<member>/src/` should detect TypeScript.
#[test]
fn detect_typescript_from_file_deep_in_packages_src() {
    let tmp = TempDir::new().unwrap();
    let file = tmp
        .path()
        .join("packages")
        .join("my-pkg")
        .join("src")
        .join("index.ts");
    fs::create_dir_all(file.parent().unwrap()).unwrap();
    fs::write(&file, "export const x = 1;\n").unwrap();
    let fp = FilePath::new(file.to_string_lossy().to_string()).unwrap();
    assert_eq!(make_detector().detect(&fp), WorkspaceType::TypeScript);
}

/// A file nested 4 levels deep under modules should still detect Python.
/// This tests the increased depth limit (from 2 to 5).
#[test]
fn detect_python_from_file_very_deep_in_modules() {
    let tmp = TempDir::new().unwrap();
    let file = tmp
        .path()
        .join("modules")
        .join("security")
        .join("src")
        .join("sub")
        .join("deep")
        .join("test.py");
    fs::create_dir_all(file.parent().unwrap()).unwrap();
    fs::write(&file, "# test\n").unwrap();
    let fp = FilePath::new(file.to_string_lossy().to_string()).unwrap();
    assert_eq!(make_detector().detect(&fp), WorkspaceType::Python);
}

/// A plain file NOT under any workspace directory should return Unknown.
#[test]
fn detect_unknown_for_file_not_in_workspace() {
    let tmp = TempDir::new().unwrap();
    let file = tmp.path().join("random_script.py");
    fs::write(&file, "# test\n").unwrap();
    let fp = FilePath::new(file.to_string_lossy().to_string()).unwrap();
    assert_eq!(make_detector().detect(&fp), WorkspaceType::Unknown);
}

#[test]
fn default_and_new_are_equivalent() {
    let a = WorkspaceDetector::new();
    let b = WorkspaceDetector::new();
    let tmp = TempDir::new().unwrap();
    create_file(tmp.path(), "Cargo.toml");
    let fp = FilePath::new(tmp.path().to_string_lossy().to_string()).unwrap();
    assert_eq!(a.detect(&fp), b.detect(&fp));
}
