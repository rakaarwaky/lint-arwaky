use criterion::{Criterion, criterion_group, criterion_main};

fn bench_import_rules_throughput(c: &mut Criterion) {
    c.bench_function("placeholder", |b| b.iter(|| {}));
}

criterion_group!(benches, bench_import_rules_throughput);
criterion_main!(benches);
