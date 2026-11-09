// BE-4-01 — the watch session must be able to fail.
//
// `run_watch_loop` returned `ExitCode::OK` on every path: the initial scan's
// score was computed and dropped, `lint_changed`'s `Result` was discarded with
// `let _ =`, and the only non-OK returns were for a runtime that could not be
// built. A `--watch` run in a pre-commit hook therefore could not fail a
// commit, whatever the tree looked like.
use file_watch_lint_arwaky::capabilities_change_lint::ChangeLintHandler;
use shared_file_watch::contract_watch_protocol::IChangeLintProtocol;
use shared_file_watch::taxonomy_file_watch_vo::{ChangeLintSummary, WatchEvent, WatchEventKind};
use std::sync::Arc;

fn handler() -> ChangeLintHandler {
    let container = quality_rules::CodeAnalysisContainer::new();
    ChangeLintHandler::new(container.code_analysis_linter())
}

#[test]
fn lint_changed_reports_a_summary_instead_of_unit() {
    // The signature is the fix: the caller cannot decide an exit code from
    // `()`. This test fails to compile if the trait regresses.
    let event = WatchEvent::new("src/main.rs".to_string(), WatchEventKind::Modified);
    let summary: ChangeLintSummary = handler()
        .lint_changed(&event)
        .expect("linting a path must not error");
    assert_eq!(summary.path, "src/main.rs");
    assert!(summary.score <= 100.0, "score must be a percentage");
}

#[test]
fn lint_changed_keeps_the_path_on_an_unparseable_event() {
    // An empty path is not a violation, but it must still come back as a
    // summary for that path rather than as a silent unit.
    let event = WatchEvent::new(String::new(), WatchEventKind::Modified);
    let summary = handler().lint_changed(&event).expect("must not error");
    assert_eq!(summary.path, "");
    assert_eq!(summary.violations, 0);
    assert!(!summary.fails_policy(80.0));
}

#[test]
fn a_summary_with_a_critical_violation_fails_policy_at_any_threshold() {
    let summary = ChangeLintSummary::new("src/x.rs", 1, 1, 100.0);
    assert!(
        summary.fails_policy(80.0),
        "a CRITICAL violation auto-fails, like `ci`"
    );
    assert!(summary.fails_policy(0.0));
}

#[test]
fn a_summary_below_the_threshold_fails_policy() {
    let summary = ChangeLintSummary::new("src/x.rs", 2, 0, 79.0);
    assert!(summary.fails_policy(80.0));
}

#[test]
fn a_clean_summary_passes_policy() {
    let summary = ChangeLintSummary::new("src/x.rs", 0, 0, 100.0);
    assert!(!summary.fails_policy(80.0));
}

#[test]
fn the_orchestrator_is_constructible_through_the_container() {
    // Wiring check: the orchestrator now stores the same `Arc`s the container
    // hands out, so a container-built aggregate must still be usable.
    let container = file_watch_lint_arwaky::root_file_watch_container::FileWatchContainer::new();
    let qa = quality_rules::CodeAnalysisContainer::new();
    let _agg: Arc<dyn shared_file_watch::contract_watch_aggregate::IWatchAggregate> =
        container.aggregate(qa.code_analysis_linter());
}
