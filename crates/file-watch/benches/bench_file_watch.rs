// Benchmark tests for file-watch — change filter throughput.
use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};
use shared_file_watch::contract_watch_protocol::IChangeFilterProtocol;
use shared_file_watch::{WatchEvent, WatchEventKind};

fn bench_change_filter_filter_events(c: &mut Criterion) {
    let mut group = c.benchmark_group("change_filter_filter_events");
    group.significance_level(0.05).confidence_level(0.95);

    let filter = file_watch_lint_arwaky::ChangeFilter::new();

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
                std::hint::black_box(filter.filter_events(e.to_vec()));
            });
        });
    }

    group.finish();
}

criterion_group!(benches, bench_change_filter_filter_events);
criterion_main!(benches);
