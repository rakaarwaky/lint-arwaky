// PURPOSE: Unit tests for ArchOrphanAnalyzer — orchestration logic, reachability tracing, layer evaluation.
// Layer: Agent (ArchOrphanAnalyzer)
// Speed: ms

use orphan_detector_lint_arwaky::agent_orphan_orchestrator::{ArchOrphanAnalyzer, ArchOrphanDeps};
use orphan_detector_lint_arwaky::capabilities_orphan_agent_analyzer::AgentOrphanAnalyzer;
use orphan_detector_lint_arwaky::capabilities_orphan_capabilities_analyzer::CapabilitiesOrphanAnalyzer;
use orphan_detector_lint_arwaky::capabilities_orphan_contract_analyzer::ContractOrphanAnalyzer;
use orphan_detector_lint_arwaky::capabilities_orphan_graph_resolver::OrphanGraphResolver;
use orphan_detector_lint_arwaky::capabilities_orphan_surfaces_analyzer::SurfacesOrphanAnalyzer;
use orphan_detector_lint_arwaky::capabilities_orphan_taxonomy_analyzer::TaxonomyOrphanAnalyzer;
use orphan_detector_lint_arwaky::capabilities_orphan_utility_analyzer::UtilityOrphanAnalyzer;
use shared::common::taxonomy_path_vo::FilePath;
use shared::config_system::taxonomy_config_vo::ArchitectureConfig;
use shared::orphan_detector::contract_orphan_aggregate::IOrphanAggregate;
use shared::orphan_detector::taxonomy_orphan_contract_vo::OrphanFileListVO;
use std::sync::Arc;

fn build_analyzer(config: ArchitectureConfig) -> ArchOrphanAnalyzer {
    ArchOrphanAnalyzer::new(
        ArchOrphanDeps {
            resolver: Arc::new(OrphanGraphResolver::new()),
            taxonomy_analyzer: Arc::new(TaxonomyOrphanAnalyzer::new()),
            contract_analyzer: Arc::new(ContractOrphanAnalyzer::new()),
            capabilities_analyzer: Arc::new(CapabilitiesOrphanAnalyzer::new()),
            utility_analyzer: Arc::new(UtilityOrphanAnalyzer::new()),
            agent_analyzer: Arc::new(AgentOrphanAnalyzer::new()),
            surfaces_analyzer: Arc::new(SurfacesOrphanAnalyzer::new()),
        },
        config,
    )
}

// ─── Disabled config returns empty results ────────────────

#[test]
fn check_orphans_disabled_config_returns_empty() {
    let config = ArchitectureConfig {
        enabled: shared::common::taxonomy_common_vo::BooleanVO::new(false),
        ..Default::default()
    };
    let analyzer = build_analyzer(config);
    let files = OrphanFileListVO::new(vec!["src/lib.rs".to_string()]);
    let root = FilePath::new("/tmp/project".to_string()).unwrap();
    let results = analyzer.check_orphans(&files, &root);
    assert!(results.is_empty());
}

// ─── Empty file list returns empty results ────────────────

#[test]
fn check_orphans_empty_files_returns_empty() {
    let config = ArchitectureConfig::default();
    let analyzer = build_analyzer(config);
    let files = OrphanFileListVO::new(vec![]);
    let root = FilePath::new("/tmp/project".to_string()).unwrap();
    let results = analyzer.check_orphans(&files, &root);
    assert!(results.is_empty());
}

// ─── build_orphan_graph_context ───────────────────────────

#[test]
fn build_orphan_graph_context_returns_valid_context() {
    let config = ArchitectureConfig::default();
    let analyzer = build_analyzer(config);
    let files = OrphanFileListVO::new(vec!["src/lib.rs".to_string()]);
    let root = FilePath::new("/tmp/project".to_string()).unwrap();
    let ctx = analyzer.build_orphan_graph_context(&files, &root);
    assert!(ctx.import_graph.mapping.contains_key("src/lib.rs"));
}

// ─── identify_orphan_entry_points ─────────────────────────

#[test]
fn identify_orphan_entry_points_finds_main_and_lib() {
    let config = ArchitectureConfig::default();
    let analyzer = build_analyzer(config);
    let files = OrphanFileListVO::new(vec![
        "src/main.rs".to_string(),
        "src/lib.rs".to_string(),
        "src/capabilities_foo.rs".to_string(),
    ]);
    let entries = analyzer.identify_orphan_entry_points(&files);
    assert!(entries.values.contains(&"src/main.rs".to_string()));
    assert!(entries.values.contains(&"src/lib.rs".to_string()));
    assert!(!entries
        .values
        .contains(&"src/capabilities_foo.rs".to_string()));
}

// ─── Ignored paths are filtered ───────────────────────────

#[test]
fn check_orphans_respects_ignored_paths() {
    let config = ArchitectureConfig {
        ignored_paths: shared::common::taxonomy_paths_vo::FilePathList::new(vec![
            shared::common::taxonomy_path_vo::FilePath::new("src/generated".to_string()).unwrap(),
        ]),
        ..Default::default()
    };
    let analyzer = build_analyzer(config);
    let files = OrphanFileListVO::new(vec!["src/generated/taxonomy_auto_vo.rs".to_string()]);
    let root = FilePath::new("/tmp/project".to_string()).unwrap();
    let results = analyzer.check_orphans(&files, &root);
    assert!(results.is_empty());
}
