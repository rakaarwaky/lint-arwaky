// PURPOSE: SurfacesOrphanAnalyzer — ISurfacesOrphanProtocol for orphan surface detection
use shared::cli_commands::taxonomy_severity_vo::Severity;
use shared::code_analysis::taxonomy_analysis_vo::OrphanIndicatorResult;
use shared::code_analysis::taxonomy_analysis_vo::ReachabilityResult;
use shared::common::taxonomy_path_vo::FilePath;
use shared::orphan_detector::contract_orphan_protocol::ISurfacesOrphanProtocol;
use shared::orphan_detector::taxonomy_violation_orphan_vo::AesOrphanViolation;
use shared::orphan_detector::utility_orphan_filename::{file_basename, file_stem, file_suffix};
use shared::taxonomy_definition_vo::LayerDefinition;

// ─── Block 1: Struct Definition ───────────────────────────

pub struct SurfacesOrphanAnalyzer {}

// ─── Block 2: Protocol Trait Implementation ───────────────

impl ISurfacesOrphanProtocol for SurfacesOrphanAnalyzer {
    fn is_surface_orphan(
        &self,
        f: &FilePath,
        alive_files: &ReachabilityResult,
        _definition: Option<&LayerDefinition>,
    ) -> OrphanIndicatorResult {
        let alive: Vec<String> = alive_files
            .paths
            .iter()
            .map(|fp| fp.value().to_string())
            .collect();
        let is_reachable = alive.contains(&f.value().to_string());
        if is_reachable {
            return OrphanIndicatorResult::new(false, String::new(), Severity::LOW);
        }

        // Check if imported by entry or router
        let fp_val = f.value();
        let basename = file_basename(fp_val);
        let stem = file_stem(fp_val);

        if let Ok(content) = std::fs::read_to_string(fp_val) {
            // Check if this surface is imported by any entry or router file
            let root = std::path::Path::new(".");
            if let Ok(workspace_root) =
                shared::orphan_detector::utility_workspace::find_workspace_root(root)
            {
                if let Ok(imported) = Self::check_imported_by_entry_or_router(&workspace_root, &stem) {
                    if imported {
                        return OrphanIndicatorResult::new(false, String::new(), Severity::LOW);
                    }
                }

                // Also check for function/struct names from the surface file
                let mut identifiers: Vec<String> = Vec::new();
                // Extract pub fn names
                for line in content.lines() {
                    let trimmed = line.trim();
                    if trimmed.starts_with("pub fn ") {
                        if let Some(name) = trimmed.strip_prefix("pub fn ") {
                            if let Some(name) = name.split('(').next() {
                                identifiers.push(name.trim().to_string());
                            }
                        }
                    }
                    // Extract pub struct names
                    if trimmed.starts_with("pub struct ") {
                        if let Some(name) = trimmed.strip_prefix("pub struct ") {
                            if let Some(name) = name.split('{').next() {
                                identifiers.push(name.trim().to_string());
                            }
                        }
                    }
                }

                for id in &identifiers {
                    if let Ok(imported) = Self::check_imported_by_entry_or_router(&workspace_root, id) {
                        if imported {
                            return OrphanIndicatorResult::new(false, String::new(), Severity::LOW);
                        }
                    }
                }
            }
        }

        let suffix = Self::get_surface_suffix(&basename);
        let category = Self::surface_category(&suffix);
        OrphanIndicatorResult::new(
            true,
            AesOrphanViolation::SurfaceOrphan {
                category,
                stem,
                reason: Some("Surface is unreachable.".into()),
            }
            .to_string(),
            Severity::HIGH,
        )
    }
}

// ─── Block 3: Constructors, Helpers, Private Methods ──────

impl Default for SurfacesOrphanAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

impl SurfacesOrphanAnalyzer {
    pub fn new() -> Self {
        Self {}
    }

    fn check_imported_by_entry_or_router(
        workspace_root: &std::path::Path,
        stem: &str,
    ) -> Result<bool, std::io::Error> {
        for dir_name in &["crates", "packages", "modules"] {
            let dir = workspace_root.join(dir_name);
            if dir.is_dir() && Self::check_dir_imports(&dir, stem)? {
                return Ok(true);
            }
        }
        Ok(false)
    }

    fn check_dir_imports(dir: &std::path::Path, stem: &str) -> Result<bool, std::io::Error> {
        if let Ok(entries) = std::fs::read_dir(dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_dir() {
                    if Self::check_dir_imports(&path, stem)? {
                        return Ok(true);
                    }
                } else if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                    let is_entry_or_router = name.starts_with("root_")
                        || name.starts_with("cli_")
                        || name.starts_with("mcp_")
                        || name.contains("_entry")
                        || name.contains("_router");
                    if is_entry_or_router
                        && (name.ends_with(".rs")
                            || name.ends_with(".py")
                            || name.ends_with(".ts")
                            || name.ends_with(".js"))
                    {
                        if let Ok(content) = std::fs::read_to_string(&path) {
                            if content.contains(stem) {
                                return Ok(true);
                            }
                        }
                    }
                }
            }
        }
        Ok(false)
    }

    /// Get surface suffix from filename
    fn get_surface_suffix(basename: &str) -> String {
        file_suffix(basename)
    }

    /// Surface category
    fn surface_category(suffix: &str) -> &'static str {
        match suffix {
            "command" | "controller" | "page" => "smart",
            "hook" | "store" | "action" | "screen" | "router" => "utility",
            "component" | "view" | "layout" => "passive",
            _ => "unknown",
        }
    }
}
