// PURPOSE: Benchmark tests for import-rules performance.
// Requirement: Check 1000 files in < 2 seconds (FRD non-functional requirement).

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use import_rules_lint_arwaky::capabilities_dummy_import_checker::DummyImportChecker;
use import_rules_lint_arwaky::capabilities_import_unused_checker::UnusedImportRuleChecker;
use import_rules_lint_arwaky::root_import_rules_container::ImportContainer;
use shared::common::taxonomy_path_vo::FilePath;
use shared::common::taxonomy_source_vo::ContentString;
use shared::config_system::taxonomy_config_vo::ArchitectureConfig;
use shared::import_rules::contract_dummy_import_protocol::IDummyImportCheckerProtocol;
use shared::import_rules::contract_import_runner_aggregate::IImportRunnerAggregate;
use shared::import_rules::contract_unused_import_protocol::IUnusedImportProtocol;
use shared::taxonomy_definition_vo::LayerMapVO;
use std::collections::HashMap;

fn generate_clean_content(id: usize) -> String {
    format!(
        r#"use std::collections::HashMap;

pub struct Struct{} {{
    pub data: HashMap<String, i32>,
}}

impl Struct{} {{
    pub fn new() -> Self {{
        Self {{ data: HashMap::new() }}
    }}
}}
"#,
        id, id
    )
}

fn generate_violation_content(id: usize) -> String {
    format!(
        r#"use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::BTreeMap;

fn _use_mandatory_imports() {{
    let _ = HashMap::new();
    let _ = HashSet::new();
    let _ = BTreeMap::new();
}}

pub struct Struct{} {{
    pub value: i32,
}}
"#,
        id
    )
}

// ─── Benchmark: Unused Import Check Throughput ────────────

fn bench_unused_import_check(c: &mut Criterion) {
    let checker = UnusedImportRuleChecker::new();
    let mut group = c.benchmark_group("unused_import_check");

    for file_count in [10, 100, 1000] {
        let contents: Vec<String> = (0..file_count).map(|i| generate_clean_content(i)).collect();

        group.bench_with_input(
            BenchmarkId::new("clean_files", file_count),
            &contents,
            |b, files| {
                b.iter(|| {
                    let mut violations = Vec::new();
                    for (i, content) in files.iter().enumerate() {
                        checker.check_unused_imports(
                            &format!("file_{}.rs", i),
                            content,
                            &mut violations,
                        );
                    }
                });
            },
        );
    }
    group.finish();
}

// ─── Benchmark: Dummy Import Check Throughput ─────────────

fn bench_dummy_import_check(c: &mut Criterion) {
    let checker = DummyImportChecker::new();
    let layer_map = LayerMapVO::new(HashMap::new());
    let root = FilePath::new(".").unwrap();
    let mut group = c.benchmark_group("dummy_import_check");

    for file_count in [10, 100, 1000] {
        let contents: Vec<(FilePath, ContentString)> = (0..file_count)
            .map(|i| {
                (
                    FilePath::new(format!("capabilities_file_{}.rs", i)).unwrap(),
                    ContentString::new(generate_violation_content(i)),
                )
            })
            .collect();

        group.bench_with_input(
            BenchmarkId::new("violation_files", file_count),
            &contents,
            |b, files| {
                b.iter(|| {
                    let mut violations = Vec::new();
                    for (file, content) in files {
                        checker.check_dummy_functions(
                            file,
                            content,
                            &mut violations,
                            &root,
                            &layer_map,
                        );
                        checker.check_dummy_imports(
                            file,
                            content,
                            &mut violations,
                            &root,
                            &layer_map,
                        );
                    }
                });
            },
        );
    }
    group.finish();
}

// ─── Benchmark: Full Orchestrator (1000 files) ────────────

fn bench_full_orchestrator(c: &mut Criterion) {
    let mut group = c.benchmark_group("full_orchestrator");
    group.sample_size(10); // Fewer samples for I/O-heavy benchmark

    group.bench_function("1000_clean_files", |b| {
        let dir = tempfile::tempdir().unwrap();
        for i in 0..1000 {
            let path = dir.path().join(format!("taxonomy_bench_{}_vo.rs", i));
            std::fs::write(&path, generate_clean_content(i)).unwrap();
        }

        let config = ArchitectureConfig::default();
        let container = ImportContainer::new_with_config(config);
        let orch = container.orchestrator();
        let target = FilePath::new(dir.path().to_string_lossy().to_string()).unwrap();

        b.iter(|| {
            let rt = tokio::runtime::Runtime::new().unwrap();
            rt.block_on(async { orch.run_audit(&target).await })
        });
    });

    group.finish();
}

criterion_group!(
    benches,
    bench_unused_import_check,
    bench_dummy_import_check,
    bench_full_orchestrator,
);
criterion_main!(benches);
