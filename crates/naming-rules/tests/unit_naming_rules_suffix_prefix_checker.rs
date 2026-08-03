// Unit tests for SuffixPrefixChecker — AES102 suffix/prefix rules.
use naming_rules_lint_arwaky::capabilities_suffix_prefix_checker::SuffixPrefixChecker;
use shared::common::{LayerDefinition, LayerMapVO, LayerNameVO, PatternList, SuffixPolicyVO};
use shared::naming_rules::SUFFIX_POLICY_STRICT;
use std::collections::HashMap;

fn checker() -> SuffixPrefixChecker {
    SuffixPrefixChecker::new()
}

fn layer_map_with_strict_capabilities() -> LayerMapVO {
    let mut def = LayerDefinition::default();
    def.naming.suffix_policy = SuffixPolicyVO::new(SUFFIX_POLICY_STRICT.to_string());
    def.naming.allowed_suffix =
        PatternList::new(vec!["checker".to_string(), "adapter".to_string()]);
    def.naming.forbidden_suffix = PatternList::new(vec!["vo".to_string()]);
    let mut layers = HashMap::new();
    layers.insert(LayerNameVO::new("capabilities"), def);
    LayerMapVO::new(layers)
}

fn layer_def(map: &LayerMapVO) -> &LayerDefinition {
    map.values.get(&LayerNameVO::new("capabilities")).unwrap()
}

#[test]
fn construction_succeeds() {
    let _ = checker();
}

#[test]
fn allowed_suffix_no_violation() {
    let map = layer_map_with_strict_capabilities();
    let suffix_map = SuffixPrefixChecker::build_suffix_to_layer_map(&map);
    let all = SuffixPrefixChecker::build_all_suffixes(&map);
    let result = checker()._check_domain_suffixes(
        "src/capabilities_user_checker.rs",
        "capabilities_user_checker.rs",
        Some(layer_def(&map)),
        &Some(LayerNameVO::new("capabilities")),
        &suffix_map,
        &all,
    );
    assert!(result.is_none());
}

#[test]
fn forbidden_suffix_produces_violation() {
    let map = layer_map_with_strict_capabilities();
    let suffix_map = SuffixPrefixChecker::build_suffix_to_layer_map(&map);
    let all = SuffixPrefixChecker::build_all_suffixes(&map);
    let result = checker()._check_domain_suffixes(
        "src/capabilities_user_vo.rs",
        "capabilities_user_vo.rs",
        Some(layer_def(&map)),
        &Some(LayerNameVO::new("capabilities")),
        &suffix_map,
        &all,
    );
    assert!(
        result.is_some(),
        "forbidden suffix 'vo' must produce a violation"
    );
}

#[test]
fn strict_policy_wrong_suffix_produces_violation() {
    let map = layer_map_with_strict_capabilities();
    let suffix_map = SuffixPrefixChecker::build_suffix_to_layer_map(&map);
    let all = SuffixPrefixChecker::build_all_suffixes(&map);
    let result = checker()._check_domain_suffixes(
        "src/capabilities_user_handler.rs",
        "capabilities_user_handler.rs",
        Some(layer_def(&map)),
        &Some(LayerNameVO::new("capabilities")),
        &suffix_map,
        &all,
    );
    assert!(
        result.is_some(),
        "suffix not in allowed list under strict policy must produce a violation"
    );
}

#[test]
fn barrel_file_skipped() {
    let map = layer_map_with_strict_capabilities();
    let suffix_map = SuffixPrefixChecker::build_suffix_to_layer_map(&map);
    let all = SuffixPrefixChecker::build_all_suffixes(&map);
    let result = checker()._check_domain_suffixes(
        "src/capabilities/mod.rs",
        "mod.rs",
        Some(layer_def(&map)),
        &Some(LayerNameVO::new("capabilities")),
        &suffix_map,
        &all,
    );
    assert!(result.is_none(), "barrel files must be skipped");
}

#[test]
fn unknown_suffix_strict_produces_violation() {
    let map = layer_map_with_strict_capabilities();
    let suffix_map = SuffixPrefixChecker::build_suffix_to_layer_map(&map);
    let all = SuffixPrefixChecker::build_all_suffixes(&map);
    let result = checker()._check_domain_suffixes(
        "src/capabilities_user_foo.rs",
        "capabilities_user_foo.rs",
        Some(layer_def(&map)),
        &Some(LayerNameVO::new("capabilities")),
        &suffix_map,
        &all,
    );
    assert!(
        result.is_some(),
        "unknown suffix under strict policy must produce violation"
    );
}

#[test]
fn unknown_suffix_flexible_no_violation() {
    let mut def = LayerDefinition::default();
    def.naming.suffix_policy = SuffixPolicyVO::new("flexible".to_string());
    def.naming.forbidden_suffix = PatternList::new(vec!["vo".to_string()]);
    let mut layers = HashMap::new();
    layers.insert(LayerNameVO::new("capabilities"), def);
    let map = LayerMapVO::new(layers);
    let suffix_map = SuffixPrefixChecker::build_suffix_to_layer_map(&map);
    let all = SuffixPrefixChecker::build_all_suffixes(&map);
    let result = checker()._check_domain_suffixes(
        "src/capabilities_user_foo.rs",
        "capabilities_user_foo.rs",
        Some(layer_def(&map)),
        &Some(LayerNameVO::new("capabilities")),
        &suffix_map,
        &all,
    );
    assert!(
        result.is_none(),
        "unknown suffix under flexible policy must not produce violation"
    );
}

#[test]
fn unknown_prefix_produces_violation() {
    let result = checker()._check_unknown_prefix("src/foo_bar_baz.rs", "foo_bar_baz.rs");
    assert!(
        result.is_some(),
        "unknown prefix must produce AES102 violation"
    );
}

#[test]
fn unknown_prefix_barrel_skipped() {
    let result = checker()._check_unknown_prefix("src/foo/mod.rs", "mod.rs");
    assert!(
        result.is_none(),
        "barrel files must be skipped for unknown prefix"
    );
}
