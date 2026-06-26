use shared::config_system::contract_workspace_detector_port::WorkspaceType;

#[test]
fn test_workspace_detection_concept() {
    assert_eq!(WorkspaceType::Rust.as_str(), "rust");
    assert_eq!(WorkspaceType::TypeScript.as_str(), "typescript");
    assert_eq!(WorkspaceType::Python.as_str(), "python");
    assert_eq!(WorkspaceType::Unknown.as_str(), "unknown");
}
