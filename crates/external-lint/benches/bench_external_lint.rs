use criterion::{Criterion, criterion_group, criterion_main};

fn bench_external_lint(c: &mut Criterion) {
    c.bench_function("external_lint_container_creation", |b| {
        b.iter_with_large_drop(|| {
            let c = filesystem::root_filesystem_container::FilesystemContainer::new();
            external_lint_lint_arwaky::root_external_lint_container::ExternalLintContainer::new(
                c.orchestrator(),
                c.io(),
                c.tool_resolution(),
            )
        });
    });
}

criterion_group!(benches, bench_external_lint);
criterion_main!(benches);
