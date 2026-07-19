// PURPOSE: SurfacesOrphanAnalyzer — ISurfacesOrphanProtocol for orphan surface detection
use shared::cli_commands::taxonomy_result_vo::LintResult;
use shared::cli_commands::taxonomy_severity_vo::Severity;
use shared::code_analysis::taxonomy_analysis_vo::OrphanIndicatorResult;
use shared::code_analysis::taxonomy_analysis_vo::ReachabilityResult;
use shared::common::taxonomy_path_vo::FilePath;
use shared::orphan_detector::contract_orphan_protocol::{
    IOrphanFileCachePort, IOrphanFilenameExtractorProtocol, ISurfacesOrphanProtocol,
};
use shared::orphan_detector::taxonomy_orphan_result_utility::mk_orphan_result;
use shared::orphan_detector::taxonomy_surface_utility::{get_surface_suffix, surface_category};
use shared::orphan_detector::taxonomy_violation_orphan_vo::AesOrphanViolation;
use shared::orphan_detector::taxonomy_workspace_utility::find_workspace_root;
use shared::taxonomy_definition_vo::LayerDefinition;
use std::sync::Arc;

// ─── Block 1: Struct Definition ───────────────────────────
pub struct SurfacesOrphanAnalyzer {
    extractor: Arc<dyn IOrphanFilenameExtractorProtocol>,
    cache: Arc<dyn IOrphanFileCachePort>,
}

// ─── Block 2: Public Contract (domain protocol ONLY) ──────
impl ISurfacesOrphanProtocol for SurfacesOrphanAnalyzer {
    fn is_surface_orphan(
        &self,
        f: &FilePath,
        alive_files: &ReachabilityResult,
        definition: Option<&LayerDefinition>,
    ) -> OrphanIndicatorResult {
        self.check_surface_orphan(f, alive_files, definition)
    }
}

// ─── Block 3: Constructors, Std Traits & Helpers ─────────
impl SurfacesOrphanAnalyzer {
    pub fn new(
        extractor: Arc<dyn IOrphanFilenameExtractorProtocol>,
        cache: Arc<dyn IOrphanFileCachePort>,
    ) -> Self {
        Self { extractor, cache }
    }

    fn check_surface_orphan(
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

        let fp_val = f.value();
        let basename = self.extractor.file_basename(f).value;
        let stem = self.extractor.file_stem(f).value;

        let content = self.cache.read_cached(f).value;
        if !content.is_empty() {
            let file_parent = std::path::Path::new(fp_val)
                .parent()
                .unwrap_or(std::path::Path::new("."));
            if let Ok(workspace_root) = find_workspace_root(file_parent) {
                if self.check_imported_by_entry_or_router(&workspace_root, &stem) {
                    return OrphanIndicatorResult::new(false, String::new(), Severity::LOW);
                }

                let mut identifiers: Vec<String> = Vec::new();
                for line in content.lines() {
                    let trimmed = line.trim();
                    if trimmed.starts_with("pub fn ") {
                        if let Some(name) = trimmed.strip_prefix("pub fn ") {
                            if let Some(name) = name.split('(').next() {
                                identifiers.push(name.trim().to_string());
                            }
                        }
                    }
                    if trimmed.starts_with("pub struct ") {
                        if let Some(name) = trimmed.strip_prefix("pub struct ") {
                            if let Some(name) = name.split('{').next() {
                                identifiers.push(name.trim().to_string());
                            }
                        }
                    }
                }

                for id in &identifiers {
                    if self.check_imported_by_entry_or_router(&workspace_root, id) {
                        return OrphanIndicatorResult::new(false, String::new(), Severity::LOW);
                    }
                }
            }
        }

        let suffix = get_surface_suffix(&basename, &self.extractor);
        let category = surface_category(&suffix);
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

    fn check_imported_by_entry_or_router(
        &self,
        workspace_root: &std::path::Path,
        stem: &str,
    ) -> bool {
        for dir_name in &["crates", "packages", "modules"] {
            let dir = workspace_root.join(dir_name);
            if dir.is_dir() && self.check_dir_imports(&dir, stem) {
                return true;
            }
        }
        false
    }

    fn check_dir_imports(&self, dir: &std::path::Path, stem: &str) -> bool {
        if let Ok(fp) = shared::common::taxonomy_path_vo::FilePath::new(dir.to_str().unwrap_or("")) {
            let entries = self.cache.read_dir(&fp);
            for entry_path in &entries {
                let path = std::path::Path::new(entry_path);
                if path.is_dir() {
                    if self.check_dir_imports(path, stem) {
                        return true;
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
                        let fp = FilePath {
                            value: entry_path.clone(),
                        };
                        let content = self.cache.read_cached(&fp).value;
                        if content.contains(stem) {
                            return true;
                        }
                    }
                }
            }
        }
        false
    }

    /// Batch entry point for surfaces orphan detection from a flat file list.
    /// Concrete implementation of the older `check_surfaces_orphan` batch path,
    /// rehomed as an inherent method (no `agent_*` import).
    pub fn check_surfaces_orphan(
        &self,
        fp: &str,
        all_files: &[String],
        violations: &mut Vec<LintResult>,
    ) {
        let fp_vo = match FilePath::new(fp.to_string()) {
            Ok(p) => p,
            Err(_) => return,
        };
        let basename = self.extractor.file_basename(&fp_vo).value;
        let suffix = get_surface_suffix(&basename, &self.extractor);
        let category = surface_category(&suffix);
        let stem = self.extractor.file_stem(&fp_vo).value;

        let mut is_orphan = false;
        let mut reason = String::new();

        match category {
            "smart" => {
                let mut imported = false;
                for cf in all_files {
                    let cb = self
                        .extractor
                        .file_basename(&FilePath { value: cf.clone() })
                        .value;
                    let cf_suffix = get_surface_suffix(&cb, &self.extractor);
                    if cb.starts_with("cli_")
                        || cb.starts_with("mcp_")
                        || cf_suffix == "entry"
                        || cf_suffix == "router"
                    {
                        let cf_content = self
                            .cache
                            .read_cached(&FilePath { value: cf.clone() })
                            .value;
                        if cf_content.contains(&stem) {
                            imported = true;
                            break;
                        }
                    }
                }
                if !imported {
                    is_orphan = true;
                    reason = format!(
                        "Smart surface '{}' not imported by any entry point or router.",
                        stem
                    );
                }
            }
            "utility" => {
                let mut imported = false;
                for cf in all_files {
                    let cb = self
                        .extractor
                        .file_basename(&FilePath { value: cf.clone() })
                        .value;
                    let cf_suffix = get_surface_suffix(&cb, &self.extractor);
                    if surface_category(&cf_suffix) == "smart" {
                        let cf_content = self
                            .cache
                            .read_cached(&FilePath { value: cf.clone() })
                            .value;
                        if cf_content.contains(&stem) {
                            imported = true;
                            break;
                        }
                    }
                }
                if !imported {
                    is_orphan = true;
                    reason = format!(
                        "Utility surface '{}' not imported by any smart surface.",
                        stem
                    );
                }
            }
            "passive" => {
                let mut imported = false;
                for cf in all_files {
                    let cb = self
                        .extractor
                        .file_basename(&FilePath { value: cf.clone() })
                        .value;
                    let cf_suffix = get_surface_suffix(&cb, &self.extractor);
                    let cat = surface_category(&cf_suffix);
                    if cat == "smart" || cat == "utility" {
                        let cf_content = self
                            .cache
                            .read_cached(&FilePath { value: cf.clone() })
                            .value;
                        if cf_content.contains(&stem) {
                            imported = true;
                            break;
                        }
                    }
                }
                if !imported {
                    is_orphan = true;
                    reason = format!(
                        "Passive surface '{}' not imported by any smart or utility surface.",
                        stem
                    );
                }
            }
            _ => {}
        }

        if is_orphan {
            violations.push(mk_orphan_result(
                fp,
                &AesOrphanViolation::SurfaceOrphan {
                    category,
                    stem,
                    reason: Some(reason.into()),
                }
                .to_string(),
                Severity::HIGH,
                "AES506",
            ));
        }
    }
}
