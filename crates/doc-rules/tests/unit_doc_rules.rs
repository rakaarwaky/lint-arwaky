// Real tests for the doc-rules capabilities (AES601–AES610).
//
// These replace the placeholder `assert!(true)` that shipped with the crate.
// They exercise the audit pipeline end to end: a capability is driven over an
// in-memory `DocAuditContext`, and the emitted `DocFinding`s are inspected for
// the properties the UX plan (issue #1397) pins — per-document stamping
// (UX-S1-001), the audit manifest (UX-S1-002 / UX-S5-004), severity per
// violation kind (UX-S3-001), and the shared heading-defect copy (UX-S2-004).

use doc_rules_lint_arwaky::capabilities_doc_agents::AgentsCapability;
use doc_rules_lint_arwaky::root_doc_rules_container::RootDocRulesContainer;
use shared_common::taxonomy_severity_vo::Severity;
use shared_doc_rules::contract_doc_protocol::IAgentsDocProtocol;
use shared_doc_rules::taxonomy_doc_audit_context_vo::{DocAuditContext, StrictModeOverride};
use shared_doc_rules::taxonomy_doc_rules_constant as consts;
use shared_doc_rules::taxonomy_doc_rules_request::{DocRequest, DocSource};
use shared_doc_rules::taxonomy_doc_rules_response::DocResponse;
use std::path::PathBuf;

/// Build an audit context over in-memory documents keyed by path relative to
/// *root*. No filesystem access: the capabilities read `DocSource.text`
/// directly, so a test can stage any document shape without touching disk.
fn context(root: &str, docs: Vec<(&str, &str)>) -> DocAuditContext {
    let documents: Vec<DocSource> = docs
        .into_iter()
        .map(|(rel, text)| DocSource {
            path: PathBuf::from(root).join(rel),
            text: text.to_string(),
        })
        .collect();
    DocAuditContext::new(
        std::path::Path::new(root),
        documents,
        None,
        Vec::new(),
        None,
        StrictModeOverride::UseConfigFile,
    )
}

/// UX-S1-001 regression: findings must be stamped with the document that
/// produced them, not with the last document in the loop.
///
/// Two `AGENTS.md` files with the same name live in different folders. The
/// first carries an off-template H2 (`h2_unexpected`); the second lacks the
/// YAML frontmatter (`frontmatter_missing`). Before the fix the shared
/// `findings` vec was re-stamped with every iteration, so the first
/// document's finding was overwritten with the second document's path.
#[test]
fn findings_are_stamped_with_their_own_document() {
    let doc_a =
        "---\ntitle: A\n---\n# AGENTS A\n\n## Agent Overview\n\nbody\n\n## Off Template\n\nstuff\n";
    let doc_b = "# AGENTS B\n\n## Agent Overview\n\nbody\n";
    let context = context(
        "/tmp/ux1397",
        vec![("a/AGENTS.md", doc_a), ("b/AGENTS.md", doc_b)],
    );
    let findings = AgentsCapability::new().audit_agents(&context);

    let unexpected: Vec<_> = findings
        .iter()
        .filter(|f| f.violation_type == consts::DOC_STRUCTURE_VIOLATION_H2_UNEXPECTED)
        .collect();
    assert!(
        !unexpected.is_empty(),
        "doc_a must produce an h2_unexpected finding"
    );
    for finding in &unexpected {
        assert_eq!(
            finding.doc, "a/AGENTS.md",
            "h2_unexpected from doc_a must carry a/AGENTS.md, got {}",
            finding.doc
        );
    }

    let frontmatter: Vec<_> = findings
        .iter()
        .filter(|f| f.violation_type == consts::DOC_STRUCTURE_VIOLATION_FRONTMATTER_MISSING)
        .collect();
    assert_eq!(
        frontmatter.len(),
        1,
        "doc_b must produce exactly one frontmatter_missing finding"
    );
    assert_eq!(
        frontmatter[0].doc, "b/AGENTS.md",
        "frontmatter_missing from doc_b must carry b/AGENTS.md"
    );
}

/// UX-S1-002 / UX-S5-004: the response reports which documents were audited
/// and which required root documents were absent.
#[test]
fn response_reports_audited_and_absent_documents() {
    let dir = tempfile::tempdir().expect("tempdir");
    let root = dir.path();
    // Stage one recognised document so it is collected and audited.
    std::fs::write(root.join("AGENTS.md"), "---\ntitle: x\n---\n# AGENTS X\n")
        .expect("write AGENTS.md");

    let aggregate = RootDocRulesContainer::orchestrator();
    let response = aggregate.execute(DocRequest::audit_all(
        root,
        None,
        StrictModeOverride::UseConfigFile,
    ));

    let DocResponse::Findings {
        audited, absent, ..
    } = response;
    assert!(
        audited.iter().any(|path| path == "AGENTS.md"),
        "audited must include AGENTS.md, got {audited:?}"
    );
    // PRD.md is a required root document that was never written.
    assert!(
        absent.iter().any(|path| path == "PRD.md"),
        "absent must include PRD.md, got {absent:?}"
    );
}

/// UX-S3-001: template-drift violations are MEDIUM; architectural-policy
/// violations stay HIGH.
#[test]
fn severity_is_assigned_per_violation_kind() {
    use shared_doc_rules::taxonomy_doc_rules_violation_vo::severity_for_violation;
    assert_eq!(
        severity_for_violation(consts::DOC_STRUCTURE_VIOLATION_LINE_DRIFT),
        Severity::MEDIUM
    );
    assert_eq!(
        severity_for_violation(consts::DOC_STRUCTURE_VIOLATION_H1_COUNT),
        Severity::MEDIUM
    );
    assert_eq!(
        severity_for_violation(consts::SPEC_PURITY_VIOLATION_SOURCE_FILE_NAMED),
        Severity::HIGH
    );
    assert_eq!(
        severity_for_violation(consts::CROSSLINKS_VIOLATION_NO_BACKLOG_LINK),
        Severity::HIGH
    );
}

/// UX-S2-004: the shared heading-defect copy names the document under audit
/// and returns the matching violation type.
#[test]
fn heading_defect_messages_names_the_document() {
    use shared_doc_rules::utility_doc_common_checks::{HeadingDefect, heading_defect_messages};
    let (violation, message, _why, _fix) = heading_defect_messages(
        consts::AGENTS_DOC,
        &HeadingDefect::H2Missing {
            line: 0,
            missing: vec!["Commands".to_string()],
        },
    );
    assert_eq!(violation, consts::DOC_STRUCTURE_VIOLATION_H2_MISSING);
    assert!(
        message.contains(consts::AGENTS_DOC),
        "message must name the document, got {message}"
    );
    assert!(
        message.contains("Commands"),
        "message must name the missing section, got {message}"
    );
}
