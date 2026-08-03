// Unit tests for utility_layer_detector — layer detection from module paths.
use shared_lint_arwaky::common::utility_layer_detector::{
    detect_module_layer, resolve_module_path_to_layer,
};

#[test]
fn test_detect_module_layer_with_prefix() {
    let layer_names: Vec<String> = vec![
        "taxonomy".into(),
        "contract".into(),
        "utility".into(),
        "capabilities".into(),
        "agent".into(),
        "surface".into(),
    ];

    // Standard module path with layer prefix in segment
    assert_eq!(
        detect_module_layer("shared.src.contract_protocol", &layer_names),
        Some("contract".to_string())
    );
}

#[test]
fn test_resolve_module_path_to_layer() {
    // Test with blender-arwaky structure
    // modules/shared/src/common/ has contract_* and taxonomy_* files
    let result = resolve_module_path_to_layer(
        "modules.shared.src.common",
        "/home/raka/mcp-arwaky/blender-arwaky",
    );
    assert!(
        result.is_some(),
        "Should detect layer from common directory (has contract_* and taxonomy_* files)"
    );
    assert!(
        result.as_ref().unwrap() == "contract" || result.as_ref().unwrap() == "taxonomy",
        "Detected layer should be contract or taxonomy"
    );
}
