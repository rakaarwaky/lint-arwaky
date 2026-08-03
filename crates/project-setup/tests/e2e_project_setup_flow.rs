// E2E tests — full project setup flow: container → generate configs → write → verify.
use project_setup_lint_arwaky::root_project_setup_container::SetupContainer;
use shared::common::taxonomy_path_vo::DirectoryPath;
use shared::project_setup::SetupManagementAggregate;
use tempfile::TempDir;

fn make_container() -> SetupContainer {
    let fs = filesystem::root_filesystem_container::FilesystemContainer::new().orchestrator();
    SetupContainer::new(fs)
}

#[test]
fn e2e_generate_and_write_env() {
    let container = make_container();
    let agg = container.aggregate();
    let tmp = TempDir::new().unwrap();
    let home = DirectoryPath::new(tmp.path().to_string_lossy().to_string()).unwrap();
    let env = agg.generate_env(&home);
    assert!(env.value().contains("PHANTOM_ROOT="));

    let proto = container.protocol();
    let result = proto.write_config_file(
        &tmp.path().join(".env").to_string_lossy(),
        env.value(),
    );
    assert!(result.is_ok(), "Should write .env file: {:?}", result);
    let content = std::fs::read_to_string(tmp.path().join(".env")).unwrap();
    assert!(content.contains("PHANTOM_ROOT="));
}

#[test]
fn e2e_generate_mcp_config_claude_and_write() {
    let container = make_container();
    let agg = container.aggregate();
    let tmp = TempDir::new().unwrap();
    let config = agg.mcp_config_claude();
    let json_str = serde_json::to_string_pretty(config.value()).unwrap();
    let path = tmp.path().join("mcp_claude.json");
    let proto = container.protocol();
    let result = proto.write_config_file(&path.to_string_lossy(), &json_str);
    assert!(result.is_ok());
    let content = std::fs::read_to_string(&path).unwrap();
    assert!(content.contains("claude") || content.contains("lint-arwaky"));
}

#[test]
fn e2e_generate_mcp_config_vscode_and_write() {
    let container = make_container();
    let agg = container.aggregate();
    let tmp = TempDir::new().unwrap();
    let config = agg.mcp_config_vscode();
    let json_str = serde_json::to_string_pretty(config.value()).unwrap();
    let path = tmp.path().join("mcp_vscode.json");
    let proto = container.protocol();
    let result = proto.write_config_file(&path.to_string_lossy(), &json_str);
    assert!(result.is_ok());
    let content = std::fs::read_to_string(&path).unwrap();
    assert!(content.contains("mcp") || content.contains("vscode") || content.contains("lint-arwaky"));
}

#[test]
fn e2e_detect_language_and_write_config() {
    let container = make_container();
    let agg = container.aggregate();
    let tmp = TempDir::new().unwrap();

    let lang = agg.detect_language().unwrap();
    let template = agg.get_config_template(lang.value()).unwrap();
    let path = tmp.path().join("lint_arwaky.config.yaml");
    let proto = container.protocol();
    let result = proto.write_config_file(&path.to_string_lossy(), template);
    assert!(result.is_ok());
    let content = std::fs::read_to_string(&path).unwrap();
    assert!(!content.is_empty());
}

#[test]
fn e2e_file_exists_round_trip() {
    let container = make_container();
    let proto = container.protocol();
    let tmp = TempDir::new().unwrap();
    let path = tmp.path().join("round_trip.txt");
    let path_str = path.to_string_lossy().to_string();

    assert!(!proto.file_exists(&path_str));
    proto.write_config_file(&path_str, "hello").unwrap();
    assert!(proto.file_exists(&path_str));
}
