use config_system_lint_arwaky::infrastructure_workspace_detector_provider::{WorkspaceType};

#[test]
fn test_workspace_detection_concept() {
    assert_eq!(WorkspaceType::Rust.as_str(), "rust");
    assert_eq!(WorkspaceType::TypeScript.as_str(), "typescript");
    assert_eq!(WorkspaceType::Python.as_str(), "python");
    assert_eq!(WorkspaceType::Unknown.as_str(), "unknown");
}
