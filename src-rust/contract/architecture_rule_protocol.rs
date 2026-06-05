use crate::taxonomy::{
    ArchitectureConfig, FilePath, FilePathList, LintResultList, PatternList, LayerMapVO,
    LayerNameVO, Count, Identity, ErrorMessage, CustomMessageVO, ModuleName,
};
use crate::contract::file_system_port::IFileSystemPort;
use crate::contract::source_parser_port::ISourceParserPort;
use async_trait::async_trait;

pub trait IAnalyzer {
    fn config(&self) -> &ArchitectureConfig;
    fn layer_map(&self) -> &LayerMapVO;
    fn fs(&self) -> &dyn IFileSystemPort;
    fn parser(&self) -> &dyn ISourceParserPort;
    fn detect_layer(&self, f: &FilePath, root_dir: &FilePath) -> Option<LayerNameVO>;
    fn detect_module_layer(&self, module_path: &FilePath) -> Option<LayerNameVO>;
}

pub trait IArchRuleProtocol {
    fn rule_name(&self) -> Identity;
}

#[async_trait]
pub trait INamingCheckerProtocol: Send + Sync {
    async fn check_file_naming(
        &self, analyzer: &dyn IAnalyzer, files: &FilePathList, root_dir: &FilePath, results: &mut LintResultList,
    );
    async fn check_domain_suffixes(
        &self, analyzer: &dyn IAnalyzer, files: &FilePathList, root_dir: &FilePath, results: &mut LintResultList,
    );
}

#[async_trait]
pub trait IInternalCheckerProtocol: Send + Sync {
    async fn check_layer_internal_rules(
        &self, analyzer: &dyn IAnalyzer, files: &FilePathList, root_dir: &FilePath, results: &mut LintResultList,
    );
}

#[async_trait]
pub trait IMetricCheckerProtocol: Send + Sync {
    async fn check_line_counts(
        &self, analyzer: &dyn IAnalyzer, files: &FilePathList, root_dir: &FilePath, results: &mut LintResultList,
    );
    async fn check_mandatory_class_definition(
        &self, analyzer: &dyn IAnalyzer, files: &FilePathList, root_dir: &FilePath, results: &mut LintResultList,
    );
}

#[async_trait]
pub trait IRoleCheckerProtocol: Send + Sync {
    async fn check_agent_roles(
        &self, analyzer: &dyn IAnalyzer, files: &FilePathList, root_dir: &FilePath, results: &mut LintResultList,
    );
    async fn check_surface_roles(
        &self, analyzer: &dyn IAnalyzer, files: &FilePathList, root_dir: &FilePath, results: &mut LintResultList,
    );
}

#[async_trait]
pub trait IArchImportProcessorProtocol: Send + Sync {
    async fn process_file_imports(
        &self, analyzer: &dyn IAnalyzer, file_path: &FilePath, root_dir: &FilePath, results: &mut LintResultList,
    );
    async fn validate_imports_present(
        &self,
        analyzer: &dyn IAnalyzer,
        file_path: &FilePath,
        root_dir: &FilePath,
        required_layers: &PatternList,
        results: &mut LintResultList,
        message_template: &ErrorMessage,
        layer_name: &LayerNameVO,
        layers_display: &PatternList,
    );
}

#[async_trait]
pub trait INamingRuleProtocol: IArchRuleProtocol + Send + Sync {
    async fn check_file_naming(
        &self,
        files: &FilePathList,
        root_dir: &FilePath,
        layer_map: &LayerMapVO,
        global_expected: Count,
        global_exceptions: &PatternList,
        results: &mut LintResultList,
        detect_layer_fn: &dyn Fn(&FilePath, &FilePath) -> Option<LayerNameVO>,
    );
    async fn check_class_naming(
        &self, files: &FilePathList, results: &mut LintResultList, source_parser: &dyn ISourceParserPort,
    );
    async fn check_function_naming(
        &self, files: &FilePathList, results: &mut LintResultList, source_parser: &dyn ISourceParserPort,
    );
}

#[async_trait]
pub trait ICodeQualityProtocol: IArchRuleProtocol + Send + Sync {
    async fn check_no_bypass_comments(
        &self,
        file_path: &FilePath,
        fs: &dyn IFileSystemPort,
        results: &mut LintResultList,
        forbidden_words: Option<&PatternList>,
        violation_message: Option<&ErrorMessage>,
        custom_messages: Option<&[CustomMessageVO]>,
    );
    async fn check_unused_mandatory_imports(
        &self,
        files: &FilePathList,
        parser: &dyn ISourceParserPort,
        results: &mut LintResultList,
        violation_message: Option<&ErrorMessage>,
        mandatory_imports: Option<&PatternList>,
        layer_resolver: Option<&dyn Fn(&ModuleName) -> Option<LayerNameVO>>,
    );
    async fn check_dead_inheritance_bypass(
        &self, analyzer: &dyn IAnalyzer, files: &FilePathList, root_dir: &FilePath, results: &mut LintResultList,
    );
    async fn check_forbidden_inheritance(
        &self, analyzer: &dyn IAnalyzer, files: &FilePathList, root_dir: &FilePath, results: &mut LintResultList,
    );
}

#[async_trait]
pub trait IArchStructureProtocol: IArchRuleProtocol + Send + Sync {
    async fn check_file_naming(&self, analyzer: &dyn IAnalyzer, files: &FilePathList, root_dir: &FilePath, results: &mut LintResultList);
    async fn check_domain_suffixes(&self, analyzer: &dyn IAnalyzer, files: &FilePathList, root_dir: &FilePath, results: &mut LintResultList);
    async fn check_layer_internal_rules(&self, analyzer: &dyn IAnalyzer, files: &FilePathList, root_dir: &FilePath, results: &mut LintResultList);
    async fn check_line_counts(&self, analyzer: &dyn IAnalyzer, files: &FilePathList, root_dir: &FilePath, results: &mut LintResultList);
    async fn check_mandatory_class_definition(&self, analyzer: &dyn IAnalyzer, files: &FilePathList, root_dir: &FilePath, results: &mut LintResultList);
    async fn check_agent_roles(&self, analyzer: &dyn IAnalyzer, files: &FilePathList, root_dir: &FilePath, results: &mut LintResultList);
    async fn check_surface_roles(&self, analyzer: &dyn IAnalyzer, files: &FilePathList, root_dir: &FilePath, results: &mut LintResultList);
}

#[async_trait]
pub trait IArchImportProtocol: IArchRuleProtocol + Send + Sync {
    async fn check_mandatory_imports(&self, analyzer: &dyn IAnalyzer, files: &FilePathList, root_dir: &FilePath, results: &mut LintResultList);
    async fn check_forbidden_imports(&self, analyzer: &dyn IAnalyzer, files: &FilePathList, root_dir: &FilePath, results: &mut LintResultList);
    async fn check_legacy_import_rules(&self, analyzer: &dyn IAnalyzer, files: &FilePathList, root_dir: &FilePath, results: &mut LintResultList);
}
