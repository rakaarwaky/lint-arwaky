// PURPOSE: Parses folder layout into the layer inventory the structure auditor reads
//
// AES701–AES703 classify files by their AES filename prefix and read the
// documents sitting beside the source, so the whole check is a matter of
// walking a folder and labelling each entry. This utility holds that walk; it
// performs no rule decisions and defines no types.
use std::path::{Path, PathBuf};

use super::taxonomy_structure_rules_constant::{
    AGENT_PREFIX, CAPABILITIES_PREFIX, ORCHESTRATOR_SUFFIX, SKIPPED_DIRS, SURFACE_PREFIX,
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

        inventory.has_orchestrator |= is_orchestrator;
        inventory.has_capabilities |= is_capability;
        inventory.has_surfaces |= is_surface;
        inventory.surface_count += usize::from(is_surface);
        inventory.total_count += 1;
        inventory.files.push(LayerFile {
            name: name.to_string(),
            path,
            stem,
        });
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
