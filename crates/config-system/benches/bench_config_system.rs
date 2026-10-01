// Benchmark tests for config-system — parsing, loading, and workspace discovery.
// Best practices: significance_level(0.05), sample_size(30+), throughput measurement
use config_system_lint_arwaky::capabilities_workspace_detector::WorkspaceDetector;
use config_system_lint_arwaky::root_config_system_container::ConfigContainer;
use criterion::{BenchmarkId, Criterion, Throughput, criterion_group, criterion_main};
use shared_common::taxonomy_adapter_name_vo::AdapterName;
use shared_common::taxonomy_path_vo::FilePath;
use shared_config_system::ConfigRequest;
use shared_config_system::contract_config_protocol::IWorkspaceMembersProtocol;
use shared_config_system::taxonomy_config_system_vo::{AdapterEntry, AdapterStatus, ProjectConfig};
use shared_config_system::utility_config_parser::parse_config_yaml;
use shared_config_system::utility_config_parser::validate_thresholds;
use std::fs;
use tempfile::TempDir;

fn bench_parse_config_yaml(c: &mut Criterion) {
    let yaml_small = "architecture:\n  enabled: true\n  rules: []\n";
    let yaml_large = format!("architecture:\n  enabled: true\n  rules:\n{}\n", (0..100).map(|i| format!("    - name: rule_{}\n      description: Rule {}\n      rule_type: AES{}\n      enabled: true\n      scope: capabilities\n", i, i, 300 + i)).collect::<String>());
    let mut group = c.benchmark_group("parse_config_yaml");
    group.significance_level(0.05).confidence_level(0.95);

    group.bench_with_input(
        BenchmarkId::new("small", "10_lines"),
        &yaml_small,
        |b, yaml| b.iter(|| std::hint::black_box(parse_config_yaml(yaml))),
    );
    group.bench_with_input(
        BenchmarkId::new("large", "100_rules"),
        &yaml_large,
        |b, yaml| b.iter(|| std::hint::black_box(parse_config_yaml(yaml))),
    );
    group.finish();
}

fn bench_workspace_detect(c: &mut Criterion) {
    let tmp = TempDir::new().unwrap();
    fs::write(tmp.path().join("Cargo.toml"), "[package]\nname=\"x\"\n").unwrap();
    let fp = FilePath::new(tmp.path().to_string_lossy().to_string()).unwrap();
    let fs_container = filesystem::root_filesystem_container::FilesystemContainer::new();
    let fs_arc: std::sync::Arc<
        dyn shared_filesystem::contract_filesystem_protocol::IFileSystemIOProtocol,
    > = fs_container.io();
    let detector = WorkspaceDetector::new(fs_arc);
    let mut group = c.benchmark_group("workspace_detect");
    group.sample_size(30);
    group.bench_with_input(
        BenchmarkId::new("detect", "rust_project"),
        &fp,
        |b, path| {
            b.iter(|| std::hint::black_box(detector.detect(path)));
        },
    );
    group.finish();
}

fn bench_validate_thresholds(c: &mut Criterion) {
    let mut group = c.benchmark_group("validate_thresholds");
    group.sample_size(30);

    for n in [10, 50, 200] {
        group.throughput(Throughput::Elements(n as u64));
        group.bench_with_input(BenchmarkId::new("validate", n), &n, |b, val| {
            let config = ProjectConfig {
                adapters: (0..*val)
                    .map(|i| {
                        AdapterEntry::new(
                            AdapterName::raw(format!("adapter_{}", i)),
                            AdapterStatus::Enabled,
                            1.0,
                        )
                    })
                    .collect(),
                ..Default::default()
            };
            b.iter(|| std::hint::black_box(validate_thresholds(&config)))
        });
    }
    group.finish();
}

fn bench_load_config_sync(c: &mut Criterion) {
    let tmp = TempDir::new().unwrap();
    fs::write(
        tmp.path().join("lint_arwaky.config.yaml"),
        "architecture:\n  enabled: true\n  rules: []\n",
    )
    .unwrap();
    fs::write(tmp.path().join("Cargo.toml"), "[package]\nname=\"x\"\n").unwrap();
    let root = FilePath::new(tmp.path().to_string_lossy().to_string()).unwrap();
    let mut group = c.benchmark_group("load_config_sync");
    group.sample_size(30);
    group.bench_with_input(
        BenchmarkId::new("load", "rust_project"),
        &root,
        |b, path| {
            let fs_container = filesystem::root_filesystem_container::FilesystemContainer::new();
            let fs: std::sync::Arc<
                dyn shared_filesystem::contract_filesystem_aggregate::IFilesystemAggregate,
            > = fs_container.orchestrator();
            let orch = ConfigContainer::new(fs, fs_container.io()).orchestrator();
            b.iter(|| {
                std::hint::black_box(
                    orch.execute(ConfigRequest::load_sync(path))
                        .into_sync_config(),
                )
            })
        },
    );
    group.finish();
}

criterion_group!(
    benches,
    bench_parse_config_yaml,
    bench_workspace_detect,
    bench_validate_thresholds,
    bench_load_config_sync
);
criterion_main!(benches);
