// PURPOSE: Benchmark tests for maintenance operations — stats file walking and dependency parsing.
// Layer: Benchmark (criterion, runs at release gate / nightly).

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use maintenance_lint_arwaky::agent_maintenance_orchestrator::MaintenanceCommandsOrchestrator;
use maintenance_lint_arwaky::capabilities_maintenance_checker::MaintenanceChecker;
use shared::common::taxonomy_path_vo::FilePath;
use shared::project_setup::contract_maintenance_aggregate::MaintenanceCommandsAggregate;
use shared::project_setup::contract_maintenance_protocol::IMaintenanceCheckerProtocol;

fn setup_project_with_n_files(n: usize) -> String {
    let dir = format!("/tmp/bench_maintenance_{}", n);
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    for i in 0..n {
        std::fs::write(format!("{}/file_{}.py", dir, i), "pass\n").unwrap();
    }
    dir
}

fn bench_stats_file_walking(c: &mut Criterion) {
    let mut group = c.benchmark_group("stats_file_walking");

    for size in [10, 100, 500] {
        let dir = setup_project_with_n_files(size);
        let orch = MaintenanceCommandsOrchestrator::new();
        let path = FilePath::new(dir.clone()).unwrap();

        group.bench_with_input(BenchmarkId::new("stats", size), &path, |b, p| {
            b.to_async(tokio::runtime::Runtime::new().unwrap())
                .iter(|| orch.stats(p));
        });

        let _ = std::fs::remove_dir_all(&dir);
    }
    group.finish();
}

fn bench_dependency_parsing(c: &mut Criterion) {
    let dir = "/tmp/bench_dep_parsing";
    let _ = std::fs::remove_dir_all(dir);
    std::fs::create_dir_all(dir).unwrap();

    // Generate a Cargo.lock with many packages
    let mut lock_content = String::new();
    for i in 0..200 {
        lock_content.push_str(&format!(
            "[[package]]\nname = \"dep_{}\"\nversion = \"1.0.{}\"\n\n",
            i, i
        ));
    }
    std::fs::write(format!("{}/Cargo.lock", dir), &lock_content).unwrap();
    std::fs::write(
        format!("{}/Cargo.toml", dir),
        "[package]\nname = \"bench\"\nversion = \"0.1.0\"\n\n[dependencies]\ndep_0 = \"1.0\"\n",
    )
    .unwrap();

    let checker = MaintenanceChecker::new();
    let path = FilePath::new(dir.to_string()).unwrap();

    let mut group = c.benchmark_group("dependency_parsing");
    group.bench_function("cargo_lock_200_packages", |b| {
        b.to_async(tokio::runtime::Runtime::new().unwrap())
            .iter(|| checker.run_dependency_report(&path));
    });
    group.finish();

    let _ = std::fs::remove_dir_all(dir);
}

criterion_group!(benches, bench_stats_file_walking, bench_dependency_parsing);
criterion_main!(benches);
