// PURPOSE: CapabilitiesOrphanAnalyzer — ICapabilitiesOrphanProtocol for orphan capability detection
use shared::code_analysis::{OrphanIndicatorResult, ReachabilityResult};

use shared::common::{FilePath, Severity};

use shared::orphan_detector::{AesOrphanViolation, ICapabilitiesOrphanProtocol};

use shared::orphan_detector::utility_file_cache;
use shared::orphan_detector::utility_orphan_detector::{extract_struct_names, extract_trait_names};
use shared::orphan_detector::utility_orphan_filename::file_stem;
use shared::orphan_detector::utility_workspace_scanner::{
    check_wired_in_container, find_workspace_root,
};

// ─── Block 1: Struct Definition ───────────────────────────

pub struct CapabilitiesOrphanAnalyzer {}

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
            let content_ref = content.value();
            identifiers.extend(extract_struct_names(content_ref));
            identifiers.extend(extract_trait_names(content_ref));
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
        Self {}
    }
}
