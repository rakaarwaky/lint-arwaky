// Acceptance test AES404 — Utility purity.
// Utility files must not define structs, enums, classes, impl blocks, or type aliases.
use role_rules_lint_arwaky::root_role_rules_container::RoleContainer;
use shared::config_system::taxonomy_config_vo::ArchitectureConfig;
use shared::filesystem::taxonomy_filesystem_vo::{
    FileEntry, Language, ParseMetadata, RustMetadata,
};
use shared::role_rules::taxonomy_role_request::RoleRequest;
use std::path::PathBuf;

fn make_file(path: &str, lang: Language, content: &str) -> FileEntry {
    FileEntry {
        path: PathBuf::from(path),
        extension: match lang {
            Language::Rust => "rs",
            Language::Python => "py",
            Language::TypeScript | Language::JavaScript => "ts",
            _ => "txt",
        }
        .to_string(),
        language: lang,
        size: content.len() as u64,
        content: content.to_string(),
        parse_ok: true,
        parse_metadata: None,
    }
}

/// Build a utility entry carrying explicit Rust AST metadata, so each Rust
/// forbidden item is covered without depending on what tree-sitter reports.
///
/// `content` is a non-empty placeholder because the orchestrator skips entries
/// whose body is empty; the auditor reads `parse_metadata`, never the source.
fn make_file_with_rust_meta(path: &str, meta: RustMetadata) -> FileEntry {
    FileEntry {
        path: PathBuf::from(path),
        extension: "rs".to_string(),
        language: Language::Rust,
        size: 1,
        content: "fn placeholder() {}".to_string(),
        parse_ok: true,
        parse_metadata: Some(ParseMetadata::Rust(meta)),
    }
}

fn run_audit(files: Vec<FileEntry>) -> Vec<shared::common::LintResult> {
    let config = ArchitectureConfig::default();
    let container = RoleContainer::new_with_config(config);
    let orch = container.orchestrator();
    orch.execute(RoleRequest::audit(&files)).into_violations()
}

// ── Rust utility with struct → AES404 ──

#[test]
fn aes404_rust_utility_with_struct_detected() {
    let file = make_file(
        "src/utility_helpers.rs",
        Language::Rust,
        "pub struct HelperState {\n    count: i32,\n}\n",
    );
    let results = run_audit(vec![file]);
    let aes404: Vec<_> = results
        .iter()
        .filter(|r| r.code.code() == "AES404")
        .collect();
    assert!(
        !aes404.is_empty(),
        "utility with struct should trigger AES404"
    );
}

// ── Rust utility with enum → AES404 ──

#[test]
fn aes404_rust_utility_with_enum_detected() {
    let file = make_file(
        "src/utility_formatters.rs",
        Language::Rust,
        "pub enum Format {\n    Json,\n    Csv,\n}\n",
    );
    let results = run_audit(vec![file]);
    let aes404: Vec<_> = results
        .iter()
        .filter(|r| r.code.code() == "AES404")
        .collect();
    assert!(
        !aes404.is_empty(),
        "utility with enum should trigger AES404"
    );
}

// ── Rust utility with trait definition → AES404 ──

#[test]
fn aes404_rust_utility_with_trait_detected() {
    let file = make_file(
        "src/utility_aes404_trait.rs",
        Language::Rust,
        "pub trait Helper {\n    fn run(&self);\n}\n",
    );
    let results = run_audit(vec![file]);
    let aes404: Vec<_> = results
        .iter()
        .filter(|r| r.code.code() == "AES404")
        .collect();
    assert!(
        !aes404.is_empty(),
        "utility with trait definition should trigger AES404"
    );
}

// ── Rust utility with impl block → AES404 ──
//
// The source contains an inherent impl only — no top-level struct. We verify
// the fallback line-scanner still catches the impl keyword so the acceptance
// test works without a parser. The metadata-path version is covered by
// aes404_metadata_trait_impl_reported / aes404_metadata_inherent_impl_reported.

#[test]
fn aes404_rust_utility_with_impl_block_detected() {
    let file = make_file(
        "src/utility_aes404_impl_block.rs",
        Language::Rust,
        "struct Helper;\n\nimpl Helper {\n    pub fn run(&self) {}\n}\n",
    );
    let results = run_audit(vec![file]);
    let aes404: Vec<_> = results
        .iter()
        .filter(|r| r.code.code() == "AES404")
        .collect();
    assert!(
        !aes404.is_empty(),
        "utility with impl block should trigger AES404"
    );
}

// ── Rust utility with type alias → AES404 ──
//
// Fallback path check. Metadata-path coverage is in aes404_metadata_type_alias_reported.

#[test]
fn aes404_rust_utility_with_type_alias_detected() {
    let file = make_file(
        "src/utility_aes404_type_alias.rs",
        Language::Rust,
        "pub type Helper = String;\n",
    );
    let results = run_audit(vec![file]);
    let aes404: Vec<_> = results
        .iter()
        .filter(|r| r.code.code() == "AES404")
        .collect();
    assert!(
        !aes404.is_empty(),
        "utility with type alias should trigger AES404"
    );
}

// ── Metadata path: trait impl reported with trait and implementor ──

#[test]
fn aes404_metadata_trait_impl_reported() {
    let meta = RustMetadata {
        impl_blocks: vec![shared::filesystem::taxonomy_filesystem_vo::RustImplItem {
            trait_name: Some("IHelperProtocol".into()),
            trait_path: Some("shared::IHelperProtocol".into()),
            implementor_type: "Helper".into(),
            has_generics: false,
        }],
        ..Default::default()
    };
    let results = run_audit(vec![make_file_with_rust_meta(
        "src/utility_aes404_meta_impl.rs",
        meta,
    )]);
    let aes404: Vec<_> = results
        .iter()
        .filter(|r| r.code.code() == "AES404")
        .collect();
    assert_eq!(
        aes404.len(),
        1,
        "trait impl block in metadata should produce exactly one AES404"
    );
    let msg = aes404[0].message.value();
    assert!(
        msg.contains("IHelperProtocol") && msg.contains("Helper"),
        "message should name the trait and implementor, got: {msg}"
    );
}

// ── Metadata path: inherent impl reported ──

#[test]
fn aes404_metadata_inherent_impl_reported() {
    let meta = RustMetadata {
        impl_blocks: vec![shared::filesystem::taxonomy_filesystem_vo::RustImplItem {
            trait_name: None,
            trait_path: None,
            implementor_type: "Helper".into(),
            has_generics: false,
        }],
        ..Default::default()
    };
    let results = run_audit(vec![make_file_with_rust_meta(
        "src/utility_aes404_meta_inherent.rs",
        meta,
    )]);
    let aes404: Vec<_> = results
        .iter()
        .filter(|r| r.code.code() == "AES404")
        .collect();
    assert_eq!(
        aes404.len(),
        1,
        "inherent impl block in metadata should produce exactly one AES404"
    );
}

// ── Metadata path: type alias reported ──

#[test]
fn aes404_metadata_type_alias_reported() {
    let meta = RustMetadata {
        type_definitions: vec!["Helper".into()],
        ..Default::default()
    };
    let results = run_audit(vec![make_file_with_rust_meta(
        "src/utility_aes404_meta_type.rs",
        meta,
    )]);
    let aes404: Vec<_> = results
        .iter()
        .filter(|r| r.code.code() == "AES404")
        .collect();
    assert_eq!(
        aes404.len(),
        1,
        "type alias in metadata should produce exactly one AES404"
    );
    assert!(
        aes404[0].message.value().contains("type alias 'Helper'"),
        "message should label the item as a type alias, got: {}",
        aes404[0].message.value()
    );
}

// ── Metadata path: clean metadata produces no violation ──

#[test]
fn aes404_metadata_clean_no_violation() {
    let meta = RustMetadata {
        function_definitions: vec![shared::filesystem::taxonomy_filesystem_vo::RustFnItem {
            name: "helper".into(),
            has_body: true,
        }],
        ..Default::default()
    };
    let results = run_audit(vec![make_file_with_rust_meta(
        "src/utility_aes404_meta_clean.rs",
        meta,
    )]);
    let aes404: Vec<_> = results
        .iter()
        .filter(|r| r.code.code() == "AES404")
        .collect();
    assert!(
        aes404.is_empty(),
        "utility metadata with only functions should not trigger AES404"
    );
}

// ── Rust utility with macro containing impl → no violation ──
//
// A `macro_rules!` body is token soup, not a top-level item. The metadata
// path must not report it, and the fallback path must skip macro bodies.

#[test]
fn aes404_rust_utility_with_macro_body_impl_not_flagged() {
    let file = make_file(
        "src/utility_aes404_macro_builder.rs",
        Language::Rust,
        "#[macro_export]\nmacro_rules! build_helper {\n    ($name:ident) => {\n        pub struct $name { value: String }\n        impl std::fmt::Display for $name {\n            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {\n                write!(f, \"{}\", self.value)\n            }\n        }\n    };\n}\n",
    );
    let results = run_audit(vec![file]);
    let aes404: Vec<_> = results
        .iter()
        .filter(|r| r.code.code() == "AES404")
        .collect();
    assert!(
        aes404.is_empty(),
        "macro body impl should not trigger AES404, got: {aes404:?}"
    );
}

// ── Clean Rust utility (pure functions) → no violation ──

#[test]
fn aes404_clean_utility_no_violation() {
    let file = make_file(
        "src/utility_helpers.rs",
        Language::Rust,
        "pub fn add(a: i32, b: i32) -> i32 { a + b }\npub fn mul(a: i32, b: i32) -> i32 { a * b }\n",
    );
    let results = run_audit(vec![file]);
    let aes404: Vec<_> = results
        .iter()
        .filter(|r| r.code.code() == "AES404")
        .collect();
    assert!(
        aes404.is_empty(),
        "clean utility with pure functions should not trigger AES404"
    );
}

// ── Python utility with class → AES404 ──

#[test]
fn aes404_python_utility_with_class_detected() {
    let file = make_file(
        "src/utility_converters.py",
        Language::Python,
        "class Converter:\n    pass\n",
    );
    let results = run_audit(vec![file]);
    let aes404: Vec<_> = results
        .iter()
        .filter(|r| r.code.code() == "AES404")
        .collect();
    assert!(
        !aes404.is_empty(),
        "python utility with class should trigger AES404"
    );
}

// ── Python utility with only module-level functions → no violation ──
//
// Regression guard for the fallback path: `def` at module level is the
// expected shape of a utility file and must never be reported.

#[test]
fn aes404_python_utility_with_only_functions_not_flagged() {
    let file = make_file(
        "src/utility_aes404_pure_functions.py",
        Language::Python,
        "def normalize(value: str) -> str:\n    return value.strip()\n\n\ndef join_all(values: list[str]) -> str:\n    return \",\".join(values)\n",
    );
    let results = run_audit(vec![file]);
    let aes404: Vec<_> = results
        .iter()
        .filter(|r| r.code.code() == "AES404")
        .collect();
    assert!(
        aes404.is_empty(),
        "python utility with only module-level functions should not trigger AES404, got: {aes404:?}"
    );
}

// ── TypeScript utility with class → AES404 ──

#[test]
fn aes404_typescript_utility_with_class_detected() {
    let file = make_file(
        "src/utility_formatters.ts",
        Language::TypeScript,
        "export class Formatter {\n    format(s: string): string { return s; }\n}\n",
    );
    let results = run_audit(vec![file]);
    let aes404: Vec<_> = results
        .iter()
        .filter(|r| r.code.code() == "AES404")
        .collect();
    assert!(
        !aes404.is_empty(),
        "typescript utility with class should trigger AES404"
    );
}

// ── Non-utility file is not checked ──

#[test]
fn aes404_non_utility_file_ignored() {
    let file = make_file(
        "src/capabilities_feature.rs",
        Language::Rust,
        "pub struct Feature {}\nimpl IFeature for Feature {}\n",
    );
    let results = run_audit(vec![file]);
    let aes404: Vec<_> = results
        .iter()
        .filter(|r| r.code.code() == "AES404")
        .collect();
    assert!(
        aes404.is_empty(),
        "non-utility file should not trigger AES404"
    );
}
