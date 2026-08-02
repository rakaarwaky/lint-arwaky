// Unit tests for ASTParser — FR-001: AST Parsing & Import Extraction.
use filesystem_lint_arwaky::capabilities_ast_parser::ASTParser;
use shared::common::taxonomy_language_vo::Language;
use shared::filesystem::contract_parser_protocol::IParserProtocol;
use shared::filesystem::taxonomy_filesystem_vo::FileEntry;
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
