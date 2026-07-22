// PURPOSE: Benchmark — ChangeAnalyzer deduplication and filtering throughput.
// Uses criterion. Register in Cargo.toml with harness = false.

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};

use file_watch_lint_arwaky::capabilities_change_analyzer::ChangeAnalyzer;
use shared::file_watch::contract_change_analyzer_protocol::IChangeAnalyzerProtocol;
use shared::file_watch::taxonomy_watch_event_vo::{WatchEvent, WatchEventKind};

fn generate_events(count: usize, unique_ratio: f64) -> Vec<WatchEvent> {
    let unique_count = ((count as f64) * unique_ratio).max(1.0) as usize;
    (0..count)
        .map(|i| {
            let path = format!("src/file_{}.rs", i % unique_count);
            WatchEvent::new(path, WatchEventKind::Modified)
        })
        .collect()
}

fn bench_analyze_dedup(c: &mut Criterion) {
    let analyzer = ChangeAnalyzer::new();
    let mut group = c.benchmark_group("change_analyzer_analyze");

    for size in [10, 100, 1_000, 10_000] {
        let events = generate_events(size, 0.3); // 30% unique → heavy duplication
        group.bench_with_input(
            BenchmarkId::new("dedup_30pct_unique", size),
            &events,
            |b, data| b.iter(|| analyzer.analyze(data.clone())),
        );
    }
    group.finish();
}

fn bench_filter_lintable(c: &mut Criterion) {
    let analyzer = ChangeAnalyzer::new();
    let mut group = c.benchmark_group("change_analyzer_filter");

    for size in [10, 100, 1_000, 10_000] {
        let events: Vec<WatchEvent> = (0..size)
            .map(|i| {
                let path = if i % 3 == 0 {
                    format!("src/file_{}.rs", i)
                } else {
                    format!("assets/image_{}.png", i)
                };
                WatchEvent::new(path, WatchEventKind::Modified)
            })
            .collect();

        group.bench_with_input(
            BenchmarkId::new("filter_mixed", size),
            &events,
            |b, data| b.iter(|| analyzer.filter_lintable(data.clone())),
        );
    }
    group.finish();
}

fn bench_is_lintable(c: &mut Criterion) {
    let mut group = c.benchmark_group("change_analyzer_is_lintable");

    let paths = vec![
        "src/main.rs",
        "app.py",
        "index.ts",
        "image.png",
        "binary.exe",
        "no_extension",
    ];

    for path in &paths {
        group.bench_with_input(
            BenchmarkId::new("is_lintable", path.to_string()),
            path,
            |b, p| b.iter(|| ChangeAnalyzer::is_lintable(p)),
        );
    }
    group.finish();
}

criterion_group!(
    benches,
    bench_analyze_dedup,
    bench_filter_lintable,
    bench_is_lintable
);
criterion_main!(benches);
