// PURPOSE: unit tests for the logging crate — LogVerbosity filter_directive
// and trace_scans.
use logging_lint_arwaky::LogVerbosity;

// ─── LogVerbosity::filter_directive ─────────────────────────────────────

#[test]
fn filter_directive_default() {
    assert_eq!(
        LogVerbosity::Default.filter_directive(),
        "warn,lint_arwaky::audit=info"
    );
}

#[test]
fn filter_directive_info() {
    assert_eq!(
        LogVerbosity::Info.filter_directive(),
        "info,lint_arwaky::audit=info"
    );
}

#[test]
fn filter_directive_debug() {
    assert_eq!(
        LogVerbosity::Debug.filter_directive(),
        "debug,lint_arwaky::audit=debug"
    );
}

// ─── LogVerbosity::trace_scans ──────────────────────────────────────────

#[test]
fn trace_scans_default_is_false() {
    assert!(!LogVerbosity::Default.trace_scans());
}

#[test]
fn trace_scans_info_and_debug_are_true() {
    assert!(LogVerbosity::Info.trace_scans());
    assert!(LogVerbosity::Debug.trace_scans());
}
