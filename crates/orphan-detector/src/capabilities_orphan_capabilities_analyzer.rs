use shared::cli_commands::taxonomy_severity_vo::Severity;
use shared::code_analysis::taxonomy_analysis_vo::OrphanIndicatorResult;
use shared::code_analysis::taxonomy_analysis_vo::ReachabilityResult;
use shared::common::taxonomy_path_vo::FilePath;
use shared::orphan_detector::contract_orphan_protocol::{
    ICapabilitiesOrphanProtocol, IOrphanFileCachePort, IOrphanFilenameExtractorProtocol,
};
use shared::orphan_detector::taxonomy_violation_orphan_vo::AesOrphanViolation;
use std::sync::Arc;

// ─── Block 1: Struct Definition ───────────────────────────
pub struct CapabilitiesOrphanAnalyzer {
    extractor: Arc<dyn IOrphanFilenameExtractorProtocol>,
    cache: Arc<dyn IOrphanFileCachePort>,
}

// ─── Block 2: Public Contract (domain protocol ONLY) ──────
impl ICapabilitiesOrphanProtocol for CapabilitiesOrphanAnalyzer {
    fn is_capabilities_orphan(
        &self,
        f: &FilePath,
        root_dir: &FilePath,
        alive_files: &ReachabilityResult,
    ) -> OrphanIndicatorResult {
        self.check_capabilities_orphan(f, root_dir, alive_files)
    }
}

// ─── Block 3: Constructors, Std Traits & Helpers ─────────
impl CapabilitiesOrphanAnalyzer {
    pub fn new(
        extractor: Arc<dyn IOrphanFilenameExtractorProtocol>,
        cache: Arc<dyn IOrphanFileCachePort>,
    ) -> Self {
        Self { extractor, cache }
    }

    fn check_capabilities_orphan(
        &self,
        f: &FilePath,
        root_dir: &FilePath,
        alive_files: &ReachabilityResult,
    ) -> OrphanIndicatorResult {
        let is_reachable = alive_files.paths.contains(f);
        if is_reachable {
            return OrphanIndicatorResult::new(false, String::new(), Severity::LOW);
        }

        let stem = self.extractor.file_stem(f).value;
        let content = self.cache.read_cached(f).value;

        if !content.is_empty() {
            let mut identifiers: Vec<String> = Vec::new();
            identifiers.extend(
                self.extractor
                    .extract_struct_names(&content)
                    .into_iter()
                    .map(|sn| sn.value),
            );
            identifiers.extend(
                self.extractor
                    .extract_trait_names(&content)
                    .into_iter()
                    .map(|sn| sn.value),
            );
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

            let root = std::path::Path::new(root_dir.value());
            if let Some(workspace_root) = self.find_workspace_root(root) {
                if self.check_wired_in_container(&workspace_root, &identifiers) {
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

    fn find_workspace_root(&self, start: &std::path::Path) -> Option<std::path::PathBuf> {
        let member_dirs = ["crates", "packages", "modules"];
        let mut current = start.to_path_buf();
        loop {
            let has_cargo = current.join("Cargo.toml").exists();
            let has_package_json = current.join("package.json").exists();
            let has_pyproject = current.join("pyproject.toml").exists();
            let has_member_dir = member_dirs.iter().any(|d| current.join(d).is_dir());

            if has_member_dir && (has_cargo || has_package_json || has_pyproject) {
                return Some(current);
            }

            if !current.pop() {
                return None;
            }
        }
    }

    fn check_wired_in_container(
        &self,
        workspace_root: &std::path::Path,
        identifiers: &[String],
    ) -> bool {
        for dir_name in &["crates", "packages", "modules"] {
            let dir = workspace_root.join(dir_name);
            if dir.is_dir() && self.check_dir_containers(&dir, identifiers) {
                return true;
            }
        }
        false
    }

    fn check_dir_containers(&self, dir: &std::path::Path, identifiers: &[String]) -> bool {
        let entries = self.cache.read_dir(dir.to_str().unwrap_or(""));
        for entry_path in &entries {
            let path = std::path::Path::new(entry_path);
            if path.is_dir() {
                if self.check_dir_containers(path, identifiers) {
                    return true;
                }
            } else if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                if name.ends_with("_container.rs")
                    || name.ends_with("_container.py")
                    || name.ends_with("_container.ts")
                    || name.ends_with("_container.js")
                    || name.ends_with("_entry.rs")
                    || name.ends_with("_entry.py")
                    || name.ends_with("_entry.ts")
                    || name.ends_with("_entry.js")
                {
                    let fp = FilePath {
                        value: entry_path.clone(),
                    };
                    let content = self.cache.read_cached(&fp).value;
                    for id in identifiers {
                        if content.contains(id) {
                            return true;
                        }
                    }
                }
            }
        }
        false
    }
}
