// PURPOSE: Behavioural guards for the hardcoded AES architecture.
//
// The business rules moved out of YAML into
// `utility_architecture_constants::hardcoded_default_architecture`. These tests
// pin the behaviour that matters to the rule engine, so a careless edit to the
// table fails here instead of silently changing what the linter reports.
use shared_config_system::utility_architecture_constants::hardcoded_default_architecture;
use shared_config_system::utility_config_merger::merge_config;

fn merged() -> std::collections::HashMap<
    shared_common::taxonomy_layer_vo::LayerNameVO,
    shared_common::taxonomy_layer_vo::LayerDefinition,
> {
    merge_config(&hardcoded_default_architecture()).0
}

fn layer(name: &str) -> shared_common::taxonomy_layer_vo::LayerDefinition {
    merged()
        .get(&shared_common::taxonomy_layer_vo::LayerNameVO::new(name))
        .cloned()
        .unwrap_or_else(|| panic!("missing layer {name}"))
}

fn rule(code: &str) -> shared_config_system::ArchitectureRule {
    hardcoded_default_architecture()
        .rules
        .into_iter()
        .find(|r| r.rule_type.code() == code)
        .unwrap_or_else(|| panic!("missing rule {code}"))
}

#[test]
fn all_seven_layers_are_defined() {
    let config = hardcoded_default_architecture();
    assert_eq!(config.layers.len(), 7);
    for name in [
        "taxonomy",
        "utility",
        "contract",
        "capabilities",
        "agent",
        "surface",
        "root",
    ] {
        assert!(
            config
                .layers
                .contains_key(&shared_common::taxonomy_layer_vo::LayerNameVO::new(name)),
            "missing layer {name}"
        );
    }
}

#[test]
fn every_aes_rule_is_present_and_enabled() {
    let config = hardcoded_default_architecture();
    for code in [
        "AES101", "AES102", "AES103", "AES201", "AES202", "AES203", "AES204", "AES205", "AES301",
        "AES302", "AES303", "AES304", "AES305", "AES401", "AES402", "AES403", "AES404", "AES405",
        "AES406", "AES501", "AES502", "AES503", "AES504", "AES505", "AES506", "AES601", "AES602",
        "AES603", "AES604", "AES605", "AES701", "AES702", "AES703", "AES704",
    ] {
        assert!(
            config.rules.iter().any(|r| r.rule_type.code() == code),
            "missing {code}"
        );
    }
    assert!(config.rules.iter().all(|r| r.enabled.value));
}

#[test]
fn taxonomy_sub_layers_forbid_upward_dependencies() {
    for suffix in [
        "vo", "request", "response", "entity", "error", "event", "constant",
    ] {
        let def = layer(&format!("taxonomy({suffix})"));
        for forbidden in [
            "agent",
            "surface",
            "contract",
            "utility",
            "capabilities",
            "root",
        ] {
            assert!(
                def.forbidden.values.contains(&forbidden.to_string()),
                "taxonomy({suffix}) must forbid {forbidden}"
            );
        }
    }
}

#[test]
fn contract_protocol_forbids_aggregate_but_aggregate_does_not() {
    let protocol = layer("contract(protocol)");
    assert!(
        protocol
            .forbidden
            .values
            .contains(&"contract(aggregate)".to_string())
    );

    let aggregate = layer("contract(aggregate)");
    assert!(
        !aggregate
            .forbidden
            .values
            .contains(&"contract(aggregate)".to_string())
    );
}

#[test]
fn contract_requires_taxonomy() {
    assert!(
        layer("contract(protocol)")
            .mandatory
            .values
            .contains(&"taxonomy".to_string())
    );
    assert!(
        layer("contract(aggregate)")
            .mandatory
            .values
            .contains(&"taxonomy".to_string())
    );
}

#[test]
fn capabilities_forbids_surface_and_agent() {
    let def = layer("capabilities");
    assert!(def.forbidden.values.contains(&"surface".to_string()));
    assert!(def.forbidden.values.contains(&"agent".to_string()));
}

#[test]
fn the_specified_layers_carry_their_mandatory_imports() {
    // The specification fixes mandatory imports for exactly these five scopes.
    // `surface` deliberately has none, and taxonomy only for entity/error/event.
    let cases: &[(&str, &[&str])] = &[
        ("taxonomy(entity)", &["taxonomy(vo|constant)"]),
        ("taxonomy(error)", &["taxonomy(vo|constant)"]),
        ("taxonomy(event)", &["taxonomy(vo|constant)"]),
        ("contract(protocol)", &["taxonomy"]),
        ("contract(aggregate)", &["taxonomy"]),
        ("capabilities", &["taxonomy", "contract(protocol)"]),
        ("agent(orchestrator)", &["taxonomy", "contract(aggregate)"]),
    ];

    for (scope, required) in cases {
        let def = layer(scope);
        for dependency in *required {
            assert!(
                def.mandatory.values.contains(&dependency.to_string()),
                "{scope} must require {dependency}"
            );
        }
    }
}

#[test]
fn layers_without_specified_mandatory_imports_have_none() {
    for scope in ["surface", "taxonomy(vo)", "utility"] {
        assert!(
            layer(scope).mandatory.values.is_empty(),
            "{scope} must have no mandatory imports"
        );
    }
}

#[test]
fn utility_forbids_everything_except_taxonomy() {
    let def = layer("utility");
    for forbidden in [
        "agent",
        "surface",
        "contract",
        "capabilities",
        "root",
        "utility",
    ] {
        assert!(
            def.forbidden.values.contains(&forbidden.to_string()),
            "utility must forbid {forbidden}"
        );
    }
}

#[test]
fn suffix_policies_match_the_specification() {
    let config = hardcoded_default_architecture();
    let naming = |name: &str| {
        &config.layers[&shared_common::taxonomy_layer_vo::LayerNameVO::new(name)].naming
    };

    for strict in ["taxonomy", "contract", "agent", "surface", "root"] {
        assert_eq!(naming(strict).suffix_policy.value, "strict", "{strict}");
    }
    for flexible in ["utility", "capabilities"] {
        assert_eq!(
            naming(flexible).suffix_policy.value,
            "flexible",
            "{flexible}"
        );
    }
}

#[test]
fn taxonomy_accepts_only_its_seven_suffixes() {
    let def = layer("taxonomy");
    let allowed = &def.naming.allowed_suffix.values;
    for suffix in [
        "vo", "entity", "error", "event", "constant", "request", "response",
    ] {
        assert!(
            allowed.contains(&suffix.to_string()),
            "taxonomy allows {suffix}"
        );
    }
    assert_eq!(allowed.len(), 7);
}

#[test]
fn utility_forbids_taxonomy_suffixes() {
    let def = layer("utility");
    for suffix in ["vo", "entity", "error", "constant", "request", "response"] {
        assert!(
            def.naming
                .forbidden_suffix
                .values
                .contains(&suffix.to_string()),
            "utility must forbid suffix {suffix}"
        );
    }
}

#[test]
fn aes304_keeps_its_bypass_ban_list() {
    let patterns = &rule("AES304").code_analysis.forbidden_bypass.values;
    for banned in [
        "#[allow(",
        "unwrap",
        "expect",
        "panic",
        "todo",
        "unimplemented",
        "noqa",
        "eslint-disable",
    ] {
        assert!(
            patterns.contains(&banned.to_string()),
            "AES304 must ban {banned}"
        );
    }
}

#[test]
fn aes305_keeps_its_health_checker_exception() {
    assert!(
        rule("AES305")
            .exceptions
            .values
            .contains(&"capabilities_adapter_health_checker".to_string())
    );
}

#[test]
fn naming_word_count_default_is_three() {
    assert_eq!(hardcoded_default_architecture().naming.word_count.value, 3);
}

#[test]
fn architecture_is_enabled() {
    assert!(hardcoded_default_architecture().enabled.value);
}
