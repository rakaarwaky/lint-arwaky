// PURPOSE: Benchmark tests for git-hooks diff operations.
// Measures: DiffChecker throughput, GitCommandAdapter latency.
// Layer: Capabilities performance
// Speed: s–min (release gate / nightly)

use criterion::{criterion_group, criterion_main, Criterion};
use git_hooks_lint_arwaky::capabilities_diff_checker::DiffChecker;
use shared::common::taxonomy_git_vo::GitBranchName;
use shared::common::taxonomy_path_vo::FilePath;
use shared::git_hooks::contract_diff_protocol::IDiffProtocol;

fn bench_get_default_branch(c: &mut Criterion) {
    let checker = DiffChecker::new();
    let path = FilePath::new(".").unwrap_or_default();
    let rt = tokio::runtime::Runtime::new().unwrap();

    c.bench_function("get_default_branch", |b| {
        b.iter(|| {
            rt.block_on(checker.get_default_branch(&path));
        });
    });
}

fn bench_get_changed_files(c: &mut Criterion) {
    let checker = DiffChecker::new();
    let path = FilePath::new(".").unwrap_or_default();
    let branch = GitBranchName::new("main");
    let rt = tokio::runtime::Runtime::new().unwrap();

    c.bench_function("get_changed_files", |b| {
        b.iter(|| {
            rt.block_on(checker.get_changed_files(&path, &branch));
        });
    });
}

fn bench_get_diff(c: &mut Criterion) {
    let checker = DiffChecker::new();
    let path = FilePath::new(".").unwrap_or_default();
    let rt = tokio::runtime::Runtime::new().unwrap();

    c.bench_function("get_diff", |b| {
        b.iter(|| {
            rt.block_on(checker.get_diff(&path));
        });
    });
}

fn bench_run_git_diff_check(c: &mut Criterion) {
    let checker = DiffChecker::new();
    let path = FilePath::new(".").unwrap_or_default();
    let rt = tokio::runtime::Runtime::new().unwrap();

    c.bench_function("run_git_diff_check", |b| {
        b.iter(|| {
            rt.block_on(checker.run_git_diff_check(&path));
        });
    });
}

fn bench_container_construction(c: &mut Criterion) {
    let mut group = c.benchmark_group("container");

    group.bench_function("new_default", |b| {
        b.iter(|| {
            let _container =
                git_hooks_lint_arwaky::root_git_hooks_container::GitContainer::new_default();
        });
    });

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
