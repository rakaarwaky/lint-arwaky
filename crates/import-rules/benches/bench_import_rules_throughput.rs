// PURPOSE: Criterion throughput benchmarks for import-rules — AES203
// unused-import detection at file scales that matter (10–500 imports per
// file). Measures the per-file unused-import pipeline end to end: import
// alias extraction, used-symbol extraction from content, and the
// unused/exported/dispatch classification.
use criterion::{BenchmarkId, Criterion, Throughput, criterion_group, criterion_main};
use import_rules_lint_arwaky::capabilities_import_unused_checker::UnusedImportRuleChecker;
use shared_common::{FilePath, SymbolName};
use shared_filesystem::taxonomy_filesystem_vo::{ImportEntry, ImportType, Language};
use shared_import_rules::IUnusedImportProtocol;
use std::path::PathBuf;

/// Generate `n` Rust `use` imports for a single file.
fn generate_imports(n: usize) -> Vec<ImportEntry> {
    (0..n)
        .map(|i| ImportEntry {
            source_file: PathBuf::from("src/module.rs"),
            raw_path: format!("crate::symbols::symbol_{i}"),
            resolved_path: Some(PathBuf::from(format!("src/symbols/symbol_{i}.rs"))),
            import_type: ImportType::Use,
            language: Language::Rust,
            is_dynamic: false,
            is_resolved: true,
            symbols: vec![format!("symbol_{i}")],
            is_reexport: false,
            is_wildcard: false,
            line: i + 1,
        })
        .collect()
}

/// Generate Rust content that references the first `used` symbols so only the
/// remaining imports can be reported as unused.
fn generate_content(used: usize) -> String {
    (0..used)
        .map(|i| format!("fn use_{i}() {{ let x = symbol_{i}; }}"))
        .collect::<Vec<_>>()
        .join("\n")
}

fn bench_unused_imports(c: &mut Criterion) {
    let checker = UnusedImportRuleChecker::new();
    let path = FilePath::new("src/module.rs").unwrap();
    let mut group = c.benchmark_group("unused_imports_throughput");
    group.significance_level(0.05).confidence_level(0.95);

    for n in [10, 50, 200] {
        let imports = generate_imports(n);
        let used = n / 2;
        let content = generate_content(used);
        let used_identifiers: Vec<SymbolName> =
            (0..used).map(|i| SymbolName::new(format!("symbol_{i}"))).collect();
        group.throughput(Throughput::Elements(n as u64));
        group.bench_with_input(BenchmarkId::new("rust_files", n), &n, |b, &_n| {
            b.iter(|| {
                let messages = checker
                    .find_unused_imports(&path, &content, &imports, &used_identifiers)
                    .unwrap();
                std::hint::black_box(&messages);
            });
        });
    }

    group.finish();
}

criterion_group!(benches, bench_unused_imports);
criterion_main!(benches);
