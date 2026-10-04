// PURPOSE: Regression test — cooperative scan cancellation actually stops the
// dispatcher early instead of running to completion (fixes #564 / re-closes #366).
//
// The dispatcher's `collect_scan_with_cancel` must return a
// `CancellableScanOutcome` whose `stopped_early` flag is set only when real
// work was abandoned. A cancelled scan therefore returns *partial or zero*
// violations AND omits the "Scan complete" milestone — it does not just emit
// a cosmetic Cancelled message.

mod common;

use std::sync::atomic::AtomicBool;

use shared_common::taxonomy_path_vo::FilePath;

/// Many-file fixture (100+ linted files across crates/modules/packages) so a
/// full scan is not trivially fast.
fn many_file_fixture() -> std::path::PathBuf {
    let root = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .join("workspaces-bad");
    assert!(root.is_dir(), "fixture missing: {}", root.display());
    root
}

fn seam() -> std::sync::Arc<dispatcher_lint_arwaky::surface_check_action::FilesystemSeam> {
    let c = filesystem::root_filesystem_container::FilesystemContainer::new();
    std::sync::Arc::new(
        dispatcher_lint_arwaky::surface_check_action::FilesystemSeam {
            workspace: c.workspace(),
            parser: c.parser(),
            aggregate: c.orchestrator(),
        },
    )
}

fn scan_opts() -> dispatcher_lint_arwaky::surface_check_action::ScanOptions {
    let fixture = many_file_fixture();
    let root_str = fixture.to_string_lossy().to_string();
    dispatcher_lint_arwaky::surface_check_action::ScanOptions {
        path: Some(FilePath::new(fixture.to_string_lossy().to_string()).unwrap()),
        multi_project_orchestrator: None,
        filter: None,
        member: None,
        filesystem: seam(),
        scan_aggregates: Some(common::build_scan_aggregates(&root_str)),
    }
}

/// Cancel set *before* the call: the dispatcher must stop before running any
/// linter, return zero violations, set `stopped_early`, and emit no
/// "Scan complete" milestone.
#[test]
fn cancel_before_scan_stops_short_with_no_completion_milestone() {
    let cancel = std::sync::Arc::new(AtomicBool::new(true));
    let mut milestones: Vec<String> = Vec::new();
    let outcome = dispatcher_lint_arwaky::surface_check_action::collect_scan_with_cancel(
        scan_opts(),
        &cancel,
        |phase, _done, _total| {
            milestones.push(phase);
        },
    )
    .expect("pre-cancelled scan must not error");

    assert!(
        outcome.stopped_early,
        "a scan aborted before any linter ran must report stopped_early"
    );
    assert_eq!(
        outcome.violations.len(),
        0,
        "no linter phase ran, so no violations can be produced"
    );
    assert!(
        !milestones.iter().any(|m| m == "Scan complete"),
        "a stopped-short scan must never emit the 'Scan complete' milestone; got {milestones:?}"
    );
}

/// No cancel: the dispatcher runs to completion and reports a normal result.
#[test]
fn no_cancel_runs_to_completion() {
    let cancel = std::sync::Arc::new(AtomicBool::new(false));
    let mut milestones: Vec<String> = Vec::new();
    let outcome = dispatcher_lint_arwaky::surface_check_action::collect_scan_with_cancel(
        scan_opts(),
        &cancel,
        |phase, _done, _total| {
            milestones.push(phase);
        },
    )
    .expect("uncancelled scan must not error");

    assert!(
        !outcome.stopped_early,
        "a fully completed scan must not report stopped_early"
    );
    assert!(
        milestones.iter().any(|m| m == "Scan complete"),
        "a completed scan must emit the 'Scan complete' milestone; got {milestones:?}"
    );
}

/// Cancel flipped *mid-scan*: the dispatcher must observe the token before the
/// final milestone, stop short of "Scan complete", and return whatever it had
/// collected (no "Scan complete" for the phases it never reached).
#[test]
fn cancel_flipped_mid_scan_stops_short() {
    let cancel = std::sync::Arc::new(AtomicBool::new(false));
    let flipped = std::sync::Arc::new(AtomicBool::new(false));
    let mut milestones: Vec<String> = Vec::new();
    let outcome = dispatcher_lint_arwaky::surface_check_action::collect_scan_with_cancel(
        scan_opts(),
        &cancel,
        |phase, _done, _total| {
            // Flip the token after the first progress milestone so the
            // dispatcher sees it before it can finish.
            if !flipped.swap(true, std::sync::atomic::Ordering::Relaxed) {
                let _ = phase;
            }
            milestones.push(phase.clone());
            cancel.store(true, std::sync::atomic::Ordering::Relaxed);
        },
    )
    .expect("mid-cancel scan must not error");

    assert!(
        outcome.stopped_early,
        "a scan cancelled mid-flight must report stopped_early"
    );
    assert!(
        !milestones.iter().any(|m| m == "Scan complete"),
        "a scan cancelled mid-flight must not emit the 'Scan complete' milestone; got {milestones:?}"
    );
}
