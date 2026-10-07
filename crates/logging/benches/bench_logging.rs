// Benchmarks for logging — filter construction and subscriber init cost.
use criterion::{Criterion, criterion_group, criterion_main};
use logging_lint_arwaky::ISubscriberInitProtocol;
use logging_lint_arwaky::LogVerbosity;
use logging_lint_arwaky::SubscriberInit;

fn bench_filter_construction(c: &mut Criterion) {
    c.bench_function("filter_construction", |b| {
        let subscriber = SubscriberInit::default();
        b.iter(|| {
            [
                LogVerbosity::Default,
                LogVerbosity::Info,
                LogVerbosity::Debug,
            ]
            .into_iter()
            .for_each(|v| {
                let _ = subscriber.build_filter(v);
            })
        })
    });
}

fn bench_filter_directive(c: &mut Criterion) {
    c.bench_function("filter_directive_string", |b| {
        b.iter(|| LogVerbosity::Debug.filter_directive())
    });
}

criterion_group!(benches, bench_filter_construction, bench_filter_directive);
criterion_main!(benches);
