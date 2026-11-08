// PURPOSE: RequirementFolderParity — AES706: PR-<project> id to crate folder mapping.
//
// Every crate folder's FRD.md carries FR ids whose feature prefix matches the
// folder name (crates/filesystem → FR-Filesystem-NNN), and every PR-<segment>-001..N
// id in PRD.md / ROADMAP.md maps 1:1 to a crate folder by order
// (PR-<segment>-001 → the first folder, `filesystem`). A mismatch fires AES706 so the
// linter enforces the 1:1 contract between spec ids and crate folders.

use shared_doc_rules::taxonomy_doc_audit_context_vo::MemberRole;
use shared_structure_rules::contract_structure_protocol::IStructurePrLaProtocol;
use shared_structure_rules::taxonomy_structure_rules_constant as consts;
use shared_structure_rules::taxonomy_structure_rules_request::{
    StructureFinding, StructureRequest,
};
use shared_structure_rules::taxonomy_structure_rules_response::StructureResponse;
use shared_structure_rules::utility_structure_parsers;
use shared_structure_rules::utility_structure_parsers::{find_workspace_root, sorted};

use std::path::{Path, PathBuf};

// ─── Block 1: Module Declaration ────────────────────────────────────────────

/// AES706: checks that every crate folder's FR ids match its folder name and
/// that every PR-<segment>-001..N id maps to a real crate folder by order.
pub struct RequirementFolderParity {}

impl RequirementFolderParity {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for RequirementFolderParity {
    fn default() -> Self {
        Self::new()
    }
}

// ─── Block 2: Module Contract ───

impl IStructurePrLaProtocol for RequirementFolderParity {
    /// AES706 — Part A: feature folders' FR ids vs folder name.
    /// Part B: PR-<segment> ids in PRD/ROADMAP vs crate folders by order.
    fn audit_pr_la_folder(&self, request: StructureRequest) -> StructureResponse {
        let StructureRequest::AuditAll { root } = request;
        let ws_root = workspace_root(&root);

        let findings: Vec<StructureFinding> = audit_feature_folders(&root, &ws_root)
            .into_iter()
            .chain(audit_pr_la_documents(&root, &ws_root))
            .collect();

        StructureResponse::Findings {
            findings: sorted(findings),
        }
    }
}

// ─── Block 3: Module Helpers ──────────────────────────────────────────────

/// Part A: every *feature* crate folder's FR ids must match its folder name.
/// Surface and layer folders owe DESIGN.md/DATA.md, not FRD.md, so FRD.md
/// presence is only checked on feature folders.
fn audit_feature_folders(root: &Path, ws_root: &Path) -> Vec<StructureFinding> {
    let mut findings = Vec::new();
    for member in utility_structure_parsers::scoped_member_dirs(root, ws_root) {
        let lang = member_lang(&member);
        for folder in crate_folders(&member) {
            if member_role(&folder, lang) != MemberRole::Feature {
                continue;
            }
            findings.extend(audit_feature_folder(&folder, ws_root));
        }
    }
    findings
}

/// Part A for one feature folder: FRD.md must exist, and every FR id it
/// declares must carry the folder's feature prefix.
fn audit_feature_folder(folder: &Path, ws_root: &Path) -> Vec<StructureFinding> {
    let folder_name = match folder.file_name().and_then(|n| n.to_str()) {
        Some(n) => n.to_string(),
        None => return Vec::new(),
    };
    let frd = folder.join(consts::FRD_DOC);
    if !frd.is_file() {
        return vec![missing_frd_finding(folder, ws_root, &folder_name)];
    }
    let Ok(contents) = std::fs::read_to_string(&frd) else {
        return Vec::new();
    };
    let prefix = fr_prefix(&folder_name);
    extract_fr_ids(&contents)
        .iter()
        .filter(|fr_id| !fr_id_matches_folder(fr_id, &prefix))
        .map(|fr_id| fr_mismatch_finding(&frd, ws_root, &folder_name, fr_id, &prefix))
        .collect()
}

/// Ids match when the feature prefix equals the folder's PascalCase prefix.
/// The leading `FR-` token is case-insensitive: an id may read
/// `FR-ADDITION-001` while the folder is `addition`.
fn fr_id_matches_folder(fr_id: &str, prefix: &str) -> bool {
    fr_id
        .to_ascii_uppercase()
        .starts_with(&format!("FR-{}", prefix.to_ascii_uppercase()))
}

/// Part B: every PR-<segment>-001..N id in PRD.md / ROADMAP.md maps to a real
/// crate folder by order (PR-<segment>-001 → first folder), the folder exists
/// under a member dir, and the heading's title names that folder.
///
/// The folder order is derived from the actual crate folders present under
/// each member dir, so AES706 can lint any project — not just lint-arwaky
/// itself. Sorted alphabetically for a stable order independent of scan order.
fn audit_pr_la_documents(root: &Path, ws_root: &Path) -> Vec<StructureFinding> {
    let members = utility_structure_parsers::scoped_member_dirs(root, ws_root);
    let order = crate_folder_names(&members);
    let mut findings = Vec::new();
    for doc_name in consts::PR_LA_DOCUMENTS {
        let doc = ws_root.join(doc_name);
        if !doc.is_file() {
            continue;
        }
        let Ok(contents) = std::fs::read_to_string(&doc) else {
            continue;
        };
        for (position, pr_la_id, title) in extract_pr_la_ids(&contents) {
            let expected_folder = order.get(position).map(|s| s.as_str()).unwrap_or("");
            if expected_folder.is_empty() {
                findings.push(dangling_pr_la_finding(
                    &doc, ws_root, &pr_la_id, position, doc_name,
                ));
                continue;
            }
            if !members
                .iter()
                .any(|member| member.join(expected_folder).is_dir())
            {
                findings.push(missing_folder_finding(
                    &doc,
                    ws_root,
                    &pr_la_id,
                    expected_folder,
                ));
                continue;
            }
            if !title.is_empty() && !title_names_folder(&title, expected_folder) {
                findings.push(title_mismatch_finding(
                    &doc,
                    ws_root,
                    &pr_la_id,
                    &title,
                    expected_folder,
                ));
            }
        }
    }
    findings
}

/// Finding: a feature crate folder has no FRD.md at all.
fn missing_frd_finding(frd: &Path, ws_root: &Path, folder_name: &str) -> StructureFinding {
    StructureFinding::new(
        consts::RULE_CODE_PR_LA_FOLDER,
        consts::PR_LA_VIOLATION_MISSING_FRD,
        relative_to(frd, ws_root),
        format!(
            "'{folder_name}' holds no FRD.md, so its FR ids cannot be verified against its prefix"
        ),
    )
    .with_reason(
        "Every feature crate folder must carry an FRD.md whose FR ids match its folder name.",
        format!(
            "Create '{}/FRD.md' with ids of the form 'FR-{}-NNN'.",
            folder_name, folder_name
        ),
    )
}

/// Finding: an FR id's prefix does not match the owning folder's prefix.
fn fr_mismatch_finding(
    frd: &Path,
    ws_root: &Path,
    folder_name: &str,
    fr_id: &str,
    prefix: &str,
) -> StructureFinding {
    StructureFinding::new(
        consts::RULE_CODE_PR_LA_FOLDER,
        consts::PR_LA_VIOLATION_FR_ID_MISMATCH,
        relative_to(frd, ws_root),
        format!("folder '{folder_name}' carries FR id '{fr_id}' but its prefix must be 'FR-{prefix}'"),
    )
    .with_reason(
        "A crate folder's FR ids all share the folder's feature prefix so a reader can match an FR id to the crate that owns it.".to_string(),
        format!("Rename '{fr_id}' to 'FR-{prefix}-NNN' or move the FRD into the folder that owns its prefix.",),
    )
}

/// Finding: a PR-<segment> id has no crate folder at its position.
fn dangling_pr_la_finding(
    doc: &Path,
    ws_root: &Path,
    pr_la_id: &str,
    position: usize,
    doc_name: &str,
) -> StructureFinding {
    StructureFinding::new(
        consts::RULE_CODE_PR_LA_FOLDER,
        consts::PR_LA_VIOLATION_PR_LA_ID_MISMATCH,
        relative_to(doc, ws_root),
        format!("'{pr_la_id}' has no crate folder at position {position}"),
    )
    .with_reason(
        "PR-<segment> ids map 1:1 to crate folders by order, so a position with no folder cannot be satisfied.",
        format!(
            "Add a crate folder for position {position}, or remove the dangling '{pr_la_id}' from '{doc_name}'."
        ),
    )
}

/// Finding: the PR-<segment> id's mapped folder does not exist under any member dir.
fn missing_folder_finding(
    doc: &Path,
    ws_root: &Path,
    pr_la_id: &str,
    expected_folder: &str,
) -> StructureFinding {
    StructureFinding::new(
        consts::RULE_CODE_PR_LA_FOLDER,
        consts::PR_LA_VIOLATION_PR_LA_ID_MISMATCH,
        relative_to(doc, ws_root),
        format!(
            "'{pr_la_id}' maps to '{expected_folder}' but no crates|modules|packages/{expected_folder} folder exists"
        ),
    )
    .with_reason(
        "A PR-<segment> id must point at a real crate folder, by order.",
        format!(
            "Create crates|modules|packages/{expected_folder}, or remap '{pr_la_id}' to an existing folder."
        ),
    )
}

/// Finding: the heading title does not name the expected folder.
fn title_mismatch_finding(
    doc: &Path,
    ws_root: &Path,
    pr_la_id: &str,
    title: &str,
    expected_folder: &str,
) -> StructureFinding {
    StructureFinding::new(
        consts::RULE_CODE_PR_LA_FOLDER,
        consts::PR_LA_VIOLATION_PR_LA_ID_MISMATCH,
        relative_to(doc, ws_root),
        format!("'{pr_la_id}' titles '{title}' but maps to folder '{expected_folder}'"),
    )
    .with_reason(
        "The heading title of a PR-<segment> id should name the crate folder it maps to.",
        format!(
            "Rename '{pr_la_id}'s heading to mention '{expected_folder}', or remap it to the folder named by '{title}'."
        ),
    )
}

/// Every immediate subdirectory of a member dir is a candidate crate folder.
fn crate_folders(member: &Path) -> Vec<PathBuf> {
    let Ok(entries) = std::fs::read_dir(member) else {
        return Vec::new();
    };
    let mut dirs: Vec<PathBuf> = entries
        .flatten()
        .map(|e| e.path())
        .filter(|p| p.is_dir())
        .collect();
    dirs.sort();
    dirs
}

/// Sorted names of every crate folder under *members*, used as the PR-<segment>
/// folder order. Derived from the actual workspace being audited so AES706
/// can lint any project — not just lint-arwaky itself.
fn crate_folder_names(members: &[PathBuf]) -> Vec<String> {
    let mut names: Vec<String> = members
        .iter()
        .flat_map(|member| {
            crate_folders(member).into_iter().filter_map(|folder| {
                folder
                    .file_name()
                    .and_then(|n| n.to_str())
                    .map(|s| s.to_string())
            })
        })
        .collect();
    names.sort();
    names
}

/// Classify *folder* by its crate structure, mirroring the doc-rules
/// collector.
fn member_role(folder: &Path, lang: &str) -> MemberRole {
    let name = folder.file_name().and_then(|n| n.to_str()).unwrap_or("");
    if name == "shared" || name == "kernel" {
        return MemberRole::Kernel;
    }
    if has_orchestrator_file(folder, lang)
        || (lang == "rust" && exposes_capability_in_cargo_toml(folder))
    {
        return MemberRole::Feature;
    }
    if has_surface_files(folder, lang) || has_surface_cargo_deps(folder) {
        return MemberRole::Surface;
    }
    MemberRole::None
}

/// The audit language of a member dir: `modules` is Python, `packages` is
/// JS/TS, and every other layout is Rust.
fn member_lang(member: &Path) -> &'static str {
    match member
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or_default()
    {
        "modules" => "python",
        "packages" => "ts",
        _ => "rust",
    }
}

/// Walk *folder*'s own entries and its `src/` sub-entries, returning true when
/// any name satisfies *pred*.
fn walk_top_and_src(folder: &Path, pred: impl Fn(&str) -> bool) -> bool {
    let Ok(entries) = std::fs::read_dir(folder) else {
        return false;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_file() {
            if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                if pred(name) {
                    return true;
                }
            }
        } else if path.is_dir() {
            if let Some(src) = path.file_name().and_then(|n| n.to_str()) {
                if src == "src" {
                    if let Ok(inner) = std::fs::read_dir(&path) {
                        for entry in inner.flatten() {
                            if let Some(n) = entry.path().file_name().and_then(|n| n.to_str()) {
                                if pred(n) {
                                    return true;
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    false
}

/// Does *folder* contain an `agent_*_orchestrator*.rs` file (top level or
/// under `src/`)?
fn has_orchestrator_file(folder: &Path, lang: &str) -> bool {
    match lang {
        "rust" => walk_top_and_src(folder, |n| {
            n.contains("agent_") && n.contains("_orchestrator") && n.ends_with(".rs")
        }),
        "python" => walk_top_and_src(folder, |n| {
            n.contains("agent_") && n.contains("orchestrator")
        }),
        _ => false,
    }
}

/// Does *folder*'s Cargo.toml mention `agent_`, `capability`, or `orchestrator`?
pub fn exposes_capability_in_cargo_toml(folder: &Path) -> bool {
    let cargo_path = folder.join("Cargo.toml");
    let Some(cargo) = cargo_path.to_str() else {
        return false;
    };
    let Ok(text) = std::fs::read_to_string(cargo) else {
        return false;
    };
    text.contains("agent_") || text.contains("capability") || text.contains("orchestrator")
}

/// Does *folder* contain `surface_*` files?
fn has_surface_files(folder: &Path, lang: &str) -> bool {
    match lang {
        "rust" => walk_top_and_src(folder, |n| n.starts_with("surface_")),
        "python" => walk_top_and_src(folder, |n| n.starts_with("surface_")),
        "ts" => walk_top_and_src(folder, |n| n.starts_with("surface_")),
        _ => false,
    }
}

/// Does *folder*'s Cargo.toml depend on cli/mcp/tui/dispatcher?
pub fn has_surface_cargo_deps(folder: &Path) -> bool {
    let cargo_path = folder.join("Cargo.toml");
    let Some(cargo) = cargo_path.to_str() else {
        return false;
    };
    let Ok(text) = std::fs::read_to_string(cargo) else {
        return false;
    };
    text.contains("cli-commands")
        || text.contains("mcp-server")
        || text.contains("tui")
        || text.contains("dispatcher")
}

/// Extract every `PR-<segment>-NNN` id from document contents as
/// `(position, id, title)`. The position is the id's slot in the crate folder
/// order (`PR-<segment>-001` → 0), so it stays stable even when an id is
/// mentioned more than once. The title is the heading text after the colon
/// (empty when the heading has none), e.g. `### PR-LA-001: Multi-language
/// scanning (Rust, Python, JS/TS)`.
fn extract_pr_la_ids(contents: &str) -> Vec<(usize, String, String)> {
    let mut ids = Vec::new();
    for line in contents.lines() {
        // PR-<segment>-NNN ids appear inside markdown headings
        // (`### PR-LA-001: …`), so scan the whole line rather than
        // requiring a line prefix. The segment (e.g. `LA`, `GAMING`) is
        // not hardcoded — any `PR-<segment>-<number>` shape is accepted.
        let mut search_from = 0;
        while let Some(rel) = line[search_from..].find("PR-") {
            let start = search_from + rel;
            let rest = &line[start + 3..]; // after "PR-"

            // Project segment: non-empty run of characters up to the dash
            // that precedes the number, e.g. "LA" in "PR-LA-001".
            let segment_end = rest.find('-').unwrap_or(rest.len());
            let segment = &rest[..segment_end];
            if segment.is_empty() || segment_end == rest.len() {
                search_from = start + 3;
                continue;
            }

            // After the segment dash, expect digits.
            let after_dash = &rest[segment_end + 1..];
            let digits: String = after_dash
                .chars()
                .take_while(|c| c.is_ascii_digit())
                .collect();
            if digits.is_empty() {
                search_from = start + 3 + segment.len() + 1;
                continue;
            }

            let position = digits
                .parse::<usize>()
                .map(|n| n.saturating_sub(1))
                .unwrap_or(usize::MAX);
            let after = &after_dash[digits.len()..];
            let title_raw = after.trim_start_matches([':', '|', ' ', '\t']);
            let title = title_raw.split('|').next().unwrap_or("").trim();
            ids.push((
                position,
                format!("PR-{segment}-{digits}"),
                title.to_string(),
            ));
            search_from = start + 3 + segment.len() + 1 + digits.len();
        }
    }
    ids
}

/// True when every token of *folder* appears in *title*, matching on exact
/// tokens or a common stem — e.g. "CLI core commands" names `cli-commands`,
/// while "Multi-language scanning" names no folder at all.
fn title_names_folder(title: &str, folder: &str) -> bool {
    let title_tokens = slug_tokens(title);
    let folder_tokens = slug_tokens(folder);
    folder_tokens.iter().all(|folder_token| {
        title_tokens.iter().any(|title_token| {
            title_token == folder_token || stems_match(title_token, folder_token)
        })
    })
}

/// Extract every `FR-<prefix>-NNN` id from FRD.md contents.
fn extract_fr_ids(contents: &str) -> Vec<String> {
    let mut ids = Vec::new();
    let mut search_from = 0;
    while let Some(rel) = contents[search_from..].find("FR-") {
        let start = search_from + rel;
        let rest = &contents[start + 3..];
        let digits: String = rest.chars().take_while(|c| c.is_ascii_digit()).collect();
        if digits.is_empty() {
            search_from = start + 3;
            continue;
        }
        ids.push(format!("FR-{}", digits));
        search_from = start + 3 + digits.len();
    }
    ids
}

/// The PascalCase feature prefix for a folder name, e.g. `filesystem` →
/// `Filesystem`, `doc-rules` → `DOC` (via override), `structure-rules` →
/// `STR` (via override).
fn fr_prefix(folder_name: &str) -> String {
    for (kebab, override_prefix) in consts::FR_PREFIX_OVERRIDES {
        if folder_name == *kebab {
            return override_prefix.to_string();
        }
    }
    let mut prefix = String::new();
    let mut capitalize = true;
    for ch in folder_name.chars() {
        if ch == '-' || ch == '_' {
            capitalize = true;
        } else if capitalize {
            prefix.extend(ch.to_uppercase());
            capitalize = false;
        } else {
            prefix.push(ch);
        }
    }
    prefix
}

/// Turn a heading title into lowercase slug tokens, stripping punctuation.
fn slug_tokens(text: &str) -> Vec<String> {
    let cleaned: String = text
        .chars()
        .map(|c| if c.is_ascii_alphanumeric() { c } else { ' ' })
        .collect();
    cleaned
        .split_whitespace()
        .map(|t| t.to_ascii_lowercase())
        .collect()
}

/// True when two tokens share a common stem: one is a prefix of the other
/// (at least 3 chars) or they match after stripping a trailing `s`/`ing`/`ed`.
fn stems_match(a: &str, b: &str) -> bool {
    let (shorter, longer) = if a.len() <= b.len() { (a, b) } else { (b, a) };
    if shorter.len() >= 3 && longer.starts_with(shorter) {
        return true;
    }
    strip_suffix(a) == strip_suffix(b)
}

/// Strip trailing `s`, `ing`, or `ed` from *s*.
fn strip_suffix(s: &str) -> String {
    s.strip_suffix("ing")
        .or(s.strip_suffix("ed"))
        .or(s.strip_suffix("s"))
        .unwrap_or(s)
        .to_string()
}

/// Relative path of *file* under *ws_root* for the finding report.
fn relative_to(file: &Path, ws_root: &Path) -> String {
    file.strip_prefix(ws_root)
        .ok()
        .and_then(|p| p.to_str())
        .unwrap_or("")
        .to_string()
}

/// Workspace root for *root*: the root itself when it is already a workspace
/// root, otherwise the nearest ancestor that contains a workspace marker.
fn workspace_root(root: &Path) -> PathBuf {
    find_workspace_root(root)
}
