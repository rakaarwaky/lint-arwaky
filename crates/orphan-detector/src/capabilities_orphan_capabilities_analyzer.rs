// PURPOSE: CapabilitiesOrphanAnalyzer — ICapabilitiesOrphanProtocol for orphan capability detection
use shared::cli_commands::taxonomy_severity_vo::Severity;
use shared::code_analysis::taxonomy_analysis_vo::OrphanIndicatorResult;
use shared::code_analysis::taxonomy_analysis_vo::ReachabilityResult;
use shared::common::taxonomy_path_vo::FilePath;
use shared::orphan_detector::contract_orphan_protocol::ICapabilitiesOrphanProtocol;
use shared::orphan_detector::taxonomy_violation_orphan_vo::AesOrphanViolation;
use shared::orphan_detector::utility_file_cache;
use shared::orphan_detector::utility_orphan::{extract_struct_names, extract_trait_names};
use shared::orphan_detector::utility_orphan_filename::file_stem;
use shared::orphan_detector::utility_workspace::{check_wired_in_container, find_workspace_root};
use std::sync::Mutex;

// ─── Block 1: Struct Definition ───────────────────────────

pub struct CapabilitiesOrphanAnalyzer {
    container_cache: Mutex<Option<(std::path::PathBuf, Vec<std::path::PathBuf>)>>,
}

// ─── Block 2: Protocol Trait Implementation ───────────────

impl ICapabilitiesOrphanProtocol for CapabilitiesOrphanAnalyzer {
    fn is_capabilities_orphan(
        &self,
        f: &FilePath,
        root_dir: &FilePath,
        alive_files: &ReachabilityResult,
    ) -> OrphanIndicatorResult {
        let is_reachable = alive_files.paths.contains(f);
        if is_reachable {
            return OrphanIndicatorResult::new(false, String::new(), Severity::LOW);
        }

        // Check if wired in any container
        let fp = f.value();
        let stem = file_stem(fp);

        if !fp.is_empty() {
            let path = FilePath::new(fp).unwrap_or_default();
            let content = utility_file_cache::read_cached(&path);
            let mut identifiers: Vec<String> = Vec::new();
            identifiers.extend(extract_struct_names(&content.value()));
            identifiers.extend(extract_trait_names(&content.value()));
            identifiers.push(stem.clone());

            let pascal_stem: String = stem
                .split('_')
                .filter(|s| !s.is_empty())
                .map(|s| {
                    let mut c = s.chars();
                    match c.next() {
                        Some(f) => f.to_uppercase().to_string() + c.as_str(),
                        None => String::new(),
                    }
                })
                .collect();
            identifiers.push(pascal_stem);

            // Search for container files in workspace root (cached)
            let root = std::path::Path::new(root_dir.value());
            if let Ok(workspace_root) = find_workspace_root(root) {
                let _container_files = self.cached_container_files(&workspace_root);
                let wired = check_wired_in_container(&workspace_root, &identifiers);
                if wired {
                    return OrphanIndicatorResult::new(false, String::new(), Severity::LOW);
                }
            }
        }

        OrphanIndicatorResult::new(
            true,
            AesOrphanViolation::CapabilitiesOrphan {
                stem,
                reason: Some("Not reachable from any entry point.".into()),
            }
            .to_string(),
            Severity::MEDIUM,
        )
    }
}

// ─── Block 3: Constructors, Helpers, Private Methods ──────

impl Default for CapabilitiesOrphanAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

impl CapabilitiesOrphanAnalyzer {
    pub fn new() -> Self {
        Self {
            container_cache: Mutex::new(None),
        }
    }

    fn cached_container_files(
        &self,
        workspace_root: &std::path::Path,
    ) -> Option<Vec<std::path::PathBuf>> {
        if let Ok(mut guard) = self.container_cache.lock() {
            if let Some((ref cached_root, ref cached_files)) = *guard {
                if cached_root == workspace_root {
                    return Some(cached_files.clone());
                }
            }
            // Cache miss: find container files
            let mut container_files = Vec::new();
            for dir_name in &["crates", "packages", "modules"] {
                let dir = workspace_root.join(dir_name);
                if dir.is_dir() {
                    let files =
                        shared::orphan_detector::utility_orphan_io::scan_directory_recursive(&dir);
                    for file_path in &files {
                        if let Some(name) = std::path::Path::new(file_path)
                            .file_name()
                            .and_then(|n| n.to_str())
                        {
                            if name.ends_with("_container.rs")
                                || name.ends_with("_container.py")
                                || name.ends_with("_container.ts")
                                || name.ends_with("_container.js")
                            {
                                container_files.push(std::path::PathBuf::from(file_path));
                            }
                        }
                    }
                }
            }
            *guard = Some((workspace_root.to_path_buf(), container_files.clone()));
            Some(container_files)
        } else {
            None
        }
    }
}
