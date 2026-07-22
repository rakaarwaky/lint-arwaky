// PURPOSE: Integration tests — full DI wiring via NamingContainer
// Uses real container, real checkers, real orchestrator
// Coverage target: validates wiring correctness

use naming_rules_lint_arwaky::root_naming_rules_container::NamingContainer;
use shared::common::taxonomy_common_vo::PatternList;
use shared::common::taxonomy_definition_vo::{LayerDefinition, LayerMapVO};
use shared::common::taxonomy_layer_vo::LayerNameVO;
use shared::common::taxonomy_path_vo::FilePath;
use shared::common::taxonomy_paths_vo::FilePathList;
use shared::common::taxonomy_common_vo::SuffixPolicyVO;
use shared::config_system::taxonomy_config_vo::ArchitectureConfig;
use shared::config_system::taxonomy_config_vo::NamingRuleVO;
use std::collections::HashMap;
use std::sync::Arc;

fn build_test_config() -> ArchitectureConfig {
    let mut layers = HashMap::new();

    let mut taxonomy_def = LayerDefinition::default();
    taxonomy_def.naming = NamingRuleVO {
        suffix_policy: SuffixPolicyVO::new("strict".to_string()),
        allowed_suffix: PatternList::new(vec![
            "vo".to_string(),
            "entity".to_string(),
            "error".to_string(),
            "event".to_string(),
            "constant".to_string(),
        ]),
        ..Default::default()
    };
    layers.insert(LayerNameVO::new("taxonomy"), taxonomy_def);

    let mut agent_def = LayerDefinition::default();
    agent_def.naming = NamingRuleVO {
        suffix_policy: SuffixPolicyVO::new("strict".to_string()),
        allowed_suffix: PatternList::new(vec!["orchestrator".to_string()]),
        ..Default::default()
    };
    layers.insert(LayerNameVO::new("agent"), agent_def);

    let capabilities_def = LayerDefinition::default();
    layers.insert(LayerNameVO::new("capabilities"), capabilities_def);

    ArchitectureConfig {
        layers,
        ..Default::default()
    }
}

fn build_container() -> NamingContainer {
    let config = build_test_config();
    let layer_map = LayerMapVO::new(config.layers.clone());
    NamingContainer::new(Arc::new(config), Arc::new(layer_map))
}

// ─── Container Wiring ─────────────────────────────────────

#[test]
fn container_creates_orchestrator_successfully() {
    let container = build_container();
    let orch = container.orchestrator();
    assert_eq!(orch.name(), "naming-rules");
}

#[test]
fn container_exposes_both_checkers() {
    let container = build_container();
    let _conv = container.naming_convention_checker();
    let _suf = container.suffix_prefix_checker();
}

// ─── Full Pipeline: Valid Files ───────────────────────────

#[tokio::test]
async fn full_pipeline_all_valid_files_zero_violations() {
    let dir = tempfile::tempdir().unwrap();
    std::fs::write(dir.path().join("taxonomy_user_vo.rs"), "").unwrap();
    std::fs::write(dir.path().join("taxonomy_order_entity.rs"), "").unwrap();
    std::fs::write(dir.path().join("agent_naming_orchestrator.rs"), "").unwrap();
    std::fs::write(dir.path().join("capabilities_user_checker.rs"), "").unwrap();

    let container = build_container();
    let orch = container.orchestrator();
    let target = FilePath::new(dir.path().to_string_lossy().to_string()).unwrap();
    let results = orch.run_audit(&target).await.unwrap();

    assert!(
        results.is_empty(),
        "All valid files should produce zero violations, got: {:?}",
        results.iter().map(|r| r.code.code()).collect::<Vec<_>>()
    );
}

// ─── Full Pipeline: Mixed Violations ──────────────────────

#[tokio::test]
async fn full_pipeline_detects_naming_and_suffix_violations() {
    let dir = tempfile::tempdir().unwrap();
    // AES101: uppercase
    std::fs::write(dir.path().join("capabilities_Bad_Name.rs"), "").unwrap();
    // AES102: wrong suffix for taxonomy
    std::fs::write(dir.path().join("taxonomy_user_orchestrator.rs"), "").unwrap();
    // Valid
    std::fs::write(dir.path().join("taxonomy_user_vo.rs"), "").unwrap();

    let container = build_container();
    let orch = container.orchestrator();
    let target = FilePath::new(dir.path().to_string_lossy().to_string()).unwrap();
    let results = orch.run_audit(&target).await.unwrap();

    assert!(results.len() >= 2, "Should detect at least 2 violations");
    let codes: Vec<&str> = results.iter().map(|r| r.code.code()).collect();
    assert!(codes.contains(&"AES101"), "Should contain AES101");
    assert!(codes.contains(&"AES102"), "Should contain AES102");
}

// ─── Full Pipeline: Ignored Paths ─────────────────────────

#[tokio::test]
async fn full_pipeline_respects_ignored_paths() {
    let dir = tempfile::tempdir().unwrap();
    let ignored_dir = dir.path().join("target");
    std::fs::create_dir_all(&ignored_dir).unwrap();
    std::fs::write(ignored_dir.join("capabilities_Bad_Name.rs"), "").unwrap();
    std::fs::write(dir.path().join("taxonomy_user_vo.rs"), "").unwrap();

    let mut config = build_test_config();
    config.ignored_paths = FilePathList::new(vec![FilePath::new("target".to_string()).unwrap()]);
    let layer_map = LayerMapVO::new(config.layers.clone());
    let container = NamingContainer::new(Arc::new(config), Arc::new(layer_map));
    let orch = container.orchestrator();
    let target = FilePath::new(dir.path().to_string_lossy().to_string()).unwrap();
    let results = orch.run_audit(&target).await.unwrap();

    assert!(
        results.is_empty(),
        "Ignored directory files should not produce violations"
    );
}

// ─── Full Pipeline: Non-existent Target ───────────────────

#[tokio::test]
async fn full_pipeline_nonexistent_target_returns_error() {
    let container = build_container();
    let orch = container.orchestrator();
    let target = FilePath::new("/does/not/exist".to_string()).unwrap();
    let result = orch.run_audit(&target).await;
    assert!(result.is_err());
}
