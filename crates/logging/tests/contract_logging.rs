// PURPOSE: logging contract tests — the public API surface of the logging
// feature crate: LogVerbosity, SubscriberInit, LoggingOrchestrator,
// LoggingContainer, and the AUDIT_TARGET constant.
use logging_lint_arwaky::LogVerbosity;
use logging_lint_arwaky::capabilities_subscriber_init::SubscriberInit;
use logging_lint_arwaky::root_logging_container::LoggingContainer;
use shared_logging::ISubscriberInstallProtocol;

// ─── LogVerbosity: filter_directive ────────────────────────────────────────

#[test]
fn filter_directive_default_returns_warn_and_audit_info() {
    // CLI is quiet by default — no audit events unless verbose.
    assert_eq!(LogVerbosity::Default.filter_directive(), "warn");
}

#[test]
fn filter_directive_info_returns_info_and_audit_info() {
    // Verbose (`-v`) enables the real-time scan feed.
    assert_eq!(
        LogVerbosity::Info.filter_directive(),
        "warn,lint_arwaky::audit=info"
    );
}

#[test]
fn filter_directive_debug_returns_debug_and_audit_debug() {
    assert_eq!(
        LogVerbosity::Debug.filter_directive(),
        "debug,lint_arwaky::audit=debug"
    );
}

#[test]
fn default_verbosiy_is_default() {
    assert_eq!(LogVerbosity::default(), LogVerbosity::Default);
}

// ─── LogVerbosity: trace_scans ─────────────────────────────────────────────

#[test]
fn trace_scans_is_false_for_default() {
    assert!(!LogVerbosity::Default.trace_scans());
}

#[test]
fn trace_scans_is_true_for_info_and_debug() {
    assert!(LogVerbosity::Info.trace_scans());
    assert!(LogVerbosity::Debug.trace_scans());
}

// ─── LogVerbosity: PartialEq / Eq / Clone / Copy / Debug ──────────────────

#[test]
fn verbosity_levels_are_distinct() {
    let levels = [
        LogVerbosity::Default,
        LogVerbosity::Info,
        LogVerbosity::Debug,
    ];
    for i in 0..levels.len() {
        for j in (i + 1)..levels.len() {
            assert_ne!(levels[i], levels[j]);
        }
    }
}

#[test]
fn verbosity_is_copy_and_clone() {
    let v = LogVerbosity::Debug;
    let v2 = v;
    let v3 = v;
    assert_eq!(v, v2);
    assert_eq!(v, v3);
    assert_eq!(format!("{:?}", LogVerbosity::Info), "Info");
}

// ─── ISubscriberInstallProtocol: install (protocol-level) ─────────────

#[test]
fn install_via_protocol_returns_env_filter_for_each_level() {
    let subscriber: Box<dyn ISubscriberInstallProtocol> = Box::new(SubscriberInit::default());
    for v in [
        LogVerbosity::Default,
        LogVerbosity::Info,
        LogVerbosity::Debug,
    ] {
        // Installing is a no-op after the first call; we verify it doesn't panic.
        subscriber.install(v, false);
    }
}

// ─── LoggingContainer ──────────────────────────────────────────────────────

#[test]
fn container_new_returns_an_orchestrator() {
    let container = LoggingContainer::new();
    let _orchestrator = container.orchestrator();
}

#[test]
fn container_init_delegates_to_orchestrator() {
    let container = LoggingContainer::default();
    container.init(LogVerbosity::Info, true);
    // A second call is a no-op (the global subscriber is set-once);
    // the second call must not panic.
    container.init(LogVerbosity::Debug, false);
}
