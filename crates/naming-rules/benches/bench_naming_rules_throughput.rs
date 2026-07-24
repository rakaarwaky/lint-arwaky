// PURPOSE: Benchmark — naming checks throughput
// NFR: Check 1000 files in < 1 second
// Uses criterion — never hand-rolled timing
// Best practices: significance_level(0.05), sample_size(30+)

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use naming_rules_lint_arwaky::capabilities_naming_convention_checker::NamingConventionChecker;
use naming_rules_lint_arwaky::capabilities_suffix_prefix_checker::SuffixPrefixChecker;
use shared::cli_commands::taxonomy_result_vo::LintResultList;
use shared::common::taxonomy_definition_vo::{LayerDefinition, LayerMapVO};
use shared::common::taxonomy_layer_vo::LayerNameVO;
use shared::common::taxonomy_path_vo::FilePath;
use shared::common::taxonomy_paths_vo::FilePathList;
use shared::config_system::taxonomy_config_vo::ArchitectureConfig;
use shared::naming_rules::contract_naming_checker_protocol::{
    INamingConventionChecker, ISuffixPrefixChecker,
};
use std::collections::HashMap;

fn build_layer_map() -> LayerMapVO {
    let mut map = HashMap::new();
    map.insert(LayerNameVO::new("capabilities"), LayerDefinition::default());
    map.insert(LayerNameVO::new("taxonomy"), LayerDefinition::default());
    map.insert(LayerNameVO::new("utility"), LayerDefinition::default());
    map.insert(LayerNameVO::new("agent"), LayerDefinition::default());
    LayerMapVO::new(map)
}

fn generate_files(count: usize) -> FilePathList {
    let prefixes = ["capabilities", "taxonomy", "utility", "agent"];
    let suffixes = ["checker", "vo", "parser", "orchestrator"];
    let files: Vec<FilePath> = (0..count)
        .map(|i| {
            let prefix = prefixes[i % prefixes.len()];
            let suffix = suffixes[i % suffixes.len()];
            let name = format!("{}_item_{}_{}.rs", prefix, i, suffix);
            FilePath::new(name).unwrap()
        })
        .collect();
    FilePathList::new(files)
}

fn bench_naming_convention_checker(c: &mut Criterion) {
    let rt = tokio::runtime::Runtime::new().unwrap();
    let checker = NamingConventionChecker::new();
    let config = ArchitectureConfig::default();
    let layer_map = build_layer_map();
    let root = FilePath::new("/project".to_string()).unwrap();

    let mut group = c.benchmark_group("naming_convention_checker");
    group.significance_level(0.05).confidence_level(0.95);

    for size in [100, 500, 1000, 5000] {
        let files = generate_files(size);
        group.bench_with_input(
            BenchmarkId::new("check_file_naming", size),
            &files,
            |b, files| {
                b.iter(|| {
                    rt.block_on(async {
                        let mut results = LintResultList::new(Vec::new());
                        checker
                            .check_file_naming(&config, &layer_map, files, &root, &mut results)
                            .await;
                        results
                    })
                });
            },
        );
    }
    group.finish();
}

fn bench_suffix_prefix_checker(c: &mut Criterion) {
    let rt = tokio::runtime::Runtime::new().unwrap();
    let checker = SuffixPrefixChecker::new();
    let config = ArchitectureConfig::default();
    let layer_map = build_layer_map();
    let root = FilePath::new("/project".to_string()).unwrap();

    let mut group = c.benchmark_group("suffix_prefix_checker");
    group.significance_level(0.05).confidence_level(0.95);

    for size in [100, 500, 1000, 5000] {
        let files = generate_files(size);
        group.bench_with_input(
            BenchmarkId::new("check_domain_suffixes", size),
            &files,
            |b, files| {
                b.iter(|| {
                    rt.block_on(async {
                        let mut results = LintResultList::new(Vec::new());
                        checker
                            .check_domain_suffixes(&config, &layer_map, files, &root, &mut results)
                            .await;
                        results
                    })
                });
            },
        );
    }
    group.finish();
}

criterion_group!(
    benches,
    bench_naming_convention_checker,
    bench_suffix_prefix_checker
);
criterion_main!(benches);
