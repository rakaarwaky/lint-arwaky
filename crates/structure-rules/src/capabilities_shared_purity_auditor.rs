// PURPOSE: SharedPurityAuditor — AES701: shared/kernel folder purity and docs

use shared_structure_rules::utility_structure_parsers;

use shared_structure_rules::utility_structure_parsers::{find_workspace_root, sorted};

use shared_structure_rules::contract_structure_protocol::IStructureSharedPurityProtocol;
use shared_structure_rules::taxonomy_structure_rules_constant as consts;
use shared_structure_rules::taxonomy_structure_rules_request::{
    StructureFinding, StructureRequest,
};
use shared_structure_rules::taxonomy_structure_rules_response::StructureResponse;

// ─── Block 1: Struct Definition ────────────────────────────

/// AES701: checks that shared folders contain only taxonomy, utility, and contract
/// files, and that kernel folders carry no doc pair.
pub struct SharedPurityAuditor {}

// ─── Block 2: Protocol Trait Implementation ────────────────

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

// ─── Block 3: Constructors, Std Traits, Helpers ────────────

/// Resolve the workspace root by walking up from *root* until we find
/// a directory that contains at least one of the member directories.
fn workspace_root(root: &std::path::Path) -> std::path::PathBuf {
    find_workspace_root(root)
}

/// AES701 — a shared folder holds taxonomy, utility, and contract files. A
/// capabilities, agent, or surface file there belongs in a feature folder.
fn check_shared_purity(
    folder_rel: &str,
    ws_root: &std::path::Path,
    inventory: &shared_structure_rules::taxonomy_structure_rules_vo::FolderInventory,
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
                "'{}' sits in the shared folder '{}' but is a {} file",
                file.name,
                folder_rel,
                layer_label(prefix),
            ),
        )
        .with_reason(
            format!(
                "Shared holds only taxonomy, utility, and contract files. A {} file there puts feature logic in the kernel every member depends on.",
                layer_label(prefix)
            ),
            "Move it to the feature folder that owns its layer.",
        ));
    }
}

/// AES701 kernel check: a shared/kernel folder must carry DATA.md + BACKLOG.md.
fn check_shared_has_docs(
    folder: &std::path::Path,
    rel: &str,
    findings: &mut Vec<StructureFinding>,
) {
    let missing = missing_docs(folder, consts::SHARED_DOC_PAIR);
    if !missing.is_empty() {
        findings.push(StructureFinding::new(
            consts::RULE_CODE_SHARED_PURITY,
            consts::SHARED_PURITY_VIOLATION_NO_DOC_PAIR,
            rel.to_string(),
            format!("shared folder '{rel}' is missing {}", missing.join(" and ")),
        )
        .with_reason(
            format!(
                "A shared folder carries {} beside its source, so a reader of the kernel knows what it is for.",
                consts::SHARED_DOC_PAIR.join(" and ")
            ),
            format!("Add {} to the shared folder.", missing.join(" and ")),
        ));
    }
}

/// Strip the trailing underscore from a layer prefix for prose.
fn layer_label(prefix: &str) -> String {
    prefix.trim_end_matches('_').to_string()
}

/// Which of *names* do not sit directly in *folder*. A folder's documents sit
/// beside its source, so this reads one level rather than walking.
fn missing_docs<'a>(folder: &std::path::Path, names: &'a [&'a str]) -> Vec<&'a str> {
    let mut missing: Vec<&'a str> = names
        .iter()
        .copied()
        .filter(|name| !folder.join(name).is_file())
        .collect();
    missing.sort_unstable();
    missing
}
