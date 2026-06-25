// PURPOSE: IAnalyzer trait — core analyzer interface for import checks
use crate::cli_commands::taxonomy_result_vo::LintResultList;
use crate::common::contract_parser_port::ISourceParserPort;
use crate::common::contract_system_port::IFileSystemPort;
use crate::common::taxonomy_path_vo::FilePath;
use crate::common::taxonomy_paths_vo::FilePathList;
use crate::taxonomy_common_error::ErrorMessage;
use crate::taxonomy_common_vo::Count;
use crate::taxonomy_common_vo::PatternList;
use crate::taxonomy_definition_vo::LayerMapVO;
use crate::taxonomy_layer_vo::Identity;
use crate::taxonomy_layer_vo::LayerNameVO;

pub trait IAnalyzer:
    crate::naming_rules::contract_naming_analyzer_protocol::INamingAnalyzerProtocol + Send + Sync
{
    fn fs(&self) -> &dyn IFileSystemPort;
    fn parser(&self) -> &dyn ISourceParserPort;
    fn detect_module_layer(&self, module_path: &FilePath) -> Option<LayerNameVO>;
}

pub trait IArchRuleProtocol {
    fn rule_name(&self) -> Identity;
}

pub trait IInternalCheckerProtocol: Send + Sync {
    fn check_layer_internal_rules(
        &self,
        analyzer: &dyn IAnalyzer,
        files: &FilePathList,
        root_dir: &FilePath,
        results: &mut LintResultList,
    );
}

pub trait IMetricCheckerProtocol: Send + Sync {
    fn check_line_counts(
        &self,
        analyzer: &dyn IAnalyzer,
        files: &FilePathList,
        root_dir: &FilePath,
        results: &mut LintResultList,
    );
    fn check_mandatory_class_definition(
        &self,
        analyzer: &dyn IAnalyzer,
        files: &FilePathList,
        root_dir: &FilePath,
        results: &mut LintResultList,
    );
}

pub struct ValidateImportsParams<'a> {
    pub analyzer: &'a dyn IAnalyzer,
    pub file_path: &'a FilePath,
    pub root_dir: &'a FilePath,
    pub required_layers: &'a PatternList,
    pub results: &'a mut LintResultList,
    pub message_template: &'a ErrorMessage,
    pub layer_name: &'a LayerNameVO,
    pub layers_display: &'a PatternList,
}

pub trait IArchImportProcessorProtocol: Send + Sync {
    fn process_file_imports(
        &self,
        analyzer: &dyn IAnalyzer,
        file_path: &FilePath,
        root_dir: &FilePath,
        results: &mut LintResultList,
    );
    fn validate_imports_present(&self, params: ValidateImportsParams<'_>);
}

pub struct CheckFileNamingParams<'a> {
    pub files: &'a FilePathList,
    pub root_dir: &'a FilePath,
    pub layer_map: &'a LayerMapVO,
    pub global_expected: Count,
    pub global_exceptions: &'a PatternList,
    pub results: &'a mut LintResultList,
    pub detect_layer_fn: &'a dyn Fn(&FilePath, &FilePath) -> Option<LayerNameVO>,
}

pub trait INamingRuleProtocol: IArchRuleProtocol + Send + Sync {
    fn check_file_naming(&self, params: CheckFileNamingParams<'_>);
    fn check_class_naming(
        &self,
        files: &FilePathList,
        results: &mut LintResultList,
        source_parser: &dyn ISourceParserPort,
    );
    fn check_function_naming(
        &self,
        files: &FilePathList,
        results: &mut LintResultList,
        source_parser: &dyn ISourceParserPort,
    );
}

pub trait IArchStructureProtocol: IArchRuleProtocol + Send + Sync {
    fn check_file_naming(
        &self,
        analyzer: &dyn IAnalyzer,
        files: &FilePathList,
        root_dir: &FilePath,
        results: &mut LintResultList,
    );
    fn check_domain_suffixes(
        &self,
        analyzer: &dyn IAnalyzer,
        files: &FilePathList,
        root_dir: &FilePath,
        results: &mut LintResultList,
    );
    fn check_layer_internal_rules(
        &self,
        analyzer: &dyn IAnalyzer,
        files: &FilePathList,
        root_dir: &FilePath,
        results: &mut LintResultList,
    );
    fn check_line_counts(
        &self,
        analyzer: &dyn IAnalyzer,
        files: &FilePathList,
        root_dir: &FilePath,
        results: &mut LintResultList,
    );
    fn check_mandatory_class_definition(
        &self,
        analyzer: &dyn IAnalyzer,
        files: &FilePathList,
        root_dir: &FilePath,
        results: &mut LintResultList,
    );
}

#[async_trait::async_trait]
pub trait IArchImportProtocol: IArchRuleProtocol + Send + Sync {
    async fn check_mandatory_imports(
        &self,
        analyzer: &dyn IAnalyzer,
        files: &FilePathList,
        root_dir: &FilePath,
        results: &mut LintResultList,
    );
    async fn check_forbidden_imports(
        &self,
        analyzer: &dyn IAnalyzer,
        files: &FilePathList,
        root_dir: &FilePath,
        results: &mut LintResultList,
    );
}
