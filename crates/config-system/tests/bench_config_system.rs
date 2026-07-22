// Benchmark tests for config-system — parsing, loading, and workspace discovery.
use config_system_lint_arwaky::capabilities_rules_validator::ConfigRulesValidator;
use config_system_lint_arwaky::capabilities_workspace_detector::WorkspaceDetector;
use config_system_lint_arwaky::root_config_system_container::ConfigContainer;
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use shared::common::taxonomy_adapter_name_vo::AdapterName;
use shared::common::taxonomy_path_vo::FilePath;
use shared::config_system::contract_config_orchestrator_aggregate::IConfigOrchestratorAggregate;
use shared::config_system::contract_validator_protocol::IConfigValidatorProtocol;
use shared::config_system::contract_workspace_detector_protocol::IWorkspaceDetectorProtocol;
use shared::config_system::taxonomy_setting_vo::{AdapterEntry, AdapterStatus, ProjectConfig};
use shared::config_system::utility_config_parser::parse_config_yaml;
use std::fs;
use tempfile::TempDir;

fn bench_parse_config_yaml(c: &mut Criterion) {
    let yaml_small = "architecture:\n  enabled: true\n  rules: []\n";
    let yaml_large = format!("architecture:\n  enabled: true\n  rules:\n{}\n", (0..100).map(|i| format!("    - name: rule_{}\n      description: Rule {}\n      rule_type: AES{}\n      enabled: true\n      scope: capabilities\n", i, i, 300 + i)).collect::<String>());
    let mut group = c.benchmark_group("parse_config_yaml");
    group.bench_with_input(
        BenchmarkId::new("small", "10_lines"),
        &yaml_small,
        |b, yaml| b.iter(|| parse_config_yaml(yaml)),
    );
    group.bench_with_input(
        BenchmarkId::new("large", "100_rules"),
        &yaml_large,
        |b, yaml| b.iter(|| parse_config_yaml(yaml)),
    );
    group.finish();
}

fn bench_workspace_detect(c: &mut Criterion) {
    let tmp = TempDir::new().unwrap();
    fs::write(tmp.path().join("Cargo.toml"), "[package]\nname=\"x\"\n").unwrap();
    let fp = FilePath::new(tmp.path().to_string_lossy().to_string()).unwrap();
    let detector = WorkspaceDetector::new();
    c.bench_function("workspace_detect_rust", |b| b.iter(|| detector.detect(&fp)));
}

fn bench_validate_thresholds(c: &mut Criterion) {
    let validator = ConfigRulesValidator::new();
    let mut config = ProjectConfig::default();
    config.adapters = (0..50)
        .map(|i| {
            AdapterEntry::new(
                AdapterName::raw(format!("adapter_{}", i)),
                AdapterStatus::Enabled,
                1.0,
            )
        })
        .collect();
    c.bench_function("validate_thresholds_50_adapters", |b| {
        b.iter(|| validator.validate_thresholds(&config))
    });
}

fn bench_load_config_sync(c: &mut Criterion) {
    let tmp = TempDir::new().unwrap();
    fs::write(
        tmp.path().join("lint_arwaky.config.rust.yaml"),
        "architecture:\n  enabled: true\n  rules: []\n",
    )
    .unwrap();
    fs::write(tmp.path().join("Cargo.toml"), "[package]\nname=\"x\"\n").unwrap();
    let orch = ConfigContainer::new().orchestrator();
    let root_str = tmp.path().to_str().unwrap().to_string();
    c.bench_function("load_config_sync", |b| {
        b.iter(|| orch.load_config_sync(&root_str))
    });
}

criterion_group!(
    benches,
    bench_parse_config_yaml,
    bench_workspace_detect,
    bench_validate_thresholds,
    bench_load_config_sync
);
criterion_main!(benches);
