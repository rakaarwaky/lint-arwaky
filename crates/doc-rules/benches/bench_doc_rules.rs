use criterion::{Criterion, criterion_group, criterion_main};
use doc_rules_lint_arwaky::capabilities_doc_heading_structure_checker::HeadingStructureChecker;
use shared_doc_rules::contract_doc_protocol::IDocHeadingProtocol;
use shared_doc_rules::taxonomy_doc_audit_context_vo::DocAuditContext;
use shared_doc_rules::taxonomy_doc_rules_request::DocSource;
use std::path::PathBuf;

/// The per-document cost of the AES605 heading audit.
///
/// The heading rule runs once per document in the chain, so its cost tracks the
/// size of the documentation set rather than the codebase. The bench grows a
/// document with many level-2 headings and measures the audit over it — the
/// shape a large PRD/FRD chain has at scan time.
fn bench_doc_heading_audit(c: &mut Criterion) {
    let checker = HeadingStructureChecker::new();
    let context = heading_context(200);

    c.bench_function("doc_heading_audit", |b| {
        b.iter(|| {
            let findings = checker.audit_doc_heading(&context);
            std::hint::black_box(findings);
        });
    });
}

/// An audit context holding one `AGENTS.md` with *sections* level-2 headings.
///
/// `AGENTS.md` is one of the documents with a registered H2 contract, so the
/// heading rule does its full closed-set comparison rather than skipping it.
fn heading_context(sections: usize) -> DocAuditContext {
    let mut text = String::from("# AGENTS\n\n");
    for index in 0..sections {
        text.push_str(&format!("## Section {index}\n\nbody text\n\n"));
    }
    let documents = vec![DocSource {
        path: PathBuf::from("/repo/AGENTS.md"),
        text,
    }];
    let master = documents[0].text.clone();
    DocAuditContext::new(std::path::Path::new("/repo"), documents, Some(master))
}

criterion_group!(benches, bench_doc_heading_audit);
criterion_main!(benches);
