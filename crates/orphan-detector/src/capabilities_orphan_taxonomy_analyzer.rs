use shared::cli_commands::taxonomy_severity_vo::Severity;
use shared::code_analysis::taxonomy_analysis_vo::InboundLinkMap;
use shared::code_analysis::taxonomy_analysis_vo::OrphanIndicatorResult;
use shared::common::taxonomy_path_vo::FilePath;
use shared::orphan_detector::contract_orphan_protocol::{
    IOrphanFileCachePort, IOrphanFilenameExtractorProtocol, ITaxonomyOrphanProtocol,
};
use shared::orphan_detector::taxonomy_violation_orphan_vo::AesOrphanViolation;
use shared::taxonomy_definition_vo::LayerDefinition;
use std::sync::Arc;

// ─── Block 1: Struct Definition ───────────────────────────
pub struct TaxonomyOrphanAnalyzer {
    extractor: Arc<dyn IOrphanFilenameExtractorProtocol>,
    cache: Arc<dyn IOrphanFileCachePort>,
}

// ─── Block 2: Public Contract ─────────────────────────────
impl ITaxonomyOrphanProtocol for TaxonomyOrphanAnalyzer {
    fn is_taxonomy_orphan(
        &self,
        f: &FilePath,
        root_dir: &FilePath,
        definition: Option<&LayerDefinition>,
        inbound_links: &InboundLinkMap,
    ) -> OrphanIndicatorResult {
        self.check_taxonomy_orphan(f, root_dir, definition, inbound_links)
    }
}

// ─── Block 3: Constructors & Helpers ──────────────────────
impl TaxonomyOrphanAnalyzer {
    pub fn new(
        extractor: Arc<dyn IOrphanFilenameExtractorProtocol>,
        cache: Arc<dyn IOrphanFileCachePort>,
    ) -> Self {
        Self { extractor, cache }
    }

    fn check_taxonomy_orphan(
        &self,
        f: &FilePath,
        _root: &FilePath,
        _def: Option<&LayerDefinition>,
        inbound: &InboundLinkMap,
    ) -> OrphanIndicatorResult {
        let stem = self.extractor.file_stem(f).value;

        let suffix = match stem.rfind('_') {
            Some(pos) => &stem[pos + 1..],
            None => "",
        };

        let is_utility_or_helper = matches!(suffix, "utility" | "helper");

        let is_orphan = if is_utility_or_helper {
            let importers = match inbound.mapping.get(f.value()) {
                Some(v) => v,
                None => {
                    if self.has_crate_self_import(f.value()) {
                        return OrphanIndicatorResult::new(false, String::new(), Severity::LOW);
                    }
                    return OrphanIndicatorResult::new(
                        true,
                        AesOrphanViolation::TaxonomyOrphan {
                            stem: stem.clone(),
                            category: "utility",
                            reason: Some(format!("Taxonomy '{}' (utility/helper) is not imported by any file outside taxonomy.", stem).into()),
                        }.to_string(),
                        Severity::LOW,
                    );
                }
            };
            let has_outside_taxonomy = importers.iter().any(|importer| {
                importer
                    .split('/')
                    .next_back()
                    .is_some_and(|b| !b.starts_with("taxonomy_"))
            });
            !has_outside_taxonomy
        } else {
            let importers = match inbound.mapping.get(f.value()) {
                Some(v) => v,
                None => {
                    return OrphanIndicatorResult::new(
                        true,
                        AesOrphanViolation::TaxonomyOrphan {
                            stem: stem.clone(),
                            category: "taxonomy",
                            reason: Some(
                                format!("Taxonomy '{}' is not imported by any contract.", stem)
                                    .into(),
                            ),
                        }
                        .to_string(),
                        Severity::LOW,
                    )
                }
            };
            let has_any_importer = importers.iter().any(|importer| {
                importer
                    .split('/')
                    .next_back()
                    .is_some_and(|b| !b.starts_with("taxonomy_"))
            });
            !has_any_importer
        };

        let category = if is_utility_or_helper {
            "utility"
        } else {
            "taxonomy"
        };

        OrphanIndicatorResult::new(
            is_orphan,
            AesOrphanViolation::TaxonomyOrphan {
                stem,
                category,
                reason: None,
            }
            .to_string(),
            Severity::LOW,
        )
    }

    fn has_crate_self_import(&self, file_path: &str) -> bool {
        let stem = std::path::Path::new(file_path)
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("");
        if stem.is_empty() {
            return false;
        }
        let search = format!("crate::{}", stem);
        if let Some(parent) = std::path::Path::new(file_path).parent() {
            let entries = self.cache.read_dir(parent.to_str().unwrap_or(""));
            for entry_path in &entries {
                let path = std::path::Path::new(entry_path);
                if path == std::path::Path::new(file_path) {
                    continue;
                }
                if path.extension().is_some_and(|e| e == "rs") {
                    let fp = FilePath {
                        value: entry_path.clone(),
                    };
                    let content = self.cache.read_cached(&fp).value;
                    if content.contains(&search) {
                        return true;
                    }
                }
            }
        }
        false
    }
}
