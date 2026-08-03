// Benchmark tests for file-watch — change analyzer throughput.
use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};

fn bench_change_analyzer_classify(c: &mut Criterion) {
    let mut group = c.benchmark_group("change_analyzer_classify");
    group.significance_level(0.05).confidence_level(0.95);

    let analyzer = file_watch_lint_arwaky::ChangeAnalyzer::new();

    let entries: Vec<shared::common::FileEntry> = (0..100)
        .map(|i| shared::common::FileEntry {
            path: shared::common::FilePath::new(format!("src/file_{}.rs", i)).unwrap(),
            language: shared::common::Language::Rust,
            size: shared::common::Count::new(1024),
        })
        .collect();

    for n in [10, 50, 100] {
        group.bench_with_input(
            BenchmarkId::new("classify_batch", n),
            &entries[..n],
            |b, e| {
                b.iter(|| {
                    std::hint::black_box(analyzer.classify_changes(e));
                });
            },
        );
    }

    group.finish();
}

criterion_group!(benches, bench_change_analyzer_classify,);
criterion_main!(benches);
