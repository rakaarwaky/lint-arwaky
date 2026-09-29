// PURPOSE: StructureVO — folder-layout value objects for AES701–AES703
//
// A workspace folder inventory is data the auditor reads and labels, so it
// lives in the taxonomy layer. The parse helpers in the utility layer build
// these; no rule decision happens here.
use std::path::PathBuf;

/// A source file classified by its AES layer prefix.
#[derive(Clone, Debug)]
pub struct LayerFile {
    /// File name including its extension.
    pub name: String,
    /// Full path to the file.
    pub path: PathBuf,
    /// File stem, the name the layer prefix is read from.
    pub stem: String,
}

impl LayerFile {
    /// The file path relative to *ws_root*, so a finding names the offending
    /// file with its full sub-directory path (e.g. `crates/shared/src/…`).
    pub fn rel(&self, ws_root: &std::path::Path) -> String {
        self.path
            .strip_prefix(ws_root)
            .map(|p| p.to_string_lossy().replace('\\', "/"))
            .unwrap_or_else(|_| self.name.clone())
    }
}

/// What layers a folder actually holds.
#[derive(Clone, Debug, Default)]
pub struct FolderInventory {
    /// Every classified file, sorted by name for stable output.
    pub files: Vec<LayerFile>,
    /// Does the folder hold at least one `agent_*_orchestrator` file.
    pub has_orchestrator: bool,
    /// Does the folder hold at least one `capabilities_*` file.
    pub has_capabilities: bool,
    /// Does the folder hold at least one `surface_*` file.
    pub has_surfaces: bool,
    /// How many `surface_*` files the folder holds, used to spot a surface folder.
    pub surface_count: usize,
    /// How many classified files the folder holds in total.
    pub total_count: usize,
}

impl FolderInventory {
    /// Whether the folder is dominated by surface files — the signal that it
    /// is a surface folder rather than a feature folder.
    pub fn is_surface_dominated(&self) -> bool {
        self.surface_count > 0 && self.surface_count * 2 >= self.total_count
    }
}
