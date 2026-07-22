// PURPOSE: E2E integration test — full naming-rules pipeline on mixed valid/invalid files
//
// Tests the complete flow: container → orchestrator → checkers → results
// Uses a real tempdir with mixed valid and invalid file names

use naming_rules_lint_arwaky::root_naming_rules_container::NamingContainer;
use shared::cli_commands::taxonomy_result_vo::LintResult;
use shared::common::taxonomy_definition_vo::{LayerDefinition, LayerMapVO};
use shared::common::taxonomy_layer_vo::LayerNameVO;
use shared::common::taxonomy_path_vo::FilePath;
use shared::config_system::taxonomy_config_vo::ArchitectureConfig;
use shared::naming_rules::contract_naming_runner_aggregate::INamingRunnerAggregate;
use std::collections::HashMap;
use std::sync::Arc;

// ─── Helpers ──────────────────────────────────────────────────────────────────

/// Build a naming container with all AES layer definitions (strict + flexible).
fn build_full_container() -> NamingContainer {
    let mut layers = HashMap::new();

    // Strict layers
    let mut taxonomy_def = LayerDefinition::default();
    taxonomy_def.naming = shared::naming_rules::taxonomy_naming_rule_vo::NamingRuleVO {
        naming_convention: shared::common::taxonomy_common_vo::BooleanVO::new(true),
        suffix_policy: shared::common::taxonomy_suffix_vo::SuffixPolicyVO::new("strict".to_string()),
        allowed_suffix: shared::common::taxonomy_common_vo::PatternList::new(vec![
            "vo".to_string(),
            "entity".to_string(),
            "error".to_string(),
            "event".to_string(),
            "constant".to_string(),
        ]),
        forbidden_suffix: shared::common::taxonomy_common_vo::PatternList::new(vec![]),
    };
    layers.insert(LayerNameVO::new("taxonomy"), taxonomy_def);

    let mut contract_def = LayerDefinition::default();
    contract_def.naming = shared::naming_rules::taxonomy_naming_rule_vo::NamingRuleVO {
        naming_convention: shared::common::taxonomy_common_vo::BooleanVO::new(true),
        suffix_policy: shared::common::taxonomy_suffix_vo::SuffixPolicyVO::new("strict".to_string()),
        allowed_suffix: shared::common::taxonomy_common_vo::PatternList::new(vec![
            "protocol".to_string(),
            "aggregate".to_string(),
        ]),
        forbidden_suffix: shared::common::taxonomy_common_vo::PatternList::new(vec![]),
    };
    layers.insert(LayerNameVO::new("contract"), contract_def);

    let mut agent_def = LayerDefinition::default();
    agent_def.naming = shared::naming_rules::taxonomy_naming_rule_vo::NamingRuleVO {
        naming_convention: shared::common::taxonomy_common_vo::BooleanVO::new(true),
        suffix_policy: shared::common::taxonomy_suffix_vo::SuffixPolicyVO::new("strict".to_string()),
        allowed_suffix: shared::common::taxonomy_common_vo::PatternList::new(vec![
            "orchestrator".to_string(),
        ]),
        forbidden_suffix: shared::common::taxonomy_common_vo::PatternList::new(vec![]),
    };
    layers.insert(LayerNameVO::new("agent"), agent_def);

    // Flexible layers (any suffix accepted)
    layers.insert(
        LayerNameVO::new("capabilities"),
        LayerDefinition::default(),
    );
    layers.insert(
        LayerNameVO::new("utility"),
        LayerDefinition::default(),
    );
    layers.insert(
        LayerNameVO::new("surface"),
        LayerDefinition::default(),
    );
    layers.insert(
        LayerNameVO::new("root"),
        LayerDefinition::default(),
    );

    let config = ArchitectureConfig {
        layers: layers.clone(),
        ..Default::default()
    };
    let layer_map = LayerMapVO::new(layers);
    NamingContainer::new(Arc::new(config), Arc::new(layer_map))
}

/// Create a tempdir and write empty files with the given filenames.
fn setup_test_files(filenames: &[&str]) -> tempfile::TempDir {
    let dir = tempfile::tempdir().unwrap();
    for name in filenames {
        std::fs::write(dir.path().join(*name), "").unwrap();
    }
    dir
}

// ─── Test: Mixed valid and invalid files produce correct results ──────────────

/// E2E: Full pipeline on a directory with 10 mixed files — validates that
/// valid names pass, invalid names fail with correct rule codes.
#[tokio::test]
async fn e2e_mixed_files_produces_correct_violations() {
    let dir = setup_test_files(&[
        // Valid files (should produce NO violations)
        "taxonomy_user_vo.rs",
        "contract_naming_checker_protocol.rs",
        "agent_pipeline_orchestrator.rs",
        "capabilities_user_checker.rs",
        "utility_json_parser.rs",
        // Invalid files (should produce violations)
        "badfile.rs",           // No layer prefix, too few words → AES101
        "Capabilities_Check.rs", // Uppercase → AES101
        "taxonomy_bad_orchestrator.rs", // Wrong suffix for taxonomy → AES102
        "agent_naming_vo.rs",   // Wrong suffix for agent → AES102
    ]);

    let container = build_full_container();
    let orch = container.orchestrator();
    let target = FilePath::new(dir.path().to_string_lossy().to_string()).unwrap();
    let results = orch.run_audit(&target).await.unwrap();

    // Count violations by rule code
    let aes101_count: usize = results
        .iter()
        .filter(|r| r.code.code() == "AES101")
        .count();
    let aes102_count: usize = results
        .iter()
        .filter(|r| r.code.code() == "AES102")
        .count();

    // badfile + Capabilities_Check should fail AES101 (naming convention)
    assert!(
        aes101_count >= 2,
        "Expected at least 2 AES101 violations for badfile.rs and Capabilities_Check.rs, got {}",
        aes101_count
    );

    // taxonomy_bad_orchestrator + agent_naming_vo should fail AES102 (suffix mismatch)
    assert!(
        aes102_count >= 2,
        "Expected at least 2 AES102 violations for wrong suffixes, got {}",
        aes102_count
    );

    // Total violations should be positive
    assert!(results.len() > 0, "Expected violations from mixed files");

    // Verify no violations for valid files (check that valid filenames are NOT in results)
    let valid_names = [
        "taxonomy_user_vo.rs",
        "contract_naming_checker_protocol.rs",
        "agent_pipeline_orchestrator.rs",
        "capabilities_user_checker.rs",
        "utility_json_parser.rs",
    ];

    for name in &valid_names {
        let file_in_results = results
            .iter()
            .any(|r| r.file.value.contains(*name) || r.file.value.ends_with(*name));
        assert!(
            !file_in_results,
            "Valid file '{}' should not appear in violations",
            name
        );
    }
}

// ─── Test: Barrel and entry point files are always skipped ────────────────────

/// E2E: A directory with only barrel/entry files should produce zero violations.
#[tokio::test]
async fn e2e_barrel_files_always_skip() {
    let dir = setup_test_files(&[
        "mod.rs",
        "__init__.py",
        "index.ts",
        "index.js",
        "main.rs",
        "lib.rs",
    ]);

    let container = build_full_container();
    let orch = container.orchestrator();
    let target = FilePath::new(dir.path().to_string_lossy().to_string()).unwrap();
    let results = orch.run_audit(&target).await.unwrap();

    assert!(
        results.is_empty(),
        "Barrel and entry point files should produce zero violations, got {}",
        results.len()
    );
}

// ─── Test: Non-source extensions are filtered out ─────────────────────────────

/// E2E: A directory with non-source files (e.g., .txt, .json) should produce
/// zero violations because they're filtered before checking.
#[tokio::test]
async fn e2e_non_source_extensions_filtered() {
    let dir = setup_test_files(&[
        "bad_file_name.txt",
        "also_bad.json",
        "config.yaml",
    ]);

    let container = build_full_container();
    let orch = container.orchestrator();
    let target = FilePath::new(dir.path().to_string_lossy().to_string()).unwrap();
    let results = orch.run_audit(&target).await.unwrap();

    assert!(
        results.is_empty(),
        "Non-source files should be filtered out and produce zero violations, got {}",
        results.len()
    );
}

// ─── Test: All valid layer prefixes are recognized ────────────────────────────

/// E2E: A directory with one valid file per layer prefix should produce zero AES101.
#[tokio::test]
async fn e2e_all_valid_prefixes_recognized() {
    let dir = setup_test_files(&[
        "taxonomy_user_vo.rs",
        "contract_data_protocol.rs",
        "utility_json_parser.rs",
        "capabilities_user_checker.rs",
        "agent_pipeline_orchestrator.rs",
        "surface_check_command.rs",
        "root_cli_container.rs",
    ]);

    let container = build_full_container();
    let orch = container.orchestrator();
    let target = FilePath::new(dir.path().to_string_lossy().to_string()).unwrap();
    let results = orch.run_audit(&target).await.unwrap();

    let aes101: Vec<_> = results.iter().filter(|r| r.code.code() == "AES101").collect();
    assert!(
        aes101.is_empty(),
        "All valid layer prefixes should produce zero AES101 violations, got {}",
        aes101.len()
    );
}

// ─── Test: Orchestrator name returns correct value ────────────────────────────

/// E2E: Verify the orchestrator's name() method returns "naming-rules".
#[tokio::test]
async fn e2e_orchestrator_returns_name() {
    let container = build_full_container();
    let orch = container.orchestrator();
    assert_eq!(orch.name(), "naming-rules");
}
