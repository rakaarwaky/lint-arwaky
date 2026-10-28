// PURPOSE: PrLaFolderAuditor — AES706: PR-LA id to crate folder mapping.
//
// Every crate folder's FRD.md carries FR ids whose feature prefix matches the
// folder name (crates/filesystem → FR-Filesystem-NNN), and every PR-LA-001..N
// id in PRD.md / ROADMAP.md maps 1:1 to a crate folder by order
// (PR-LA-001 → the first folder, `filesystem`). A mismatch fires AES706 so the
// linter enforces the 1:1 contract between spec ids and crate folders.

use shared_structure_rules::contract_structure_protocol::IStructurePrLaProtocol;
use shared_structure_rules::taxonomy_structure_rules_constant as consts;
use shared_structure_rules::taxonomy_structure_rules_request::{
    StructureFinding, StructureRequest,
};
use shared_structure_rules::taxonomy_structure_rules_response::StructureResponse;
use shared_structure_rules::utility_structure_parsers;
use shared_structure_rules::utility_structure_parsers::{find_workspace_root, sorted};

use std::path::{Path, PathBuf};

// ─── Block 1: Struct Definition ────────────────────────────────────────────

/// AES706: checks that every crate folder's FR ids match its folder name and
/// that every PR-LA-001..N id maps to a real crate folder by order.
pub struct PrLaFolderAuditor {}

// ─── Block 2: Protocol Trait Implementation ────────────────────────────────

impl IStructurePrLaProtocol for PrLaFolderAuditor {
    fn audit_pr_la_folder(&self, request: StructureRequest) -> StructureResponse {
        let StructureRequest::AuditAll { root } = request;
        let ws_root = workspace_root(&root);
        let mut findings = Vec::new();

        // Part A: every crate folder's FR ids must match its folder name.
        for member in utility_structure_parsers::member_dirs(&ws_root) {
            for folder in crate_folders(&member) {
                let folder_name = match folder.file_name().and_then(|n| n.to_str()) {
                    Some(n) => n.to_string(),
                    None => continue,
                };
                let frd = folder.join(consts::FRD_DOC);
                if !frd.is_file() {
                    findings.push(StructureFinding::new(
                        consts::RULE_CODE_PR_LA_FOLDER,
                        consts::PR_LA_VIOLATION_MISSING_FRD,
                        relative_to(&folder, &ws_root),
                        format!("crate folder '{folder_name}' has no FRD.md"),
                    )
                    .with_reason(
                        "Every crate folder carries an FRD so its FR ids can be checked against the folder name.",
                        format!(
                            "Add '{}/FRD.md' declaring FR-{prefix}-NNN ids.",
                            folder.display(),
                            prefix = fr_prefix(&folder_name)
                        ),
                    ));
                    continue;
                }
                let contents = match std::fs::read_to_string(&frd) {
                    Ok(c) => c,
                    Err(_) => continue,
                };
                for fr_id in extract_fr_ids(&contents) {
                    let prefix = fr_prefix(&folder_name);
                    if !fr_id.starts_with(&format!("FR-{prefix}")) {
                        findings.push(StructureFinding::new(
                            consts::RULE_CODE_PR_LA_FOLDER,
                            consts::PR_LA_VIOLATION_FR_ID_MISMATCH,
                            relative_to(&frd, &ws_root),
                            format!(
                                "folder '{folder_name}' carries FR id '{fr_id}' but its prefix must be 'FR-{prefix}'"
                            ),
                        )
                        .with_reason(
                            "A crate folder's FR ids all share the folder's feature prefix so a reader can match an FR id to the crate that owns it.".to_string(),
                            format!(
                                "Rename '{fr_id}' to 'FR-{prefix}-NNN' or move the FRD into the folder that owns its prefix.",
                            ),
                        ));
                    }
                }
            }
        }

        // Part B: every PR-LA-001..N id in PRD.md / ROADMAP.md maps to a real
        // crate folder by order (PR-LA-001 → first folder), the folder exists
        // under a member dir, and the heading's title names that folder.
        let order = consts::PR_LA_FOLDER_ORDER;
        let members = utility_structure_parsers::member_dirs(&ws_root);
        for doc_name in consts::PR_LA_DOCUMENTS {
            let doc = ws_root.join(doc_name);
            if !doc.is_file() {
                continue;
            }
            let contents = match std::fs::read_to_string(&doc) {
                Ok(c) => c,
                Err(_) => continue,
            };
            for (position, pr_la_id, title) in extract_pr_la_ids(&contents) {
                let expected_folder = order.get(position).copied().unwrap_or("");
                if expected_folder.is_empty() {
                    findings.push(StructureFinding::new(
                        consts::RULE_CODE_PR_LA_FOLDER,
                        consts::PR_LA_VIOLATION_PR_LA_ID_MISMATCH,
                        relative_to(&doc, &ws_root),
                        format!("'{pr_la_id}' has no crate folder at position {position}"),
                    )
                    .with_reason(
                        "PR-LA ids map 1:1 to crate folders by order, so a position with no folder cannot be satisfied.",
                        format!(
                            "Add a crate folder for position {position}, or remove the dangling '{pr_la_id}' from '{doc_name}'."
                        ),
                    ));
                    continue;
                }
                let folder_exists = members
                    .iter()
                    .any(|member| member.join(expected_folder).is_dir());
                if !folder_exists {
                    findings.push(StructureFinding::new(
                        consts::RULE_CODE_PR_LA_FOLDER,
                        consts::PR_LA_VIOLATION_PR_LA_ID_MISMATCH,
                        relative_to(&doc, &ws_root),
                        format!(
                            "'{pr_la_id}' maps to '{expected_folder}' but no crates|modules|packages/{expected_folder} folder exists"
                        ),
                    )
                    .with_reason(
                        "A PR-LA id must point at a real crate folder, by order.",
                        format!(
                            "Create '{expected_folder}' under crates/, modules/, or packages/ for '{pr_la_id}', or renumber the PR-LA ids so '{pr_la_id}' lands on an existing folder."
                        ),
                    ));
                    continue;
                }
                if !title.is_empty() && !title_names_folder(&title, expected_folder) {
                    findings.push(StructureFinding::new(
                        consts::RULE_CODE_PR_LA_FOLDER,
                        consts::PR_LA_VIOLATION_PR_LA_ID_MISMATCH,
                        relative_to(&doc, &ws_root),
                        format!(
                            "'{pr_la_id}' titled '{title}' does not name its folder '{expected_folder}'"
                        ),
                    )
                    .with_reason(
                        "Each PR-LA heading names the crate folder it maps to, by order, so a reader can match the spec row to crates|modules|packages/<folder>.",
                        format!(
                            "Title '{pr_la_id}' with '{expected_folder}' after the colon, e.g. '### {pr_la_id}: {expected_folder}'."
                        ),
                    ));
                }
            }
        }

        StructureResponse::Findings {
            findings: sorted(findings),
        }
    }
}

// ─── Block 3: Constructors, Std Traits, Helpers ────────────────────────────

impl Default for PrLaFolderAuditor {
    fn default() -> Self {
        Self::new()
    }
}

impl PrLaFolderAuditor {
    pub fn new() -> Self {
        Self {}
    }
}

/// Resolve the workspace root by walking up from *root* until we find
/// a directory that contains at least one of the member directories.
fn workspace_root(root: &Path) -> PathBuf {
    find_workspace_root(root)
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

/// The feature prefix for a folder name: kebab-case → PascalCase, with
/// explicit overrides for abbreviation folders (doc-rules → DOC).
fn fr_prefix(folder_name: &str) -> String {
    if let Some(override_) = consts::FR_PREFIX_OVERRIDES
        .iter()
        .find(|(name, _)| *name == folder_name)
        .map(|(_, prefix)| *prefix)
    {
        return override_.to_string();
    }
    folder_name
        .split('-')
        .map(|part| {
            let mut chars = part.chars();
            match chars.next() {
                None => String::new(),
                Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
            }
        })
        .collect()
}

/// Relative path of *path* against the workspace root.
fn relative_to(path: &Path, ws_root: &Path) -> String {
    match path.strip_prefix(ws_root) {
        Ok(p) => p.to_string_lossy().replace('\\', "/"),
        Err(_) => path.to_string_lossy().replace('\\', "/"),
    }
}

/// Extract every `FR-<FEATURE>-NNN` style id from document contents.
fn extract_fr_ids(contents: &str) -> Vec<String> {
    let mut ids = Vec::new();
    for line in contents.lines() {
        // FR ids can appear mid-line (e.g. `### FR-005B`), so scan the whole line.
        let mut search_from = 0;
        while let Some(fr_pos) = line[search_from..].find("FR-") {
            let rest = &line[search_from + fr_pos + 3..];
            // Take the next token up to whitespace or punctuation that ends an id.
            let mut end = rest.len();
            for (i, ch) in rest.char_indices() {
                if ch.is_whitespace()
                    || ch == '|'
                    || ch == ':'
                    || ch == '`'
                    || ch == '('
                    || ch == ')'
                {
                    end = i;
                    break;
                }
            }
            let candidate = &rest[..end];
            // Must contain at least one digit so `FR-` alone never matches.
            if !candidate.is_empty() && candidate.chars().any(|c| c.is_ascii_digit()) {
                ids.push(format!("FR-{candidate}"));
            }
            search_from += fr_pos + 3;
        }
    }
    ids
}

/// Extract every `PR-LA-NNN` id from document contents as
/// `(position, id, title)`. The position is the id's slot in
/// `PR_LA_FOLDER_ORDER` (`PR-LA-001` → 0), so it stays stable even when an id
/// is mentioned more than once. The title is the heading text after the colon
/// (empty when the heading has none), e.g. `### PR-LA-001: Multi-language
/// scanning (Rust, Python, JS/TS)`.
fn extract_pr_la_ids(contents: &str) -> Vec<(usize, String, String)> {
    let mut ids = Vec::new();
    for line in contents.lines() {
        // PR-LA ids appear inside markdown headings (`### PR-LA-001: …`), so
        // scan the whole line rather than requiring a line prefix.
        let mut search_from = 0;
        while let Some(rel) = line[search_from..].find("PR-LA-") {
            let start = search_from + rel;
            let rest = &line[start + 6..];
            let digits: String = rest.chars().take_while(|c| c.is_ascii_digit()).collect();
            if digits.is_empty() {
                search_from = start + 6;
                continue;
            }
            let position = digits
                .parse::<usize>()
                .map(|n| n.saturating_sub(1))
                .unwrap_or(usize::MAX);
            let after = &rest[digits.len()..];
            let title_raw = after.trim_start_matches([':', '|', ' ', '\t']);
            let title = title_raw.split('|').next().unwrap_or("").trim();
            ids.push((position, format!("PR-LA-{digits}"), title.to_string()));
            search_from = start + 6 + digits.len();
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
            title_token == folder_token
                || (title_token.len() >= 3 && title_token.starts_with(folder_token.as_str()))
                || (folder_token.len() >= 3 && folder_token.starts_with(title_token.as_str()))
        })
    })
}

/// Lowercase *text* and split it into alphanumeric tokens on every separator.
fn slug_tokens(text: &str) -> Vec<String> {
    let slug: String = text
        .chars()
        .map(|c| {
            if c.is_alphanumeric() {
                c.to_ascii_lowercase()
            } else {
                '-'
            }
        })
        .collect();
    slug.split('-')
        .filter(|token| !token.is_empty())
        .map(str::to_string)
        .collect()
}
