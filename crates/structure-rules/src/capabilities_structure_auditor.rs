// PURPOSE: StructureAuditor — the folder-layout invariant auditor behind IStructureAuditProtocol
//
// Walks the workspace members under a root and audits each folder against
// AES701 (shared purity), AES702 (feature health), AES703 (surface
// purity), AES704 (feature doc pair), and AES705 (surface DESIGN.md).
// Each finding carries a machine-readable violation_type.
use std::path::Path;

use shared::structure_rules::contract_structure_protocol::IStructureAuditProtocol;
use shared::structure_rules::taxonomy_structure_constant as consts;
use shared::structure_rules::taxonomy_structure_request::{StructureFinding, StructureRequest};
use shared::structure_rules::taxonomy_structure_response::StructureResponse;

use crate::utility_structure_parsers;
use shared::structure_rules::FolderInventory;

/// The invariant auditor behind the structure audit protocol.
pub struct StructureAuditor {}

impl IStructureAuditProtocol for StructureAuditor {
    /// Walk the member folders under the request's root and audit each against
    /// every structure invariant.
    ///
    /// The audit root is the *workspace root*, the folder that directly
    /// contains the member directories. When the caller passes a member
    /// directory itself (`workspaces-bad/crates`), the root is one level up.
    fn audit(&self, request: StructureRequest) -> StructureResponse {
        let StructureRequest::AuditAll { root } = request;
        let ws_root = workspace_root(&root);
        let mut findings = Vec::new();

        for member in utility_structure_parsers::member_dirs(&ws_root) {
            let member_has_agent = utility_structure_parsers::has_member_orchestrator(&member);
            for folder in utility_structure_parsers::feature_dirs(&member) {
                let inventory = utility_structure_parsers::inventory(&folder);
                let folder_rel = match folder.strip_prefix(&ws_root) {
                    Ok(p) => p.to_string_lossy().replace('\\', "/"),
                    Err(_) => folder.to_string_lossy().replace('\\', "/"),
                };
                if folder_name(&folder) == consts::KERNEL_DIR {
                    check_shared_purity(&folder_rel, &ws_root, &inventory, &mut findings);
                } else {
                    check_feature_health(&folder_rel, &inventory, member_has_agent, &mut findings);
                    check_feature_docs(&folder, &folder_rel, &inventory, &mut findings);
                    check_surface_purity(&folder_rel, &ws_root, &inventory, &mut findings);
                    if inventory.is_surface_dominated() {
                        check_surface_docs(&folder, &folder_rel, &mut findings);
                    }
                }
            }
        }

        StructureResponse::Findings {
            findings: sorted(findings),
        }
    }
}

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

/// AES704 — a feature folder documents itself. A folder that carries the
/// layer files of a feature — capabilities, an orchestrator, or both — also
/// carries the two documents that say what the feature does and where its work
/// stands. A folder holding neither document is a feature nobody can read
/// before changing.
fn check_feature_docs(
    folder: &Path,
    rel: &str,
    inventory: &FolderInventory,
    findings: &mut Vec<StructureFinding>,
) {
    if !inventory.has_capabilities && !inventory.has_orchestrator {
        return;
    }
    let missing = missing_docs(folder, consts::FEATURE_DOC_PAIR);
    if missing.is_empty() {
        return;
    }
    findings.push(StructureFinding::new(
        consts::RULE_CODE_FEATURE_DOCS,
        consts::FEATURE_DOCS_VIOLATION_NO_DOC_PAIR,
        rel,
        format!(
            "feature folder '{rel}' is missing {}; a feature folder carries {} beside its source",
            missing.join(" and "),
            consts::FEATURE_DOC_PAIR.join(" and "),
        ),
    ));
}

/// AES705 — a surface folder documents itself. The source of a surface says
/// what the surface does; `DESIGN.md` says what it looks like, which entry
/// points reach it, and which states a user sees. Without it the next reader
/// has to infer the surface's shape from its handlers.
fn check_surface_docs(folder: &Path, rel: &str, findings: &mut Vec<StructureFinding>) {
    let names: [&str; 1] = [consts::SURFACE_DOC];
    let missing = missing_docs(folder, &names);
    if missing.is_empty() {
        return;
    }
    findings.push(StructureFinding::new(
        consts::RULE_CODE_SURFACE_DOCS,
        consts::SURFACE_DOCS_VIOLATION_NO_DESIGN,
        rel,
        format!(
            "surface folder '{rel}' is missing {}; a surface folder carries a {} recording its kind, entry points, and visible states",
            missing.join(" and "),
            consts::SURFACE_DOC,
        ),
    ));
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

/// AES702 — a feature folder carries capabilities and agents. One without the
/// other is a split feature: the orchestrator has nothing to coordinate, or
/// the capabilities have no orchestrator driving them. A member-level
/// orchestrator counts for every feature folder beneath it, since it is what
/// drives them.
fn check_feature_health(
    rel: &str,
    inventory: &FolderInventory,
    member_has_agent: bool,
    findings: &mut Vec<StructureFinding>,
) {
    match (inventory.has_capabilities, inventory.has_orchestrator || member_has_agent) {
        (true, false) => findings.push(StructureFinding::new(
            consts::RULE_CODE_FEATURE_HEALTH,
            consts::FEATURE_HEALTH_VIOLATION_MISSING_AGENT,
            rel,
            format!(
                "feature folder '{rel}' holds capabilities but no agent_*_orchestrator file; a feature folder needs at least one agent and one capability"
            ),
        )),
        (false, true) if inventory.has_orchestrator => findings.push(StructureFinding::new(
            consts::RULE_CODE_FEATURE_HEALTH,
            consts::FEATURE_HEALTH_VIOLATION_MISSING_CAPABILITY,
            rel,
            format!(
                "feature folder '{rel}' holds an agent orchestrator but no capabilities file; a feature folder needs at least one agent and one capability"
            ),
        )),
        _ => {}
    }
}

/// AES703 — a surface folder carries surfaces. A capabilities or agent file
/// there has to move to a feature folder; a surface crate legitimately keeps
/// utility, root, and barrel files alongside its surfaces.
fn check_surface_purity(
    folder_rel: &str,
    ws_root: &Path,
    inventory: &FolderInventory,
    findings: &mut Vec<StructureFinding>,
) {
    if !inventory.is_surface_dominated() {
        return;
    }
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
                file.name
            ),
        ));
    }
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

/// Collect unique findings, sorted for stable output.
fn sorted(findings: Vec<StructureFinding>) -> Vec<StructureFinding> {
    let mut seen = std::collections::BTreeSet::new();
    let mut out: Vec<StructureFinding> = findings
        .into_iter()
        .filter(|f| {
            seen.insert((
                f.code.clone(),
                f.violation_type.clone(),
                f.file.clone(),
                f.message.clone(),
            ))
        })
        .collect();
    out.sort_by(|a, b| {
        (&a.file, &a.code, &a.violation_type, &a.message).cmp(&(
            &b.file,
            &b.code,
            &b.violation_type,
            &b.message,
        ))
    });
    out
}
