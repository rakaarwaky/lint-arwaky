// Benchmark tests for tui — state construction and formatting throughput.
use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};

fn bench_app_state_construction(c: &mut Criterion) {
    let mut group = c.benchmark_group("app_state_construction");
    group.significance_level(0.05).confidence_level(0.95);

    let entries: Vec<shared::tui::FileEntry> = (0..200)
        .map(|i| shared::tui::FileEntry {
            path: shared::common::FilePath::new(format!("src/module_{}.rs", i)).unwrap(),
            language: shared::common::Language::Rust,
            size: shared::common::Count::new(1024),
        })
        .collect();

    for n in [10, 50, 200] {
        group.bench_with_input(
            BenchmarkId::new("create_state", n),
            &entries[..n],
            |b, e| {
                b.iter(|| {
                    let state = shared::tui::AppState {
                        project_root: "/tmp/project".to_string(),
                        current_dir: "/tmp/project/src".to_string(),
                        entries: e.to_vec(),
                        selected_index: 0,
                        ..Default::default()
                    };
                    std::hint::black_box(state);
                });
            },
        );
    }

    group.finish();
}

criterion_group!(benches, bench_app_state_construction,);
criterion_main!(benches);
