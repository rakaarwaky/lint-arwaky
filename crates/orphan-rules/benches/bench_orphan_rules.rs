use criterion::{Criterion, criterion_group, criterion_main};

fn bench_orphan_detection(c: &mut Criterion) {
    c.bench_function("orphan_container_creation", |b| {
        b.iter(|| {
            let fs_container = filesystem::root_filesystem_container::FilesystemContainer::new();
            orphan_rules_lint_arwaky::root_orphan_detector_container::OrphanContainer::new(
                fs_container.orchestrator(),
                fs_container.workspace(),
            )
        });
    });
}

criterion_group!(benches, bench_orphan_detection);
criterion_main!(benches);
