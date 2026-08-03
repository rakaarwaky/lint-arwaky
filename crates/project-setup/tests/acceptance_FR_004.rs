// Acceptance test — FR-004: Config file creation and global config directory.
use project_setup_lint_arwaky::root_project_setup_container::SetupContainer;
use shared::project_setup::SetupError;
use tempfile::TempDir;

fn make_container() -> SetupContainer {
    let fs = filesystem::root_filesystem_container::FilesystemContainer::new().orchestrator();
    SetupContainer::new(fs)
}

#[test]
fn fr004_write_config_file_returns_description() {
    let container = make_container();
    let proto = container.protocol();
    let tmp = TempDir::new().unwrap();
    let path = tmp.path().join("config.yaml");
    let result = proto.write_config_file(&path.to_string_lossy(), "rules:\n  enabled: true\n");
    assert!(result.is_ok(), "FR-004: write_config_file should succeed");
    let desc = result.unwrap();
    assert!(desc.value().contains("config.yaml"));
}

#[test]
fn fr004_write_config_file_creates_file_on_disk() {
    let container = make_container();
    let proto = container.protocol();
    let tmp = TempDir::new().unwrap();
    let path = tmp.path().join("verify.yaml");
    proto
        .write_config_file(&path.to_string_lossy(), "key: value")
        .unwrap();
    assert!(path.exists(), "FR-004: config file should exist on disk");
}

#[test]
fn fr004_write_config_file_error_on_invalid_path() {
    let container = make_container();
    let proto = container.protocol();
    let result = proto.write_config_file("/nonexistent/deeply/nested/file.yaml", "content");
    assert!(result.is_err(), "FR-004: invalid path should return error");
    match result.unwrap_err() {
        SetupError::Io(_) => {}
        other => panic!("FR-004: expected Io error, got: {:?}", other),
    }
}

#[test]
fn fr004_create_global_config_dir() {
    let container = make_container();
    let proto = container.protocol();
    let result = proto.create_global_config_dir();
    match result {
        Ok(path) => {
            assert!(
                path.to_string_lossy().contains("lint-arwaky"),
                "FR-004: config dir should contain 'lint-arwaky'"
            );
        }
        Err(SetupError::InvalidState(_)) => {}
        Err(e) => panic!("FR-004: unexpected error: {:?}", e),
    }
}
