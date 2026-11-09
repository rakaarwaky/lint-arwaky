// Benchmark tests for dispatcher — full-root scan vs member-targeted scan latency.
// Best practices: significance_level(0.05), sample_size(30+), no --check CLI surface changes.
use criterion::{Criterion, criterion_group, criterion_main};
use dispatcher_lint_arwaky::surface_check_action::{
    FilesystemSeam, ScanAggregates, ScanOptions, collect_scan,
};
use shared_common::taxonomy_path_vo::FilePath;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::Instant;

/// Member-targeted scan latency advisory threshold (mirrors the regression
/// test in `crates/dispatcher/tests/regression_scan_latency.rs`).
///
/// Sourced from `surface_check_action::LATENCY_GUARD_MAX_RATIO` so the bench
/// and the regression test never drift apart.
use dispatcher_lint_arwaky::surface_check_action::LATENCY_GUARD_MAX_RATIO;

/// Resolve a fixture path relative to the workspace root (parent of
/// `crates/dispatcher`).
fn fixture_path(relative: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(|p| p.parent())
        .map(|p| p.join(relative))
        .expect("CARGO_MANIFEST_DIR must be crates/dispatcher")
}

/// Mirror of the test wiring in `crates/dispatcher/tests/mock_common.rs` so the
/// benchmark drives the same in-process aggregate path the CLI uses (W10).
fn build_scan_aggregates(root: &str) -> ScanAggregates {
    let fs_container = filesystem::root_filesystem_container::FilesystemContainer::new();
    let filesystem = fs_container.orchestrator();
    let fs_io = fs_container.io();
    let fs_orchestrator = fs_container.orchestrator();
    let fs_workspace = fs_container.workspace();
    let fs_parser = fs_container.parser();

    let fs_seam = Arc::new(FilesystemSeam {
        workspace: fs_orchestrator.clone(),
        parser: fs_orchestrator.clone(),
        aggregate: filesystem.clone(),
    });

    let config_container = config_system::root_config_system_container::ConfigContainer::new(
        filesystem.clone(),
        fs_io.clone(),
    );
    let config_orchestrator = config_container.orchestrator();

    let quality_container =
        quality_rules::root_quality_rules_container::CodeAnalysisContainer::from_orchestrator(
            &config_orchestrator,
            root,
        );
    let quality = quality_container.code_analysis_linter();

    let sync_config = config_orchestrator
        .execute(shared_config_system::ConfigRequest::load_sync(
            &FilePath::new(root.to_string()).unwrap_or_default(),
        ))
        .into_sync_config();

    let import_container =
        import_rules::root_import_rules_container::ImportContainer::new_with_config(
            sync_config.clone(),
            filesystem.clone(),
            fs_io.clone(),
            fs_workspace.clone(),
            fs_parser.clone(),
        );
    let import = import_container.orchestrator();

    let naming_container = naming_rules::root_naming_rules_container::NamingContainer::new(
        Arc::new(sync_config.clone()),
        Arc::new(shared_common::LayerMapVO::new(sync_config.layers.clone())),
    );
    let naming = naming_container.orchestrator();

    let orphan_container =
        orphan_rules::root_orphan_detector_container::OrphanContainer::from_orchestrator(
            &config_orchestrator,
            root,
            filesystem.clone(),
            fs_workspace.clone(),
        );
    let orphan = orphan_container.analyzer();

    let external_container =
        external_lint::root_external_lint_container::ExternalLintContainer::new(
            filesystem.clone(),
            fs_io.clone(),
            fs_container.tool_resolution().clone(),
            false,
        );
    let external = external_container.aggregate();

    let role_container =
        role_rules::root_role_rules_container::RoleContainer::new_with_config(sync_config);
    let role = role_container.orchestrator();

    let structure =
        structure_rules::root_structure_rules_container::RootStructureRulesContainer::orchestrator(
        );
    let doc = doc_rules::root_doc_rules_container::RootDocRulesContainer::orchestrator();

    ScanAggregates {
        config_file_reader: config_container.file_reader(),
        quality,
        role,
        import,
        naming,
        external,
        orphan,
        config: config_orchestrator,
        structure,
        doc,
        fs_seam,
    }
}

/// Drive `collect_scan` through the in-process aggregate path for a given target.
/// `member` mirrors the CLI `--member` flag: when set, the scan still walks the
/// full root (per PR 2) but narrows the reported output via `retain_scope`.
fn run_scan(
    agg: &ScanAggregates,
    root: &str,
    member: Option<&str>,
) -> Result<Vec<shared_common::ViolationItem>, String> {
    let full = fixture_path(root);
    let opts = ScanOptions {
        path: Some(FilePath::new(full.to_string_lossy().to_string()).expect("valid fixture path")),
        multi_project_orchestrator: None,
        filter: None,
        member: member.map(|m| m.to_string()),
        filesystem: agg.fs_seam.clone(),
        scan_aggregates: Some(agg.clone()),
    };
    collect_scan(opts)
}

/// Scan the whole workspace root (`workspaces-good`) — `scan .`.
fn scan_full_root(agg: &ScanAggregates) -> Result<Vec<shared_common::ViolationItem>, String> {
    run_scan(agg, "workspaces-good", None)
}

/// Scan a single member dir (`workspaces-good/crates`) — `scan crates`.
/// Per PR 2 this still walks the full root; only the report narrows.
fn scan_member_targeted(agg: &ScanAggregates) -> Result<Vec<shared_common::ViolationItem>, String> {
    run_scan(agg, "workspaces-good", Some("crates"))
}

/// Time a single `collect_scan` call with `Instant` for the advisory latency
/// output (excludes criterion warm-up/iteration overhead).
fn time_scan<F>(f: F) -> u128
where
    F: Fn() -> Result<Vec<shared_common::ViolationItem>, String>,
{
    let start = Instant::now();
    let violations = f().expect("scan must succeed");
    let elapsed = start.elapsed().as_nanos();
    eprintln!(
        "scan returned {} violations in {} ms",
        violations.len(),
        elapsed / 1_000_000
    );
    elapsed
}

/// Median of `BENCH_LATENCY_REPS` timings of `f` for stable advisory output.
fn median_ns<F>(f: F) -> u128
where
    F: Fn() -> Result<Vec<shared_common::ViolationItem>, String>,
{
    const BENCH_LATENCY_REPS: usize = 5;
    let mut samples: Vec<u128> = (0..BENCH_LATENCY_REPS).map(|_| time_scan(&f)).collect();
    samples.sort_unstable();
    samples[samples.len() / 2]
}

/// Build aggregates once per bench run (container setup is not part of the
/// measured scan work), then measure both scan shapes on the same fixture.
fn bench_scan_latency(c: &mut Criterion) {
    let root = fixture_path("workspaces-good");
    let agg = build_scan_aggregates(&root.to_string_lossy());

    let mut group = c.benchmark_group("scan_latency");
    group.significance_level(0.05).confidence_level(0.95);
    group.sample_size(30);
    // Warm up via Criterion's warm_up_time rather than manual pre-group calls.
    group.warm_up_time(std::time::Duration::from_secs(1));

    group.bench_function("full_root_scan", |b| {
        b.iter(|| {
            let violations = scan_full_root(&agg).expect("full-root scan must succeed");
            std::hint::black_box(violations);
        });
    });

    group.bench_function("member_targeted_scan", |b| {
        b.iter(|| {
            let violations = scan_member_targeted(&agg).expect("member-targeted scan must succeed");
            std::hint::black_box(violations);
        });
    });

    group.finish();

    // Latency guard (advisory output only): per PR 2 both `scan .` and
    // `scan crates` walk the full root (`scan_root = root.clone()`), so the
    // member-targeted scan latency must be within LATENCY_GUARD_MAX_RATIO of
    // the full-root scan — not lower by design (it is not skipping work).
    // No CI gate enforces the ratio: benches are excluded from nextest and
    // `benchmarks.yml` is nightly/advisory. The regression test
    // `crates/dispatcher/tests/regression_scan_latency.rs` prints the same
    // ratio and can enforce it opt-in via LATENCY_GUARD_ENFORCE=1.
    let root_ns = median_ns(|| scan_full_root(&agg));
    let member_ns = median_ns(|| scan_member_targeted(&agg));
    let ratio = member_ns as f64 / root_ns as f64;
    eprintln!(
        "latency guard (advisory): root={} ns member={} ns ratio={:.3} (threshold {})",
        root_ns, member_ns, ratio, LATENCY_GUARD_MAX_RATIO
    );
}

criterion_group!(benches, bench_scan_latency);
criterion_main!(benches);
