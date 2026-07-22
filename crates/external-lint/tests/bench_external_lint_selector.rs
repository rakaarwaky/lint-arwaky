// PURPOSE: Benchmark tests for CapabilitiesExternalLintSelector — measures
// adapter selection throughput under various language combinations.

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use external_lint_lint_arwaky::capabilities_external_lint_selector::CapabilitiesExternalLintSelector;
use shared::external_lint::contract_external_lint_selector_protocol::IExternalLintSelectorProtocol;

fn bench_select_adapters(c: &mut Criterion) {
    let selector = CapabilitiesExternalLintSelector::with_defaults();

    let mut group = c.benchmark_group("external_lint_selector");

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
        group.bench_with_input(
            BenchmarkId::new("select_adapters", name),
            &(rs, py, js),
            |b, &(rs, py, js)| {
                b.iter(|| selector.select_adapters(rs, py, js));
            },
        );
    }

    group.finish();
}

fn bench_container_creation(c: &mut Criterion) {
    c.bench_function("external_lint_container_new", |b| {
        b.iter(|| external_lint_lint_arwaky::ExternalLintContainer::new());
    });
}

fn bench_adapter_names(c: &mut Criterion) {
    let container = external_lint_lint_arwaky::ExternalLintContainer::new();
    let aggregate = container.aggregate();

    c.bench_function("external_lint_adapter_names", |b| {
        b.iter(|| aggregate.adapter_names());
    });
}

criterion_group!(
    benches,
    bench_select_adapters,
    bench_container_creation,
    bench_adapter_names
);
criterion_main!(benches);
