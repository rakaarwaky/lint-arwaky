// PURPOSE: FeatureHealthAuditor — AES702: feature folder health and docs

use shared_structure_rules::utility_structure_parsers::{self, sorted};

use shared_structure_rules::contract_structure_protocol::IStructureFeatureHealthProtocol;
use shared_structure_rules::taxonomy_structure_rules_constant as consts;
use shared_structure_rules::taxonomy_structure_rules_request::{
    StructureFinding, StructureRequest,
};
use shared_structure_rules::taxonomy_structure_rules_response::StructureResponse;

// ─── Block 1: Struct Definition ────────────────────────────

/// AES702: checks that feature folders hold capabilities + orchestrator pairs,
/// carry no foreign-layer files, and document themselves with FRD.md + BACKLOG.md.
pub struct FeatureHealthAuditor {}

// ─── Block 2: Protocol Trait Implementation ────────────────

impl IStructureFeatureHealthProtocol for FeatureHealthAuditor {
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

// ─── Private helpers ───────────────────────────────────────────────────────────

// ─── Block 3: Constructors, Std Traits, Helpers ────────────

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

/// AES702 — a feature folder carries capabilities and agents, has no foreign
/// layer files (utility, surface, taxonomy, contract), and documents itself
/// with FRD.md + BACKLOG.md.
fn check_feature_folder(
    folder: &std::path::Path,
    rel: &str,
    ws_root: &std::path::Path,
    inventory: &shared_structure_rules::taxonomy_structure_rules_vo::FolderInventory,
    findings: &mut Vec<StructureFinding>,
) {
    // Health: both sides required.
    match (inventory.has_capabilities, inventory.has_orchestrator) {
        (true, false) => findings.push(StructureFinding::new(
            consts::RULE_CODE_FEATURE_HEALTH,
            consts::FEATURE_HEALTH_VIOLATION_MISSING_AGENT,
            rel.to_string(),
            format!(
                "feature folder '{rel}' holds capabilities but no agent_*_orchestrator file"
            ),
        )
        .with_reason(
            "A feature folder is the agent/capability pair: the agent routes, the capability does. A capability with no agent has no entry point.",
            "Add an agent_*_orchestrator file that composes the folder's capabilities.",
        )),
        (false, true) => findings.push(StructureFinding::new(
            consts::RULE_CODE_FEATURE_HEALTH,
            consts::FEATURE_HEALTH_VIOLATION_MISSING_CAPABILITY,
            rel.to_string(),
            format!(
                "feature folder '{rel}' holds an agent orchestrator but no capabilities file"
            ),
        )
        .with_reason(
            "An orchestrator that routes to nothing is dead wiring; the feature does no work.",
            "Add a capabilities_* file carrying the feature's checks.",
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
            format!("feature folder '{rel}' holds '{}'", file.name),
        )
        .with_reason(
            "A feature folder carries only capabilities and agent files. Utility, taxonomy, contract, and surface files belong elsewhere.",
            "Move the file to shared/, or to the surface folder that owns its layer.",
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
fn check_feature_docs(folder: &std::path::Path, rel: &str, findings: &mut Vec<StructureFinding>) {
    let missing = missing_docs(folder, consts::FEATURE_DOC_PAIR);
    if !missing.is_empty() {
        findings.push(StructureFinding::new(
            consts::RULE_CODE_FEATURE_HEALTH,
            consts::FEATURE_HEALTH_VIOLATION_NO_DOC_PAIR,
            rel.to_string(),
            format!("feature folder '{rel}' is missing {}", missing.join(" and ")),
        )
        .with_reason(
            format!(
                "A feature folder carries {} beside its source, so a reader can tell what the feature does without reading the code.",
                consts::FEATURE_DOC_PAIR.join(" and ")
            ),
            format!("Add {} to the feature folder.", missing.join(" and ")),
        ));
    }
}

/// Reverse direction of AES702: any non-shared folder holding a feature doc pair
/// must hold an orchestrator. A surface folder carrying DESIGN.md + BACKLOG.md
/// is not subject to this check.
fn check_reverse_doc_orchestrator(
    folder: &std::path::Path,
    rel: &str,
    findings: &mut Vec<StructureFinding>,
) {
    let has_frd = folder.join(consts::FRD_DOC).is_file();
    let has_backlog = folder.join(consts::BACKLOG_DOC).is_file();
    // Only fire if the folder carries a feature doc pair (FRD + BACKLOG).
    // Surface folders with DESIGN.md + BACKLOG.md are exempt.
    if !has_frd || !has_backlog {
        return;
    }
    if !has_orchestrator_in_folder(folder) {
        findings.push(StructureFinding::new(
            consts::RULE_CODE_FEATURE_HEALTH,
            consts::FEATURE_HEALTH_VIOLATION_REVERSE_MISSING_ORCHESTRATOR,
            rel.to_string(),
            format!("folder '{rel}' carries a doc pair but holds no *_orchestrator"),
        )
        .with_reason(
            "A documented feature with no orchestrator has nothing to route; the doc describes wiring that does not exist.",
            "Add an agent_*_orchestrator file, or remove the doc pair from this folder.",
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

/// Recursively check whether *dir* (or a subdirectory) holds a file whose stem
/// ends with the orchestrator suffix.
///
/// Symlinks are never followed (`DirEntry::file_type` inspects the entry
/// itself), and the walk stops at [`consts::MAX_ORCHESTRATOR_WALK_DEPTH`], so
/// a symlink cycle or a pathologically deep layout cannot hang the audit.
fn has_orchestrator_in_folder(dir: &std::path::Path) -> bool {
    has_orchestrator_at_depth(dir, 0)
}

fn has_orchestrator_at_depth(dir: &std::path::Path, depth: u32) -> bool {
    if depth >= consts::MAX_ORCHESTRATOR_WALK_DEPTH {
        return false;
    }
    use std::fs;
    let Ok(entries) = fs::read_dir(dir) else {
        return false;
    };
    for entry in entries.flatten() {
        // `file_type` does not follow symlinks, so a link to a directory is
        // skipped instead of re-entering an already-walked tree.
        if entry.file_type().is_ok_and(|t| t.is_dir()) {
            if has_orchestrator_at_depth(&entry.path(), depth + 1) {
                return true;
            }
        } else if entry
            .path()
            .file_stem()
            .and_then(|n| n.to_str())
            .is_some_and(|n| n.ends_with(consts::ORCHESTRATOR_SUFFIX))
        {
            return true;
        }
    }
    false
}
