// Benchmarks for git-hooks — hook script generation.
use criterion::{Criterion, criterion_group, criterion_main};
use git_hooks_lint_arwaky::capabilities_hook_installer::HookInstaller;
use git_hooks_lint_arwaky::capabilities_hook_uninstaller::HookUninstaller;
use shared::common::FilePath;
use shared::git_hooks::{IHookInstallProtocol, IHookUninstallProtocol};
use tempfile::TempDir;

fn bench_hook_install(c: &mut Criterion) {
    c.bench_function("hook_install_uninstall_cycle", |b| {
        b.iter_batched(
            || {
                let tmp = TempDir::new().unwrap();
                std::fs::create_dir_all(tmp.path().join(".git/hooks")).unwrap();
                let io = filesystem::root_filesystem_container::FilesystemContainer::new().io();
                let fp = FilePath::new(tmp.path().to_string_lossy().to_string()).unwrap();
                let installer = HookInstaller::new(fp.clone(), io.clone());
                let uninstaller =
                    HookUninstaller::new(tmp.path().to_string_lossy().to_string(), io);
                (tmp, installer, uninstaller)
            },
            |(_, installer, uninstaller)| {
                let exe = FilePath::new("lint-arwaky-cli".to_string()).unwrap();
                installer.install_pre_commit(&exe).unwrap();
                uninstaller.uninstall_pre_commit().unwrap();
            },
            criterion::BatchSize::SmallInput,
        );
    });
}

criterion_group!(benches, bench_hook_install);
criterion_main!(benches);
