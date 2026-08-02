// FR-001 — AST Parsing & Import Extraction
// US1: Valid Rust file produces parse_ok=true with structured metadata.
// US2: File with syntax error produces parse_ok=false with warning.
// US3: Empty file produces parse_ok=true with no metadata.
// US4: Import extraction resolves use statements.
// US5: Wildcard imports are detected.
// US6: External dependencies are excluded.
// US7: Parallel parsing completes for many files.

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
            _ => "",
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
fn us1_valid_rust_file_produces_metadata() {
    let parser = ASTParser::new();
    let mut files = vec![make_entry(
        "/project/src/main.rs",
        "use std::collections::HashMap;\npub fn main() { let _ = HashMap::new(); }\n",
        Language::Rust,
    )];
    parser.parse_all(&mut files);
    assert!(files[0].parse_ok);
    assert!(files[0].parse_metadata.is_some());
    match files[0].parse_metadata.as_ref().unwrap() {
        shared::filesystem::taxonomy_filesystem_vo::ParseMetadata::Rust(meta) => {
            assert!(!meta.function_definitions.is_empty());
        }
        _ => panic!("Expected Rust metadata"),
    }
}

#[test]
fn us2_syntax_error_produces_warning() {
    let parser = ASTParser::new();
    let mut files = vec![make_entry(
        "/project/src/broken.rs",
        "fn broken( { invalid",
        Language::Rust,
    )];
    parser.parse_all(&mut files);
    assert!(!files[0].parse_ok);
    assert!(!parser.parse_warnings().is_empty());
    assert!(parser.parse_warnings()[0]
        .error_detail
        .contains("errors"));
}

#[test]
fn us3_empty_file_parses_ok() {
    let parser = ASTParser::new();
    let mut files = vec![make_entry("/project/src/empty.rs", "", Language::Rust)];
    parser.parse_all(&mut files);
    assert!(files[0].parse_ok);
}

#[test]
fn us4_use_statements_extracted() {
    let parser = ASTParser::new();
    let mut files = vec![make_entry(
        "/project/src/lib.rs",
        "use std::collections::HashMap;\nuse crate::module::Item;\npub fn foo() {}\n",
        Language::Rust,
    )];
    parser.parse_all(&mut files);
    let imports = parser.import_list();
    assert!(
        imports.iter().any(|i| i.raw_path.contains("std::collections")),
        "Should extract std::collections import"
    );
}

#[test]
fn us5_wildcard_import_detected() {
    let parser = ASTParser::new();
    let imports = parser.extract(
        &PathBuf::from("/test.rs"),
        "use crate::module::*;\npub fn foo() {}\n",
        Language::Rust,
    );
    assert!(
        imports.iter().any(|i| i.is_wildcard),
        "Should detect wildcard import, got: {:?}",
        imports.iter().map(|i| (&i.raw_path, i.is_wildcard)).collect::<Vec<_>>()
    );
}

#[test]
fn us6_external_dependencies_not_in_workspace() {
    let parser = ASTParser::new();
    let imports = parser.extract(
        &PathBuf::from("/project/src/main.rs"),
        "use std::io::Read;\nuse serde::Deserialize;\n",
        Language::Rust,
    );
    // External deps should be extracted but not resolved to workspace files
    for import in &imports {
        assert!(
            !import.is_resolved || import.raw_path.starts_with("crate"),
            "External dep should not be resolved: {}",
            import.raw_path
        );
    }
}

#[test]
fn us7_parallel_parsing_handles_hundred_files() {
    let parser = ASTParser::new();
    let mut files: Vec<FileEntry> = (0..200)
        .map(|i| {
            make_entry(
                &format!("/project/src/file_{}.rs", i),
                &format!("pub fn func_() {{ let x = {}; }}", i),
                Language::Rust,
            )
        })
        .collect();
    parser.parse_all(&mut files);
    let ok_count = files.iter().filter(|f| f.parse_ok).count();
    assert_eq!(ok_count, 200, "All 200 files should parse successfully");
}

#[test]
fn fr001_python_file_parse_metadata() {
    let parser = ASTParser::new();
    let mut files = vec![make_entry(
        "/project/module.py",
        "import os\nclass MyClass: pass\ndef my_func(): pass\n",
        Language::Python,
    )];
    parser.parse_all(&mut files);
    assert!(files[0].parse_ok);
    match files[0].parse_metadata.as_ref().unwrap() {
        shared::filesystem::taxonomy_filesystem_vo::ParseMetadata::Python(meta) => {
            assert!(!meta.class_declarations.is_empty());
            assert!(!meta.function_definitions.is_empty());
        }
        _ => panic!("Expected Python metadata"),
    }
}

#[test]
fn fr001_typescript_file_parse_metadata() {
    let parser = ASTParser::new();
    let mut files = vec![make_entry(
        "/project/index.ts",
        "import { foo } from './bar';\nexport interface MyInterface {}\nfunction hello() {}\n",
        Language::TypeScript,
    )];
    parser.parse_all(&mut files);
    assert!(files[0].parse_ok);
    match files[0].parse_metadata.as_ref().unwrap() {
        shared::filesystem::taxonomy_filesystem_vo::ParseMetadata::TypeScript(meta) => {
            assert!(!meta.import_statements.is_empty());
        }
        _ => panic!("Expected TypeScript metadata"),
    }
}
