// PURPOSE: Benchmark — graph building and reachability tracing performance.
// Layer: Benchmark
// Speed: s–min

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use orphan_detector_lint_arwaky::capabilities_orphan_graph_resolver::OrphanGraphResolver;
use orphan_detector_lint_arwaky::root_orphan_detector_container::OrphanContainer;
use shared::orphan_detector::contract_orphan_graph_resolver_protocol::IOrphanGraphResolverProtocol;
use shared::orphan_detector::taxonomy_orphan_contract_vo::OrphanFileListVO;

fn generate_file_list(count: usize) -> Vec<String> {
    (0..count)
        .map(|i| format!("/tmp/bench/src/capabilities_module_{:04}_analyzer.rs", i))
        .collect()
}

fn bench_build_graph_context(c: &mut Criterion) {
    let resolver = OrphanGraphResolver::new();
    let mut group = c.benchmark_group("build_graph_context");

    for size in [10, 100, 500] {
        let files = generate_file_list(size);
        let file_vo = vec![OrphanFileListVO::new(files)];
        group.bench_with_input(BenchmarkId::new("files", size), &file_vo, |b, data| {
            b.iter(|| resolver.build_graph_context(data, "/tmp/bench"));
        });
    }
    group.finish();
}

fn bench_identify_entry_points(c: &mut Criterion) {
    let resolver = OrphanGraphResolver::new();
    let mut group = c.benchmark_group("identify_entry_points");

    for size in [10, 100, 500] {
        let mut files = generate_file_list(size);
        files.push("/tmp/bench/src/main.rs".to_string());
        files.push("/tmp/bench/src/root_app_container.rs".to_string());
        let file_vo = vec![OrphanFileListVO::new(files)];
        group.bench_with_input(BenchmarkId::new("files", size), &file_vo, |b, data| {
            b.iter(|| resolver.identify_entry_points(data, &[]));
        });
    }
    group.finish();
}

fn bench_check_orphans(c: &mut Criterion) {
    let container = OrphanContainer::new();
    let analyzer = container.analyzer();
    let mut group = c.benchmark_group("check_orphans");

    for size in [10, 50] {
        let files = generate_file_list(size);
        group.bench_with_input(BenchmarkId::new("files", size), &files, |b, data| {
            b.iter(|| analyzer.check_orphans(data, "/tmp/bench"));
        });
    }
    group.finish();
}

criterion_group!(
    benches,
    bench_build_graph_context,
    bench_identify_entry_points,
    bench_check_orphans,
);
criterion_main!(benches);
