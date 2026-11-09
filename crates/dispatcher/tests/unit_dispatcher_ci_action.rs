// BE-5-01 — behavioural tests for `collect_ci`.
//
// Before this file, no test in the workspace called `collect_ci` at all: the
// CI gate was verified only by `let _ = collect_ci;` compile checks. That is
// how `ci` came to score four of the ten rule groups without anyone noticing —
// the function type-checked and was never run.
//
// These tests assert the *contract*, not a score: a group that is omitted from
// the run must be visible in `groups_run`.
mod mock_common;

use dispatcher_lint_arwaky::surface_ci_action::{CI_RULE_GROUPS, collect_ci};
use shared_common::{FilePath, Threshold};

fn threshold(value: u32) -> Threshold {
    Threshold::try_new(value).expect("threshold in range")
}

fn fp(path: &str) -> FilePath {
    FilePath::new(path.to_string()).expect("valid path")
}

#[test]
fn ci_reports_every_rule_group_it_scored() {
    // The regression this guards: `ci` used to run quality, import, naming and
    // orphan only, so AES6xx/AES7xx-only trees passed with a score of 100.
    let root = env!("CARGO_MANIFEST_DIR");
    let aggregates = mock_common::build_scan_aggregates(root);
    let report = collect_ci(aggregates, Some(fp(root)), threshold(80))
        .expect("ci run against the workspace root must not error");

    for group in CI_RULE_GROUPS {
        assert!(
            report.groups_run.iter().any(|g| g == group),
            "rule group '{group}' was not scored; ran: {:?}",
            report.groups_run
        );
    }
    assert_eq!(
        report.groups_run.len(),
        CI_RULE_GROUPS.len(),
        "groups_run must describe exactly the groups scored"
    );
    assert!(
        report.groups_run.iter().any(|g| g == "structure"),
        "structure (AES7xx) must reach the gate"
    );
    assert!(
        report.groups_run.iter().any(|g| g == "doc"),
        "doc (AES6xx) must reach the gate"
    );
}

#[test]
fn ci_rejects_a_nonexistent_path() {
    let root = "/definitely/not/a/real/path/lint-arwaky-ci-test";
    let aggregates = mock_common::build_scan_aggregates(root);
    let error = collect_ci(aggregates, Some(fp(root)), threshold(80))
        .expect_err("a missing path must be an error, not a clean run");
    assert!(error.contains("does not exist"), "unexpected error: {error}");
}

#[test]
fn ci_rejects_an_out_of_range_threshold() {
    let root = env!("CARGO_MANIFEST_DIR");
    let aggregates = mock_common::build_scan_aggregates(root);
    // Threshold::try_new is the validator; collect_ci re-checks it because the
    // MCP layer can construct one from a bare integer.
    let result = collect_ci(aggregates, Some(fp(root)), threshold(0));
    assert!(result.is_ok(), "threshold 0 is in range");

    let aggregates = mock_common::build_scan_aggregates(root);
    let bad = Threshold::new(101);
    let error = collect_ci(aggregates, Some(fp(root)), bad)
        .expect_err("threshold above 100 must be rejected");
    assert!(error.contains("threshold") || error.contains("Threshold"), "{error}");
}

#[test]
fn ci_pass_flag_agrees_with_reasons() {
    // `pass` and `reasons` are two encodings of one decision; a report where
    // they disagree is how a failing gate printed "PASS".
    let root = env!("CARGO_MANIFEST_DIR");
    let aggregates = mock_common::build_scan_aggregates(root);
    let report = collect_ci(aggregates, Some(fp(root)), threshold(80)).expect("ci run");

    assert_eq!(
        report.pass,
        report.reasons.is_empty(),
        "pass={} but reasons={:?}",
        report.pass,
        report.reasons
    );
    assert_eq!(
        report.total_violations,
        report.critical + report.high + report.medium + report.low,
        "severity counters must add up to the total"
    );
}
