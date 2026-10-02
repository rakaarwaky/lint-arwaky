// PURPOSE: e2e test for doc-rules — the full request lifecycle over a real
// workspace, from documents on disk through the aggregate to a named finding.
//
// The integration test proves each layout is reached; this proves one complete
// journey: build a workspace whose FRD drifts from its contract, run the real
// container over it, and read a finding that names the defect.
use doc_rules_lint_arwaky::root_doc_rules_container::RootDocRulesContainer;
use shared_doc_rules::taxonomy_doc_rules_request::DocRequest;
use shared_doc_rules::taxonomy_doc_rules_response::DocResponse;

use std::fs;
use std::path::Path;

/// Write *contents* as a file at *path*, creating parent dirs.
fn write(path: &Path, contents: &str) {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).expect("fixture parent dir must be creatable");
    }
    fs::write(path, contents).expect("fixture file must be writable");
}

/// Audit *root* through the public container and return the findings.
fn audit(root: &Path) -> Vec<(String, String, String)> {
    let aggregate = RootDocRulesContainer::orchestrator();
    match aggregate.execute(DocRequest::audit_all(root)) {
        DocResponse::Findings { findings } => findings
            .into_iter()
            .map(|f| {
                (
                    f.code.to_string(),
                    f.violation_type.to_string(),
                    f.doc.clone(),
                )
            })
            .collect(),
    }
}

/// A feature folder carrying both layers and its doc pair.
fn write_workspace(root: &Path) {
    let feature = root.join("crates/sample");
    write(
        &feature.join("src/capabilities_sample_checker.rs"),
        "pub struct SampleChecker;\n",
    );
    write(
        &feature.join("src/agent_sample_orchestrator.rs"),
        "pub struct SampleOrchestrator;\n",
    );
    write(
        &feature.join("BACKLOG.md"),
        "# Feature Backlog: Sample\n\nFRD: [FRD.md](FRD.md)\n",
    );
    write(&root.join("PRD.md"), "# PRD\n");
    write(&root.join("ROADMAP.md"), "# ROADMAP\n");
}

/// A requirement block carrying every field the FRD template mandates.
fn requirement(id: &str, title: &str) -> String {
    format!(
        "### {id}: {title}\n\n\
         - **Description**: The feature performs a responsibility.\n\
         - **Input**: A request value object.\n\
         - **Output**: A response value object.\n\
         - **Business Rules**: The capability is stateless.\n\
         - **Edge Cases**: An absent input yields a default.\n\
         - **Error Handling**: Failures surface as a reason-coded outcome.\n"
    )
}

/// An FRD carrying every mandated section, minus whichever one the case drops.
///
/// The section list is read from the AES605 contract rather than transcribed, so
/// this fixture cannot drift away from what the rule actually demands — a
/// hand-copied list would start failing the moment the template grew a section,
/// and the failure would look like a rule defect.
fn frd_without(dropped: &str) -> String {
    use shared_doc_rules::taxonomy_doc_rules_constant::{DOC_HEADING_CONTRACTS, FRD_DOC};
    let (_, required, _) = DOC_HEADING_CONTRACTS
        .iter()
        .find(|(name, _, _)| *name == FRD_DOC)
        .expect("FRD.md must have a heading contract");

    let bodies = section_bodies();
    let mut frd = String::from("# FRD — sample\n\n");
    for heading in *required {
        if *heading == dropped {
            continue;
        }
        frd.push_str(&format!("## {heading}\n\n"));
        frd.push_str(
            bodies
                .get(heading)
                .map(String::as_str)
                .unwrap_or("- none\n\n"),
        );
    }
    frd
}

/// The body each mandated section carries, keyed by its heading.
///
/// Only the sections whose *shape* a rule judges have a specific body; the rest
/// carry a single bullet, which satisfies the rules that only ask for content.
fn section_bodies() -> std::collections::HashMap<&'static str, String> {
    let mut bodies = std::collections::HashMap::new();
    bodies.insert(
        "Reference",
        String::from("- PRD: [PRD.md](../../PRD.md)\n- Backlog: [BACKLOG.md](BACKLOG.md)\n\n"),
    );
    bodies.insert(
        "System Overview",
        String::from("- delegates to a capability\n\n"),
    );
    bodies.insert(
        "Functional Requirements",
        format!("{}\n", requirement("FR-SAMPLE-001", "Do The Thing")),
    );
    bodies.insert(
        "API Contract",
        String::from(
            "### Protocol API\n\n\
             | Method | Input | Output | Error | Event | Description |\n\
             | --- | --- | --- | --- | --- | --- |\n\
             | execute | request | response | none | none | single entry |\n\n\
             ### Aggregate API\n\n\
             | Method | Input | Output | Error | Event | Description |\n\
             | --- | --- | --- | --- | --- | --- |\n\
             | execute | request | response | none | none | routes to capability |\n\n",
        ),
    );
    bodies.insert(
        "Integration Points",
        String::from(
            "| System | Direction | Purpose | Failure mode |\n\
             | --- | --- | --- | --- |\n\
             | filesystem | out | read source | missing path |\n\n",
        ),
    );
    bodies.insert(
        "Non-functional Requirements",
        String::from(
            "| Metric | Target | Measurement method |\n\
             | --- | --- | --- |\n\
             | audit time | under 1s | wall clock |\n\n",
        ),
    );
    bodies.insert(
        "Test Scenarios",
        String::from("- audits a clean workspace\n\n"),
    );
    bodies.insert("Assumptions & Constraints", String::from("- none\n\n"));
    bodies.insert(
        "Glossary",
        String::from("- **Finding**: a rule violation\n\n"),
    );
    bodies
}

/// The findings that belong to *doc*, so a case can ignore the documents it did
/// not mean to exercise.
fn for_document<'a>(
    findings: &'a [(String, String, String)],
    doc: &str,
) -> Vec<&'a (String, String, String)> {
    findings
        .iter()
        .filter(|(_, _, reported)| reported == doc)
        .collect()
}

const FRD_PATH: &str = "crates/sample/FRD.md";

#[test]
fn e2e_a_complete_frd_survives_the_whole_pipeline() {
    let tmp = tempfile::tempdir().expect("temp dir must be creatable");
    let root = tmp.path();
    write_workspace(root);
    write(&root.join(FRD_PATH), &frd_without(""));

    let findings = audit(root);
    let on_frd = for_document(&findings, FRD_PATH);
    assert!(
        on_frd.is_empty(),
        "a complete FRD must survive the audit end to end; got {on_frd:?}"
    );
}

#[test]
fn e2e_a_missing_mandated_section_surfaces_as_a_heading_finding_on_that_document() {
    // AES605 owns the presence of a mandated H2, so dropping one is reported
    // there rather than by the shape rules — a shape rule judges a section that
    // is present. Each case drops a different one to prove the rule names the
    // section the template lost.
    for section in ["API Contract", "Integration Points", "Glossary"] {
        let tmp = tempfile::tempdir().expect("temp dir must be creatable");
        let root = tmp.path();
        write_workspace(root);
        let frd = frd_without(section);
        write(&root.join(FRD_PATH), &frd);

        let findings = audit(root);
        assert!(
            for_document(&findings, FRD_PATH)
                .iter()
                .any(|(code, violation, _)| code == "AES605" && violation == "h2_missing"),
            "dropping '## {section}' must surface as AES605/h2_missing on the FRD; \
             got {:?}",
            for_document(&findings, FRD_PATH)
        );
    }
}

#[test]
fn e2e_a_malformed_section_body_surfaces_as_a_shape_finding_on_that_document() {
    // The section is present but its body is the wrong shape: a bullet list
    // where the template demands a table. This is AES602's question, and it is
    // the half AES605 cannot see.
    let tmp = tempfile::tempdir().expect("temp dir must be creatable");
    let root = tmp.path();
    write_workspace(root);
    let conforming = frd_without("");
    let table = "| System | Direction | Purpose | Failure mode |\n\
         | --- | --- | --- | --- |\n\
         | filesystem | out | read source | missing path |";
    let frd = conforming.replace(table, "- filesystem, outbound, reads source");
    assert_ne!(
        frd, conforming,
        "the table under test must actually differ; a no-op replace would make this \
         case assert the conforming document and pass for the wrong reason"
    );
    write(&root.join(FRD_PATH), &frd);

    let findings = audit(root);
    assert!(
        for_document(&findings, FRD_PATH)
            .iter()
            .any(|(code, violation, _)| code == "AES602" && violation == "integration_not_table"),
        "a bullet list under Integration Points must fire AES602/integration_not_table \
         on the FRD; got {:?}",
        for_document(&findings, FRD_PATH)
    );
}

#[test]
fn e2e_a_requirement_id_without_its_feature_prefix_is_rejected_by_the_fr_format_rule() {
    let tmp = tempfile::tempdir().expect("temp dir must be creatable");
    let root = tmp.path();
    write_workspace(root);
    // Same document, one requirement ID stripped of its `FR-<FEATURE>-` prefix.
    let frd = frd_without("").replace("### FR-SAMPLE-001:", "### FR-001:");
    write(&root.join(FRD_PATH), &frd);

    let findings = audit(root);
    assert!(
        for_document(&findings, FRD_PATH).iter().any(
            |(code, violation, _)| code == "AES601" && violation == "id_missing_feature_prefix"
        ),
        "an FR-NNN heading without its feature prefix must fire AES601 on the FRD; \
         got {:?}",
        for_document(&findings, FRD_PATH)
    );
}
