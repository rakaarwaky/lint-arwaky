// PURPOSE: PrFolderOwnershipChecker — AES607: every PR-ID declared in the root
// PRD's `Project Requirements` must be satisfied by the module-folder layout
// under `crates|modules|packages`. A PR-ID `PR-<PROJECT>-NNN` is satisfied when
// a top-level folder named `PROJECT` exists, or — for a PRD whose PR-IDs all
// share one project identifier — when the layout root exists and is non-empty
// (that project owns the whole folder set). Capabilities perform zero I/O;
// module-folder discovery stays in the shared utility collector
// (`collect_pr_owned_module_folders`). The check is root-gated: it fires only
// when the audit root is a real workspace root. Findings are stamped against
// the PRD document (the document that declares the PR-IDs), not the folder.
use shared_doc_rules::contract_doc_protocol::IPrFolderOwnershipProtocol;
use shared_doc_rules::taxonomy_doc_audit_context_vo::DocAuditContext;
use shared_doc_rules::taxonomy_doc_rules_constant as consts;
use shared_doc_rules::taxonomy_doc_rules_request::DocFinding;
use shared_doc_rules::utility_protocol_counter::{collect_pr_id_set, pr_id_heading_re};
use std::collections::BTreeSet;

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

        // The root PRD is the only document that declares PR-IDs in `Project
        // Requirements`. Find it among the collected documents — root-level,
        // so a member's PRD.md never answers for the workspace.
        let Some(prd) = context
            .documents()
            .iter()
            .find(|d| context.document_name(d) == consts::PRD_DOC && context.is_root_document(d))
        else {
            return Vec::new();
        };

        // No PR-IDs declared → nothing to check, stay silent.
        let Some(pr_ids) = collect_pr_id_set(&prd.text) else {
            return Vec::new();
        };

        // The set of top-level module folders owned by any declared PR-ID's
        // project identifier. None layouts → no layouts → silent.
        let Some(owned) = shared_doc_rules::utility_doc_collector::collect_pr_owned_module_folders(
            context.root(),
        ) else {
            return Vec::new();
        };

        // Map each PR-ID to its heading line in the PRD, so each finding is
        // anchored to its own declaration line.
        let id_to_line: Vec<(String, usize)> = pr_id_heading_lines(&prd.text);

        // A PRD whose PR-IDs all share one project identifier (e.g. lint-arwaky's
        // `PR-LA-001..015` → `LA`) lets that project own the whole non-empty
        // layout: every module folder under `crates|modules|packages` counts as
        // owned, so the identifier set is satisfied without a folder literally
        // named after it. With more than one identifier in play, each project
        // must own a matching folder name.
        let project_keys: BTreeSet<String> = pr_ids
            .iter()
            .filter_map(|id| project_identifier(id))
            .collect();
        let layout_owns_single_project = project_keys.len() == 1 && !owned.is_empty();

        let mut findings = Vec::new();

        for id in &pr_ids {
            // PR-<PROJECT>-NNN → PROJECT segment.
            let Some(project) = project_identifier(id) else {
                continue;
            };
            if layout_owns_single_project || owned.contains(&project) {
                continue;
            }
            let line = id_to_line
                .iter()
                .find_map(|(declared_id, line)| (declared_id == id).then_some(*line))
                .unwrap_or(1);
            findings.push(
                DocFinding::new_with_line(
                    context.relative_path(prd),
                    line,
                    consts::RULE_CODE_PR_FOLDER_OWNERSHIP,
                    consts::PR_FOLDER_NOT_OWNED_VIOLATION,
                    format!(
                        "PR-ID `{id}` declared in the PRD's `Project Requirements` has no module folder owned by `{project}` under `crates|modules|packages`",
                        id = id,
                        project = project,
                    ),
                )
                .with_reason(
                    "Every PRD `PR-<PROJECT>-NNN` item must own exactly one top-level module folder whose name matches `PROJECT`, so each requirement has a concrete place in the tree.",
                    format!(
                        "Create a module folder `crates/{project}` (or `modules/{project}` / `packages/{project}`) owned by `{id}`, or remove the `{id}` item from the PRD's `Project Requirements`.",
                        project = project,
                        id = id,
                    ),
                ),
            );
        }

        findings
    }
}

/// `(1-based heading line, id)` for every PR-ID heading in *text*.
fn pr_id_heading_lines(text: &str) -> Vec<(String, usize)> {
    let Some(re) = pr_id_heading_re() else {
        return Vec::new();
    };
    re.captures_iter(text)
        .filter_map(|caps| {
            let id = caps.name("id")?.as_str().to_string();
            let start = caps.get(0)?.start();
            // 1-based line = newlines strictly before the heading + 1
            // (`lines().count()` drops a trailing newline and undercounts).
            let line = text[..start].matches('\n').count() + 1;
            Some((id, line))
        })
        .collect()
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
pub fn project_identifier(id: &str) -> Option<String> {
    id.strip_prefix("PR-")?
        .split_once('-')
        .map(|(project, _)| project.to_string())
}
