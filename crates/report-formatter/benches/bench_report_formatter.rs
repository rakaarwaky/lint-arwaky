// Benchmark — report-formatter: formatting N violations in each output format.
use criterion::{BenchmarkId, Criterion, Throughput, criterion_group, criterion_main};
use report_formatter_lint_arwaky::capabilities_json_formatter::JsonFormatter;
use report_formatter_lint_arwaky::capabilities_junit_formatter::JunitFormatter;
use report_formatter_lint_arwaky::capabilities_sarif_formatter::SarifFormatter;
use report_formatter_lint_arwaky::capabilities_text_formatter::TextFormatter;
use shared::cli_commands::{LintResult, ScanReport};
use shared::common::{AdapterName, ErrorCode, FilePath, LineNumber, LintMessage, Severity};

fn report_with_n_violations(n: usize) -> ScanReport {
    let results = (0..n)
        .map(|i| LintResult {
            file: FilePath::new(format!("src/file_{}.rs", i % 32)).unwrap(),
            line: LineNumber::new((i as i64 % 500) + 1),
            code: ErrorCode::raw(if i % 2 == 0 { "AES201" } else { "AES301" }),
            message: LintMessage::new(format!("violation number {i}")),
            source: Some(AdapterName::raw("architecture")),
            severity: Severity::HIGH,
            ..Default::default()
        })
        .collect();
    ScanReport {
        results,
        diagnostics: vec![],
        score: Some(shared::common::Score::new(60.0)),
    }
}

fn bench_text_format(c: &mut Criterion) {
    let mut group = c.benchmark_group("text_format");
    for n in [10usize, 100, 1000] {
        let report = report_with_n_violations(n);
        group.throughput(Throughput::Elements(n as u64));
        group.bench_with_input(BenchmarkId::new("violations", n), &report, |b, r| {
            b.iter(|| std::hint::black_box(TextFormatter::new().format_text(r)))
        });
    }
    group.finish();
}

fn bench_json_format(c: &mut Criterion) {
    let mut group = c.benchmark_group("json_format");
    for n in [10usize, 100, 1000] {
        let report = report_with_n_violations(n);
        group.throughput(Throughput::Elements(n as u64));
        group.bench_with_input(BenchmarkId::new("violations", n), &report, |b, r| {
            b.iter(|| std::hint::black_box(JsonFormatter::new().format_json(r)))
        });
    }
    group.finish();
}

fn bench_sarif_format(c: &mut Criterion) {
    let mut group = c.benchmark_group("sarif_format");
    for n in [10usize, 100, 1000] {
        let report = report_with_n_violations(n);
        group.throughput(Throughput::Elements(n as u64));
        group.bench_with_input(BenchmarkId::new("violations", n), &report, |b, r| {
            b.iter(|| std::hint::black_box(SarifFormatter::new().format_sarif_report(r)))
        });
    }
    group.finish();
}

fn bench_junit_format(c: &mut Criterion) {
    let mut group = c.benchmark_group("junit_format");
    for n in [10usize, 100, 1000] {
        let report = report_with_n_violations(n);
        group.throughput(Throughput::Elements(n as u64));
        group.bench_with_input(BenchmarkId::new("violations", n), &report, |b, r| {
            b.iter(|| std::hint::black_box(JunitFormatter::new().format_junit_report(r)))
        });
    }
    group.finish();
}

criterion_group!(
    benches,
    bench_text_format,
    bench_json_format,
    bench_sarif_format,
    bench_junit_format
);
criterion_main!(benches);
