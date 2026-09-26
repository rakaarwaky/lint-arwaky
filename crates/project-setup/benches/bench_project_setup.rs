use criterion::{Criterion, criterion_group, criterion_main};
use project_setup_lint_arwaky::root_project_setup_container::SetupContainer;
use shared::common::taxonomy_path_vo::DirectoryPath;
use shared::project_setup::SetupRequest;

fn bench_container_creation(c: &mut Criterion) {
    c.bench_function("setup_container_creation", |b| {
        b.iter(|| {
            let fs_container = filesystem::root_filesystem_container::FilesystemContainer::new();
            SetupContainer::new(fs_container.io())
        });
    });
}

fn bench_generate_mcp_config(c: &mut Criterion) {
    let fs_container = filesystem::root_filesystem_container::FilesystemContainer::new();
    let fs = fs_container.io();
    let container = SetupContainer::new(fs);
    let agg = container.aggregate();
    c.bench_function("setup_generate_mcp_config", |b| {
        b.iter(|| {
            agg.execute(SetupRequest::mcp_config_claude())
                .into_mcp_config()
        });
    });
}

fn bench_detect_language(c: &mut Criterion) {
    let fs_container = filesystem::root_filesystem_container::FilesystemContainer::new();
    let fs = fs_container.io();
    let container = SetupContainer::new(fs);
    let agg = container.aggregate();
    c.bench_function("setup_detect_language", |b| {
        b.iter(|| agg.execute(SetupRequest::detect_language()).into_language());
    });
}

fn bench_generate_env(c: &mut Criterion) {
    let fs_container = filesystem::root_filesystem_container::FilesystemContainer::new();
    let fs = fs_container.io();
    let container = SetupContainer::new(fs);
    let agg = container.aggregate();
    let home = DirectoryPath::new("/tmp/bench").unwrap();
    c.bench_function("setup_generate_env", |b| {
        b.iter(|| agg.execute(SetupRequest::generate_env(&home)).into_env());
    });
}

fn bench_get_config_template(c: &mut Criterion) {
    let fs_container = filesystem::root_filesystem_container::FilesystemContainer::new();
    let fs = fs_container.io();
    let container = SetupContainer::new(fs);
    let agg = container.aggregate();
    c.bench_function("setup_get_config_template", |b| {
        b.iter(|| {
            agg.execute(SetupRequest::get_config_template("rust"))
                .into_template()
        });
    });
}

criterion_group!(
    benches,
    bench_container_creation,
    bench_generate_mcp_config,
    bench_detect_language,
    bench_generate_env,
    bench_get_config_template,
);
criterion_main!(benches);
