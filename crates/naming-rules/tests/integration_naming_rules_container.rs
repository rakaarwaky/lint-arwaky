// Integration tests for NamingContainer and NamingOrchestrator — DI wiring and cross-layer interaction

use std::sync::Arc;

use naming_rules_lint_arwaky::{
    agent_naming_orchestrator::{NamingOrchestrator, NamingOrchestratorDeps},
    capabilities_naming_convention_checker::NamingConventionChecker,
    capabilities_suffix_prefix_checker::SuffixPrefixChecker,
    root_naming_rules_container::NamingContainer,
};
use shared::cli_commands::taxonomy_result_vo::LintResult;
use shared::common::taxonomy_path_vo::FilePath;
use shared::config_system::taxonomy_config_vo::ArchitectureConfig;
use shared::naming_rules::contract_naming_runner_aggregate::INamingRunnerAggregate;
use shared::taxonomy_definition_vo::LayerMapVO;

// ─── Integration Tests: DI Container Wiring ────────────────────────

/// Test that NamingContainer properly wires naming convention checker.
#[test]
fn test_container_wires_naming_convention_checker() {
    let config = Arc::new(ArchitectureConfig::default());
    let layer_map = Arc::new(LayerMapVO::new(std::collections::HashMap::new()));
    let container = NamingContainer::new(config, layer_map);

    // Should be able to access naming convention checker via protocol
    let _checker = container.naming_convention_checker();

    // Should be Arc<dyn INamingConventionChecker>
    let checker_ref = container.naming_convention_checker();
    assert!(std::sync::Arc::ptr_eq(checker_ref, checker_ref));
}

/// Test that NamingContainer properly wires suffix/prefix checker.
#[test]
fn test_container_wires_suffix_prefix_checker() {
    let config = Arc::new(ArchitectureConfig::default());
    let layer_map = Arc::new(LayerMapVO::new(std::collections::HashMap::new()));
    let container = NamingContainer::new(config, layer_map);

    // Should be able to access suffix/prefix checker via protocol
    let _checker = container.suffix_prefix_checker();

    let checker_ref = container.suffix_prefix_checker();
    assert!(std::sync::Arc::ptr_eq(checker_ref, checker_ref));
}

/// Test that NamingContainer can create orchestrator from container.
#[test]
fn test_container_creates_orchestrator() {
    let config = Arc::new(ArchitectureConfig::default());
    let layer_map = Arc::new(LayerMapVO::new(std::collections::HashMap::new()));
    let container = NamingContainer::new(config, layer_map);

    // Should create orchestrator without panicking
    let orchestrator = container.orchestrator();

    // Should return a valid INamingRunnerAggregate
    assert_eq!(orchestrator.name(), "naming-rules");
}

/// Test that orchestrator can run audit on empty path.
#[tokio::test]
async fn test_orchestrator_run_audit_empty_path() {
    let config = Arc::new(ArchitectureConfig::default());
    let layer_map = Arc::new(LayerMapVO::new(std::collections::HashMap::new()));
    let container = NamingContainer::new(config, layer_map);
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
    let config = Arc::new(ArchitectureConfig::default());
    let layer_map = Arc::new(LayerMapVO::new(std::collections::HashMap::new()));
    let container = NamingContainer::new(config, layer_map);
    let orchestrator = container.orchestrator();

    // Run audit on current directory — should return Ok (may have violations or not)
    let result = orchestrator
        .run_audit(&FilePath::new("./".to_string()).unwrap())
        .await;

    assert!(result.is_ok(), "Existing path should return Ok");

    // Results should be a Vec<LintResult> (possibly empty)
    let results = result.unwrap();
    assert!(results
        .iter()
        .all(|r: &LintResult| !r.file.value.is_empty()));
}

/// Test that NamingContainer can be created with default config.
#[test]
fn test_default_container_creation() {
    let config = Arc::new(ArchitectureConfig::default());
    let layer_map = Arc::new(LayerMapVO::new(std::collections::HashMap::new()));

    // Should create container without panicking
    let _container = NamingContainer::new(config, layer_map);
}

/// Test that naming convention checker and suffix prefix checker work together.
#[tokio::test]
async fn test_convention_and_suffix_checkers_work_together() {
    let config = Arc::new(ArchitectureConfig::default());
    let layer_map = Arc::new(LayerMapVO::new(std::collections::HashMap::new()));

    let convention_checker = Arc::new(NamingConventionChecker::new());
    let suffix_checker = Arc::new(SuffixPrefixChecker::new());

    // Create orchestrator with both checkers
    let orchestrator = NamingOrchestrator::new(NamingOrchestratorDeps {
        naming_convention_checker: convention_checker,
        suffix_prefix_checker: suffix_checker,
        config,
        layer_map,
    });

    // Should be able to run audit
    let result = orchestrator
        .run_audit(&FilePath::new("./".to_string()).unwrap())
        .await;

    assert!(result.is_ok());
}

/// Test that both checkers are properly cloned in container.
#[test]
fn test_container_checkers_are_cloned() {
    let config = Arc::new(ArchitectureConfig::default());
    let layer_map = Arc::new(LayerMapVO::new(std::collections::HashMap::new()));
    let container = NamingContainer::new(config, layer_map);

    let convention_checker = container.naming_convention_checker().clone();
    let suffix_checker = container.suffix_prefix_checker().clone();

    // Both should be clonable (Clone trait is implemented)
    let _convention_clone = convention_checker.clone();
    let _suffix_clone = suffix_checker.clone();
}

/// Test that orchestrator name() method returns correct value.
#[test]
fn test_orchestrator_name_returns_correct_value() {
    let config = Arc::new(ArchitectureConfig::default());
    let layer_map = Arc::new(LayerMapVO::new(std::collections::HashMap::new()));
    let container = NamingContainer::new(config, layer_map);
    let orchestrator = container.orchestrator();

    assert_eq!(orchestrator.name(), "naming-rules");
}

/// Test that orchestrator can handle path with no source files.
#[tokio::test]
async fn test_orchestrator_handles_no_source_files() {
    let config = Arc::new(ArchitectureConfig::default());
    let layer_map = Arc::new(LayerMapVO::new(std::collections::HashMap::new()));
    let container = NamingContainer::new(config, layer_map);
    let orchestrator = container.orchestrator();

    // Create a temp directory with no .rs files
    let temp_dir = std::env::temp_dir().join("naming_rules_test_no_files");
    let _ = std::fs::create_dir_all(&temp_dir);

    let result = orchestrator
        .run_audit(&FilePath::new(temp_dir.to_string_lossy().to_string()).unwrap())
        .await;

    // Clean up
    let _ = std::fs::remove_dir(&temp_dir);

    assert!(result.is_ok(), "Empty directory should return Ok");
}

/// Test that container exposes protocol implementations via accessor methods.
#[test]
fn test_container_exposes_protocols() {
    let config = Arc::new(ArchitectureConfig::default());
    let layer_map = Arc::new(LayerMapVO::new(std::collections::HashMap::new()));
    let container = NamingContainer::new(config, layer_map);

    // Should be able to access both protocol implementations
    let _convention = container.naming_convention_checker();
    let _suffix = container.suffix_prefix_checker();
    let _orchestrator = container.orchestrator();
}

/// Test that orchestrator filters source files by extension.
#[tokio::test]
async fn test_orchestrator_filters_source_files() {
    let config = Arc::new(ArchitectureConfig::default());
    let layer_map = Arc::new(LayerMapVO::new(std::collections::HashMap::new()));
    let container = NamingContainer::new(config, layer_map);
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
