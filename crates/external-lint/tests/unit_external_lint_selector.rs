// PURPOSE: Unit tests for CapabilitiesExternalLintSelector — pure business logic
// mapping language flags to adapter name lists.

use external_lint_lint_arwaky::capabilities_external_lint_selector::CapabilitiesExternalLintSelector;
use shared::common::taxonomy_adapter_list_vo::AdapterNameList;
use shared::common::taxonomy_common_vo::BooleanVO;
use shared::external_lint::contract_external_lint_selector_protocol::IExternalLintSelectorProtocol;

fn sut() -> CapabilitiesExternalLintSelector {
    CapabilitiesExternalLintSelector::with_defaults()
}

// ─── Happy Path ───────────────────────────────────────────

#[test]
fn select_all_languages_returns_nine_adapters() {
    let selector = sut();
    let result = selector.select_adapters(
        BooleanVO::new(true),
        BooleanVO::new(true),
        BooleanVO::new(true),
    );
    assert_eq!(result.len(), 9);
}

#[test]
fn select_rust_only_returns_three_adapters() {
    let selector = sut();
    let result = selector.select_adapters(
        BooleanVO::new(true),
        BooleanVO::new(false),
        BooleanVO::new(false),
    );
    assert_eq!(result.len(), 3);
    let names: Vec<&str> = result.iter().map(|n| n.value()).collect();
    assert!(names.contains(&"clippy"));
    assert!(names.contains(&"rustfmt"));
    assert!(names.contains(&"cargo-audit"));
}

#[test]
fn select_python_only_returns_three_adapters() {
    let selector = sut();
    let result = selector.select_adapters(
        BooleanVO::new(false),
        BooleanVO::new(true),
        BooleanVO::new(false),
    );
    assert_eq!(result.len(), 3);
    let names: Vec<&str> = result.iter().map(|n| n.value()).collect();
    assert!(names.contains(&"ruff"));
    assert!(names.contains(&"mypy"));
    assert!(names.contains(&"bandit"));
}

#[test]
fn select_js_only_returns_three_adapters() {
    let selector = sut();
    let result = selector.select_adapters(
        BooleanVO::new(false),
        BooleanVO::new(false),
        BooleanVO::new(true),
    );
    assert_eq!(result.len(), 3);
    let names: Vec<&str> = result.iter().map(|n| n.value()).collect();
    assert!(names.contains(&"eslint"));
    assert!(names.contains(&"prettier"));
    assert!(names.contains(&"tsc"));
}

// ─── Edge Cases ───────────────────────────────────────────

#[test]
fn select_no_languages_returns_empty_list() {
    let selector = sut();
    let result = selector.select_adapters(
        BooleanVO::new(false),
        BooleanVO::new(false),
        BooleanVO::new(false),
    );
    assert!(result.is_empty());
}

#[test]
fn select_rust_and_python_returns_six_adapters() {
    let selector = sut();
    let result = selector.select_adapters(
        BooleanVO::new(true),
        BooleanVO::new(true),
        BooleanVO::new(false),
    );
    assert_eq!(result.len(), 6);
}

#[test]
fn select_python_and_js_returns_six_adapters() {
    let selector = sut();
    let result = selector.select_adapters(
        BooleanVO::new(false),
        BooleanVO::new(true),
        BooleanVO::new(true),
    );
    assert_eq!(result.len(), 6);
}

// ─── Custom Adapter Lists ─────────────────────────────────

#[test]
fn custom_adapter_lists_are_respected() {
    use shared::common::taxonomy_adapter_name_vo::AdapterName;

    let selector = CapabilitiesExternalLintSelector::new(
        vec![AdapterName::raw("custom-rs")],
        vec![],
        vec![
            AdapterName::raw("custom-js-1"),
            AdapterName::raw("custom-js-2"),
        ],
    );

    let result = selector.select_adapters(
        BooleanVO::new(true),
        BooleanVO::new(false),
        BooleanVO::new(true),
    );
    assert_eq!(result.len(), 3);
    let names: Vec<&str> = result.iter().map(|n| n.value()).collect();
    assert!(names.contains(&"custom-rs"));
    assert!(names.contains(&"custom-js-1"));
    assert!(names.contains(&"custom-js-2"));
}

#[test]
fn empty_custom_lists_return_nothing() {
    let selector = CapabilitiesExternalLintSelector::new(vec![], vec![], vec![]);
    let result = selector.select_adapters(
        BooleanVO::new(true),
        BooleanVO::new(true),
        BooleanVO::new(true),
    );
    assert!(result.is_empty());
}

// ─── Ordering ─────────────────────────────────────────────

#[test]
fn adapter_order_is_rust_then_python_then_js() {
    let selector = sut();
    let result = selector.select_adapters(
        BooleanVO::new(true),
        BooleanVO::new(true),
        BooleanVO::new(true),
    );
    let names: Vec<&str> = result.iter().map(|n| n.value()).collect();
    // Rust adapters first
    assert_eq!(names[0], "clippy");
    assert_eq!(names[1], "rustfmt");
    assert_eq!(names[2], "cargo-audit");
    // Python adapters next
    assert_eq!(names[3], "ruff");
    assert_eq!(names[4], "mypy");
    assert_eq!(names[5], "bandit");
    // JS adapters last
    assert_eq!(names[6], "eslint");
    assert_eq!(names[7], "prettier");
    assert_eq!(names[8], "tsc");
}
