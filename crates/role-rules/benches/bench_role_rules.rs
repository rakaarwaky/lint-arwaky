// PURPOSE: Criterion benchmarks for role-rules — AES401–AES406 audit
// throughput across files with different layer prefixes. Replaces the
// previous container-construction-only benchmark (issue #1365): the audit
// pipeline (classifier routing + per-layer auditors) is what the scan
// actually pays for.
use criterion::{BenchmarkId, Criterion, Throughput, criterion_group, criterion_main};
use role_rules_lint_arwaky::root_role_rules_container::RoleContainer;
use shared_config_system::ArchitectureConfig;
use shared_filesystem::taxonomy_filesystem_vo::{FileEntry, Language};
use shared_role_rules::taxonomy_role_rules_request::RoleRequest;
use std::path::PathBuf;

/// Generate `n` pre-parsed Rust file entries with a `capabilities_` filename
/// prefix so the classifier routes them to the capabilities auditors.
fn generate_entries(n: usize) -> Vec<FileEntry> {
    (0..n)
        .map(|i| FileEntry {
            path: PathBuf::from(format!("crates/feature/src/capabilities_module_{i}.rs")),
            extension: "rs".to_string(),
            language: Language::Rust,
            size: 0,
            content: format!(
                "pub struct Module_{i} {{}}\nimpl Module_{i} {{ pub fn new() -> Self {{ Self {{}} }} }}"
            ),
            parse_ok: true,
            parse_metadata: None,
        })
        .collect()
}

fn bench_role_audit(c: &mut Criterion) {
    let container = RoleContainer::new_with_config(ArchitectureConfig::default());
    let orchestrator = container.orchestrator();
    let mut group = c.benchmark_group("role_audit_throughput");
    group.significance_level(0.05).confidence_level(0.95);

    for n in [10, 50, 100] {
        let files = generate_entries(n);
        group.throughput(Throughput::Elements(n as u64));
        group.bench_with_input(BenchmarkId::new("capabilities_files", n), &n, |b, &_n| {
            b.iter(|| {
                let response = orchestrator.execute(RoleRequest::RunAuditWithEntries {
                    files: files.clone(),
                });
                std::hint::black_box(response.into_violations());
            });
        });
    }

    group.finish();
}

criterion_group!(benches, bench_role_audit);
criterion_main!(benches);
