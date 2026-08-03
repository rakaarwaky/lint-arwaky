// Benchmark tests for cli-commands — formatting and scan dispatch throughput.
use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};

fn bench_format_lint_results(c: &mut Criterion) {
    let mut group = c.benchmark_group("format_lint_results");
    group.significance_level(0.05).confidence_level(0.95);

    let results: Vec<shared::common::LintResult> = (0..100)
        .map(|i| shared::common::taxonomy_lint_result_vo::LintResult {
            file: shared::common::FilePath::new(format!("src/file_{}.rs", i)).unwrap(),
            line: shared::common::LineNumber::new(i as i64),
            column: Default::default(),
            code: shared::common::ErrorCode::raw(format!("AES{}", 100 + (i % 10))),
            message: shared::common::LintMessage::new(format!("violation at file_{}.rs:{}", i, i)),
            source: None,
            severity: shared::common::Severity::Warning,
            enclosing_scope: None,
            related_locations: Default::default(),
        })
        .collect();

    for n in [10, 50, 100] {
        group.bench_with_input(BenchmarkId::new("json_format", n), &results[..n], |b, r| {
            b.iter(|| {
                std::hint::black_box(serde_json::to_string(r).unwrap());
            });
        });
    }

    group.finish();
}

fn bench_scan_request_construction(c: &mut Criterion) {
    let mut group = c.benchmark_group("scan_request_construction");
    group.significance_level(0.05).confidence_level(0.95);
    group.sample_size(30);

    let root = shared::common::FilePath::new("/tmp/project".to_string()).unwrap();
    group.bench_function("create_scan_request", |b| {
        b.iter(|| {
            let req = shared::cli_commands::ScanRequest {
                project_root: root.clone(),
                languages: vec![shared::common::ConfigLanguage::Rust],
                use_default_config: true,
            };
            std::hint::black_box(req);
        });
    });

    group.finish();
}

criterion_group!(
    benches,
    bench_format_lint_results,
    bench_scan_request_construction,
);
criterion_main!(benches);
