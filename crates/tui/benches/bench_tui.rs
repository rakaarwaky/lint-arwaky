// Benchmark tests for tui — state construction and formatting throughput.
use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};

fn bench_file_entry_construction(c: &mut Criterion) {
    let mut group = c.benchmark_group("file_entry_construction");
    group.significance_level(0.05).confidence_level(0.95);

    group.bench_function("create_file_entry", |b| {
        b.iter(|| {
            let entry = shared::tui::FileEntry {
                name: "module.rs".to_string(),
                full_path: "/tmp/project/src/module.rs".to_string(),
                is_dir: false,
                layer: shared::tui::AesLayer::Capabilities,
                violation_count: 3,
                extension: "rs".to_string(),
                size_bytes: 1024,
            };
            std::hint::black_box(entry);
        });
    });

    group.finish();
}

fn bench_file_entry_batch(c: &mut Criterion) {
    let mut group = c.benchmark_group("file_entry_batch");
    group.significance_level(0.05).confidence_level(0.95);

    for n in [10, 50, 200] {
        group.bench_with_input(BenchmarkId::new("create_batch", n), &n, |b, &n| {
            b.iter(|| {
                let entries: Vec<shared::tui::FileEntry> = (0..n)
                    .map(|i| shared::tui::FileEntry {
                        name: format!("file_{}.rs", i),
                        full_path: format!("/tmp/project/src/file_{}.rs", i),
                        is_dir: false,
                        layer: shared::tui::AesLayer::Capabilities,
                        violation_count: i % 5,
                        extension: "rs".to_string(),
                        size_bytes: 1024,
                    })
                    .collect();
                std::hint::black_box(entries);
            });
        });
    }

    group.finish();
}

criterion_group!(
    benches,
    bench_file_entry_construction,
    bench_file_entry_batch,
);
criterion_main!(benches);
