// PURPOSE: Benchmark — measure report-formatter component throughput.
// Layer: Benchmark (performance validation).
// Best practices: significance_level(0.05), sample_size(30+), throughput measurement,
//                 real data instead of empty vectors

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use report_formatter_lint_arwaky::capabilities_json_formatter::JsonFormatter;
use report_formatter_lint_arwaky::capabilities_junit_formatter::JunitFormatter;
use report_formatter_lint_arwaky::capabilities_sarif_formatter::SarifFormatter;
use report_formatter_lint_arwaky::capabilities_text_formatter::TextFormatter;
use shared::cli_commands::taxonomy_result_vo::LintResult;
use shared::cli_commands::taxonomy_scan_report_vo::ScanReport;
use shared::common::taxonomy_severity_vo::Severity;

fn generate_results(count: usize) -> Vec<LintResult> {
    (0..count)
        .map(|i| {
            LintResult::new_arch(
                &format!("src/file_{i}.rs"),
                i + 1,
                "AES301",
                Severity::LOW,
                format!("benchmark violation {i}"),
            )
        })
        .collect()
}

fn bench_formatter_instantiation(c: &mut Criterion) {
    let mut group = c.benchmark_group("formatter_instantiation");
    group.significance_level(0.05).confidence_level(0.95);

    for n in [1, 10, 100] {
        group.throughput(Throughput::Elements(n as u64));
        group.bench_with_input(BenchmarkId::new("instances", n), &n, |b, val| {
            let count = *val;
            b.iter(|| {
                for _ in 0..count {
                    let _text = TextFormatter::new(
                        code_analysis::root_code_analysis_container::CodeAnalysisContainer::default()
                            .code_analysis_linter(),
                    );
                    let _json = JsonFormatter::new();
                    let _sarif = SarifFormatter::new();
                    let _junit = JunitFormatter::new();
                }
            });
        });
    }
    group.finish();
}

fn bench_text_format(c: &mut Criterion) {
    let mut group = c.benchmark_group("text_format");
    group.sample_size(30);

    for result_count in [10, 100, 1000] {
        let results = generate_results(result_count);
        let report = ScanReport::new(results.clone(), Vec::new());
        group.throughput(Throughput::Elements(result_count as u64));
        let text = TextFormatter::new(
            code_analysis::root_code_analysis_container::CodeAnalysisContainer::default()
                .code_analysis_linter(),
        );
        group.bench_with_input(
            BenchmarkId::new("results", result_count),
            &report,
            |b, report| b.iter(|| black_box(text.format_text(report))),
        );
    }
    group.finish();
}

fn bench_sarif_format(c: &mut Criterion) {
    let mut group = c.benchmark_group("sarif_format");
    group.sample_size(30);

    for result_count in [10, 100, 1000] {
        let results = generate_results(result_count);
        group.throughput(Throughput::Elements(result_count as u64));
        let sarif = SarifFormatter::new();
        group.bench_with_input(
            BenchmarkId::new("results", result_count),
            &results,
            |b, results| b.iter(|| black_box(sarif.format_sarif(results))),
        );
    }
    group.finish();
}

fn bench_junit_format(c: &mut Criterion) {
    let mut group = c.benchmark_group("junit_format");
    group.sample_size(30);

    for result_count in [10, 100, 1000] {
        let results = generate_results(result_count);
        group.throughput(Throughput::Elements(result_count as u64));
        let junit = JunitFormatter::new();
        group.bench_with_input(
            BenchmarkId::new("results", result_count),
            &results,
            |b, results| b.iter(|| black_box(junit.format_junit(results))),
        );
    }
    group.finish();
}

criterion_group!(
    benches,
    bench_formatter_instantiation,
    bench_text_format,
    bench_sarif_format,
    bench_junit_format
);
criterion_main!(benches);
