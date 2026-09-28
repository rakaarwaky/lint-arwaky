// PURPOSE: Parses folder layout into the layer inventory the structure auditor reads
//
// AES701–AES705 classify files by their AES filename prefix and read the
// documents sitting beside the source, so the whole check is a matter of
// walking a folder and labelling each entry. This utility holds that walk; it
// performs no rule decisions and defines no types.
use std::path::{Path, PathBuf};

use shared::structure_rules::taxonomy_structure_constant::{
    AGENT_PREFIX, CAPABILITIES_PREFIX, ORCHESTRATOR_SUFFIX, SKIPPED_DIRS, SURFACE_PREFIX,
};
use shared::structure_rules::taxonomy_structure_vo::{FolderInventory, LayerFile};

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

/// Whether *member* directly holds an `agent_*_orchestrator` file. A
/// member-level orchestrator coordinates every feature folder beneath it, so
/// a feature folder that ships capabilities without its own agent is still
/// driven.
pub fn has_member_orchestrator(member: &Path) -> bool {
    let Ok(entries) = std::fs::read_dir(member) else {
        return false;
    };
    entries.flatten().any(|entry| {
        entry
            .path()
            .file_stem()
            .and_then(|n| n.to_str())
            .is_some_and(|stem| {
                stem.starts_with(AGENT_PREFIX) && stem.ends_with(ORCHESTRATOR_SUFFIX)
            })
    })
}

/// Which of *names* sit directly in *folder*. A folder's documents sit beside
/// its source, not inside `src/`, so this reads one level.
pub fn docs_present(folder: &Path, names: &[&str]) -> Vec<String> {
    let Ok(entries) = std::fs::read_dir(folder) else {
        return Vec::new();
    };
    let mut found: Vec<String> = entries
        .flatten()
        .map(|entry| entry.file_name().to_string_lossy().to_string())
        .filter(|name| names.contains(&name.as_str()))
        .collect();
    found.sort();
    found
}
