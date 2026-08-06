// Smoke tests — quick boot and respond within time budget.
#[path = "mock_filesystem.rs"]
mod mock_filesystem;

use mock_filesystem::mock_filesystem;
use orphan_rules_lint_arwaky::agent_orphan_orchestrator::{ArchOrphanAnalyzer, ArchOrphanDeps};
use orphan_rules_lint_arwaky::capabilities_orphan_agent_analyzer::AgentOrphanAnalyzer;
use orphan_rules_lint_arwaky::capabilities_orphan_capabilities_analyzer::CapabilitiesOrphanAnalyzer;
use orphan_rules_lint_arwaky::capabilities_orphan_contract_analyzer::ContractOrphanAnalyzer;
use orphan_rules_lint_arwaky::capabilities_orphan_surfaces_analyzer::SurfacesOrphanAnalyzer;
use orphan_rules_lint_arwaky::capabilities_orphan_taxonomy_analyzer::TaxonomyOrphanAnalyzer;
use orphan_rules_lint_arwaky::capabilities_orphan_utility_analyzer::UtilityOrphanAnalyzer;
use orphan_rules_lint_arwaky::root_orphan_detector_container::OrphanContainer;
use shared::common::taxonomy_path_vo::FilePath;
use shared::config_system::ArchitectureConfig;
use shared::orphan_rules::{IOrphanAggregate, OrphanFileListVO};
use std::sync::Arc;

#[test]
fn smoke_container_creation() {
    let fs = mock_filesystem();
    let container = OrphanContainer::new(fs);
    let analyzer = container.analyzer();
    assert!(Arc::strong_count(&analyzer) >= 1);
}

#[test]
fn smoke_analyzer_returns_quickly() {
    let start = std::time::Instant::now();
    let fs = mock_filesystem();
    let container = OrphanContainer::new(fs);
    let analyzer = container.analyzer();
    let files = OrphanFileListVO::new(vec![
        "src/taxonomy_foo.rs".to_string(),
        "src/contract_bar_protocol.rs".to_string(),
    ]);
    let root = FilePath::new(".".to_string()).unwrap();
    let results = analyzer.check_orphans(&files, &root);
    let elapsed = start.elapsed();
    assert!(
        elapsed.as_secs() < 5,
        "Smoke test took too long: {:?}",
        elapsed
    );
    // Mock returns empty graph, so no orphans detected from mock
    drop(results);
}

#[test]
fn smoke_individual_analyzers_construct() {
    let _ = TaxonomyOrphanAnalyzer::new();
    let _ = UtilityOrphanAnalyzer::new();
    let _ = AgentOrphanAnalyzer::new();
    let _ = SurfacesOrphanAnalyzer::new();
}

#[test]
fn smoke_contract_analyzer_construct() {
    let _analyzer = ContractOrphanAnalyzer::new();
}

#[test]
fn smoke_capabilities_analyzer_construct() {
    let fs = mock_filesystem();
    let _analyzer = CapabilitiesOrphanAnalyzer::new(fs);
}

#[test]
fn smoke_arch_analyzer_construct() {
    let fs = mock_filesystem();
    let deps = ArchOrphanDeps {
        taxonomy_analyzer: Arc::new(TaxonomyOrphanAnalyzer::new()),
        contract_analyzer: Arc::new(ContractOrphanAnalyzer::new()),
        capabilities_analyzer: Arc::new(CapabilitiesOrphanAnalyzer::new(fs.clone())),
        utility_analyzer: Arc::new(UtilityOrphanAnalyzer::new()),
        agent_analyzer: Arc::new(AgentOrphanAnalyzer::new()),
        surfaces_analyzer: Arc::new(SurfacesOrphanAnalyzer::new()),
        filesystem: fs,
    };
    let config = ArchitectureConfig::default();
    let analyzer = ArchOrphanAnalyzer::new(deps, config);
    let files = OrphanFileListVO::new(vec![]);
    let root = FilePath::new(".".to_string()).unwrap();
    let results = analyzer.check_orphans(&files, &root);
    assert!(results.is_empty());
}
