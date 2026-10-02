// PURPOSE: Unit tests for the `ignored_rules:` config key — the rule-level
// suppression list that lets a project silence individual linter rules (AES or
// external-tool native) without a separate per-tool config file.
//
// This is the key that carries a project's markdownlint policy, so its parse
// contract matters: a malformed entry must never silently widen suppression.

use shared_config_system::utility_config_parser::parse_ignored_rules_from_yaml;

#[test]
fn absent_key_yields_no_suppressions() {
    let yaml = "adapters:\n  - name: \"clippy\"\n";
    assert!(parse_ignored_rules_from_yaml(yaml).is_empty());
}

#[test]
fn empty_list_yields_no_suppressions() {
    assert!(parse_ignored_rules_from_yaml("ignored_rules: []\n").is_empty());
}

#[test]
fn external_tool_codes_are_read_verbatim() {
    let yaml = "\
ignored_rules:
  - \"markdownlint::MD013\"
  - \"markdownlint::MD060\"
";
    let rules = parse_ignored_rules_from_yaml(yaml);
    assert_eq!(
        rules,
        vec![
            "markdownlint::MD013".to_string(),
            "markdownlint::MD060".to_string()
        ]
    );
}

#[test]
fn aes_codes_mix_freely_with_tool_codes() {
    let yaml = "\
ignored_rules:
  - \"AES205\"
  - \"markdownlint::MD013\"
";
    let rules = parse_ignored_rules_from_yaml(yaml);
    assert_eq!(rules.len(), 2, "both entries survive: {rules:?}");
    assert!(rules.iter().any(|r| r == "AES205"));
    assert!(rules.iter().any(|r| r == "markdownlint::MD013"));
}

#[test]
fn non_string_entries_are_skipped_not_stringified() {
    // A bare number or a nested map cannot be a rule code. Skipping it keeps a
    // typo from becoming the literal string "42" and matching a code by
    // accident.
    let yaml = "\
ignored_rules:
  - 42
  - \"MD013\"
  - true
";
    let rules = parse_ignored_rules_from_yaml(yaml);
    assert_eq!(rules, vec!["MD013".to_string()]);
}

#[test]
fn a_non_list_value_is_treated_as_no_suppressions() {
    // Fail closed: an unreadable shape must not suppress everything.
    let rules = parse_ignored_rules_from_yaml("ignored_rules: \"MD013\"\n");
    assert!(rules.is_empty());
}

#[test]
fn malformed_yaml_yields_no_suppressions() {
    // The dangerous failure mode would be returning *all* rules here, so the
    // fallback is explicitly the empty list: report everything.
    let rules = parse_ignored_rules_from_yaml("ignored_rules: [unclosed\n");
    assert!(rules.is_empty());
}

#[test]
fn an_empty_yaml_document_yields_no_suppressions() {
    assert!(parse_ignored_rules_from_yaml("").is_empty());
}
