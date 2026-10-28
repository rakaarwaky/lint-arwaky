// PURPOSE: unit tests for FR-DOC-010 (PrFolderOwnershipChecker) — AES607,
// the PR-ID ↔ module-folder 1:1 requirement.
//
// Each case builds the smallest workspace that decides the rule: a root that
// does (or does not) declare PR-IDs, and a `crates|modules|packages` layout
// that does (or does not) own them. The shipped checker is driven through the
// same protocol seam the orchestrator uses.
use doc_rules_lint_arwaky::capabilities_doc_pr_folder_ownership_checker::{
    PrFolderOwnershipChecker, project_identifier,
};
use shared_doc_rules::contract_doc_protocol::IPrFolderOwnershipProtocol;
use shared_doc_rules::taxonomy_doc_audit_context_vo::{DocAuditContext, StrictModeOverride};
use shared_doc_rules::taxonomy_doc_rules_request::DocSource;
use std::path::Path;

// ─── Context builders ────────────────────────────────────────────────────

/// An audit context whose only document is `PRD.md` carrying *text*, under
/// root *root*.
fn prd_context(root: &Path, text: &str) -> DocAuditContext {
    let text_owned = text.to_string();
    let documents = vec![DocSource {
        path: root.join("PRD.md"),
        text: text_owned.clone(),
    }];
    DocAuditContext::new(
        root,
        documents,
        Some(text_owned),
        Vec::new(),
        None,
        StrictModeOverride::UseConfigFile,
    )
}

/// A workspace root: tempdir with a `Cargo.toml` marker so `is_workspace_root`
/// is true.
fn temp_workspace() -> tempfile::TempDir {
    let tmp = tempfile::tempdir().unwrap();
    std::fs::write(tmp.path().join("Cargo.toml"), "[workspace]\nmembers = []\n").unwrap();
    tmp
}

/// Create *rel* (slash-separated) under *root* as a directory.
fn mkdir(root: &Path, rel: &str) {
    std::fs::create_dir_all(root.join(rel)).unwrap();
}

// ─── FR-DOC-010 — PrFolderOwnershipChecker ───────────────────────────────

/// The project-identifier segment (`PR-LA-001` → `LA`) is what keys folder
/// ownership; any other shape is not a PR-ID.
#[test]
fn project_identifier_parses_pr_id() {
    assert_eq!(project_identifier("PR-LA-001"), Some("LA".to_string()));
    assert_eq!(project_identifier("PR-FOO-001"), Some("FOO".to_string()));
    assert_eq!(project_identifier("NOT-A-PR-ID"), None);
}

/// Layout owns a folder literally named after the PR-ID's project → clean.
#[test]
fn pr_folder_ownership_matching_folder_is_clean() {
    let tmp = temp_workspace();
    mkdir(tmp.path(), "crates/LA");
    let text = "# PRD\n\n## Project Requirements\n\n### PR-LA-001: Multi-language scanning\n\n- Description: x\n";
    let context = prd_context(tmp.path(), text);
    let findings = PrFolderOwnershipChecker::new().audit_pr_folder_ownership(&context);
    assert!(
        findings.is_empty(),
        "a PR-ID with its matching module folder must be clean; got {findings:?}"
    );
}

/// A layout root exists but carries no folder for the declared PR-ID → one
/// finding, stamped on the PRD, with the AES607 code and violation type.
#[test]
fn pr_folder_ownership_missing_folder_fires_one_finding() {
    let tmp = temp_workspace();
    mkdir(tmp.path(), "crates"); // layout root present, no `crates/FOO`
    let text = "# PRD\n\n## Project Requirements\n\n### PR-FOO-001: Some requirement\n\n- Description: x\n";
    let context = prd_context(tmp.path(), text);
    let findings = PrFolderOwnershipChecker::new().audit_pr_folder_ownership(&context);
    assert_eq!(
        findings.len(),
        1,
        "a PR-ID with no owned module folder must fire exactly one finding; got {findings:?}"
    );
    let finding = &findings[0];
    assert_eq!(finding.code, "AES607");
    assert_eq!(finding.violation_type, "pr_folder_not_owned");
    assert!(
        finding.doc.ends_with("PRD.md"),
        "finding must stamp the PRD document; got {finding:?}"
    );
    // Heading line: `# PRD` (1), blank (2), `## Project Requirements` (3),
    // blank (4), `### PR-FOO-001: ...` (5).
    assert_eq!(
        finding.line, 5,
        "finding must anchor the PR-ID heading; got {finding:?}"
    );
    assert!(
        finding.message.contains("PR-FOO-001"),
        "message must name the PR-ID; got {finding:?}"
    );
    assert!(
        !finding.why.is_empty(),
        "finding must carry why; got {finding:?}"
    );
    assert!(
        !finding.fix.is_empty(),
        "finding must carry fix; got {finding:?}"
    );
}

/// Multi-identifier PRD: the identifier with a matching folder stays clean,
/// the identifier without one fires — exactly one finding total.
#[test]
fn pr_folder_ownership_multi_project_membership() {
    let tmp = temp_workspace();
    mkdir(tmp.path(), "crates/BAR"); // `BAR` owned, `FOO` not
    let text = "# PRD\n\n## Project Requirements\n\n### PR-FOO-001: Missing owner\n\n### PR-BAR-002: Owned folder\n\n- Description: x\n";
    let context = prd_context(tmp.path(), text);
    let findings = PrFolderOwnershipChecker::new().audit_pr_folder_ownership(&context);
    assert_eq!(
        findings.len(),
        1,
        "only the identifier without a folder must fire; got {findings:?}"
    );
    assert!(
        findings[0].message.contains("PR-FOO-001"),
        "finding must name the unowned PR-ID; got {:?}",
        findings[0]
    );
}

/// The lint-arwaky shape: every PR-ID shares one project identifier (`LA`)
/// and the each folder under `crates/` is owned by that project → clean,
/// even though no folder is literally named `LA`.
#[test]
fn pr_folder_ownership_single_project_owns_whole_nonempty_layout() {
    let tmp = temp_workspace();
    mkdir(tmp.path(), "crates/auto-fix");
    mkdir(tmp.path(), "crates/cli-commands");
    mkdir(tmp.path(), "modules/logging");
    let text = "# PRD\n\n## Project Requirements\n\n### PR-LA-001: First requirement\n\n### PR-LA-015: Second requirement\n\n- Description: x\n";
    let context = prd_context(tmp.path(), text);
    let findings = PrFolderOwnershipChecker::new().audit_pr_folder_ownership(&context);
    assert!(
        findings.is_empty(),
        "one project identifier owning the whole layout must be clean; got {findings:?}"
    );
}

/// The lint-arwaky shape minus the folders: single identifier, layout root
/// exists but is empty → every PR-ID fires (the project owns nothing).
#[test]
fn pr_folder_ownership_single_project_empty_layout_fires_each() {
    let tmp = temp_workspace();
    mkdir(tmp.path(), "crates"); // layout root present, no feature folders
    let text = "# PRD\n\n## Project Requirements\n\n### PR-LA-001: First requirement\n\n### PR-LA-002: Second requirement\n\n- Description: x\n";
    let context = prd_context(tmp.path(), text);
    let findings = PrFolderOwnershipChecker::new().audit_pr_folder_ownership(&context);
    assert_eq!(
        findings.len(),
        2,
        "an empty layout must fire one finding per PR-ID; got {findings:?}"
    );
}

/// A root without a `Cargo.toml` marker is not a workspace root → silent.
#[test]
fn pr_folder_ownership_non_root_is_silent() {
    let tmp = tempfile::tempdir().unwrap();
    let text = "# PRD\n\n## Project Requirements\n\n### PR-FOO-001: Some requirement\n\n- Description: x\n";
    let context = prd_context(tmp.path(), text);
    let findings = PrFolderOwnershipChecker::new().audit_pr_folder_ownership(&context);
    assert!(
        findings.is_empty(),
        "a root without a workspace marker must stay silent; got {findings:?}"
    );
}

/// A PRD declaring no PR-IDs → nothing to own, silent.
#[test]
fn pr_folder_ownership_no_pr_ids_is_silent() {
    let tmp = temp_workspace();
    mkdir(tmp.path(), "crates");
    let text = "# PRD\n\n## Project Requirements\n\n- No identifiers here\n";
    let context = prd_context(tmp.path(), text);
    let findings = PrFolderOwnershipChecker::new().audit_pr_folder_ownership(&context);
    assert!(
        findings.is_empty(),
        "a PRD with no PR-Ids must stay silent; got {findings:?}"
    );
}

/// No `crates|modules|packages` layout root at all → silent (nothing to
/// compare against).
#[test]
fn pr_folder_ownership_no_layout_root_is_silent() {
    let tmp = temp_workspace();
    let text = "# PRD\n\n## Project Requirements\n\n### PR-FOO-001: Some requirement\n\n- Description: x\n";
    let context = prd_context(tmp.path(), text);
    let findings = PrFolderOwnershipChecker::new().audit_pr_folder_ownership(&context);
    assert!(
        findings.is_empty(),
        "a workspace with no feature layout must stay silent; got {findings:?}"
    );
}
