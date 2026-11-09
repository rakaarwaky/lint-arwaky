// Regression tests for the dispatcher scan-latency guard and scan-output
// correctness.
//
// The latency guard asserts that a member-targeted scan (`scan crates`) is no
// slower than the full-root scan (`scan .`). Post PR 2 both shapes walk the
// full workspace root (`scan_root = root.clone()`); only `retain_scope`
// narrows the reported set, so the member-targeted latency must be within
// `LATENCY_GUARD_MAX_RATIO` of the full-root latency — never lower by design.
//
// Timing is measured with `Instant` over `LATENCY_GUARD_REPS` repetitions,
// taking the median of each shape. Because timing tests are flaky on loaded
// CI machines, the timing test below is `#[ignore]`d; its ratio is printed
// as advisory output. The correctness invariants (non-empty violations, the
// member-set == root-set equivalence contract) run unconditionally.
//
// The bench file (`crates/dispatcher/benches/bench_dispatcher.rs`) keeps only
// advisory `eprintln` output for the same measurement.

use dispatcher_lint_arwaky::surface_check_action::{FilesystemSeam, ScanOptions, collect_scan};
use mock_common::build_scan_aggregates;
use shared_common::taxonomy_path_vo::FilePath;

mod mock_common;
use std::collections::HashSet;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::Instant;

/// Member-targeted scan latency must be <= this x the full-root scan latency.
pub(crate) const LATENCY_GUARD_MAX_RATIO: f64 = 1.2;

/// Repetitions per shape when timing; the median of each is compared.
const LATENCY_GUARD_REPS: usize = 5;

/// Resolve a fixture path relative to the workspace root (parent of
/// `crates/dispatcher`).
fn fixture_path(relative: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(|p| p.parent())
        .map(|p| p.join(relative))
        .expect("CARGO_MANIFEST_DIR must be crates/dispatcher")
}

/// Seam bundle for `ScanOptions`: io + workspace + parser + aggregate.
fn seam() -> Arc<FilesystemSeam> {
    let container = filesystem::root_filesystem_container::FilesystemContainer::new();
    let orchestrator = container.orchestrator();
    Arc::new(FilesystemSeam {
        workspace: orchestrator.clone(),
        parser: orchestrator.clone(),
        aggregate: orchestrator,
    })
}

/// Run `collect_scan` for a fixture root, returning the reported violations.
fn run_scan(root: &str, member: Option<&str>) -> Vec<shared_common::ViolationItem> {
    let full = fixture_path(root);
    let opts = ScanOptions {
        path: Some(FilePath::new(full.to_string_lossy().to_string()).expect("valid fixture path")),
        multi_project_orchestrator: None,
        filter: None,
        member: member.map(|m| m.to_string()),
        filesystem: seam(),
        scan_aggregates: Some(build_scan_aggregates(&full.to_string_lossy())),
    };
    collect_scan(opts).expect("scan must succeed")
}

/// Time a single scan call with `Instant`.
fn time_scan<F>(f: F) -> u128
where
    F: Fn() -> Vec<shared_common::ViolationItem>,
{
    let start = Instant::now();
    let _ = f();
    start.elapsed().as_nanos()
}

/// Median of `LATENCY_GUARD_REPS` timings of `f`.
fn median_ns<F>(f: F) -> u128
where
    F: Fn() -> Vec<shared_common::ViolationItem>,
{
    let mut samples: Vec<u128> = (0..LATENCY_GUARD_REPS).map(|_| time_scan(&f)).collect();
    samples.sort_unstable();
    samples[samples.len() / 2]
}

/// Correctness invariants, run unconditionally in CI.
///
/// Post PR 2 both `scan .` (`member: None`) and `scan crates` (`member:
/// Some("crates")`) walk the full root. `member=crates` resolves to
/// `ScanScope::TopLevel` whose `filter_prefix()` is `None`, so no crates
/// prefix filtering is applied — the member-targeted reported set equals the
/// root set. That is the equivalence contract asserted here (file paths, not
/// just codes).
#[test]
fn scan_output_correctness_equivalence() {
    let root = "workspaces-bad"; // violation-loaded fixture (workspaces-good is clean)
    let root_violations = run_scan(root, None);
    let member_violations = run_scan(root, Some("crates"));

    assert!(
        !root_violations.is_empty(),
        "full-root scan of {root} must report violations"
    );
    assert!(
        !member_violations.is_empty(),
        "member-targeted scan of {root}/crates must report violations"
    );

    let root_set: HashSet<String> = root_violations
        .iter()
        .map(|v| v.file.value.clone())
        .collect();
    let member_set: HashSet<String> = member_violations
        .iter()
        .map(|v| v.file.value.clone())
        .collect();

    assert_eq!(
        member_set, root_set,
        "member-targeted reported set must equal the full-root reported set \
         (both shapes walk the full root; TopLevel applies no prefix narrowing)"
    );
}

/// Latency guard: member-targeted scan must be within `LATENCY_GUARD_MAX_RATIO`
/// of the full-root scan. Timing on `workspaces-good` (~1,000 clean files).
///
/// `#[ignore]`d because single-shot/median `Instant` timing is flaky on loaded
// CI machines. The ratio is printed as advisory output; the correctness
// invariants above run unconditionally.
#[ignore = "timing tests are flaky on loaded CI machines; the latency ratio is advisory output. Correctness invariants run unconditionally in scan_output_correctness_equivalence."]
#[test]
fn latency_guard_member_within_ratio_of_root() {
    let root = "workspaces-good";
    let agg_root = || run_scan(root, None);
    let agg_member = || run_scan(root, Some("crates"));

    let root_ns = median_ns(agg_root);
    let member_ns = median_ns(agg_member);
    let ratio = member_ns as f64 / root_ns as f64;

    eprintln!(
        "latency guard: root={root_ns} ns member={member_ns} ns ratio={ratio:.3} \
         (threshold {LATENCY_GUARD_MAX_RATIO})"
    );

    assert!(
        ratio <= LATENCY_GUARD_MAX_RATIO,
        "member-targeted scan latency ({member_ns} ns, ratio {ratio:.3}x) must be <= \
         {LATENCY_GUARD_MAX_RATIO}x full-root scan latency ({root_ns} ns)",
    );
}
