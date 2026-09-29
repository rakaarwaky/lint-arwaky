// PURPOSE: SharedPurityAuditor — AES701: shared/kernel folder purity and docs

use shared::structure_rules::utility_structure_parsers::{self, sorted};

use shared::structure_rules::contract_structure_protocol::IStructureSharedPurityProtocol;
use shared::structure_rules::taxonomy_structure_rules_constant as consts;
use shared::structure_rules::taxonomy_structure_rules_request::{
    StructureFinding, StructureRequest,
};
use shared::structure_rules::taxonomy_structure_rules_response::StructureResponse;

/// AES701: checks that shared folders contain only taxonomy, utility, and contract
/// files, and that kernel folders carry no doc pair.
pub struct SharedPurityAuditor {}

impl IStructureSharedPurityProtocol for SharedPurityAuditor {
    fn audit_shared(&self, request: StructureRequest) -> StructureResponse {
        let StructureRequest::AuditAll { root } = request;
        let ws_root = workspace_root(&root);
        let mut findings = Vec::new();

        for member in utility_structure_parsers::member_dirs(&ws_root) {
            let shared = member.join("shared");
            if !shared.is_dir() {
                continue;
            }
            let inventory = utility_structure_parsers::inventory(&shared);
            let folder_rel = match shared.strip_prefix(&ws_root) {
                Ok(p) => p.to_string_lossy().replace('\\', "/"),
                Err(_) => shared.to_string_lossy().replace('\\', "/"),
            };
            check_shared_purity(&folder_rel, &ws_root, &inventory, &mut findings);
            check_shared_has_docs(&shared, &folder_rel, &mut findings);
        }

        StructureResponse::Findings {
            findings: sorted(findings),
        }
    }
}

// ─── Private helpers ───────────────────────────────────────────────────────────

/// Resolve the workspace root: the nearest ancestor that contains one of the
/// member directories. If *root* itself holds them, it is the root; if it is
/// a member directory, its parent is.
fn workspace_root(root: &std::path::Path) -> std::path::PathBuf {
    if root.join("crates").is_dir()
        || root.join("modules").is_dir()
        || root.join("packages").is_dir()
    {
        return root.to_path_buf();
    }
    root.parent()
        .map(|p| p.to_path_buf())
        .unwrap_or_else(|| root.to_path_buf())
}

/// AES701 — a shared folder holds taxonomy, utility, and contract files. A
/// capabilities, agent, or surface file there belongs in a feature folder.
fn check_shared_purity(
    folder_rel: &str,
    ws_root: &std::path::Path,
    inventory: &shared::structure_rules::taxonomy_structure_rules_vo::FolderInventory,
    findings: &mut Vec<StructureFinding>,
) {
    for file in &inventory.files {
        let Some(prefix) = consts::SHARED_FORBIDDEN_PREFIXES
            .iter()
            .find(|p| file.stem.starts_with(**p))
        else {
            continue;
        };
        findings.push(StructureFinding::new(
            consts::RULE_CODE_SHARED_PURITY,
            consts::SHARED_PURITY_VIOLATION_FORBIDDEN_FILES,
            file.rel(ws_root),
            format!(
                "'{}' sits in the shared folder '{}' but is a {} file; shared holds only taxonomy, utility, and contract files — move it to a feature folder",
                file.name,
                folder_rel,
                layer_label(prefix),
            ),
        ));
    }
}

/// AES701 kernel check: a shared/kernel folder must not carry a doc pair at all.
fn check_shared_has_docs(
    folder: &std::path::Path,
    rel: &str,
    findings: &mut Vec<StructureFinding>,
) {
    let has_frd = folder.join(consts::FRD_DOC).is_file();
    let has_backlog = folder.join(consts::BACKLOG_DOC).is_file();
    if !has_frd && !has_backlog {
        return;
    }
    findings.push(StructureFinding::new(
        consts::RULE_CODE_SHARED_PURITY,
        consts::SHARED_PURITY_VIOLATION_HAS_DOCS,
        rel,
        format!("kernel folder '{rel}' must not carry a doc pair; move it to a feature folder"),
    ));
}

/// Strip the trailing underscore from a layer prefix for prose.
fn layer_label(prefix: &str) -> String {
    prefix.trim_end_matches('_').to_string()
}
