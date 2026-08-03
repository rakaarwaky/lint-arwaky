// Benchmark tests for file-watch — change analyzer throughput.
use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};
use shared::file_watch::IChangeAnalyzerProtocol;
use shared::file_watch::{WatchEvent, WatchEventKind};

fn bench_change_analyzer_filter_lintable(c: &mut Criterion) {
    let mut group = c.benchmark_group("change_analyzer_filter_lintable");
    group.significance_level(0.05).confidence_level(0.95);

    let analyzer = file_watch_lint_arwaky::ChangeAnalyzer::new();

    let events: Vec<WatchEvent> = (0..100)
        .map(|i| WatchEvent {
            path: format!("src/file_{}.rs", i),
            kind: WatchEventKind::Modified,
            timestamp_ms: i as u64,
        })
        .collect();

    for n in [10, 50, 100] {
        group.bench_with_input(BenchmarkId::new("filter_batch", n), &events[..n], |b, e| {
            b.iter(|| {
                std::hint::black_box(analyzer.filter_lintable(e.to_vec()));
            });
        });
    }

    group.finish();
}

fn bench_change_analyzer_analyze(c: &mut Criterion) {
    let mut group = c.benchmark_group("change_analyzer_analyze");
    group.significance_level(0.05).confidence_level(0.95);

    let analyzer = file_watch_lint_arwaky::ChangeAnalyzer::new();

    let events: Vec<WatchEvent> = (0..100)
        .map(|i| WatchEvent {
            path: format!("src/file_{}.rs", i),
            kind: WatchEventKind::Modified,
            timestamp_ms: i as u64,
        })
        .collect();

    for n in [10, 50, 100] {
        group.bench_with_input(BenchmarkId::new("dedup_batch", n), &events[..n], |b, e| {
            b.iter(|| {
                std::hint::black_box(analyzer.analyze(e.to_vec()));
            });
        });
    }

    group.finish();
}

criterion_group!(
    benches,
    bench_change_analyzer_filter_lintable,
    bench_change_analyzer_analyze,
);
criterion_main!(benches);
