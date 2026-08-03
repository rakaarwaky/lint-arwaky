// Benchmark tests for mcp-server — tool command dispatch throughput.
use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};

fn bench_tool_args_parse(c: &mut Criterion) {
    let mut group = c.benchmark_group("tool_args_parse");
    group.significance_level(0.05).confidence_level(0.95);

    let args_json = r#"{"project_root": "/tmp/project", "language": "rust"}"#;
    let args_large = r#"{"project_root": "/tmp/very/deeply/nested/project/path/that/goes/on/and/on", "language": "rust", "use_default_config": true, "include_patterns": ["src/**"], "exclude_patterns": ["target/**", ".git/**"]}"#;

    group.bench_with_input(
        BenchmarkId::new("small_args", "2_fields"),
        &args_json,
        |b, json| {
            b.iter(|| {
                std::hint::black_box(
                    serde_json::from_str::<serde_json::Value>(json).unwrap(),
                );
            });
        },
    );
    group.bench_with_input(
        BenchmarkId::new("large_args", "6_fields"),
        &args_large,
        |b, json| {
            b.iter(|| {
                std::hint::black_box(
                    serde_json::from_str::<serde_json::Value>(json).unwrap(),
                );
            });
        },
    );

    group.finish();
}

criterion_group!(benches, bench_tool_args_parse,);
criterion_main!(benches);
