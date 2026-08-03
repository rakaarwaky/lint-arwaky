// Acceptance test — FR-005: MCP binary resolution and pre-flight checks.
use project_setup_lint_arwaky::root_project_setup_container::SetupContainer;

fn make_container() -> SetupContainer {
    let fs = filesystem::root_filesystem_container::FilesystemContainer::new().orchestrator();
    SetupContainer::new(fs)
}

#[test]
fn fr005_which_mcp_binary_returns_non_empty() {
    let container = make_container();
    let proto = container.protocol();
    let binary = proto.which_mcp_binary();
    assert!(
        !binary.value().is_empty(),
        "FR-005: which_mcp_binary should return a non-empty path"
    );
}

#[test]
fn fr005_which_mcp_binary_contains_lint_arwaky_mcp() {
    let container = make_container();
    let proto = container.protocol();
    let binary = proto.which_mcp_binary();
    assert!(
        binary.value().contains("lint-arwaky-mcp") || binary.value().contains("lint-arwaky"),
        "FR-005: binary path '{}' should reference lint-arwaky-mcp",
        binary.value()
    );
}

#[test]
fn fr005_file_exists_check() {
    let container = make_container();
    let proto = container.protocol();
    assert!(
        proto.file_exists("Cargo.toml"),
        "FR-005: Cargo.toml should exist"
    );
    assert!(!proto.file_exists("definitely_does_not_exist_12345.txt"));
}

#[test]
fn fr005_pre_flight_check() {
    let container = make_container();
    let proto = container.protocol();
    let results = proto.pre_flight_check();
    assert!(
        !results.is_empty(),
        "FR-005: pre_flight_check should return results"
    );
}
