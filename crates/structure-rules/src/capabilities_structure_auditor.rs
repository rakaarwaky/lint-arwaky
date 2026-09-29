// PURPOSE: StructureAuditor — the folder-layout invariant auditor behind all
// three structure-rules capability seams (AES701–AES703).
//
// Walks the workspace members under a root and audits each folder against
// AES701 (shared purity), AES702 (feature health and docs), and AES703
// (surface purity and docs). Each finding carries a machine-readable
// violation_type.
use std::path::Path;

use shared::structure_rules::FolderInventory;
use shared::structure_rules::contract_structure_protocol::{
    IStructureFeatureHealthProtocol, IStructureSharedPurityProtocol,
    IStructureSurfacePurityProtocol,
};
use shared::structure_rules::taxonomy_structure_rules_constant as consts;
use shared::structure_rules::taxonomy_structure_rules_request::{
    StructureFinding, StructureRequest,
};
use shared::structure_rules::taxonomy_structure_rules_response::StructureResponse;
use shared::structure_rules::utility_structure_parsers::{self, sorted};

/// The invariant auditor behind the three structure rule protocols.
pub struct StructureAuditor {}

// ─── AES701: Shared folder purity ──────────────────────────────────────────────

impl IStructureSharedPurityProtocol for StructureAuditor {
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

// ─── AES702: Feature folder health + docs ─────────────────────────────────────

impl IStructureFeatureHealthProtocol for StructureAuditor {
    fn audit_feature(&self, request: StructureRequest) -> StructureResponse {
        let StructureRequest::AuditAll { root } = request;
        let ws_root = workspace_root(&root);
        let mut findings = Vec::new();

        for member in utility_structure_parsers::member_dirs(&ws_root) {
            for folder in utility_structure_parsers::feature_dirs(&member) {
                if folder_name(&folder) == consts::KERNEL_DIR {
                    continue;
                }
                let inventory = utility_structure_parsers::inventory(&folder);
                let folder_rel = match folder.strip_prefix(&ws_root) {
                    Ok(p) => p.to_string_lossy().replace('\\', "/"),
                    Err(_) => folder.to_string_lossy().replace('\\', "/"),
                };
                if inventory.has_capabilities || inventory.has_orchestrator {
                    check_feature_folder(&folder, &folder_rel, &ws_root, &inventory, &mut findings);
                }
                // Reverse check: any non-shared folder with a doc pair must
                // have an orchestrator.
                check_reverse_doc_orchestrator(&folder, &folder_rel, &mut findings);
            }
        }

        StructureResponse::Findings {
            findings: sorted(findings),
        }
    }
}

// ─── AES703: Surface folder purity + docs ─────────────────────────────────────

impl IStructureSurfacePurityProtocol for StructureAuditor {
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
fn workspace_root(root: &Path) -> std::path::PathBuf {
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
    ws_root: &Path,
    inventory: &FolderInventory,
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
fn check_shared_has_docs(folder: &Path, rel: &str, findings: &mut Vec<StructureFinding>) {
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

/// AES702 — a feature folder carries capabilities and agents, has no foreign
/// layer files (utility, surface, taxonomy, contract), and documents itself
/// with FRD.md + BACKLOG.md.
fn check_feature_folder(
    folder: &Path,
    rel: &str,
    ws_root: &Path,
    inventory: &FolderInventory,
    findings: &mut Vec<StructureFinding>,
) {
    // Health: both sides required.
    match (inventory.has_capabilities, inventory.has_orchestrator) {
        (true, false) => findings.push(StructureFinding::new(
            consts::RULE_CODE_FEATURE_HEALTH,
            consts::FEATURE_HEALTH_VIOLATION_MISSING_AGENT,
            rel.to_string(),
            format!(
                "feature folder '{rel}' holds capabilities but no agent_*_orchestrator file; a feature folder needs at least one agent and one capability"
            ),
        )),
        (false, true) => findings.push(StructureFinding::new(
            consts::RULE_CODE_FEATURE_HEALTH,
            consts::FEATURE_HEALTH_VIOLATION_MISSING_CAPABILITY,
            rel.to_string(),
            format!(
                "feature folder '{rel}' holds an agent orchestrator but no capabilities file; a feature folder needs at least one agent and one capability"
            ),
        )),
        _ => {}
    }

    // Forbidden files: feature folders hold only capabilities and agents.
    for file in &inventory.files {
        let foreign = consts::FEATURE_FORBIDDEN_PREFIXES
            .iter()
            .any(|p| file.stem.starts_with(*p));
        if !foreign {
            continue;
        }
        findings.push(StructureFinding::new(
            consts::RULE_CODE_FEATURE_HEALTH,
            consts::FEATURE_HEALTH_VIOLATION_FORBIDDEN_FILES,
            file.rel(ws_root),
            format!(
                "feature folder '{rel}' holds '{}'; feature folders carry only capabilities and agent files — move utility, taxonomy, contract, and surface files to shared or a dedicated surface folder",
                file.name,
            ),
        ));
    }

    // Docs: forward and reverse direction.
    check_feature_docs(folder, rel, findings);
}

/// AES702 doc pair check, both directions.
///
/// Forward: a folder with source files must carry FRD.md + BACKLOG.md.
/// Reverse: a folder that carries FRD.md + BACKLOG.md must also carry at
/// least one *_orchestrator file.
fn check_feature_docs(folder: &Path, rel: &str, findings: &mut Vec<StructureFinding>) {
    let missing = missing_docs(folder, consts::FEATURE_DOC_PAIR);
    if !missing.is_empty() {
        findings.push(StructureFinding::new(
            consts::RULE_CODE_FEATURE_HEALTH,
            consts::FEATURE_HEALTH_VIOLATION_NO_DOC_PAIR,
            rel.to_string(),
            format!(
                "feature folder '{rel}' is missing {}; a feature folder carries {} beside its source",
                missing.join(" and "),
                consts::FEATURE_DOC_PAIR.join(" and "),
            ),
        ));
    }
}

/// Reverse direction of AES702: any non-shared folder holding a doc pair must
/// hold an orchestrator.
fn check_reverse_doc_orchestrator(folder: &Path, rel: &str, findings: &mut Vec<StructureFinding>) {
    let has_frd = folder.join(consts::FRD_DOC).is_file();
    let has_backlog = folder.join(consts::BACKLOG_DOC).is_file();
    if !has_frd && !has_backlog {
        return;
    }
    if !has_orchestrator_in_folder(folder) {
        findings.push(StructureFinding::new(
            consts::RULE_CODE_FEATURE_HEALTH,
            consts::FEATURE_HEALTH_VIOLATION_REVERSE_MISSING_ORCHESTRATOR,
            rel.to_string(),
            format!(
                "folder '{rel}' carries a doc pair but holds no *_orchestrator; a doc pair implies an orchestrator"
            ),
        ));
    }
}

/// AES703 — a surface-dominated folder carries surfaces plus permitted support
/// files. Capabilities and agent files are misplaced. DESIGN.md is required.
fn check_surface_folder(
    folder: &Path,
    folder_rel: &str,
    ws_root: &Path,
    inventory: &FolderInventory,
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
fn missing_docs<'a>(folder: &Path, names: &'a [&'a str]) -> Vec<&'a str> {
    let mut missing: Vec<&'a str> = names
        .iter()
        .copied()
        .filter(|name| !folder.join(name).is_file())
        .collect();
    missing.sort_unstable();
    missing
}

/// Strip the trailing underscore from a layer prefix for prose.
fn layer_label(prefix: &str) -> String {
    prefix.trim_end_matches('_').to_string()
}

/// The folder's own name.
fn folder_name(folder: &Path) -> String {
    folder
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or_default()
        .to_string()
}

/// Recursively check whether *dir* (or a subdirectory) holds a file whose stem
/// ends with the orchestrator suffix.
fn has_orchestrator_in_folder(dir: &Path) -> bool {
    use std::fs;
    let Ok(entries) = fs::read_dir(dir) else {
        return false;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            if has_orchestrator_in_folder(&path) {
                return true;
            }
        } else if path
            .file_stem()
            .and_then(|n| n.to_str())
            .is_some_and(|n| n.ends_with(consts::ORCHESTRATOR_SUFFIX))
        {
            return true;
        }
    }
    false
}
