use criterion::{criterion_group, criterion_main, Criterion};

fn bench_external_lint(c: &mut Criterion) {
    c.bench_function("external_lint_container_creation", |b| {
        b.iter(|| {
            external_lint_lint_arwaky::root_external_lint_container::ExternalLintContainer::new()
        });
    });
}

criterion_group!(benches, bench_external_lint);
criterion_main!(benches);
