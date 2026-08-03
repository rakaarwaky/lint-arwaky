// Benchmarks for git-hooks — diff data comparison and hook script generation.
use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};
use git_hooks_lint_arwaky::capabilities_hook_adapter::GitHookAdapter;
use git_hooks_lint_arwaky::capabilities_hook_manager::HookManager;
use shared::common::FilePath;
use shared::git_hooks::{IHookManagerProtocol, IHookProtocol};
use std::sync::Arc;
use tempfile::TempDir;

fn make_hook_manager(tmp: &TempDir) -> HookManager {
    let filesystem = filesystem::root_filesystem_container::FilesystemContainer::new()
        .orchestrator();
    let fp = FilePath::new(tmp.path().to_string_lossy().to_string()).unwrap();
    let adapter: Arc<dyn IHookManagerProtocol> =
        Arc::new(GitHookAdapter::new(fp, filesystem.clone()));
    HookManager::new(adapter, filesystem)
}

fn bench_diff_data_comparison(c: &mut Criterion) {
    let mut group = c.benchmark_group("diff_data_comparison");

    for size in [64, 1024, 65536] {
        let tmp = TempDir::new().unwrap();
        let p1 = tmp.path().join("a.txt");
        let p2 = tmp.path().join("b.txt");

        // Create files with known content
        let content_a = "x".repeat(size);
        let mut content_b = "x".repeat(size);
        // Change last 10% of bytes
        let change_start = (size * 9) / 10;
        for i in change_start..size {
            content_b.as_bytes_mut()[i] = b'y';
        }
        std::fs::write(&p1, &content_a).unwrap();
        std::fs::write(&p2, &content_b).unwrap();

        let mgr = make_hook_manager(&tmp);
        let path1 = p1.to_str().unwrap().to_string();
        let path2 = p2.to_str().unwrap().to_string();

        group.bench_with_input(
            BenchmarkId::new("partial_diff", size),
            &size,
            |b, _| {
                b.iter(|| {
                    mgr.get_diff_data(&path1, &path2);
                });
            },
        );
    }
    group.finish();
}

fn bench_hook_install(c: &mut Criterion) {
    c.bench_function("hook_install_uninstall_cycle", |b| {
        b.iter_batched(
            || {
                let tmp = TempDir::new().unwrap();
                std::fs::create_dir_all(tmp.path().join(".git/hooks")).unwrap();
                let filesystem =
                    filesystem::root_filesystem_container::FilesystemContainer::new().orchestrator();
                let fp = FilePath::new(tmp.path().to_string_lossy().to_string()).unwrap();
                let adapter = GitHookAdapter::new(fp, filesystem);
                (tmp, adapter)
            },
            |(tmp, adapter)| {
                let exe = FilePath::new("lint-arwaky-cli".to_string()).unwrap();
                adapter.install_pre_commit(&exe).unwrap();
                adapter.uninstall_pre_commit().unwrap();
                drop(tmp);
            },
            criterion::BatchSize::SmallInput,
        );
    });
}

criterion_group!(benches, bench_diff_data_comparison, bench_hook_install);
criterion_main!(benches);
