// Benchmark tests for naming-rules — convention checker, suffix checker, orchestrator.
use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};
use naming_rules_lint_arwaky::agent_naming_orchestrator::{
    NamingOrchestrator, NamingOrchestratorDeps,
};
use naming_rules_lint_arwaky::capabilities_naming_convention_checker::NamingConventionChecker;
use naming_rules_lint_arwaky::capabilities_suffix_prefix_checker::SuffixPrefixChecker;
use shared::common::PatternList;
use shared::common::SuffixPolicyVO;
use shared::common::taxonomy_definition_vo::{LayerDefinition, LayerMapVO};
use shared::common::taxonomy_layer_vo::LayerNameVO;
use shared::common::taxonomy_lint_result_vo::LintResultList;
use shared::common::taxonomy_path_vo::FilePath;
use shared::common::taxonomy_paths_vo::FilePathList;
use shared::config_system::taxonomy_config_vo::ArchitectureConfig;
use shared::filesystem::taxonomy_filesystem_vo::{FileEntry, Language};
use shared::naming_rules::INamingConventionChecker;
use shared::naming_rules::INamingRunnerAggregate;
use shared::naming_rules::ISuffixPrefixChecker;
use shared::naming_rules::SUFFIX_POLICY_STRICT;
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;

fn make_layer_map() -> LayerMapVO {
    let mut def = LayerDefinition::default();
    def.naming.suffix_policy = SuffixPolicyVO::new(SUFFIX_POLICY_STRICT.to_string());
    def.naming.allowed_suffix =
        PatternList::new(vec!["checker".to_string(), "adapter".to_string()]);
    def.naming.forbidden_suffix = PatternList::new(vec!["vo".to_string()]);

    let mut layers = HashMap::new();
    layers.insert(LayerNameVO::new("capabilities"), def);
    LayerMapVO::new(layers)
}

fn generate_file_paths(n: usize, name_pattern: &str) -> FilePathList {
    let paths: Vec<FilePath> = (0..n)
        .map(|i| {
            let name = format!("src/{}_{}.rs", name_pattern, i);
            FilePath::new(name).unwrap()
        })
        .collect();
    FilePathList::new(paths)
}

fn generate_file_entries(n: usize, name_pattern: &str) -> Vec<FileEntry> {
    (0..n)
        .map(|i| {
            let name = format!("{}_{}.rs", name_pattern, i);
            let content = format!("fn dummy_{}() {{}}", i);
            FileEntry {
                path: PathBuf::from(format!("src/{}", name)),
                extension: "rs".to_string(),
                language: Language::Rust,
                size: content.len() as u64,
                content,
                parse_ok: false,
                parse_metadata: None,
            }
        })
        .collect()
}

fn bench_naming_convention_checker(c: &mut Criterion) {
    let mut group = c.benchmark_group("naming_convention_checker");
    group.significance_level(0.05).confidence_level(0.95);
    group.sample_size(30);

    let checker = NamingConventionChecker::new();
    let config = ArchitectureConfig::default();
    let layer_map = make_layer_map();
    let root = FilePath::new(".".to_string()).unwrap();

    for (n, label) in [(10, "10_files"), (50, "50_files"), (100, "100_files")] {
        group.bench_with_input(BenchmarkId::new("valid_names", label), &n, |b, &n| {
            let files = generate_file_paths(n, "capabilities_user_checker");
            b.iter(|| {
                let mut results = LintResultList::new(Vec::new());
                checker.check_file_naming(&config, &layer_map, &files, &root, &mut results);
                std::hint::black_box(&results);
            });
        });

        group.bench_with_input(BenchmarkId::new("mixed_names", label), &n, |b, &n| {
            let mut paths: Vec<FilePath> = Vec::new();
            for i in 0..n {
                let name = if i % 3 == 0 {
                    format!("src/capabilities_Bad_{}.rs", i)
                } else if i % 3 == 1 {
                    format!("src/capabilities_user_checker_{}.rs", i)
                } else {
                    format!("src/capabilities-user-hyphen-{}.rs", i)
                };
                paths.push(FilePath::new(name).unwrap());
            }
            let files = FilePathList::new(paths);
            b.iter(|| {
                let mut results = LintResultList::new(Vec::new());
                checker.check_file_naming(&config, &layer_map, &files, &root, &mut results);
                std::hint::black_box(&results);
            });
        });
    }

    group.finish();
}

fn bench_suffix_prefix_checker(c: &mut Criterion) {
    let mut group = c.benchmark_group("suffix_prefix_checker");
    group.significance_level(0.05).confidence_level(0.95);
    group.sample_size(30);

    let checker = SuffixPrefixChecker::new();
    let config = ArchitectureConfig::default();
    let layer_map = make_layer_map();
    let root = FilePath::new(".".to_string()).unwrap();

    for (n, label) in [(10, "10_files"), (50, "50_files"), (100, "100_files")] {
        group.bench_with_input(BenchmarkId::new("valid_suffixes", label), &n, |b, &n| {
            let files = generate_file_paths(n, "capabilities_user_checker");
            b.iter(|| {
                let mut results = LintResultList::new(Vec::new());
                checker.check_domain_suffixes(&config, &layer_map, &files, &root, &mut results);
                std::hint::black_box(&results);
            });
        });

        group.bench_with_input(BenchmarkId::new("mixed_suffixes", label), &n, |b, &n| {
            let mut paths: Vec<FilePath> = Vec::new();
            for i in 0..n {
                let name = if i % 2 == 0 {
                    format!("src/capabilities_user_vo_{}.rs", i) // forbidden
                } else {
                    format!("src/capabilities_user_checker_{}.rs", i) // allowed
                };
                paths.push(FilePath::new(name).unwrap());
            }
            let files = FilePathList::new(paths);
            b.iter(|| {
                let mut results = LintResultList::new(Vec::new());
                checker.check_domain_suffixes(&config, &layer_map, &files, &root, &mut results);
                std::hint::black_box(&results);
            });
        });
    }

    group.finish();
}

fn bench_orchestrator_full_audit(c: &mut Criterion) {
    let mut group = c.benchmark_group("orchestrator_full_audit");
    group.significance_level(0.05).confidence_level(0.95);
    group.sample_size(30);

    let config = Arc::new(ArchitectureConfig::default());
    let layer_map = Arc::new(make_layer_map());
    let deps = NamingOrchestratorDeps {
        naming_convention_checker: Arc::new(NamingConventionChecker::new()),
        suffix_prefix_checker: Arc::new(SuffixPrefixChecker::new()),
        config: config.clone(),
        layer_map: layer_map.clone(),
    };
    let orch = NamingOrchestrator::new(deps);

    for (n, label) in [(10, "10_files"), (50, "50_files"), (100, "100_files")] {
        group.bench_with_input(BenchmarkId::new("mixed_entries", label), &n, |b, &n| {
            let entries = generate_file_entries(n, "capabilities_user");
            b.iter(|| {
                std::hint::black_box(orch.run_audit_with_entries(&entries));
            });
        });
    }

    group.finish();
}

criterion_group!(
    benches,
    bench_naming_convention_checker,
    bench_suffix_prefix_checker,
    bench_orchestrator_full_audit,
);
criterion_main!(benches);
