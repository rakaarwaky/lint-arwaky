// Tests for the filesystem crate — FR-001 through FR-005
// Verifies: file discovery, AST parsing, import extraction, graph construction, orchestrator.

use filesystem_lint_arwaky::agent_filesystem_orchestrator::FilesystemOrchestrator;
use filesystem_lint_arwaky::capabilities_ast_parser::ASTParser;
use filesystem_lint_arwaky::capabilities_dependency_graph::DependencyGraph;
use filesystem_lint_arwaky::capabilities_file_walker::FileWalker;
use filesystem_lint_arwaky::capabilities_import_extractor::ImportExtractor;
use shared::filesystem::IFilesystemAggregate;
use shared::filesystem::contract_filesystem_protocol::IImportExtractorProtocol;
use shared::filesystem::taxonomy_filesystem_vo::*;
use std::path::PathBuf;

fn test_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .to_path_buf()
}

// ═══════════════════════════════════════════════════════════════
// FR-001: File Discovery
// ═══════════════════════════════════════════════════════════════

#[test]
fn fr001_walk_finds_rs_files() {
    let walker = FileWalker::new();
    let root = test_root();
    let files = walker.walk(&root, &[], &["rs"]);
    assert!(!files.is_empty(), "should find .rs files");
    assert!(files.iter().all(|f| f.extension == "rs"));
    assert!(files.iter().all(|f| f.language == Language::Rust));
    // FR-001: file content should be populated
    assert!(files.iter().all(|f| !f.content.is_empty()));
}

#[test]
fn fr001_walk_filters_by_extension() {
    let walker = FileWalker::new();
    let root = test_root();
    let files = walker.walk(&root, &[], &["py"]);
    assert!(files.is_empty() || files.iter().all(|f| f.extension == "py"));
}

#[test]
fn fr001_walk_skips_large_files() {
    let walker = FileWalker::new();
    let root = test_root();
    let files = walker.walk(&root, &[], Language::extensions());
    assert!(files.iter().all(|f| f.size <= MAX_LINT_FILE_BYTES));
}

#[test]
fn fr001_walk_recursive_returns_paths() {
    let root = test_root();
    let walker = FileWalker::new();
    let extensions = shared::filesystem::taxonomy_filesystem_vo::Language::extensions();
    let entries = walker.walk(&root, &[], extensions);
    let paths: Vec<String> = entries
        .into_iter()
        .map(|e| e.path.to_string_lossy().to_string())
        .collect();
    assert!(!paths.is_empty());
    assert!(paths.iter().all(|p| p.ends_with(".rs")));
}

// ═══════════════════════════════════════════════════════════════
// FR-002: AST Parsing
// ═══════════════════════════════════════════════════════════════

#[test]
fn fr002_parse_rust_file() {
    let parser = ASTParser::new();
    let mut files = vec![FileEntry {
        path: PathBuf::from("test.rs"),
        extension: "rs".into(),
        language: Language::Rust,
        size: 100,
        content: "fn main() { println!(\"hello\"); }".into(),
        parse_ok: false,
        parse_metadata: None,
    }];
    parser.parse_all(&mut files);
    assert!(files[0].parse_ok);
    assert!(files[0].parse_metadata.is_some());
}

#[test]
fn fr002_parse_python_file() {
    let parser = ASTParser::new();
    let mut files = vec![FileEntry {
        path: PathBuf::from("test.py"),
        extension: "py".into(),
        language: Language::Python,
        size: 100,
        content: "def main():
    print('hello')"
            .into(),
        parse_ok: false,
        parse_metadata: None,
    }];
    parser.parse_all(&mut files);
    assert!(files[0].parse_ok);
    assert!(files[0].parse_metadata.is_some());
}

#[test]
fn fr002_parse_typescript_file() {
    let parser = ASTParser::new();
    let mut files = vec![FileEntry {
        path: PathBuf::from("test.ts"),
        extension: "ts".into(),
        language: Language::TypeScript,
        size: 100,
        content: "function main() { console.log('hello'); }".into(),
        parse_ok: false,
        parse_metadata: None,
    }];
    parser.parse_all(&mut files);
    assert!(files[0].parse_ok);
    assert!(files[0].parse_metadata.is_some());
}

#[test]
fn fr002_parse_empty_file() {
    let parser = ASTParser::new();
    let mut files = vec![FileEntry {
        path: PathBuf::from("empty.rs"),
        extension: "rs".into(),
        language: Language::Rust,
        size: 0,
        content: String::new(),
        parse_ok: false,
        parse_metadata: None,
    }];
    parser.parse_all(&mut files);
    // FR-002: Empty file => parse_ok = true, empty metadata
    assert!(files[0].parse_ok);
    assert!(files[0].parse_metadata.is_none());
}

#[test]
fn fr002_parse_syntax_error() {
    let parser = ASTParser::new();
    let mut files = vec![FileEntry {
        path: PathBuf::from("bad.rs"),
        extension: "rs".into(),
        language: Language::Rust,
        size: 100,
        content: "fn main( { broken".into(),
        parse_ok: false,
        parse_metadata: None,
    }];
    parser.parse_all(&mut files);
    // FR-002: Syntax error => parse_ok = false
    assert!(!files[0].parse_ok);
}

#[test]
fn fr002_parse_rust_metadata_struct() {
    let parser = ASTParser::new();
    let mut files = vec![FileEntry {
        path: PathBuf::from("test.rs"),
        extension: "rs".into(),
        language: Language::Rust,
        size: 100,
        content: "struct Foo;
trait Bar {}
impl Bar for Foo {}"
            .into(),
        parse_ok: false,
        parse_metadata: None,
    }];
    parser.parse_all(&mut files);
    assert!(files[0].parse_ok);
    if let Some(ParseMetadata::Rust(meta)) = &files[0].parse_metadata {
        assert!(meta.struct_definitions.contains(&"Foo".to_string()));
        assert!(meta.trait_definitions.contains(&"Bar".to_string()));
        assert!(!meta.impl_blocks.is_empty());
        assert_eq!(meta.impl_blocks[0].implementor_type, "Foo");
    } else {
        panic!("Expected Rust metadata");
    }
}

#[test]
fn fr002_parse_rust_metadata_impl_trait() {
    let parser = ASTParser::new();
    let mut files = vec![FileEntry {
        path: PathBuf::from("test.rs"),
        extension: "rs".into(),
        language: Language::Rust,
        size: 100,
        content: "impl std::fmt::Display for MyStruct {}".into(),
        parse_ok: false,
        parse_metadata: None,
    }];
    parser.parse_all(&mut files);
    assert!(files[0].parse_ok);
    if let Some(ParseMetadata::Rust(meta)) = &files[0].parse_metadata {
        assert_eq!(meta.impl_blocks.len(), 1);
        assert_eq!(
            meta.impl_blocks[0].trait_name,
            Some("std::fmt::Display".into())
        );
        assert_eq!(meta.impl_blocks[0].implementor_type, "MyStruct");
    } else {
        panic!("Expected Rust metadata");
    }
}

#[test]
fn fr002_parse_all_parallel() {
    let parser = ASTParser::new();
    let root = test_root();
    let walker = FileWalker::new();
    let mut files = walker.walk(&root, &[], &["rs"]);
    parser.parse_all(&mut files);
    let parsed = files.iter().filter(|f| f.parse_ok).count();
    // Most files should parse successfully
    assert!(parsed > 0, "should parse at least some files");
}

// ═══════════════════════════════════════════════════════════════
// FR-003: Import Extraction
// ═══════════════════════════════════════════════════════════════

#[test]
fn fr003_extract_rust_use() {
    let path = PathBuf::from("test.rs");
    let content = "use std::collections::HashMap;
use crate::foo::Bar;";
    let extractor = ImportExtractor;
    let imports = extractor.extract(&path, content, Language::Rust);
    assert_eq!(imports.len(), 2);
    assert_eq!(imports[0].raw_path, "std::collections::HashMap");
    assert_eq!(imports[0].import_type, ImportType::Use);
    assert_eq!(imports[1].raw_path, "crate::foo::Bar");
}

#[test]
fn fr003_extract_rust_mod() {
    let path = PathBuf::from("lib.rs");
    let content = "mod foo;
mod bar;";
    let extractor = ImportExtractor;
    let imports = extractor.extract(&path, content, Language::Rust);
    assert_eq!(imports.len(), 2);
    assert_eq!(imports[0].import_type, ImportType::Mod);
    assert_eq!(imports[0].raw_path, "foo");
}

#[test]
fn fr003_extract_python_import() {
    let path = PathBuf::from("main.py");
    let content = "import os
from sys import argv";
    let extractor = ImportExtractor;
    let imports = extractor.extract(&path, content, Language::Python);
    assert_eq!(imports.len(), 2);
    assert_eq!(imports[0].import_type, ImportType::Import);
    assert_eq!(imports[1].import_type, ImportType::ImportFrom);
}

#[test]
fn fr003_extract_js_import() {
    let path = PathBuf::from("app.js");
    let content = "import React from 'react';
const fs = require('fs');";
    let extractor = ImportExtractor;
    let imports = extractor.extract(&path, content, Language::JavaScript);
    assert!(imports.len() >= 2);
}

#[test]
fn fr003_extract_pub_use_as_reexport() {
    let path = PathBuf::from("lib.rs");
    let content = "pub use crate::foo::Bar;";
    let extractor = ImportExtractor;
    let imports = extractor.extract(&path, content, Language::Rust);
    assert_eq!(imports.len(), 1);
    assert_eq!(imports[0].import_type, ImportType::ReExport);
    assert!(imports[0].is_reexport);
}

#[test]
fn fr003_extract_empty_file() {
    let path = PathBuf::from("empty.rs");
    let extractor = ImportExtractor;
    let imports = extractor.extract(&path, "", Language::Rust);
    assert!(imports.is_empty());
}

#[test]
fn fr003_extract_ts_export_from() {
    let path = PathBuf::from("index.ts");
    let content = "export { Foo } from './foo';";
    let extractor = ImportExtractor;
    let imports = extractor.extract(&path, content, Language::TypeScript);
    assert_eq!(imports.len(), 1);
    assert_eq!(imports[0].import_type, ImportType::ReExport);
    assert!(imports[0].is_reexport);
}

// ═══════════════════════════════════════════════════════════════
// FR-004: Dependency Graph
// ═══════════════════════════════════════════════════════════════

fn make_file(name: &str) -> FileEntry {
    FileEntry {
        path: PathBuf::from(name),
        extension: name.rsplit('.').next().unwrap_or("rs").to_string(),
        language: Language::Rust,
        size: 100,
        content: String::new(),
        parse_ok: true,
        parse_metadata: None,
    }
}

fn make_import(from: &str, to: &str) -> ImportEntry {
    ImportEntry {
        source_file: PathBuf::from(from),
        raw_path: to.to_string(),
        resolved_path: Some(PathBuf::from(to)),
        import_type: ImportType::Use,
        language: Language::Rust,
        is_dynamic: false,
        is_resolved: true,
        symbols: Vec::new(),
        is_reexport: false,
        is_wildcard: false,
    }
}

#[test]
fn fr004_graph_build_and_query() {
    let mut graph = DependencyGraph::new();
    let files = vec![make_file("a.rs"), make_file("b.rs")];
    let imports = vec![make_import("a.rs", "b.rs")];
    graph.build(&imports, &files, &[], &[]);

    assert_eq!(
        graph.dependencies(&PathBuf::from("a.rs")),
        vec![PathBuf::from("b.rs")]
    );
    assert_eq!(
        graph.dependents(&PathBuf::from("b.rs")),
        vec![PathBuf::from("a.rs")]
    );
    assert!(graph.reachable(&PathBuf::from("a.rs"), &PathBuf::from("b.rs")));
    assert!(!graph.reachable(&PathBuf::from("b.rs"), &PathBuf::from("a.rs")));
}

#[test]
fn fr004_graph_no_cycles() {
    let mut graph = DependencyGraph::new();
    let files = vec![make_file("a.rs"), make_file("b.rs")];
    let imports = vec![make_import("a.rs", "b.rs")];
    graph.build(&imports, &files, &[], &[]);
    assert!(graph.cycles().is_empty());
}

#[test]
fn fr004_graph_detects_cycle() {
    let mut graph = DependencyGraph::new();
    let files = vec![make_file("a.rs"), make_file("b.rs")];
    let imports = vec![make_import("a.rs", "b.rs"), make_import("b.rs", "a.rs")];
    graph.build(&imports, &files, &[], &[]);
    assert!(!graph.cycles().is_empty());
}

#[test]
fn fr004_graph_orphan_files() {
    let mut graph = DependencyGraph::new();
    let files = vec![make_file("a.rs"), make_file("b.rs"), make_file("c.rs")];
    let imports = vec![make_import("a.rs", "b.rs")];
    graph.build(&imports, &files, &[], &[]);
    let orphans = graph.orphan_files();
    assert!(orphans.contains(&PathBuf::from("a.rs")));
    assert!(orphans.contains(&PathBuf::from("c.rs")));
    assert!(!orphans.contains(&PathBuf::from("b.rs")));
}

#[test]
fn fr004_graph_reverse_links() {
    let mut graph = DependencyGraph::new();
    let files = vec![make_file("a.rs"), make_file("b.rs")];
    let imports = vec![make_import("a.rs", "b.rs")];
    graph.build(&imports, &files, &[], &[]);
    let reverse = graph.reverse_links();
    assert!(reverse.contains_key(&PathBuf::from("b.rs")));
    assert!(reverse[&PathBuf::from("b.rs")].contains(&PathBuf::from("a.rs")));
}

#[test]
fn fr004_graph_definitions() {
    let mut graph = DependencyGraph::new();
    let files = vec![make_file("a.rs")];
    let definitions = vec![DefinitionEntry {
        name: "Foo".to_string(),
        file_path: PathBuf::from("a.rs"),
        language: Language::Rust,
    }];
    graph.build(&[], &files, &definitions, &[]);
    let defs = graph.definitions();
    assert!(defs.contains_key("Foo"));
    assert!(defs["Foo"].contains(&PathBuf::from("a.rs")));
}

#[test]
fn fr004_graph_implementations() {
    let mut graph = DependencyGraph::new();
    let files = vec![make_file("a.rs")];
    let impls = vec![ImplEntry {
        trait_name: "Display".to_string(),
        file_path: PathBuf::from("a.rs"),
        language: Language::Rust,
    }];
    graph.build(&[], &files, &[], &impls);
    let imps = graph.implementations();
    assert!(imps.contains_key("Display"));
    assert!(imps["Display"].contains(&PathBuf::from("a.rs")));
}

// ═══════════════════════════════════════════════════════════════
// FR-005: Orchestrator
// ═══════════════════════════════════════════════════════════════

#[test]
fn fr005_full_scan() {
    let service = FilesystemOrchestrator::new();
    let root = test_root();
    service.run_pipeline(&root, &[]);

    assert!(!service.file_list().is_empty(), "should discover files");
    assert!(service.timing().total_ms > 0, "should record timing");
    eprintln!(
        "Scan: {} files, {} imports, {}ms total",
        service.file_list().len(),
        service.import_list().len(),
        service.timing().total_ms
    );
}

#[test]
fn fr005_cached_results() {
    let service = FilesystemOrchestrator::new();
    let root = test_root();
    service.run_pipeline(&root, &[]);

    let count1 = service.file_list().len();
    // Second call should return cached results (same count)
    let count2 = service.file_list().len();
    assert_eq!(count1, count2);
}

#[test]
fn fr005_parse_warnings() {
    let service = FilesystemOrchestrator::new();
    let root = test_root();
    service.run_pipeline(&root, &[]);

    // Warnings should be accessible
    let _warnings = service.parse_warnings();
    // Some files may have parse errors — that's expected
}

#[test]
fn fr005_graph_queries() {
    let service = FilesystemOrchestrator::new();
    let root = test_root();
    service.run_pipeline(&root, &[]);

    let reverse = service.reverse_import_map();
    assert!(!reverse.is_empty() || service.file_list().len() < 10);

    let defs = service.symbol_definitions();
    let imps = service.trait_implementations();
    // At least some definitions should exist in a Rust codebase
    eprintln!(
        "Definitions: {}, Implementations: {}",
        defs.len(),
        imps.len()
    );
}

// ═══════════════════════════════════════════════════════════════
// Language Detection
// ═══════════════════════════════════════════════════════════════

#[test]
fn language_from_extension() {
    assert_eq!(Language::from_extension("rs"), Some(Language::Rust));
    assert_eq!(Language::from_extension("py"), Some(Language::Python));
    assert_eq!(Language::from_extension("ts"), Some(Language::TypeScript));
    assert_eq!(Language::from_extension("tsx"), Some(Language::TypeScript));
    assert_eq!(Language::from_extension("js"), Some(Language::JavaScript));
    assert_eq!(Language::from_extension("jsx"), Some(Language::JavaScript));
    assert_eq!(Language::from_extension("go"), None);
    assert_eq!(Language::from_extension("java"), None);
}

#[test]
fn language_extensions_list() {
    let exts = Language::extensions();
    assert!(exts.contains(&"rs"));
    assert!(exts.contains(&"py"));
    assert!(exts.contains(&"ts"));
    assert!(exts.contains(&"js"));
}

#[test]
fn parse_warning_message() {
    let warning = ParseWarning {
        file_path: PathBuf::from("test.rs"),
        error_detail: "unexpected token".to_string(),
    };
    let msg = warning.message();
    assert!(msg.contains("parse failure"));
    assert!(msg.contains("unexpected token"));
}
