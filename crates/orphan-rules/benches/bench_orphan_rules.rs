use criterion::{criterion_group, criterion_main, Criterion};
use shared::common::taxonomy_path_vo::FilePath;
use shared::orphan_rules::IOrphanDetectionAggregate;

fn bench_orphan_detection(c: &mut Criterion) {
    c.bench_function("orphan_container_creation", |b| {
        b.iter(|| {
            let fs = filesystem::root_filesystem_container::FilesystemContainer::new().orchestrator();
            orphan_rules_lint_arwaky::root_orphan_detector_container::OrphanDetectorContainer::new(fs)
        });
    });
}

criterion_group!(benches, bench_orphan_detection);
criterion_main!(benches);
