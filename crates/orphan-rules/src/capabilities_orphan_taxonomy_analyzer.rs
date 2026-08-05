use crate::utility_orphan_filename::file_stem;
use shared::common::taxonomy_definition_vo::LayerDefinition;
use shared::common::taxonomy_path_vo::FilePath;
use shared::common::taxonomy_severity_vo::Severity;
use shared::orphan_rules::ITaxonomyOrphanProtocol;
use shared::quality_rules::taxonomy_analysis_vo::{
    InboundLinkMap, OrphanIndicatorResult, ReachabilityResult,
};

pub struct TaxonomyOrphanAnalyzer;

impl Default for TaxonomyOrphanAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

impl TaxonomyOrphanAnalyzer {
    pub fn new() -> Self {
        Self
    }
}

impl ITaxonomyOrphanProtocol for TaxonomyOrphanAnalyzer {
    fn is_taxonomy_orphan(
        &self,
        f: &FilePath,
        _root_dir: &FilePath,
        _definition: Option<&LayerDefinition>,
        inbound_links: &InboundLinkMap,
        alive_files: &ReachabilityResult,
    ) -> OrphanIndicatorResult {
        let stem = file_stem(f.value());

        // Condition 1: not reachable from any _entry file
        let is_reachable = alive_files.paths.contains(f);
        if !is_reachable {
            return OrphanIndicatorResult::new(
                true,
                format!(
                    "AES501 TAXONOMY_ORPHAN: '{}' is not reachable.\nWHY? Taxonomy file '{}' is not reachable from any _entry file.\nFIX: Import '{}' in a contract_* file.",
                    stem, stem, stem
                ),
                Severity::LOW,
            );
        }

        // Condition 2: not imported by any contract_ file
        let importers = match inbound_links.get_importers(f.value()) {
            Some(v) => v,
            None => {
                return OrphanIndicatorResult::new(
                    true,
                    format!(
                        "AES501 TAXONOMY_ORPHAN: '{}' is not imported by any higher-layer file.\nWHY? Taxonomy file '{}' has no importers.\nFIX: Import '{}' in a contract_* or higher-layer file.",
                        stem, stem, stem
                    ),
                    Severity::LOW,
                );
            }
        };

        let has_higher_layer_importer = importers.iter().any(|importer| {
            if importer == f.value() {
                return false;
            }
            let imp_filename = shared::common::utility_layer_detector::extract_filename(importer);
            // Taxonomy is the foundation layer — valid importers are ANY higher layer:
            // contract_*, capabilities_*, agent_*, surface_*, root_*
            imp_filename.starts_with("contract_")
                || imp_filename.starts_with("capabilities_")
                || imp_filename.starts_with("agent_")
                || imp_filename.starts_with("surface_")
                || imp_filename.starts_with("root_")
        });

        if has_higher_layer_importer {
            OrphanIndicatorResult::new(false, String::new(), Severity::LOW)
        } else {
            OrphanIndicatorResult::new(
                true,
                format!(
                    "AES501 TAXONOMY_ORPHAN: '{}' is not imported by any higher-layer file.\nWHY? Taxonomy file '{}' has no importers in contract, capabilities, agent, or surface layers.\nFIX: Import '{}' in a contract_* or higher-layer file.",
                    stem, stem, stem
                ),
                Severity::LOW,
            )
        }
    }
}
