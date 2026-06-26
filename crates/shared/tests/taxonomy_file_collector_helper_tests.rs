use shared_lint_arwaky::common::taxonomy_file_collector_helper::is_path_ignored;

fn ignored(patterns: &[&str]) -> Vec<String> {
    patterns.iter().map(|s| s.to_string()).collect()
}

#[test]
fn absolute_prefix_matches_at_any_depth() {
    let ig = ignored(&["/test-workspaces"]);
    assert!(is_path_ignored("/home/raka/mcp-arwaky/lint-arwaky/test-workspaces", &ig));
    assert!(is_path_ignored("/home/raka/mcp-arwaky/lint-arwaky/test-workspaces/crates/foo.rs", &ig));
    assert!(is_path_ignored("test-workspaces", &ig));
    assert!(is_path_ignored("test-workspaces/crates/foo.rs", &ig));
}

#[test]
fn absolute_prefix_does_not_match_unrelated_segment() {
    let ig = ignored(&["/test-workspaces"]);
    assert!(!is_path_ignored("/home/not-test-workspaces/foo.rs", &ig));
    assert!(!is_path_ignored("/home/raka/lint-arwaky/crates/test.rs", &ig));
    assert!(!is_path_ignored("/home/not-test-workspaces", &ig));
}

#[test]
fn absolute_prefix_nested_path() {
    let ig = ignored(&["/packages/vscode-extension"]);
    assert!(is_path_ignored("packages/vscode-extension/src/extension.ts", &ig));
    assert!(!is_path_ignored("packages/some-other/src/foo.ts", &ig));
}

#[test]
fn bare_segment_matches_anywhere() {
    let ig = ignored(&["node_modules"]);
    assert!(is_path_ignored("node_modules/lodash/index.js", &ig));
    assert!(is_path_ignored("frontend/node_modules/react/index.js", &ig));
}

#[test]
fn suffix_glob_matches_minified_vendor_files() {
    let ig = ignored(&[".min.js", ".min.css"]);
    assert!(is_path_ignored("packages/vscode-extension/media/cytoscape.min.js", &ig));
    assert!(is_path_ignored("static/style.min.css", &ig));
    assert!(!is_path_ignored("packages/foo/index.js", &ig));
}

#[test]
fn empty_pattern_ignored() {
    let ig = ignored(&[""]);
    assert!(!is_path_ignored("anything.rs", &ig));
}

#[test]
fn multiple_patterns_any_match() {
    let ig = ignored(&["/target", "/test-workspaces", ".min.js"]);
    assert!(is_path_ignored("/home/raka/target/debug/foo.rs", &ig));
    assert!(is_path_ignored("/home/raka/test-workspaces/foo.rs", &ig));
    assert!(is_path_ignored("/home/raka/lib/vendor.min.js", &ig));
    assert!(!is_path_ignored("/home/raka/crates/foo.rs", &ig));
}

#[test]
fn packages_pattern_excludes_only_packages_segment() {
    let ig = ignored(&["/packages"]);
    assert!(!is_path_ignored("/home/raka/crates/foo.rs", &ig));
    assert!(is_path_ignored("/home/raka/packages/foo.ts", &ig));
    assert!(is_path_ignored("/home/raka/packages/vscode-extension/src/extension.ts", &ig));
    assert!(!is_path_ignored("/home/raka/crates/packages-fake/foo.ts", &ig));
}
