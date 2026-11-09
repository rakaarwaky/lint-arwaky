// PURPOSE: RequirementFolderParity — AES706: PR-<segment> id to crate folder mapping.
//
// Every crate folder's FRD.md carries FR ids whose feature prefix matches the
// folder name (crates/filesystem → FR-Filesystem-NNN), and every PR-<segment>-001..N
// id in PRD.md / ROADMAP.md maps 1:1 to a crate folder by order
// (PR-<segment>-001 → the first folder, `filesystem`). A mismatch fires AES706 so the
// linter enforces the 1:1 contract between spec ids and crate folders.
//
// The `<segment>` token belongs to the audited project (`LA`, `PSD`, `CALC`, …),
// never to lint-arwaky, so no violation name and no message may hardcode one
// project's segment. The folder order is likewise derived from the folder names
// that are actually crate folders: language caches (`__pycache__`), build
// output (`target`, `dist`), and dependency trees never take a position, because
// a single phantom folder at position 0 shifts every id onto its neighbour's
// folder.

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

/// Part B: every `PR-<segment>-001..N` id in PRD.md / ROADMAP.md maps to a real
/// crate folder by order (`PR-<segment>-001` → first folder), and the heading's
/// title names that folder.
///
/// Folder order is scoped by language whenever the spec documents are: a
/// document that carries one `PR-<segment>` sequence per language (`PR-PY-…`
/// rows for the Python modules, `PR-TS-…` rows for the TypeScript packages)
/// audits each sequence against its own member dir's folder order, so the
/// Python rows never get compared against a TypeScript folder. A document with
/// a single sequence spans the whole tree by design, and is audited against the
/// folder order of every member dir in turn.
///
/// The order itself comes from the crate folders that actually exist — caches,
/// build output, and dependency trees never take a position — so AES706 can
/// lint any project, not just lint-arwaky itself.
fn audit_pr_la_documents(root: &Path, ws_root: &Path) -> Vec<StructureFinding> {
    let members = utility_structure_parsers::scoped_member_dirs(root, ws_root);
    // Read every spec document once: the segment set spans both, so a folder
    // reached by an id in either document counts as covered.
    let documents: Vec<(&str, PathBuf, String)> = consts::PR_LA_DOCUMENTS
        .iter()
        .filter_map(|doc_name| {
            let doc = ws_root.join(doc_name);
            if !doc.is_file() {
                return None;
            }
            let contents = std::fs::read_to_string(&doc).ok()?;
            Some((*doc_name, doc, contents))
        })
        .collect();
    // No spec document: the reverse check has nothing to agree with, so a
    // project with folders but no PRD.md/ROADMAP.md is not penalized.
    if documents.is_empty() {
        return Vec::new();
    }

    let segments = documents
        .iter()
        .flat_map(|(_, _, contents)| document_segments(contents))
        .fold(Vec::<String>::new(), |mut all, segment| {
            if !all.contains(&segment) {
                all.push(segment);
            }
            all
        });
    // Every `PR-<segment>` id across the spec documents, grouped by segment as
    // (position, title). Used to bind each segment to the member dir whose
    // folders its ids actually reference, so segment order need not match
    // member dir order.
    let segment_ids: Vec<(String, Vec<(usize, String)>)> = segments
        .iter()
        .map(|segment| {
            let ids: Vec<(usize, String)> = documents
                .iter()
                .flat_map(|(_, _, contents)| segment_id_positions(contents, segment))
                .collect();
            (segment.clone(), ids)
        })
        .collect();
    let mut lanes = member_lanes(&members, &segment_ids);
    let mut findings = Vec::new();

    for (doc_name, doc, contents) in &documents {
        for lane in lanes.iter_mut() {
            // Only segment-bound lanes process ids. Unbound-member lanes
            // (segment == None) exist solely to report their folders as
            // unreached in the reverse check below.
            let Some(segment) = lane.segment.as_deref() else {
                continue;
            };
            for (position, pr_id, title) in extract_pr_ids(contents, Some(segment)) {
                lane.covered_positions.insert(position);
                let Some(expected_folder) = lane.order.get(position) else {
                    findings.push(dangling_pr_id_finding(
                        doc, ws_root, &pr_id, position, doc_name,
                    ));
                    continue;
                };
                if !title.is_empty() && !title_names_folder(&title, expected_folder) {
                    findings.push(title_mismatch_finding(
                        doc,
                        ws_root,
                        &pr_id,
                        &title,
                        expected_folder,
                        &lane.label(),
                    ));
                }
            }
        }
    }

    // Reverse check: every crate folder position in a lane's order must be
    // reached by a `PR-<segment>` id in at least one spec document. Runs per
    // lane because a position only means something inside its own member:
    // merging members first would report one language's folder position
    // against another language's folder.
    for lane in &lanes {
        for (position, folder) in lane.order.iter().enumerate() {
            if lane.covered_positions.contains(&position) {
                continue;
            }
            findings.push(folder_without_pr_id_finding(
                position,
                folder,
                &lane.label(),
                &lane.rel_path(folder),
            ));
        }
    }

    findings
}

/// One audit lane: a set of member dirs, the crate folder order over them, and
/// the `PR-<segment>` token the document uses for that order.
struct FolderLane {
    /// The member dirs this lane orders, each paired with its own name.
    owners: Vec<(PathBuf, String)>,
    /// Every crate folder name across *owners*, sorted.
    order: Vec<String>,
    /// The `PR-<segment>` token this lane audits, e.g. `PSD`. `None` audits
    /// every segment present in the documents.
    segment: Option<String>,
    /// Positions the spec documents reached, so the reverse check can report
    /// the folders no id maps onto.
    covered_positions: std::collections::BTreeSet<usize>,
}

impl FolderLane {
    /// The member dirs this lane covers, e.g. `modules` or
    /// `crates|modules|packages`, for message text.
    fn label(&self) -> String {
        self.owners
            .iter()
            .map(|(_, name)| name.as_str())
            .collect::<Vec<_>>()
            .join("|")
    }

    /// The path a finding for *folder* names: the member dir that holds it.
    fn rel_path(&self, folder: &str) -> String {
        for (member, name) in &self.owners {
            if member.join(folder).is_dir() {
                return format!("{name}/{folder}");
            }
        }
        format!("{}/{folder}", self.label())
    }
}

/// Build the audit lanes *members* owe the spec documents as a set.
///
/// `segment_ids` pairs each `PR-<segment>` token with its `(position, title)`
/// ids across every spec document, so lanes bind segment→member by which
/// member's folders the titles actually name — never by assuming segment order
/// equals member dir order. A segment whose titles name no folder is audited
/// against every member's merged order, and no position is silently excused.
fn member_lanes(
    members: &[PathBuf],
    segment_ids: &[(String, Vec<(usize, String)>)],
) -> Vec<FolderLane> {
    let owners: Vec<(PathBuf, String)> = members
        .iter()
        .map(|member| (member.clone(), member_name(member)))
        .collect();
    if members.is_empty()
        || members
            .iter()
            .all(|member| crate_folder_names(std::slice::from_ref(member)).is_empty())
    {
        return Vec::new();
    }
    if segment_ids.is_empty() {
        // A spec document exists but declares no `PR-<segment>` id at all, so
        // no position can be reached: one lane over the merged order reports
        // every crate folder, which is how the missing ids surface.
        return vec![FolderLane {
            order: crate_folder_names(members),
            owners,
            segment: None,
            covered_positions: std::collections::BTreeSet::new(),
        }];
    }
    // Bind each segment to the member dir whose crate folders its titles name,
    // matching titles to folder names. Empty titles are skipped; a segment
    // binds to the member all its non-empty titles consistently name, and a
    // segment whose titles name no folder stays audited against every member's
    // merged order.
    let mut lanes: Vec<FolderLane> = Vec::new();
    let mut laned: Vec<(PathBuf, String)> = Vec::new();
    for (segment, ids) in segment_ids {
        let seg_bound = bound_members_for_segment(members, ids);
        if seg_bound.len() == 1 {
            let Some((member, name)) = seg_bound.first().cloned() else {
                continue;
            };
            laned.push((member.clone(), name.clone()));
            lanes.push(FolderLane {
                owners: vec![(member.clone(), name.clone())],
                order: crate_folder_names(std::slice::from_ref(&member)),
                segment: Some(segment.clone()),
                covered_positions: std::collections::BTreeSet::new(),
            });
            continue;
        }
        // Segment binds to no single member: audit it against every member's
        // merged order, so all members are treated as laned.
        laned.extend(owners.clone());
        let order = crate_folder_names(members);
        lanes.push(FolderLane {
            owners: owners.clone(),
            order: order.clone(),
            segment: Some(segment.clone()),
            covered_positions: std::collections::BTreeSet::new(),
        });
    }
    // Members bound to no segment keep their folders unreached: one fallback
    // lane reports every folder they hold, so `pr_folder_without_id` still
    // fires for folders with no spec id.
    let unbound: Vec<(PathBuf, String)> = owners
        .into_iter()
        .filter(|(m, _)| !laned.iter().any(|(b, _)| b == m))
        .collect();
    if !unbound.is_empty() {
        let unbound_members: Vec<PathBuf> = unbound.iter().map(|(m, _)| m.clone()).collect();
        let order = crate_folder_names(&unbound_members);
        lanes.push(FolderLane {
            owners: unbound,
            order,
            segment: None,
            covered_positions: std::collections::BTreeSet::new(),
        });
    }
    lanes
}

/// The member dirs whose crate folders are named by *ids*' titles. Empty
/// titles are skipped. A segment binds to the member all its non-empty titles
/// consistently name; a title naming no folder, or titles naming folders under
/// multiple members, leaves the segment unbound.
fn bound_members_for_segment(
    members: &[PathBuf],
    ids: &[(usize, String)],
) -> Vec<(PathBuf, String)> {
    // Non-empty titles only: an untitled heading (`### PR-LA-001`) names no
    // folder and must not veto a segment whose other titles bind cleanly.
    let titles: Vec<&str> = ids
        .iter()
        .map(|(_, t)| t.as_str())
        .filter(|t| !t.is_empty())
        .collect();
    if titles.is_empty() {
        return Vec::new(); // segment has no titles at all → unbound
    }
    let mut bound: Option<(PathBuf, String)> = None;
    for title in titles {
        let folder = named_folder(members, title);
        let Some(folder) = folder else {
            return Vec::new(); // a title names no folder → unbound
        };
        let owning_member = owning_member(members, &folder);
        match bound {
            None => bound = Some(owning_member),
            Some(ref b) if b.0 == owning_member.0 => {}
            Some(_) => return Vec::new(), // titles name folders under multiple members
        }
    }
    bound.into_iter().collect()
}

/// The crate folder name that *title* names, if exactly one folder under any
/// member matches it case-insensitively or by stem.
fn named_folder(members: &[PathBuf], title: &str) -> Option<String> {
    let mut matches: Vec<String> = Vec::new();
    for member in members {
        for folder in crate_folders(member) {
            let name = folder.file_name()?.to_str()?;
            if title_names_folder(title, name) && !matches.contains(&name.to_string()) {
                matches.push(name.to_string());
            }
        }
    }
    if matches.len() == 1 {
        if let Some(name) = matches.first() {
            return Some(name.clone());
        }
        return None;
    }
    None
}

/// The member dir that holds *folder*, paired with its name.
fn owning_member(members: &[PathBuf], folder: &str) -> (PathBuf, String) {
    for member in members {
        if member.join(folder).is_dir() {
            return (member.clone(), member_name(member));
        }
    }
    (
        members.last().unwrap_or(&PathBuf::new()).clone(),
        member_name(members.last().unwrap_or(&PathBuf::new())),
    )
}

/// The distinct `PR-<segment>` tokens a document uses, in first-seen order.
fn document_segments(contents: &str) -> Vec<String> {
    let mut segments: Vec<String> = Vec::new();
    for (_, pr_id, _) in extract_pr_ids(contents, None) {
        let token = pr_id.split('-').nth(1).unwrap_or_default().to_string();
        if !token.is_empty() && !segments.contains(&token) {
            segments.push(token);
        }
    }
    segments
}

/// Every `PR-<segment>-NNN` id in *contents* whose segment equals *segment*,
/// as `(position, title)` pairs. Used to bind a segment to the member dir
/// whose folders its titles name.
fn segment_id_positions(contents: &str, segment: &str) -> Vec<(usize, String)> {
    extract_pr_ids(contents, Some(segment))
        .into_iter()
        .map(|(position, _id, title)| (position, title))
        .collect()
}

/// The member dir's own name (`crates`, `modules`, `packages`).
fn member_name(member: &Path) -> String {
    member
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or_default()
        .to_string()
}

/// Finding: a feature crate folder has no FRD.md at all.
fn missing_frd_finding(frd: &Path, ws_root: &Path, folder_name: &str) -> StructureFinding {
    StructureFinding::new(
        consts::RULE_CODE_PR_LA_FOLDER,
        consts::PR_VIOLATION_MISSING_FRD,
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

/// Finding: a crate folder exists at a position in its member's crate order
/// that no PR-<segment>-NNN id in PRD.md / ROADMAP.md reaches.
fn folder_without_pr_id_finding(
    position: usize,
    folder: &str,
    member: &str,
    path: &str,
) -> StructureFinding {
    StructureFinding::new(
        consts::RULE_CODE_PR_LA_FOLDER,
        consts::PR_VIOLATION_FOLDER_WITHOUT_ID,
        path,
        format!(
            "'{folder}' sits at {member} crate position {position} but no PR-<segment> id maps to it"
        ),
    )
    .with_reason(
        "PR-<segment> ids map 1:1 to crate folders by order, so every crate folder position needs a PR-<segment> id.",
        format!(
            "Add a 'PR-<segment>-{pos}' heading to PRD.md and ROADMAP.md that names '{folder}', where pos = {position} + 1.",
            pos = position + 1,
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
        consts::PR_VIOLATION_FR_ID_MISMATCH,
        relative_to(frd, ws_root),
        format!("folder '{folder_name}' carries FR id '{fr_id}' but its prefix must be 'FR-{prefix}'"),
    )
    .with_reason(
        "A crate folder's FR ids all share the folder's feature prefix so a reader can match an FR id to the crate that owns it.".to_string(),
        format!("Rename '{fr_id}' to 'FR-{prefix}-NNN' or move the FRD into the folder that owns its prefix.",),
    )
}

/// Finding: a PR-<segment> id has no crate folder at its position.
fn dangling_pr_id_finding(
    doc: &Path,
    ws_root: &Path,
    pr_id: &str,
    position: usize,
    doc_name: &str,
) -> StructureFinding {
    StructureFinding::new(
        consts::RULE_CODE_PR_LA_FOLDER,
        consts::PR_VIOLATION_PR_ID_MISMATCH,
        relative_to(doc, ws_root),
        format!("'{pr_id}' has no crate folder at position {position}"),
    )
    .with_reason(
        "PR-<segment> ids map 1:1 to crate folders by order, so a position with no folder cannot be satisfied.",
        format!(
            "Add a crate folder for position {position}, or remove the dangling '{pr_id}' from '{doc_name}'."
        ),
    )
}

/// Finding: the heading title does not name the expected folder.
fn title_mismatch_finding(
    doc: &Path,
    ws_root: &Path,
    pr_id: &str,
    title: &str,
    expected_folder: &str,
    member: &str,
) -> StructureFinding {
    StructureFinding::new(
        consts::RULE_CODE_PR_LA_FOLDER,
        consts::PR_VIOLATION_PR_ID_MISMATCH,
        relative_to(doc, ws_root),
        format!(
            "'{pr_id}' titles '{title}' but maps to '{member}/{expected_folder}' at its position"
        ),
    )
    .with_reason(
        "The heading title of a PR-<segment> id should name the crate folder it maps to.",
        format!(
            "Rename '{pr_id}'s heading to mention '{expected_folder}', or remap it to the folder named by '{title}'."
        ),
    )
}

/// Every immediate subdirectory of a member dir is a candidate crate folder,
/// except the non-crate ones: a language cache, build output, or dependency
/// tree must never take a position in the order.
fn crate_folders(member: &Path) -> Vec<PathBuf> {
    let Ok(entries) = std::fs::read_dir(member) else {
        return Vec::new();
    };
    let mut dirs: Vec<PathBuf> = entries
        .flatten()
        .map(|e| e.path())
        .filter(|p| p.is_dir())
        .filter(|p| !is_non_crate_dir(p))
        .collect();
    dirs.sort();
    dirs
}

/// True when *folder* is a cache, build, dependency, or test-scaffolding
/// directory rather than a crate folder.
fn is_non_crate_dir(folder: &Path) -> bool {
    let name = folder.file_name().and_then(|n| n.to_str()).unwrap_or("");
    consts::NON_CRATE_DIRS.contains(&name) || name.ends_with(".egg-info")
}

/// Sorted names of every crate folder under *members*, the folder order a
/// `PR-<segment>` sequence maps onto. Derived from the actual member layouts so
/// AES706 can lint any project — not just lint-arwaky itself.
fn crate_folder_names(members: &[PathBuf]) -> Vec<String> {
    let mut names: Vec<String> = members
        .iter()
        .flat_map(|member| {
            crate_folders(member).into_iter().filter_map(|folder| {
                folder
                    .file_name()
                    .and_then(|n| n.to_str())
                    .map(str::to_string)
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
/// (empty when the heading has none), e.g. `### PR-PSD-001: Multi-language
/// scanning (Rust, Python, JS/TS)`.
///
/// *segment* narrows the scan to one project token (`PSD`, `CALC`, …) when the
/// document carries one id sequence per language; `None` returns every
/// `PR-<segment>-<number>` shape.
fn extract_pr_ids(contents: &str, segment: Option<&str>) -> Vec<(usize, String, String)> {
    let mut ids = Vec::new();
    for line in contents.lines() {
        // PR-<segment>-NNN ids appear inside markdown headings
        // (`### PR-PSD-001: …`), so scan the whole line rather than
        // requiring a line prefix. The segment (e.g. `PSD`, `CALC`) is
        // not hardcoded — any `PR-<segment>-<number>` shape is accepted.
        let mut search_from = 0;
        while let Some(rel) = line[search_from..].find("PR-") {
            let start = search_from + rel;
            let rest = &line[start + 3..]; // after "PR-"

            // Project segment: non-empty run of characters up to the dash
            // that precedes the number, e.g. "PSD" in "PR-PSD-001".
            let segment_end = rest.find('-').unwrap_or(rest.len());
            let token = &rest[..segment_end];
            if token.is_empty() || segment_end == rest.len() {
                search_from = start + 3;
                continue;
            }
            if segment.is_some_and(|wanted| wanted != token) {
                search_from = start + 3 + token.len() + 1;
                continue;
            }

            // After the segment dash, expect digits.
            let after_dash = &rest[segment_end + 1..];
            let digits: String = after_dash
                .chars()
                .take_while(|c| c.is_ascii_digit())
                .collect();
            if digits.is_empty() {
                search_from = start + 3 + token.len() + 1;
                continue;
            }

            let position = digits
                .parse::<usize>()
                .map(|n| n.saturating_sub(1))
                .unwrap_or(usize::MAX);
            let after = &after_dash[digits.len()..];
            let title_raw = after.trim_start_matches([':', '|', ' ', '\t']);
            let title = title_raw.split('|').next().unwrap_or("").trim();
            ids.push((position, format!("PR-{token}-{digits}"), title.to_string()));
            search_from = start + 3 + token.len() + 1 + digits.len();
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

/// True when two tokens share a common stem: they match after stripping a
/// trailing `s`/`ing`/`ed`, e.g. "watching" vs "watch". A bare prefix match is
/// intentionally *not* used, because "file" is a prefix of "filesystem" but
/// the two are different folders and must not be conflated.
fn stems_match(a: &str, b: &str) -> bool {
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
