// Acceptance test — FR-002: Environment generation produces valid .env with PHANTOM_ROOT.
use project_setup_lint_arwaky::root_project_setup_container::SetupContainer;
use shared::cli_commands::taxonomy_protocol_vo::TransportProtocol;
use shared::common::taxonomy_path_vo::DirectoryPath;
use shared::project_setup::SetupManagementAggregate;
use tempfile::TempDir;

fn make_container() -> SetupContainer {
    let fs = filesystem::root_filesystem_container::FilesystemContainer::new().orchestrator();
    SetupContainer::new(fs)
}

#[test]
fn fr002_env_contains_phantom_root() {
    let container = make_container();
    let agg = container.aggregate();
    let tmp = TempDir::new().unwrap();
    let home = DirectoryPath::new(tmp.path().to_string_lossy().to_string()).unwrap();
    let env = agg.generate_env(&TransportProtocol::STDAggregate, &home);
    assert!(
        env.value().contains("PHANTOM_ROOT="),
        "FR-002: .env must contain PHANTOM_ROOT"
    );
}

#[test]
fn fr002_env_contains_transport() {
    let container = make_container();
    let agg = container.aggregate();
    let tmp = TempDir::new().unwrap();
    let home = DirectoryPath::new(tmp.path().to_string_lossy().to_string()).unwrap();
    let env = agg.generate_env(&TransportProtocol::STDAggregate, &home);
    assert!(
        env.value().contains("TRANSPORT="),
        "FR-002: .env must contain TRANSPORT"
    );
}

#[test]
fn fr002_env_writable_to_disk() {
    let container = make_container();
    let agg = container.aggregate();
    let proto = container.protocol();
    let tmp = TempDir::new().unwrap();
    let home = DirectoryPath::new(tmp.path().to_string_lossy().to_string()).unwrap();
    let env = agg.generate_env(&TransportProtocol::STDAggregate, &home);
    let path = tmp.path().join(".env");
    let result = proto.write_config_file(&path.to_string_lossy(), env.value());
    assert!(result.is_ok(), "FR-002: .env should be writable");
    let content = std::fs::read_to_string(&path).unwrap();
    assert!(content.contains("PHANTOM_ROOT="));
}
