use import_rules_lint_arwaky::capabilities_layer_detection_analyzer::LayerDetectionAnalyzer;
use shared::code_analysis::contract_layer_detection_protocol::ILayerDetectionProtocol;
use shared::common::contract_parser_port::ISourceParserPort;
use shared::common::contract_system_port::IFileSystemPort;
use shared::common::taxonomy_common_vo::PatternList;
use shared::common::taxonomy_path_vo::FilePath;
use shared::config_system::taxonomy_config_vo::{ArchitectureConfig, ArchitectureRule};
use std::collections::HashMap;

use async_trait::async_trait;
use shared::code_analysis::taxonomy_import_source_vo::{ImportInfoList, PrimitiveViolationList};
use shared::common::taxonomy_common_vo::{BooleanVO, Count};
use shared::common::taxonomy_definition_vo::LayerDefinition;
use shared::common::taxonomy_filesystem_error::FileSystemError;
use shared::common::taxonomy_layer_vo::Identity;
use shared::common::taxonomy_name_vo::SymbolName;
use shared::common::taxonomy_naming_list_vo::PrimitiveTypeList;
use shared::common::taxonomy_parser_error::SourceParserError;
use shared::common::taxonomy_paths_vo::FilePathList;
use shared::common::taxonomy_response_data_vo::ResponseData;
use shared::common::taxonomy_source_vo::ContentString;
use shared::common::taxonomy_suggestion_vo::MetadataVO;
use shared::mcp_server::taxonomy_job_vo::SuccessStatus;
use shared::taxonomy_layer_vo::LayerNameVO;

#[allow(dead_code)]
struct MockFs;

#[async_trait]
impl IFileSystemPort for MockFs {
    async fn walk(
        &self,
        _path: &FilePath,
        _ignored_patterns: Option<&PatternList>,
    ) -> FilePathList {
        FilePathList::new(vec![])
    }
    async fn is_directory(&self, _path: &FilePath) -> SuccessStatus {
        SuccessStatus::new(false)
    }
    async fn is_file(&self, _path: &FilePath) -> SuccessStatus {
        SuccessStatus::new(false)
    }
    async fn get_relative_path(&self, path: &FilePath, _start: &FilePath) -> FilePath {
        path.clone()
    }
    async fn read_text(&self, _path: &FilePath) -> Result<ContentString, FileSystemError> {
        Ok(ContentString::new(""))
    }
    async fn get_line_count(&self, _path: &FilePath) -> Count {
        Count::new(0)
    }
    async fn exists(&self, _path: &FilePath) -> SuccessStatus {
        SuccessStatus::new(true)
    }
    async fn get_parent(&self, path: &FilePath) -> FilePath {
        path.clone()
    }
    async fn write_text(
        &self,
        _path: &FilePath,
        _content: &ContentString,
        _mode: Option<&Identity>,
    ) -> Result<SuccessStatus, FileSystemError> {
        Ok(SuccessStatus::new(true))
    }
    async fn glob(&self, _pattern: &Identity) -> FilePathList {
        FilePathList::new(vec![])
    }
    async fn get_cwd(&self) -> FilePath {
        FilePath::new(".").unwrap_or_default()
    }
    async fn get_basename(&self, path: &FilePath) -> Identity {
        Identity::new(
            std::path::Path::new(&path.value)
                .file_name()
                .and_then(|s| s.to_str())
                .unwrap_or(""),
        )
    }
    async fn path_join(&self, parts: &[Identity]) -> FilePath {
        FilePath::new(
            parts
                .iter()
                .map(|p| p.value.as_str())
                .collect::<Vec<_>>()
                .join("/"),
        )
        .unwrap_or_default()
    }
    async fn read_file(&self, _path: &FilePath) -> Result<ContentString, FileSystemError> {
        Ok(ContentString::new(""))
    }
    fn walk_recursive(&self, _: &std::path::Path, _: &[String], _: &mut Vec<FilePath>) {}
}

#[allow(dead_code)]
struct MockParser;

impl ISourceParserPort for MockParser {
    fn extract_imports(&self, _path: &FilePath) -> Result<ImportInfoList, SourceParserError> {
        Ok(ImportInfoList::new())
    }
    fn get_raw_symbols(&self, _path: &FilePath) -> Result<ResponseData, SourceParserError> {
        Ok(ResponseData::new())
    }
    fn get_class_attributes(&self, _path: &FilePath) -> ResponseData {
        ResponseData::new()
    }
    fn has_all_export(&self, _path: &FilePath) -> SuccessStatus {
        SuccessStatus::new(false)
    }
    fn find_primitive_violations(
        &self,
        _path: &FilePath,
        _primitive_types: &PrimitiveTypeList,
    ) -> PrimitiveViolationList {
        PrimitiveViolationList::new()
    }
    fn find_unused_imports(&self, _path: &FilePath) -> ImportInfoList {
        ImportInfoList::new()
    }
    fn get_class_definitions(&self, _path: &FilePath) -> Result<MetadataVO, SourceParserError> {
        Ok(MetadataVO::new(HashMap::new()))
    }
    fn get_function_definitions(&self, _path: &FilePath) -> MetadataVO {
        MetadataVO::new(HashMap::new())
    }
    fn is_symbol_exported(&self, _path: &FilePath, _symbol: &SymbolName) -> SuccessStatus {
        SuccessStatus::new(false)
    }
    fn get_class_methods(&self, _path: &FilePath) -> MetadataVO {
        MetadataVO::new(HashMap::new())
    }
    fn get_class_bases_map(&self, _path: &FilePath) -> MetadataVO {
        MetadataVO::new(HashMap::new())
    }
    fn get_assignment_targets(&self, _path: &FilePath) -> MetadataVO {
        MetadataVO::new(HashMap::new())
    }
    fn get_control_flow_count(&self, _path: &FilePath) -> Count {
        Count::new(0)
    }
    fn is_barrel_file(&self, _path: &FilePath) -> BooleanVO {
        BooleanVO::new(false)
    }
    fn get_stem(&self, _path: &FilePath) -> SymbolName {
        SymbolName::new("")
    }
    fn is_entry_point(&self, _path: &FilePath) -> BooleanVO {
        BooleanVO::new(false)
    }
    fn get_supported_extensions(&self) -> PatternList {
        PatternList::new(vec![".rs".to_string()])
    }
}

fn make_config() -> ArchitectureConfig {
    let mut layers = HashMap::new();
    layers.insert(
        LayerNameVO::new("taxonomy"),
        LayerDefinition {
            allowed: PatternList::new(vec!["shared".to_string()]),
            ..LayerDefinition::default()
        },
    );
    layers.insert(LayerNameVO::new("capabilities"), LayerDefinition::default());
    layers.insert(
        LayerNameVO::new("surface"),
        LayerDefinition {
            forbidden: PatternList::new(vec!["agent".to_string(), "infrastructure".to_string()]),
            ..LayerDefinition::default()
        },
    );
    ArchitectureConfig {
        layers,
        rules: vec![],
        ..ArchitectureConfig::default()
    }
}

#[test]
fn test_detect_layer_by_prefix_taxonomy() {
    let config = make_config();
    let analyzer = LayerDetectionAnalyzer::new(config);
    let result = analyzer.detect_layer(
        &FilePath::new("src/taxonomy_config_vo.rs".to_string()).unwrap_or_default(),
        &FilePath::new(".".to_string()).unwrap_or_default(),
    );
    assert_eq!(
        result,
        Some(shared::common::taxonomy_layer_vo::LayerNameVO::new(
            "taxonomy"
        ))
    );
}

#[test]
fn test_detect_layer_by_prefix_capabilities() {
    let config = make_config();
    let analyzer = LayerDetectionAnalyzer::new(config);
    let result = analyzer.detect_layer(
        &FilePath::new("src/capabilities_import_checker.rs".to_string()).unwrap_or_default(),
        &FilePath::new(".".to_string()).unwrap_or_default(),
    );
    assert_eq!(
        result,
        Some(shared::common::taxonomy_layer_vo::LayerNameVO::new(
            "capabilities"
        ))
    );
}

#[test]
fn test_detect_layer_no_prefix_returns_none() {
    let config = make_config();
    let analyzer = LayerDetectionAnalyzer::new(config);
    let result = analyzer.detect_layer(
        &FilePath::new("src/main.rs".to_string()).unwrap_or_default(),
        &FilePath::new(".".to_string()).unwrap_or_default(),
    );
    assert_eq!(result, None);
}

#[test]
fn test_detect_layer_unknown_prefix_returns_none() {
    let config = make_config();
    let analyzer = LayerDetectionAnalyzer::new(config);
    let result = analyzer.detect_layer(
        &FilePath::new("src/random_file.rs".to_string()).unwrap_or_default(),
        &FilePath::new(".".to_string()).unwrap_or_default(),
    );
    assert_eq!(result, None);
}

#[test]
fn test_detect_module_layer_direct_match() {
    let config = make_config();
    let analyzer = LayerDetectionAnalyzer::new(config);
    let result = analyzer.detect_module_layer("shared::taxonomy::taxonomy_config_vo");
    assert_eq!(
        result,
        Some(shared::common::taxonomy_layer_vo::LayerNameVO::new(
            "taxonomy"
        ))
    );
}

#[test]
fn test_detect_module_layer_prefix_match() {
    let config = make_config();
    let analyzer = LayerDetectionAnalyzer::new(config);
    let result = analyzer.detect_module_layer("crate::taxonomy_config_vo::Config");
    assert_eq!(
        result,
        Some(shared::common::taxonomy_layer_vo::LayerNameVO::new(
            "taxonomy"
        ))
    );
}

#[test]
fn test_resolve_specialized_layer_with_scoped_rule() {
    let mut config = make_config();
    let spec_key = LayerNameVO::new("capabilities(checker)");
    let spec_def = config
        .layers
        .get(&LayerNameVO::new("capabilities"))
        .cloned()
        .unwrap();
    config.layers.insert(spec_key, spec_def);
    let analyzer = LayerDetectionAnalyzer::new(config);
    let result = analyzer.detect_layer(
        &FilePath::new("src/capabilities_import_checker.rs".to_string()).unwrap_or_default(),
        &FilePath::new(".".to_string()).unwrap_or_default(),
    );
    assert_eq!(
        result,
        Some(shared::common::taxonomy_layer_vo::LayerNameVO::new(
            "capabilities(checker)"
        ))
    );
}

#[test]
fn test_detect_layer_empty_path() {
    let config = make_config();
    let analyzer = LayerDetectionAnalyzer::new(config);
    let result = analyzer.detect_layer(
        &FilePath::new("".to_string()).unwrap_or_default(),
        &FilePath::new(".".to_string()).unwrap_or_default(),
    );
    assert_eq!(result, None);
}

#[test]
fn test_get_layer_def_exists() {
    let config = make_config();
    let analyzer = LayerDetectionAnalyzer::new(config);
    let result = analyzer.get_layer_def(&LayerNameVO::new("taxonomy"));
    assert!(result.is_some());
    assert!(result
        .unwrap()
        .allowed
        .values
        .contains(&"shared".to_string()));
}

#[test]
fn test_get_layer_def_not_found() {
    let config = make_config();
    let analyzer = LayerDetectionAnalyzer::new(config);
    let result = analyzer.get_layer_def(&LayerNameVO::new("nonexistent"));
    assert!(result.is_none());
}

#[test]
fn test_detect_empty_module_path() {
    let config = make_config();
    let analyzer = LayerDetectionAnalyzer::new(config);
    let result = analyzer.detect_module_layer("");
    assert_eq!(result, None);
}

#[test]
fn test_new_merges_global_rules() {
    let mut config = make_config();
    config.rules.push(ArchitectureRule {
        name: "global-mandatory".to_string().into(),
        scope: "".to_string().into(),
        mandatory: PatternList::new(vec!["shared::contract".to_string()]),
        ..ArchitectureRule::default()
    });
    let analyzer = LayerDetectionAnalyzer::new(config);
    let taxonomy_def = analyzer
        .get_layer_def(&LayerNameVO::new("taxonomy"))
        .unwrap();
    assert!(taxonomy_def
        .mandatory
        .values
        .contains(&"shared::contract".to_string()));
}

#[test]
fn test_detect_module_layer_prefix_fallback() {
    let config = make_config();
    let analyzer = LayerDetectionAnalyzer::new(config);
    let result = analyzer.detect_module_layer("some_module::taxonomy_random_thing");
    assert_eq!(
        result,
        Some(shared::common::taxonomy_layer_vo::LayerNameVO::new(
            "taxonomy"
        ))
    );
}

#[test]
fn test_layer_detection_case_sensitive() {
    let config = make_config();
    let analyzer = LayerDetectionAnalyzer::new(config);
    let result = analyzer.detect_layer(
        &FilePath::new("src/Taxonomy_Config.rs".to_string()).unwrap_or_default(),
        &FilePath::new(".".to_string()).unwrap_or_default(),
    );
    assert_eq!(result, None);
}
