use criterion::{criterion_group, criterion_main, Criterion};

fn bench_shared_common(c: &mut Criterion) {
    c.bench_function("shared_path_normalize", |b| {
        b.iter(|| {
            let path = shared::common::taxonomy_path_vo::FilePath::new(
                "/home/user/../../project/src/main.rs".to_string(),
            )
            .unwrap();
            std::hint::black_box(&path);
        });
    });
}

criterion_group!(benches, bench_shared_common);
criterion_main!(benches);
