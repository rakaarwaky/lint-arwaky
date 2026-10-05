// Unit tests for ASTParser — FR-001: AST Parsing & Import Extraction.
use filesystem_lint_arwaky::capabilities_ast_parser::ASTParser;
use shared_common::taxonomy_language_vo::Language;
use shared_filesystem::contract_filesystem_protocol::IParserProtocol;
use shared_filesystem::taxonomy_filesystem_vo::FileEntry;
use std::path::PathBuf;

fn make_entry(path: &str, content: &str, language: Language) -> FileEntry {
    FileEntry {
        path: PathBuf::from(path),
        extension: match language {
            Language::Rust => "rs",
            Language::Python => "py",
            Language::TypeScript => "ts",
            Language::JavaScript => "js",
            Language::Unknown => "",
        }
        .to_string(),
        language,
        size: content.len() as u64,
        content: content.to_string(),
        parse_ok: false,
        parse_metadata: None,
    }
}

#[test]
fn parse_valid_rust_file_sets_parse_ok() {
    let parser = ASTParser::new();
    let mut files = vec![make_entry(
        "/test.rs",
        "fn main() { println!(\"hello\"); }",
        Language::Rust,
    )];
    parser.parse_all(&mut files);
    assert!(files[0].parse_ok);
    assert!(files[0].parse_metadata.is_some());
}

#[test]
fn parse_rust_file_with_syntax_error_sets_parse_ok_false() {
    let parser = ASTParser::new();
    let mut files = vec![make_entry(
        "/test.rs",
        "fn main( { broken }",
        Language::Rust,
    )];
    parser.parse_all(&mut files);
    assert!(!files[0].parse_ok);
    assert!(!parser.parse_warnings().is_empty());
}

#[test]
fn parse_empty_file_sets_parse_ok_true() {
    let parser = ASTParser::new();
    let mut files = vec![make_entry("/test.rs", "", Language::Rust)];
    parser.parse_all(&mut files);
    assert!(files[0].parse_ok);
}

#[test]
fn parse_python_file_extracts_metadata() {
    let parser = ASTParser::new();
    let mut files = vec![make_entry(
        "/test.py",
        "import os\ndef hello(): pass\nclass Foo: pass\n",
        Language::Python,
    )];
    parser.parse_all(&mut files);
    assert!(files[0].parse_ok);
    assert!(files[0].parse_metadata.is_some());
}

#[test]
fn parse_typescript_file_extracts_metadata() {
    let parser = ASTParser::new();
    let mut files = vec![make_entry(
        "/test.ts",
        "import { foo } from './bar';\nfunction hello() {}\n",
        Language::TypeScript,
    )];
    parser.parse_all(&mut files);
    assert!(files[0].parse_ok);
    assert!(files[0].parse_metadata.is_some());
}

#[test]
fn parse_typescript_captures_enum_declarations() {
    let parser = ASTParser::new();
    let mut files = vec![make_entry(
        "/enum_test.ts",
        "export enum Mode { Fast, Slow }\nenum Bare { One }\nfunction pick(m: Mode): string { return String(m); }\n",
        Language::TypeScript,
    )];
    parser.parse_all(&mut files);
    assert!(files[0].parse_ok);
    let meta = match &files[0].parse_metadata {
        Some(shared_filesystem::taxonomy_filesystem_vo::ParseMetadata::TypeScript(m)) => m,
        other => panic!("expected TypeScript metadata, got: {other:?}"),
    };
    assert!(
        meta.enum_declarations.contains(&"Mode".to_string()),
        "exported enum should be captured, got: {:?}",
        meta.enum_declarations
    );
    assert!(
        meta.enum_declarations.contains(&"Bare".to_string()),
        "non-exported enum should be captured, got: {:?}",
        meta.enum_declarations
    );
}

#[test]
fn parse_unknown_language_sets_parse_ok_false() {
    let parser = ASTParser::new();
    let mut files = vec![make_entry("/test.xyz", "some content", Language::Unknown)];
    parser.parse_all(&mut files);
    assert!(!files[0].parse_ok);
}

#[test]
fn parse_all_collects_imports() {
    let parser = ASTParser::new();
    let mut files = vec![make_entry(
        "/test.rs",
        "use std::collections::HashMap;\nuse crate::foo::Bar;\n",
        Language::Rust,
    )];
    parser.parse_all(&mut files);
    let imports = parser.import_list();
    assert!(!imports.is_empty(), "Expected at least 1 import");
}

#[test]
fn imports_for_returns_only_matching_file() {
    let parser = ASTParser::new();
    let mut files = vec![
        make_entry("/a.rs", "use std::collections::HashMap;\n", Language::Rust),
        make_entry("/b.rs", "use std::io::Read;\n", Language::Rust),
    ];
    parser.parse_all(&mut files);
    let imports_a = parser.imports_for(&PathBuf::from("/a.rs"));
    let imports_b = parser.imports_for(&PathBuf::from("/b.rs"));
    for import in &imports_a {
        assert_eq!(import.source_file, PathBuf::from("/a.rs"));
    }
    for import in &imports_b {
        assert_eq!(import.source_file, PathBuf::from("/b.rs"));
    }
}

#[test]
fn extract_returns_imports_for_snippet() {
    let parser = ASTParser::new();
    let imports = parser.extract(
        &PathBuf::from("/test.rs"),
        "use std::fs;\nuse crate::module::Item;\n",
        Language::Rust,
    );
    assert!(!imports.is_empty(), "Expected imports from extract");
}

#[test]
fn parse_warnings_empty_when_all_files_parse_ok() {
    let parser = ASTParser::new();
    let mut files = vec![make_entry("/test.rs", "fn main() {}", Language::Rust)];
    parser.parse_all(&mut files);
    assert!(parser.parse_warnings().is_empty());
}

#[test]
fn parse_parallel_multiple_files() {
    let parser = ASTParser::new();
    let mut files: Vec<FileEntry> = (0..50)
        .map(|i| {
            make_entry(
                &format!("/file_{}.rs", i),
                &format!("fn func_{}() {{}}", i),
                Language::Rust,
            )
        })
        .collect();
    parser.parse_all(&mut files);
    for entry in &files {
        assert!(
            entry.parse_ok,
            "File {} should parse OK",
            entry.path.display()
        );
    }
}

// ── Migrated from inline #[cfg(test)] in utility_ast_rust.rs ──

#[test]
fn test_extract_rust_metadata_used_identifiers() {
    use shared_filesystem::utility_ast_rust::extract_rust_metadata;

    let content = r#"
use taxonomy::vo::UserVO;
use contract::protocol::ContractProtocol;

pub fn process() -> UserVO {
    let proto = ContractProtocol::new();
    proto.validate();
    UserVO::new()
}
"#;
    let mut parser = tree_sitter::Parser::new();
    parser
        .set_language(&tree_sitter_rust::LANGUAGE.into())
        .unwrap();
    let tree = parser.parse(content, None).unwrap();
    let meta = extract_rust_metadata(&tree, content);
    eprintln!("used_identifiers: {:?}", meta.used_identifiers);
    assert!(
        meta.used_identifiers.contains(&"UserVO".to_string()),
        "UserVO should be in used_identifiers"
    );
    assert!(
        meta.used_identifiers
            .contains(&"ContractProtocol".to_string()),
        "ContractProtocol should be in used_identifiers"
    );
}

#[test]
fn ast_cache_evicts_old_entries_at_capacity() {
    let parser = ASTParser::with_cache_capacity(2);
    let mut files = vec![
        make_entry("first.rs", "fn first() {}", Language::Rust),
        make_entry("second.rs", "fn second() {}", Language::Rust),
        make_entry("third.rs", "fn third() {}", Language::Rust),
    ];
    parser.parse_all(&mut files);
    assert!(parser.cached_ast_count() <= 2);
}

// ── impl trait-name extraction ─────────────────────────────
//
// The trait name is what every downstream consumer keys on — AES405's
// protocol check, orphan reachability, and the implementation graph. Reading up
// to the last `>` in the impl head mis-reads a parameterized trait
// (`impl IFoo<u32> for Bar`) as `u32`, silently dropping the implementation, so
// each head shape is pinned here.

fn rust_impl_trait(content: &str) -> Option<String> {
    use shared_filesystem::utility_ast_rust::extract_rust_metadata;
    let mut parser = tree_sitter::Parser::new();
    parser
        .set_language(&tree_sitter_rust::LANGUAGE.into())
        .unwrap();
    let tree = parser.parse(content, None).unwrap();
    let meta = extract_rust_metadata(&tree, content);
    meta.impl_blocks
        .into_iter()
        .next()
        .and_then(|b| b.trait_name)
}

#[test]
fn plain_impl_trait_name() {
    assert_eq!(
        rust_impl_trait("impl Foo for Bar {\n}\n").as_deref(),
        Some("Foo")
    );
}

#[test]
fn generic_impl_keeps_trait_name() {
    assert_eq!(
        rust_impl_trait("impl<T> Foo for Bar {\n}\n").as_deref(),
        Some("Foo")
    );
}

#[test]
fn lifetime_generic_impl_keeps_trait_name() {
    assert_eq!(
        rust_impl_trait("impl<'a, T> Foo for Bar {\n}\n").as_deref(),
        Some("Foo")
    );
}

#[test]
fn parameterized_trait_keeps_its_own_name() {
    // The regression: reading up to the last `>` yields `u32` here.
    assert_eq!(
        rust_impl_trait("impl Foo<u32> for Bar {\n}\n").as_deref(),
        Some("Foo")
    );
}

#[test]
fn path_qualified_trait_keeps_full_path() {
    assert_eq!(
        rust_impl_trait("impl crate::Foo for Bar {\n}\n").as_deref(),
        Some("crate::Foo")
    );
}

#[test]
fn inherent_impl_has_no_trait_name() {
    assert_eq!(rust_impl_trait("impl Bar {\n}\n"), None);
}

// ─── Implementor type ──────────────────────────────────────────────────────
//
// The implementor is what AES404 reports and what the implementation graph keys
// on. Reading it from the whole impl block mislabels it: for
// `impl RuleRow { fn into_rule(self) -> ArchitectureRule { .. } }` the first `>`
// sits in the body's return arrow, so a full-block scan yields
// "ArchitectureRow"… "ArchitectureRule" instead of "RuleRow".

fn rust_impl_implementor(content: &str) -> Option<String> {
    use shared_filesystem::utility_ast_rust::extract_rust_metadata;
    let mut parser = tree_sitter::Parser::new();
    parser
        .set_language(&tree_sitter_rust::LANGUAGE.into())
        .unwrap();
    let tree = parser.parse(content, None).unwrap();
    let meta = extract_rust_metadata(&tree, content);
    meta.impl_blocks
        .into_iter()
        .next()
        .map(|b| b.implementor_type)
}

#[test]
fn plain_impl_implementor_name() {
    assert_eq!(
        rust_impl_implementor("impl Foo for Bar {\n}\n").as_deref(),
        Some("Bar")
    );
}

#[test]
fn inherent_impl_implementor_name() {
    assert_eq!(
        rust_impl_implementor("impl Bar {\n}\n").as_deref(),
        Some("Bar")
    );
}

#[test]
fn inherent_impl_with_arrow_in_body_keeps_its_own_name() {
    // The regression: the body's `-> ArchitectureRule` must not become the
    // implementor.
    let content =
        "impl RuleRow {\n    fn into_rule(self) -> ArchitectureRule {\n        todo!()\n    }\n}\n";
    assert_eq!(rust_impl_implementor(content).as_deref(), Some("RuleRow"));
}

#[test]
fn inherent_impl_with_generic_body_keeps_its_own_name() {
    let content =
        "impl RuleRow {\n    fn build(&self) -> Vec<String> {\n        Vec::new()\n    }\n}\n";
    assert_eq!(rust_impl_implementor(content).as_deref(), Some("RuleRow"));
}

#[test]
fn generic_impl_implementor_drops_its_own_generics() {
    assert_eq!(
        rust_impl_implementor("impl<T> Foo for Bar<T> {\n}\n").as_deref(),
        Some("Bar")
    );
}

#[test]
fn path_qualified_implementor_keeps_full_path() {
    assert_eq!(
        rust_impl_implementor("impl crate::Foo for crate::Bar {\n}\n").as_deref(),
        Some("crate::Bar")
    );
}
