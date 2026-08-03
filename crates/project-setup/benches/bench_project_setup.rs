use criterion::{criterion_group, criterion_main, Criterion};
use project_setup_lint_arwaky::root_project_setup_container::SetupContainer;
use shared::common::taxonomy_path_vo::DirectoryPath;
use shared::common::taxonomy_protocol_vo::TransportProtocol;
use shared::project_setup::{ISetupManagementProtocol, SetupManagementAggregate};
use std::sync::Arc;

fn bench_container_creation(c: &mut Criterion) {
    c.bench_function("setup_container_creation", |b| {
        b.iter(|| {
            let fs =
                filesystem::root_filesystem_container::FilesystemContainer::new().orchestrator();
            SetupContainer::new(fs)
        });
    });
}

fn bench_generate_mcp_config(c: &mut Criterion) {
    let fs = filesystem::root_filesystem_container::FilesystemContainer::new().orchestrator();
    let container = SetupContainer::new(fs);
    let agg = container.aggregate();
    c.bench_function("setup_generate_mcp_config", |b| {
        b.iter(|| agg.mcp_config_claude(&TransportProtocol::Stdio));
    });
}

fn bench_detect_language(c: &mut Criterion) {
    let fs = filesystem::root_filesystem_container::FilesystemContainer::new().orchestrator();
    let container = SetupContainer::new(fs);
    let agg = container.aggregate();
    c.bench_function("setup_detect_language", |b| {
        b.iter(|| agg.detect_language());
    });
}

fn bench_generate_env(c: &mut Criterion) {
    let fs = filesystem::root_filesystem_container::FilesystemContainer::new().orchestrator();
    let container = SetupContainer::new(fs);
    let agg = container.aggregate();
    let home = DirectoryPath::new("/tmp/bench").unwrap();
    c.bench_function("setup_generate_env", |b| {
        b.iter(|| agg.generate_env(&TransportProtocol::Stdio, &home));
    });
}

fn bench_get_config_template(c: &mut Criterion) {
    let fs = filesystem::root_filesystem_container::FilesystemContainer::new().orchestrator();
    let container = SetupContainer::new(fs);
    let agg = container.aggregate();
    c.bench_function("setup_get_config_template", |b| {
        b.iter(|| agg.get_config_template("rust"));
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
