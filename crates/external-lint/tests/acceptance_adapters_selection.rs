// Acceptance tests — verify that the selector correctly picks adapters based on detected languages.
//
// These tests validate the business requirement: given a set of detected languages,
// the selector returns the correct adapter names. The tests use the real
// CapabilitiesExternalLintSelector with its default configuration.

use shared::common::taxonomy_adapter_name_vo::AdapterName;
use shared::external_lint::contract_external_lint_selector_protocol::IExternalLintSelectorProtocol;

use external_lint_lint_arwaky::capabilities_external_lint_selector::CapabilitiesExternalLintSelector;

// ─── Acceptance: Default selector configuration ───────────

#[test]
fn acceptance_selector_has_exactly_nine_default_adapters() {
    let selector = CapabilitiesExternalLintSelector::with_defaults();
    // All languages present → all 9 adapters
    let selected = selector.select_adapters(true, true, true);
    assert_eq!(selected.len(), 9, "Expected 9 adapters for mixed project");
}

#[test]
fn acceptance_rust_adapters_are_clippy_rustfmt_cargo_audit() {
    let selector = CapabilitiesExternalLintSelector::with_defaults();
    let selected = selector.select_adapters(true, false, false);
    let names: Vec<&str> = selected.iter().map(|a| a.value()).collect();
    assert_eq!(names, vec!["clippy", "rustfmt", "cargo-audit"]);
}

#[test]
fn acceptance_python_adapters_are_ruff_mypy_bandit() {
    let selector = CapabilitiesExternalLintSelector::with_defaults();
    let selected = selector.select_adapters(false, true, false);
    let names: Vec<&str> = selected.iter().map(|a| a.value()).collect();
    assert_eq!(names, vec!["ruff", "mypy", "bandit"]);
}

#[test]
fn acceptance_js_adapters_are_eslint_prettier_tsc() {
    let selector = CapabilitiesExternalLintSelector::with_defaults();
    let selected = selector.select_adapters(false, false, true);
    let names: Vec<&str> = selected.iter().map(|a| a.value()).collect();
    assert_eq!(names, vec!["eslint", "prettier", "tsc"]);
}

// ─── Acceptance: Two-language combinations ────────────────

#[test]
fn acceptance_rust_plus_python() {
    let selector = CapabilitiesExternalLintSelector::with_defaults();
    let selected = selector.select_adapters(true, true, false);
    let names: Vec<&str> = selected.iter().map(|a| a.value()).collect();
    assert_eq!(names.len(), 6);
    assert!(names.contains(&"clippy"));
    assert!(names.contains(&"rustfmt"));
    assert!(names.contains(&"cargo-audit"));
    assert!(names.contains(&"ruff"));
    assert!(names.contains(&"mypy"));
    assert!(names.contains(&"bandit"));
    assert!(!names.contains(&"eslint"));
    assert!(!names.contains(&"prettier"));
    assert!(!names.contains(&"tsc"));
}

#[test]
fn acceptance_rust_plus_js() {
    let selector = CapabilitiesExternalLintSelector::with_defaults();
    let selected = selector.select_adapters(true, false, true);
    let names: Vec<&str> = selected.iter().map(|a| a.value()).collect();
    assert_eq!(names.len(), 6);
    assert!(names.contains(&"clippy"));
    assert!(names.contains(&"rustfmt"));
    assert!(names.contains(&"cargo-audit"));
    assert!(names.contains(&"eslint"));
    assert!(names.contains(&"prettier"));
    assert!(names.contains(&"tsc"));
    assert!(!names.contains(&"ruff"));
}

#[test]
fn acceptance_python_plus_js() {
    let selector = CapabilitiesExternalLintSelector::with_defaults();
    let selected = selector.select_adapters(false, true, true);
    let names: Vec<&str> = selected.iter().map(|a| a.value()).collect();
    assert_eq!(names.len(), 6);
    assert!(names.contains(&"ruff"));
    assert!(names.contains(&"mypy"));
    assert!(names.contains(&"bandit"));
    assert!(names.contains(&"eslint"));
    assert!(names.contains(&"prettier"));
    assert!(names.contains(&"tsc"));
    assert!(!names.contains(&"clippy"));
}

// ─── Acceptance: No languages ─────────────────────────────

#[test]
fn acceptance_no_languages_returns_empty() {
    let selector = CapabilitiesExternalLintSelector::with_defaults();
    let selected = selector.select_adapters(false, false, false);
    assert!(selected.is_empty());
}

// ─── Acceptance: Custom selector configuration ────────────

#[test]
fn acceptance_custom_selector_respects_configuration() {
    // A project that only wants clippy and ruff
    let selector = CapabilitiesExternalLintSelector::new(
        vec![AdapterName::raw("clippy")],
        vec![AdapterName::raw("ruff")],
        vec![],
    );
    let selected = selector.select_adapters(true, true, true);
    let names: Vec<&str> = selected.iter().map(|a| a.value()).collect();
    assert_eq!(names, vec!["clippy", "ruff"]);
}

#[test]
fn acceptance_custom_selector_empty_for_disabled_languages() {
    // A project that only wants JS adapters but has no JS
    let selector =
        CapabilitiesExternalLintSelector::new(vec![], vec![], vec![AdapterName::raw("eslint")]);
    let selected = selector.select_adapters(true, true, false);
    assert!(selected.is_empty()); // no JS files → no adapters selected
}

// ─── Acceptance: Adapter names match expected set ─────────

#[test]
fn acceptance_adapter_names_are_lowercase_ascii() {
    let selector = CapabilitiesExternalLintSelector::with_defaults();
    let selected = selector.select_adapters(true, true, true);
    for name in selected.iter() {
        let val = name.value();
        assert!(
            val.chars().all(|c| c.is_ascii_lowercase() || c == '-'),
            "Adapter name '{}' should be lowercase ASCII with hyphens only",
            val
        );
    }
}

#[test]
fn acceptance_no_duplicate_adapters_across_languages() {
    let selector = CapabilitiesExternalLintSelector::with_defaults();
    let selected = selector.select_adapters(true, true, true);
    let names: Vec<&str> = selected.iter().map(|a| a.value()).collect();
    let mut unique = names.clone();
    unique.sort();
    unique.dedup();
    assert_eq!(
        names.len(),
        unique.len(),
        "Duplicate adapter names detected"
    );
}
