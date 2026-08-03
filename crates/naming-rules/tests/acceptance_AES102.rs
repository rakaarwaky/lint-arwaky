// Acceptance tests — AES102 suffix/prefix rules (map to FRD user stories).
use naming_rules_lint_arwaky::capabilities_suffix_prefix_checker::SuffixPrefixChecker;
use shared::common::taxonomy_definition_vo::{LayerDefinition, LayerMapVO};
use shared::common::taxonomy_layer_vo::LayerNameVO;
use shared::common::taxonomy_lint_result_vo::LintResultList;
use shared::common::taxonomy_path_vo::FilePath;
use shared::common::taxonomy_paths_vo::FilePathList;
use shared::common::PatternList;
use shared::common::SuffixPolicyVO;
use shared::naming_rules::RULE_CODE_SUFFIX_PREFIX;
use shared::naming_rules::SUFFIX_POLICY_STRICT;
use std::collections::HashMap;

fn checker() -> SuffixPrefixChecker {
    SuffixPrefixChecker::new()
}

fn strict_capabilities_layer_map() -> LayerMapVO {
    let mut def = LayerDefinition::default();
    def.naming.suffix_policy = SuffixPolicyVO::new(SUFFIX_POLICY_STRICT.to_string());
    def.naming.allowed_suffix =
        PatternList::new(vec!["checker".to_string(), "adapter".to_string()]);
    def.naming.forbidden_suffix = PatternList::new(vec!["vo".to_string()]);

    let mut layers = HashMap::new();
    layers.insert(LayerNameVO::new("capabilities"), def);
    LayerMapVO::new(layers)
}

fn multi_layer_map() -> LayerMapVO {
    let mut cap_def = LayerDefinition::default();
    cap_def.naming.suffix_policy = SuffixPolicyVO::new(SUFFIX_POLICY_STRICT.to_string());
    cap_def.naming.allowed_suffix =
        PatternList::new(vec!["checker".to_string(), "adapter".to_string()]);
    cap_def.naming.forbidden_suffix = PatternList::new(vec![]);

    let mut agent_def = LayerDefinition::default();
    agent_def.naming.suffix_policy = SuffixPolicyVO::new(SUFFIX_POLICY_STRICT.to_string());
    agent_def.naming.allowed_suffix =
        PatternList::new(vec!["orchestrator".to_string(), "runner".to_string()]);
    agent_def.naming.forbidden_suffix = PatternList::new(vec![]);

    let mut layers = HashMap::new();
    layers.insert(LayerNameVO::new("capabilities"), cap_def);
    layers.insert(LayerNameVO::new("agent"), agent_def);
    LayerMapVO::new(layers)
}

// ── FR-AES102-01: Missing layer prefix produces violation ─

#[test]
fn unknown_prefix_produces_violation() {
    let result = checker()._check_unknown_prefix("src/foo_bar_baz.rs", "foo_bar_baz.rs");
    assert!(result.is_some(), "unknown prefix must produce AES102 violation");
    assert_eq!(result.unwrap().code.value(), RULE_CODE_SUFFIX_PREFIX);
}

#[test]
fn recognised_prefix_no_violation() {
    let result = checker()._check_unknown_prefix(
        "src/capabilities_user_checker.rs",
        "capabilities_user_checker.rs",
    );
    assert!(result.is_none(), "recognised prefix must not produce violation");
}

#[test]
fn unknown_prefix_barrel_file_skipped() {
    let result = checker()._check_unknown_prefix("src/foo/mod.rs", "mod.rs");
    assert!(result.is_none(), "barrel files must be skipped");
}

#[test]
fn all_layer_prefixes_recognised() {
    use shared::naming_rules::LAYER_PREFIXES;
    for prefix in LAYER_PREFIXES {
        let filename = format!("{}foo_bar.rs", prefix);
        let result = checker()._check_unknown_prefix(&format!("src/{}", filename), &filename);
        assert!(
            result.is_none(),
            "prefix '{}' should be recognised",
            prefix
        );
    }
}

// ── FR-AES102-02: Wrong suffix for layer produces violation ─

#[test]
fn wrong_suffix_for_layer_produces_violation() {
    let map = strict_capabilities_layer_map();
    let suffix_map = SuffixPrefixChecker::build_suffix_to_layer_map(&map);
    let all = SuffixPrefixChecker::build_all_suffixes(&map);
    let def = map.values.get(&LayerNameVO::new("capabilities")).unwrap();

    let result = checker()._check_domain_suffixes(
        "src/capabilities_user_handler.rs",
        "capabilities_user_handler.rs",
        Some(def),
        &Some(LayerNameVO::new("capabilities")),
        &suffix_map,
        &all,
    );
    assert!(result.is_some(), "suffix 'handler' not in allowed list must fail");
    assert_eq!(result.unwrap().code.value(), RULE_CODE_SUFFIX_PREFIX);
}

#[test]
fn correct_suffix_for_layer_passes() {
    let map = strict_capabilities_layer_map();
    let suffix_map = SuffixPrefixChecker::build_suffix_to_layer_map(&map);
    let all = SuffixPrefixChecker::build_all_suffixes(&map);
    let def = map.values.get(&LayerNameVO::new("capabilities")).unwrap();

    let result = checker()._check_domain_suffixes(
        "src/capabilities_user_checker.rs",
        "capabilities_user_checker.rs",
        Some(def),
        &Some(LayerNameVO::new("capabilities")),
        &suffix_map,
        &all,
    );
    assert!(result.is_none(), "suffix 'checker' is allowed for capabilities");
}

// ── FR-AES102-03: Forbidden suffix detected ───────────────

#[test]
fn forbidden_suffix_produces_violation() {
    let map = strict_capabilities_layer_map();
    let suffix_map = SuffixPrefixChecker::build_suffix_to_layer_map(&map);
    let all = SuffixPrefixChecker::build_all_suffixes(&map);
    let def = map.values.get(&LayerNameVO::new("capabilities")).unwrap();

    let result = checker()._check_domain_suffixes(
        "src/capabilities_user_vo.rs",
        "capabilities_user_vo.rs",
        Some(def),
        &Some(LayerNameVO::new("capabilities")),
        &suffix_map,
        &all,
    );
    assert!(result.is_some(), "forbidden suffix 'vo' must produce violation");
}

// ── FR-AES102-04: Cross-layer suffix violation ────────────

#[test]
fn cross_layer_suffix_violation_detected() {
    let map = multi_layer_map();
    let suffix_map = SuffixPrefixChecker::build_suffix_to_layer_map(&map);
    let all = SuffixPrefixChecker::build_all_suffixes(&map);
    let cap_def = map.values.get(&LayerNameVO::new("capabilities")).unwrap();

    // 'orchestrator' belongs to agent layer, not capabilities
    let result = checker()._check_domain_suffixes(
        "src/capabilities_user_orchestrator.rs",
        "capabilities_user_orchestrator.rs",
        Some(cap_def),
        &Some(LayerNameVO::new("capabilities")),
        &suffix_map,
        &all,
    );
    assert!(
        result.is_some(),
        "cross-layer suffix 'orchestrator' (agent) in capabilities must fail"
    );
}

#[test]
fn same_layer_suffix_no_cross_violation() {
    let map = multi_layer_map();
    let suffix_map = SuffixPrefixChecker::build_suffix_to_layer_map(&map);
    let all = SuffixPrefixChecker::build_all_suffixes(&map);
    let agent_def = map.values.get(&LayerNameVO::new("agent")).unwrap();

    // 'orchestrator' belongs to agent layer — file is also agent
    let result = checker()._check_domain_suffixes(
        "src/agent_naming_orchestrator.rs",
        "agent_naming_orchestrator.rs",
        Some(agent_def),
        &Some(LayerNameVO::new("agent")),
        &suffix_map,
        &all,
    );
    assert!(
        result.is_none(),
        "same-layer suffix should not trigger cross-layer violation"
    );
}

// ── FR-AES102-05: Valid file with correct suffix passes ───

#[test]
fn valid_file_with_correct_suffix_passes() {
    let map = strict_capabilities_layer_map();
    let suffix_map = SuffixPrefixChecker::build_suffix_to_layer_map(&map);
    let all = SuffixPrefixChecker::build_all_suffixes(&map);
    let def = map.values.get(&LayerNameVO::new("capabilities")).unwrap();

    let result = checker()._check_domain_suffixes(
        "src/capabilities_db_adapter.rs",
        "capabilities_db_adapter.rs",
        Some(def),
        &Some(LayerNameVO::new("capabilities")),
        &suffix_map,
        &all,
    );
    assert!(result.is_none(), "correct suffix 'adapter' should pass");
}

// ── FR-AES102-06: Flexible policy allows unknown suffix ───

#[test]
fn flexible_policy_allows_unknown_suffix() {
    let mut def = LayerDefinition::default();
    def.naming.suffix_policy = SuffixPolicyVO::new("flexible".to_string());
    def.naming.forbidden_suffix = PatternList::new(vec![]);

    let mut layers = HashMap::new();
    layers.insert(LayerNameVO::new("capabilities"), def);
    let map = LayerMapVO::new(layers);
    let suffix_map = SuffixPrefixChecker::build_suffix_to_layer_map(&map);
    let all = SuffixPrefixChecker::build_all_suffixes(&map);
    let def = map.values.get(&LayerNameVO::new("capabilities")).unwrap();

    let result = checker()._check_domain_suffixes(
        "src/capabilities_user_handler.rs",
        "capabilities_user_handler.rs",
        Some(def),
        &Some(LayerNameVO::new("capabilities")),
        &suffix_map,
        &all,
    );
    assert!(result.is_none(), "flexible policy should allow unknown suffixes");
}

// ── FR-AES102-07: Barrel/entry files skipped ──────────────

#[test]
fn aes102_barrel_file_skipped() {
    let map = strict_capabilities_layer_map();
    let suffix_map = SuffixPrefixChecker::build_suffix_to_layer_map(&map);
    let all = SuffixPrefixChecker::build_all_suffixes(&map);
    let def = map.values.get(&LayerNameVO::new("capabilities")).unwrap();

    let result = checker()._check_domain_suffixes(
        "src/capabilities/mod.rs",
        "mod.rs",
        Some(def),
        &Some(LayerNameVO::new("capabilities")),
        &suffix_map,
        &all,
    );
    assert!(result.is_none(), "barrel file must be skipped");
}

// ── FR-AES102-08: Exception list bypasses suffix check ────

#[test]
fn excepted_file_bypasses_suffix_check() {
    let mut def = LayerDefinition::default();
    def.naming.suffix_policy = SuffixPolicyVO::new(SUFFIX_POLICY_STRICT.to_string());
    def.naming.allowed_suffix =
        PatternList::new(vec!["checker".to_string()]);
    def.naming.forbidden_suffix = PatternList::new(vec![]);
    def.exceptions = PatternList::new(vec!["special_adapter.rs".to_string()]);

    let mut layers = HashMap::new();
    layers.insert(LayerNameVO::new("capabilities"), def);
    let map = LayerMapVO::new(layers);
    let suffix_map = SuffixPrefixChecker::build_suffix_to_layer_map(&map);
    let all = SuffixPrefixChecker::build_all_suffixes(&map);
    let def = map.values.get(&LayerNameVO::new("capabilities")).unwrap();

    let result = checker()._check_domain_suffixes(
        "src/special_adapter.rs",
        "special_adapter.rs",
        Some(def),
        &Some(LayerNameVO::new("capabilities")),
        &suffix_map,
        &all,
    );
    assert!(result.is_none(), "excepted file must bypass suffix check");
}

// ── FR-AES102-09: Convention check via public trait API ───

#[test]
fn check_domain_suffixes_via_trait_api() {
    use shared::naming_rules::ISuffixPrefixChecker;

    let config = shared::config_system::taxonomy_config_vo::ArchitectureConfig::default();
    let map = strict_capabilities_layer_map();
    let files = FilePathList::new(vec![
        FilePath::new("src/capabilities_user_vo.rs".to_string()).unwrap(),
        FilePath::new("src/capabilities_user_checker.rs".to_string()).unwrap(),
    ]);
    let root = FilePath::new(".".to_string()).unwrap();
    let mut results = LintResultList::new(Vec::new());

    checker().check_domain_suffixes(&config, &map, &files, &root, &mut results);

    assert_eq!(
        results.len(),
        1,
        "only the forbidden-suffix file should produce a violation"
    );
    assert_eq!(results.values[0].code.value(), RULE_CODE_SUFFIX_PREFIX);
}
