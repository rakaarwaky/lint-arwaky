// PURPOSE: unit tests for the doc-rules capabilities — AES601–AES605 seams
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
    DocAuditContext::new(Path::new("/repo"), documents, Some(master))
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

/// The API Contract tables the template requires: one per mandated subsection,
/// each carrying the full column set the rule checks for.
fn api_contract_section() -> &'static str {
    "## API Contract\n\n### Protocol API\n\n\
     | Method | Input | Output | Error | Event | Description |\n\
     | --- | --- | --- | --- | --- | --- |\n\
     | scan | path | findings | none | none | audit a target |\n\n\
     ### Aggregate API\n\n\
     | Method | Input | Output | Error | Event | Description |\n\
     | --- | --- | --- | --- | --- | --- |\n\
     | execute | request | response | none | none | single entry point |\n\n"
}

/// A complete FRD: every mandated section, each in the shape the rule demands.
fn conforming_frd() -> String {
    format!(
        "# FRD\n\n{}\
         ## Integration Points\n\n\
         | System | Direction | Purpose | Failure mode |\n\
         | --- | --- | --- | --- |\n\
         | filesystem | out | read source | missing path |\n\n\
         ## Non-functional Requirements\n\n\
         | Metric | Target | Measurement method |\n\
         | --- | --- | --- |\n\
         | scan time | under 1s | wall clock |\n\n\
         ## Test Scenarios\n\n- scans a clean workspace without findings\n\n\
         ## Glossary\n\n- **Finding**: a single rule violation\n",
        api_contract_section()
    )
}

#[test]
fn aes602_accepts_a_frd_carrying_every_mandated_section() {
    let findings = SectionStructureChecker::new()
        .audit_section_structure(&context("FRD.md", &conforming_frd()));
    assert!(
        codes(&findings).is_empty(),
        "a complete FRD must report nothing; got {findings:?}"
    );
}

#[test]
fn aes602_rejects_a_frd_missing_its_test_scenarios() {
    // The same document with the bullet list removed from Test Scenarios — a
    // heading that promises scenarios but states none.
    let text = conforming_frd().replace("- scans a clean workspace without findings\n", "");
    let findings =
        SectionStructureChecker::new().audit_section_structure(&context("FRD.md", &text));
    assert!(
        findings
            .iter()
            .any(|f| f.violation_type == "scenarios_empty"),
        "an empty Test Scenarios section must fire AES602; got {findings:?}"
    );
}

#[test]
fn aes602_rejects_a_frd_whose_api_contract_is_not_a_table() {
    // Same document, but Protocol API states its method as prose instead of a
    // table — the shape the template forbids.
    let table = "| Method | Input | Output | Error | Event | Description |\n\
         | --- | --- | --- | --- | --- | --- |\n\
         | scan | path | findings | none | none | audit a target |";
    let text = conforming_frd().replace(table, "the scan method audits a target path");
    assert_ne!(
        text,
        conforming_frd(),
        "the table under test must actually differ; a no-op replace would make this \
         case assert the conforming document and pass for the wrong reason"
    );
    let findings =
        SectionStructureChecker::new().audit_section_structure(&context("FRD.md", &text));
    assert!(
        findings
            .iter()
            .any(|f| f.violation_type == "api_subsection_no_table"),
        "a prose method list must fire AES602; got {findings:?}"
    );
}
