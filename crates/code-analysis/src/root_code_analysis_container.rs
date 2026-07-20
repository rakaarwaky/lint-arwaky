// PURPOSE: Root container for code-analysis — defines CodeAnalysisCheckerContainer and CodeAnalysisContainer
// Wiring: ICodeMetricAnalyzerProtocol → CodeDuplicationAnalyzer (capabilities layer)
// ALGORITHM:
//   CodeAnalysisCheckerContainer: injects checkers (BypassChecker, ArchLineChecker,
//     MandatoryDefinitionChecker, CodeDuplicationAnalyzer) and exposes them via typed accessors.
//   CodeAnalysisContainer: wraps CodeAnalysisOrchestrator as IArchLintProtocol for surface consumption.

use crate::capabilities_check_bypass_checker::BypassChecker;
use crate::capabilities_code_duplication_analyzer::CodeDuplicationAnalyzer;
use crate::capabilities_line_checker::ArchLineChecker;
use crate::capabilities_mandatory_definition_checker::MandatoryDefinitionChecker;
use shared::code_analysis::contract_bypass_checker_protocol::IBypassCheckerProtocol;
use shared::code_analysis::contract_class_protocol::IMandatoryClassProtocol;
use shared::code_analysis::contract_dead_inheritance_protocol::IDeadInheritanceProtocol;
use shared::code_analysis::contract_line_protocol::ILineCheckerProtocol;
use shared::common::taxonomy_path_vo::FilePath;
use shared::config_system::taxonomy_config_vo::ArchitectureConfig;
use shared::import_rules::contract_rule_protocol::IAnalyzer;
use std::sync::Arc;

/// CodeAnalysisCheckerContainer holds only code-analysis protocol implementations.
/// Other crates (import-rules, naming-rules, role-rules, orphan-detector)
/// have their own containers and orchestrators.
#[derive(Clone)]
pub struct CodeAnalysisCheckerContainer {
    analyzer: Arc<dyn IAnalyzer>,
    bypass_checker: Arc<dyn IBypassCheckerProtocol>,
    mandatory_definition_checker: Arc<MandatoryDefinitionChecker>,
    line_checker: Arc<dyn ILineCheckerProtocol>,
    code_duplication_analyzer: Arc<CodeDuplicationAnalyzer>,
}

impl CodeAnalysisCheckerContainer {
    pub fn new(analyzer: Arc<dyn IAnalyzer>) -> Self {
        let mandatory = Arc::new(MandatoryDefinitionChecker::new());
        // Honor AES304 forbidden_bypass from config when the analyzer exposes one;
        // fall back to the in-code default list otherwise.
        let bypass = analyzer
            .config()
            .rules
            .iter()
            .find(|r| r.name.value == "AES304")
            .map(|r| BypassChecker::from_patterns(&r.code_analysis.forbidden_bypass))
            .unwrap_or_default();
        Self {
            analyzer,
            bypass_checker: Arc::new(bypass),
            mandatory_definition_checker: mandatory,
            line_checker: Arc::new(ArchLineChecker {}),
            code_duplication_analyzer: Arc::new(CodeDuplicationAnalyzer::new()),
        }
    }

    pub fn bypass_checker(&self) -> &Arc<dyn IBypassCheckerProtocol> {
        &self.bypass_checker
    }

    pub fn dead_inheritance_checker(&self) -> Arc<dyn IDeadInheritanceProtocol> {
        self.mandatory_definition_checker.clone()
    }

    pub fn line_checker(&self) -> &Arc<dyn ILineCheckerProtocol> {
        &self.line_checker
    }

    pub fn class_checker(&self) -> Arc<dyn IMandatoryClassProtocol> {
        self.mandatory_definition_checker.clone()
    }

    pub fn detect_layer(
        &self,
        file: &str,
        root_dir: &str,
    ) -> Option<shared::taxonomy_layer_vo::LayerNameVO> {
        let f = match shared::common::taxonomy_path_vo::FilePath::new(file.to_string()) {
            Ok(fp) => fp,
            Err(_) => return None,
        };
        let rd = match shared::common::taxonomy_path_vo::FilePath::new(root_dir.to_string()) {
            Ok(fp) => fp,
            Err(_) => return None,
        };
        self.analyzer.detect_layer(&f, &rd)
    }

    pub fn get_layer_def(
        &self,
        layer: &shared::taxonomy_layer_vo::LayerNameVO,
    ) -> Option<&shared::common::taxonomy_definition_vo::LayerDefinition> {
        self.analyzer.layer_map().values.get(layer)
    }

    pub fn analyzer(&self) -> &Arc<dyn IAnalyzer> {
        &self.analyzer
    }

    pub fn duplication_checker(&self) -> &Arc<CodeDuplicationAnalyzer> {
        &self.code_duplication_analyzer
    }

    pub fn as_checker_ref(&self) -> &dyn CodeAnalysisCheckerContainerRef {
        self
    }
}

/// Trait for dynamic dispatch of CodeAnalysisCheckerContainer
pub trait CodeAnalysisCheckerContainerRef {
    fn detect_layer(
        &self,
        file: &str,
        root_dir: &str,
    ) -> Option<shared::taxonomy_layer_vo::LayerNameVO>;
    fn get_layer_def(
        &self,
        layer: &shared::taxonomy_layer_vo::LayerNameVO,
    ) -> Option<&shared::common::taxonomy_definition_vo::LayerDefinition>;
}

impl CodeAnalysisCheckerContainerRef for CodeAnalysisCheckerContainer {
    fn detect_layer(
        &self,
        file: &str,
        root_dir: &str,
    ) -> Option<shared::taxonomy_layer_vo::LayerNameVO> {
        self.detect_layer(file, root_dir)
    }
    fn get_layer_def(
        &self,
        layer: &shared::taxonomy_layer_vo::LayerNameVO,
    ) -> Option<&shared::common::taxonomy_definition_vo::LayerDefinition> {
        self.get_layer_def(layer)
    }
}

impl Default for CodeAnalysisCheckerContainer {
    fn default() -> Self {
        Self::new(Arc::new(PlaceholderAnalyzer))
    }
}

struct NullFileSystem;

#[async_trait::async_trait]
impl shared::common::contract_system_protocol::IFileSystemProtocol for NullFileSystem {
    async fn walk(
        &self,
        _path: &FilePath,
        _ignored_patterns: Option<&shared::common::taxonomy_common_vo::PatternList>,
    ) -> shared::common::taxonomy_paths_vo::FilePathList {
        shared::common::taxonomy_paths_vo::FilePathList::new(vec![])
    }
    async fn is_directory(
        &self,
        _path: &FilePath,
    ) -> shared::mcp_server::taxonomy_job_vo::SuccessStatus {
        shared::mcp_server::taxonomy_job_vo::SuccessStatus::new(false)
    }
    async fn is_file(
        &self,
        _path: &FilePath,
    ) -> shared::mcp_server::taxonomy_job_vo::SuccessStatus {
        shared::mcp_server::taxonomy_job_vo::SuccessStatus::new(false)
    }
    async fn get_relative_path(&self, path: &FilePath, _start: &FilePath) -> FilePath {
        path.clone()
    }
    async fn read_text(
        &self,
        _path: &FilePath,
    ) -> Result<
        shared::common::taxonomy_source_vo::ContentString,
        shared::common::taxonomy_filesystem_error::FileSystemError,
    > {
        Err(
            shared::common::taxonomy_filesystem_error::FileSystemError::new(
                FilePath::default(),
                shared::common::taxonomy_common_error::ErrorMessage::new(
                    "null filesystem: not initialized",
                ),
                shared::mcp_server::taxonomy_action_vo::ActionName::new("read"),
            ),
        )
    }
    async fn get_line_count(&self, _path: &FilePath) -> shared::common::taxonomy_common_vo::Count {
        shared::common::taxonomy_common_vo::Count::default()
    }
    async fn exists(&self, _path: &FilePath) -> shared::mcp_server::taxonomy_job_vo::SuccessStatus {
        shared::mcp_server::taxonomy_job_vo::SuccessStatus::new(false)
    }
    async fn get_parent(&self, _path: &FilePath) -> FilePath {
        FilePath::default()
    }
    async fn write_text(
        &self,
        _path: &FilePath,
        _content: &shared::common::taxonomy_source_vo::ContentString,
        _mode: Option<&shared::common::taxonomy_layer_vo::Identity>,
    ) -> Result<
        shared::mcp_server::taxonomy_job_vo::SuccessStatus,
        shared::common::taxonomy_filesystem_error::FileSystemError,
    > {
        Err(
            shared::common::taxonomy_filesystem_error::FileSystemError::new(
                FilePath::default(),
                shared::common::taxonomy_common_error::ErrorMessage::new(
                    "null filesystem: not initialized",
                ),
                shared::mcp_server::taxonomy_action_vo::ActionName::new("write"),
            ),
        )
    }
    async fn glob(
        &self,
        _pattern: &shared::common::taxonomy_layer_vo::Identity,
    ) -> shared::common::taxonomy_paths_vo::FilePathList {
        shared::common::taxonomy_paths_vo::FilePathList::new(vec![])
    }
    async fn get_cwd(&self) -> FilePath {
        FilePath::default()
    }
    async fn get_basename(&self, _path: &FilePath) -> shared::common::taxonomy_layer_vo::Identity {
        shared::common::taxonomy_layer_vo::Identity::default()
    }
    async fn path_join(&self, _parts: &[shared::common::taxonomy_layer_vo::Identity]) -> FilePath {
        FilePath::default()
    }
    async fn read_file(
        &self,
        _path: &FilePath,
    ) -> Result<
        shared::common::taxonomy_source_vo::ContentString,
        shared::common::taxonomy_filesystem_error::FileSystemError,
    > {
        Err(
            shared::common::taxonomy_filesystem_error::FileSystemError::new(
                FilePath::default(),
                shared::common::taxonomy_common_error::ErrorMessage::new(
                    "null filesystem: not initialized",
                ),
                shared::mcp_server::taxonomy_action_vo::ActionName::new("read"),
            ),
        )
    }
}

struct NullSourceParser;

impl shared::common::contract_parser_protocol::ISourceParserProtocol for NullSourceParser {
    fn extract_imports(
        &self,
        _path: &FilePath,
    ) -> Result<
        shared::code_analysis::taxonomy_import_source_vo::ImportInfoList,
        shared::common::taxonomy_parser_error::SourceParserError,
    > {
        Ok(shared::code_analysis::taxonomy_import_source_vo::ImportInfoList::default())
    }
    fn get_raw_symbols(
        &self,
        _path: &FilePath,
    ) -> Result<
        shared::mcp_server::taxonomy_job_vo::ResponseData,
        shared::common::taxonomy_parser_error::SourceParserError,
    > {
        Ok(shared::mcp_server::taxonomy_job_vo::ResponseData::default())
    }
    fn get_class_attributes(
        &self,
        _path: &FilePath,
    ) -> shared::mcp_server::taxonomy_job_vo::ResponseData {
        shared::mcp_server::taxonomy_job_vo::ResponseData::default()
    }
    fn has_all_export(
        &self,
        _path: &FilePath,
    ) -> shared::mcp_server::taxonomy_job_vo::SuccessStatus {
        shared::mcp_server::taxonomy_job_vo::SuccessStatus::new(false)
    }
    fn find_primitive_violations(
        &self,
        _path: &FilePath,
        _primitive_types: &shared::common::taxonomy_naming_list_vo::PrimitiveTypeList,
    ) -> shared::code_analysis::taxonomy_import_source_vo::PrimitiveViolationList {
        shared::code_analysis::taxonomy_import_source_vo::PrimitiveViolationList::default()
    }
    fn find_unused_imports(
        &self,
        _path: &FilePath,
    ) -> shared::code_analysis::taxonomy_import_source_vo::ImportInfoList {
        shared::code_analysis::taxonomy_import_source_vo::ImportInfoList::default()
    }
    fn get_class_definitions(
        &self,
        _path: &FilePath,
    ) -> Result<
        shared::common::taxonomy_suggestion_vo::MetadataVO,
        shared::common::taxonomy_parser_error::SourceParserError,
    > {
        Ok(shared::common::taxonomy_suggestion_vo::MetadataVO::new(
            std::collections::HashMap::new(),
        ))
    }
    fn get_function_definitions(
        &self,
        _path: &FilePath,
    ) -> shared::common::taxonomy_suggestion_vo::MetadataVO {
        shared::common::taxonomy_suggestion_vo::MetadataVO::new(std::collections::HashMap::new())
    }
    fn is_symbol_exported(
        &self,
        _path: &FilePath,
        _symbol: &shared::common::taxonomy_name_vo::SymbolName,
    ) -> shared::mcp_server::taxonomy_job_vo::SuccessStatus {
        shared::mcp_server::taxonomy_job_vo::SuccessStatus::new(false)
    }
    fn get_class_methods(
        &self,
        _path: &FilePath,
    ) -> shared::common::taxonomy_suggestion_vo::MetadataVO {
        shared::common::taxonomy_suggestion_vo::MetadataVO::new(std::collections::HashMap::new())
    }
    fn get_class_bases_map(
        &self,
        _path: &FilePath,
    ) -> shared::common::taxonomy_suggestion_vo::MetadataVO {
        shared::common::taxonomy_suggestion_vo::MetadataVO::new(std::collections::HashMap::new())
    }
    fn get_assignment_targets(
        &self,
        _path: &FilePath,
    ) -> shared::common::taxonomy_suggestion_vo::MetadataVO {
        shared::common::taxonomy_suggestion_vo::MetadataVO::new(std::collections::HashMap::new())
    }
    fn get_control_flow_count(
        &self,
        _path: &FilePath,
    ) -> shared::common::taxonomy_common_vo::Count {
        shared::common::taxonomy_common_vo::Count::default()
    }
    fn is_barrel_file(&self, _path: &FilePath) -> shared::common::taxonomy_common_vo::BooleanVO {
        shared::common::taxonomy_common_vo::BooleanVO::default()
    }
    fn get_stem(&self, _path: &FilePath) -> shared::common::taxonomy_name_vo::SymbolName {
        shared::common::taxonomy_name_vo::SymbolName::new("")
    }
    fn is_entry_point(&self, _path: &FilePath) -> shared::common::taxonomy_common_vo::BooleanVO {
        shared::common::taxonomy_common_vo::BooleanVO::default()
    }
    fn get_supported_extensions(&self) -> shared::common::taxonomy_common_vo::PatternList {
        shared::common::taxonomy_common_vo::PatternList::default()
    }
}

struct PlaceholderAnalyzer;
impl shared::naming_rules::contract_naming_analyzer_protocol::INamingAnalyzerProtocol
    for PlaceholderAnalyzer
{
    fn config(&self) -> &ArchitectureConfig {
        static CONFIG: std::sync::OnceLock<ArchitectureConfig> = std::sync::OnceLock::new();
        CONFIG.get_or_init(ArchitectureConfig::default)
    }
    fn layer_map(&self) -> &shared::taxonomy_definition_vo::LayerMapVO {
        static MAP: std::sync::OnceLock<shared::taxonomy_definition_vo::LayerMapVO> =
            std::sync::OnceLock::new();
        MAP.get_or_init(|| {
            shared::taxonomy_definition_vo::LayerMapVO::new(std::collections::HashMap::new())
        })
    }
    fn detect_layer(
        &self,
        _f: &FilePath,
        _root_dir: &FilePath,
    ) -> Option<shared::taxonomy_layer_vo::LayerNameVO> {
        None
    }
}

impl IAnalyzer for PlaceholderAnalyzer {
    fn fs(&self) -> &dyn shared::common::contract_system_protocol::IFileSystemProtocol {
        static FS: std::sync::OnceLock<NullFileSystem> = std::sync::OnceLock::new();
        FS.get_or_init(|| NullFileSystem)
    }
    fn parser(&self) -> &dyn shared::common::contract_parser_protocol::ISourceParserProtocol {
        static PARSER: std::sync::OnceLock<NullSourceParser> = std::sync::OnceLock::new();
        PARSER.get_or_init(|| NullSourceParser)
    }
    fn detect_module_layer(
        &self,
        _module_path: &FilePath,
    ) -> Option<shared::taxonomy_layer_vo::LayerNameVO> {
        None
    }
}

// CodeAnalysisContainer — wiring for code-analysis feature
use crate::CodeAnalysisOrchestrator;
use shared::code_analysis::contract_code_analysis_aggregate::ICodeAnalysisAggregate;

pub struct CodeAnalysisContainer {
    code_analysis_linter: Arc<CodeAnalysisOrchestrator>,
}

impl CodeAnalysisContainer {
    pub fn new() -> Self {
        Self {
            code_analysis_linter: Arc::new(CodeAnalysisOrchestrator::new()),
        }
    }

    pub fn new_with_analyzer(analyzer: Arc<dyn IAnalyzer>) -> Self {
        let checker_container = CodeAnalysisCheckerContainer::new(analyzer);
        Self {
            code_analysis_linter: Arc::new(CodeAnalysisOrchestrator::new_with_container(Arc::new(
                checker_container,
            ))),
        }
    }

    pub fn code_analysis_linter(&self) -> Arc<dyn ICodeAnalysisAggregate> {
        self.code_analysis_linter.clone()
    }
}

impl Default for CodeAnalysisContainer {
    fn default() -> Self {
        Self::new()
    }
}
