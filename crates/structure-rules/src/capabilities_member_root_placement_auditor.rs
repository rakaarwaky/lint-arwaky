// PURPOSE: MemberRootPlacementAuditor — AES705: a member dir root carries
// wiring only; every layer file belongs to a folder beneath it.

use shared_structure_rules::utility_structure_parsers;

use shared_structure_rules::contract_structure_protocol::IStructureMemberRootProtocol;
use shared_structure_rules::taxonomy_structure_rules_constant as consts;
use shared_structure_rules::taxonomy_structure_rules_request::{
    StructureFinding, StructureRequest,
};
use shared_structure_rules::taxonomy_structure_rules_response::StructureResponse;

// ─── Block 1: Struct Definition ────────────────────────────

/// AES705: checks that `crates/`, `modules/`, and `packages/` hold folders and
/// wiring, and that no `agent_*`, `capabilities_*`, `surface_*`, or `utility_*`
/// file sits loose at a member root.
pub struct MemberRootPlacementAuditor {}

// ─── Block 2: Protocol Trait Implementation ────────────────

impl IStructureMemberRootProtocol for MemberRootPlacementAuditor {
    fn audit_member_root(&self, request: StructureRequest) -> StructureResponse {
        let StructureRequest::AuditAll { root } = request;
        let ws_root = workspace_root(&root);
        let mut findings = Vec::new();

        for member in utility_structure_parsers::member_dirs(&ws_root) {
            let member_rel = match member.strip_prefix(&ws_root) {
                Ok(p) => p.to_string_lossy().replace('\\', "/"),
                Err(_) => member.to_string_lossy().into_owned(),
            };
            for file in loose_layer_files(&member) {
                let Some((prefix, destination)) = forbidden_prefix(&file) else {
                    continue;
                };
                findings.push(StructureFinding::new(
                    consts::RULE_CODE_MEMBER_ROOT_PLACEMENT,
                    consts::MEMBER_ROOT_VIOLATION_MISPLACED_FILE,
                    file.rel(&ws_root),
                    format!(
                        "member dir '{member_rel}' holds '{name}' (prefix '{prefix}') at its root; a member dir carries folders and wiring — {destination}",
                        name = file.name,
                    ),
                ));
            }
        }

        StructureResponse::Findings {
            findings: utility_structure_parsers::sorted(findings),
        }
    }
}

// ─── Block 3: Constructors, Std Traits, Helpers ────────────

/// The layer files sitting directly in a member dir.
///
/// Only the top level is read: a file inside a folder beneath it belongs to
/// that folder, which is the shape AES705 exists to preserve. Files whose name
/// does not start with a classified layer prefix are ignored rather than
/// reported, so a `README.md` or a `Cargo.toml` at a member root stays legal.
fn loose_layer_files(
    member: &std::path::Path,
) -> Vec<shared_structure_rules::taxonomy_structure_rules_vo::LayerFile> {
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
fn layer_file(
    path: &std::path::Path,
) -> Option<shared_structure_rules::taxonomy_structure_rules_vo::LayerFile> {
    use shared_structure_rules::taxonomy_structure_rules_vo::LayerFile;
    let name = path.file_name()?.to_str()?.to_string();
    let stem = path.file_stem()?.to_str()?.to_string();
    Some(LayerFile {
        name,
        path: path.to_path_buf(),
        stem,
    })
}

/// The forbidden prefix this file carries, and where such a file belongs.
fn forbidden_prefix(
    file: &shared_structure_rules::taxonomy_structure_rules_vo::LayerFile,
) -> Option<(&'static str, &'static str)> {
    consts::MEMBER_ROOT_FORBIDDEN_PREFIXES
        .iter()
        .find(|(prefix, _)| file.stem.starts_with(prefix))
        .copied()
}

/// Resolve the workspace root: the nearest ancestor that holds a member dir.
fn workspace_root(root: &std::path::Path) -> std::path::PathBuf {
    if consts::MEMBER_DIRS
        .iter()
        .any(|name| root.join(name).is_dir())
    {
        return root.to_path_buf();
    }
    root.parent()
        .filter(|parent| {
            consts::MEMBER_DIRS
                .iter()
                .any(|name| parent.join(name).is_dir())
        })
        .map_or_else(|| root.to_path_buf(), std::path::Path::to_path_buf)
}
