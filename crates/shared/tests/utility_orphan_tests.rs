extern crate shared_lint_arwaky as shared;

use shared::orphan_detector::utility_orphan_detector::has_trait_implementation;

// ─── Regression Tests for Phase 3 Fixes ──────────────────────────────────────────

/// Regression test for Phase 3.3: has_trait_implementation strips generics correctly.
/// Trait names with generics like `HashMap<String, i32>` should be matched by their
/// base name (e.g., "HashMap") without the generic parameters.
#[test]
fn has_trait_implementation_strips_generics() {
    let content = "impl HashMap<String, i32> for MyStruct {\n    fn foo(&self) {}\n}";
    assert!(
        has_trait_implementation(content, "HashMap"),
        "Should match 'HashMap' from 'impl HashMap<String, i32>'"
    );
}

/// Regression test: has_trait_implementation handles nested generics.
#[test]
fn has_trait_implementation_nested_generics() {
    let content = "impl Result<Vec<String>, Error> for MyStruct {\n    fn bar(&self) {}\n}";
    assert!(
        has_trait_implementation(content, "Result"),
        "Should match 'Result' from nested generics"
    );
    assert!(
        !has_trait_implementation(content, "Vec"),
        "Should NOT match 'Vec' when it's a nested generic param"
    );
}

/// Regression test: has_trait_implementation handles empty string.
#[test]
fn has_trait_implementation_empty_content() {
    assert!(!has_trait_implementation("", "SomeTrait"));
}

/// Regression test: has_trait_implementation handles non-matching traits.
#[test]
fn has_trait_implementation_no_match() {
    let content = "impl Vec<String> for MyStruct {\n    fn foo(&self) {}\n}";
    assert!(
        !has_trait_implementation(content, "HashMap"),
        "Should NOT match 'HashMap' when impl is for 'Vec'"
    );
}
