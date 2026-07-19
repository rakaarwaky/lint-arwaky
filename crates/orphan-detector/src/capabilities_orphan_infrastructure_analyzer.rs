use shared::cli_commands::taxonomy_severity_vo::Severity;
use shared::code_analysis::taxonomy_analysis_vo::OrphanIndicatorResult;
use shared::code_analysis::taxonomy_analysis_vo::ReachabilityResult;
use shared::common::taxonomy_path_vo::FilePath;
use shared::orphan_detector::contract_orphan_protocol::{
    IInfrastructureOrphanProtocol, IOrphanFileCachePort, IOrphanFilenameExtractorProtocol,
};
use shared::orphan_detector::taxonomy_violation_orphan_vo::AesOrphanViolation;
use shared::orphan_detector::taxonomy_workspace_utility::{
    check_wired_in_container, find_workspace_root,
};
use std::sync::Arc;

// ─── Block 1: Struct Definition ───────────────────────────
pub struct InfrastructureOrphanAnalyzer {
    extractor: Arc<dyn IOrphanFilenameExtractorProtocol>,
    cache: Arc<dyn IOrphanFileCachePort>,
}

// ─── Block 2: Public Contract (domain protocol ONLY) ──────
impl IInfrastructureOrphanProtocol for InfrastructureOrphanAnalyzer {
    fn is_infrastructure_orphan(
        &self,
        f: &FilePath,
        root_dir: &FilePath,
        alive_files: &ReachabilityResult,
    ) -> OrphanIndicatorResult {
        self.check_infrastructure_orphan(f, root_dir, alive_files)
    }
}

// ─── Block 3: Constructors, Std Traits & Helpers ─────────
impl InfrastructureOrphanAnalyzer {
    pub fn new(
        extractor: Arc<dyn IOrphanFilenameExtractorProtocol>,
        cache: Arc<dyn IOrphanFileCachePort>,
    ) -> Self {
        Self { extractor, cache }
    }

    fn check_infrastructure_orphan(
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
            if let Ok(workspace_root) = find_workspace_root(root) {
                if check_wired_in_container(&workspace_root, &identifiers, self.cache.as_ref()) {
                    return OrphanIndicatorResult::new(false, String::new(), Severity::LOW);
                }
            }
        }

        OrphanIndicatorResult::new(
            true,
            AesOrphanViolation::InfrastructureOrphan {
                stem,
                reason: Some(
                    "Not reachable from any entry point and not wired in any container.".into(),
                ),
            }
            .to_string(),
            Severity::MEDIUM,
        )
    }
}
