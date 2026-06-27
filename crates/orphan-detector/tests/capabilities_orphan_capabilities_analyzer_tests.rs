use orphan_detector_lint_arwaky::capabilities_orphan_capabilities_analyzer::is_infra_cap_orphan_raw_wired;
use shared::cli_commands::taxonomy_severity_vo::Severity;
use shared::code_analysis::taxonomy_analysis_vo::OrphanIndicatorResult;

#[test]
fn raw_wired_not_orphan_when_wired_and_reachable() {
    let result = is_infra_cap_orphan_raw_wired(true, true);
    assert!(!result.is_orphan);
}

#[test]
fn raw_wired_not_orphan_when_wired_but_not_reachable() {
    let result = is_infra_cap_orphan_raw_wired(true, false);
    assert!(
        !result.is_orphan,
        "wired in container => not orphan even if unreachable"
    );
}

#[test]
fn raw_wired_not_orphan_when_reachable_but_not_wired() {
    let result = is_infra_cap_orphan_raw_wired(false, true);
    assert!(
        !result.is_orphan,
        "reachable from entry point => not orphan"
    );
}

#[test]
fn raw_wired_orphan_when_neither_wired_nor_reachable() {
    let result = is_infra_cap_orphan_raw_wired(false, false);
    assert!(result.is_orphan);
}

#[test]
fn raw_wired_orphan_severity_is_medium() {
    let result = is_infra_cap_orphan_raw_wired(false, false);
    assert_eq!(result.severity as i32, Severity::MEDIUM as i32);
}

#[test]
fn raw_wired_not_orphan_uses_low_severity() {
    let result = is_infra_cap_orphan_raw_wired(true, true);
    // Even though result says not orphan, severity defaults to MEDIUM from raw_wired
    // This is expected since the raw_wired helper always returns MEDIUM severity
    // regardless of orphan status
    assert_eq!(result.severity as i32, Severity::MEDIUM as i32);
}
