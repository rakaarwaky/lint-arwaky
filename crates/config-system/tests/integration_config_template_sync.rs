// PURPOSE: Guard that the embedded init template (crates/shared/config/lint_arwaky.config.yaml,
// shipped by `lint-arwaky-cli init`) stays in sync with the root
// lint_arwaky.config.yaml single source of truth.
//
// The template must carry at least every rule code and group defined in the
// root config; repo-specific values such as the AES403 exception list are
// intentionally absent from the template and are not compared.

use serde_yaml_ng::{Mapping, Value};

const EMBEDDED: &str = "crates/shared/config/lint_arwaky.config.yaml";
const ROOT: &str = "lint_arwaky.config.yaml";

/// Load a YAML file relative to the repo root (tests run with CWD = crate dir).
fn load_yaml(path: &str) -> Value {
    let full = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
        .join(path);
    let text = std::fs::read_to_string(&full).unwrap_or_else(|e| panic!("cannot read {path}: {e}"));
    serde_yaml_ng::from_str(&text).unwrap_or_else(|e| panic!("cannot parse {path}: {e}"))
}

/// Extract the `architecture.rules` mapping; panic if the shape is missing.
fn rules_of(path: &str) -> Mapping {
    load_yaml(path)
        .get("architecture")
        .and_then(|a| a.get("rules"))
        .and_then(|r| r.as_mapping())
        .cloned()
        .unwrap_or_else(|| panic!("no architecture.rules mapping in {path}"))
}

/// Rule codes of a config file, in source order.
fn rule_codes(path: &str) -> Vec<String> {
    rules_of(path)
        .keys()
        .map(|key| {
            key.as_str()
                .expect("rule code keys are strings")
                .to_string()
        })
        .collect()
}

/// AES group of a rule code, e.g. `AES601` -> `AES6`.
fn group_of(code: &str) -> &str {
    &code[..code.len().saturating_sub(2).max(1)]
}

#[test]
fn embedded_init_template_defines_the_same_rule_groups_as_root_config() {
    let root_codes = rule_codes(ROOT);
    let embedded_codes = rule_codes(EMBEDDED);

    assert!(
        !root_codes.is_empty(),
        "root config {ROOT} has no architecture.rules entries; test is broken"
    );
    assert!(
        !embedded_codes.is_empty(),
        "embedded template {EMBEDDED} has no architecture.rules entries; init would ship an empty template"
    );

    let missing_groups: Vec<String> = root_codes
        .iter()
        .map(|code| group_of(code))
        .filter(|group| embedded_codes.iter().all(|code| group_of(code) != *group))
        .collect::<std::collections::BTreeSet<_>>()
        .into_iter()
        .map(|g| g.to_string())
        .collect();
    assert!(
        missing_groups.is_empty(),
        "embedded init template is missing rule group(s) {missing_groups:?} defined in the root config; \
         sync {EMBEDDED} with {ROOT}"
    );
}

#[test]
fn embedded_init_template_contains_every_root_rule_code() {
    let root_codes = rule_codes(ROOT);
    let embedded: std::collections::BTreeSet<String> = rule_codes(EMBEDDED).into_iter().collect();
    let missing: Vec<String> = root_codes
        .iter()
        .filter(|code| !embedded.contains(*code))
        .cloned()
        .collect();
    assert!(
        missing.is_empty(),
        "embedded init template is missing rule(s) {missing:?} defined in the root config; \
         sync {EMBEDDED} with {ROOT}"
    );
}

#[test]
fn embedded_rule_entries_share_the_root_enabled_state() {
    let root_rules = rules_of(ROOT);
    let embedded_rules = rules_of(EMBEDDED);

    let mut divergences: Vec<String> = Vec::new();
    for code in root_rules
        .keys()
        .map(|key| key.as_str().expect("rule code keys are strings"))
    {
        let Some(embedded_rule) = embedded_rules.get(Value::String(code.to_string())) else {
            continue; // absence is covered by the rule-code test
        };
        let root_enabled = root_rules
            .get(Value::String(code.to_string()))
            .and_then(|rule| rule.get("enabled"));
        if root_enabled.is_some()
            && embedded_rule.get("enabled").is_some()
            && root_enabled != embedded_rule.get("enabled")
        {
            divergences.push(format!(
                "{code} enabled differs: root={root_enabled:?} embedded={:?}",
                embedded_rule.get("enabled")
            ));
        }
    }
    assert!(
        divergences.is_empty(),
        "enabled state drifted between root config and embedded template: {divergences:?}"
    );
}
