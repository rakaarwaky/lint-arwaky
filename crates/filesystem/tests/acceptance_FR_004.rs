// FR-004 — Tool Resolution
// US1: Binary in system PATH is detected.
// US2: Local node_modules/.bin binary is detected.
// US3: Config file presence is detected.
// US4: Cargo.toml detection in ancestors.
// US5: Python file recursive detection.
// US6: Working directory resolution for JS and Cargo tools.

use filesystem_lint_arwaky::capabilities_tool_resolution::CapabilitiesToolResolution;
use shared::common::taxonomy_path_vo::FilePath;
use shared::filesystem::contract_tool_resolution_protocol::IToolResolutionProtocol;
use shared::filesystem::taxonomy_filesystem_vo::ToolName;
use tempfile::TempDir;

fn make_tool() -> CapabilitiesToolResolution {
    CapabilitiesToolResolution::new()
}

#[test]
fn us1_binary_in_path_detected() {
    let tool = make_tool();
    let sh = ToolName::new("sh").unwrap();
    assert!(tool.is_binary_available(&sh));
    assert!(tool.is_executable_in_path(&sh));
}

#[test]
fn us1_nonexistent_binary_not_detected() {
    let tool = make_tool();
    let name = ToolName::new("definitely_not_a_real_binary_9999").unwrap();
    assert!(!tool.is_binary_available(&name));
    assert!(!tool.is_executable_in_path(&name));
}

#[test]
fn us2_local_node_modules_binary() {
    let tmp = TempDir::new().unwrap();
    let bin_dir = tmp.path().join("node_modules").join(".bin");
    std::fs::create_dir_all(&bin_dir).unwrap();
    std::fs::write(bin_dir.join("eslint"), "#!/bin/sh\necho ok\n").unwrap();
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        std::fs::set_permissions(
            bin_dir.join("eslint"),
            std::fs::Permissions::from_mode(0o755),
        )
        .unwrap();
    }
    let tool = make_tool();
    let name = ToolName::new("eslint").unwrap();
    assert!(tool.has_local_bin(tmp.path(), &name));
}

#[test]
fn us2_missing_local_binary() {
    let tmp = TempDir::new().unwrap();
    let tool = make_tool();
    let name = ToolName::new("eslint").unwrap();
    assert!(!tool.has_local_bin(tmp.path(), &name));
}

#[test]
fn us3_config_file_detected() {
    let tmp = TempDir::new().unwrap();
    std::fs::write(tmp.path().join("tsconfig.json"), "{}").unwrap();
    let tool = make_tool();
    assert!(tool.has_config_file(tmp.path()));
}

#[test]
fn us3_no_config_file() {
    let tmp = TempDir::new().unwrap();
    let tool = make_tool();
    assert!(!tool.has_config_file(tmp.path()));
}

#[test]
fn us4_cargo_toml_found_at_path() {
    let tmp = TempDir::new().unwrap();
    std::fs::write(tmp.path().join("Cargo.toml"), "").unwrap();
    let tool = make_tool();
    let fp = FilePath::new(tmp.path().to_string_lossy().to_string()).unwrap();
    let result = tool.has_cargo_toml(&fp);
    assert!(result.is_some());
}

#[test]
fn us4_cargo_lock_found_at_path() {
    let tmp = TempDir::new().unwrap();
    std::fs::write(tmp.path().join("Cargo.lock"), "").unwrap();
    let tool = make_tool();
    let fp = FilePath::new(tmp.path().to_string_lossy().to_string()).unwrap();
    let result = tool.has_cargo_lock(&fp);
    assert!(result.is_some());
}

#[test]
fn us5_python_file_recursive_true() {
    let tmp = TempDir::new().unwrap();
    let deep = tmp.path().join("src").join("modules");
    std::fs::create_dir_all(&deep).unwrap();
    std::fs::write(deep.join("handler.py"), "").unwrap();
    let tool = make_tool();
    let fp = FilePath::new(tmp.path().to_string_lossy().to_string()).unwrap();
    assert!(tool.is_python_file_recursive(&fp));
}

#[test]
fn us5_python_file_recursive_false() {
    let tmp = TempDir::new().unwrap();
    std::fs::write(tmp.path().join("main.rs"), "").unwrap();
    let tool = make_tool();
    let fp = FilePath::new(tmp.path().to_string_lossy().to_string()).unwrap();
    assert!(!tool.is_python_file_recursive(&fp));
}

#[test]
fn us6_resolve_cargo_working_dir() {
    let tmp = TempDir::new().unwrap();
    std::fs::write(tmp.path().join("Cargo.toml"), "").unwrap();
    let nested = tmp.path().join("src");
    std::fs::create_dir_all(&nested).unwrap();
    let tool = make_tool();
    let fp = FilePath::new(nested.to_string_lossy().to_string()).unwrap();
    let result = tool.resolve_cargo_working_dir(&fp);
    assert!(result.value.contains(tmp.path().to_str().unwrap()));
}

#[test]
fn us6_resolve_js_working_dir() {
    let tmp = TempDir::new().unwrap();
    let tool = make_tool();
    let fp = FilePath::new(tmp.path().to_string_lossy().to_string()).unwrap();
    let result = tool.resolve_js_working_dir(&fp);
    assert!(!result.value.is_empty());
}

#[test]
fn us6_default_working_dir() {
    let tmp = TempDir::new().unwrap();
    let tool = make_tool();
    let fp = FilePath::new(tmp.path().to_string_lossy().to_string()).unwrap();
    let result = tool.default_working_dir(&fp);
    assert!(!result.value.is_empty());
}
