// PURPOSE: Benchmark — naming rules throughput, scaling, and parallel execution.
// Layer: Benchmark
// NFR: Check 1000 files in < 1 second

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
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

// ─── Generators ───────────────────────────────────────────

fn build_layer_map() -> LayerMapVO {
    let mut map = HashMap::new();
    map.insert(LayerNameVO::new("capabilities"), LayerDefinition::default());
    map.insert(LayerNameVO::new("taxonomy"), LayerDefinition::default());
    map.insert(LayerNameVO::new("utility"), LayerDefinition::default());
    map.insert(LayerNameVO::new("agent"), LayerDefinition::default());
    map.insert(LayerNameVO::new("contract"), LayerDefinition::default());
    map.insert(LayerNameVO::new("surfaces"), LayerDefinition::default());
    LayerMapVO::new(map)
}

fn generate_files(count: usize) -> FilePathList {
    let prefixes = [
        "capabilities",
        "taxonomy",
        "utility",
        "agent",
        "contract",
        "surfaces",
    ];
    let suffixes = [
        "checker",
        "vo",
        "parser",
        "orchestrator",
        "analyzer",
        "adapter",
    ];
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

fn generate_files_with_invalid_names(count: usize) -> FilePathList {
    let mut files = generate_files(count);
    // Add 10% invalid names (uppercase, wrong separators)
    let invalid: Vec<FilePath> = (0..count / 10)
        .map(|i| {
            let name = format!("Capabilities_Item_Invalid{}.rs", i);
            FilePath::new(name).unwrap()
        })
        .collect();
    files.values.extend(invalid);
    files
}

// ─── 1. Naming Convention Checker — Scaling ──────────────

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
        group.throughput(Throughput::Elements(size as u64));
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
                        black_box(results)
                    })
                });
            },
        );
    }
    group.finish();
}

// ─── 2. Suffix Prefix Checker — Scaling ──────────────────

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
        group.throughput(Throughput::Elements(size as u64));
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
                        black_box(results)
                    })
                });
            },
        );
    }
    group.finish();
}

// ─── 3. Sequential vs Parallel Checkers ──────────────────

fn bench_sequential_vs_parallel(c: &mut Criterion) {
    let rt = tokio::runtime::Runtime::new().unwrap();
    let naming_checker = NamingConventionChecker::new();
    let suffix_checker = SuffixPrefixChecker::new();
    let config = ArchitectureConfig::default();
    let layer_map = build_layer_map();
    let root = FilePath::new("/project".to_string()).unwrap();

    let mut group = c.benchmark_group("seq_vs_par_checkers");
    group.significance_level(0.05).confidence_level(0.95);

    for size in [100, 500, 1000, 5000] {
        let files = generate_files(size);
        group.throughput(Throughput::Elements(size as u64));

        // Sequential (original)
        group.bench_with_input(BenchmarkId::new("sequential", size), &files, |b, files| {
            b.iter(|| {
                rt.block_on(async {
                    let mut naming_results = LintResultList::new(Vec::new());
                    naming_checker
                        .check_file_naming(&config, &layer_map, files, &root, &mut naming_results)
                        .await;

                    let mut suffix_results = LintResultList::new(Vec::new());
                    suffix_checker
                        .check_domain_suffixes(
                            &config,
                            &layer_map,
                            files,
                            &root,
                            &mut suffix_results,
                        )
                        .await;

                    naming_results.values.extend(suffix_results.values);
                    black_box(naming_results)
                })
            });
        });

        // Parallel (tokio::join!)
        group.bench_with_input(BenchmarkId::new("parallel", size), &files, |b, files| {
            b.iter(|| {
                rt.block_on(async {
                    let mut naming_results = LintResultList::new(Vec::new());
                    let mut suffix_results = LintResultList::new(Vec::new());

                    let ((), ()) = tokio::join!(
                        naming_checker.check_file_naming(
                            &config,
                            &layer_map,
                            files,
                            &root,
                            &mut naming_results
                        ),
                        suffix_checker.check_domain_suffixes(
                            &config,
                            &layer_map,
                            files,
                            &root,
                            &mut suffix_results
                        ),
                    );

                    naming_results.values.extend(suffix_results.values);
                    black_box(naming_results)
                })
            });
        });
    }
    group.finish();
}

// ─── 4. Throughput — Files Per Second ────────────────────

fn bench_throughput_files_per_second(c: &mut Criterion) {
    let rt = tokio::runtime::Runtime::new().unwrap();
    let naming_checker = NamingConventionChecker::new();
    let suffix_checker = SuffixPrefixChecker::new();
    let config = ArchitectureConfig::default();
    let layer_map = build_layer_map();
    let root = FilePath::new("/project".to_string()).unwrap();

    let mut group = c.benchmark_group("throughput");
    group.significance_level(0.05).confidence_level(0.95);
    group.sample_size(50);

    for size in [100, 500, 1000, 5000] {
        let files = generate_files(size);
        group.throughput(Throughput::Elements(size as u64));
        group.bench_with_input(
            BenchmarkId::new("files_per_second", size),
            &files,
            |b, files| {
                b.iter(|| {
                    rt.block_on(async {
                        let mut naming_results = LintResultList::new(Vec::new());
                        let mut suffix_results = LintResultList::new(Vec::new());

                        let ((), ()) = tokio::join!(
                            naming_checker.check_file_naming(
                                &config,
                                &layer_map,
                                files,
                                &root,
                                &mut naming_results
                            ),
                            suffix_checker.check_domain_suffixes(
                                &config,
                                &layer_map,
                                files,
                                &root,
                                &mut suffix_results
                            ),
                        );

                        naming_results.values.extend(suffix_results.values);
                        black_box(naming_results)
                    })
                });
            },
        );
    }
    group.finish();
}

// ─── 5. Invalid Names — Overhead Measurement ─────────────

fn bench_invalid_names_overhead(c: &mut Criterion) {
    let rt = tokio::runtime::Runtime::new().unwrap();
    let naming_checker = NamingConventionChecker::new();
    let suffix_checker = SuffixPrefixChecker::new();
    let config = ArchitectureConfig::default();
    let layer_map = build_layer_map();
    let root = FilePath::new("/project".to_string()).unwrap();

    let mut group = c.benchmark_group("invalid_names_overhead");
    group.significance_level(0.05).confidence_level(0.95);

    for size in [100, 500, 1000] {
        let files = generate_files_with_invalid_names(size);
        group.throughput(Throughput::Elements(files.values.len() as u64));
        group.bench_with_input(
            BenchmarkId::new("with_invalid", size),
            &files,
            |b, files| {
                b.iter(|| {
                    rt.block_on(async {
                        let mut naming_results = LintResultList::new(Vec::new());
                        let mut suffix_results = LintResultList::new(Vec::new());

                        let ((), ()) = tokio::join!(
                            naming_checker.check_file_naming(
                                &config,
                                &layer_map,
                                files,
                                &root,
                                &mut naming_results
                            ),
                            suffix_checker.check_domain_suffixes(
                                &config,
                                &layer_map,
                                files,
                                &root,
                                &mut suffix_results
                            ),
                        );

                        naming_results.values.extend(suffix_results.values);
                        black_box(naming_results)
                    })
                });
            },
        );
    }
    group.finish();
}

// ─── Main ────────────────────────────────────────────────

criterion_group!(
    benches,
    bench_naming_convention_checker,
    bench_suffix_prefix_checker,
    bench_sequential_vs_parallel,
    bench_throughput_files_per_second,
    bench_invalid_names_overhead,
);
criterion_main!(benches);
