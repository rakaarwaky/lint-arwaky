use crate::utility_orphan_filename::file_stem;
use shared::common::taxonomy_definition_vo::LayerDefinition;
use shared::common::taxonomy_path_vo::FilePath;
use shared::common::taxonomy_severity_vo::Severity;
use shared::orphan_rules::ITaxonomyOrphanProtocol;
use shared::quality_rules::taxonomy_analysis_vo::{InboundLinkMap, OrphanIndicatorResult};

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
    ) -> OrphanIndicatorResult {
        let stem = file_stem(f.value());

        // In unified workspace scans, f.value() is absolute but inbound_links keys
        // are relative paths. get_importers already handles /crates/ prefix stripping,
        // but as a last resort try suffix matching against the mapping keys.
        let importers = inbound_links.get_importers(f.value()).or_else(|| {
            let path_str = f.value();
            inbound_links.mapping.keys().find_map(|key| {
                if path_str.ends_with(key) {
                    inbound_links.mapping.get(key)
                } else {
                    None
                }
            })
        });

        let importers = match importers {
            Some(v) => v,
            None => {
                return OrphanIndicatorResult::new(
                    true,
                    format!(
                        "AES501 TAXONOMY_ORPHAN: '{}' is not imported.\nWHY? Taxonomy file '{}' is not imported by any other layer file.\nFIX: Import '{}' in a contract_* file.",
                        stem, stem, stem
                    ),
                    Severity::LOW,
                );
            }
        };

        let has_other_layer_importer = importers.iter().any(|importer| {
            if importer == f.value() {
                return false;
            }
            let imp_filename = shared::common::utility_layer_detector::extract_filename(importer);
            let imp_layer =
                shared::common::utility_layer_detector::detect_layer_from_prefix(imp_filename);
            imp_layer.as_deref() != Some("taxonomy")
        });

        if has_other_layer_importer {
            OrphanIndicatorResult::new(false, String::new(), Severity::LOW)
        } else {
            OrphanIndicatorResult::new(
                true,
                format!(
                    "AES501 TAXONOMY_ORPHAN: '{}' is not imported.\nWHY? Taxonomy file '{}' is not imported by any other layer file.\nFIX: Import '{}' in a contract_* file.",
                    stem, stem, stem
                ),
                Severity::LOW,
            )
        }
    }
}
