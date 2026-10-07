// PURPOSE: integration tests — the real LoggingContainer +
// LoggingOrchestrator DI wiring, exercising the full
// container → orchestrator → capability chain.
use logging_lint_arwaky::ILoggingAggregate;
use logging_lint_arwaky::LogVerbosity;
use logging_lint_arwaky::root_logging_container::LoggingContainer;

// ─── LoggingContainer::new() returns an orchestrator ─────────────

#[test]
fn container_new_returns_orchestrator() {
    let container = LoggingContainer::new();
    let _orchestrator = container.orchestrator();
}

// ─── LoggingContainer::init(Default, true) doesn't panic ────────────

#[test]
fn container_init_default_ansi_does_not_panic() {
    let container = LoggingContainer::new();
    container.init(LogVerbosity::Default, true);
}

// ─── Second init call is a no-op ────────────────────────────────────

#[test]
fn second_init_call_is_noop() {
    let container = LoggingContainer::new();
    container.init(LogVerbosity::Info, true);
    // A second call must not panic (global subscriber is set-once).
    container.init(LogVerbosity::Debug, false);
}
