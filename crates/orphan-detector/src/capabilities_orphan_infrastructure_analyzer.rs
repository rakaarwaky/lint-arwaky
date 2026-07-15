use shared::cli_commands::taxonomy_severity_vo::Severity;
use shared::code_analysis::taxonomy_analysis_vo::OrphanIndicatorResult;
use shared::code_analysis::taxonomy_analysis_vo::ReachabilityResult;
use shared::common::taxonomy_path_vo::FilePath;
use shared::orphan_detector::contract_orphan_protocol::{
    IInfrastructureOrphanProtocol, IOrphanFilenameExtractorProtocol,
};
use shared::orphan_detector::taxonomy_violation_orphan_vo::AesOrphanViolation;
use std::sync::Arc;

pub struct InfrastructureOrphanAnalyzer {
    extractor: Arc<dyn IOrphanFilenameExtractorProtocol>,
}

impl Default for InfrastructureOrphanAnalyzer {
    fn default() -> Self {
        Self {
            extractor: Arc::new(
                crate::capabilities_orphan_filename_extractor::OrphanFilenameExtractor::new(),
            ),
        }
    }
}

impl InfrastructureOrphanAnalyzer {
    pub fn new(extractor: Arc<dyn IOrphanFilenameExtractorProtocol>) -> Self {
        Self { extractor }
    }
}

impl IInfrastructureOrphanProtocol for InfrastructureOrphanAnalyzer {
    fn is_infrastructure_orphan(
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
        let stem = self.extractor.file_stem(f).value;

        if let Ok(content) = std::fs::read_to_string(fp) {
            let mut identifiers: Vec<String> = Vec::new();
            identifiers.extend(self.extractor.extract_struct_names(&content));
            identifiers.extend(self.extractor.extract_trait_names(&content));
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
            if let Ok(workspace_root) =
                crate::capabilities_orphan_capabilities_analyzer::find_workspace_root(root)
            {
                if let Ok(wired) =
                    crate::capabilities_orphan_capabilities_analyzer::check_wired_in_container(
                        &workspace_root,
                        &identifiers,
                    )
                {
                    if wired {
                        return OrphanIndicatorResult::new(false, String::new(), Severity::LOW);
                    }
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
