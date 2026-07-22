// PURPOSE: Acceptance tests for FR-002 — AES102 Suffix/Prefix Layer Alignment
// Maps 1:1 to FRD business rules

use naming_rules_lint_arwaky::root_naming_rules_container::NamingContainer;
use shared::common::taxonomy_common_vo::PatternList;
use shared::common::taxonomy_common_vo::SuffixPolicyVO;
use shared::common::taxonomy_definition_vo::{LayerDefinition, LayerMapVO};
use shared::common::taxonomy_layer_vo::LayerNameVO;
use shared::common::taxonomy_path_vo::FilePath;
use shared::config_system::taxonomy_config_vo::ArchitectureConfig;
use shared::config_system::taxonomy_config_vo::NamingRuleVO;
use std::collections::HashMap;
use std::sync::Arc;

fn build_strict_container() -> NamingContainer {
    let mut layers = HashMap::new();

    // taxonomy: strict — only _vo, _entity, _error, _event, _constant
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
        forbidden_suffix: PatternList::new(vec!["orchestrator".to_string(), "checker".to_string()]),
        ..Default::default()
    };
    layers.insert(LayerNameVO::new("taxonomy"), taxonomy_def);

    // contract: strict — only _protocol, _aggregate
    let mut contract_def = LayerDefinition::default();
    contract_def.naming = NamingRuleVO {
        suffix_policy: SuffixPolicyVO::new("strict".to_string()),
        allowed_suffix: PatternList::new(vec!["protocol".to_string(), "aggregate".to_string()]),
        ..Default::default()
    };
    layers.insert(LayerNameVO::new("contract"), contract_def);

    // agent: strict — only _orchestrator
    let mut agent_def = LayerDefinition::default();
    agent_def.naming = NamingRuleVO {
        suffix_policy: SuffixPolicyVO::new("strict".to_string()),
        allowed_suffix: PatternList::new(vec!["orchestrator".to_string()]),
        ..Default::default()
    };
    layers.insert(LayerNameVO::new("agent"), agent_def);

    // capabilities: flexible
    layers.insert(LayerNameVO::new("capabilities"), LayerDefinition::default());

    // utility: flexible
    layers.insert(LayerNameVO::new("utility"), LayerDefinition::default());

    let config = ArchitectureConfig {
        layers: layers.clone(),
        ..Default::default()
    };
    let layer_map = LayerMapVO::new(layers);
    NamingContainer::new(Arc::new(config), Arc::new(layer_map))
}

async fn scan_file(filename: &str) -> Vec<shared::cli_commands::taxonomy_result_vo::LintResult> {
    let dir = tempfile::tempdir().unwrap();
    std::fs::write(dir.path().join(filename), "").unwrap();

    let container = build_strict_container();
    let orch = container.orchestrator();
    let target = FilePath::new(dir.path().to_string_lossy().to_string()).unwrap();
    orch.run_audit(&target).await.unwrap()
}

// ─── FR-002: taxonomy_ allowed suffixes ───────────────────

/// FR-002: taxonomy_: _vo, _entity, _error, _event, _constant
#[tokio::test]
async fn fr002_taxonomy_vo_passes() {
    let results = scan_file("taxonomy_user_vo.rs").await;
    let aes102: Vec<_> = results
        .iter()
        .filter(|r| r.code.code() == "AES102")
        .collect();
    assert!(aes102.is_empty(), "taxonomy_*_vo should pass");
}

/// FR-002: taxonomy_: _vo, _entity, _error, _event, _constant
#[tokio::test]
async fn fr002_taxonomy_entity_passes() {
    let results = scan_file("taxonomy_order_entity.rs").await;
    let aes102: Vec<_> = results
        .iter()
        .filter(|r| r.code.code() == "AES102")
        .collect();
    assert!(aes102.is_empty());
}

/// FR-002: taxonomy_: _vo, _entity, _error, _event, _constant
#[tokio::test]
async fn fr002_taxonomy_error_passes() {
    let results = scan_file("taxonomy_parse_error.rs").await;
    let aes102: Vec<_> = results
        .iter()
        .filter(|r| r.code.code() == "AES102")
        .collect();
    assert!(aes102.is_empty());
}

/// FR-002: taxonomy_: _vo, _entity, _error, _event, _constant
#[tokio::test]
async fn fr002_taxonomy_event_passes() {
    let results = scan_file("taxonomy_user_event.rs").await;
    let aes102: Vec<_> = results
        .iter()
        .filter(|r| r.code.code() == "AES102")
        .collect();
    assert!(aes102.is_empty());
}

/// FR-002: taxonomy_: _vo, _entity, _error, _event, _constant
#[tokio::test]
async fn fr002_taxonomy_constant_passes() {
    let results = scan_file("taxonomy_naming_constant.rs").await;
    let aes102: Vec<_> = results
        .iter()
        .filter(|r| r.code.code() == "AES102")
        .collect();
    assert!(aes102.is_empty());
}

// ─── FR-002: contract_ allowed suffixes ───────────────────

/// FR-002: contract_: _protocol, _aggregate
#[tokio::test]
async fn fr002_contract_protocol_passes() {
    let results = scan_file("contract_naming_checker_protocol.rs").await;
    let aes102: Vec<_> = results
        .iter()
        .filter(|r| r.code.code() == "AES102")
        .collect();
    assert!(aes102.is_empty());
}

/// FR-002: contract_: _protocol, _aggregate
#[tokio::test]
async fn fr002_contract_aggregate_passes() {
    let results = scan_file("contract_naming_runner_aggregate.rs").await;
    let aes102: Vec<_> = results
        .iter()
        .filter(|r| r.code.code() == "AES102")
        .collect();
    assert!(aes102.is_empty());
}

// ─── FR-002: agent_ allowed suffixes ──────────────────────

/// FR-002: agent_: _orchestrator
#[tokio::test]
async fn fr002_agent_orchestrator_passes() {
    let results = scan_file("agent_naming_orchestrator.rs").await;
    let aes102: Vec<_> = results
        .iter()
        .filter(|r| r.code.code() == "AES102")
        .collect();
    assert!(aes102.is_empty());
}

// ─── FR-002: Wrong suffix fails ───────────────────────────

/// FR-002: File suffix must align with its layer prefix.
#[tokio::test]
async fn fr002_taxonomy_with_orchestrator_fails() {
    let results = scan_file("taxonomy_user_orchestrator.rs").await;
    let aes102: Vec<_> = results
        .iter()
        .filter(|r| r.code.code() == "AES102")
        .collect();
    assert!(!aes102.is_empty(), "orchestrator not allowed in taxonomy");
}

/// FR-002: File suffix must align with its layer prefix.
#[tokio::test]
async fn fr002_agent_with_vo_fails() {
    let results = scan_file("agent_naming_vo.rs").await;
    let aes102: Vec<_> = results
        .iter()
        .filter(|r| r.code.code() == "AES102")
        .collect();
    assert!(!aes102.is_empty(), "vo not allowed in agent");
}

/// FR-002: File suffix must align with its layer prefix.
#[tokio::test]
async fn fr002_contract_with_vo_fails() {
    let results = scan_file("contract_naming_vo.rs").await;
    let aes102: Vec<_> = results
        .iter()
        .filter(|r| r.code.code() == "AES102")
        .collect();
    assert!(!aes102.is_empty(), "vo not allowed in contract");
}

// ─── FR-002: Flexible layers accept any suffix ────────────

/// FR-002: utility_: any role suffix (flexible)
#[tokio::test]
async fn fr002_utility_any_suffix_passes() {
    let results = scan_file("utility_json_parser.rs").await;
    let aes102: Vec<_> = results
        .iter()
        .filter(|r| r.code.code() == "AES102")
        .collect();
    assert!(aes102.is_empty(), "utility is flexible");
}

/// FR-002: capabilities_: any role suffix (flexible)
#[tokio::test]
async fn fr002_capabilities_any_suffix_passes() {
    let results = scan_file("capabilities_data_transformer.rs").await;
    let aes102: Vec<_> = results
        .iter()
        .filter(|r| r.code.code() == "AES102")
        .collect();
    assert!(aes102.is_empty(), "capabilities is flexible");
}

// ─── FR-002: Error handling emits AES102 ──────────────────

/// FR-002: Emit AES102 with expected suffixes
#[tokio::test]
async fn fr002_emits_aes102_code() {
    let results = scan_file("taxonomy_bad_orchestrator.rs").await;
    assert!(!results.is_empty());
    assert_eq!(results[0].code.code(), "AES102");
}
