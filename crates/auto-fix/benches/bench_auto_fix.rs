// Benchmark tests for auto-fix — dry-run pipeline throughput.
use auto_fix_lint_arwaky::root_auto_fix_container::AutoFixContainer;
use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};
use shared::auto_fix::LintFixOrchestratorAggregate;
use shared::common::FilePath;
use std::sync::Arc;
use tempfile::TempDir;

fn make_dry_run_orch() -> Arc<dyn LintFixOrchestratorAggregate> {
    let filesystem = filesystem_lint_arwaky::root_filesystem_container::FilesystemContainer::new()
        .orchestrator();
    let qa = quality_rules_lint_arwaky::CodeAnalysisContainer::new();
    let container = AutoFixContainer::new(qa.code_analysis_linter());
    container.orchestrator_with_filesystem(true, filesystem)
}

fn generate_rust_files(dir: &std::path::Path, n: usize) -> Vec<FilePath> {
    let mut fps = Vec::new();
    for i in 0..n {
        let code = format!(
            "use std::collections::HashMap;\nuse std::io::Read;\n#[allow(dead_code)]\nfn func_{}() {{ let _x = HashMap::new(); }}\n",
            i
        );
        let file = dir.join(format!("bench_{}.rs", i));
        std::fs::write(&file, &code).unwrap();
        fps.push(FilePath::new(file.to_string_lossy().to_string()).unwrap());
    }
    fps
}

fn bench_dry_run_single(c: &mut Criterion) {
    let mut group = c.benchmark_group("dry_run_single");
    group.significance_level(0.05).confidence_level(0.95);
    group.sample_size(30);

    let tmp = TempDir::new().unwrap();
    std::fs::write(
        tmp.path().join("single.rs"),
        "use std::collections::HashMap;\nfn main() {}\n",
    )
    .unwrap();
    let fp = FilePath::new(tmp.path().join("single.rs").to_string_lossy().to_string()).unwrap();
    let orch = make_dry_run_orch();

    group.bench_function("single_file", |b| {
        b.iter(|| {
            std::hint::black_box(orch.execute(&fp));
        });
    });

    group.finish();
}

fn bench_dry_run_batch(c: &mut Criterion) {
    let mut group = c.benchmark_group("dry_run_batch");
    group.significance_level(0.05).confidence_level(0.95);

    let tmp = TempDir::new().unwrap();
    let orch = make_dry_run_orch();

    for n in [5, 20, 50] {
        let fps = generate_rust_files(tmp.path(), n);
        group.bench_with_input(BenchmarkId::new("files", n), &fps, |b, fps| {
            b.iter(|| {
                for fp in fps {
                    std::hint::black_box(orch.execute(fp));
                }
            });
        });
    }

    group.finish();
}

criterion_group!(benches, bench_dry_run_single, bench_dry_run_batch,);
criterion_main!(benches);
