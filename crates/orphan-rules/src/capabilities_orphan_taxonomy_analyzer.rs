use crate::utility_orphan_filename::file_stem;
use shared::common::taxonomy_definition_vo::LayerDefinition;
use shared::common::taxonomy_path_vo::FilePath;
use shared::common::taxonomy_severity_vo::Severity;
use shared::orphan_rules::{AesOrphanViolation, ITaxonomyOrphanProtocol};
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

#[cfg(test)]
mod tests {
    use super::*;
    use shared::common::taxonomy_path_vo::FilePath;
    use shared::quality_rules::taxonomy_analysis_vo::InboundLinkMap;
    use std::collections::HashMap;

    #[test]
    fn test_constructor() {
        let analyzer = TaxonomyOrphanAnalyzer::new();
        // Verify construction succeeds
        let _analyzer2 = TaxonomyOrphanAnalyzer;
        // Just ensure the analyzer can be created without panicking
        let fp = FilePath::new(".".to_string()).unwrap();
        let root = FilePath::new(".".to_string()).unwrap();
        let empty_map = InboundLinkMap::new(HashMap::new());
        let _result = analyzer.is_taxonomy_orphan(&fp, &root, None, &empty_map);
    }

    #[test]
    fn test_file_with_no_importers_is_orphan() {
        let analyzer = TaxonomyOrphanAnalyzer::new();
        let fp = FilePath::new("crates/shared/src/taxonomy_color.rs".to_string()).unwrap();
        let root = FilePath::new(".".to_string()).unwrap();
        let empty_map = InboundLinkMap::new(HashMap::new());

        let result = analyzer.is_taxonomy_orphan(&fp, &root, None, &empty_map);
        assert!(result.is_orphan);
        assert!(!result.reason.is_empty());
    }

    #[test]
    fn test_file_imported_by_other_layer_is_not_orphan() {
        let analyzer = TaxonomyOrphanAnalyzer::new();
        let target = "crates/shared/src/taxonomy_color.rs";
        let fp = FilePath::new(target.to_string()).unwrap();
        let root = FilePath::new(".".to_string()).unwrap();

        let mut mapping = HashMap::new();
        // An agent-layer file imports the taxonomy file
        mapping.insert(
            target.to_string(),
            vec!["crates/orphan-rules/src/agent_orphan_orchestrator.rs".to_string()],
        );
        let inbound = InboundLinkMap::new(mapping);

        let result = analyzer.is_taxonomy_orphan(&fp, &root, None, &inbound);
        assert!(!result.is_orphan);
    }

    #[test]
    fn test_file_only_imported_by_taxonomy_layer_is_orphan() {
        let analyzer = TaxonomyOrphanAnalyzer::new();
        let target = "crates/shared/src/taxonomy_color.rs";
        let fp = FilePath::new(target.to_string()).unwrap();
        let root = FilePath::new(".".to_string()).unwrap();

        let mut mapping = HashMap::new();
        // Only taxonomy-layer files import this taxonomy file
        mapping.insert(
            target.to_string(),
            vec!["crates/shared/src/taxonomy_size.rs".to_string()],
        );
        let inbound = InboundLinkMap::new(mapping);

        let result = analyzer.is_taxonomy_orphan(&fp, &root, None, &inbound);
        assert!(result.is_orphan);
        assert!(result.reason.contains("taxonomy_color"));
    }

    #[test]
    fn test_utility_suffix_categorization() {
        let analyzer = TaxonomyOrphanAnalyzer::new();
        let fp = FilePath::new("crates/shared/src/taxonomy_helper.rs".to_string()).unwrap();
        let root = FilePath::new(".".to_string()).unwrap();
        let empty_map = InboundLinkMap::new(HashMap::new());

        let result = analyzer.is_taxonomy_orphan(&fp, &root, None, &empty_map);
        // "taxonomy_helper" has suffix "helper" → category is "utility"
        assert!(result.is_orphan);
        assert!(!result.reason.is_empty());
    }
}
