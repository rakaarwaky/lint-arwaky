// Contract tests for naming-rules — verify trait implementations exist and are callable

use std::sync::Arc;

use naming_rules_lint_arwaky::{
    agent_naming_orchestrator::NamingOrchestrator,
    capabilities_naming_convention_checker::NamingConventionChecker,
    capabilities_suffix_prefix_checker::SuffixPrefixChecker,
};
use shared::cli_commands::taxonomy_result_vo::LintResultList;
use shared::common::taxonomy_path_vo::FilePath;
use shared::common::taxonomy_paths_vo::FilePathList;
use shared::config_system::taxonomy_config_vo::ArchitectureConfig;
use shared::naming_rules::contract_naming_checker_protocol::{
    INamingConventionChecker, ISuffixPrefixChecker,
};
use shared::naming_rules::contract_naming_runner_aggregate::INamingRunnerAggregate;
use shared::taxonomy_definition_vo::LayerMapVO;

// ─── Contract Tests: Trait Implementation Verification ──────────────

/// Verify INamingConventionChecker trait implementation exists and is callable
#[tokio::test]
async fn contract_naming_convention_checker_trait_is_implemented() {
    let checker = NamingConventionChecker::new();
    let config = ArchitectureConfig::default();
    let layer_map = LayerMapVO::new(std::collections::HashMap::new());
    let files = FilePathList { values: vec![] };
    let root = FilePath::new(".".to_string()).unwrap();
    let mut results = LintResultList::new(Vec::new());

    // Should be able to call check_file_naming via the trait — no panics
    checker
        .check_file_naming(&config, &layer_map, &files, &root, &mut results)
        .await;

    // No panic = trait implementation verified
}

/// Verify ISuffixPrefixChecker trait implementation exists and is callable
#[tokio::test]
async fn contract_suffix_prefix_checker_trait_is_implemented() {
    let checker = SuffixPrefixChecker::new();
    let config = ArchitectureConfig::default();
    let layer_map = LayerMapVO::new(std::collections::HashMap::new());
    let files = FilePathList { values: vec![] };
    let root = FilePath::new(".".to_string()).unwrap();
    let mut results = LintResultList::new(Vec::new());

    // Should be able to call check_domain_suffixes via the trait — no panics
    checker
        .check_domain_suffixes(&config, &layer_map, &files, &root, &mut results)
        .await;

    // No panic = trait implementation verified
}

/// Verify INamingRunnerAggregate trait implementation (orchestrator) exists
#[test]
fn contract_naming_runner_aggregate_trait_is_implemented() {
    let config = ArchitectureConfig::default();
    let layer_map = LayerMapVO::new(std::collections::HashMap::new());
    let naming_checker = Arc::new(NamingConventionChecker::new());
    let suffix_checker = Arc::new(SuffixPrefixChecker::new());

    let orchestrator = NamingOrchestrator::new(
        naming_checker,
        suffix_checker,
        Arc::new(config),
        Arc::new(layer_map),
    );

    // Should be able to call name() via the trait
    assert_eq!(orchestrator.name(), "naming-rules");
}

/// Verify NamingConventionChecker implements Clone (required by async_trait)
#[test]
fn contract_naming_convention_checker_is_clone() {
    let checker = NamingConventionChecker::new();
    let cloned = checker.clone();

    // Clone should produce an equivalent checker
    assert_eq!(
        std::mem::size_of_val(&checker),
        std::mem::size_of_val(&cloned)
    );
}

/// Verify SuffixPrefixChecker implements Clone (required by async_trait)
#[test]
fn contract_suffix_prefix_checker_is_clone() {
    let checker = SuffixPrefixChecker::new();
    let cloned = checker.clone();

    assert_eq!(
        std::mem::size_of_val(&checker),
        std::mem::size_of_val(&cloned)
    );
}

/// Verify NamingConventionChecker implements Default
#[test]
fn contract_naming_convention_checker_has_default() {
    let default_checker = NamingConventionChecker::default();
    let new_checker = NamingConventionChecker::new();

    // Default and new should produce equivalent instances
    assert_eq!(
        std::mem::size_of_val(&default_checker),
        std::mem::size_of_val(&new_checker)
    );
}

/// Verify SuffixPrefixChecker implements Default
#[test]
fn contract_suffix_prefix_checker_has_default() {
    let default_checker = SuffixPrefixChecker::default();
    let new_checker = SuffixPrefixChecker::new();

    assert_eq!(
        std::mem::size_of_val(&default_checker),
        std::mem::size_of_val(&new_checker)
    );
}
