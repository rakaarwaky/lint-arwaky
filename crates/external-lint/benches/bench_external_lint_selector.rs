// PURPOSE: Benchmark tests for CapabilitiesExternalLintSelector — measures
// adapter selection throughput under various language combinations.
// Best practices: significance_level(0.05), sample_size(30+), throughput measurement

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use external_lint_lint_arwaky::capabilities_external_lint_selector::CapabilitiesExternalLintSelector;
use shared::external_lint::contract_external_lint_selector_protocol::IExternalLintSelectorProtocol;

fn bench_select_adapters(c: &mut Criterion) {
    let selector = CapabilitiesExternalLintSelector::with_defaults();
    let mut group = c.benchmark_group("external_lint_selector");
    group.significance_level(0.05).confidence_level(0.95);

    let cases: Vec<(&str, bool, bool, bool)> = vec![
        ("none", false, false, false),
        ("rust_only", true, false, false),
        ("python_only", false, true, false),
        ("js_only", false, false, true),
        ("rust_python", true, true, false),
        ("rust_js", true, false, true),
        ("python_js", false, true, true),
        ("all", true, true, true),
    ];

    for (name, rs, py, js) in cases {
        group.throughput(Throughput::Elements(1));
        group.bench_with_input(
            BenchmarkId::new("select_adapters", name),
            &(rs, py, js),
            |b, &(rs, py, js)| {
                b.iter(|| black_box(selector.select_adapters(rs, py, js)));
            },
        );
    }

    group.finish();
}

fn bench_container_creation(c: &mut Criterion) {
    let mut group = c.benchmark_group("container_creation");
    group.sample_size(30);

    for n in [1, 10, 100] {
        group.throughput(Throughput::Elements(n as u64));
        group.bench_with_input(BenchmarkId::new("instances", n), &n, |b, val| {
            let count = *val;
            b.iter(|| {
                for _ in 0..count {
                    black_box(external_lint_lint_arwaky::ExternalLintContainer::new());
                }
            });
        });
    }
    group.finish();
}

fn bench_adapter_names(c: &mut Criterion) {
    let mut group = c.benchmark_group("adapter_names");
    group.sample_size(30);

    for n in [1, 10, 100] {
        group.throughput(Throughput::Elements(n as u64));
        group.bench_with_input(BenchmarkId::new("calls", n), &n, |b, val| {
            let count = *val;
            let container = external_lint_lint_arwaky::ExternalLintContainer::new();
            let aggregate = container.aggregate();
            b.iter(|| {
                for _ in 0..count {
                    black_box(aggregate.adapter_names());
                }
            });
        });
    }
    group.finish();
}

criterion_group!(
    benches,
    bench_select_adapters,
    bench_container_creation,
    bench_adapter_names
);
criterion_main!(benches);
