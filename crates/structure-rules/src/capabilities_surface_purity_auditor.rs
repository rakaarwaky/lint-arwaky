// PURPOSE: SurfacePurityAuditor — AES703: surface folder purity and docs

use shared_structure_rules::utility_structure_parsers;

use shared_structure_rules::utility_structure_parsers::{find_workspace_root, sorted};

use shared_structure_rules::contract_structure_protocol::IStructureSurfacePurityProtocol;
use shared_structure_rules::taxonomy_structure_rules_constant as consts;
use shared_structure_rules::taxonomy_structure_rules_request::{
    StructureFinding, StructureRequest,
};
use shared_structure_rules::taxonomy_structure_rules_response::StructureResponse;

// ─── Block 1: Struct Definition ────────────────────────────

/// AES703: checks that surface-dominated folders carry only surface files plus
/// permitted support files, and document themselves with DESIGN.md.
pub struct SurfacePurityAuditor {}

// ─── Block 2: Protocol Trait Implementation ────────────────

impl IStructureSurfacePurityProtocol for SurfacePurityAuditor {
    fn audit_surface(&self, request: StructureRequest) -> StructureResponse {
        let StructureRequest::AuditAll { root } = request;
        let ws_root = workspace_root(&root);
        let mut findings = Vec::new();

        // If root is a member sub-dir (like crates/cli-commands), check it directly
        let root_abs = std::fs::canonicalize(&root).unwrap_or_else(|_| root.clone());
        let ws_root_abs = std::fs::canonicalize(&ws_root).unwrap_or_else(|_| ws_root.clone());
        if root_abs != ws_root_abs
            && root_abs.starts_with(&ws_root_abs)
            && !root_abs.join("crates").is_dir()
            && !root_abs.join("modules").is_dir()
            && !root_abs.join("packages").is_dir()
        {
            check_member_folder(&root_abs, &ws_root, &mut findings);
        }

        for member in utility_structure_parsers::member_dirs(&ws_root) {
            // Check the member folder itself
            check_member_folder(&member, &ws_root, &mut findings);

            // Check subdirectories (feature folders)
            for folder in utility_structure_parsers::feature_dirs(&member) {
                if folder_name(&folder) == consts::KERNEL_DIR {
                    continue;
                }
                check_member_folder(&folder, &ws_root, &mut findings);
            }
        }

        StructureResponse::Findings {
            findings: sorted(findings),
        }
    }
}

/// Check a single folder candidate for AES703 violations.
fn check_member_folder(
    folder: &std::path::Path,
    ws_root: &std::path::Path,
    findings: &mut Vec<StructureFinding>,
) {
    let inventory = utility_structure_parsers::inventory(folder);
    if !inventory.is_surface_dominated() {
        return;
    }
    let folder_rel = match folder.strip_prefix(ws_root) {
        Ok(p) => p.to_string_lossy().replace('\\', "/"),
        Err(_) => folder.to_string_lossy().replace('\\', "/"),
    };
    check_surface_folder(folder, &folder_rel, ws_root, &inventory, findings);
}

// ─── Block 3: Constructors, Std Traits, Helpers ────────────

/// Resolve the workspace root by walking up from *root* until we find
/// a directory that contains at least one of the member directories.
fn workspace_root(root: &std::path::Path) -> std::path::PathBuf {
    find_workspace_root(root)
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
    inventory: &shared_structure_rules::taxonomy_structure_rules_vo::FolderInventory,
    findings: &mut Vec<StructureFinding>,
) {
    // Purity: no capabilities, agent, or utility files.
    for file in &inventory.files {
        let misplaced_kind = if file.stem.starts_with(consts::CAPABILITIES_PREFIX)
            || file.stem.starts_with(consts::AGENT_PREFIX)
        {
            Some("a capability/agent file")
        } else if file.stem.starts_with(consts::UTILITY_PREFIX) {
            Some("a utility file")
        } else {
            None
        };
        let Some(kind) = misplaced_kind else {
            continue;
        };
        let destination = if kind == "a utility file" {
            "move it to the shared folder"
        } else {
            "move it to a feature folder"
        };
        findings.push(StructureFinding::new(
            consts::RULE_CODE_SURFACE_PURITY,
            consts::SURFACE_PURITY_VIOLATION_MISPLACED_FILES,
            file.rel(ws_root),
            format!(
                "surface folder '{folder_rel}' holds '{kind}' ({})",
                file.name,
            ),
        )
        .with_reason(
            "A surface folder carries surface files only. A layer file there would make the folder's own layer unclear.",
            destination.to_string(),
        ));
    }

    // Docs: DESIGN.md + BACKLOG.md required.
    let names = consts::SURFACE_DOC_PAIR;
    let missing = missing_docs(folder, names);
    if !missing.is_empty() {
        findings.push(StructureFinding::new(
            consts::RULE_CODE_SURFACE_PURITY,
            if names.contains(&consts::DESIGN_DOC) && missing.contains(&consts::DESIGN_DOC) {
                consts::SURFACE_PURITY_VIOLATION_NO_DESIGN
            } else {
                consts::SURFACE_PURITY_VIOLATION_NO_BACKLOG
            },
            folder_rel.to_string(),
            format!(
                "surface folder '{folder_rel}' is missing {}",
                missing.join(" and ")
            ),
        )
        .with_reason(
            format!(
                "A surface folder carries {} recording its kind, entry points, and visible states.",
                names.join(" and ")
            ),
            format!("Add {} to the surface folder.", missing.join(" and ")),
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
