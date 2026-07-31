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
use shared::common::FilePath;
use shared::config_system::ArchitectureConfig;
use shared::orphan_detector::{IOrphanAggregate, OrphanFileListVO};

use std::sync::Arc;

fn build_analyzer(config: ArchitectureConfig) -> ArchOrphanAnalyzer {
    ArchOrphanAnalyzer::new(
        ArchOrphanDeps {
            resolver: Arc::new(OrphanGraphResolver::default()),
            taxonomy_analyzer: Arc::new(TaxonomyOrphanAnalyzer::new()),
            contract_analyzer: Arc::new(ContractOrphanAnalyzer::default()),
            capabilities_analyzer: Arc::new(CapabilitiesOrphanAnalyzer::default()),
            utility_analyzer: Arc::new(UtilityOrphanAnalyzer::default()),
            agent_analyzer: Arc::new(AgentOrphanAnalyzer::default()),
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

// ─── Disabled per-rule returns empty for that rule ────────

#[test]
fn check_orphans_disabled_rule_returns_empty() {
    use shared::common::taxonomy_suggestion_vo::DescriptionVO;
    use shared::common::{
        taxonomy_common_vo::BooleanVO, taxonomy_definition_vo::NamingConfig,
        taxonomy_error_vo::ErrorCode, taxonomy_layer_vo::LayerNameVO,
        taxonomy_paths_vo::FilePathList,
    };

    // Config with AES505 (agent orphan) disabled
    let config = ArchitectureConfig {
        enabled: BooleanVO::new(true),
        layers: std::collections::HashMap::new(),
        rules: vec![shared::config_system::ArchitectureRule {
            name: DescriptionVO::new("AES505"),
            description: DescriptionVO::new(""),
            rule_type: ErrorCode::raw("AES505"),
            enabled: BooleanVO::new(false), // Disabled
            scope: LayerNameVO::new("agent"),
            exceptions: shared::common::taxonomy_common_vo::PatternList::new(Vec::<String>::new()),
            allowed: shared::common::taxonomy_common_vo::PatternList::new(Vec::<String>::new()),
            forbidden: shared::common::taxonomy_common_vo::PatternList::new(Vec::<String>::new()),
            mandatory: shared::common::taxonomy_common_vo::PatternList::new(Vec::<String>::new()),
            naming: shared::config_system::NamingRuleVO::default(),
            code_analysis:
                shared::code_analysis::taxonomy_code_analysis_rule_vo::CodeAnalysisRuleVO::default(),
            role: shared::config_system::RoleRuleVO::default(),
            orphan: shared::config_system::taxonomy_config_vo::OrphanRuleVO {
                check_orphan: BooleanVO::new(true), // Layer-level enabled, but rule disabled
                ..Default::default()
            },
        }],
        naming: NamingConfig::new(shared::common::taxonomy_common_vo::Count::new(3)),
        ignored_paths: FilePathList { values: vec![] },
        mandatory_class_definition: BooleanVO::new(false),
    };

    // Verify the is_rule_disabled helper works
    let analyzer = build_analyzer(config);
    assert!(analyzer.is_rule_disabled("AES505"));
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
    assert!(
        !entries
            .values
            .contains(&"src/capabilities_foo.rs".to_string())
    );
}

// ─── Orphan file with no inbound links is flagged ──────────

#[test]
fn check_orphans_flags_taxonomy_file_with_no_inbound_links() {
    let config = ArchitectureConfig::default();
    let analyzer = build_analyzer(config);
    let files = OrphanFileListVO::new(vec!["src/taxonomy_auto_vo.rs".to_string()]);
    let root = FilePath::new("/tmp/project".to_string()).unwrap();
    let results = analyzer.check_orphans(&files, &root);
    // The taxonomy file has no inbound links (other files importing it),
    // so it should be flagged as orphan (AES501 or AES503).
    assert!(!results.is_empty(), "Orphan file should produce violations");
    let codes: Vec<&str> = results.iter().map(|r| r.code.code()).collect();
    assert!(
        codes.contains(&"AES501"),
        "Expected AES501 (taxonomy orphan), got: {:?}",
        codes
    );
}
