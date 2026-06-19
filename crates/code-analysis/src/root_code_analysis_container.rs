// PURPOSE: Root container for code-analysis — defines CodeAnalysisCheckerContainer and CodeAnalysisContainer
// Wiring: ICodeMetricAnalyzerProtocol → CodeDuplicationAnalyzer (capabilities layer)

use crate::capabilities_check_bypass_checker::BypassChecker;
use crate::capabilities_code_duplication_analyzer::CodeDuplicationAnalyzer;
use crate::capabilities_line_checker::ArchLineChecker;
use crate::capabilities_mandatory_definition_checker::MandatoryDefinitionChecker;
use shared::code_analysis::contract_bypass_checker_protocol::IBypassCheckerProtocol;
use shared::code_analysis::contract_class_protocol::IMandatoryClassProtocol;
use shared::code_analysis::contract_dead_inheritance_protocol::IDeadInheritanceProtocol;
use shared::code_analysis::contract_line_protocol::ILineCheckerProtocol;
use shared::config_system::taxonomy_config_vo::ArchitectureConfig;
use shared::import_rules::contract_rule_protocol::IAnalyzer;
use shared::source_parsing::taxonomy_path_vo::FilePath;
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
    _code_duplication_analyzer: Arc<CodeDuplicationAnalyzer>,
}

impl CodeAnalysisCheckerContainer {
    pub fn new(analyzer: Arc<dyn IAnalyzer>) -> Self {
        let mandatory = Arc::new(MandatoryDefinitionChecker::new());
        Self {
            analyzer,
            bypass_checker: Arc::new(BypassChecker {}),
            mandatory_definition_checker: mandatory,
            line_checker: Arc::new(ArchLineChecker {}),
            _code_duplication_analyzer: Arc::new(CodeDuplicationAnalyzer::new()),
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
        self.analyzer.detect_layer(
            &shared::source_parsing::taxonomy_path_vo::FilePath::new(file.to_string())
                .unwrap_or_default(),
            &shared::source_parsing::taxonomy_path_vo::FilePath::new(root_dir.to_string())
                .unwrap_or_default(),
        )
    }

    pub fn get_layer_def(
        &self,
        _layer: &shared::taxonomy_layer_vo::LayerNameVO,
    ) -> Option<&shared::common::taxonomy_definition_vo::LayerDefinition> {
        None
    }

    pub fn analyzer(&self) -> &Arc<dyn IAnalyzer> {
        &self.analyzer
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
impl shared::file_system::contract_system_port::IFileSystemPort for NullFileSystem {
    async fn walk(
        &self,
        _path: &FilePath,
        _ignored_patterns: Option<&shared::common::taxonomy_common_vo::PatternList>,
    ) -> shared::source_parsing::taxonomy_paths_vo::FilePathList {
        shared::source_parsing::taxonomy_paths_vo::FilePathList::new(vec![])
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
        shared::file_system::taxonomy_filesystem_error::FileSystemError,
    > {
        Err(
            shared::file_system::taxonomy_filesystem_error::FileSystemError::new(
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
        shared::file_system::taxonomy_filesystem_error::FileSystemError,
    > {
        Err(
            shared::file_system::taxonomy_filesystem_error::FileSystemError::new(
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
    ) -> shared::source_parsing::taxonomy_paths_vo::FilePathList {
        shared::source_parsing::taxonomy_paths_vo::FilePathList::new(vec![])
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
        shared::file_system::taxonomy_filesystem_error::FileSystemError,
    > {
        Err(
            shared::file_system::taxonomy_filesystem_error::FileSystemError::new(
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

impl shared::source_parsing::contract_parser_port::ISourceParserPort for NullSourceParser {
    fn extract_imports(
        &self,
        _path: &FilePath,
    ) -> Result<
        shared::code_analysis::taxonomy_import_source_vo::ImportInfoList,
        shared::source_parsing::taxonomy_parser_error::SourceParserError,
    > {
        Ok(shared::code_analysis::taxonomy_import_source_vo::ImportInfoList::default())
    }
    fn get_raw_symbols(
        &self,
        _path: &FilePath,
    ) -> Result<
        shared::mcp_server::taxonomy_job_vo::ResponseData,
        shared::source_parsing::taxonomy_parser_error::SourceParserError,
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
        _primitive_types: &shared::source_parsing::taxonomy_naming_list_vo::PrimitiveTypeList,
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
        shared::source_parsing::taxonomy_parser_error::SourceParserError,
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
impl IAnalyzer for PlaceholderAnalyzer {
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
    fn fs(&self) -> &dyn shared::file_system::contract_system_port::IFileSystemPort {
        static FS: std::sync::OnceLock<NullFileSystem> = std::sync::OnceLock::new();
        FS.get_or_init(|| NullFileSystem)
    }
    fn parser(&self) -> &dyn shared::source_parsing::contract_parser_port::ISourceParserPort {
        static PARSER: std::sync::OnceLock<NullSourceParser> = std::sync::OnceLock::new();
        PARSER.get_or_init(|| NullSourceParser)
    }
    fn detect_layer(
        &self,
        _f: &FilePath,
        _root_dir: &FilePath,
    ) -> Option<shared::taxonomy_layer_vo::LayerNameVO> {
        None
    }
    fn detect_module_layer(
        &self,
        _module_path: &FilePath,
    ) -> Option<shared::taxonomy_layer_vo::LayerNameVO> {
        None
    }
}

// CodeAnalysisContainer — wiring for code-analysis feature
use crate::CodeAnalysisArchLint;
use crate::CodeAnalysisOrchestrator;
use shared::code_analysis::contract_lint_protocol::IArchLintProtocol;

pub struct CodeAnalysisContainer {
    arch_linter: Arc<dyn IArchLintProtocol>,
}

impl CodeAnalysisContainer {
    pub fn new() -> Self {
        let orchestrator = Arc::new(CodeAnalysisOrchestrator::new());
        Self {
            arch_linter: Arc::new(CodeAnalysisArchLint::new(orchestrator)),
        }
    }

    pub fn architecture_linter(&self) -> Arc<dyn IArchLintProtocol> {
        self.arch_linter.clone()
    }
}

impl Default for CodeAnalysisContainer {
    fn default() -> Self {
        Self::new()
    }
}
