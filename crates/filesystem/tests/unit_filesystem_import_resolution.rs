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
use filesystem_lint_arwaky::utility_barrel_resolution::{
    parse_barrel_reexports, resolve_single_import,
};
use shared::filesystem::contract_filesystem_io_protocol::IFileSystemIOProtocol;
use shared::filesystem::contract_parser_protocol::IParserProtocol;
use shared::filesystem::contract_tool_resolution_protocol::IToolResolutionProtocol;
use shared::filesystem::contract_workspace_protocol::IWorkspaceProtocol;
use shared::filesystem::taxonomy_filesystem_vo::{ImportEntry, ImportType, Language};

use std::collections::{HashMap, HashSet};
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

fn build_stem_index(paths: &[&str]) -> HashMap<String, Vec<String>> {
    let string_paths: Vec<String> = paths.iter().map(|&s| s.to_string()).collect();
    FilesystemOrchestrator::build_stem_index(&string_paths)
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
    let stem_index = build_stem_index(&[
        "modules/image/src/agent_image_orchestrator.py",
        "modules/image/src/__init__.py",
    ]);
    let root = PathBuf::from(".");
    let result =
        orch.resolve_import_target(&imp, "modules/test/src/test.py", &root, &files, &stem_index);
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
    let stem_index = build_stem_index(&[
        "modules/image/src/capabilities_image_processing_processor.py",
        "modules/test/src/test.py",
    ]);
    let root = PathBuf::from(".");
    let result =
        orch.resolve_import_target(&imp, "modules/test/src/test.py", &root, &files, &stem_index);
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
    let files: HashSet<&str> =
        build_file_set(&["modules/some_module.py", "modules/test/src/test.py"]);
    let stem_index = build_stem_index(&["modules/some_module.py", "modules/test/src/test.py"]);
    let root = PathBuf::from(".");
    let result =
        orch.resolve_import_target(&imp, "modules/test/src/test.py", &root, &files, &stem_index);
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
    let stem_index = build_stem_index(&[
        "modules/image/src/agent_image_orchestrator.py",
        "modules/modules/image/src/agent_image_orchestrator.py",
        "modules/test/src/test.py",
    ]);
    let root = PathBuf::from(".");
    let result =
        orch.resolve_import_target(&imp, "modules/test/src/test.py", &root, &files, &stem_index);
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
    let stem_index = build_stem_index(&["modules/some/module.py"]);
    let root = PathBuf::from(".");
    let result =
        orch.resolve_import_target(&imp, "modules/test/src/test.py", &root, &files, &stem_index);
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
    let stem_index = build_stem_index(&["modules/test/src/test.py"]);
    let root = PathBuf::from(".");
    let result =
        orch.resolve_import_target(&imp, "modules/test/src/test.py", &root, &files, &stem_index);
    assert!(result.is_none(), "Should return None when no match found");
}

#[test]
fn test_python_stem_match_skips_self_import() {
    // A file must not resolve to itself via Strategy C (stem match).
    // Regression: self-import through stem match creates a self-loop edge.
    let orch = make_orchestrator();
    let imp = make_python_import("capabilities_image_processing_processor", None);
    let files: HashSet<&str> =
        build_file_set(&["modules/image/src/capabilities_image_processing_processor.py"]);
    let stem_index =
        build_stem_index(&["modules/image/src/capabilities_image_processing_processor.py"]);
    let root = PathBuf::from(".");
    let result = orch.resolve_import_target(
        &imp,
        "modules/image/src/capabilities_image_processing_processor.py",
        &root,
        &files,
        &stem_index,
    );
    assert!(
        result.is_none(),
        "Should NOT resolve a module to its own file via stem match"
    );
}

#[test]
fn test_python_stem_match_ambiguous_uses_member_dir() {
    // Two files share a stem in different members; the importing member should win.
    // Determinism: prefer the importing file's member dir, then lexicographic order.
    let orch = make_orchestrator();
    let imp = make_python_import("capabilities_shared_processor", None);
    let files: HashSet<&str> = build_file_set(&[
        "modules/image/src/capabilities_shared_processor.py",
        "modules/audio/src/capabilities_shared_processor.py",
        "modules/image/src/root_image_entry.py",
    ]);
    let stem_index = build_stem_index(&[
        "modules/image/src/capabilities_shared_processor.py",
        "modules/audio/src/capabilities_shared_processor.py",
        "modules/image/src/root_image_entry.py",
    ]);
    let root = PathBuf::from(".");
    // Importing file lives in modules/image/src — should prefer modules/image's copy.
    let result = orch.resolve_import_target(
        &imp,
        "modules/image/src/root_image_entry.py",
        &root,
        &files,
        &stem_index,
    );
    assert_eq!(
        result,
        Some("modules/image/src/capabilities_shared_processor.py".to_string()),
        "Member-dir preference should pick the importing member's file"
    );
}

#[test]
fn e2e_chained_python_import_graph_reaches_capabilities() {
    // End-to-end: a real temp Python project with the chained import chain
    // root entry → surface → agent → capabilities must produce graph edges
    // so that BFS reachability marks the capabilities file alive.
    // This exercises resolve_import_target through the real pipeline,
    // not a hand-built graph.
    use shared::filesystem::contract_filesystem_aggregate::IFilesystemAggregate;

    let tmp = tempfile::TempDir::new().unwrap();
    let member_src = tmp.path().join("modules").join("image").join("src");
    std::fs::create_dir_all(&member_src).unwrap();

    // Entry → Surface (dotted import with member prefix)
    std::fs::write(
        member_src.join("root_image_entry.py"),
        "from modules.image.src.surfaces_image_cli import CliCommand\n\n\ndef main():\n    CliCommand().run()\n",
    )
    .unwrap();
    // Surface → Agent (dotted import with member prefix)
    std::fs::write(
        member_src.join("surfaces_image_cli.py"),
        "from modules.image.src.agent_image_orchestrator import ImageOrchestrator\n\n\nclass CliCommand:\n    def run(self):\n        ImageOrchestrator().process()\n",
    )
    .unwrap();
    // Agent → Capabilities (bare module name, Strategy C)
    std::fs::write(
        member_src.join("agent_image_orchestrator.py"),
        "from capabilities_image_processing_processor import ImageProcessor\n\n\nclass ImageOrchestrator:\n    def process(self):\n        ImageProcessor().run()\n",
    )
    .unwrap();
    // Capabilities — no imports needed; should be reachable via the chain
    std::fs::write(
        member_src.join("capabilities_image_processing_processor.py"),
        "class ImageProcessor:\n    def run(self):\n        pass\n",
    )
    .unwrap();

    let orch = make_orchestrator();
    let root = tmp.path().to_path_buf();
    let context = orch.build_orphan_graph_context(&root, &[]);

    let graph = &context.import_graph.mapping;
    // The entry file must import the surface (Strategy A on dotted path).
    assert!(
        graph
            .get("modules/image/src/root_image_entry.py")
            .map(|v| v.contains(&"modules/image/src/surfaces_image_cli.py".to_string()))
            .unwrap_or(false),
        "entry → surface edge missing: {:?}",
        graph
    );
    // The surface must import the agent.
    assert!(
        graph
            .get("modules/image/src/surfaces_image_cli.py")
            .map(|v| v.contains(&"modules/image/src/agent_image_orchestrator.py".to_string()))
            .unwrap_or(false),
        "surface → agent edge missing: {:?}",
        graph
    );
    // The agent must import the capabilities (Strategy C stem match).
    assert!(
        graph
            .get("modules/image/src/agent_image_orchestrator.py")
            .map(|v| {
                v.contains(
                    &"modules/image/src/capabilities_image_processing_processor.py".to_string(),
                )
            })
            .unwrap_or(false),
        "agent → capabilities edge missing: {:?}",
        graph
    );
}

#[test]
fn test_python_public_barrel_resolves_grouped_reexports() {
    let barrel = r#"
from .contract_core_protocol import (
    IUploadProtocol,
    ISendProtocol,
)
from .taxonomy_config_vo import (
    AppConfig,
)
from .utility_core_events import (
    EVENT_WEB_LOADED,
)
from .taxonomy_core_error import (
    QwenCliError,
)
"#;
    let reexports = parse_barrel_reexports(barrel);

    assert_eq!(
        reexports.get("IUploadProtocol"),
        Some(&"contract_core_protocol".to_string())
    );
    assert_eq!(
        reexports.get("AppConfig"),
        Some(&"taxonomy_config_vo".to_string())
    );
    assert_eq!(
        reexports.get("EVENT_WEB_LOADED"),
        Some(&"utility_core_events".to_string())
    );
    assert_eq!(
        reexports.get("QwenCliError"),
        Some(&"taxonomy_core_error".to_string())
    );

    let temp = tempfile::tempdir().unwrap();
    let shared_src = temp.path().join("modules/shared/src");
    std::fs::create_dir_all(&shared_src).unwrap();
    std::fs::write(shared_src.join("__init__.py"), barrel).unwrap();
    std::fs::write(shared_src.join("contract_core_protocol.py"), "").unwrap();
    std::fs::write(shared_src.join("taxonomy_config_vo.py"), "").unwrap();
    std::fs::write(shared_src.join("utility_core_events.py"), "").unwrap();
    std::fs::write(shared_src.join("taxonomy_core_error.py"), "").unwrap();

    for (symbol, expected) in [
        (
            "IUploadProtocol",
            "modules/shared/src/contract_core_protocol.py",
        ),
        ("AppConfig", "modules/shared/src/taxonomy_config_vo.py"),
        (
            "EVENT_WEB_LOADED",
            "modules/shared/src/utility_core_events.py",
        ),
        ("QwenCliError", "modules/shared/src/taxonomy_core_error.py"),
    ] {
        let entry = ImportEntry {
            source_file: temp.path().join("modules/consumer/src/consumer.py"),
            raw_path: "modules.shared.src".to_string(),
            resolved_path: None,
            import_type: ImportType::ImportFrom,
            language: Language::Python,
            is_dynamic: false,
            is_resolved: false,
            symbols: vec![symbol.to_string()],
            is_reexport: false,
            is_wildcard: false,
        };
        let resolved = resolve_single_import(entry, temp.path());
        assert_eq!(
            resolved.resolved_path,
            Some(PathBuf::from(expected)),
            "public barrel symbol {symbol} should resolve to its canonical source"
        );
        assert!(resolved.is_resolved);
    }
}

#[test]
fn test_python_grouped_reexports_ignore_inline_comments() {
    let barrel = r#"
from .taxonomy_config_vo import (  # public config exports
    AppConfig,  # application configuration
    BrowserConfig,  # browser configuration
)
"#;
    let reexports = parse_barrel_reexports(barrel);

    assert_eq!(
        reexports.get("AppConfig"),
        Some(&"taxonomy_config_vo".to_string())
    );
    assert_eq!(
        reexports.get("BrowserConfig"),
        Some(&"taxonomy_config_vo".to_string())
    );
}

#[test]
fn test_python_relative_barrel_preserves_parent_depth() {
    let temp = tempfile::tempdir().unwrap();
    let barrel_dir = temp.path().join("pkg/sub");
    std::fs::create_dir_all(&barrel_dir).unwrap();
    std::fs::write(
        barrel_dir.join("__init__.py"),
        "from ..shared import (\n    SharedType,\n)\n",
    )
    .unwrap();
    std::fs::write(temp.path().join("pkg/shared.py"), "").unwrap();

    let entry = ImportEntry {
        source_file: temp.path().join("consumer.py"),
        raw_path: "pkg.sub".to_string(),
        resolved_path: None,
        import_type: ImportType::ImportFrom,
        language: Language::Python,
        is_dynamic: false,
        is_resolved: false,
        symbols: vec!["SharedType".to_string()],
        is_reexport: false,
        is_wildcard: false,
    };

    let resolved = resolve_single_import(entry, temp.path());
    assert_eq!(
        resolved.resolved_path,
        Some(PathBuf::from("pkg/shared.py")),
        "a two-dot re-export from pkg/sub must resolve to pkg/shared.py"
    );
    assert!(resolved.is_resolved);
}
