// PURPOSE: TaxonomyOrphanAnalyzer — ITaxonomyOrphanProtocol for orphan taxonomy detection.
// AST-based: uses inbound_links from AST-built graph. No regex fallback.

use shared::code_analysis::{InboundLinkMap, OrphanIndicatorResult};
use shared::common::LayerDefinition;
use shared::common::{FilePath, Severity};
use shared::orphan_detector::utility_orphan_filename::file_stem;
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

        // Check if any importer is from another layer (not taxonomy, not barrel)
        let has_other_layer_importer = importers.iter().any(|importer| {
            let b = importer.rsplit('/').next().unwrap_or(importer);
            // Barrel files don't count as real consumers
            if matches!(b, "mod.rs" | "__init__.py" | "index.ts" | "index.js") {
                return false;
            }
            // Same-layer imports don't count
            !b.starts_with("taxonomy_")
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
