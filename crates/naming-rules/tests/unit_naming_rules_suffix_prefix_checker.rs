// PURPOSE: Unit tests for SuffixPrefixChecker (AES102 — suffix/prefix alignment)
// Covers: happy path, forbidden suffix, strict policy mismatch, edge cases
// Coverage target: Capabilities ≥ 70%

use naming_rules_lint_arwaky::capabilities_suffix_prefix_checker::SuffixPrefixChecker;
use shared::cli_commands::LintResultList;
use shared::common::{
    FilePath, FilePathList, LayerDefinition, LayerMapVO, LayerNameVO, PatternList, SuffixPolicyVO,
};

use shared::config_system::{ArchitectureConfig, NamingRuleVO};

use shared::naming_rules::ISuffixPrefixChecker;
use std::collections::HashMap;

fn default_config() -> ArchitectureConfig {
    ArchitectureConfig::default()
}

fn strict_taxonomy_layer() -> LayerMapVO {
    let mut map = HashMap::new();
    let def = LayerDefinition {
        naming: NamingRuleVO {
            suffix_policy: SuffixPolicyVO::new("strict".to_string()),
            allowed_suffix: PatternList::new(vec![
                "vo".to_string(),
                "entity".to_string(),
                "error".to_string(),
                "event".to_string(),
                "constant".to_string(),
            ]),
            forbidden_suffix: PatternList::new(vec!["orchestrator".to_string()]),
            ..Default::default()
        },
        ..Default::default()
    };
    map.insert(LayerNameVO::new("taxonomy"), def);
    LayerMapVO::new(map)
}

fn strict_agent_layer() -> LayerMapVO {
    let mut map = HashMap::new();
    let def = LayerDefinition {
        naming: NamingRuleVO {
            suffix_policy: SuffixPolicyVO::new("strict".to_string()),
            allowed_suffix: PatternList::new(vec!["orchestrator".to_string()]),
            forbidden_suffix: PatternList::new(vec!["vo".to_string(), "entity".to_string()]),
            ..Default::default()
        },
        ..Default::default()
    };
    map.insert(LayerNameVO::new("agent"), def);
    LayerMapVO::new(map)
}

fn flexible_capabilities_layer() -> LayerMapVO {
    let mut map = HashMap::new();
    let def = LayerDefinition::default(); // no strict policy
    map.insert(LayerNameVO::new("capabilities"), def);
    LayerMapVO::new(map)
}

fn make_files(paths: &[&str]) -> FilePathList {
    FilePathList::new(
        paths
            .iter()
            .map(|p| FilePath::new(p.to_string()).unwrap())
            .collect(),
    )
}

async fn run_check(
    files: &[&str],
    config: &ArchitectureConfig,
    layer_map: &LayerMapVO,
) -> LintResultList {
    let checker = SuffixPrefixChecker::new();
    let mut results = LintResultList::new(Vec::new());
    let root = FilePath::new("/project".to_string()).unwrap();
    checker
        .check_domain_suffixes(config, layer_map, &make_files(files), &root, &mut results)
        .await;
    results
}

// ─── Happy Path ───────────────────────────────────────────

#[tokio::test]
async fn taxonomy_vo_suffix_passes_strict() {
    let config = default_config();
    let lm = strict_taxonomy_layer();
    let results = run_check(&["taxonomy_user_vo.rs"], &config, &lm).await;
    assert!(
        results.is_empty(),
        "taxonomy_*_vo should pass strict policy"
    );
}

#[tokio::test]
async fn taxonomy_entity_suffix_passes_strict() {
    let config = default_config();
    let lm = strict_taxonomy_layer();
    let results = run_check(&["taxonomy_order_entity.rs"], &config, &lm).await;
    assert!(results.is_empty());
}

#[tokio::test]
async fn taxonomy_constant_suffix_passes_strict() {
    let config = default_config();
    let lm = strict_taxonomy_layer();
    let results = run_check(&["taxonomy_naming_constant.rs"], &config, &lm).await;
    assert!(results.is_empty());
}

#[tokio::test]
async fn agent_orchestrator_suffix_passes_strict() {
    let config = default_config();
    let lm = strict_agent_layer();
    let results = run_check(&["agent_naming_orchestrator.rs"], &config, &lm).await;
    assert!(results.is_empty());
}

// ─── AES102 Suffix Mismatch ───────────────────────────────

#[tokio::test]
async fn taxonomy_with_orchestrator_suffix_fails() {
    let config = default_config();
    let lm = strict_taxonomy_layer();
    let results = run_check(&["taxonomy_user_orchestrator.rs"], &config, &lm).await;
    assert!(
        !results.is_empty(),
        "orchestrator suffix forbidden in taxonomy"
    );
    assert_eq!(results.values[0].code.code(), "AES102");
}

#[tokio::test]
async fn taxonomy_with_unknown_suffix_fails_strict() {
    let config = default_config();
    let lm = strict_taxonomy_layer();
    let results = run_check(&["taxonomy_user_parser.rs"], &config, &lm).await;
    assert!(
        !results.is_empty(),
        "parser not in allowed list for taxonomy"
    );
    assert_eq!(results.values[0].code.code(), "AES102");
}

#[tokio::test]
async fn agent_with_vo_suffix_fails_forbidden() {
    let config = default_config();
    let lm = strict_agent_layer();
    let results = run_check(&["agent_naming_vo.rs"], &config, &lm).await;
    assert!(!results.is_empty(), "vo suffix forbidden in agent layer");
    assert_eq!(results.values[0].code.code(), "AES102");
}

// ─── Flexible Policy (no strict) ──────────────────────────

#[tokio::test]
async fn capabilities_any_suffix_passes_flexible() {
    let config = default_config();
    let lm = flexible_capabilities_layer();
    let results = run_check(&["capabilities_user_checker.rs"], &config, &lm).await;
    assert!(
        results.is_empty(),
        "capabilities is flexible — any suffix OK"
    );
}

#[tokio::test]
async fn capabilities_custom_suffix_passes_flexible() {
    let config = default_config();
    let lm = flexible_capabilities_layer();
    let results = run_check(&["capabilities_data_transformer.rs"], &config, &lm).await;
    assert!(results.is_empty());
}

// ─── Barrel / Entry Point Exceptions ──────────────────────

#[tokio::test]
async fn barrel_file_mod_rs_skipped() {
    let config = default_config();
    let lm = strict_taxonomy_layer();
    let results = run_check(&["mod.rs"], &config, &lm).await;
    assert!(results.is_empty(), "mod.rs is skipped");
}

#[tokio::test]
async fn entry_point_lib_rs_skipped() {
    let config = default_config();
    let lm = strict_taxonomy_layer();
    let results = run_check(&["lib.rs"], &config, &lm).await;
    assert!(results.is_empty(), "lib.rs is skipped");
}

// ─── Layer Definition Exceptions ──────────────────────────

#[tokio::test]
async fn exception_file_in_definition_skipped() {
    let mut map = HashMap::new();
    let def = LayerDefinition {
        naming: NamingRuleVO {
            suffix_policy: SuffixPolicyVO::new("strict".to_string()),
            allowed_suffix: PatternList::new(vec!["vo".to_string()]),
            ..Default::default()
        },
        exceptions: PatternList::new(vec!["taxonomy_special.rs".to_string()]),
        ..Default::default()
    };
    map.insert(LayerNameVO::new("taxonomy"), def);
    let lm = LayerMapVO::new(map);

    let config = default_config();
    let results = run_check(&["taxonomy_special.rs"], &config, &lm).await;
    assert!(results.is_empty(), "Exception file should be skipped");
}

// ─── Edge Cases ───────────────────────────────────────────

#[tokio::test]
async fn no_layer_definition_returns_no_violation() {
    let config = default_config();
    let lm = LayerMapVO::new(HashMap::new()); // empty layer map
    let results = run_check(&["unknown_file_thing.rs"], &config, &lm).await;
    assert!(results.is_empty(), "No layer def → no suffix check");
}

#[tokio::test]
async fn multiple_files_mixed_suffix_results() {
    let config = default_config();
    let lm = strict_taxonomy_layer();
    let results = run_check(
        &[
            "taxonomy_user_vo.rs",          // valid
            "taxonomy_order_entity.rs",     // valid
            "taxonomy_bad_orchestrator.rs", // forbidden suffix
        ],
        &config,
        &lm,
    )
    .await;
    assert_eq!(results.len(), 1, "Only one file should violate");
}

// ─── Cross-Layer Suffix Validation (FR-002: PrefixSuffixMismatch) ─────

fn strict_multi_layer() -> LayerMapVO {
    let mut map = HashMap::new();

    let taxonomy_def = LayerDefinition {
        naming: NamingRuleVO {
            suffix_policy: SuffixPolicyVO::new("strict".to_string()),
            allowed_suffix: PatternList::new(vec![
                "vo".to_string(),
                "entity".to_string(),
                "error".to_string(),
                "event".to_string(),
                "constant".to_string(),
            ]),
            forbidden_suffix: PatternList::new(vec!["orchestrator".to_string()]),
            ..Default::default()
        },
        ..Default::default()
    };
    map.insert(LayerNameVO::new("taxonomy"), taxonomy_def);

    let contract_def = LayerDefinition {
        naming: NamingRuleVO {
            suffix_policy: SuffixPolicyVO::new("strict".to_string()),
            allowed_suffix: PatternList::new(vec!["protocol".to_string(), "aggregate".to_string()]),
            ..Default::default()
        },
        ..Default::default()
    };
    map.insert(LayerNameVO::new("contract"), contract_def);

    let agent_def = LayerDefinition {
        naming: NamingRuleVO {
            suffix_policy: SuffixPolicyVO::new("strict".to_string()),
            allowed_suffix: PatternList::new(vec!["orchestrator".to_string()]),
            ..Default::default()
        },
        ..Default::default()
    };
    map.insert(LayerNameVO::new("agent"), agent_def);

    LayerMapVO::new(map)
}

/// FR-002: taxonomy_user_protocol — suffix 'protocol' belongs to contract layer
#[tokio::test]
async fn cross_layer_taxonomy_protocol_fails_prefix_suffix_mismatch() {
    let config = default_config();
    let lm = strict_multi_layer();
    let results = run_check(&["taxonomy_user_protocol.rs"], &config, &lm).await;
    let aes102: Vec<_> = results
        .values
        .iter()
        .filter(|r| r.code.code() == "AES102")
        .collect();
    assert!(
        !aes102.is_empty(),
        "taxonomy + contract suffix should produce AES102"
    );
}

/// FR-002: contract_user_vo — suffix 'vo' belongs to taxonomy layer
#[tokio::test]
async fn cross_layer_contract_vo_fails_prefix_suffix_mismatch() {
    let config = default_config();
    let lm = strict_multi_layer();
    let results = run_check(&["contract_user_vo.rs"], &config, &lm).await;
    let aes102: Vec<_> = results
        .values
        .iter()
        .filter(|r| r.code.code() == "AES102")
        .collect();
    assert!(
        !aes102.is_empty(),
        "contract + taxonomy suffix should produce AES102"
    );
}

/// FR-002: taxonomy_user_orchestrator — suffix 'orchestrator' belongs to agent layer
/// (also forbidden in taxonomy)
#[tokio::test]
async fn cross_layer_taxonomy_orchestrator_fails() {
    let config = default_config();
    let lm = strict_multi_layer();
    let results = run_check(&["taxonomy_user_orchestrator.rs"], &config, &lm).await;
    let aes102: Vec<_> = results
        .values
        .iter()
        .filter(|r| r.code.code() == "AES102")
        .collect();
    assert!(
        !aes102.is_empty(),
        "taxonomy + agent suffix should produce AES102"
    );
}

/// FR-002: agent_naming_vo — suffix 'vo' belongs to taxonomy layer
#[tokio::test]
async fn cross_layer_agent_vo_fails() {
    let config = default_config();
    let lm = strict_multi_layer();
    let results = run_check(&["agent_naming_vo.rs"], &config, &lm).await;
    let aes102: Vec<_> = results
        .values
        .iter()
        .filter(|r| r.code.code() == "AES102")
        .collect();
    assert!(
        !aes102.is_empty(),
        "agent + taxonomy suffix should produce AES102"
    );
}

/// FR-002: Same-layer suffix should NOT produce PrefixSuffixMismatch
#[tokio::test]
async fn same_layer_suffix_no_violation() {
    let config = default_config();
    let lm = strict_multi_layer();
    let results = run_check(&["taxonomy_user_vo.rs"], &config, &lm).await;
    assert!(results.is_empty(), "taxonomy + taxonomy suffix should pass");
}
