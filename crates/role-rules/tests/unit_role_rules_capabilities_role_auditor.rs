// Unit tests for CapabilitiesRoleChecker — capabilities-layer role audit (AES403).
use role_rules_lint_arwaky::capabilities_capabilities_role_auditor::CapabilitiesRoleChecker;
use shared::common::Severity;
use shared::filesystem::taxonomy_filesystem_vo::FileEntry;
use shared::role_rules::ICapabilitiesRoleChecker;

use shared::filesystem::taxonomy_filesystem_vo::{Language, ParseMetadata, RustMetadata};
use std::path::PathBuf;

fn checker() -> CapabilitiesRoleChecker {
    CapabilitiesRoleChecker::new()
}

fn make_file(path: &str, lang: Language, content: &str) -> FileEntry {
    FileEntry {
        path: PathBuf::from(path),
        extension: match lang {
            Language::Rust => "rs",
            Language::Python => "py",
            _ => "ts",
        }
        .to_string(),
        language: lang,
        size: content.len() as u64,
        content: content.to_string(),
        parse_ok: false,
        parse_metadata: None,
    }
}

fn make_file_with_rust_meta(path: &str, meta: RustMetadata) -> FileEntry {
    FileEntry {
        path: PathBuf::from(path),
        extension: "rs".to_string(),
        language: Language::Rust,
        size: 100,
        content: String::new(),
        parse_ok: true,
        parse_metadata: Some(ParseMetadata::Rust(meta)),
    }
}

#[test]
fn construction_succeeds() {
    let _ = checker();
}

#[test]
fn non_capabilities_layer_skipped() {
    let f = make_file("src/agent_foo.rs", Language::Rust, "pub struct Foo {}");
    let mut v = Vec::new();
    checker().check_capability_routing(&f, "agent", &mut v);
    assert!(
        v.is_empty(),
        "non-capabilities layer must not produce violations"
    );
}

#[test]
fn capabilities_layer_with_parens_routes() {
    // "capabilities(agent)" starts with "capabilities(" so it IS routed
    let f = make_file("src/agent_foo.rs", Language::Rust, "pub struct Foo {}");
    let mut v = Vec::new();
    checker().check_capability_routing(&f, "capabilities(agent)", &mut v);
    // Single struct without impl — should be flagged
    assert!(
        !v.is_empty(),
        "capabilities(agent) should route to capabilities checker"
    );
}

#[test]
fn fallback_rust_no_implementor_flagged() {
    let content = "pub struct Foo {}\n";
    let f = make_file("src/capabilities_something.rs", Language::Rust, content);
    let mut v = Vec::new();
    checker().check_capability_routing(&f, "capabilities", &mut v);
    assert!(!v.is_empty(), "should flag missing implementor");
    assert_eq!(v[0].code.code(), "AES403");
}

#[test]
fn fallback_rust_too_many_types_flagged() {
    let content = "pub struct A {}\npub struct B {}\npub struct C {}\npub struct D {}\n";
    let f = make_file("src/capabilities_something.rs", Language::Rust, content);
    let mut v = Vec::new();
    checker().check_capability_routing(&f, "capabilities", &mut v);
    assert!(!v.is_empty(), "should flag too many types");
    assert_eq!(v[0].code.code(), "AES403");
    assert_eq!(v[0].severity, Severity::HIGH);
}

#[test]
fn fallback_rust_valid_composition_no_violation() {
    let content = "pub struct Foo {}\nimpl IFooProtocol for Foo {}\n";
    let f = make_file("src/capabilities_something.rs", Language::Rust, content);
    let mut v = Vec::new();
    checker().check_capability_routing(&f, "capabilities", &mut v);
    assert!(v.is_empty(), "valid capability composition should pass");
}

#[test]
fn fallback_python_no_parent_flagged() {
    let content = "class Foo:\n    pass\n";
    let f = make_file("src/capabilities_something.py", Language::Python, content);
    let mut v = Vec::new();
    checker().check_capability_routing(&f, "capabilities", &mut v);
    assert!(
        !v.is_empty(),
        "python class without parent should be flagged"
    );
    assert_eq!(v[0].code.code(), "AES403");
}

#[test]
fn fallback_python_with_parent_no_violation() {
    let content = "class Foo(Protocol):\n    pass\n";
    let f = make_file("src/capabilities_something.py", Language::Python, content);
    let mut v = Vec::new();
    checker().check_capability_routing(&f, "capabilities", &mut v);
    assert!(v.is_empty(), "python class with parent should pass");
}

#[test]
fn metadata_rust_no_implementor_flagged() {
    let meta = RustMetadata {
        struct_definitions: vec!["Foo".into()],
        ..Default::default()
    };
    let f = make_file_with_rust_meta("src/capabilities_something.rs", meta);
    let mut v = Vec::new();
    checker().check_capability_routing(&f, "capabilities", &mut v);
    assert!(
        !v.is_empty(),
        "should flag missing implementor via metadata"
    );
    assert_eq!(v[0].code.code(), "AES403");
}

#[test]
fn metadata_rust_valid_composition_no_violation() {
    let meta = RustMetadata {
        struct_definitions: vec!["Foo".into()],
        impl_blocks: vec![shared::filesystem::taxonomy_filesystem_vo::RustImplItem {
            trait_name: Some("IFooProtocol".into()),
            trait_path: None,
            implementor_type: "Foo".into(),
            has_generics: false,
        }],
        ..Default::default()
    };
    let f = make_file_with_rust_meta("src/capabilities_something.rs", meta);
    let mut v = Vec::new();
    checker().check_capability_routing(&f, "capabilities", &mut v);
    assert!(v.is_empty(), "valid metadata composition should pass");
}
