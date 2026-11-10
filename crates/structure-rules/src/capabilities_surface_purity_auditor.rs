// PURPOSE: SurfacePurityAuditor — AES703: surface folder purity and docs

use shared_structure_rules::utility_structure_parsers;

use shared_structure_rules::utility_structure_parsers::{find_workspace_root, rel_posix, sorted};

use shared_structure_rules::contract_structure_protocol::IStructureSurfacePurityProtocol;
use shared_structure_rules::taxonomy_structure_rules_constant as consts;
use shared_structure_rules::taxonomy_structure_rules_request::{
    StructureFinding, StructureRequest,
};
use shared_structure_rules::taxonomy_structure_rules_response::StructureResponse;

// ─── Block 1: Module Declaration ────────────────────────────

/// AES703: checks that surface-dominated folders carry only surface files plus
/// permitted support files, and document themselves with DESIGN.md + BACKLOG.md.
#[derive(Default)]
pub struct SurfacePurityAuditor {}

impl SurfacePurityAuditor {
    /// Stateless auditor: every audit call is self-contained.
    pub fn new() -> Self {
        Self {}
    }
}

// ─── Block 2: Module Contract ────────────────

impl IStructureSurfacePurityProtocol for SurfacePurityAuditor {
    fn audit_surface(&self, request: StructureRequest) -> StructureResponse {
        let StructureRequest::AuditAll { root } = request;
        let ws_root = workspace_root(&root);
        let mut findings = Vec::new();

        // When the target is a member dir, the walk is scoped to that
        // member; when it is the workspace root, all members are walked.
        let scoped = utility_structure_parsers::scoped_member_dirs(&root, &ws_root);
        for member in &scoped {
            // Check subdirectories (feature/surface folders). The member dir
            // itself is a container, not a surface folder.
            for folder in utility_structure_parsers::feature_dirs(member) {
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

// ─── Block 3: Module Helpers ────────────

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

/// Check a single folder candidate for AES703 violations.
fn check_member_folder(
    folder: &std::path::Path,
    ws_root: &std::path::Path,
    findings: &mut Vec<StructureFinding>,
) {
    let inventory = utility_structure_parsers::inventory(folder);
    // A surface folder is surface-dominated: surface files must outnumber all
    // other classified files. A single surface file beside capabilities does
    // not make the folder a surface folder — that stays a feature folder.
    if !inventory.is_surface_dominated() {
        return;
    }
    let folder_rel = rel_posix(folder, ws_root);
    check_surface_folder(folder, &folder_rel, ws_root, &inventory, findings);
}

/// AES703 — a surface-dominated folder carries surfaces plus permitted support
/// files. Capabilities and agent files are misplaced; foreign taxonomy,
/// contract, and logical files belong in shared/.
fn check_surface_folder(
    folder: &std::path::Path,
    folder_rel: &str,
    ws_root: &std::path::Path,
    inventory: &shared_structure_rules::taxonomy_structure_rules_vo::FolderInventory,
    findings: &mut Vec<StructureFinding>,
) {
    // Purity: no foreign-layer files. Barrels, entry wiring, packaging, and
    // utility support files are permitted in a surface folder.
    for file in &inventory.files {
        let stem = &file.stem;
        let (kind, destination) = if stem.starts_with(consts::CAPABILITIES_PREFIX)
            || stem.starts_with(consts::AGENT_PREFIX)
        {
            ("a capability/agent file", "move it to a feature folder")
        } else if stem.starts_with(consts::UTILITY_PREFIX) {
            ("a utility file", "move it to the shared folder")
        } else if stem.starts_with("taxonomy_")
            || stem.starts_with("contract_")
            || stem.starts_with("logical_")
        {
            ("a foreign-layer file", "move it to shared/")
        } else {
            continue;
        };
        if consts::SURFACE_ALLOWED_FILENAMES.contains(&file.name.as_str()) {
            continue;
        }
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
    for name in missing {
        let violation_type = if name == consts::DESIGN_DOC {
            consts::SURFACE_PURITY_VIOLATION_NO_DESIGN
        } else {
            consts::SURFACE_PURITY_VIOLATION_NO_BACKLOG
        };
        findings.push(StructureFinding::new(
            consts::RULE_CODE_SURFACE_PURITY,
            violation_type,
            folder_rel.to_string(),
            format!("surface folder '{folder_rel}' is missing '{name}'"),
        )
        .with_reason(
            format!(
                "A surface folder carries {} recording its kind, entry points, and visible states.",
                name
            ),
            format!("Add '{name}' to the surface folder."),
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
