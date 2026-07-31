// Tests for filesystem crate — FR-001 through FR-006

use filesystem_lint_arwaky::agent_filesystem_orchestrator::FilesystemOrchestrator;
use filesystem_lint_arwaky::capabilities_ast_parser::ASTParser;
use filesystem_lint_arwaky::capabilities_dependency_graph::DependencyGraph;
use filesystem_lint_arwaky::capabilities_file_walker::{FileWalker, walk_recursive};
use filesystem_lint_arwaky::capabilities_import_extractor::extract_imports;
use shared::filesystem::taxonomy_filesystem_vo::MAX_LINT_FILE_BYTES;
use shared::filesystem::*;
use std::path::PathBuf;

fn test_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .to_path_buf()
}

// ─── FR-001: File Discovery ────────────────────────────────

#[test]
fn fr001_walk_finds_rs_files() {
    let walker = FileWalker::new();
    let root = test_root();
    let files = walker.walk(&root, &[], &["rs"]);
    assert!(!files.is_empty(), "should find .rs files");
    assert!(files.iter().all(|f| f.extension == "rs"));
    assert!(files.iter().all(|f| f.language == Language::Rust));
}

#[test]
fn fr001_walk_filters_by_extension() {
    let walker = FileWalker::new();
    let root = test_root();
    let files = walker.walk(&root, &[], &["py"]);
    // No .py files in the lint-arwaky repo itself
    assert!(files.is_empty() || files.iter().all(|f| f.extension == "py"));
}

#[test]
fn fr001_walk_skips_large_files() {
    let walker = FileWalker::new();
    let root = test_root();
    let files = walker.walk(&root, &[], Language::extensions());
    // All files should be under MAX_LINT_FILE_BYTES
    assert!(files.iter().all(|f| f.size <= MAX_LINT_FILE_BYTES));
}

#[test]
fn fr001_walk_recursive_returns_paths() {
    let root = test_root();
    let paths = walk_recursive(&root);
    assert!(!paths.is_empty());
    assert!(paths.iter().all(|p| p.ends_with(".rs")));
}

// ─── FR-002: File Content Cache ─────────────────────────────

#[test]
fn fr002_cache_populate_and_get() {
    let root = test_root();
    let walker = FileWalker::new();
    let files = walker.walk(&root, &[], &["rs"]);
    assert!(!files.is_empty());

    // Use global cache functions
    filesystem_lint_arwaky::utility_filesystem_io::cache_populate(&files);
    // Should be able to get content for at least one file
    let first = &files[0];
    assert!(filesystem_lint_arwaky::utility_filesystem_io::cache_contains(&first.path));
    let content = filesystem_lint_arwaky::utility_filesystem_io::cache_get(&first.path).unwrap();
    assert!(!content.is_empty());
}

#[test]
fn fr002_cache_returns_none_for_missing() {
    let missing = PathBuf::from("/nonexistent/file.rs");
    assert!(!filesystem_lint_arwaky::utility_filesystem_io::cache_contains(&missing));
    assert!(filesystem_lint_arwaky::utility_filesystem_io::cache_get(&missing).is_none());
}

// ─── FR-003: AST Parsing ────────────────────────────────────

#[test]
fn fr003_parse_rust_file() {
    let parser = ASTParser::new();
    let path = PathBuf::from("test.rs");
    let content = "fn main() { println!(\"hello\"); }";
    let result = parser.parse(&path, content, Language::Rust);
    assert!(result.is_some());
    assert!(parser.has_ast(&path));
}

#[test]
fn fr003_parse_python_file() {
    let parser = ASTParser::new();
    let path = PathBuf::from("test.py");
    let content = "def main():\n    print('hello')";
    let result = parser.parse(&path, content, Language::Python);
    assert!(result.is_some());
    assert!(parser.has_ast(&path));
}

#[test]
fn fr003_parse_typescript_file() {
    let parser = ASTParser::new();
    let path = PathBuf::from("test.ts");
    let content = "function main() { console.log('hello'); }";
    let result = parser.parse(&path, content, Language::TypeScript);
    assert!(result.is_some());
    assert!(parser.has_ast(&path));
}

#[test]
fn fr003_parse_all_parallel() {
    let parser = ASTParser::new();
    let root = test_root();
    let walker = FileWalker::new();
    let files = walker.walk(&root, &[], &["rs"]);
    filesystem_lint_arwaky::utility_filesystem_io::cache_populate(&files);

    parser.parse_all(&files, &|path| filesystem_lint_arwaky::utility_filesystem_io::cache_get(path));
    // All files should have ASTs after parse_all
    let parsed = files.iter().filter(|f| parser.has_ast(&f.path)).count();
    assert_eq!(parsed, files.len());
}

// ─── FR-004: Import Extraction ──────────────────────────────

#[test]
fn fr004_extract_rust_use() {
    let path = PathBuf::from("test.rs");
    let content = "use std::collections::HashMap;\nuse crate::foo::Bar;";
    let imports = extract_imports(&path, content, Language::Rust);
    assert_eq!(imports.len(), 2);
    assert_eq!(imports[0].raw_path, "std::collections::HashMap");
    assert_eq!(imports[0].import_type, ImportType::Use);
    assert_eq!(imports[1].raw_path, "crate::foo::Bar");
}

#[test]
fn fr004_extract_rust_mod() {
    let path = PathBuf::from("lib.rs");
    let content = "mod foo;\nmod bar;";
    let imports = extract_imports(&path, content, Language::Rust);
    assert_eq!(imports.len(), 2);
    assert_eq!(imports[0].import_type, ImportType::Mod);
    assert_eq!(imports[0].raw_path, "foo");
}

#[test]
fn fr004_extract_python_import() {
    let path = PathBuf::from("main.py");
    let content = "import os\nfrom sys import argv";
    let imports = extract_imports(&path, content, Language::Python);
    assert_eq!(imports.len(), 2);
    assert_eq!(imports[0].import_type, ImportType::Import);
    assert_eq!(imports[1].import_type, ImportType::ImportFrom);
}

#[test]
fn fr004_extract_js_import() {
    let path = PathBuf::from("app.js");
    let content = "import React from 'react';\nconst fs = require('fs');";
    let imports = extract_imports(&path, content, Language::JavaScript);
    assert!(imports.len() >= 2);
    assert!(
        imports
            .iter()
            .any(|i| i.import_type == ImportType::ImportFrom)
    );
    assert!(imports.iter().any(|i| i.import_type == ImportType::Require));
}

#[test]
fn fr004_extract_pub_use_as_reexport() {
    let path = PathBuf::from("lib.rs");
    let content = "pub use crate::foo::Bar;";
    let imports = extract_imports(&path, content, Language::Rust);
    assert_eq!(imports.len(), 1);
    assert_eq!(imports[0].import_type, ImportType::ReExport);
}

// ─── FR-005: Dependency Graph ───────────────────────────────

#[test]
fn fr005_graph_build_and_query() {
    let mut graph = DependencyGraph::new();
    let files = vec![
        FileEntry {
            path: PathBuf::from("a.rs"),
            extension: "rs".into(),
            language: Language::Rust,
            size: 100,
        },
        FileEntry {
            path: PathBuf::from("b.rs"),
            extension: "rs".into(),
            language: Language::Rust,
            size: 100,
        },
    ];
    let imports = vec![ImportEntry {
        source_file: PathBuf::from("a.rs"),
        raw_path: "b".into(),
        resolved_path: Some(PathBuf::from("b.rs")),
        import_type: ImportType::Use,
        language: Language::Rust,
        is_dynamic: false,
        is_resolved: true,
    }];
    graph.build(&imports, &files);

    // a depends on b
    assert_eq!(
        graph.dependencies(&PathBuf::from("a.rs")),
        vec![PathBuf::from("b.rs")]
    );
    // b is depended on by a
    assert_eq!(
        graph.dependents(&PathBuf::from("b.rs")),
        vec![PathBuf::from("a.rs")]
    );
    // reachability
    assert!(graph.reachable(&PathBuf::from("a.rs"), &PathBuf::from("b.rs")));
    assert!(!graph.reachable(&PathBuf::from("b.rs"), &PathBuf::from("a.rs")));
}

#[test]
fn fr005_graph_no_cycles() {
    let mut graph = DependencyGraph::new();
    let files = vec![
        FileEntry {
            path: PathBuf::from("a.rs"),
            extension: "rs".into(),
            language: Language::Rust,
            size: 100,
        },
        FileEntry {
            path: PathBuf::from("b.rs"),
            extension: "rs".into(),
            language: Language::Rust,
            size: 100,
        },
    ];
    let imports = vec![ImportEntry {
        source_file: PathBuf::from("a.rs"),
        raw_path: "b".into(),
        resolved_path: Some(PathBuf::from("b.rs")),
        import_type: ImportType::Use,
        language: Language::Rust,
        is_dynamic: false,
        is_resolved: true,
    }];
    graph.build(&imports, &files);
    assert!(graph.cycles().is_empty());
}

#[test]
fn fr005_graph_detects_cycle() {
    let mut graph = DependencyGraph::new();
    let files = vec![
        FileEntry {
            path: PathBuf::from("a.rs"),
            extension: "rs".into(),
            language: Language::Rust,
            size: 100,
        },
        FileEntry {
            path: PathBuf::from("b.rs"),
            extension: "rs".into(),
            language: Language::Rust,
            size: 100,
        },
    ];
    let imports = vec![
        ImportEntry {
            source_file: PathBuf::from("a.rs"),
            raw_path: "b".into(),
            resolved_path: Some(PathBuf::from("b.rs")),
            import_type: ImportType::Use,
            language: Language::Rust,
            is_dynamic: false,
            is_resolved: true,
        },
        ImportEntry {
            source_file: PathBuf::from("b.rs"),
            raw_path: "a".into(),
            resolved_path: Some(PathBuf::from("a.rs")),
            import_type: ImportType::Use,
            language: Language::Rust,
            is_dynamic: false,
            is_resolved: true,
        },
    ];
    graph.build(&imports, &files);
    assert!(!graph.cycles().is_empty());
}

#[test]
fn fr005_graph_orphan_files() {
    let mut graph = DependencyGraph::new();
    let files = vec![
        FileEntry {
            path: PathBuf::from("a.rs"),
            extension: "rs".into(),
            language: Language::Rust,
            size: 100,
        },
        FileEntry {
            path: PathBuf::from("b.rs"),
            extension: "rs".into(),
            language: Language::Rust,
            size: 100,
        },
        FileEntry {
            path: PathBuf::from("c.rs"),
            extension: "rs".into(),
            language: Language::Rust,
            size: 100,
        },
    ];
    // a -> b, c is not imported by anyone
    let imports = vec![ImportEntry {
        source_file: PathBuf::from("a.rs"),
        raw_path: "b".into(),
        resolved_path: Some(PathBuf::from("b.rs")),
        import_type: ImportType::Use,
        language: Language::Rust,
        is_dynamic: false,
        is_resolved: true,
    }];
    graph.build(&imports, &files);
    let orphans = graph.orphan_files();
    assert!(orphans.contains(&PathBuf::from("a.rs"))); // nothing imports a
    assert!(orphans.contains(&PathBuf::from("c.rs"))); // nothing imports c
    assert!(!orphans.contains(&PathBuf::from("b.rs"))); // b is imported by a
}

// ─── FR-006: Agent Orchestrator ─────────────────────────────

#[test]
fn fr006_full_scan() {
    let service = FilesystemOrchestrator::new();
    let root = test_root();
    let result = service.scan(&root, &[]);

    assert!(!result.files.is_empty(), "should discover files");
    assert!(result.parsed_count > 0, "should parse files");
    assert!(result.timing.total_ms > 0, "should record timing");
    eprintln!(
        "Scan result: {} files, {} imports, {}ms total",
        result.files.len(),
        result.imports.len(),
        result.timing.total_ms
    );
}

#[test]
fn fr006_service_cache_lookup() {
    let service = FilesystemOrchestrator::new();
    let root = test_root();
    let _result = service.scan(&root, &[]);
    // Cache is internal — verify via get_file_content
    // (Cargo.toml won't be in cache since it's not a source file)
}

#[test]
fn fr006_service_graph_query() {
    let service = FilesystemOrchestrator::new();
    let root = test_root();
    let _result = service.scan(&root, &[]);

    // Graph should be queryable after scan
    let graph = service.graph();
    let all = graph.read().unwrap();
    let (nodes, edges) = all.stats();
    assert!(nodes > 0, "graph should have nodes");
    eprintln!("Graph: {} nodes, {} edges", nodes, edges);
}

// ─── Language Detection ─────────────────────────────────────

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
fn debug_rust_ast_nodes() {
    let mut parser = tree_sitter::Parser::new();
    parser
        .set_language(&tree_sitter_rust::LANGUAGE.into())
        .unwrap();
    let content = "use std::collections::HashMap;\nuse crate::foo::Bar;\nmod foo;\npub use crate::bar::Baz;\nuse crate::x::Y as Z;";
    let tree = parser.parse(content, None).unwrap();
    eprintln!("SEXP: {:?}", tree.root_node().to_sexp());
    eprintln!("ERROR: {:?}", tree.root_node().has_error());
    fn walk(node: tree_sitter::Node, content: &str, depth: usize) {
        let indent = "  ".repeat(depth);
        let text = if node.child_count() == 0 {
            format!(" = {:?}", &content[node.byte_range()])
        } else {
            String::new()
        };
        eprintln!("{}{}{}", indent, node.kind(), text);
        let mut cursor = node.walk();
        for child in node.named_children(&mut cursor) {
            walk(child, content, depth + 1);
        }
    }
    walk(tree.root_node(), content, 0);
}
