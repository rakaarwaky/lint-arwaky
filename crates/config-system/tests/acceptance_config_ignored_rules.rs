// PURPOSE: `ignored_rules` parsing — the rule-suppression switch every project
// uses to silence an individual external-tool rule (e.g. `markdownlint::MD013`)
// without a second, per-tool config file.
//
// The list is read straight from the YAML rather than through
// `ProjectConfig`, because the external-lint surface walks up for the nearest
// `lint_arwaky.config.yaml` and parses only the keys it needs — the same
// contract `parse_adapter_entries_from_yaml` already has.

use shared_config_system::utility_config_parser::parse_ignored_rules_from_yaml;

#[test]
fn empty_list_parses_to_no_suppressions() {
    assert!(parse_ignored_rules_from_yaml("ignored_rules: []").is_empty());
}

#[test]
fn absent_key_parses_to_no_suppressions() {
    // A config that never mentions the key must not suppress anything: an
    // empty list is the "report everything" state, so a project that adds an
    // adapter without touching this key still sees all of its findings.
    let yaml = "adapters:\n  - name: \"markdownlint\"\n    weight: 1.0\n";
    assert!(parse_ignored_rules_from_yaml(yaml).is_empty());
}

#[test]
fn external_tool_codes_are_read_verbatim() {
    let yaml = "ignored_rules:\n  - \"markdownlint::MD013\"\n  - \"markdownlint::MD060\"\n";
    assert_eq!(
        parse_ignored_rules_from_yaml(yaml),
        vec!["markdownlint::MD013", "markdownlint::MD060"]
    );
}

#[test]
fn aes_codes_and_tool_codes_coexist_in_one_list() {
    // The point of routing tool rules through `ignored_rules` is that a project
    // suppresses by reported code, whatever produced it.
    let yaml = "ignored_rules:\n  - \"AES205\"\n  - \"markdownlint::MD024\"\n";
    assert_eq!(
        parse_ignored_rules_from_yaml(yaml),
        vec!["AES205", "markdownlint::MD024"]
    );
}

#[test]
fn malformed_yaml_degrades_to_no_suppressions() {
    // Failing open is deliberate: a broken config must never silently hide
    // findings. Reporting everything is loud; reporting nothing is not.
    assert!(parse_ignored_rules_from_yaml("ignored_rules: [unclosed").is_empty());
    assert!(parse_ignored_rules_from_yaml("\t\tnot: yaml: at: all").is_empty());
}

#[test]
fn non_list_value_is_treated_as_absent() {
    assert!(parse_ignored_rules_from_yaml("ignored_rules: \"MD013\"").is_empty());
}

#[test]
fn non_string_entries_are_skipped_not_stringified() {
    // `42` is not a rule code. Coercing it would produce a pattern that can
    // never match, silently misleading whoever reads the config.
    let yaml = "ignored_rules:\n  - 42\n  - \"markdownlint::MD013\"\n  - true\n";
    assert_eq!(
        parse_ignored_rules_from_yaml(yaml),
        vec!["markdownlint::MD013"]
    );
}

#[test]
fn duplicate_entries_are_preserved() {
    // Deduplication is the filter's job, not the parser's; keeping the list
    // faithful means the config round-trips as written.
    let yaml = "ignored_rules:\n  - \"MD013\"\n  - \"MD013\"\n";
    assert_eq!(parse_ignored_rules_from_yaml(yaml), vec!["MD013", "MD013"]);
}
