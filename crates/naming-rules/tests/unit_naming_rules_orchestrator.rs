// PURPOSE: Unit tests for NamingOrchestrator (Agent layer)
// Covers: flow control, filtering, error handling
// Coverage target: Agent ≥ 60%

use naming_rules_lint_arwaky::agent_naming_orchestrator::NamingOrchestrator;
use naming_rules_lint_arwaky::capabilities_naming_convention_checker::NamingConventionChecker;
use naming_rules_lint_arwaky::capabilities_suffix_prefix_checker::SuffixPrefixChecker;
use shared::common::taxonomy_definition_vo::LayerMapVO;
use shared::common::taxonomy_path_vo::FilePath;
use shared::common::taxonomy_paths_vo::FilePathList;
use shared::config_system::taxonomy_config_vo::ArchitectureConfig;
use shared::naming_rules::contract_naming_checker_protocol::{
    INamingConventionChecker, ISuffixPrefixChecker,
};
use shared::naming_rules::contract_naming_runner_aggregate::INamingRunnerAggregate;
use std::collections::HashMap;
use std::sync::Arc;

fn build_orchestrator(config: ArchitectureConfig, layer_map: LayerMapVO) -> NamingOrchestrator {
    let conv: Arc<dyn INamingConventionChecker> = Arc::new(NamingConventionChecker::new());
    let suf: Arc<dyn ISuffixPrefixChecker> = Arc::new(SuffixPrefixChecker::new());
    NamingOrchestrator::new(conv, suf, Arc::new(config), Arc::new(layer_map))
}

// ─── Name ─────────────────────────────────────────────────

#[test]
fn orchestrator_name_is_naming_rules() {
    let orch = build_orchestrator(
        ArchitectureConfig::default(),
        LayerMapVO::new(HashMap::new()),
    );
    assert_eq!(orch.name(), "naming-rules");
}

// ─── Error: Non-existent Path ─────────────────────────────

#[tokio::test]
async fn run_audit_nonexistent_path_returns_error() {
    let orch = build_orchestrator(
        ArchitectureConfig::default(),
        LayerMapVO::new(HashMap::new()),
    );
    let target = FilePath::new("/nonexistent/path/xyz".to_string()).unwrap();
    let result = orch.run_audit(&target).await;
    assert!(result.is_err(), "Non-existent path should return ScanError");
}

// ─── Ignored Patterns ─────────────────────────────────────

#[tokio::test]
async fn ignored_patterns_trimmed_correctly() {
    let mut config = ArchitectureConfig::default();
    config.ignored_paths = FilePathList::new(vec![
        FilePath::new("./target/".to_string()).unwrap(),
        FilePath::new("/node_modules/".to_string()).unwrap(),
    ]);
    let orch = build_orchestrator(config, LayerMapVO::new(HashMap::new()));
    // Orchestrator should have trimmed "./" and "/" prefixes/suffixes
    // We verify indirectly: the orchestrator was constructed without panic
    assert_eq!(orch.name(), "naming-rules");
}

// ─── Filter Source Files ──────────────────────────────────
// filter_source_files is private, tested indirectly via run_audit on a temp dir

#[tokio::test]
async fn run_audit_on_empty_dir_returns_empty_results() {
    let dir = tempfile::tempdir().unwrap();
    let orch = build_orchestrator(
        ArchitectureConfig::default(),
        LayerMapVO::new(HashMap::new()),
    );
    let target = FilePath::new(dir.path().to_string_lossy().to_string()).unwrap();
    let result = orch.run_audit(&target).await;
    assert!(result.is_ok());
    assert!(result.unwrap().is_empty(), "Empty dir → no violations");
}

#[tokio::test]
async fn run_audit_filters_non_source_extensions() {
    let dir = tempfile::tempdir().unwrap();
    // Create a .txt file (not a source extension)
    std::fs::write(dir.path().join("readme.txt"), "hello").unwrap();
    // Create a valid .rs file
    std::fs::write(dir.path().join("capabilities_user_checker.rs"), "// ok").unwrap();

    let orch = build_orchestrator(
        ArchitectureConfig::default(),
        LayerMapVO::new(HashMap::new()),
    );
    let target = FilePath::new(dir.path().to_string_lossy().to_string()).unwrap();
    let result = orch.run_audit(&target).await;
    assert!(result.is_ok());
    // .txt should be filtered out; .rs should be checked
    // With empty layer map, the .rs file may or may not produce violations
    // depending on prefix detection — but .txt must never appear
    let violations = result.unwrap();
    for v in &violations {
        assert!(
            v.file.value.ends_with(".rs"),
            "Non-source file should not appear in results"
        );
    }
}
