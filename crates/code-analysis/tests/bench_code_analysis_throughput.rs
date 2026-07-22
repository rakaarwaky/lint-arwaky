// PURPOSE: Benchmark tests for code-analysis — measures throughput of
// bypass checking, line counting, and duplication analysis.

use code_analysis_lint_arwaky::{
    ArchLineChecker, BypassChecker, CodeDuplicationAnalyzer, MandatoryDefinitionChecker,
};
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use shared::code_analysis::contract_bypass_checker_protocol::IBypassCheckerProtocol;
use shared::code_analysis::contract_class_protocol::IMandatoryClassProtocol;
use shared::code_analysis::contract_line_protocol::ILineCheckerProtocol;
use shared::common::taxonomy_common_vo::{BooleanVO, Count};
use shared::common::taxonomy_definition_vo::LayerDefinition;

fn generate_clean_rust(lines: usize) -> String {
    (0..lines)
        .map(|i| format!("    let field_{}: i32 = {};", i, i))
        .collect::<Vec<_>>()
        .join("\n")
}

fn generate_bypass_rust(lines: usize) -> String {
    (0..lines)
        .map(|i| {
            if i % 10 == 0 {
                format!("    let x_{} = opt.unwrap();", i)
            } else {
                format!("    let val_{}: i32 = {};", i, i)
            }
        })
        .collect::<Vec<_>>()
        .join("\n")
}

fn make_def() -> LayerDefinition {
    LayerDefinition {
        code_analysis: shared::code_analysis::taxonomy_code_analysis_rule_vo::CodeAnalysisRuleVO {
            min_lines: Count::new(5),
            max_lines: Count::new(1000),
            mandatory_class_definition: BooleanVO::new(true),
            ..Default::default()
        },
        ..Default::default()
    }
}

// ─── Benchmark: Bypass checking throughput ───────────────────────────

fn bench_bypass_checker(c: &mut Criterion) {
    let checker = BypassChecker::new();
    let mut group = c.benchmark_group("bypass_checker");

    for size in [100, 500, 1000] {
        let content = generate_bypass_rust(size);
        group.bench_with_input(
            BenchmarkId::new("check_bypass_comments", size),
            &content,
            |b, data| {
                b.iter(|| {
                    let mut violations = Vec::new();
                    checker.check_bypass_comments("bench.rs", data, &mut violations);
                    violations
                })
            },
        );
    }
    group.finish();
}

// ─── Benchmark: Line checker throughput ──────────────────────────────

fn bench_line_checker(c: &mut Criterion) {
    let checker = ArchLineChecker::new();
    let def = make_def();
    let mut group = c.benchmark_group("line_checker");

    for size in [100, 500, 1000] {
        let content = generate_clean_rust(size);
        group.bench_with_input(
            BenchmarkId::new("check_line_counts", size),
            &content,
            |b, data| {
                b.iter(|| {
                    let mut violations = Vec::new();
                    checker.check_line_counts("bench.rs", Some(&def), data, &mut violations);
                    violations
                })
            },
        );
    }
    group.finish();
}

// ─── Benchmark: Mandatory definition checker throughput ──────────────

fn bench_mandatory_checker(c: &mut Criterion) {
    let checker = MandatoryDefinitionChecker::new();
    let def = make_def();
    let mut group = c.benchmark_group("mandatory_checker");

    for size in [100, 500, 1000] {
        let content = format!(
            "pub struct BenchStruct {{\n{}\n}}",
            generate_clean_rust(size)
        );
        group.bench_with_input(
            BenchmarkId::new("check_mandatory_class_definition", size),
            &content,
            |b, data| {
                b.iter(|| {
                    let mut violations = Vec::new();
                    checker.check_mandatory_class_definition(
                        "bench.rs",
                        Some(&def),
                        data,
                        &mut violations,
                    );
                    violations
                })
            },
        );
    }
    group.finish();
}

// ─── Benchmark: Duplication analysis throughput ──────────────────────

fn bench_duplication_analyzer(c: &mut Criterion) {
    let analyzer = CodeDuplicationAnalyzer::new();
    let mut group = c.benchmark_group("duplication_analyzer");

    for file_count in [5, 10, 20] {
        let entries: Vec<(String, String)> = (0..file_count)
            .map(|i| {
                let content = generate_clean_rust(50);
                (format!("file_{}.rs", i), content)
            })
            .collect();

        group.bench_with_input(
            BenchmarkId::new("check_file_similarity_entries", file_count),
            &entries,
            |b, data| b.iter(|| analyzer.check_file_similarity_entries(data, 5, 50.0)),
        );
    }
    group.finish();
}

criterion_group!(
    benches,
    bench_bypass_checker,
    bench_line_checker,
    bench_mandatory_checker,
    bench_duplication_analyzer,
);
criterion_main!(benches);
