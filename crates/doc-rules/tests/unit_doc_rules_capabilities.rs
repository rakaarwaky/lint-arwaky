// PURPOSE: unit tests for the doc-rules capabilities — AES601–AES606 seams
//
// Each case exercises the smallest input that decides its rule: a heading that
// must be counted, a section that must be found, a shape that must be rejected.
// The contract tests prove the seams exist; these prove each one answers its own
// question.
use doc_rules_lint_arwaky::capabilities_doc_heading_structure_checker::{
    AGENTS_TEMPLATE, HeadingStructureChecker,
};
use doc_rules_lint_arwaky::capabilities_doc_section_structure_checker::SectionStructureChecker;
use shared_doc_rules::contract_doc_protocol::{IDocHeadingProtocol, ISectionStructureProtocol};
use shared_doc_rules::taxonomy_doc_audit_context_vo::DocAuditContext;
use shared_doc_rules::taxonomy_doc_rules_request::{DocFinding, DocSource};
use std::path::{Path, PathBuf};

/// An audit context holding one document named *name* with *text* as its body.
fn context(name: &str, text: &str) -> DocAuditContext {
    let documents = vec![DocSource {
        path: PathBuf::from(format!("/repo/{name}")),
        text: text.to_string(),
    }];
    let master = documents[0].text.clone();
    let members = Vec::new();
    DocAuditContext::new(
        Path::new("/repo"),
        documents,
        Some(master),
        members,
        None,
        shared_doc_rules::taxonomy_doc_audit_context_vo::StrictModeOverride::UseConfigFile,
    )
}

/// The distinct codes reported for one document, so a test asserts on the set of
/// rules that fired rather than on message wording.
fn codes(findings: &[DocFinding]) -> Vec<&'static str> {
    let mut codes: Vec<&'static str> = findings.iter().map(|f| f.code).collect();
    codes.sort_unstable();
    codes.dedup();
    codes
}

/// An `AGENTS.md` that matches the template exactly.
///
/// Built from the same literal the checker enforces against, so the fixture and
/// the contract can never drift apart: a conforming document is the template
/// itself.
fn conforming_agents() -> String {
    AGENTS_TEMPLATE.to_string()
}

/// An `AGENTS.md` that is conforming except for the one defect under test.
fn agents_with_extra_h2() -> String {
    format!("{}## Invented Section\n\nbody\n", conforming_agents())
}

// ─── AES605 — heading structure ────────────────────────────────────────────

#[test]
fn aes605_accepts_one_h1_and_every_required_h2() {
    let findings = HeadingStructureChecker::new()
        .audit_doc_heading(&context("AGENTS.md", &conforming_agents()));
    assert!(
        codes(&findings).is_empty(),
        "a conforming heading shape must report nothing; got {findings:?}"
    );
}

#[test]
fn aes605_rejects_a_second_h1() {
    let text = format!("# AGENTS\n\n{}\n# Second\n", conforming_agents());
    let findings = HeadingStructureChecker::new().audit_doc_heading(&context("AGENTS.md", &text));
    assert!(
        codes(&findings).contains(&"AES605"),
        "two level-1 headings must fire AES605; exactly one H1 is the contract"
    );
}

#[test]
fn aes605_rejects_an_off_template_h2() {
    let findings = HeadingStructureChecker::new()
        .audit_doc_heading(&context("AGENTS.md", &agents_with_extra_h2()));
    assert!(
        codes(&findings).contains(&"AES605"),
        "an H2 outside the contract's closed set must fire AES605; a heading the \
         rule does not know is reported rather than ignored"
    );
}

// ─── AES602 — section structure ────────────────────────────────────────────

/// A complete FRD in template order, with free-form H3s under it.
fn conforming_frd() -> String {
    "# FRD\n\n\
     ## Feature Overview\n\nThe feature does the work a surface delegates to.\n\n\
     ## Functional Requirements\n\n\
     ### FR-SAMPLE-001: Document Invariant Enforcement\n\n\
     | Aspect | Detail |\n\
     |---|---|\n\
     | Description | The capability audits documents. |\n\n\
     ## Public Interface\n\n\
     | Aspect | Detail |\n\
     |---|---|\n\
     | Method | `execute` |\n\n\
     ## Reference\n\n- PRD: [PRD.md](../../PRD.md)\n"
        .to_string()
}

#[test]
fn aes602_accepts_a_frd_in_template_order() {
    let findings = SectionStructureChecker::new()
        .audit_section_structure(&context("FRD.md", &conforming_frd()));
    assert!(
        codes(&findings).is_empty(),
        "an FRD in template order must report nothing; got {findings:?}"
    );
}

#[test]
fn aes602_rejects_a_frd_out_of_template_order() {
    // Swap Feature Overview and Functional Requirements in place: the same
    // four sections, the wrong reading order.
    let text = conforming_frd()
        .replace("## Feature Overview", "## @@MARK@@")
        .replace("## Functional Requirements", "## Feature Overview")
        .replace("## @@MARK@@", "## Functional Requirements");
    assert_ne!(
        text,
        conforming_frd(),
        "the order under test must actually differ; a no-op replace would make this \
         case assert the conforming document and pass for the wrong reason"
    );
    let findings =
        SectionStructureChecker::new().audit_section_structure(&context("FRD.md", &text));
    assert!(
        codes(&findings).contains(&"AES602"),
        "sections out of template order must fire AES602; got {findings:?}"
    );
}
