// Regression test — full-root scan detects cross-member violations under a
// subfolder target, but narrows the *reported* output to that subfolder
// (pr-3 of plan 4ae3d58a).
//
// PR 2 made `collect_scan*` scan the full workspace root
// (`scan_root = root.clone()`) and narrow the *reported* output via
// `retain_scope(violations, &scope.filter_prefix())`. This file guards that
// behavior: a subfolder target (`crates/member_a`) scans the whole member dir
// (so cross-file import evidence from `member_b` is seen) but reports only
// violations under `member_a`.
//
// Fixtures:
//   workspaces-bad/crates/member_a/capabilities_foo.rs — capabilities layer
//     importing `agent` from member_b (AES201 evidence).
//   workspaces-bad/crates/member_b/agent_bar.rs — agent layer, clean.
//
// Drive the in-process path (same as production), mirroring
// `regression_scan_modes.rs`. `path` is the workspace root (`workspaces-bad`),
// scanned in full; `member` narrows the *reported* output to that subfolder.

mod mock_common;
use std::collections::HashSet;

use dispatcher_lint_arwaky::surface_check_action::{ScanAggregates, ScanOptions, collect_scan};
use shared_common::ViolationItem;
use shared_common::taxonomy_path_vo::FilePath;

// ── Test helpers ──────────────────────────────────────────────────────────

/// Resolve a fixture path relative to the workspace root (parent of
/// `crates/dispatcher`).
fn fixture_path(relative: &str) -> std::path::PathBuf {
    std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(|p| p.parent())
        .map(|p| p.join(relative))
        .expect("CARGO_MANIFEST_DIR must be crates/dispatcher")
}

/// Drive `collect_scan` through the in-process aggregate path with a subfolder
/// target (`member`) under `root`. `root` is the workspace root scanned in
/// full; `member` narrows the *reported* output to that subfolder.
fn in_process_subfolder_scan(
    agg: &ScanAggregates,
    root: &str,
    member: &str,
) -> Result<Vec<ViolationItem>, String> {
    let full = fixture_path(root);
    let opts = ScanOptions {
        path: Some(FilePath::new(full.to_string_lossy().to_string()).expect("valid fixture path")),
        multi_project_orchestrator: None,
        filter: None,
        member: Some(member.to_string()),
        filesystem: agg.fs_seam.clone(),
        scan_aggregates: Some(agg.clone()),
    };
    collect_scan(opts)
}

/// Assert the PR-2 scope contract: every reported violation is under
/// `kept` and none leak under `dropped`. `kept` must be a subfolder of the
/// scanned root (`crates/<x>`); `dropped` is its sibling.
fn assert_scope_kept_and_dropped(violations: &[ViolationItem], kept: &str, dropped: &str) {
    for v in violations {
        assert!(
            v.file.value.contains(kept),
            "reported violation outside {kept}: {:?}",
            v.file.value
        );
    }
    let leaked: HashSet<String> = violations
        .iter()
        .filter(|v| v.file.value.contains(dropped))
        .map(|v| v.file.value.clone())
        .collect();
    assert!(
        leaked.is_empty(),
        "expected no violations under {dropped}, got {leaked:?}"
    );
}

// ── Tests ─────────────────────────────────────────────────────────────────

/// A subfolder target reports violations under `member_a` and reports nothing
/// under `member_b`. Asserts the scope contract from PR 2: full-root scan
/// sees cross-member imports, but `retain_scope` narrows reported output to
/// the subfolder path prefix.
#[test]
fn subfolder_scan_reports_only_member_a() {
    let root = fixture_path("workspaces-bad");
    let root_str = root.to_string_lossy().to_string();
    let agg = mock_common::build_scan_aggregates(&root_str);
    let violations = in_process_subfolder_scan(&agg, "workspaces-bad", "crates/member_a")
        .expect("subfolder scan of member_a must succeed");

    // The subfolder target must report something (cross-member evidence seen
    // during the full-root scan is surfaced under member_a).
    assert!(
        !violations.is_empty(),
        "subfolder scan of member_a must report violations, got none"
    );

    assert_scope_kept_and_dropped(&violations, "crates/member_a", "crates/member_b");
}

/// The same scan applied to `member_b` reports only `member_b` paths — the
/// scope contract holds symmetrically for both members. `member_b` carries no
/// local violations (its files are clean), so a non-emptiness assert is
/// omitted by design: the guard here is the negative direction — no leak into
/// `member_a` — and global (root-level) findings still report with paths
/// that contain neither member dir.
#[test]
fn subfolder_scan_reports_only_member_b() {
    let root = fixture_path("workspaces-bad");
    let root_str = root.to_string_lossy().to_string();
    let agg = mock_common::build_scan_aggregates(&root_str);
    let violations = in_process_subfolder_scan(&agg, "workspaces-bad", "crates/member_b")
        .expect("subfolder scan of member_b must succeed");

    assert_scope_kept_and_dropped(&violations, "crates/member_b", "crates/member_a");
}
