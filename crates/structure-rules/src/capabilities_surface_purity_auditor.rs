// PURPOSE: SurfacePurityAuditor — AES703: surface folder purity and docs

use shared::structure_rules::utility_structure_parsers::{self, sorted};

use shared::structure_rules::contract_structure_protocol::IStructureSurfacePurityProtocol;
use shared::structure_rules::taxonomy_structure_rules_constant as consts;
use shared::structure_rules::taxonomy_structure_rules_request::{
    StructureFinding, StructureRequest,
};
use shared::structure_rules::taxonomy_structure_rules_response::StructureResponse;

/// AES703: checks that surface-dominated folders carry only surface files plus
/// permitted support files, and document themselves with DESIGN.md.
pub struct SurfacePurityAuditor {}

impl IStructureSurfacePurityProtocol for SurfacePurityAuditor {
    fn audit_surface(&self, request: StructureRequest) -> StructureResponse {
        let StructureRequest::AuditAll { root } = request;
        let ws_root = workspace_root(&root);
        let mut findings = Vec::new();

        for member in utility_structure_parsers::member_dirs(&ws_root) {
            for folder in utility_structure_parsers::feature_dirs(&member) {
                if folder_name(&folder) == consts::KERNEL_DIR {
                    continue;
                }
                let inventory = utility_structure_parsers::inventory(&folder);
                if !inventory.is_surface_dominated() {
                    continue;
                }
                let folder_rel = match folder.strip_prefix(&ws_root) {
                    Ok(p) => p.to_string_lossy().replace('\\', "/"),
                    Err(_) => folder.to_string_lossy().replace('\\', "/"),
                };
                check_surface_folder(&folder, &folder_rel, &ws_root, &inventory, &mut findings);
            }
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

/// The folder's own name.
fn folder_name(folder: &std::path::Path) -> String {
    folder
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or_default()
        .to_string()
}

/// AES703 — a surface-dominated folder carries surfaces plus permitted support
/// files. Capabilities and agent files are misplaced. DESIGN.md is required.
fn check_surface_folder(
    folder: &std::path::Path,
    folder_rel: &str,
    ws_root: &std::path::Path,
    inventory: &shared::structure_rules::taxonomy_structure_rules_vo::FolderInventory,
    findings: &mut Vec<StructureFinding>,
) {
    // Purity: no capabilities or agent files.
    for file in &inventory.files {
        let misplaced = file.stem.starts_with(consts::CAPABILITIES_PREFIX)
            || file.stem.starts_with(consts::AGENT_PREFIX);
        if !misplaced {
            continue;
        }
        findings.push(StructureFinding::new(
            consts::RULE_CODE_SURFACE_PURITY,
            consts::SURFACE_PURITY_VIOLATION_MISPLACED_FILES,
            file.rel(ws_root),
            format!(
                "surface folder '{folder_rel}' holds '{}'; a surface folder carries surface files only — move it to a feature folder",
                file.name,
            ),
        ));
    }

    // Docs: DESIGN.md required.
    let names: [&str; 1] = [consts::SURFACE_DOC];
    let missing = missing_docs(folder, &names);
    if !missing.is_empty() {
        findings.push(StructureFinding::new(
            consts::RULE_CODE_SURFACE_PURITY,
            consts::SURFACE_PURITY_VIOLATION_NO_DESIGN,
            folder_rel.to_string(),
            format!(
                "surface folder '{folder_rel}' is missing {}; a surface folder carries a {} recording its kind, entry points, and visible states",
                missing.join(" and "),
                consts::SURFACE_DOC,
            ),
        ));
    }
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
