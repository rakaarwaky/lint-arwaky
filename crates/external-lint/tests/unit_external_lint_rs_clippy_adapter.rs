// Unit tests for clippy adapter — severity mapping and lint group classification.
use external_lint_lint_arwaky::capabilities_rs_clippy_adapter::{
    clippy_lint_group, map_clippy_severity,
};

use shared::common::taxonomy_severity_vo::Severity;

#[test]
fn correctness_lint_maps_to_critical() {
    assert_eq!(
        map_clippy_severity("clippy::never_loop", "warning"),
        Severity::CRITICAL
    );
    assert_eq!(
        map_clippy_severity("clippy::unused_imports", "warning"),
        Severity::CRITICAL
    );
    assert_eq!(
        map_clippy_severity("clippy::unreachable", "warning"),
        Severity::CRITICAL
    );
}

#[test]
fn suspicious_lint_maps_to_high() {
    assert_eq!(
        map_clippy_severity("clippy::clone_on_copy", "warning"),
        Severity::HIGH
    );
    assert_eq!(
        map_clippy_severity("clippy::deref_addrof", "warning"),
        Severity::HIGH
    );
}

#[test]
fn style_lint_maps_to_medium() {
    assert_eq!(
        map_clippy_severity("clippy::needless_return", "warning"),
        Severity::MEDIUM
    );
    assert_eq!(
        map_clippy_severity("clippy::collapsible_if", "warning"),
        Severity::MEDIUM
    );
}

#[test]
fn complexity_lint_maps_to_medium() {
    assert_eq!(
        map_clippy_severity("clippy::type_complexity", "warning"),
        Severity::MEDIUM
    );
    assert_eq!(
        map_clippy_severity("clippy::needless_range_loop", "warning"),
        Severity::MEDIUM
    );
}

#[test]
fn perf_lint_maps_to_high() {
    assert_eq!(
        map_clippy_severity("clippy::manual_clamp", "warning"),
        Severity::HIGH
    );
    assert_eq!(
        map_clippy_severity("clippy::map_entry", "warning"),
        Severity::HIGH
    );
}

#[test]
fn pedantic_lint_maps_to_low() {
    assert_eq!(
        map_clippy_severity("clippy::must_use_candidate", "warning"),
        Severity::LOW
    );
    assert_eq!(
        map_clippy_severity("clippy::module_name_repetitions", "warning"),
        Severity::LOW
    );
}

#[test]
fn nursery_lint_maps_to_low() {
    assert_eq!(
        map_clippy_severity("clippy::unused_self", "warning"),
        Severity::LOW
    );
}

#[test]
fn restriction_lint_maps_to_low() {
    assert_eq!(
        map_clippy_severity("clippy::print_stdout", "warning"),
        Severity::LOW
    );
    assert_eq!(
        map_clippy_severity("clippy::unwrap_used", "warning"),
        Severity::LOW
    );
    assert_eq!(
        map_clippy_severity("clippy::dbg_macro", "warning"),
        Severity::LOW
    );
}

#[test]
fn unknown_lint_falls_back_to_level() {
    assert_eq!(
        map_clippy_severity("clippy::some_future_lint", "error"),
        Severity::HIGH
    );
    assert_eq!(
        map_clippy_severity("clippy::some_future_lint", "warning"),
        Severity::MEDIUM
    );
}

#[test]
fn lint_group_determination() {
    assert_eq!(clippy_lint_group("never_loop"), "correctness");
    assert_eq!(clippy_lint_group("clone_on_copy"), "suspicious");
    assert_eq!(clippy_lint_group("needless_return"), "style");
    assert_eq!(clippy_lint_group("type_complexity"), "complexity");
    assert_eq!(clippy_lint_group("manual_clamp"), "perf");
    assert_eq!(clippy_lint_group("must_use_candidate"), "pedantic");
    assert_eq!(clippy_lint_group("unused_self"), "nursery");
    assert_eq!(clippy_lint_group("print_stdout"), "restriction");
    assert_eq!(clippy_lint_group("totally_unknown_lint"), "unknown");
}
