// PURPOSE: Criterion benchmarks for quality-rules crate
use criterion::{Criterion, criterion_group, criterion_main};
use std::path::PathBuf;
use std::sync::Arc;

use quality_rules_lint_arwaky::CodeAnalysisContainer;
use quality_rules_lint_arwaky::capabilities_check_bypass_checker::BypassChecker;
use quality_rules_lint_arwaky::capabilities_code_duplication_analyzer::CodeDuplicationAnalyzer;
use quality_rules_lint_arwaky::capabilities_line_checker::ArchLineChecker;

use shared::common::{Count, LayerDefinition};
use shared::config_system::ArchitectureConfig;
use shared::filesystem::taxonomy_filesystem_vo::FileEntry;
use shared::quality_rules::IBypassCheckerProtocol;
use shared::quality_rules::ICodeMetricAnalyzerProtocol;
use shared::quality_rules::ILineCheckerProtocol;

fn generate_content(lines: usize) -> String {
    (0..lines)
        .map(|i| format!("fn function_{}() {{ let x = {}; return x; }}", i, i))
        .collect::<Vec<_>>()
        .join("\n")
}

fn generate_violating_content() -> String {
    (0..100)
        .map(|i| format!("fn func_{}() {{ let x = foo.unwrap(); }}", i))
        .collect::<Vec<_>>()
        .join("\n")
}

fn bench_line_checker(c: &mut Criterion) {
    let checker = ArchLineChecker::new();
    let def = LayerDefinition {
        code_analysis: shared::quality_rules::CodeAnalysisRuleVO {
            min_lines: Count::new(5),
            max_lines: Count::new(1000),
            ..Default::default()
        },
        ..Default::default()
    };

    let content_500 = generate_content(500);
    let content_1500 = generate_content(1500);

    c.bench_function("line_checker_500_lines", |b| {
        b.iter(|| {
            let mut violations = Vec::new();
            checker.check_line_counts("src/lib.rs", Some(&def), &content_500, &mut violations);
        })
    });

    c.bench_function("line_checker_1500_lines", |b| {
        b.iter(|| {
            let mut violations = Vec::new();
            checker.check_line_counts("src/lib.rs", Some(&def), &content_1500, &mut violations);
        })
    });
}

fn bench_bypass_checker(c: &mut Criterion) {
    let checker = BypassChecker::new();
    let content = generate_violating_content();

    c.bench_function("bypass_checker_clean_file", |b| {
        let clean = "fn main() {\n    println!(\"hello\");\n}\n";
        b.iter(|| {
            let mut violations = Vec::new();
            checker.check_bypass_comments("src/lib.rs", clean, &mut violations);
        })
    });

    c.bench_function("bypass_checker_violating_file", |b| {
        b.iter(|| {
            let mut violations = Vec::new();
            checker.check_bypass_comments("src/lib.rs", &content, &mut violations);
        })
    });
}

fn bench_duplication_analyzer(c: &mut Criterion) {
    let config = Arc::new(ArchitectureConfig::default());
    let ana = CodeDuplicationAnalyzer::from_config(config);

    let base_content = generate_content(50);
    let entries_identical: Vec<(PathBuf, String)> = vec![
        (PathBuf::from("src/a.rs"), base_content.clone()),
        (PathBuf::from("src/b.rs"), base_content),
    ];

    let different_content = (0..50)
        .map(|i| format!("fn unique_{}() {{ let z = {}; }}", i, i))
        .collect::<Vec<_>>()
        .join("\n");
    let entries_different: Vec<(PathBuf, String)> = vec![
        (PathBuf::from("src/a.rs"), generate_content(50)),
        (PathBuf::from("src/b.rs"), different_content),
    ];

    c.bench_function("duplication_analyzer_identical", |b| {
        b.iter(|| {
            ana.handle_duplicates_entries(&entries_identical);
        })
    });

    c.bench_function("duplication_analyzer_different", |b| {
        b.iter(|| {
            ana.handle_duplicates_entries(&entries_different);
        })
    });
}

fn bench_full_analysis(c: &mut Criterion) {
    let container = CodeAnalysisContainer::new();
    let linter = container.code_analysis_linter();

    let violating = generate_violating_content();
    let entries: Vec<FileEntry> = (0..5)
        .map(|i| FileEntry {
            path: PathBuf::from(format!("src/file_{}.rs", i)),
            extension: "rs".to_string(),
            language: shared::common::taxonomy_language_vo::Language::Rust,
            size: violating.len() as u64,
            content: violating.clone(),
            parse_ok: true,
            parse_metadata: None,
        })
        .collect();

    c.bench_function("full_analysis_5_violating_files", |b| {
        b.iter(|| {
            linter.run_analysis_with_entries(&entries);
        })
    });

    let clean_entries: Vec<FileEntry> = (0..5)
        .map(|i| FileEntry {
            path: PathBuf::from(format!("src/clean_{}.rs", i)),
            extension: "rs".to_string(),
            language: shared::common::taxonomy_language_vo::Language::Rust,
            size: 50,
            content: "fn main() {\n    println!(\"hello\");\n}\n".to_string(),
            parse_ok: true,
            parse_metadata: None,
        })
        .collect();

    c.bench_function("full_analysis_5_clean_files", |b| {
        b.iter(|| {
            linter.run_analysis_with_entries(&clean_entries);
        })
    });
}

criterion_group!(
    benches,
    bench_line_checker,
    bench_bypass_checker,
    bench_duplication_analyzer,
    bench_full_analysis,
);
criterion_main!(benches);
