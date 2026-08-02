// FR-002 — Dependency Graph Construction
// US1: A imports B creates edge A → B.
// US2: Circular imports produce both edges.
// US3: Symbol definitions map name → file.
// US4: Implementations map trait → implementing file.
// US5: Graph queries (dependents, dependencies, reachability).

use filesystem_lint_arwaky::capabilities_dependency_graph::DependencyGraph;
use shared::common::taxonomy_language_vo::Language;
use shared::filesystem::contract_graph_protocol::IGraphProtocol;
use shared::filesystem::taxonomy_filesystem_vo::{
    DefinitionEntry, FileEntry, ImportEntry, ImportType, ImplEntry,
};
use std::path::{Path, PathBuf};

fn make_file_entry(path: &str, lang: Language) -> FileEntry {
    FileEntry {
        path: PathBuf::from(path),
        extension: match lang {
            Language::Rust => "rs",
            Language::Python => "py",
            _ => "",
        }
        .to_string(),
        language: lang,
        size: 0,
        content: String::new(),
        parse_ok: true,
        parse_metadata: None,
    }
}

fn make_import(source: &str, target: &str, import_type: ImportType) -> ImportEntry {
    ImportEntry {
        source_file: PathBuf::from(source),
        raw_path: target.to_string(),
        resolved_path: Some(PathBuf::from(target)),
        import_type,
        language: Language::Rust,
        is_dynamic: false,
        is_resolved: true,
        symbols: vec![],
        is_reexport: false,
        is_wildcard: false,
    }
}

#[test]
fn us1_a_imports_b_creates_edge() {
    let graph = DependencyGraph::new();
    let files = vec![
        make_file_entry("/a.rs", Language::Rust),
        make_file_entry("/b.rs", Language::Rust),
    ];
    let imports = vec![make_import("/a.rs", "/b.rs", ImportType::Use)];
    graph.build_graph(&imports, &files, &[], &[]);

    let deps = graph.dependencies(Path::new("/a.rs"));
    assert!(deps.contains(&PathBuf::from("/b.rs")));
}

#[test]
fn us2_circular_imports_both_edges() {
    let graph = DependencyGraph::new();
    let files = vec![
        make_file_entry("/a.rs", Language::Rust),
        make_file_entry("/b.rs", Language::Rust),
    ];
    let imports = vec![
        make_import("/a.rs", "/b.rs", ImportType::Use),
        make_import("/b.rs", "/a.rs", ImportType::Use),
    ];
    graph.build_graph(&imports, &files, &[], &[]);

    assert!(graph.dependencies(Path::new("/a.rs")).contains(&PathBuf::from("/b.rs")));
    assert!(graph.dependencies(Path::new("/b.rs")).contains(&PathBuf::from("/a.rs")));
}

#[test]
fn us3_symbol_definitions_map() {
    let graph = DependencyGraph::new();
    let files = vec![make_file_entry("/a.rs", Language::Rust)];
    let definitions = vec![DefinitionEntry {
        name: "MyStruct".to_string(),
        file_path: PathBuf::from("/a.rs"),
        language: Language::Rust,
    }];
    graph.build_graph(&imports_empty(), &files, &definitions, &[]);

    let defs = graph.symbol_definitions();
    assert!(defs.contains_key("MyStruct"));
    assert!(defs["MyStruct"].contains(&PathBuf::from("/a.rs")));
}

#[test]
fn us4_implementations_map() {
    let graph = DependencyGraph::new();
    let files = vec![make_file_entry("/a.rs", Language::Rust)];
    let implementations = vec![ImplEntry {
        trait_name: "Display".to_string(),
        file_path: PathBuf::from("/a.rs"),
        language: Language::Rust,
    }];
    graph.build_graph(&imports_empty(), &files, &[], &implementations);

    let impls = graph.implementations();
    assert!(impls.contains_key("Display"));
    assert!(impls["Display"].contains(&PathBuf::from("/a.rs")));
}

#[test]
fn us5_dependents_and_reverse_links() {
    let graph = DependencyGraph::new();
    let files = vec![
        make_file_entry("/a.rs", Language::Rust),
        make_file_entry("/b.rs", Language::Rust),
    ];
    let imports = vec![make_import("/a.rs", "/b.rs", ImportType::Use)];
    graph.build_graph(&imports, &files, &[], &[]);

    let reverse = graph.reverse_links();
    assert!(reverse.contains_key(&PathBuf::from("/b.rs")));
    assert!(reverse[&PathBuf::from("/b.rs")].contains(&PathBuf::from("/a.rs")));

    let dependents = graph.dependents(Path::new("/b.rs"));
    assert!(dependents.contains(&PathBuf::from("/a.rs")));
}

#[test]
fn us5_reachability() {
    let graph = DependencyGraph::new();
    let files = vec![
        make_file_entry("/a.rs", Language::Rust),
        make_file_entry("/b.rs", Language::Rust),
        make_file_entry("/c.rs", Language::Rust),
    ];
    let imports = vec![
        make_import("/a.rs", "/b.rs", ImportType::Use),
        make_import("/b.rs", "/c.rs", ImportType::Use),
    ];
    graph.build_graph(&imports, &files, &[], &[]);

    assert!(graph.reachable(Path::new("/a.rs"), Path::new("/c.rs")));
}

#[test]
fn us5_nonexistent_file_has_no_deps() {
    let graph = DependencyGraph::new();
    let files = vec![make_file_entry("/a.rs", Language::Rust)];
    graph.build_graph(&imports_empty(), &files, &[], &[]);

    assert!(graph.dependencies(Path::new("/nonexistent.rs")).is_empty());
    assert!(graph.dependents(Path::new("/nonexistent.rs")).is_empty());
}

#[test]
fn fr002_orphan_files_detected() {
    let graph = DependencyGraph::new();
    let files = vec![
        make_file_entry("/a.rs", Language::Rust),
        make_file_entry("/orphan.rs", Language::Rust),
    ];
    let imports = vec![make_import("/a.rs", "/b.rs", ImportType::Use)];
    graph.build_graph(&imports, &files, &[], &[]);

    let orphans = graph.orphan_files();
    assert!(orphans.contains(&PathBuf::from("/orphan.rs")));
}

#[test]
fn fr002_cycle_detection() {
    let graph = DependencyGraph::new();
    let files = vec![
        make_file_entry("/a.rs", Language::Rust),
        make_file_entry("/b.rs", Language::Rust),
        make_file_entry("/c.rs", Language::Rust),
    ];
    let imports = vec![
        make_import("/a.rs", "/b.rs", ImportType::Use),
        make_import("/b.rs", "/c.rs", ImportType::Use),
        make_import("/c.rs", "/a.rs", ImportType::Use),
    ];
    graph.build_graph(&imports, &files, &[], &[]);

    let cycles = graph.cycles();
    assert!(!cycles.is_empty(), "Should detect cycle");
}

#[test]
fn fr002_graph_stats() {
    let graph = DependencyGraph::new();
    let files = vec![
        make_file_entry("/a.rs", Language::Rust),
        make_file_entry("/b.rs", Language::Rust),
    ];
    let imports = vec![make_import("/a.rs", "/b.rs", ImportType::Use)];
    graph.build_graph(&imports, &files, &[], &[]);

    let (nodes, edges) = graph.stats();
    assert_eq!(nodes, 2);
    assert_eq!(edges, 1);
}

fn imports_empty() -> Vec<ImportEntry> {
    Vec::new()
}
