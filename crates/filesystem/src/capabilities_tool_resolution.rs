// FR-004: Tool Resolution — Capabilities layer
// Implements IToolResolutionProtocol by delegating to utility_tool_resolution stateless functions.
// 3-block structure per AES skill.

use crate::utility_tool_resolution;
use shared::common::taxonomy_path_vo::FilePath;
use shared::filesystem::contract_tool_resolution_protocol::IToolResolutionProtocol;
use shared::filesystem::taxonomy_filesystem_vo::ToolName;
use std::path::Path;

// ─── Block 1: Struct Definition ───────────────────────────

pub struct CapabilitiesToolResolution;

impl CapabilitiesToolResolution {
    pub fn new() -> Self {
        Self
    }
}

// ─── Block 2: Public Contract (domain protocol ONLY) ──────

impl IToolResolutionProtocol for CapabilitiesToolResolution {
    fn is_executable_in_path(&self, executable: &ToolName) -> bool {
        utility_tool_resolution::is_executable_in_path(&executable.value)
    }

    fn is_binary_available(&self, bin_name: &ToolName) -> bool {
        utility_tool_resolution::is_binary_available(&bin_name.value)
    }

    fn has_local_bin(&self, working_dir: &Path, executable: &ToolName) -> bool {
        utility_tool_resolution::has_local_bin(working_dir, &executable.value)
    }

    fn resolve_js_cmd(
        &self,
        executable: &ToolName,
        args: Vec<String>,
        working_dir: &FilePath,
    ) -> Option<Vec<String>> {
        utility_tool_resolution::resolve_js_cmd(&executable.value, args, &working_dir.value)
    }

    fn resolve_js_working_dir(&self, path: &FilePath) -> FilePath {
        let resolved =
            utility_tool_resolution::resolve_js_working_dir(std::path::Path::new(&path.value));
        FilePath::new(resolved.to_string_lossy().to_string()).unwrap_or_default()
    }

    fn resolve_cargo_working_dir(&self, path: &FilePath) -> FilePath {
        let resolved = utility_tool_resolution::resolve_cargo_working_dir(&path.value);
        FilePath::new(resolved.to_string_lossy().to_string()).unwrap_or_default()
    }

    fn resolve_cargo_lock_working_dir(&self, path: &FilePath) -> FilePath {
        let resolved = utility_tool_resolution::resolve_cargo_lock_working_dir(&path.value);
        FilePath::new(resolved.to_string_lossy().to_string()).unwrap_or_default()
    }

    fn has_config_file(&self, dir: &Path) -> bool {
        utility_tool_resolution::has_config_file(dir)
    }

    fn has_cargo_toml(&self, path: &FilePath) -> Option<FilePath> {
        utility_tool_resolution::has_cargo_toml(&path.value)
            .map(|s| FilePath::new(s).unwrap_or_default())
    }

    fn has_cargo_lock(&self, path: &FilePath) -> Option<FilePath> {
        utility_tool_resolution::has_cargo_lock(&path.value)
            .map(|s| FilePath::new(s).unwrap_or_default())
    }

    fn is_python_file_recursive(&self, path: &FilePath) -> bool {
        utility_tool_resolution::has_python_files_recursive(std::path::Path::new(&path.value))
    }

    fn default_working_dir(&self, path: &FilePath) -> FilePath {
        let resolved =
            utility_tool_resolution::default_working_dir(std::path::Path::new(&path.value));
        FilePath::new(resolved.to_string_lossy().to_string()).unwrap_or_default()
    }
}

// ─── Block 3: Constructors, Std Traits & Helpers ─────────

impl Default for CapabilitiesToolResolution {
    fn default() -> Self {
        Self::new()
    }
}
