use criterion::{Criterion, criterion_group, criterion_main};
use shared_structure_rules::taxonomy_structure_rules_request::StructureRequest;
use shared_structure_rules::taxonomy_structure_rules_response::StructureResponse;
use std::path::PathBuf;
use structure_rules_lint_arwaky::root_structure_rules_container::RootStructureRulesContainer;

/// The per-folder cost of the folder-layout audit.
///
/// `scan` runs the structure audit over every feature folder in a workspace, so
/// this is the cost that grows with workspace size. The bench builds a synthetic
/// workspace of *folders* complete feature folders and measures one full audit
/// pass over it — the shape a large monorepo costs at scan time.
fn bench_structure_audit(c: &mut Criterion) {
    let workspace = synthetic_workspace(60);

    c.bench_function("structure_audit_full_workspace", |b| {
        b.iter(|| {
            let aggregate = RootStructureRulesContainer::orchestrator();
            let response = aggregate.execute(StructureRequest::audit_all(&workspace));
            let StructureResponse::Findings { findings } = response;
            std::hint::black_box(findings);
        });
    });
}

/// A workspace on disk holding *folders* complete feature folders.
///
/// Written to a real temp directory because the auditors read the filesystem —
/// that is the point being measured. Cleaned up when the bench process exits.
fn synthetic_workspace(folders: usize) -> PathBuf {
    let root = std::env::temp_dir().join(format!("lint-arwaky-bench-structure-{folders}"));
    let _ = std::fs::remove_dir_all(&root);
    for index in 0..folders {
        let crate_dir = root.join("crates").join(format!("feature_{index}"));
        let src = crate_dir.join("src");
        std::fs::create_dir_all(&src).expect("bench workspace must be writable");
        std::fs::write(
            src.join(format!("capabilities_calc_{index}_analyzer.rs")),
            "pub struct A;\n",
        )
        .expect("write must succeed");
        std::fs::write(
            src.join(format!("agent_calc_{index}_orchestrator.rs")),
            "pub struct B;\n",
        )
        .expect("write must succeed");
        std::fs::write(crate_dir.join("FRD.md"), "# FRD\n").expect("write must succeed");
        std::fs::write(crate_dir.join("BACKLOG.md"), "# BACKLOG\n").expect("write must succeed");
    }
    root
}

criterion_group!(benches, bench_structure_audit);
criterion_main!(benches);
