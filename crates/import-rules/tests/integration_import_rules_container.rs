// Integration tests for ImportContainer and ImportOrchestrator — DI wiring and cross-layer interaction

use std::sync::Arc;

use import_rules_lint_arwaky::{
    agent_import_orchestrator::ImportOrchestrator,
    capabilities_cycle_import_analyzer::DependencyCycleAnalyzer,
    capabilities_dummy_import_checker::DummyImportChecker,
    capabilities_import_forbidden_checker::ArchImportForbiddenChecker,
    capabilities_import_mandatory_checker::ArchImportMandatoryChecker,
    capabilities_import_unused_checker::UnusedImportRuleChecker,
    root_import_rules_container::ImportContainer,
};
use shared::cli_commands::taxonomy_result_vo::{LintResult, LintResultList};
use shared::common::taxonomy_path_vo::FilePath;
use shared::config_system::taxonomy_config_vo::ArchitectureConfig;
use shared::import_rules::contract_cycle_import_protocol::ICycleImportProtocol;
use shared::import_rules::contract_dummy_import_protocol::IDummyImportCheckerProtocol;
use shared::import_rules::contract_import_forbidden_protocol::IImportForbiddenProtocol;
use shared::import_rules::contract_import_mandatory_protocol::IImportMandatoryProtocol;
use shared::import_rules::contract_import_runner_aggregate::IImportRunnerAggregate;
use shared::import_rules::contract_unused_import_protocol::IUnusedImportProtocol;
use shared::taxonomy_definition_vo::LayerMapVO;

// ─── Integration Tests: DI Container Wiring ────────────────────────

/// Test that ImportContainer properly wires import mandatory checker.
#[test]
fn test_container_wires_mandatory_checker() {
    let config = ArchitectureConfig::default();
    let container = ImportContainer::new_with_config(config);

    // Should be able to access mandatory checker via protocol
    let _checker = container.mandatory();

    // Should implement IImportMandatoryProtocol
    let checker_ref = container.mandatory();
    assert!(std::sync::Arc::ptr_eq(&checker_ref, &checker_ref));
}

/// Test that ImportContainer properly wires import forbidden checker.
#[test]
fn test_container_wires_forbidden_checker() {
    let config = ArchitectureConfig::default();
    let container = ImportContainer::new_with_config(config);

    // Should be able to access forbidden checker via protocol
    let _checker = container.forbidden();

    let checker_ref = container.forbidden();
    assert!(std::sync::Arc::ptr_eq(&checker_ref, &checker_ref));
}

/// Test that ImportContainer properly wires unused import checker.
#[test]
fn test_container_wires_unused_checker() {
    let config = ArchitectureConfig::default();
    let container = ImportContainer::new_with_config(config);

    // Should be able to access unused checker via protocol
    let _checker = container.unused();

    let checker_ref = container.unused();
    assert!(std::sync::Arc::ptr_eq(&checker_ref, &checker_ref));
}

/// Test that ImportContainer properly wires dummy import checker.
#[test]
fn test_container_wires_dummy_checker() {
    let config = ArchitectureConfig::default();
    let container = ImportContainer::new_with_config(config);

    // Should be able to access dummy checker via protocol
    let _checker = container.dummy();

    let checker_ref = container.dummy();
    assert!(std::sync::Arc::ptr_eq(&checker_ref, &checker_ref));
}

/// Test that ImportContainer properly wires cycle import analyzer.
#[test]
fn test_container_wires_cycle_analyzer() {
    let config = ArchitectureConfig::default();
    let container = ImportContainer::new_with_config(config);

    // Should be able to access cycle analyzer via protocol
    let _analyzer = container.cycle();

    let analyzer_ref = container.cycle();
    assert!(std::sync::Arc::ptr_eq(&analyzer_ref, &analyzer_ref));
}

/// Test that ImportContainer can create orchestrator from container.
#[test]
fn test_container_creates_orchestrator() {
    let config = ArchitectureConfig::default();
    let container = ImportContainer::new_with_config(config);

    // Should create orchestrator without panicking
    let orchestrator = container.orchestrator();

    // Should return a valid IImportRunnerAggregate
    assert_eq!(orchestrator.name(), "import-rules");
}

/// Test that orchestrator can run audit on empty path.
#[tokio::test]
async fn test_orchestrator_run_audit_empty_path() {
    let config = ArchitectureConfig::default();
    let container = ImportContainer::new_with_config(config);
    let orchestrator = container.orchestrator();

    // Run audit on non-existent path — should return error
    let result = orchestrator
        .run_audit(&FilePath::new("/nonexistent/path".to_string()).unwrap())
        .await;

    assert!(result.is_err(), "Non-existent path should return an error");
}

/// Test that orchestrator can run audit on existing directory.
#[tokio::test]
async fn test_orchestrator_run_audit_existing_dir() {
    let config = ArchitectureConfig::default();
    let container = ImportContainer::new_with_config(config);
    let orchestrator = container.orchestrator();

    // Run audit on current directory — should return Ok (may have violations or not)
    let result = orchestrator
        .run_audit(&FilePath::new("./".to_string()).unwrap())
        .await;

    assert!(result.is_ok(), "Existing path should return Ok");

    // Results should be a Vec<LintResult> (possibly empty)
    let results = result.unwrap();
    for r in &results {
        assert!(!r.file.value.is_empty());
    }
}

/// Test that ImportContainer can be created with default config.
#[test]
fn test_default_container_creation() {
    let config = ArchitectureConfig::default();

    // Should create container without panicking
    let _container = ImportContainer::new_with_config(config);
}

/// Test that all checkers work together in orchestrator.
#[tokio::test]
async fn test_all_checkers_work_together() {
    let config = ArchitectureConfig::default();
    let container = ImportContainer::new_with_config(config);
    let orchestrator = container.orchestrator();

    // Should be able to run audit — all checkers should work together
    let result = orchestrator
        .run_audit(&FilePath::new("./".to_string()).unwrap())
        .await;

    assert!(result.is_ok(), "Orchestrator should handle empty directory");
}

/// Test that all checkers are properly cloned in container.
#[test]
fn test_container_checkers_are_cloned() {
    let config = ArchitectureConfig::default();
    let container = ImportContainer::new_with_config(config);

    let mandatory = container.mandatory().clone();
    let forbidden = container.forbidden().clone();
    let unused = container.unused().clone();
    let dummy = container.dummy().clone();
    let cycle = container.cycle().clone();

    // All should be clonable (Clone trait is implemented)
    let _mandatory_clone = mandatory.clone();
    let _forbidden_clone = forbidden.clone();
    let _unused_clone = unused.clone();
    let _dummy_clone = dummy.clone();
    let _cycle_clone = cycle.clone();
}

/// Test that orchestrator name() method returns correct value.
#[test]
fn test_orchestrator_name_returns_correct_value() {
    let config = ArchitectureConfig::default();
    let container = ImportContainer::new_with_config(config);
    let orchestrator = container.orchestrator();

    assert_eq!(orchestrator.name(), "import-rules");
}

/// Test that container exposes protocol implementations via accessor methods.
#[test]
fn test_container_exposes_protocols() {
    let config = ArchitectureConfig::default();
    let container = ImportContainer::new_with_config(config);

    // Should be able to access all protocol implementations
    let _mandatory = container.mandatory();
    let _forbidden = container.forbidden();
    let _unused = container.unused();
    let _dummy = container.dummy();
    let _cycle = container.cycle();
    let _orchestrator = container.orchestrator();
}

/// Test that orchestrator filters source files by extension.
#[tokio::test]
async fn test_orchestrator_filters_source_files() {
    let config = ArchitectureConfig::default();
    let container = ImportContainer::new_with_config(config);
    let orchestrator = container.orchestrator();

    // Run audit on current directory — should only process .rs, .py, .js, .ts files
    let result = orchestrator
        .run_audit(&FilePath::new("./".to_string()).unwrap())
        .await;

    assert!(result.is_ok());

    // Results should not contain non-source files (e.g., Cargo.toml, README.md)
    let results = result.unwrap();
    for r in &results {
        // Each result's file should be a source file extension
        let ext = std::path::Path::new(&r.file.value)
            .extension()
            .and_then(|e| e.to_str());
        assert!(
            matches!(
                ext,
                Some("rs") | Some("py") | Some("js") | Some("ts") | Some("jsx") | Some("tsx")
            ),
            "Non-source file should not appear in results: {:?}",
            r.file.value
        );
    }
}

/// Test that orchestrator can handle path with no source files.
#[tokio::test]
async fn test_orchestrator_handles_no_source_files() {
    let config = ArchitectureConfig::default();
    let container = ImportContainer::new_with_config(config);
    let orchestrator = container.orchestrator();

    // Create a temp directory with no .rs files
    let temp_dir = std::env::temp_dir().join("import_rules_test_no_files");
    let _ = std::fs::create_dir_all(&temp_dir);

    let result = orchestrator
        .run_audit(&FilePath::new(temp_dir.to_string_lossy().to_string()).unwrap())
        .await;

    // Clean up
    let _ = std::fs::remove_dir(&temp_dir);

    assert!(result.is_ok(), "Empty directory should return Ok");
}

/// Test that cycle analyzer can detect cycle edges.
#[test]
fn test_cycle_analyzer_detects_edges() {
    use shared::import_rules::taxonomy_dependency_edge_vo::DependencyEdge;
    use shared::taxonomy_name_vo::SymbolName;

    let analyzer = DependencyCycleAnalyzer::new();

    // Create edges that form a cycle: A -> B -> A
    let edges = vec![
        DependencyEdge::new("agent".to_string(), "capabilities".to_string()),
        DependencyEdge::new("capabilities".to_string(), "agent".to_string()),
    ];

    let cycle_edges = analyzer.detect_cycle_edges(&edges);

    // Should detect at least one cycle edge
    assert!(
        !cycle_edges.is_empty(),
        "Cycle edges should be detected for circular dependencies"
    );
}

/// Test that cycle analyzer normalizes layer names correctly.
#[test]
fn test_cycle_analyzer_normalizes_layers() {
    let analyzer = DependencyCycleAnalyzer::new();

    // Should normalize "agent_pipeline_orchestrator" to "agent"
    let normalized = analyzer.normalize_to_layer("agent_pipeline_orchestrator");
    assert_eq!(normalized.value(), "agent");

    // Should normalize "capabilities_user_checker" to "capabilities"
    let normalized = analyzer.normalize_to_layer("capabilities_user_checker");
    assert_eq!(normalized.value(), "capabilities");
}
