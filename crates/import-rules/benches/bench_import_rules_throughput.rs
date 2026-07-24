// PURPOSE: Benchmark tests for import-rules performance.
// Requirement: Check 1000 files in < 2 seconds (FRD non-functional requirement).
// Best practices: significance_level(0.05), sample_size(30+), throughput, scaling

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use import_rules_lint_arwaky::capabilities_dummy_import_checker::DummyImportChecker;
use import_rules_lint_arwaky::capabilities_import_unused_checker::UnusedImportRuleChecker;
use import_rules_lint_arwaky::root_import_rules_container::ImportContainer;
use shared::common::taxonomy_path_vo::FilePath;
use shared::common::taxonomy_source_vo::ContentString;
use shared::config_system::taxonomy_config_vo::ArchitectureConfig;
use shared::import_rules::contract_dummy_import_protocol::IDummyImportCheckerProtocol;
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

// ─── Benchmark: Unused Import Check — Throughput ──────────

fn bench_unused_import_check(c: &mut Criterion) {
    let checker = UnusedImportRuleChecker::new();
    let mut group = c.benchmark_group("unused_import_check");
    group.significance_level(0.05).sample_size(30);

    for file_count in [10, 100, 1000] {
        let contents: Vec<String> = (0..file_count).map(generate_clean_content).collect();
        group.throughput(Throughput::Elements(file_count as u64));

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
                    black_box(&violations);
                });
            },
        );
    }
    group.finish();
}

// ─── Benchmark: Dummy Import Check — Throughput ───────────

fn bench_dummy_import_check(c: &mut Criterion) {
    let checker = DummyImportChecker::new();
    let layer_map = LayerMapVO::new(HashMap::new());
    let root = FilePath::new(".").unwrap();
    let mut group = c.benchmark_group("dummy_import_check");
    group.significance_level(0.05).sample_size(30);

    for file_count in [10, 100, 1000] {
        let contents: Vec<(FilePath, ContentString)> = (0..file_count)
            .map(|i| {
                (
                    FilePath::new(format!("capabilities_file_{}.rs", i)).unwrap(),
                    ContentString::new(generate_violation_content(i)),
                )
            })
            .collect();
        group.throughput(Throughput::Elements(file_count as u64));

        group.bench_with_input(
            BenchmarkId::new("violation_files", file_count),
            &contents,
            |b, files| {
                b.iter(|| {
                    let mut violations = Vec::new();
                    for (file, content) in files {
                        checker.check_all_dummy(file, content, &mut violations, &root, &layer_map);
                    }
                    black_box(&violations);
                });
            },
        );
    }
    group.finish();
}

// ─── Benchmark: Dummy Import — check_all vs individual ────

fn bench_dummy_all_vs_individual(c: &mut Criterion) {
    let checker = DummyImportChecker::new();
    let layer_map = LayerMapVO::new(HashMap::new());
    let root = FilePath::new(".").unwrap();
    let mut group = c.benchmark_group("dummy_all_vs_individual");
    group.significance_level(0.05).sample_size(30);

    let file_count = 100;
    let contents: Vec<(FilePath, ContentString)> = (0..file_count)
        .map(|i| {
            (
                FilePath::new(format!("capabilities_file_{}.rs", i)).unwrap(),
                ContentString::new(generate_violation_content(i)),
            )
        })
        .collect();
    group.throughput(Throughput::Elements(file_count as u64));

    group.bench_function("check_all_dummy", |b| {
        b.iter(|| {
            let mut violations = Vec::new();
            for (file, content) in &contents {
                checker.check_all_dummy(file, content, &mut violations, &root, &layer_map);
            }
            black_box(&violations);
        });
    });

    group.bench_function("individual_checks", |b| {
        b.iter(|| {
            let mut violations = Vec::new();
            for (file, content) in &contents {
                checker.check_dummy_imports(file, content, &mut violations, &root, &layer_map);
                checker.check_dummy_functions(file, content, &mut violations, &root, &layer_map);
                checker.check_dummy_impls(file, content, &mut violations, &root, &layer_map);
                checker.check_taxonomy_intent(file, content, &mut violations, &root, &layer_map);
                checker.check_surface_logic(file, content, &mut violations, &root, &layer_map);
            }
            black_box(&violations);
        });
    });

    group.finish();
}

// ─── Benchmark: Full Orchestrator — Scaling ───────────────

fn bench_full_orchestrator(c: &mut Criterion) {
    let mut group = c.benchmark_group("full_orchestrator");
    group.significance_level(0.05).sample_size(10);

    let rt = tokio::runtime::Runtime::new().unwrap();

    for file_count in [100, 500, 1000] {
        let dir = tempfile::tempdir().unwrap();
        for i in 0..file_count {
            let path = dir.path().join(format!("taxonomy_bench_{}_vo.rs", i));
            std::fs::write(&path, generate_clean_content(i)).unwrap();
        }

        let config = ArchitectureConfig::default();
        let container = ImportContainer::new_with_config(config);
        let orch = container.orchestrator();
        let target = FilePath::new(dir.path().to_string_lossy().to_string()).unwrap();
        group.throughput(Throughput::Elements(file_count as u64));

        group.bench_with_input(
            BenchmarkId::new("audit", file_count),
            &file_count,
            |b, _| b.iter(|| black_box(rt.block_on(async { orch.run_audit(&target).await }))),
        );
    }
    group.finish();
}

// ─── Benchmark: Cycle Analyzer — Scaling ──────────────────

fn bench_cycle_analyzer(c: &mut Criterion) {
    use import_rules_lint_arwaky::capabilities_cycle_import_analyzer::DependencyCycleAnalyzer;
    use shared::import_rules::contract_cycle_import_protocol::ICycleImportProtocol;

    let analyzer = DependencyCycleAnalyzer::new();
    let config = ArchitectureConfig::default();
    let layer_map = LayerMapVO::new(HashMap::new());
    let root = FilePath::new(".").unwrap();
    let mut group = c.benchmark_group("cycle_analyzer");
    group.significance_level(0.05).sample_size(30);

    for file_count in [10, 100, 500] {
        let dir = tempfile::tempdir().unwrap();
        for i in 0..file_count {
            let path = dir.path().join(format!("taxonomy_cycle_{}_vo.rs", i));
            std::fs::write(&path, generate_clean_content(i)).unwrap();
        }
        let files: Vec<FilePath> = (0..file_count)
            .map(|i| {
                FilePath::new(
                    dir.path()
                        .join(format!("taxonomy_cycle_{}_vo.rs", i))
                        .to_string_lossy()
                        .to_string(),
                )
                .unwrap()
            })
            .collect();
        group.throughput(Throughput::Elements(file_count as u64));

        group.bench_with_input(BenchmarkId::new("scan", file_count), &files, |b, fls| {
            b.iter(|| black_box(analyzer.scan(&config, &layer_map, fls, &root)))
        });
    }
    group.finish();
}

criterion_group!(
    benches,
    bench_unused_import_check,
    bench_dummy_import_check,
    bench_dummy_all_vs_individual,
    bench_full_orchestrator,
    bench_cycle_analyzer,
);
criterion_main!(benches);

// ─── Performance Assertion Test ────────────────────────────
// This is a regular #[test] (not a criterion benchmark) that validates
// the FRD non-functional requirement: 1000 files in < 2 seconds.

#[test]
fn perf_1000_files_under_2_seconds() {
    let dir = tempfile::tempdir().unwrap();
    for i in 0..1000 {
        let path = dir.path().join(format!("taxonomy_perf_{}_vo.rs", i));
        std::fs::write(&path, generate_clean_content(i)).unwrap();
    }

    let config = ArchitectureConfig::default();
    let container = ImportContainer::new_with_config(config);
    let orch = container.orchestrator();
    let target = FilePath::new(dir.path().to_string_lossy().to_string()).unwrap();

    let rt = tokio::runtime::Runtime::new().unwrap();
    let start = std::time::Instant::now();
    let results = rt.block_on(async { orch.run_audit(&target).await.unwrap() });
    let elapsed = start.elapsed();

    assert!(
        elapsed.as_secs_f64() < 2.0,
        "FRD NFR: 1000 files must be checked in < 2 seconds, took {:.2}s",
        elapsed.as_secs_f64()
    );

    let _ = results;
}
