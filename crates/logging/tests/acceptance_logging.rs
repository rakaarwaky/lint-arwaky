// PURPOSE: acceptance tests — maps to FR-Logging-001/002 in the FRD.
// FR-Logging-001: filter_directive returns the correct directive string
//   per LogVerbosity level.
// FR-Logging-002: init installs the global subscriber without panic.
use logging_lint_arwaky::LogVerbosity;
use logging_lint_arwaky::root_logging_container::LoggingContainer;

// ─── FR-Logging-001: filter_directive per level ────────────────────────

#[test]
fn fr_001_default_filter_directive() {
    // CLI is quiet by default — no audit events unless verbose.
    assert_eq!(
        LogVerbosity::Default.filter_directive(),
        "warn"
    );
}

#[test]
fn fr_001_info_filter_directive() {
    // Verbose (`-v`) enables the real-time scan feed.
    assert_eq!(
        LogVerbosity::Info.filter_directive(),
        "warn,lint_arwaky::audit=info"
    );
}

#[test]
fn fr_001_debug_filter_directive() {
    assert_eq!(
        LogVerbosity::Debug.filter_directive(),
        "debug,lint_arwaky::audit=debug"
    );
}

// ─── FR-Logging-002: init installs without panic ──────────────────────

#[test]
fn fr_002_init_does_not_panic() {
    let container = LoggingContainer::new();
    container.init(LogVerbosity::Default, true);
}
