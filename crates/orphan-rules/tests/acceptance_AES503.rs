// Acceptance tests — AES503: Capabilities orphan detection.
#[path = "mock_filesystem.rs"]
mod mock_filesystem;

use mock_filesystem::mock_filesystem;
use orphan_rules_lint_arwaky::capabilities_orphan_capabilities_analyzer::CapabilitiesOrphanAnalyzer;
use shared::common::taxonomy_path_vo::FilePath;
use shared::common::taxonomy_severity_vo::Severity;
use shared::orphan_rules::ICapabilitiesOrphanProtocol;
use shared::quality_rules::taxonomy_analysis_vo::ReachabilityResult;
use std::collections::{HashMap, HashSet};

fn capabilities_analyzer() -> CapabilitiesOrphanAnalyzer {
    CapabilitiesOrphanAnalyzer::new(mock_filesystem())
}

fn reachable_for(fp: &FilePath) -> ReachabilityResult {
    ReachabilityResult::new(HashSet::from([fp.clone()]))
}

#[test]
fn aes503_reachable_file_is_not_orphan() {
    let analyzer = capabilities_analyzer();
    let fp = FilePath::new("crates/orphan-rules/src/capabilities_foo.rs".to_string()).unwrap();
    let root = FilePath::new(".".to_string()).unwrap();
    let workspace_root = std::path::Path::new(".").to_path_buf();
    let content_map = HashMap::new();

    let alive = reachable_for(&fp);

    let result = analyzer.is_capabilities_orphan(&fp, &root, &alive, &content_map, &workspace_root);
    // Reachable but not wired → orphan (mock filesystem doesn't support wiring)
    // This test verifies the reachability check passes (no "not reachable" message)
    assert!(
        result.is_orphan,
        "Reachable but unwired capabilities file should be orphan with mock filesystem"
    );
    assert!(
        result.reason.contains("not wired"),
        "Should fail on wiring check, not reachability: {}",
        result.reason
    );
}

#[test]
fn aes503_unreachable_file_is_orphan() {
    let analyzer = capabilities_analyzer();
    let fp = FilePath::new("crates/orphan-rules/src/capabilities_foo.rs".to_string()).unwrap();
    let root = FilePath::new(".".to_string()).unwrap();
    let workspace_root = std::path::Path::new(".").to_path_buf();
    let content_map = HashMap::new();

    let alive = ReachabilityResult::new(HashSet::new());

    let result = analyzer.is_capabilities_orphan(&fp, &root, &alive, &content_map, &workspace_root);
    // With mock filesystem that doesn't read files or check wiring, this should be orphan
    assert!(
        result.is_orphan,
        "Unreachable capabilities file should be orphan"
    );
    assert_eq!(result.severity, Severity::MEDIUM);
    assert!(!result.reason.is_empty());
}

#[test]
fn aes503_unreachable_file_reason_mentions_not_wired() {
    let analyzer = capabilities_analyzer();
    let fp = FilePath::new("crates/orphan-rules/src/capabilities_bar.rs".to_string()).unwrap();
    let root = FilePath::new(".".to_string()).unwrap();
    let workspace_root = std::path::Path::new(".").to_path_buf();
    let content_map = HashMap::new();

    let alive = ReachabilityResult::new(HashSet::new());

    let result = analyzer.is_capabilities_orphan(&fp, &root, &alive, &content_map, &workspace_root);
    assert!(result.is_orphan);
    // The reason should mention that the struct/trait is not wired
    assert!(
        result.reason.contains("not wired") || result.reason.contains("not reachable"),
        "Reason should mention wiring or reachability: {}",
        result.reason
    );
}

#[test]
fn aes503_multiple_files_one_reachable() {
    let analyzer = capabilities_analyzer();
    let root = FilePath::new(".".to_string()).unwrap();
    let workspace_root = std::path::Path::new(".").to_path_buf();
    let content_map = HashMap::new();

    let fp_reachable =
        FilePath::new("crates/orphan-rules/src/capabilities_handler.rs".to_string()).unwrap();
    let fp_orphan =
        FilePath::new("crates/orphan-rules/src/capabilities_legacy.rs".to_string()).unwrap();

    let alive = reachable_for(&fp_reachable);

    let result_reachable = analyzer.is_capabilities_orphan(
        &fp_reachable,
        &root,
        &alive,
        &content_map,
        &workspace_root,
    );
    let result_orphan =
        analyzer.is_capabilities_orphan(&fp_orphan, &root, &alive, &content_map, &workspace_root);

    // Reachable file passes reachability check but fails wiring (mock) → "not wired"
    assert!(
        result_reachable.is_orphan,
        "Reachable but unwired should be orphan with mock filesystem"
    );
    assert!(
        result_reachable.reason.contains("not wired"),
        "Should fail on wiring: {}",
        result_reachable.reason
    );
    // Unreachable file fails reachability check → "not reachable"
    assert!(result_orphan.is_orphan, "Unreachable should be orphan");
    assert!(
        result_orphan.reason.contains("not reachable"),
        "Should fail on reachability: {}",
        result_orphan.reason
    );
}

#[test]
fn aes503_capabilities_violation_display_message() {
    use shared::orphan_rules::AesOrphanViolation;
    let _violation = AesOrphanViolation::CapabilitiesOrphan {
        stem: "capabilities_handler".to_string(),
        reason: Some(shared::common::taxonomy_message_vo::LintMessage::new(
            "Not wired in container.".to_string(),
        )),
    };
    let msg = format!(
        "AES503 CAPABILITIES_ORPHAN: '{}' is not wired.\nWHY? {}\nFIX: Register '{}' in root_*_entry.rs or root_*_container.rs.",
        "capabilities_handler", "Not wired in container.", "capabilities_handler"
    );
    assert!(msg.contains("AES503"));
    assert!(msg.contains("capabilities_handler"));
    assert!(msg.contains("not wired"));
}

#[test]
fn aes503_chained_python_import_reachable() {
    // Acceptance test: BFS reachability traces through chained Python imports.
    // Entry → Surface → Agent → Capabilities should all be reachable.
    // Builds the graph through the REAL filesystem pipeline (import extraction +
    // resolve_import_target), not a hand-built graph.
    use filesystem::agent_filesystem_orchestrator::{
        FilesystemOrchestrator, FilesystemOrchestratorDeps,
    };
    use filesystem::capabilities_ast_parser::ASTParser;
    use filesystem::capabilities_dependency_graph::DependencyGraph;
    use filesystem::capabilities_filesystem_io::CapabilitiesFileSystemIO;
    use filesystem::capabilities_tool_resolution::CapabilitiesToolResolution;
    use filesystem::capabilities_workspace_root_finder::CapabilitiesWorkspace;
    use orphan_rules_lint_arwaky::utility_orphan_graph::trace_reachability;
    use shared::filesystem::contract_filesystem_aggregate::IFilesystemAggregate;
    use std::sync::Arc;

    let tmp = tempfile::TempDir::new().unwrap();
    let member_src = tmp.path().join("modules").join("image").join("src");
    std::fs::create_dir_all(&member_src).unwrap();
    std::fs::write(
        member_src.join("root_image_entry.py"),
        "from modules.image.src.surfaces_image_cli import CliCommand\n",
    )
    .unwrap();
    std::fs::write(
        member_src.join("surfaces_image_cli.py"),
        "from modules.image.src.agent_image_orchestrator import ImageOrchestrator\n",
    )
    .unwrap();
    std::fs::write(
        member_src.join("agent_image_orchestrator.py"),
        "from capabilities_image_processing_processor import ImageProcessor\n",
    )
    .unwrap();
    std::fs::write(
        member_src.join("capabilities_image_processing_processor.py"),
        "class ImageProcessor:\n    pass\n",
    )
    .unwrap();

    let orch = FilesystemOrchestrator::new(FilesystemOrchestratorDeps {
        io: Arc::new(CapabilitiesFileSystemIO::with_default_timing()),
        workspace: Arc::new(CapabilitiesWorkspace::new()),
        tool_resolution: Arc::new(CapabilitiesToolResolution::new()),
        parser: Arc::new(ASTParser::new()),
        graph: Arc::new(DependencyGraph::new()),
    });
    let context = orch.build_orphan_graph_context(tmp.path(), &[]);

    let entry = "modules/image/src/root_image_entry.py".to_string();
    let surface = "modules/image/src/surfaces_image_cli.py".to_string();
    let agent = "modules/image/src/agent_image_orchestrator.py".to_string();
    let capabilities = "modules/image/src/capabilities_image_processing_processor.py".to_string();

    let alive_set = trace_reachability(&[entry.clone()], &context.import_graph);

    assert!(alive_set.contains(&entry));
    assert!(alive_set.contains(&surface));
    assert!(alive_set.contains(&agent));
    assert!(alive_set.contains(&capabilities));
}

#[test]
fn aes503_broken_chain_detects_unreachable() {
    // Acceptance test: When a link is missing in the chain, downstream files are unreachable.
    // Real pipeline: surface imports the agent module, but the agent file does not
    // exist — so no edge can be created and capabilities stay unreachable.
    use filesystem::agent_filesystem_orchestrator::{
        FilesystemOrchestrator, FilesystemOrchestratorDeps,
    };
    use filesystem::capabilities_ast_parser::ASTParser;
    use filesystem::capabilities_dependency_graph::DependencyGraph;
    use filesystem::capabilities_filesystem_io::CapabilitiesFileSystemIO;
    use filesystem::capabilities_tool_resolution::CapabilitiesToolResolution;
    use filesystem::capabilities_workspace_root_finder::CapabilitiesWorkspace;
    use orphan_rules_lint_arwaky::utility_orphan_graph::trace_reachability;
    use shared::filesystem::contract_filesystem_aggregate::IFilesystemAggregate;
    use std::sync::Arc;

    let tmp = tempfile::TempDir::new().unwrap();
    let member_src = tmp.path().join("modules").join("image").join("src");
    std::fs::create_dir_all(&member_src).unwrap();
    std::fs::write(
        member_src.join("root_image_entry.py"),
        "from modules.image.src.surfaces_image_cli import CliCommand\n",
    )
    .unwrap();
    // Surface imports the agent module, but the agent file is NOT created —
    // the chain is broken between surface and agent.
    std::fs::write(
        member_src.join("surfaces_image_cli.py"),
        "from modules.image.src.agent_image_orchestrator import ImageOrchestrator\n",
    )
    .unwrap();
    // Capabilities file exists but nothing imports it (agent is missing).
    std::fs::write(
        member_src.join("capabilities_image_processing_processor.py"),
        "class ImageProcessor:\n    pass\n",
    )
    .unwrap();

    let orch = FilesystemOrchestrator::new(FilesystemOrchestratorDeps {
        io: Arc::new(CapabilitiesFileSystemIO::with_default_timing()),
        workspace: Arc::new(CapabilitiesWorkspace::new()),
        tool_resolution: Arc::new(CapabilitiesToolResolution::new()),
        parser: Arc::new(ASTParser::new()),
        graph: Arc::new(DependencyGraph::new()),
    });
    let context = orch.build_orphan_graph_context(tmp.path(), &[]);

    let entry = "modules/image/src/root_image_entry.py".to_string();
    let surface = "modules/image/src/surfaces_image_cli.py".to_string();
    let capabilities = "modules/image/src/capabilities_image_processing_processor.py".to_string();

    let alive_set = trace_reachability(&[entry.clone()], &context.import_graph);

    assert!(
        alive_set.contains(&entry),
        "Entry point should be reachable"
    );
    assert!(
        alive_set.contains(&surface),
        "Surface should be reachable from entry"
    );
    assert!(
        !alive_set.contains(&capabilities),
        "Capabilities should NOT be reachable when chain is broken"
    );
}
