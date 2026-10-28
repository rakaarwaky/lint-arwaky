// PURPOSE: unit tests for the AES606 capability — template body-shape
// conformance (BACKLOG/ROADMAP tables) and required-doc presence.
//
// Each case exercises the smallest input that decides its rule: a section
// body that must be a table, a column set that must be present, a document
// that must exist at the root.
use doc_rules_lint_arwaky::capabilities_doc_required_doc_checker::RequiredDocChecker;
use doc_rules_lint_arwaky::capabilities_doc_template_conformance_checker::TemplateConformanceChecker;
use shared_doc_rules::contract_doc_protocol::{IRequiredDocProtocol, ITemplateConformanceProtocol};
use shared_doc_rules::taxonomy_doc_audit_context_vo::DocAuditContext;
use shared_doc_rules::taxonomy_doc_rules_request::DocSource;
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

/// A BACKLOG.md in which every table-shaped section carries its template table.
fn conforming_backlog() -> &'static str {
    "# BACKLOG\n\n## Backlog\n\n\
     | ID | Priority | State | Dependencies | Next Action | Updated |\n\
     | --- | --- | --- | --- | --- | --- |\n\
     | SAM-01 | P0 | Ready | — | — | 2026-09-29 |\n\n\
     ## Scenario Evidence\n\n\
     | Scenario | Kind | Test file | Test name | Last verified |\n\
     | --- | --- | --- | --- | --- |\n\
     | nothing yet | Automated | tests/ | test_nothing | 2026-09-29 |\n\n\
     ## Release Readiness\n\n\
     | Area | Status | Notes |\n\
     | --- | --- | --- |\n\
     | Tests | Done | nothing |\n\n\
     ## Change Log\n\n\
     | Date | Change | By |\n\
     | --- | --- | --- |\n\
     | 2026-09-29 | init | @raka |\n"
}

/// A ROADMAP.md in which every table-shaped section carries its template table.
fn conforming_roadmap() -> &'static str {
    "# ROADMAP\n\n## Current Condition\n\n- Todo: none\n\n\
     ## State Definitions\n\n\
     | State | Meaning |\n\
     | --- | --- |\n\
     | Done | Shipped. |\n\n\
     ## Status Policy\n\n- Verified, not self-reported.\n\n\
     ## Feature Roll-up\n\n\
     | ID | Feature | Priority | State | Dependencies | Next Action | Updated |\n\
     | --- | --- | --- | --- | --- | --- | --- |\n\
     | WS-01 | Docs | P0 | Done | — | — | 2026-09-29 |\n\n\
     ## Risk Register\n\n- Risk: none.\n"
}

// ─── Template conformance — BACKLOG ───────────────────────────────────────

#[test]
fn aes606_accepts_a_backlog_carrying_every_mandated_table() {
    let findings = TemplateConformanceChecker::new()
        .audit_template_conformance(&context("BACKLOG.md", conforming_backlog()));
    assert!(
        findings.is_empty(),
        "a conforming BACKLOG must report nothing; got {findings:?}"
    );
}

#[test]
fn aes606_rejects_bullet_bodies_under_a_backlog_section() {
    // The inspected unit is a `{ID}` entry under `Backlog Status`; its body
    // must be the entry table, so bullets are the wrong shape.
    let text = "# BACKLOG\n\n## Backlog Status\n\n### {SAM-001}\n\n\
- SAM-01 is ready\n- SAM-02 is in progress\n";
    let findings =
        TemplateConformanceChecker::new().audit_template_conformance(&context("BACKLOG.md", text));
    assert!(
        findings
            .iter()
            .any(|f| f.code == "AES606" && f.violation_type == "section_not_table"),
        "a bulleted entry under Backlog Status must fire AES606 section_not_table; got {findings:?}"
    );
}

#[test]
fn aes606_rejects_a_table_missing_one_mandated_column() {
    // Reference is present as a table, but the mandated "Link" column is gone.
    let text = "# BACKLOG\n\n## Reference\n\n\
| Artifact | Purpose |\n\
| --- | --- |\n\
| PRD | Business goals. |\n";
    let findings =
        TemplateConformanceChecker::new().audit_template_conformance(&context("BACKLOG.md", text));
    assert!(
        findings
            .iter()
            .any(|f| f.code == "AES606" && f.violation_type == "table_missing_columns"),
        "a Reference table missing its Link column must fire AES606 table_missing_columns; got {findings:?}"
    );
}

// ─── Template conformance — ROADMAP ───────────────────────────────────────

#[test]
fn aes606_accepts_a_roadmap_carrying_every_mandated_table() {
    let findings = TemplateConformanceChecker::new()
        .audit_template_conformance(&context("ROADMAP.md", conforming_roadmap()));
    assert!(
        findings.is_empty(),
        "a conforming ROADMAP must report nothing; got {findings:?}"
    );
}

#[test]
fn aes606_rejects_prose_under_a_roadmap_state_definitions() {
    // The inspected unit is a `PR-…` entry under `Roadmap Status`; a prose
    // body carries no table, so the shape check rejects it.
    let text = "# ROADMAP\n\n## Roadmap Status\n\n\
### PR-DOC-001: Sample State\n\nIdea means not yet examined.\n";
    let findings =
        TemplateConformanceChecker::new().audit_template_conformance(&context("ROADMAP.md", text));
    assert!(
        findings
            .iter()
            .any(|f| f.code == "AES606" && f.violation_type == "section_not_table"),
        "prose under a Roadmap Status entry must fire AES606 section_not_table; got {findings:?}"
    );
}

// ─── Required-doc presence ─────────────────────────────────────────────────

/// Lay down a real workspace root with the required docs except *missing*.
fn write_root(root: &Path, missing: Option<&str>) {
    std::fs::create_dir_all(root).unwrap();
    std::fs::write(root.join("Cargo.toml"), "[workspace]\nmembers = []\n").unwrap();
    for name in [
        "PRD.md",
        "ROADMAP.md",
        "README.md",
        "AGENTS.md",
        "ARCHITECTURE.md",
        "CONTRIBUTING.md",
    ] {
        if Some(name) != missing {
            let text = if name == "ROADMAP.md" {
                conforming_roadmap().to_string()
            } else {
                format!("# {name}\n\nbody\n")
            };
            std::fs::write(root.join(name), text).unwrap();
        }
    }
}

#[test]
fn aes606_reports_a_missing_required_root_doc() {
    let tmp = tempfile::tempdir().unwrap();
    write_root(tmp.path(), Some("PRD.md"));
    let documents = shared_doc_rules::utility_doc_collector::collect_documents(tmp.path());
    let members = shared_doc_rules::utility_doc_collector::collect_members(tmp.path());
    let context = DocAuditContext::new(
        tmp.path(),
        documents,
        None,
        members,
        None,
        shared_doc_rules::taxonomy_doc_audit_context_vo::StrictModeOverride::UseConfigFile,
    );
    let findings = RequiredDocChecker::new().audit_required_docs(&context);
    assert!(
        findings
            .iter()
            .any(|f| f.code == "AES606" && f.doc == "PRD.md"),
        "a missing required root doc must fire AES606 naming the document; got {findings:?}"
    );
}

#[test]
fn aes606_stays_silent_for_required_docs_when_the_root_is_present() {
    let tmp = tempfile::tempdir().unwrap();
    write_root(tmp.path(), None);
    let documents = shared_doc_rules::utility_doc_collector::collect_documents(tmp.path());
    let members = shared_doc_rules::utility_doc_collector::collect_members(tmp.path());
    let context = DocAuditContext::new(
        tmp.path(),
        documents,
        None,
        members,
        None,
        shared_doc_rules::taxonomy_doc_audit_context_vo::StrictModeOverride::UseConfigFile,
    );
    let findings = RequiredDocChecker::new().audit_required_docs(&context);
    assert!(
        findings.is_empty(),
        "every required root doc present must report nothing; got {findings:?}"
    );
}
