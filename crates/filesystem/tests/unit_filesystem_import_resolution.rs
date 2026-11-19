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
use shared_filesystem::FilesystemRequest;
use shared_filesystem::contract_filesystem_aggregate::IFilesystemAggregate;
use shared_filesystem::contract_filesystem_protocol::IFileSystemIOProtocol;
use shared_filesystem::contract_filesystem_protocol::IParserProtocol;
use shared_filesystem::contract_filesystem_protocol::IToolResolutionProtocol;
use shared_filesystem::contract_filesystem_protocol::IWorkspaceProtocol;
use shared_filesystem::taxonomy_filesystem_vo::{FileEntry, ImportEntry, ImportType, Language};
use shared_filesystem::utility_agent_filesystem::build_stem_index as shared_build_stem_index;
use shared_filesystem::utility_barrel_resolution::{parse_barrel_reexports, resolve_single_import};
use shared_filesystem::utility_import_resolution::resolve_import_target;

use std::collections::{HashMap, HashSet};
use std::path::PathBuf;
use std::sync::Arc;

use tempfile::tempdir;

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
        line: 0,
    }
}

fn make_typescript_import(source_file: &str, raw_path: &str) -> ImportEntry {
    ImportEntry {
        source_file: PathBuf::from(source_file),
        raw_path: raw_path.to_string(),
        resolved_path: None,
        import_type: ImportType::Import,
        language: Language::TypeScript,
        is_dynamic: false,
        is_resolved: false,
        symbols: vec![],
        is_reexport: false,
        is_wildcard: false,
        line: 0,
    }
}

fn build_file_set<'a>(paths: &'a [&'a str]) -> HashSet<&'a str> {
    paths.iter().copied().collect()
}

fn build_stem_index(paths: &[&str]) -> HashMap<String, Vec<String>> {
    let string_paths: Vec<String> = paths.iter().map(|&s| s.to_string()).collect();
    shared_build_stem_index(&string_paths)
}

fn make_orchestrator() -> FilesystemOrchestrator {
    let io: Arc<dyn IFileSystemIOProtocol> =
        Arc::new(CapabilitiesFileSystemIO::with_default_timing());
    let workspace: Arc<dyn IWorkspaceProtocol> = Arc::new(CapabilitiesWorkspace::new());
    let tool_resolution: Arc<dyn IToolResolutionProtocol> =
        Arc::new(CapabilitiesToolResolution::new());
    let parser: Arc<dyn IParserProtocol> = Arc::new(ASTParser::new());
    let graph: Arc<dyn shared_filesystem::contract_filesystem_protocol::IGraphProtocol> =
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
    let _orch = make_orchestrator();
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
        resolve_import_target(&imp, "modules/test/src/test.py", &root, &files, &stem_index);
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
    let _orch = make_orchestrator();
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
        resolve_import_target(&imp, "modules/test/src/test.py", &root, &files, &stem_index);
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
    let _orch = make_orchestrator();
    let imp = make_python_import("some_module", None);
    let files: HashSet<&str> =
        build_file_set(&["modules/some_module.py", "modules/test/src/test.py"]);
    let stem_index = build_stem_index(&["modules/some_module.py", "modules/test/src/test.py"]);
    let root = PathBuf::from(".");
    let result =
        resolve_import_target(&imp, "modules/test/src/test.py", &root, &files, &stem_index);
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
    let _orch = make_orchestrator();
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
        resolve_import_target(&imp, "modules/test/src/test.py", &root, &files, &stem_index);
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
    let _orch = make_orchestrator();
    let imp = make_python_import("some.module", Some("modules/some/module.py"));
    let files: HashSet<&str> = build_file_set(&["modules/some/module.py"]);
    let stem_index = build_stem_index(&["modules/some/module.py"]);
    let root = PathBuf::from(".");
    let result =
        resolve_import_target(&imp, "modules/test/src/test.py", &root, &files, &stem_index);
    assert_eq!(
        result,
        Some("modules/some/module.py".to_string()),
        "Should return already-resolved path"
    );
}

#[test]
fn test_python_no_match_returns_none() {
    // When no strategy finds a match, should return None.
    let _orch = make_orchestrator();
    let imp = make_python_import("nonexistent_module", None);
    let files: HashSet<&str> = build_file_set(&["modules/test/src/test.py"]);
    let stem_index = build_stem_index(&["modules/test/src/test.py"]);
    let root = PathBuf::from(".");
    let result =
        resolve_import_target(&imp, "modules/test/src/test.py", &root, &files, &stem_index);
    assert!(result.is_none(), "Should return None when no match found");
}

#[test]
fn test_python_stem_match_skips_self_import() {
    // A file must not resolve to itself via Strategy C (stem match).
    // Regression: self-import through stem match creates a self-loop edge.
    let _orch = make_orchestrator();
    let imp = make_python_import("capabilities_image_processing_processor", None);
    let files: HashSet<&str> =
        build_file_set(&["modules/image/src/capabilities_image_processing_processor.py"]);
    let stem_index =
        build_stem_index(&["modules/image/src/capabilities_image_processing_processor.py"]);
    let root = PathBuf::from(".");
    let result = resolve_import_target(
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
    let _orch = make_orchestrator();
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
    let result = resolve_import_target(
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
    use shared_filesystem::FilesystemRequest;
    use shared_filesystem::contract_filesystem_aggregate::IFilesystemAggregate;

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
    let context = orch
        .execute(FilesystemRequest::build_orphan_graph_context(&root, &[]))
        .into_graph_context();

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
from .taxonomy_config_system_vo import (
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
        Some(&"taxonomy_config_system_vo".to_string())
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
    std::fs::write(shared_src.join("taxonomy_config_system_vo.py"), "").unwrap();
    std::fs::write(shared_src.join("utility_core_events.py"), "").unwrap();
    std::fs::write(shared_src.join("taxonomy_core_error.py"), "").unwrap();

    for (symbol, expected) in [
        (
            "IUploadProtocol",
            "modules/shared/src/contract_core_protocol.py",
        ),
        (
            "AppConfig",
            "modules/shared/src/taxonomy_config_system_vo.py",
        ),
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
            line: 0,
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
from .taxonomy_config_system_vo import (  # public config exports
    AppConfig,  # application configuration
    BrowserConfig,  # browser configuration
)
"#;
    let reexports = parse_barrel_reexports(barrel);

    assert_eq!(
        reexports.get("AppConfig"),
        Some(&"taxonomy_config_system_vo".to_string())
    );
    assert_eq!(
        reexports.get("BrowserConfig"),
        Some(&"taxonomy_config_system_vo".to_string())
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
        line: 0,
    };

    let resolved = resolve_single_import(entry, temp.path());
    assert_eq!(
        resolved.resolved_path,
        Some(PathBuf::from("pkg/shared.py")),
        "a two-dot re-export from pkg/sub must resolve to pkg/shared.py"
    );
    assert!(resolved.is_resolved);
}

#[test]
fn ast_parser_resolves_each_grouped_barrel_symbol_independently() {
    let temp = tempdir().unwrap();
    let shared_src = temp.path().join("modules/shared/src");
    let consumer = temp.path().join("modules/core/src/consumer.py");
    std::fs::create_dir_all(&shared_src).unwrap();
    std::fs::create_dir_all(consumer.parent().unwrap()).unwrap();

    std::fs::write(
        shared_src.join("__init__.py"),
        "from .taxonomy_core_vo import AppConfig\nfrom .contract_core_protocol import IUploadProtocol\nfrom .utility_core_events import EVENT_WEB_LOADED\nfrom .taxonomy_core_error import QwenCliError\n",
    )
    .unwrap();
    for source in [
        "taxonomy_core_vo.py",
        "contract_core_protocol.py",
        "utility_core_events.py",
        "taxonomy_core_error.py",
    ] {
        std::fs::write(shared_src.join(source), "").unwrap();
    }
    let consumer_content = "from modules.shared.src import (\n    AppConfig,\n    IUploadProtocol,\n    EVENT_WEB_LOADED,\n    QwenCliError,\n)\n";
    std::fs::write(&consumer, consumer_content).unwrap();

    let parser = ASTParser::new();
    let mut files = vec![FileEntry {
        path: consumer.clone(),
        extension: "py".to_string(),
        language: Language::Python,
        size: consumer_content.len() as u64,
        content: consumer_content.to_string(),
        parse_ok: false,
        parse_metadata: None,
    }];
    parser.parse_all(&mut files);
    parser.resolve_barrel_imports(temp.path());

    let imports = parser.imports_for(&consumer);
    assert_eq!(
        imports.len(),
        4,
        "grouped barrel import must be split per symbol"
    );
    let resolved: HashMap<_, _> = imports
        .into_iter()
        .map(|entry| {
            (
                entry.symbols[0].clone(),
                entry.resolved_path.unwrap().to_string_lossy().to_string(),
            )
        })
        .collect();

    assert_eq!(
        resolved.get("AppConfig"),
        Some(&"modules/shared/src/taxonomy_core_vo.py".to_string())
    );
    assert_eq!(
        resolved.get("IUploadProtocol"),
        Some(&"modules/shared/src/contract_core_protocol.py".to_string())
    );
    assert_eq!(
        resolved.get("EVENT_WEB_LOADED"),
        Some(&"modules/shared/src/utility_core_events.py".to_string())
    );
    assert_eq!(
        resolved.get("QwenCliError"),
        Some(&"modules/shared/src/taxonomy_core_error.py".to_string())
    );
}

// ─── Root-level relative-import tests (#921) ─────────────────────────────────

#[test]
fn test_root_relative_dot_slash_resolves() {
    // Root-level main.js declares `import "./x"` → x.js must resolve.
    // Regression: #921 — Path::new("").join("x") = "x" (works), but
    // Path::new("").join("../y") = "../y" (never matches clean relative keys).
    let _orch = make_orchestrator();
    let imp = ImportEntry {
        source_file: PathBuf::from("main.js"),
        raw_path: "./x".to_string(),
        resolved_path: None,
        import_type: ImportType::Import,
        language: Language::JavaScript,
        is_dynamic: false,
        is_resolved: false,
        symbols: vec![],
        is_reexport: false,
        is_wildcard: false,
        line: 0,
    };
    let files: HashSet<&str> = build_file_set(&["main.js", "x.js"]);
    let stem_index = build_stem_index(&["main.js", "x.js"]);
    let root = PathBuf::from(".");
    let result = resolve_import_target(&imp, "main.js", &root, &files, &stem_index);
    assert_eq!(
        result,
        Some("x.js".to_string()),
        "root-level relative import ./x must resolve to x.js"
    );
}

#[test]
fn test_root_relative_dotdot_resolves() {
    // Root-level main.js declares `import "../y"` → y.js must resolve.
    // Regression: #921 — Path::new("").join("../y") = "../y", which never
    // matches the clean key "y.js" in all_files_set.
    let _orch = make_orchestrator();
    let imp = ImportEntry {
        source_file: PathBuf::from("main.js"),
        raw_path: "../y".to_string(),
        resolved_path: None,
        import_type: ImportType::Import,
        language: Language::JavaScript,
        is_dynamic: false,
        is_resolved: false,
        symbols: vec![],
        is_reexport: false,
        is_wildcard: false,
        line: 0,
    };
    let files: HashSet<&str> = build_file_set(&["main.js", "y.js"]);
    let stem_index = build_stem_index(&["main.js", "y.js"]);
    let root = PathBuf::from(".");
    let result = resolve_import_target(&imp, "main.js", &root, &files, &stem_index);
    assert_eq!(
        result,
        Some("y.js".to_string()),
        "root-level relative import ../y must resolve to y.js (strip leading .. for root)"
    );
}

// ─── JS/TS extension-swapping tests ─────────────────────────────────────────

#[test]
fn test_js_import_resolves_to_ts_file_on_disk() {
    // Primary bug regression: ESM/TypeScript projects import `.js` specifiers
    // that point to `.ts` source files on disk.
    let _orch = make_orchestrator();
    let imp = make_typescript_import("packages/root_web_main_entry.ts", "./root_web_container.js");
    let files: HashSet<&str> = build_file_set(&[
        "packages/root_web_main_entry.ts",
        "packages/root_web_container.ts",
    ]);
    let stem_index = build_stem_index(&[
        "packages/root_web_main_entry.ts",
        "packages/root_web_container.ts",
    ]);
    let root = PathBuf::from(".");
    let result = resolve_import_target(
        &imp,
        "packages/root_web_main_entry.ts",
        &root,
        &files,
        &stem_index,
    );
    assert_eq!(
        result,
        Some("packages/root_web_container.ts".to_string()),
        ".js relative import must resolve to .ts file on disk"
    );
}

#[test]
fn test_jsx_import_resolves_to_tsx_file_on_disk() {
    let _orch = make_orchestrator();
    let imp = make_typescript_import("packages/ui/button.jsx", "./icon.jsx");
    let files: HashSet<&str> = build_file_set(&["packages/ui/button.jsx", "packages/ui/icon.tsx"]);
    let stem_index = build_stem_index(&["packages/ui/button.jsx", "packages/ui/icon.tsx"]);
    let root = PathBuf::from(".");
    let result = resolve_import_target(&imp, "packages/ui/button.jsx", &root, &files, &stem_index);
    assert_eq!(
        result,
        Some("packages/ui/icon.tsx".to_string()),
        ".jsx import must swap to .tsx file on disk"
    );
}

#[test]
fn test_ts_import_resolves_to_js_file_on_disk() {
    // Reverse direction: a `.ts` import specifier must swap to `.js` on disk.
    let _orch = make_orchestrator();
    let imp = make_typescript_import("packages/ui/button.ts", "./legacy.ts");
    let files: HashSet<&str> = build_file_set(&["packages/ui/button.ts", "packages/ui/legacy.js"]);
    let stem_index = build_stem_index(&["packages/ui/button.ts", "packages/ui/legacy.js"]);
    let root = PathBuf::from(".");
    let result = resolve_import_target(&imp, "packages/ui/button.ts", &root, &files, &stem_index);
    assert_eq!(
        result,
        Some("packages/ui/legacy.js".to_string()),
        ".ts import must swap to .js file on disk"
    );
}

#[test]
fn test_tsx_import_resolves_to_jsx_file_on_disk() {
    let _orch = make_orchestrator();
    let imp = make_typescript_import("packages/ui/app.tsx", "./widget.tsx");
    let files: HashSet<&str> = build_file_set(&["packages/ui/app.tsx", "packages/ui/widget.jsx"]);
    let stem_index = build_stem_index(&["packages/ui/app.tsx", "packages/ui/widget.jsx"]);
    let root = PathBuf::from(".");
    let result = resolve_import_target(&imp, "packages/ui/app.tsx", &root, &files, &stem_index);
    assert_eq!(
        result,
        Some("packages/ui/widget.jsx".to_string()),
        ".tsx import must swap to .jsx file on disk"
    );
}

#[test]
fn test_mjs_import_resolves_to_mts_file_on_disk() {
    let _orch = make_orchestrator();
    let imp = make_typescript_import("packages/core/index.mts", "./util.mjs");
    let files: HashSet<&str> =
        build_file_set(&["packages/core/index.mts", "packages/core/util.mts"]);
    let stem_index = build_stem_index(&["packages/core/index.mts", "packages/core/util.mts"]);
    let root = PathBuf::from(".");
    let result = resolve_import_target(&imp, "packages/core/index.mts", &root, &files, &stem_index);
    assert_eq!(
        result,
        Some("packages/core/util.mts".to_string()),
        ".mjs import must swap to .mts file on disk"
    );
}

#[test]
fn test_mts_import_resolves_to_mjs_file_on_disk() {
    let _orch = make_orchestrator();
    let imp = make_typescript_import("packages/core/index.mjs", "./util.mts");
    let files: HashSet<&str> =
        build_file_set(&["packages/core/index.mjs", "packages/core/util.mjs"]);
    let stem_index = build_stem_index(&["packages/core/index.mjs", "packages/core/util.mjs"]);
    let root = PathBuf::from(".");
    let result = resolve_import_target(&imp, "packages/core/index.mjs", &root, &files, &stem_index);
    assert_eq!(
        result,
        Some("packages/core/util.mjs".to_string()),
        ".mts import must swap to .mjs file on disk"
    );
}

#[test]
fn test_cjs_import_resolves_to_cts_file_on_disk() {
    let _orch = make_orchestrator();
    let imp = make_typescript_import("packages/core/index.cts", "./helper.cjs");
    let files: HashSet<&str> =
        build_file_set(&["packages/core/index.cts", "packages/core/helper.cts"]);
    let stem_index = build_stem_index(&["packages/core/index.cts", "packages/core/helper.cts"]);
    let root = PathBuf::from(".");
    let result = resolve_import_target(&imp, "packages/core/index.cts", &root, &files, &stem_index);
    assert_eq!(
        result,
        Some("packages/core/helper.cts".to_string()),
        ".cjs import must swap to .cts file on disk"
    );
}

#[test]
fn test_cts_import_resolves_to_cjs_file_on_disk() {
    let _orch = make_orchestrator();
    let imp = make_typescript_import("packages/core/index.cjs", "./helper.cts");
    let files: HashSet<&str> =
        build_file_set(&["packages/core/index.cjs", "packages/core/helper.cjs"]);
    let stem_index = build_stem_index(&["packages/core/index.cjs", "packages/core/helper.cjs"]);
    let root = PathBuf::from(".");
    let result = resolve_import_target(&imp, "packages/core/index.cjs", &root, &files, &stem_index);
    assert_eq!(
        result,
        Some("packages/core/helper.cjs".to_string()),
        ".cts import must swap to .cjs file on disk"
    );
}

#[test]
fn test_js_import_prefers_ts_when_both_ts_and_tsx_exist() {
    // Priority verification: `.js` import swaps try `.ts` before `.tsx` and `.jsx`.
    let _orch = make_orchestrator();
    let imp = make_typescript_import("packages/ui/entry.ts", "./foo.js");
    let files: HashSet<&str> = build_file_set(&[
        "packages/ui/entry.ts",
        "packages/ui/foo.ts",
        "packages/ui/foo.tsx",
    ]);
    let stem_index = build_stem_index(&[
        "packages/ui/entry.ts",
        "packages/ui/foo.ts",
        "packages/ui/foo.tsx",
    ]);
    let root = PathBuf::from(".");
    let result = resolve_import_target(&imp, "packages/ui/entry.ts", &root, &files, &stem_index);
    assert_eq!(
        result,
        Some("packages/ui/foo.ts".to_string()),
        ".js swap must prefer .ts over .tsx"
    );
}

#[test]
fn test_exact_file_match_takes_precedence_over_swap() {
    // Suggestion 2: When both `./foo.js` and `./foo.ts` exist on disk and a module
    // imports `./foo.js`, the exact match on `normalized` must take precedence.
    let _orch = make_orchestrator();
    let imp = make_typescript_import("packages/ui/entry.ts", "./foo.js");
    let files: HashSet<&str> = build_file_set(&[
        "packages/ui/entry.ts",
        "packages/ui/foo.js",
        "packages/ui/foo.ts",
    ]);
    let stem_index = build_stem_index(&[
        "packages/ui/entry.ts",
        "packages/ui/foo.js",
        "packages/ui/foo.ts",
    ]);
    let root = PathBuf::from(".");
    let result = resolve_import_target(&imp, "packages/ui/entry.ts", &root, &files, &stem_index);
    assert_eq!(
        result,
        Some("packages/ui/foo.js".to_string()),
        "exact match on ./foo.js must win over swapped .ts file"
    );
}

#[test]
fn test_non_js_ts_extensions_unaffected() {
    let _orch = make_orchestrator();
    // .rs import that does not exist on disk should return None (not swap to .py)
    let imp_rs = ImportEntry {
        source_file: PathBuf::from("crates/foo/src/lib.rs"),
        raw_path: "./missing.rs".to_string(),
        resolved_path: None,
        import_type: ImportType::Mod,
        language: Language::Rust,
        is_dynamic: false,
        is_resolved: false,
        symbols: vec![],
        is_reexport: false,
        is_wildcard: false,
        line: 0,
    };
    let files_rs: HashSet<&str> =
        build_file_set(&["crates/foo/src/lib.rs", "crates/foo/src/other.py"]);
    let stem_rs = build_stem_index(&["crates/foo/src/lib.rs", "crates/foo/src/other.py"]);
    let root = PathBuf::from(".");
    let res_rs =
        resolve_import_target(&imp_rs, "crates/foo/src/lib.rs", &root, &files_rs, &stem_rs);
    assert!(
        res_rs.is_none(),
        ".rs imports must not swap to other extensions"
    );

    // .py import that does not exist on disk should return None
    let imp_py = make_python_import("./missing.py", None);
    let files_py: HashSet<&str> = build_file_set(&["modules/foo/test.py", "modules/foo/other.rs"]);
    let stem_py = build_stem_index(&["modules/foo/test.py", "modules/foo/other.rs"]);
    let res_py = resolve_import_target(&imp_py, "modules/foo/test.py", &root, &files_py, &stem_py);
    assert!(
        res_py.is_none(),
        ".py imports must not swap to other extensions"
    );
}

#[test]
fn test_dts_declaration_import_does_not_swap() {
    // Bug 4: `.d.ts` declaration file imports must not be swapped to `.d.tsx`, `.d.js`, etc.
    let _orch = make_orchestrator();
    let imp = make_typescript_import("src/main.ts", "./types.d.ts");
    let files: HashSet<&str> =
        build_file_set(&["src/main.ts", "src/types.d.tsx", "src/types.d.js"]);
    let stem_index = build_stem_index(&["src/main.ts", "src/types.d.tsx", "src/types.d.js"]);
    let root = PathBuf::from(".");
    let result = resolve_import_target(&imp, "src/main.ts", &root, &files, &stem_index);
    assert!(
        result.is_none(),
        ".d.ts imports must not be swapped to other extensions"
    );
}

#[test]
fn test_directory_with_dot_in_name_still_gets_extension_added() {
    // Regression: `normalized.contains('.')` was true for `./lib.v2/foo` due to the dot
    // in `lib.v2`, skipping extension addition. Path::extension().is_none() returns true
    // for the final segment `foo`.
    let _orch = make_orchestrator();
    let imp = make_typescript_import("packages/client/src/index.ts", "./lib.v2/foo");
    let files: HashSet<&str> = build_file_set(&[
        "packages/client/src/index.ts",
        "packages/client/src/lib.v2/foo.ts",
    ]);
    let stem_index = build_stem_index(&[
        "packages/client/src/index.ts",
        "packages/client/src/lib.v2/foo.ts",
    ]);
    let root = PathBuf::from(".");
    let result = resolve_import_target(
        &imp,
        "packages/client/src/index.ts",
        &root,
        &files,
        &stem_index,
    );
    assert_eq!(
        result,
        Some("packages/client/src/lib.v2/foo.ts".to_string()),
        "directory with dot in name must still receive extension addition"
    );
}

#[test]
fn test_dotted_directory_with_js_extension_swap() {
    // Suggestion 3: Combined dotted directory + explicit `.js` extension swap.
    let _orch = make_orchestrator();
    let imp = make_typescript_import("packages/client/src/index.ts", "./lib.v2/container.js");
    let files: HashSet<&str> = build_file_set(&[
        "packages/client/src/index.ts",
        "packages/client/src/lib.v2/container.ts",
    ]);
    let stem_index = build_stem_index(&[
        "packages/client/src/index.ts",
        "packages/client/src/lib.v2/container.ts",
    ]);
    let root = PathBuf::from(".");
    let result = resolve_import_target(
        &imp,
        "packages/client/src/index.ts",
        &root,
        &files,
        &stem_index,
    );
    assert_eq!(
        result,
        Some("packages/client/src/lib.v2/container.ts".to_string()),
        "dotted directory path with .js specifier must swap to .ts"
    );
}

#[test]
fn test_root_level_extension_swap_resolves() {
    // Suggestion 3: Root-level (`src_dir == ""`) extension swapping: `main.ts` imports `./container.js`
    // resolving to root `container.ts`.
    let _orch = make_orchestrator();
    let imp = make_typescript_import("main.ts", "./container.js");
    let files: HashSet<&str> = build_file_set(&["main.ts", "container.ts"]);
    let stem_index = build_stem_index(&["main.ts", "container.ts"]);
    let root = PathBuf::from(".");
    let result = resolve_import_target(&imp, "main.ts", &root, &files, &stem_index);
    assert_eq!(
        result,
        Some("container.ts".to_string()),
        "root-level .js import must swap to .ts on disk"
    );
}

#[test]
fn test_unresolvable_relative_import_returns_none() {
    let _orch = make_orchestrator();
    let imp = make_typescript_import("packages/client/src/index.ts", "./missing_file.js");
    let files: HashSet<&str> = build_file_set(&["packages/client/src/index.ts"]);
    let stem_index = build_stem_index(&["packages/client/src/index.ts"]);
    let root = PathBuf::from(".");
    let result = resolve_import_target(
        &imp,
        "packages/client/src/index.ts",
        &root,
        &files,
        &stem_index,
    );
    assert!(
        result.is_none(),
        "unresolvable relative import must return None"
    );
}

#[test]
fn e2e_js_specifier_import_graph_reaches_target_files() {
    // End-to-end: TypeScript ESM project where modules import each other using
    // explicit `.js` specifiers pointing to `.ts` files on disk.
    let tmp = tempfile::TempDir::new().unwrap();
    let root = tmp.path();
    let packages_dir = root.join("packages");
    let client_src = packages_dir.join("client").join("src");
    std::fs::create_dir_all(&client_src).unwrap();

    let entry_rel = "packages/root_web_main_entry.ts";
    let container_rel = "packages/root_web_container.ts";
    let client_container_rel = "packages/client/src/root_client_container.ts";
    let orchestrator_rel = "packages/client/src/agent_client_orchestrator.ts";

    std::fs::write(
        root.join(entry_rel),
        "import { RootWebContainer } from './root_web_container.js';\nnew RootWebContainer().run();\n",
    )
    .unwrap();
    std::fs::write(
        root.join(container_rel),
        "import { RootClientContainer } from './client/src/root_client_container.js';\nexport class RootWebContainer { run() { new RootClientContainer().start(); } }\n",
    )
    .unwrap();
    std::fs::write(
        root.join(client_container_rel),
        "import { AgentClientOrchestrator } from './agent_client_orchestrator.js';\nexport class RootClientContainer { start() { new AgentClientOrchestrator().execute(); } }\n",
    )
    .unwrap();
    std::fs::write(
        root.join(orchestrator_rel),
        "export class AgentClientOrchestrator { execute() {} }\n",
    )
    .unwrap();

    let orch = make_orchestrator();
    let context = orch
        .execute(FilesystemRequest::build_orphan_graph_context(root, &[]))
        .into_graph_context();

    let graph = &context.import_graph.mapping;
    assert!(
        graph
            .get(entry_rel)
            .map(|v| v.contains(&container_rel.to_string()))
            .unwrap_or(false),
        "entry → root_web_container edge missing: {:?}",
        graph
    );
    assert!(
        graph
            .get(container_rel)
            .map(|v| v.contains(&client_container_rel.to_string()))
            .unwrap_or(false),
        "root_web_container → root_client_container edge missing: {:?}",
        graph
    );
    assert!(
        graph
            .get(client_container_rel)
            .map(|v| v.contains(&orchestrator_rel.to_string()))
            .unwrap_or(false),
        "root_client_container → agent_client_orchestrator edge missing: {:?}",
        graph
    );
}
