// PURPOSE: MemberRootPlacementAuditor — AES705: a member dir root carries
// wiring only; every layer file belongs to a folder beneath it.

use shared_structure_rules::contract_structure_protocol::IStructureMemberRootProtocol;
use shared_structure_rules::taxonomy_structure_rules_constant as consts;
use shared_structure_rules::taxonomy_structure_rules_request::{
    StructureFinding, StructureRequest,
};
use shared_structure_rules::taxonomy_structure_rules_response::StructureResponse;
use shared_structure_rules::taxonomy_structure_rules_vo::LayerFile;
use shared_structure_rules::utility_structure_parsers;
use shared_structure_rules::utility_structure_parsers::{find_workspace_root, sorted};

use std::path::{Path, PathBuf};

// ─── Block 1: Module Declaration ────────────────────────────

/// AES705: checks that `crates/`, `modules/`, and `packages/` hold folders and
/// wiring, and that no `agent_*`, `capabilities_*`, `surface_*`, or `utility_*`
/// file sits loose at a member root.
pub struct MemberRootPlacementAuditor {}

impl MemberRootPlacementAuditor {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for MemberRootPlacementAuditor {
    fn default() -> Self {
        Self::new()
    }
}

// ─── Block 2: Module Contract ────────────────

impl IStructureMemberRootProtocol for MemberRootPlacementAuditor {
    /// AES705 — every member dir root carries wiring only; a loose layer file
    /// at a root has no feature folder to belong to.
    fn audit_member_root(&self, request: StructureRequest) -> StructureResponse {
        let StructureRequest::AuditAll { root } = request;
        let ws_root = workspace_root(&root);

        let findings: Vec<StructureFinding> = audit_member_roots(&root, &ws_root);

        StructureResponse::Findings {
            findings: sorted(findings),
        }
    }
}

// ─── Block 3: Module Helpers ────────────

/// Audit every member dir: each root must hold folders and wiring only.
fn audit_member_roots(root: &Path, ws_root: &Path) -> Vec<StructureFinding> {
    let mut findings = Vec::new();
    for member in utility_structure_parsers::scoped_member_dirs(root, ws_root) {
        for file in loose_layer_files(&member) {
            let Some((prefix, destination)) = forbidden_prefix(&file) else {
                continue;
            };
            findings.push(misplaced_file_finding(
                &member,
                ws_root,
                &file,
                prefix,
                destination,
            ));
        }
    }
    findings
}

/// Finding: a layer file sits loose at a member dir root.
fn misplaced_file_finding(
    member: &Path,
    ws_root: &Path,
    file: &LayerFile,
    prefix: &str,
    destination: &str,
) -> StructureFinding {
    StructureFinding::new(
        consts::RULE_CODE_MEMBER_ROOT_PLACEMENT,
        consts::MEMBER_ROOT_VIOLATION_MISPLACED_FILE,
        file.rel(ws_root),
        format!(
            "member dir '{}' holds '{}' (prefix '{prefix}') at its root",
            relative_to(member, ws_root),
            file.name,
        ),
    )
    .with_reason(
        "A member dir carries folders and wiring. A layer file at its root has no feature folder to belong to.",
        destination.to_string(),
    )
}

/// Relative path of *path* against the workspace root.
fn relative_to(path: &Path, ws_root: &Path) -> String {
    match path.strip_prefix(ws_root) {
        Ok(p) => p.to_string_lossy().replace('\\', "/"),
        Err(_) => path.to_string_lossy().replace('\\', "/"),
    }
}

/// The layer files sitting directly in a member dir.
///
/// Only the top level is read: a file inside a folder beneath it belongs to
/// that folder, which is the shape AES705 exists to preserve. Files whose name
/// does not start with a classified layer prefix are ignored rather than
/// reported, so a `README.md` or a `Cargo.toml` at a member root stays legal.
fn loose_layer_files(member: &Path) -> Vec<LayerFile> {
    let Ok(entries) = std::fs::read_dir(member) else {
        return Vec::new();
    };
    let mut out = Vec::new();
    for entry in entries.flatten() {
        let path = entry.path();
        if !path.is_file() {
            continue;
        }
        if let Some(layer) = layer_file(&path) {
            out.push(layer);
        }
    }
    out.sort_by(|a, b| a.name.cmp(&b.name));
    out
}

/// Classify a file by its layer prefix, mirroring how the other auditors read
/// a folder: the stem's leading token is the layer.
fn layer_file(path: &Path) -> Option<LayerFile> {
    let name = path.file_name()?.to_str()?.to_string();
    let stem = path.file_stem()?.to_str()?.to_string();
    Some(LayerFile {
        name,
        path: path.to_path_buf(),
        stem,
    })
}

/// The forbidden prefix this file carries, and where such a file belongs.
fn forbidden_prefix(file: &LayerFile) -> Option<(&'static str, &'static str)> {
    consts::MEMBER_ROOT_FORBIDDEN_PREFIXES
        .iter()
        .find(|(prefix, _)| file.stem.starts_with(prefix))
        .copied()
}

/// Resolve the workspace root by walking up from *root* until we find
/// a directory that contains at least one of the member directories.
fn workspace_root(root: &Path) -> PathBuf {
    find_workspace_root(root)
}
