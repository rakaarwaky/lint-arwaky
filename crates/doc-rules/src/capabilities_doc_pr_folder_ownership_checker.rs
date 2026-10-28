// PURPOSE: PrFolderOwnershipChecker — AES607: every PR-ID declared in the root
// PRD's `Project Requirements` must own exactly one top-level module folder
// under `crates|modules|packages` whose name matches the PR-ID's project
// identifier (the `PROJECT` segment of `PR-<PROJECT>-NNN`).
//
// Capabilities perform zero I/O; module-folder discovery stays in the shared
// utility collector (`collect_pr_owned_module_folders`). The check is
// root-gated: it fires only when the audit root is a real workspace root.
// Findings are stamped against the PRD document (the document that declares
// the PR-IDs), not against the folder.
use shared_doc_rules::contract_doc_protocol::IPrFolderOwnershipProtocol;
use shared_doc_rules::taxonomy_doc_audit_context_vo::DocAuditContext;
use shared_doc_rules::taxonomy_doc_rules_constant as consts;
use shared_doc_rules::taxonomy_doc_rules_request::{DocFinding, DocSource};
use shared_doc_rules::utility_protocol_counter::{collect_pr_id_set, pr_id_heading_re};
use std::path::Path;

// ─── Block 1: Struct Definition ──────────────────────────────────────────

/// AES607 auditor: one finding per PR-ID whose project identifier owns no
/// matching top-level module folder under `crates|modules|packages`.
pub struct PrFolderOwnershipChecker;

// ─── Block 2: Protocol Trait Implementation ──────────────────────────────

impl IPrFolderOwnershipProtocol for PrFolderOwnershipChecker {
    /// Report every PR-ID declared in the root PRD whose project identifier
    /// owns no matching top-level module folder.
    fn audit_pr_folder_ownership(&self, context: &DocAuditContext) -> Vec<DocFinding> {
        // Root-gate: only fire when the audit root is a real workspace root.
        if !context.is_workspace_root() {
            return Vec::new();
        }

        // The PRD is the only document that declares PR-IDs in `Project
        // Requirements`. Find it among the collected documents.
        let Some(prd) = context
            .documents()
            .iter()
            .find(|d| context.document_name(d) == consts::PRD_DOC)
        else {
            return Vec::new();
        };

        // No PR-IDs declared → nothing to check, stay silent.
        let Some(pr_ids) = collect_pr_id_set(&prd.text) else {
            return Vec::new();
        };

        // The set of top-level module folders owned by any declared PR-ID's
        // project identifier. None layouts → no layouts → silent.
        let Some(owned) = shared_doc_rules::utility_doc_collector::collect_pr_owned_module_folders(context.root())
        else {
            return Vec::new();
        };

        let mut findings = Vec::new();

        for id in &pr_ids {
            // PR-<PROJECT>-NNN → PROJECT segment.
            let Some(project) = project_identifier(id) else {
                continue;
            };
            if !owned.contains(project) {
                let mut line = 1;
                if let Some(re) = pr_id_heading_re() {
                    if let Some(caps) = re.captures(&prd.text) {
                        if let Some(m) = caps.name("id") {
                            if m.as_str() == id {
                                line = prd.text[..m.start()].lines().count().max(1);
                            }
                        }
                    }
                }
                findings.push(
                    DocFinding::new_with_line(
                        prd.text.lines().count().max(1).to_string(),
                        line,
                        consts::RULE_CODE_PR_FOLDER_OWNERSHIP,
                        consts::PR_FOLDER_NOT_OWNED_VIOLATION,
                        format!(
                            "PR-ID `{id}` declared in the PRD's `Project Requirements` has no module folder owned by `{project}` under `crates|modules|packages`"
                        ),
                    )
                    .with_reason(
                        "Every PRD `PR-<PROJECT>-NNN` item must own exactly one top-level module folder whose name matches `PROJECT`, so each requirement has a concrete place in the tree.",
                        format!("Create a module folder `crates/{project}` (or `modules/{project}` / `packages/{project}`) owned by `{id}`, or remove the `{id}` item from the PRD's `Project Requirements`.",
                    ),
                );
            }
        }

        findings
    }
}

// ─── Block 3: Constructors, Std Traits, Helpers ──────────────────────────

impl Default for PrFolderOwnershipChecker {
    fn default() -> Self {
        Self::new()
    }
}

impl PrFolderOwnershipChecker {
    pub fn new() -> Self {
        Self
    }
}

/// Extract the project identifier segment from `PR-<PROJECT>-NNN`, e.g.
/// `PR-LA-001` → `LA`. Returns `None` for any other shape.
fn project_identifier(id: &str) -> Option<String> {
    id.strip_prefix("PR-")?
        .split_once('-')
        .map(|(project, _)| project.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Build a context with one document named PRD.md carrying *text*, under a
    /// real workspace root (Cargo.toml present).
    fn prd_context(root: &Path, text: &str) -> DocAuditContext {
        let text_owned = text.to_string();
        let documents = vec![DocSource {
            path: PathBuf::from(format!("{}/PRD.md", root.display())),
            text: text_owned.clone(),
        }];
        DocAuditContext::new(
            root,
            documents,
            Some(text_owned.clone()),
            Vec::new(),
            None,
            shared_doc_rules::taxonomy_doc_audit_context_vo::StrictModeOverride::UseConfigFile,
        )
    }

    /// Create a tempdir with Cargo.toml so is_workspace_root returns true.
    fn temp_workspace() -> tempfile::TempDir {
        let tmp = tempfile::tempdir().unwrap();
        std::fs::write(tmp.path().join("Cargo.toml"), "[workspace]\nmembers = []\n").unwrap();
        tmp
    }

    /// A PRD declaring `PR-LA-001` whose `crates/LA/` folder exists → clean.
    #[test]
    fn pr_folder_ownership_matching_folder_is_clean() {
        let tmp = temp_workspace();
        let crates_la = tmp.path().join("crates/LA");
        std::fs::create_dir_all(&crates_la).unwrap();
        let text = "# PRD\n\n## Project Requirements\n\n### PR-LA-001: Multi-language scanning\n\n- Description: x\n";
        let context = prd_context(tmp.path(), &text);
        let findings = PrFolderOwnershipChecker::new().audit_pr_folder_ownership(&context);
        assert!(
            findings.is_empty(),
            "a declared PR-ID with a matching module folder must be clean; got {findings:?}"
        );
    }

    /// A PRD declaring `PR-FOO-001` with no `crates/FOO/` folder → one finding.
    #[test]
    fn pr_folder_ownership_missing_folder_fires_one_finding() {
        let tmp = temp_workspace();
        let text = "# PRD\n\n## Project Requirements\n\n### PR-FOO-001: Some requirement\n\n- Description: x\n";
        let context = prd_context(tmp.path(), &text);
        let findings = PrFolderOwnershipChecker::new().audit_pr_folder_ownership(&context);
        assert_eq!(
            findings.len(),
            1,
            "a missing module folder must fire exactly one finding; got {findings:?}"
        );
        assert_eq!(findings[0].code, consts::RULE_CODE_PR_FOLDER_OWNERSHIP);
        assert_eq!(findings[0].violation_type, consts::PR_FOLDER_NOT_OWNED_VIOLATION);
    }

    /// A root without Cargo.toml stays silent (root-gated).
    #[test]
    fn pr_folder_ownership_non_root_is_silent() {
        let tmp = tempfile::tempdir().unwrap();
        let text = "# PRD\n\n## Project Requirements\n\n### PR-FOO-001: Some requirement\n\n- Description: x\n";
        let context = prd_context(tmp.path(), &text);
        let findings = PrFolderOwnershipChecker::new().audit_pr_folder_ownership(&context);
        assert!(
            findings.is_empty(),
            "a root without a workspace marker must stay silent; got {findings:?}"
        );
    }

    #[test]
    fn project_identifier_parses_pr_id() {
        assert_eq!(project_identifier("PR-LA-001"), Some("LA".to_string()));
        assert_eq!(project_identifier("PR-FOO-001"), Some("FOO".to_string()));
        assert_eq!(project_identifier("NOT-A-PR-ID"), None);
    }

    use std::path::PathBuf;
}
