// PURPOSE: Parses folder layout into the layer inventory the structure auditor reads
//
// AES701–AES703 classify files by their AES filename prefix and read the
// documents sitting beside the source, so the whole check is a matter of
// walking a folder and labelling each entry. This utility holds that walk; it
// performs no rule decisions and defines no types.
use std::path::{Path, PathBuf};

use super::taxonomy_structure_rules_constant::{
    AGENT_PREFIX, CAPABILITIES_PREFIX, ORCHESTRATOR_SUFFIX, ROOT_PREFIX, SKIPPED_DIRS,
    SURFACE_PREFIX,
};
use super::taxonomy_structure_rules_request::StructureFinding;
use super::taxonomy_structure_rules_vo::{FolderInventory, LayerFile};

/// How deep the walk descends. Feature folders nest one level under `src/`.
const MAX_DEPTH: usize = 3;

/// Classify every file under *dir*, skipping fixture and build directories.
pub fn inventory(dir: &Path) -> FolderInventory {
    let mut inventory = FolderInventory::default();
    collect_files(dir, &mut inventory, 0);
    inventory.files.sort_by(|a, b| a.name.cmp(&b.name));
    inventory
}

/// Recursive helper for `inventory`.
fn collect_files(dir: &Path, inventory: &mut FolderInventory, depth: usize) {
    if depth > MAX_DEPTH {
        return;
    }
    let Ok(entries) = std::fs::read_dir(dir) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            let name = path
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or_default();
            if SKIPPED_DIRS.contains(&name) {
                continue;
            }
            collect_files(&path, inventory, depth + 1);
            continue;
        }
        let Some(name) = path.file_name().and_then(|n| n.to_str()) else {
            continue;
        };
        let stem = path
            .file_stem()
            .and_then(|n| n.to_str())
            .unwrap_or_default()
            .to_string();

        let is_orchestrator = stem.starts_with(AGENT_PREFIX) && stem.ends_with(ORCHESTRATOR_SUFFIX);
        let is_capability = stem.starts_with(CAPABILITIES_PREFIX);
        let is_surface = stem.starts_with(SURFACE_PREFIX);
        let is_root = stem.starts_with(ROOT_PREFIX);

        inventory.has_orchestrator |= is_orchestrator;
        inventory.has_capabilities |= is_capability;
        inventory.has_surfaces |= is_surface;
        inventory.has_root |= is_root;
        inventory.surface_count += usize::from(is_surface);
        inventory.total_count += 1;
        inventory.files.push(LayerFile {
            name: name.to_string(),
            path,
            stem,
        });
    }
}

/// Find the workspace root by walking up from *root* until we find
/// a directory that contains at least one of the member directories.
///
/// When *root* is itself a member directory (basename is `crates`, `modules`,
/// or `packages`), the workspace root is its **parent** — the parent is the
/// dir that contains `crates|modules|packages`. This lets a per-member scan
/// resolve the same ws_root as the whole-tree scan, so `member_dirs(ws_root)`
/// returns the full set and the scope filter keeps only the target member.
pub fn find_workspace_root(root: &Path) -> PathBuf {
    let base_name = root
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or_default();
    if matches!(base_name, "crates" | "modules" | "packages") {
        return root
            .parent()
            .map(|p| p.to_path_buf())
            .unwrap_or_else(|| root.to_path_buf());
    }
    let mut dir = root.to_path_buf();
    loop {
        if dir.join("crates").is_dir()
            || dir.join("modules").is_dir()
            || dir.join("packages").is_dir()
        {
            return dir;
        }
        match dir.parent() {
            Some(parent) => dir = parent.to_path_buf(),
            None => return root.to_path_buf(),
        }
    }
}

/// True when *root* is itself a workspace member directory (`crates`,
/// `modules`, or `packages`). Used by auditors to scope their walk to the
/// target member rather than all members under the workspace root.
pub fn is_member_dir(root: &Path) -> bool {
    matches!(
        root.file_name().and_then(|n| n.to_str()),
        Some("crates" | "modules" | "packages")
    )
}

/// The member directories an audit should walk under *ws_root*, scoped to
/// the target *root* when it is itself a member directory.
///
/// When *root* is a member dir (`crates`, `modules`, or `packages`), the
/// audit is scoped to that single member — a per-member scan should only
/// surface structure findings inside the target member, not in sibling
/// members. When *root* is the workspace root (or any dir that is not a
/// member dir), all members under *ws_root* are walked, matching the
/// whole-tree scan behaviour.
/// Pick the member directories an audit of *root* must walk. A member-dir
/// target audits its own layout only — a scan of `crates` must not report
/// findings from `modules` or `packages`; any other target audits every
/// member layout under its workspace root.
pub fn scoped_member_dirs(root: &Path, ws_root: &Path) -> Vec<PathBuf> {
    if is_member_dir(root) {
        vec![root.to_path_buf()]
    } else {
        member_dirs(ws_root)
    }
}

/// The base an audit of *root* anchors its findings against. A member-dir
/// target keeps its own path (its findings read `crates/feature/...`
/// relative to the workspace parent) while every other target anchors at
/// its own path, so `collect_structure` can join one base onto either.
pub fn findings_base(root: &Path) -> PathBuf {
    if is_member_dir(root) {
        root.parent()
            .map(PathBuf::from)
            .unwrap_or_else(|| root.to_path_buf())
    } else {
        root.to_path_buf()
    }
}

/// The workspace member directories that exist under *root*.
pub fn member_dirs(root: &Path) -> Vec<PathBuf> {
    ["crates", "modules", "packages"]
        .iter()
        .map(|name| root.join(name))
        .filter(|path| path.is_dir())
        .collect()
}

/// The immediate subdirectories of a member dir, each a candidate feature folder.
pub fn feature_dirs(member: &Path) -> Vec<PathBuf> {
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

/// The subdirectory names under *dir*, empty when the directory is unreadable.
/// `SKIPPED_DIRS` are excluded, so test/build artefacts never read as a
/// mirrored feature subfolder.
pub fn subdirectory_names(dir: &Path) -> Vec<String> {
    let Ok(entries) = std::fs::read_dir(dir) else {
        return Vec::new();
    };
    let mut names: Vec<String> = entries
        .flatten()
        .filter(|entry| entry.path().is_dir())
        .filter_map(|entry| entry.file_name().to_str().map(|s| s.to_string()))
        .filter(|name| !SKIPPED_DIRS.contains(&name.as_str()))
        .collect();
    names.sort();
    names
}

/// Collect unique findings, sorted for stable output.
pub fn sorted(findings: Vec<StructureFinding>) -> Vec<StructureFinding> {
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
