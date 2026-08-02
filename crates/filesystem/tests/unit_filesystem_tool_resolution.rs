// Unit tests for CapabilitiesToolResolution — FR-004: Tool Resolution.
use filesystem_lint_arwaky::capabilities_tool_resolution::CapabilitiesToolResolution;
use shared::common::taxonomy_path_vo::FilePath;
use shared::filesystem::contract_tool_resolution_protocol::IToolResolutionProtocol;
use shared::filesystem::taxonomy_filesystem_vo::ToolName;
use tempfile::TempDir;

fn make_tool() -> CapabilitiesToolResolution {
    CapabilitiesToolResolution::new()
}

#[test]
fn is_binary_available_sh() {
    let tool = make_tool();
    let name = ToolName::new("sh").unwrap();
    assert!(tool.is_binary_available(&name));
}

#[test]
fn is_binary_available_nonexistent() {
    let tool = make_tool();
    let name = ToolName::new("totally_nonexistent_binary_99999").unwrap();
    assert!(!tool.is_binary_available(&name));
}

#[test]
fn has_config_file_detects_eslintrc() {
    let tmp = TempDir::new().unwrap();
    std::fs::write(tmp.path().join(".eslintrc"), "{}").unwrap();
    let tool = make_tool();
    assert!(tool.has_config_file(tmp.path()));
}

#[test]
fn has_config_file_false_for_empty_dir() {
    let tmp = TempDir::new().unwrap();
    let tool = make_tool();
    assert!(!tool.has_config_file(tmp.path()));
}

#[test]
fn has_cargo_toml_finds_it() {
    let tmp = TempDir::new().unwrap();
    std::fs::write(tmp.path().join("Cargo.toml"), "").unwrap();
    let tool = make_tool();
    let fp = FilePath::new(tmp.path().to_string_lossy().to_string()).unwrap();
    let result = tool.has_cargo_toml(&fp);
    assert!(result.is_some());
}

#[test]
fn has_cargo_lock_finds_it() {
    let tmp = TempDir::new().unwrap();
    std::fs::write(tmp.path().join("Cargo.lock"), "").unwrap();
    let tool = make_tool();
    let fp = FilePath::new(tmp.path().to_string_lossy().to_string()).unwrap();
    let result = tool.has_cargo_lock(&fp);
    assert!(result.is_some());
}

#[test]
fn has_local_bin_false_for_empty_dir() {
    let tmp = TempDir::new().unwrap();
    let tool = make_tool();
    let name = ToolName::new("eslint").unwrap();
    assert!(!tool.has_local_bin(tmp.path(), &name));
}

#[test]
fn has_local_bin_true_when_binary_exists() {
    let tmp = TempDir::new().unwrap();
    let bin_dir = tmp.path().join("node_modules").join(".bin");
    std::fs::create_dir_all(&bin_dir).unwrap();
    std::fs::write(bin_dir.join("eslint"), "#!/bin/sh\n").unwrap();
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        std::fs::set_permissions(bin_dir.join("eslint"), std::fs::Permissions::from_mode(0o755))
            .unwrap();
    }
    let tool = make_tool();
    let name = ToolName::new("eslint").unwrap();
    assert!(tool.has_local_bin(tmp.path(), &name));
}

#[test]
fn is_python_file_recursive_true_when_py_exists() {
    let tmp = TempDir::new().unwrap();
    std::fs::write(tmp.path().join("main.py"), "").unwrap();
    let tool = make_tool();
    let fp = FilePath::new(tmp.path().to_string_lossy().to_string()).unwrap();
    assert!(tool.is_python_file_recursive(&fp));
}

#[test]
fn is_python_file_recursive_false_for_no_python() {
    let tmp = TempDir::new().unwrap();
    std::fs::write(tmp.path().join("main.rs"), "").unwrap();
    let tool = make_tool();
    let fp = FilePath::new(tmp.path().to_string_lossy().to_string()).unwrap();
    assert!(!tool.is_python_file_recursive(&fp));
}

#[test]
fn default_working_dir_returns_path() {
    let tmp = TempDir::new().unwrap();
    let tool = make_tool();
    let fp = FilePath::new(tmp.path().join("src").join("main.rs").to_string_lossy().to_string()).unwrap();
    let result = tool.default_working_dir(&fp);
    assert!(!result.value.is_empty());
}

#[test]
fn resolve_cargo_working_dir_finds_root() {
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
fn resolve_js_working_dir_returns_path() {
    let tmp = TempDir::new().unwrap();
    let tool = make_tool();
    let fp = FilePath::new(tmp.path().to_string_lossy().to_string()).unwrap();
    let result = tool.resolve_js_working_dir(&fp);
    assert!(!result.value.is_empty());
}
