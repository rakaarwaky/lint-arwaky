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

        // Condition 2: not imported by any contract_ file
        let importers = inbound_links.get_importers(f.value());
        let has_higher_layer_importer = importers
            .as_ref()
            .map(|v| {
                v.iter()
                    .filter(|importer| *importer != f.value())
                    .any(|importer| {
                        let imp_filename =
                            shared::common::utility_layer_detector::extract_filename(importer);
                        imp_filename.starts_with("contract_")
                            || imp_filename.starts_with("capabilities_")
                            || imp_filename.starts_with("agent_")
                            || imp_filename.starts_with("surface_")
                            || imp_filename.starts_with("root_")
                    })
            })
            .unwrap_or(false);

        // Both conditions must be satisfied for non-orphan
        if is_reachable && has_higher_layer_importer {
            return OrphanIndicatorResult::new(false, String::new(), Severity::LOW);
        }

        // Build diagnostic message
        let reason = if !is_reachable && !has_higher_layer_importer {
            match &importers {
                None => format!(
                    "AES501 TAXONOMY_ORPHAN: '{}' is not reachable and has no importers.\nWHY? Taxonomy file '{}' is not reachable from any _entry file AND has no importers.\nFIX: Import '{}' from a _entry file AND a contract_* or higher-layer file.",
                    stem, stem, stem
                ),
                Some(v) if v.is_empty() => format!(
                    "AES501 TAXONOMY_ORPHAN: '{}' is not reachable and has no importers.\nWHY? Taxonomy file '{}' is not reachable from any _entry file AND has no importers.\nFIX: Import '{}' from a _entry file AND a contract_* or higher-layer file.",
                    stem, stem, stem
                ),
                Some(v) => {
                    let low_layer: Vec<String> = v
                        .iter()
                        .filter(|i| *i != f.value())
                        .map(|i| {
                            shared::common::utility_layer_detector::extract_filename(i).to_string()
                        })
                        .collect();
                    format!(
                        "AES501 TAXONOMY_ORPHAN: '{}' is not reachable and not imported by higher layers.\nWHY? Taxonomy file '{}' is not reachable from any _entry file AND only imported by lower-layer files ({}).\nFIX: Import '{}' from a _entry file AND a contract_* or higher-layer file.",
                        stem,
                        stem,
                        low_layer.join(", "),
                        stem
                    )
                }
            }
        } else if !is_reachable {
            format!(
                "AES501 TAXONOMY_ORPHAN: '{}' is not reachable.\nWHY? Taxonomy file '{}' is not reachable from any _entry file.\nFIX: Import '{}' from a _entry file.",
                stem, stem, stem
            )
        } else {
            format!(
                "AES501 TAXONOMY_ORPHAN: '{}' is not imported by any higher-layer file.\nWHY? Taxonomy file '{}' has no importers in contract, capabilities, agent, or surface layers.\nFIX: Import '{}' in a contract_* or higher-layer file.",
                stem, stem, stem
            )
        };

        OrphanIndicatorResult::new(true, reason, Severity::LOW)
    }
}
