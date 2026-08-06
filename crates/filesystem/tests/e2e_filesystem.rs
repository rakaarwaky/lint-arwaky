// E2E tests — full pipeline: scan → parse → graph → query.
use filesystem_lint_arwaky::agent_filesystem_orchestrator::FilesystemOrchestrator;
use filesystem_lint_arwaky::agent_filesystem_orchestrator::FilesystemOrchestratorDeps;
use filesystem_lint_arwaky::capabilities_ast_parser::ASTParser;
use filesystem_lint_arwaky::capabilities_dependency_graph::DependencyGraph;
use filesystem_lint_arwaky::capabilities_filesystem_io::CapabilitiesFileSystemIO;
use filesystem_lint_arwaky::capabilities_tool_resolution::CapabilitiesToolResolution;
use filesystem_lint_arwaky::capabilities_workspace_root_finder::CapabilitiesWorkspace;
use shared::common::taxonomy_language_vo::Language;
use shared::common::taxonomy_path_vo::FilePath;
use shared::common::PatternList;
use shared::filesystem::contract_filesystem_aggregate::IFilesystemAggregate;
use shared::filesystem::contract_filesystem_io_protocol::IFileSystemIOProtocol;
use shared::filesystem::contract_parser_protocol::IParserProtocol;
use shared::filesystem::contract_workspace_protocol::IWorkspaceProtocol;
use shared::filesystem::taxonomy_filesystem_vo::{DefinitionEntry, FileEntry};
use std::sync::Arc;
use tempfile::TempDir;

fn make_orchestrator() -> FilesystemOrchestrator {
    FilesystemOrchestrator::new(FilesystemOrchestratorDeps {
        io: Arc::new(CapabilitiesFileSystemIO::with_default_timing()),
        workspace: Arc::new(CapabilitiesWorkspace::new()),
        tool_resolution: Arc::new(CapabilitiesToolResolution::new()),
        parser: Arc::new(ASTParser::new()),
        graph: Arc::new(DependencyGraph::new()),
    })
}

#[test]
fn e2e_scan_parse_and_query_imports() {
    let tmp = TempDir::new().unwrap();
    let src = tmp.path().join("src");
    std::fs::create_dir_all(&src).unwrap();
    std::fs::write(
        src.join("lib.rs"),
        "pub mod util;\npub fn helper() -> i32 { 42 }\n",
    )
    .unwrap();
    std::fs::write(
        src.join("util.rs"),
        "use crate::lib::helper;\npub fn use_helper() { let _ = helper(); }\n",
    )
    .unwrap();

    let orch = make_orchestrator();

    // Step 1: Scan directory via IO protocol
    let paths = orch.scan_directory_with_ignored(&src, &PatternList::default());
    assert!(!paths.is_empty(), "Should discover source files");

    // Step 2: Build file entries and parse via parser protocol
    let mut files: Vec<FileEntry> = paths
        .iter()
        .map(|p| {
            let content = orch.read_to_string(p).unwrap_or_default().value;
            let ext = p
                .extension()
                .and_then(|e| e.to_str())
                .unwrap_or("")
                .to_string();
            let language = Language::from_extension(&ext).unwrap_or(Language::Unknown);
            FileEntry {
                path: p.clone(),
                extension: ext,
                language,
                size: content.len() as u64,
                content,
                parse_ok: false,
                parse_metadata: None,
            }
        })
        .collect();
    orch.parse_all(&mut files);

    // Step 3: Verify all files parsed
    for entry in &files {
        assert!(
            entry.parse_ok,
            "File {} should parse OK",
            entry.path.display()
        );
    }

    // Step 4: Verify imports were extracted via the parser capability directly
    // (orchestrator's import_list() returns from its own pipeline OnceLock, not the parser's)
    let parser = ASTParser::new();
    let mut parse_files: Vec<FileEntry> = files
        .iter()
        .map(|f| FileEntry {
            path: f.path.clone(),
            extension: f.extension.clone(),
            language: f.language,
            size: f.size,
            content: f.content.clone(),
            parse_ok: false,
            parse_metadata: None,
        })
        .collect();
    parser.parse_all(&mut parse_files);
    let imports = parser.import_list();
    assert!(!imports.is_empty(), "Should have extracted imports");
}

#[test]
fn e2e_full_pipeline_with_graph_query() {
    let tmp = TempDir::new().unwrap();
    let src = tmp.path().join("src");
    std::fs::create_dir_all(&src).unwrap();
    std::fs::write(src.join("a.rs"), "use crate::b::B;\npub struct A(pub B);\n").unwrap();
    std::fs::write(src.join("b.rs"), "pub struct B;\n").unwrap();

    let orch = make_orchestrator();

    // Scan
    let paths = orch.scan_directory_with_ignored(&src, &PatternList::default());

    // Parse
    let mut files: Vec<FileEntry> = paths
        .iter()
        .map(|p| {
            let content = orch.read_to_string(p).unwrap_or_default().value;
            let ext = p
                .extension()
                .and_then(|e| e.to_str())
                .unwrap_or("")
                .to_string();
            let language = Language::from_extension(&ext).unwrap_or(Language::Unknown);
            FileEntry {
                path: p.clone(),
                extension: ext,
                language,
                size: content.len() as u64,
                content,
                parse_ok: false,
                parse_metadata: None,
            }
        })
        .collect();
    orch.parse_all(&mut files);

    // Extract imports and build definitions for graph
    let mut imports = Vec::new();
    let mut definitions = Vec::new();
    let implementations = Vec::new();

    for entry in &files {
        if let Some(shared::filesystem::taxonomy_filesystem_vo::ParseMetadata::Rust(rust_meta)) =
            &entry.parse_metadata
        {
            for def in &rust_meta.struct_definitions {
                definitions.push(DefinitionEntry {
                    name: def.clone(),
                    file_path: entry.path.clone(),
                    language: Language::Rust,
                });
            }
        }
    }

    // Get imports from the parser capability directly (orchestrator's pipeline OnceLock is not populated)
    let parser = ASTParser::new();
    let mut reparse: Vec<FileEntry> = files
        .iter()
        .map(|f| FileEntry {
            path: f.path.clone(),
            extension: f.extension.clone(),
            language: f.language,
            size: f.size,
            content: f.content.clone(),
            parse_ok: false,
            parse_metadata: None,
        })
        .collect();
    parser.parse_all(&mut reparse);
    for imp in parser.import_list() {
        imports.push(imp.clone());
    }

    // Build graph via the dependency graph capability
    let graph = DependencyGraph::new();
    graph.build_graph(&imports, &files, &definitions, &implementations);

    // Query graph
    let (nodes, edges) = graph.stats();
    assert!(nodes >= 2, "Expected at least 2 nodes, got {}", nodes);
    assert!(edges >= 1, "Expected at least 1 edge, got {}", edges);
}

#[test]
fn e2e_orchestrator_collect_file_entries() {
    let tmp = TempDir::new().unwrap();
    std::fs::write(tmp.path().join("main.rs"), "fn main() {}").unwrap();
    std::fs::write(tmp.path().join("lib.rs"), "pub fn lib_fn() {}").unwrap();

    let orch = make_orchestrator();

    // Scan
    let paths = orch.scan_directory_with_ignored(tmp.path(), &PatternList::default());

    // Build file entries
    let files: Vec<FileEntry> = paths
        .iter()
        .map(|p| {
            let content = orch.read_to_string(p).unwrap_or_default().value;
            FileEntry {
                path: p.clone(),
                extension: p
                    .extension()
                    .and_then(|e| e.to_str())
                    .unwrap_or("")
                    .to_string(),
                language: Language::Rust,
                size: content.len() as u64,
                content,
                parse_ok: true,
                parse_metadata: None,
            }
        })
        .collect();

    // collect_file_entries falls through to disk reads when cache is empty
    let entries = orch.collect_file_entries(
        &PatternList::new(
            files
                .iter()
                .map(|f| f.path.to_string_lossy().to_string())
                .collect::<Vec<_>>(),
        ),
    );
    assert_eq!(entries.len(), files.len());
}

#[test]
fn e2e_workspace_detection_in_pipeline() {
    let tmp = TempDir::new().unwrap();
    std::fs::write(tmp.path().join("Cargo.toml"), "[package]\nname=\"test\"\n").unwrap();
    let src = tmp.path().join("src");
    std::fs::create_dir_all(&src).unwrap();
    std::fs::write(src.join("main.rs"), "fn main() {}").unwrap();

    let orch = make_orchestrator();
    let fp = FilePath::new(src.to_string_lossy().to_string()).unwrap();

    // Workspace detection
    let root = orch.workspace_root(&fp);
    assert!(root.is_some(), "Should find workspace root");

    // Language detection
    let lang = orch.detect_language_from_path("src/main.rs");
    assert_eq!(
        lang,
        shared::common::taxonomy_config_language_vo::ConfigLanguage::Rust
    );

    // Source dir detection — look for crates/packages/modules, not src/
    let source_dir = orch.detect_source_dir(tmp.path());
    // With no crates/packages/modules dir, falls back to root
    assert_eq!(source_dir, tmp.path().to_path_buf());
}
