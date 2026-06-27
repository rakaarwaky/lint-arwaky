use code_analysis_lint_arwaky::root_code_analysis_container::{CodeAnalysisCheckerContainer, CodeAnalysisContainer};

#[test]
fn checker_container_default_constructs() {
    let container = CodeAnalysisCheckerContainer::default();
    let analyzer = container.analyzer();
    let _config = analyzer.config();
    // All checkers should be accessible
    let _bypass = container.bypass_checker();
    let _line = container.line_checker();
    let _class = container.class_checker();
    let _dup = container.duplication_checker();
    let _dead = container.dead_inheritance_checker();
}

#[test]
fn checker_container_detect_layer_returns_none_for_empty_analyzer() {
    let container = CodeAnalysisCheckerContainer::default();
    let layer = container.detect_layer("src/foo.rs", ".");
    assert!(layer.is_none());
}

#[test]
fn container_default_constructs() {
    let container = CodeAnalysisContainer::new();
    let linter = container.code_analysis_linter();
    let _ = linter;
}

#[test]
fn container_with_analyzer_constructs() {
    use shared::code_analysis::contract_code_analysis_aggregate::ICodeAnalysisAggregate;
    use std::sync::Arc;
    let container = CodeAnalysisContainer::new();
    let linter = container.code_analysis_linter();
    let results = linter.run_code_analysis_path("/nonexistent/path/xyz");
    assert!(results.is_empty());
}

#[test]
fn container_default_is_same_as_new() {
    let c1 = CodeAnalysisContainer::new();
    let c2 = CodeAnalysisContainer::default();
    let r1 = c1.code_analysis_linter().run_code_analysis_path("/nonexistent");
    let r2 = c2.code_analysis_linter().run_code_analysis_path("/nonexistent");
    assert_eq!(r1.len(), r2.len());
}

#[test]
fn container_new_with_analyzer_provides_custom_analyzer() {
    use shared::import_rules::contract_rule_protocol::IAnalyzer;
    use std::sync::Arc;

    // Use a custom placeholder analyzer
    let custom = Arc::new(TestAnalyzer);
    let container = CodeAnalysisContainer::new_with_analyzer(custom);
    let _linter = container.code_analysis_linter();
}

/// Minimal IAnalyzer implementation for testing container wiring
struct TestAnalyzer;

impl shared::import_rules::contract_rule_protocol::IAnalyzer for TestAnalyzer {
    fn fs(&self) -> &dyn shared::common::contract_system_port::IFileSystemPort {
        static FS: std::sync::OnceLock<NullFs> = std::sync::OnceLock::new();
        FS.get_or_init(|| NullFs)
    }
    fn parser(&self) -> &dyn shared::common::contract_parser_port::ISourceParserPort {
        static P: std::sync::OnceLock<NullParser> = std::sync::OnceLock::new();
        P.get_or_init(|| NullParser)
    }
    fn detect_module_layer(
        &self,
        _module_path: &shared::common::taxonomy_path_vo::FilePath,
    ) -> Option<shared::taxonomy_layer_vo::LayerNameVO> {
        None
    }
}

impl shared::naming_rules::contract_naming_analyzer_protocol::INamingAnalyzerProtocol for TestAnalyzer {
    fn config(&self) -> &shared::config_system::taxonomy_config_vo::ArchitectureConfig {
        static C: std::sync::OnceLock<shared::config_system::taxonomy_config_vo::ArchitectureConfig> = std::sync::OnceLock::new();
        C.get_or_init(|| {
            let mut config = shared::config_system::taxonomy_config_vo::ArchitectureConfig::default();
            config.enabled = shared::taxonomy_common_vo::BooleanVO::new(false);
            config
        })
    }
    fn layer_map(&self) -> &shared::taxonomy_definition_vo::LayerMapVO {
        static M: std::sync::OnceLock<shared::taxonomy_definition_vo::LayerMapVO> = std::sync::OnceLock::new();
        M.get_or_init(|| shared::taxonomy_definition_vo::LayerMapVO::new(std::collections::HashMap::new()))
    }
    fn detect_layer(
        &self,
        _f: &shared::common::taxonomy_path_vo::FilePath,
        _root_dir: &shared::common::taxonomy_path_vo::FilePath,
    ) -> Option<shared::taxonomy_layer_vo::LayerNameVO> {
        None
    }
}

struct NullFs;
#[async_trait::async_trait]
impl shared::common::contract_system_port::IFileSystemPort for NullFs {
    async fn walk(&self, _path: &shared::common::taxonomy_path_vo::FilePath, _ignored_patterns: Option<&shared::common::taxonomy_common_vo::PatternList>) -> shared::common::taxonomy_paths_vo::FilePathList {
        shared::common::taxonomy_paths_vo::FilePathList::new(vec![])
    }
    async fn is_directory(&self, _path: &shared::common::taxonomy_path_vo::FilePath) -> shared::mcp_server::taxonomy_job_vo::SuccessStatus {
        shared::mcp_server::taxonomy_job_vo::SuccessStatus::new(false)
    }
    async fn is_file(&self, _path: &shared::common::taxonomy_path_vo::FilePath) -> shared::mcp_server::taxonomy_job_vo::SuccessStatus {
        shared::mcp_server::taxonomy_job_vo::SuccessStatus::new(false)
    }
    async fn get_relative_path(&self, path: &shared::common::taxonomy_path_vo::FilePath, _start: &shared::common::taxonomy_path_vo::FilePath) -> shared::common::taxonomy_path_vo::FilePath {
        path.clone()
    }
    async fn read_text(&self, _path: &shared::common::taxonomy_path_vo::FilePath) -> Result<shared::common::taxonomy_source_vo::ContentString, shared::common::taxonomy_filesystem_error::FileSystemError> {
        Err(shared::common::taxonomy_filesystem_error::FileSystemError::new(shared::common::taxonomy_path_vo::FilePath::default(), shared::common::taxonomy_common_error::ErrorMessage::new("null"), shared::mcp_server::taxonomy_action_vo::ActionName::new("read")))
    }
    async fn get_line_count(&self, _path: &shared::common::taxonomy_path_vo::FilePath) -> shared::common::taxonomy_common_vo::Count {
        shared::common::taxonomy_common_vo::Count::default()
    }
    async fn exists(&self, _path: &shared::common::taxonomy_path_vo::FilePath) -> shared::mcp_server::taxonomy_job_vo::SuccessStatus {
        shared::mcp_server::taxonomy_job_vo::SuccessStatus::new(false)
    }
    async fn get_parent(&self, _path: &shared::common::taxonomy_path_vo::FilePath) -> shared::common::taxonomy_path_vo::FilePath {
        _path.clone()
    }
    async fn write_text(&self, _path: &shared::common::taxonomy_path_vo::FilePath, _content: &shared::common::taxonomy_source_vo::ContentString, _mode: Option<&shared::common::taxonomy_layer_vo::Identity>) -> Result<shared::mcp_server::taxonomy_job_vo::SuccessStatus, shared::common::taxonomy_filesystem_error::FileSystemError> {
        Err(shared::common::taxonomy_filesystem_error::FileSystemError::new(shared::common::taxonomy_path_vo::FilePath::default(), shared::common::taxonomy_common_error::ErrorMessage::new("null"), shared::mcp_server::taxonomy_action_vo::ActionName::new("write")))
    }
    async fn glob(&self, _pattern: &shared::common::taxonomy_layer_vo::Identity) -> shared::common::taxonomy_paths_vo::FilePathList {
        shared::common::taxonomy_paths_vo::FilePathList::new(vec![])
    }
    async fn get_cwd(&self) -> shared::common::taxonomy_path_vo::FilePath {
        shared::common::taxonomy_path_vo::FilePath::default()
    }
    async fn get_basename(&self, _path: &shared::common::taxonomy_path_vo::FilePath) -> shared::common::taxonomy_layer_vo::Identity {
        shared::common::taxonomy_layer_vo::Identity::default()
    }
    async fn path_join(&self, _parts: &[shared::common::taxonomy_layer_vo::Identity]) -> shared::common::taxonomy_path_vo::FilePath {
        shared::common::taxonomy_path_vo::FilePath::default()
    }
    async fn read_file(&self, _path: &shared::common::taxonomy_path_vo::FilePath) -> Result<shared::common::taxonomy_source_vo::ContentString, shared::common::taxonomy_filesystem_error::FileSystemError> {
        Err(shared::common::taxonomy_filesystem_error::FileSystemError::new(shared::common::taxonomy_path_vo::FilePath::default(), shared::common::taxonomy_common_error::ErrorMessage::new("null"), shared::mcp_server::taxonomy_action_vo::ActionName::new("read")))
    }
}

struct NullParser;
impl shared::common::contract_parser_port::ISourceParserPort for NullParser {
    fn extract_imports(&self, _path: &shared::common::taxonomy_path_vo::FilePath) -> Result<shared::code_analysis::taxonomy_import_source_vo::ImportInfoList, shared::common::taxonomy_parser_error::SourceParserError> {
        Ok(shared::code_analysis::taxonomy_import_source_vo::ImportInfoList::default())
    }
    fn get_raw_symbols(&self, _path: &shared::common::taxonomy_path_vo::FilePath) -> Result<shared::mcp_server::taxonomy_job_vo::ResponseData, shared::common::taxonomy_parser_error::SourceParserError> {
        Ok(shared::mcp_server::taxonomy_job_vo::ResponseData::default())
    }
    fn get_class_attributes(&self, _path: &shared::common::taxonomy_path_vo::FilePath) -> shared::mcp_server::taxonomy_job_vo::ResponseData {
        shared::mcp_server::taxonomy_job_vo::ResponseData::default()
    }
    fn has_all_export(&self, _path: &shared::common::taxonomy_path_vo::FilePath) -> shared::mcp_server::taxonomy_job_vo::SuccessStatus {
        shared::mcp_server::taxonomy_job_vo::SuccessStatus::new(false)
    }
    fn find_primitive_violations(&self, _path: &shared::common::taxonomy_path_vo::FilePath, _primitive_types: &shared::common::taxonomy_naming_list_vo::PrimitiveTypeList) -> shared::code_analysis::taxonomy_import_source_vo::PrimitiveViolationList {
        shared::code_analysis::taxonomy_import_source_vo::PrimitiveViolationList::default()
    }
    fn find_unused_imports(&self, _path: &shared::common::taxonomy_path_vo::FilePath) -> shared::code_analysis::taxonomy_import_source_vo::ImportInfoList {
        shared::code_analysis::taxonomy_import_source_vo::ImportInfoList::default()
    }
    fn get_class_definitions(&self, _path: &shared::common::taxonomy_path_vo::FilePath) -> Result<shared::common::taxonomy_suggestion_vo::MetadataVO, shared::common::taxonomy_parser_error::SourceParserError> {
        Ok(shared::common::taxonomy_suggestion_vo::MetadataVO::new(std::collections::HashMap::new()))
    }
    fn get_function_definitions(&self, _path: &shared::common::taxonomy_path_vo::FilePath) -> shared::common::taxonomy_suggestion_vo::MetadataVO {
        shared::common::taxonomy_suggestion_vo::MetadataVO::new(std::collections::HashMap::new())
    }
    fn is_symbol_exported(&self, _path: &shared::common::taxonomy_path_vo::FilePath, _symbol: &shared::common::taxonomy_name_vo::SymbolName) -> shared::mcp_server::taxonomy_job_vo::SuccessStatus {
        shared::mcp_server::taxonomy_job_vo::SuccessStatus::new(false)
    }
    fn get_class_methods(&self, _path: &shared::common::taxonomy_path_vo::FilePath) -> shared::common::taxonomy_suggestion_vo::MetadataVO {
        shared::common::taxonomy_suggestion_vo::MetadataVO::new(std::collections::HashMap::new())
    }
    fn get_class_bases_map(&self, _path: &shared::common::taxonomy_path_vo::FilePath) -> shared::common::taxonomy_suggestion_vo::MetadataVO {
        shared::common::taxonomy_suggestion_vo::MetadataVO::new(std::collections::HashMap::new())
    }
    fn get_assignment_targets(&self, _path: &shared::common::taxonomy_path_vo::FilePath) -> shared::common::taxonomy_suggestion_vo::MetadataVO {
        shared::common::taxonomy_suggestion_vo::MetadataVO::new(std::collections::HashMap::new())
    }
    fn get_control_flow_count(&self, _path: &shared::common::taxonomy_path_vo::FilePath) -> shared::common::taxonomy_common_vo::Count {
        shared::common::taxonomy_common_vo::Count::default()
    }
    fn is_barrel_file(&self, _path: &shared::common::taxonomy_path_vo::FilePath) -> shared::common::taxonomy_common_vo::BooleanVO {
        shared::common::taxonomy_common_vo::BooleanVO::default()
    }
    fn get_stem(&self, _path: &shared::common::taxonomy_path_vo::FilePath) -> shared::common::taxonomy_name_vo::SymbolName {
        shared::common::taxonomy_name_vo::SymbolName::new("")
    }
    fn is_entry_point(&self, _path: &shared::common::taxonomy_path_vo::FilePath) -> shared::common::taxonomy_common_vo::BooleanVO {
        shared::common::taxonomy_common_vo::BooleanVO::default()
    }
    fn get_supported_extensions(&self) -> shared::common::taxonomy_common_vo::PatternList {
        shared::common::taxonomy_common_vo::PatternList::default()
    }
}
