// Benchmark tests for maintenance — checker and tool executor throughput.
use criterion::{Criterion, criterion_group, criterion_main};

fn bench_stats_collection(c: &mut Criterion) {
    use shared::common::FilePath;
    use shared::maintenance::IMaintenanceCheckerProtocol;

    let mut group = c.benchmark_group("stats_collection");
    group.significance_level(0.05).confidence_level(0.95);
    group.sample_size(10);

    let tmp = tempfile::TempDir::new().unwrap();
    for i in 0..50 {
        std::fs::write(
            tmp.path().join(format!("file_{}.rs", i)),
            "fn main() {}\n",
        )
        .unwrap();
    }
    let fp = FilePath::new(tmp.path().to_string_lossy().to_string()).unwrap();

    let filesystem = filesystem::root_filesystem_container::FilesystemContainer::new()
        .orchestrator();
    let checker = maintenance_lint_arwaky::MaintenanceChecker::new(filesystem);

    group.bench_function("stats_50_files", |b| {
        b.iter(|| {
            std::hint::black_box(checker.stats(&fp));
        });
    });

    group.finish();
}

fn bench_doctor_output(c: &mut Criterion) {
    use shared::maintenance::IMaintenanceCheckerProtocol;

    let mut group = c.benchmark_group("doctor_output");
    group.significance_level(0.05).confidence_level(0.95);
    group.sample_size(10);

    let filesystem = filesystem::root_filesystem_container::FilesystemContainer::new()
        .orchestrator();
    let checker = maintenance_lint_arwaky::MaintenanceChecker::new(filesystem);

    group.bench_function("doctor_check", |b| {
        b.iter(|| {
            std::hint::black_box(checker.doctor());
        });
    });

    group.finish();
}

criterion_group!(benches, bench_stats_collection, bench_doctor_output,);
criterion_main!(benches);
