// PURPOSE: TaxonomyOrphanAnalyzer — ITaxonomyOrphanProtocol for orphan taxonomy detection.
// AST-based: uses inbound_links from AST-built graph. No regex fallback.

use shared::code_analysis::{InboundLinkMap, OrphanIndicatorResult};
use shared::common::LayerDefinition;
use shared::common::{FilePath, Severity};
use crate::utility_orphan_filename::file_stem;
use shared::orphan_detector::{AesOrphanViolation, ITaxonomyOrphanProtocol};

// ─── Block 1: Struct Definition ───────────────────────────

pub struct TaxonomyOrphanAnalyzer {}

// ─── Block 2: Protocol Trait Implementation ───────────────

impl ITaxonomyOrphanProtocol for TaxonomyOrphanAnalyzer {
    fn is_taxonomy_orphan(
        &self,
        f: &FilePath,
        _root_dir: &FilePath,
        _definition: Option<&LayerDefinition>,
        inbound_links: &InboundLinkMap,
    ) -> OrphanIndicatorResult {
        let stem = file_stem(f.value());
        let suffix = match stem.rfind('_') {
            Some(pos) => &stem[pos + 1..],
            None => "",
        };
        let is_utility_or_helper = matches!(suffix, "utility" | "helper");
        let category = if is_utility_or_helper {
            "utility"
        } else {
            "taxonomy"
        };

        // AST-built graph captures ALL imports including crate:: self-imports.
        // No has_crate_self_import fallback needed.
        let importers = match inbound_links.get_importers(f.value()) {
            Some(v) => v,
            None => {
                return OrphanIndicatorResult::new(
                    true,
                    AesOrphanViolation::TaxonomyOrphan {
                        stem: stem.clone(),
                        category,
                        reason: Some(
                            format!(
                                "Taxonomy '{}' is not imported by any other layer file.",
                                stem
                            )
                            .into(),
                        ),
                    }
                    .to_string(),
                    Severity::LOW,
                );
            }
        };

        // FR-004: taxonomy-to-taxonomy imports do NOT count — require non-taxonomy importer
        let has_other_layer_importer = importers.iter().any(|importer| {
            if importer == f.value() {
                return false; // self-import doesn't count
            }
            let imp_filename = shared::common::utility_layer_detector::extract_filename(importer);
            let imp_layer =
                shared::common::utility_layer_detector::detect_layer_from_prefix(imp_filename);
            // Count as valid if the importer is NOT a taxonomy file
            imp_layer.as_deref() != Some("taxonomy")
        });

        if has_other_layer_importer {
            OrphanIndicatorResult::new(false, String::new(), Severity::LOW)
        } else {
            OrphanIndicatorResult::new(
                true,
                AesOrphanViolation::TaxonomyOrphan {
                    stem: stem.clone(),
                    category,
                    reason: Some(
                        format!(
                            "Taxonomy '{}' is not imported by any other layer file.",
                            stem
                        )
                        .into(),
                    ),
                }
                .to_string(),
                Severity::LOW,
            )
        }
    }
}

// ─── Block 3: Constructors ────────────────────────────────

impl Default for TaxonomyOrphanAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

impl TaxonomyOrphanAnalyzer {
    pub fn new() -> Self {
        Self {}
    }
}
