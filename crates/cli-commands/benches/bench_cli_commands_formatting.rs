//! Benchmark tests for formatting utilities — JUnit XML and SARIF output generation.
//!
//! Uses criterion for statistically rigorous measurement.
//! Registered in Cargo.toml with `harness = false`.

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};

use cli_commands_lint_arwaky::utility_format_output::{
    format_junit_output, format_sarif_output, xml_escape,
};
use shared::cli_commands::taxonomy_result_vo::LintResult;
use shared::common::taxonomy_severity_vo::Severity;

fn generate_results(count: usize) -> Vec<LintResult> {
    (0..count)
        .map(|i| {
            LintResult::new_arch(
                &format!("src/module_{:04}.rs", i),
                i + 1,
                &format!("AES{}", 100 + (i % 6)),
                match i % 4 {
                    0 => Severity::CRITICAL,
                    1 => Severity::HIGH,
                    2 => Severity::MEDIUM,
                    _ => Severity::LOW,
                },
                format!("Violation message number {} with some descriptive text", i),
            )
        })
        .collect()
}

// ─── JUnit XML Formatting ────────────────────────────────────────────────────

fn bench_junit_formatting(c: &mut Criterion) {
    let mut group = c.benchmark_group("junit_formatting");

    for size in [10, 100, 1000] {
        let results = generate_results(size);
        group.bench_with_input(BenchmarkId::new("junit_xml", size), &results, |b, data| {
            b.iter(|| format_junit_output(data))
        });
    }
    group.finish();
}

// ─── SARIF Formatting ────────────────────────────────────────────────────────

fn bench_sarif_formatting(c: &mut Criterion) {
    let mut group = c.benchmark_group("sarif_formatting");

    for size in [10, 100, 1000] {
        let results = generate_results(size);
        group.bench_with_input(BenchmarkId::new("sarif_json", size), &results, |b, data| {
            b.iter(|| format_sarif_output(data))
        });
    }
    group.finish();
}

// ─── XML Escaping ────────────────────────────────────────────────────────────

fn bench_xml_escape(c: &mut Criterion) {
    let mut group = c.benchmark_group("xml_escape");

    let inputs = [
        ("plain", "hello world no special chars"),
        ("special", "a<b>c&d\"e'f<g>h&i\"j'k"),
        ("long", &"x<y>z&a".repeat(100)),
    ];

    for (name, input) in &inputs {
        group.bench_with_input(BenchmarkId::new("escape", *name), input, |b, data| {
            b.iter(|| xml_escape(data))
        });
    }
    group.finish();
}

criterion_group!(
    benches,
    bench_junit_formatting,
    bench_sarif_formatting,
    bench_xml_escape,
);
criterion_main!(benches);
