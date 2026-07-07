// PURPOSE: Integration tests for McpContainer — DI wiring for MCP server aggregates
//
// McpContainer::new_default() creates all 8 sub-containers via their own
// new_default() / new() constructors. These tests verify the wiring works
// (no panics, no NPEs) without actually running lint pipelines.

use mcp_server_lint_arwaky::root_mcp_container::McpContainer;

#[test]
fn container_orphan_downcast_check() {
    use shared::orphan_detector::contract_orphan_aggregate::IOrphanAggregate;
    let container = McpContainer::new_default();
    let _orphan: &dyn IOrphanAggregate = container.orphan_orchestrator.as_ref();
    // downcast compiles — no need to call methods that require file I/O
}

#[test]
fn container_scanner_provider_downcasts_to_scanner_port() {
    use shared::common::contract_scanner_provider_port::IScannerProviderPort;
    use shared::common::taxonomy_path_vo::DirectoryPath;
    let container = McpContainer::new_default();
    let scanner: &dyn IScannerProviderPort = container.scanner_provider.as_ref();
    let dir = DirectoryPath::new("/tmp".to_string()).unwrap_or_default();
    let _result = scanner.scan_directory(&dir);
    // just verify the scanner compiles and doesn't panic
}

#[test]
fn container_layer_detector_get_layer_def_returns_some() {
    use shared::code_analysis::contract_layer_detection_aggregate::ILayerDetectionAggregate;
    let container = McpContainer::new_default();
    let layer: &dyn ILayerDetectionAggregate = container.layer_detector.as_ref();
    let def = layer.get_layer_def("taxonomy");
    assert!(def.is_some(), "taxonomy layer definition should exist");
}

#[test]
fn container_external_lint_adapter_names_are_non_empty() {
    let container = McpContainer::new_default();
    let names = container.external_lint.adapter_names();
    assert!(!names.is_empty(), "should have at least one adapter name");
    assert!(
        names.contains(&"ruff".to_string())
            || names.contains(&"clippy".to_string())
            || names.contains(&"eslint".to_string()),
        "should contain a known adapter"
    );
}
