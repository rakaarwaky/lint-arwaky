use std::collections::HashMap;

use import_rules_lint_arwaky::utility_import_symbol_extractor::{
    extract_used_symbols, is_name_used,
};
use shared::common::taxonomy_layer_vo::Identity;

#[test]
fn derive_macro_serialize_always_used() {
    let content = r#"
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Config {
    name: String,
}
"#;
    let mut aliases = HashMap::new();
    aliases.insert(
        Identity::new("Serialize"),
        Identity::new("serde::Serialize"),
    );
    aliases.insert(
        Identity::new("Deserialize"),
        Identity::new("serde::Deserialize"),
    );

    let used = extract_used_symbols("test.rs", content, &aliases);
    assert!(
        used.contains(&Identity::new("Serialize")),
        "Serialize should always be considered used"
    );
    assert!(
        used.contains(&Identity::new("Deserialize")),
        "Deserialize should always be considered used"
    );
}

#[test]
fn derive_macro_async_trait_always_used() {
    let content = r#"
use async_trait::async_trait;

#[async_trait]
trait MyTrait {
    async fn do_something();
}
"#;
    let mut aliases = HashMap::new();
    aliases.insert(
        Identity::new("async_trait"),
        Identity::new("async_trait::async_trait"),
    );

    let used = extract_used_symbols("test.rs", content, &aliases);
    assert!(
        used.contains(&Identity::new("async_trait")),
        "async_trait should always be considered used"
    );
}

#[test]
fn derive_macro_enum_iter_always_used() {
    // EnumIter was NOT previously in is_rust_trait_import — only DERIVE_MACROS catches it
    let content = r#"
use strum::{EnumIter, Display};

#[derive(EnumIter, Display)]
enum Color {
    Red,
    Green,
}
"#;
    let mut aliases = HashMap::new();
    aliases.insert(Identity::new("EnumIter"), Identity::new("strum::EnumIter"));
    aliases.insert(Identity::new("Display"), Identity::new("strum::Display"));

    let used = extract_used_symbols("test.rs", content, &aliases);
    assert!(
        used.contains(&Identity::new("EnumIter")),
        "EnumIter should always be considered used"
    );
    assert!(
        used.contains(&Identity::new("Display")),
        "Display should always be considered used"
    );
}

#[test]
fn derive_macro_as_ref_str_always_used() {
    // AsRefStr was NOT previously in is_rust_trait_import — only DERIVE_MACROS catches it
    let content = r#"
use strum::AsRefStr;

#[derive(AsRefStr)]
enum Status {
    Active,
    Inactive,
}
"#;
    let mut aliases = HashMap::new();
    aliases.insert(Identity::new("AsRefStr"), Identity::new("strum::AsRefStr"));

    let used = extract_used_symbols("test.rs", content, &aliases);
    assert!(
        used.contains(&Identity::new("AsRefStr")),
        "AsRefStr should always be considered used"
    );
}

#[test]
fn non_derive_import_still_checked_normally() {
    // Regular imports should NOT be auto-marked as used
    let content = r#"
use std::collections::HashMap;

fn main() {
    let _x = 42;
}
"#;
    let mut aliases = HashMap::new();
    aliases.insert(
        Identity::new("HashMap"),
        Identity::new("std::collections::HashMap"),
    );

    let used = extract_used_symbols("test.rs", content, &aliases);
    assert!(
        !used.contains(&Identity::new("HashMap")),
        "HashMap is genuinely unused"
    );
}

#[test]
fn is_name_used_detects_identifiers_via_ast() {
    // AST-based is_name_used detects identifiers in code
    let test_cases = &[
        (
            "test.rs",
            "Serialize",
            "fn main() { let x = Serialize; }",
            true,
        ),
        ("test.rs", "unused", "fn main() { let x = 42; }", false),
        (
            "test.rs",
            "my_fn",
            "fn my_fn() {} fn main() { my_fn(); }",
            true,
        ),
    ];
    for &(file_path, name, content, expected) in test_cases {
        assert_eq!(is_name_used(file_path, name, content, 0), expected);
    }
}
