// Unit tests — Python import resolution strategies.
// Tests for resolve_import_target() in agent_filesystem_orchestrator.

use filesystem_lint_arwaky::agent_filesystem_orchestrator::{
    FilesystemOrchestrator, FilesystemOrchestratorDeps,
};
use filesystem_lint_arwaky::capabilities_ast_parser::ASTParser;
use filesystem_lint_arwaky::capabilities_dependency_graph::DependencyGraph;
use filesystem_lint_arwaky::capabilities_filesystem_io::CapabilitiesFileSystemIO;
use filesystem_lint_arwaky::capabilities_tool_resolution::CapabilitiesToolResolution;
use filesystem_lint_arwaky::capabilities_workspace_root_finder::CapabilitiesWorkspace;
use shared::filesystem::contract_filesystem_io_protocol::IFileSystemIOProtocol;
use shared::filesystem::contract_parser_protocol::IParserProtocol;
use shared::filesystem::contract_tool_resolution_protocol::IToolResolutionProtocol;
use shared::filesystem::contract_workspace_protocol::IWorkspaceProtocol;
use shared::filesystem::taxonomy_filesystem_vo::{ImportEntry, ImportType, Language};

use std::collections::HashSet;
use std::path::PathBuf;
use std::sync::Arc;

fn make_python_import(raw: &str, resolved: Option<&str>) -> ImportEntry {
    ImportEntry {
        source_file: PathBuf::from("modules/test/src/test.py"),
        raw_path: raw.to_string(),
        resolved_path: resolved.map(PathBuf::from),
        import_type: ImportType::ImportFrom,
        language: Language::Python,
        is_dynamic: false,
        is_resolved: resolved.is_some(),
        symbols: vec![],
        is_reexport: false,
        is_wildcard: false,
    }
}

fn build_file_set<'a>(paths: &'a [&'a str]) -> HashSet<&'a str> {
    paths.iter().copied().collect()
}

fn make_orchestrator() -> FilesystemOrchestrator {
    let io: Arc<dyn IFileSystemIOProtocol> =
        Arc::new(CapabilitiesFileSystemIO::with_default_timing());
    let workspace: Arc<dyn IWorkspaceProtocol> = Arc::new(CapabilitiesWorkspace::new());
    let tool_resolution: Arc<dyn IToolResolutionProtocol> =
        Arc::new(CapabilitiesToolResolution::new());
    let parser: Arc<dyn IParserProtocol> = Arc::new(ASTParser::new());
    let graph: Arc<dyn shared::filesystem::contract_graph_protocol::IGraphProtocol> =
        Arc::new(DependencyGraph::new());

    FilesystemOrchestrator::new(FilesystemOrchestratorDeps {
        io,
        workspace,
        tool_resolution,
        parser,
        graph,
    })
}

#[test]
fn test_python_direct_path_resolves() {
    // Strategy A: Direct path lookup handles imports that already include member prefix.
    // `from modules.image.src.agent_image_orchestrator import X`
    // → should resolve to `modules/image/src/agent_image_orchestrator.py`
    let orch = make_orchestrator();
    let imp = make_python_import("modules.image.src.agent_image_orchestrator", None);
    let files: HashSet<&str> = build_file_set(&[
        "modules/image/src/agent_image_orchestrator.py",
        "modules/image/src/__init__.py",
    ]);
    let root = PathBuf::from(".");
    let result = orch.resolve_import_target(
        &imp,
        "modules/test/src/test.py",
        &root,
        &files,
    );
    assert_eq!(
        result,
        Some("modules/image/src/agent_image_orchestrator.py".to_string()),
        "Strategy A should resolve direct path without double prefix"
    );
}

#[test]
fn test_python_suffix_match_resolves_nested() {
    // Strategy C: Suffix/stem matching handles bare module names in nested directories.
    // `from capabilities_image_processing_processor import X`
    // where file is at `modules/image/src/capabilities_image_processing_processor.py`
    let orch = make_orchestrator();
    let imp = make_python_import("capabilities_image_processing_processor", None);
    let files: HashSet<&str> = build_file_set(&[
        "modules/image/src/capabilities_image_processing_processor.py",
        "modules/test/src/test.py",
    ]);
    let root = PathBuf::from(".");
    let result = orch.resolve_import_target(
        &imp,
        "modules/test/src/test.py",
        &root,
        &files,
    );
    assert_eq!(
        result,
        Some("modules/image/src/capabilities_image_processing_processor.py".to_string()),
        "Strategy C should find file by stem match in nested directory"
    );
}

#[test]
fn test_python_prefix_based_resolves() {
    // Strategy B: Prefix-based resolution (existing behavior).
    // `from some_module import X` where file is at `modules/some_module.py`
    let orch = make_orchestrator();
    let imp = make_python_import("some_module", None);
    let files: HashSet<&str> = build_file_set(&[
        "modules/some_module.py",
        "modules/test/src/test.py",
    ]);
    let root = PathBuf::from(".");
    let result = orch.resolve_import_target(
        &imp,
        "modules/test/src/test.py",
        &root,
        &files,
    );
    assert_eq!(
        result,
        Some("modules/some_module.py".to_string()),
        "Strategy B should resolve with modules/ prefix"
    );
}

#[test]
fn test_python_direct_path_preferred_over_prefix() {
    // Strategy A should be tried before Strategy B.
    // If direct path matches, it should be preferred over prefix-based.
    let orch = make_orchestrator();
    let imp = make_python_import("modules.image.src.agent_image_orchestrator", None);
    let files: HashSet<&str> = build_file_set(&[
        "modules/image/src/agent_image_orchestrator.py",
        "modules/modules/image/src/agent_image_orchestrator.py",
        "modules/test/src/test.py",
    ]);
    let root = PathBuf::from(".");
    let result = orch.resolve_import_target(
        &imp,
        "modules/test/src/test.py",
        &root,
        &files,
    );
    // Should match direct path (Strategy A), NOT the double-prefix (Strategy B)
    assert_eq!(
        result,
        Some("modules/image/src/agent_image_orchestrator.py".to_string()),
        "Direct path should resolve without double prefix"
    );
}

#[test]
fn test_python_import_with_resolved_path_returns_as_is() {
    // If resolved_path is already set, skip resolution.
    let orch = make_orchestrator();
    let imp = make_python_import("some.module", Some("modules/some/module.py"));
    let files: HashSet<&str> = build_file_set(&["modules/some/module.py"]);
    let root = PathBuf::from(".");
    let result = orch.resolve_import_target(
        &imp,
        "modules/test/src/test.py",
        &root,
        &files,
    );
    assert_eq!(
        result,
        Some("modules/some/module.py".to_string()),
        "Should return already-resolved path"
    );
}

#[test]
fn test_python_no_match_returns_none() {
    // When no strategy finds a match, should return None.
    let orch = make_orchestrator();
    let imp = make_python_import("nonexistent_module", None);
    let files: HashSet<&str> = build_file_set(&["modules/test/src/test.py"]);
    let root = PathBuf::from(".");
    let result = orch.resolve_import_target(
        &imp,
        "modules/test/src/test.py",
        &root,
        &files,
    );
    assert!(result.is_none(), "Should return None when no match found");
}
