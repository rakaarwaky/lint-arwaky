// Benchmark tests for dispatcher — output component and check action throughput.
use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};

fn bench_output_json(c: &mut Criterion) {
    let mut group = c.benchmark_group("output_json");
    group.significance_level(0.05).confidence_level(0.95);

    let results: Vec<shared::common::LintResult> = (0..200)
        .map(|i| shared::common::taxonomy_lint_result_vo::LintResult {
            file: shared::common::FilePath::new(format!("src/module_{}.rs", i)).unwrap(),
            line: shared::common::LineNumber::new(i as i64),
            column: Default::default(),
            code: shared::common::ErrorCode::raw(format!("AES{}", 100 + (i % 15))),
            message: shared::common::LintMessage::new(format!("violation at module_{}.rs:{}", i, i)),
            source: None,
            severity: if i % 3 == 0 {
                shared::common::Severity::HIGH
            } else {
                shared::common::Severity::MEDIUM
            },
            enclosing_scope: None,
            related_locations: Default::default(),
        })
        .collect();

    for n in [20, 100, 200] {
        group.throughput(criterion::Throughput::Elements(n as u64));
        group.bench_with_input(
            BenchmarkId::new("serialize_results", n),
            &results[..n],
            |b, r| {
                b.iter(|| {
                    std::hint::black_box(serde_json::to_string(r).unwrap());
                });
            },
        );
    }

    group.finish();
}

fn bench_ci_report_construction(c: &mut Criterion) {
    let mut group = c.benchmark_group("ci_report_construction");
    group.significance_level(0.05).confidence_level(0.95);
    group.sample_size(30);

    group.bench_function("create_ci_report", |b| {
        b.iter(|| {
            let report = dispatcher_lint_arwaky::surface_ci_action::CiReport {
                version: "1.11.0".to_string(),
                score: 0.85,
                threshold: 70,
                pass: true,
                reasons: vec!["AES101".to_string(), "AES304".to_string()],
                critical: 1,
                high: 2,
                medium: 3,
                low: 4,
                total_violations: 10,
            };
            std::hint::black_box(report);
        });
    });

    group.finish();
}

criterion_group!(benches, bench_output_json, bench_ci_report_construction,);
criterion_main!(benches);
