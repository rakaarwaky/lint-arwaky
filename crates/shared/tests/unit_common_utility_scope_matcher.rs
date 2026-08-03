// Unit tests for utility_scope_matcher — scope-based file matching helpers.
use shared_lint_arwaky::common::Identity;
use shared_lint_arwaky::common::utility_scope_matcher::{
    extract_file_stem, extract_layer_prefix, extract_suffix, file_belongs_to_scope,
};

#[test]
fn test_file_belongs_to_scope_matches() {
    let scope = Identity::new("surfaces");
    let result = file_belongs_to_scope("surfaces_auth.rs", &scope);
    assert!(result.is_some());
    let (layer, suffixes) = result.unwrap();
    assert_eq!(layer, "surfaces");
    assert!(suffixes.is_empty());
}

#[test]
fn test_file_belongs_to_scope_no_match() {
    let scope = Identity::new("surfaces");
    let result = file_belongs_to_scope("utility_auth.rs", &scope);
    assert!(result.is_none());
}

#[test]
fn test_extract_file_stem() {
    assert_eq!(extract_file_stem("surfaces_auth.rs"), "surfaces_auth");
    assert_eq!(extract_file_stem("mod.rs"), "mod");
    assert_eq!(extract_file_stem("lib.rs"), "lib");
    assert_eq!(extract_file_stem("no_extension"), "no_extension");
}

#[test]
fn test_extract_layer_prefix() {
    assert_eq!(extract_layer_prefix("surfaces_auth"), "surfaces");
    assert_eq!(extract_layer_prefix("utility_parser"), "utility");
    assert_eq!(extract_layer_prefix("unknown_file"), "unknown");
    assert_eq!(extract_layer_prefix("single"), "single");
}

#[test]
fn test_extract_suffix() {
    assert_eq!(extract_suffix("surfaces_auth"), "auth");
    assert_eq!(extract_suffix("utility_parser"), "parser");
    assert_eq!(extract_suffix("no_suffix"), "suffix");
    assert_eq!(extract_suffix("single"), "single");
}
