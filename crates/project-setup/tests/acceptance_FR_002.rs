// Acceptance test — FR-002: Environment file generation produces valid .env with PHANTOM_ROOT.
use project_setup_lint_arwaky::root_project_setup_container::SetupContainer;
use shared::common::taxonomy_path_vo::DirectoryPath;
use shared::project_setup::ISetupManagementProtocol;
use tempfile::TempDir;

fn make_container() -> SetupContainer {
    let fs = filesystem::root_filesystem_container::FilesystemContainer::new().orchestrator();
    SetupContainer::new(fs)
}

#[test]
fn fr002_env_contains_phantom_root() {
    let container = make_container();
    let proto = container.protocol();
    let tmp = TempDir::new().unwrap();
    let home = DirectoryPath::new(tmp.path().to_string_lossy().to_string()).unwrap();
    let env = proto.generate_env(&home);
    assert!(
        env.value().contains("PHANTOM_ROOT="),
        "FR-002: .env must contain PHANTOM_ROOT"
    );
    assert!(
        env.value()
            .contains(&format!("PHANTOM_ROOT={}/", home.value)),
        "FR-002: PHANTOM_ROOT must include trailing slash"
    );
}

#[test]
fn fr002_env_contains_header() {
    let container = make_container();
    let proto = container.protocol();
    let tmp = TempDir::new().unwrap();
    let home = DirectoryPath::new(tmp.path().to_string_lossy().to_string()).unwrap();
    let env = proto.generate_env(&home);
    assert!(
        env.value()
            .contains("Lint Arwaky Environment Configuration"),
        "FR-002: .env must contain header comment"
    );
}

#[test]
fn fr002_env_empty_home_path() {
    let container = make_container();
    let proto = container.protocol();
    let home = DirectoryPath::new("".to_string()).unwrap();
    let env = proto.generate_env(&home);
    assert!(
        env.value().contains("PHANTOM_ROOT=/"),
        "FR-002 QA#2: empty home path → PHANTOM_ROOT=/"
    );
}

#[test]
fn fr002_env_writable_to_disk() {
    let container = make_container();
    let proto = container.protocol();
    let tmp = TempDir::new().unwrap();
    let home = DirectoryPath::new(tmp.path().to_string_lossy().to_string()).unwrap();
    let env = proto.generate_env(&home);
    let path = tmp.path().join(".env");
    let result = proto.write_config_file(&path.to_string_lossy(), env.value());
    assert!(result.is_ok(), "FR-002: .env should be writable");
    let content = std::fs::read_to_string(&path).unwrap();
    assert!(content.contains("PHANTOM_ROOT="));
}
