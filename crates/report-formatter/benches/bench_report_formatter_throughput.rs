// PURPOSE: Benchmark — measure report-formatter component throughput.
// Layer: Benchmark (performance validation).

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use report_formatter_lint_arwaky::capabilities_json_formatter::JsonFormatter;
use report_formatter_lint_arwaky::capabilities_junit_formatter::JunitFormatter;
use report_formatter_lint_arwaky::capabilities_sarif_formatter::SarifFormatter;
use report_formatter_lint_arwaky::capabilities_text_formatter::TextFormatter;
use shared::cli_commands::taxonomy_result_vo::LintResult;
use shared::cli_commands::taxonomy_scan_report_vo::ScanReport;

fn bench_formatter_instantiation(c: &mut Criterion) {
    c.bench_function("formatter_instantiation", |b| {
        b.iter(|| {
            let _text = TextFormatter::new(
                code_analysis::root_code_analysis_container::CodeAnalysisContainer::default()
                    .code_analysis_linter(),
            );
            let _json = JsonFormatter::new();
            let _sarif = SarifFormatter::new();
            let _junit = JunitFormatter::new();
        });
    });
}

fn bench_text_format(c: &mut Criterion) {
    c.bench_function("text_format", |b| {
        b.iter(|| {
            let text = TextFormatter::new(
                code_analysis::root_code_analysis_container::CodeAnalysisContainer::default()
                    .code_analysis_linter(),
            );
            let report = ScanReport::new(vec![], vec![]);
            let _ = text.format_text(black_box(&report));
        });
    });
}

fn bench_sarif_format(c: &mut Criterion) {
    c.bench_function("sarif_format", |b| {
        b.iter(|| {
            let sarif = SarifFormatter::new();
            let results: Vec<LintResult> = vec![];
            let _ = sarif.format_sarif(black_box(&results));
        });
    });
}

fn bench_junit_format(c: &mut Criterion) {
    c.bench_function("junit_format", |b| {
        b.iter(|| {
            let junit = JunitFormatter::new();
            let results: Vec<LintResult> = vec![];
            let _ = junit.format_junit(black_box(&results));
        });
    });
}

criterion_group!(
    benches,
    bench_formatter_instantiation,
    bench_text_format,
    bench_sarif_format,
    bench_junit_format
);
criterion_main!(benches);
