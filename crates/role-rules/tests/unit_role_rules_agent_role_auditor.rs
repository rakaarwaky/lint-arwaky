// Unit tests for AgentRoleChecker — agent-layer role audit (AES405).
use role_rules_lint_arwaky::capabilities_agent_role_auditor::AgentRoleChecker;
use shared::common::Severity;
use shared::filesystem::taxonomy_filesystem_vo::FileEntry;
use shared::role_rules::IAgentRoleChecker;

use shared::filesystem::taxonomy_filesystem_vo::{Language, ParseMetadata, RustMetadata};
use std::path::PathBuf;

fn checker() -> AgentRoleChecker {
    AgentRoleChecker::new()
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
fn non_agent_layer_skipped() {
    let f = make_file(
        "src/capabilities_foo.rs",
        Language::Rust,
        "pub struct Foo {}",
    );
    let mut v = Vec::new();
    checker().check_agent_routing(&f, "capabilities", &mut v);
    assert!(v.is_empty(), "non-agent layer must not produce violations");
}

#[test]
fn agent_layer_with_agent_prefix_routes() {
    // "agent(capabilities)" starts with "agent(" so it IS routed to the agent checker
    let f = make_file(
        "src/capabilities_foo.rs",
        Language::Rust,
        "pub struct Foo {}",
    );
    let mut v = Vec::new();
    checker().check_agent_routing(&f, "agent(capabilities)", &mut v);
    // Single struct without impl — should be flagged
    assert!(
        !v.is_empty(),
        "agent(capabilities) should route to agent checker"
    );
}

#[test]
fn fallback_rust_no_implementor_flagged() {
    let content = "pub struct Foo {}\npub struct Bar {}\n";
    let f = make_file("src/agent_something.rs", Language::Rust, content);
    let mut v = Vec::new();
    checker().check_agent_routing(&f, "agent", &mut v);
    assert!(!v.is_empty(), "should flag missing implementor");
    assert_eq!(v[0].code.code(), "AES405");
}

#[test]
fn fallback_rust_too_many_types_flagged() {
    let content = "pub struct A {}\npub struct B {}\npub struct C {}\npub struct D {}\n";
    let f = make_file("src/agent_something.rs", Language::Rust, content);
    let mut v = Vec::new();
    checker().check_agent_routing(&f, "agent", &mut v);
    assert!(!v.is_empty(), "should flag too many types");
    assert_eq!(v[0].code.code(), "AES405");
    // Should be HIGH severity for too-many-types
    assert_eq!(v[0].severity, Severity::HIGH);
}

#[test]
fn fallback_rust_valid_composition_no_violation() {
    let content = "pub struct Foo {}\nimpl IFooAggregate for Foo {}\n";
    let f = make_file("src/agent_something.rs", Language::Rust, content);
    let mut v = Vec::new();
    checker().check_agent_routing(&f, "agent", &mut v);
    assert!(v.is_empty(), "valid agent composition should pass");
}

#[test]
fn metadata_rust_no_implementor_flagged() {
    let meta = RustMetadata {
        struct_definitions: vec!["Foo".into(), "Bar".into()],
        ..Default::default()
    };
    let f = make_file_with_rust_meta("src/agent_something.rs", meta);
    let mut v = Vec::new();
    checker().check_agent_routing(&f, "agent", &mut v);
    assert!(
        !v.is_empty(),
        "should flag missing implementor via metadata"
    );
    assert_eq!(v[0].code.code(), "AES405");
}

#[test]
fn metadata_rust_too_many_types_flagged() {
    let meta = RustMetadata {
        struct_definitions: vec!["A".into(), "B".into()],
        enum_definitions: vec!["C".into(), "D".into()],
        ..Default::default()
    };
    let f = make_file_with_rust_meta("src/agent_something.rs", meta);
    let mut v = Vec::new();
    checker().check_agent_routing(&f, "agent", &mut v);
    assert!(!v.is_empty(), "should flag too many types via metadata");
    assert_eq!(v[0].severity, Severity::HIGH);
}

#[test]
fn metadata_rust_valid_composition_no_violation() {
    let meta = RustMetadata {
        struct_definitions: vec!["Foo".into()],
        impl_blocks: vec![shared::filesystem::taxonomy_filesystem_vo::RustImplItem {
            trait_name: Some("IFooAggregate".into()),
            trait_path: None,
            implementor_type: "Foo".into(),
            has_generics: false,
        }],
        ..Default::default()
    };
    let f = make_file_with_rust_meta("src/agent_something.rs", meta);
    let mut v = Vec::new();
    checker().check_agent_routing(&f, "agent", &mut v);
    assert!(v.is_empty(), "valid metadata composition should pass");
}
