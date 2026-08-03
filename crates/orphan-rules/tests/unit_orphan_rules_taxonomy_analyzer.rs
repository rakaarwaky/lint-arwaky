// Unit tests for TaxonomyOrphanAnalyzer — orphan detection for taxonomy-layer files.
use orphan_rules_lint_arwaky::capabilities_orphan_taxonomy_analyzer::TaxonomyOrphanAnalyzer;
use shared::orphan_rules::ITaxonomyOrphanProtocol;

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
