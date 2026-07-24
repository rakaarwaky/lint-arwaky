// PURPOSE: Benchmark tests for git-hooks diff operations.
// Measures: DiffChecker throughput, GitCommandAdapter latency.
// Layer: Capabilities performance
// Speed: s–min (release gate / nightly)
// Best practices: significance_level(0.05), sample_size(30+), reuse runtime across iterations

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use git_hooks_lint_arwaky::capabilities_diff_checker::DiffChecker;
use shared::common::taxonomy_git_vo::GitBranchName;
use shared::common::taxonomy_path_vo::FilePath;
use shared::git_hooks::contract_diff_protocol::IDiffProtocol;

fn bench_get_default_branch(c: &mut Criterion) {
    let checker = DiffChecker::new();
    let path = FilePath::new(".").unwrap_or_default();
    let rt = tokio::runtime::Runtime::new().unwrap();
    let mut group = c.benchmark_group("get_default_branch");
    group.sample_size(30);

    group.bench_function("check", |b| {
        b.iter(|| black_box(rt.block_on(checker.get_default_branch(&path))))
    });
}

fn bench_get_changed_files(c: &mut Criterion) {
    let checker = DiffChecker::new();
    let path = FilePath::new(".").unwrap_or_default();
    let branch = GitBranchName::new("main");
    let rt = tokio::runtime::Runtime::new().unwrap();
    let mut group = c.benchmark_group("get_changed_files");
    group.sample_size(30);

    group.bench_function("check", |b| {
        b.iter(|| black_box(rt.block_on(checker.get_changed_files(&path, &branch))))
    });
}

fn bench_get_diff(c: &mut Criterion) {
    let checker = DiffChecker::new();
    let path = FilePath::new(".").unwrap_or_default();
    let rt = tokio::runtime::Runtime::new().unwrap();
    let mut group = c.benchmark_group("get_diff");
    group.sample_size(30);

    group.bench_function("check", |b| {
        b.iter(|| black_box(rt.block_on(checker.get_diff(&path))))
    });
}

fn bench_run_git_diff_check(c: &mut Criterion) {
    let checker = DiffChecker::new();
    let path = FilePath::new(".").unwrap_or_default();
    let rt = tokio::runtime::Runtime::new().unwrap();
    let mut group = c.benchmark_group("run_git_diff_check");
    group.sample_size(30);

    group.bench_function("check", |b| {
        b.iter(|| black_box(rt.block_on(checker.run_git_diff_check(&path))))
    });
}

fn bench_container_construction(c: &mut Criterion) {
    let mut group = c.benchmark_group("container");
    group.sample_size(30);

    for n in [1, 10, 100] {
        group.bench_with_input(BenchmarkId::new("instances", n), &n, |b, val| {
            let count = *val;
            b.iter(|| {
                for _ in 0..count {
                    black_box(
                        git_hooks_lint_arwaky::root_git_hooks_container::GitContainer::new_default(
                        ),
                    );
                }
            });
        });
    }
    group.finish();
}

criterion_group!(
    benches,
    bench_get_default_branch,
    bench_get_changed_files,
    bench_get_diff,
    bench_run_git_diff_check,
    bench_container_construction,
);
criterion_main!(benches);
