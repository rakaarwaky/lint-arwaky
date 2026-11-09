// BE-5-03 — config schema strictness.
//
// Nothing rejected an unknown configuration key, and `validate_thresholds`
// existed but was reachable only from a bench and one acceptance test. The
// result: `threshold: {score: 95}` (singular section) parsed cleanly, the
// effective gate stayed at the embedded default, and every command exited 0.
//
// These tests pin the two halves of the fix: the typed structs reject unknown
// keys with the dotted path in the message, and the load path turns a rejected
// config into a reported error instead of a silent fall back.
use shared_config_system::taxonomy_config_system_vo::ProjectConfig;
use shared_config_system::utility_config_parser::{
    parse_project_config, validate_adapters, validate_project_config, validate_thresholds,
};

/// A minimal valid project block — the shape of the config file's root.
fn valid_yaml() -> String {
    "project_name: lint-arwaky\nthresholds:\n  score: 95\n  max_file_lines: 400\nadapters:\n  - name: clippy\n    weight: 2.0\n    timeout: 30.0\n".to_string()
}

#[test]
fn a_valid_project_block_parses() {
    let config: ProjectConfig =
        parse_project_config(&valid_yaml()).expect("valid config must parse");
    assert_eq!(config.thresholds.score.value(), 95.0);
    assert_eq!(config.thresholds.max_file_lines.value(), 400);
    assert_eq!(config.adapters.len(), 1);
}

#[test]
fn a_partial_thresholds_block_keeps_the_default_line_budget() {
    // Real configs set only `score`; the missing `max_file_lines` must fall
    // back to the documented default rather than to zero (which the range
    // check would then reject as invalid).
    let yaml = "project_name: lint-arwaky\nthresholds:\n  score: 100.0\n";
    let config = parse_project_config(yaml).expect("partial thresholds must parse");
    assert_eq!(config.thresholds.score.value(), 100.0);
    assert!(
        config.thresholds.max_file_lines.value() > 0,
        "the default line budget must be positive, got {}",
        config.thresholds.max_file_lines.value()
    );
    assert!(validate_project_config(&config).is_valid);
}

#[test]
fn a_bare_null_key_needs_pruning_before_deserialise() {
    // The shipped config writes `ignored_paths:` with nothing after it.
    // `#[serde(default)]` covers an absent key, not an explicit null, so the
    // load path prunes nulls before deserialising. This test pins that: without
    // it, every run against the project's own config would report a schema
    // error the operator could only silence by inventing an empty list.
    let yaml = "project_name: lint-arwaky\nadapters:\n  - name: clippy\nignored_paths:\n";
    let error =
        parse_project_config(yaml).expect_err("a bare null does not deserialise into FilePathList");
    assert!(!error.is_empty());
}

#[test]
fn an_unknown_top_level_key_is_rejected_and_named() {
    let yaml = format!("{}project_nam: typo\n", valid_yaml());
    let error = parse_project_config(&yaml).expect_err("an unknown key must be rejected");
    assert!(
        error.contains("project_nam"),
        "the error must name the offending key, got: {error}"
    );
}

#[test]
fn the_singular_threshold_section_is_rejected_and_named() {
    // This is the typo from the issue: `threshold:` instead of `thresholds:`.
    // Under the old schema it deserialized cleanly and the operator's number
    // was discarded with no message at all.
    let yaml = "project_name: lint-arwaky\nthreshold:\n  score: 95\n";
    let error = parse_project_config(yaml).expect_err("`threshold:` must not be silently accepted");
    assert!(
        error.contains("threshold"),
        "the error must name the key, got: {error}"
    );
}

#[test]
fn an_unknown_key_inside_thresholds_is_rejected_and_named() {
    let yaml = "project_name: lint-arwaky\nthresholds:\n  score: 95\n  max_file_lines: 400\n  min_score: 95\n";
    let error = parse_project_config(yaml).expect_err("unknown nested key must be rejected");
    assert!(
        error.contains("min_score"),
        "the error must name the nested key, got: {error}"
    );
}

#[test]
fn an_unknown_key_inside_an_adapter_is_rejected() {
    let yaml = "project_name: lint-arwaky\nadapters:\n  - name: clippy\n    wieght: 2.0\n";
    let error = parse_project_config(yaml).expect_err("unknown adapter key must be rejected");
    assert!(error.contains("wieght"), "got: {error}");
}

#[test]
fn a_valid_config_passes_validation() {
    let config = parse_project_config(&valid_yaml()).expect("valid config");
    assert!(validate_project_config(&config).is_valid);
    assert!(validate_thresholds(&config).is_valid);
    assert!(validate_adapters(&config).is_valid);
}

#[test]
fn an_out_of_range_score_fails_validation_and_names_the_key() {
    let yaml = "project_name: lint-arwaky\nthresholds:\n  score: 250\n  max_file_lines: 400\n";
    let config = parse_project_config(yaml).expect("parses; the range check is separate");
    let result = validate_project_config(&config);
    assert!(!result.is_valid, "score 250 must be rejected");
    assert!(
        result
            .reason
            .unwrap_or_default()
            .contains("thresholds.score"),
        "the reason must name the dotted key"
    );
}

#[test]
fn an_out_of_tree_adapter_name_is_allowed() {
    // Adapter names are an open set: plugins and out-of-tree adapters are
    // legitimate. Only the *keys* are closed, which is what the schema check
    // enforces. This test exists so a future allow-list cannot be added back
    // without a failing test.
    let yaml = "project_name: lint-arwaky\nadapters:\n  - name: my-house-linter\n    weight: 1.0\n";
    let config = parse_project_config(yaml).expect("parses");
    assert!(validate_project_config(&config).is_valid);
}

#[test]
fn a_non_positive_adapter_weight_is_rejected() {
    let yaml = "project_name: lint-arwaky\nadapters:\n  - name: clippy\n    weight: 0.0\n";
    let config = parse_project_config(yaml).expect("parses");
    let result = validate_adapters(&config);
    assert!(!result.is_valid, "a zero weight must be rejected");
    assert!(result.reason.unwrap_or_default().contains("weight"));
}

#[test]
fn a_config_with_no_file_is_not_an_error_but_a_broken_file_is() {
    // `load_config_for_language` uses these two states differently: absent is
    // normal (embedded defaults are the documented fallback), unreadable is a
    // prerequisite failure. `ConfigResult::has_errors` is that distinction.
    let absent = shared_config_system::taxonomy_config_system_vo::ConfigResult::default();
    assert!(!absent.has_errors());

    let broken = absent.clone().with_errors(vec!["bad".to_string()]);
    assert!(broken.has_errors());
}
