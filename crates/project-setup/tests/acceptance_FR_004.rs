// Acceptance test — Utility: Config file writing and global config directory.
use shared_project_setup::SetupError;
use tempfile::TempDir;

#[test]
fn fr004_write_config_file_returns_description() {
    let tmp = TempDir::new().unwrap();
    let path = tmp.path().join("config.yaml");
    let result = shared_project_setup::utility_project_setup_helpers::write_config_file(
        &path.to_string_lossy(),
        "rules:\n  enabled: true\n",
    );
    assert!(result.is_ok(), "write_config_file should succeed");
    let desc = result.unwrap();
    assert!(desc.value().contains("config.yaml"));
}

#[test]
fn fr004_write_config_file_creates_file_on_disk() {
    let tmp = TempDir::new().unwrap();
    let path = tmp.path().join("verify.yaml");
    shared_project_setup::utility_project_setup_helpers::write_config_file(
        &path.to_string_lossy(),
        "key: value",
    )
    .unwrap();
    assert!(path.exists(), "config file should exist on disk");
}

#[test]
fn fr004_write_config_file_error_on_invalid_path() {
    let result = shared_project_setup::utility_project_setup_helpers::write_config_file(
        "/nonexistent/deeply/nested/file.yaml",
        "content",
    );
    assert!(result.is_err(), "invalid path should return error");
    match result.unwrap_err() {
        SetupError::Io(_) => {}
        other => panic!("expected Io error, got: {:?}", other),
    }
}

#[test]
fn fr004_create_global_config_dir() {
    let result = shared_project_setup::utility_project_setup_helpers::create_global_config_dir();
    match result {
        Ok(path) => {
            assert!(
                path.to_string_lossy().contains("lint-arwaky"),
                "config dir should contain 'lint-arwaky'"
            );
        }
        Err(SetupError::InvalidState(_)) => {}
        Err(e) => panic!("unexpected error: {:?}", e),
    }
}
