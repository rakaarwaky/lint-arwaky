// Regression tests — #920: build_orphan_graph_context must honor config ignored paths.
//
// `build_orphan_graph_context(root, ignored)` previously called
// `build_file_index(root)` (defaults only), so any caller passing config
// `ignored_paths` silently got no exclusion. The sibling method
// `build_file_index_with_ignored` honours the extra patterns; the fix routes
// through it. This test exercises the full aggregate path
// (`FilesystemRequest::build_orphan_graph_context`) so a regression that
// reintroduces the defaults-only call fails here.

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

use std::sync::Arc;

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

// Layout inside the temp dir:
//
//   <tmp>/Cargo.toml            (empty file is enough for the walk-up detector)
//   <tmp>/crates/               (member directory — makes the walk-up detector
//                                 stop here instead of going up into the real
//                                 repo)
//   <tmp>/crates/mycrate/src/lib.rs
//   <tmp>/generated/code.py     (must be excluded when "generated" is in ignored)
//
// `build_file_index_impl` scans a workspace root that it detects by walking up
// until it finds one of `crates` / `packages` / `modules` next to a
// `Cargo.toml`.  A temp dir with `crates/` satisfies that on its own, and the
// `member_dirs` filter (non-empty because `crates` exists) restricts the scan
// to paths under `crates/`, so `generated/` is never scanned unless the
// ignore-list logic is broken.

#[test]
fn regression_920_build_orphan_graph_context_honors_ignored_paths() {
    let tmp = tempfile::tempdir().unwrap();

    // Workspace marker so the walk-up detector stops here.
    std::fs::write(tmp.path().join("Cargo.toml"), "").unwrap();

    // Member crate with one real source file.
    let crate_src = tmp.path().join("crates").join("mycrate").join("src");
    std::fs::create_dir_all(&crate_src).unwrap();
    std::fs::write(crate_src.join("lib.rs"), "pub fn entry() -> i32 { 42 }\n").unwrap();

    // A generated file that must be excluded when "generated" is ignored.
    let generated_dir = tmp.path().join("generated");
    std::fs::create_dir_all(&generated_dir).unwrap();
    std::fs::write(
        generated_dir.join("code.py"),
        "def generated_fn():\n    pass\n",
    )
    .unwrap();

    let orch = make_orchestrator();
    let root = tmp.path().to_path_buf();
    let ignored: Vec<String> = vec!["generated".to_string()];

    let context = orch
        .execute(FilesystemRequest::build_orphan_graph_context(
            &root, &ignored,
        ))
        .into_graph_context();

    // The real member crate file must still be present.
    assert!(
        context
            .all_workspace_files
            .iter()
            .any(|f| f.ends_with("mycrate/src/lib.rs") || f.contains("mycrate/src/lib.rs")),
        "expected mycrate/src/lib.rs in workspace files, got: {:?}",
        context.all_workspace_files,
    );

    // No file under `generated/` may appear.
    assert!(
        !context
            .all_workspace_files
            .iter()
            .any(|f| f.contains("generated/")),
        "files under generated/ must be excluded when 'generated' is in ignored, got: {:?}",
        context.all_workspace_files,
    );

    // The import graph must not carry an edge whose source or target is a
    // generated file either.
    for (src, targets) in &context.import_graph.mapping {
        assert!(
            !src.contains("generated/"),
            "import-graph source {:?} should not be a generated path",
            src,
        );
        for tgt in targets {
            assert!(
                !tgt.contains("generated/"),
                "import-graph target {:?} should not be a generated path",
                tgt,
            );
        }
    }
}

// Second regression guard: without the "generated" entry in ignored, the
// temp-dir layout does not put generated/ under a member directory, so it
// was never picked up by the member filter.  This test confirms that the
// defaults-only path still sees the member crate file (no regression in the
// opposite direction).
#[test]
fn regression_920_defaults_only_still_sees_member_crate_files() {
    let tmp = tempfile::tempdir().unwrap();

    std::fs::write(tmp.path().join("Cargo.toml"), "").unwrap();
    let crate_src = tmp.path().join("crates").join("mycrate").join("src");
    std::fs::create_dir_all(&crate_src).unwrap();
    std::fs::write(crate_src.join("lib.rs"), "pub fn entry() -> i32 { 42 }\n").unwrap();

    let orch = make_orchestrator();
    let root = tmp.path().to_path_buf();

    let context = orch
        .execute(FilesystemRequest::build_orphan_graph_context(&root, &[]))
        .into_graph_context();

    assert!(
        context
            .all_workspace_files
            .iter()
            .any(|f| f.contains("mycrate/src/lib.rs")),
        "defaults-only call must still see the member crate file, got: {:?}",
        context.all_workspace_files,
    );
}
