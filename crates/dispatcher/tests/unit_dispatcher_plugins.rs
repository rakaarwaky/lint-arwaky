// Unit tests — collect_adapters_detailed pins the surface adapter list.
// Pins that markdownlint-cli and Markdown labelling were removed from the
// dispatcher surface layer.
#[allow(dead_code, unused_imports)]
#[path = "../../shared/tests/mock_filesystem.rs"]
mod mock_filesystem;

use dispatcher_lint_arwaky::surface_plugin_action::{AdapterDetail, collect_adapters_detailed};
use mock_filesystem::MockFilesystem;

/// Surface adapter list — 3 built-in + 8 external (markdownlint removed).
/// Update both this count and the built-in assertion together if the surface
/// adapter set ever changes.
const EXPECTED_ADAPTER_COUNT: usize = 11;

#[test]
fn collect_adapters_detailed_returns_pinned_list_without_markdownlint() {
    let list = collect_adapters_detailed(&MockFilesystem::new());

    assert_eq!(
        list.len(),
        EXPECTED_ADAPTER_COUNT,
        "surface adapter list changed: {list:?}"
    );

    // None may be the markdownlint adapter, nor labelled Markdown.
    for adapter in &list {
        assert_ne!(
            adapter.name, "markdownlint-cli",
            "markdownlint must not surface: {list:?}"
        );
        assert!(
            !adapter.label.contains("Markdown"),
            "Markdown-labelled adapter must not surface: {list:?}"
        );
    }

    // The three built-in adapters are always present and marked installed.
    let built_in: Vec<&AdapterDetail> = list.iter().filter(|a| a.installed).collect();
    assert_eq!(
        built_in.iter().map(|a| a.name.as_str()).collect::<Vec<_>>(),
        vec!["ast_rust_scanner", "ast_py_scanner", "ast_js_scanner"]
    );
    assert!(built_in.iter().all(|a| a.installed));
}
