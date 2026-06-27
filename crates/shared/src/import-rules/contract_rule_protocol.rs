// PURPOSE: IAnalyzer trait — core analyzer interface for import checks
//
// This file defines the protocol traits that capabilities-level checkers
// implement. Each trait represents a single architectural responsibility:
//   - IAnalyzer: central configuration + layer detection hub
//   - IArchRuleProtocol: base trait for all AES rule implementations
//   - IInternalCheckerProtocol: checks layer-internal import rules
//   - IMetricCheckerProtocol: line count + mandatory class definition checks
//   - IArchImportProcessorProtocol: file-level import validation
//   - INamingRuleProtocol: naming convention checks (AES101-102)
//   - IArchStructureProtocol: combined naming + structure + metric checks
//   - IArchImportProtocol: mandatory and forbidden import checks (AES201-202, AES204)
//
// The trapezoidal hierarchy exists because different layers need different
// subsets of these capabilities — the trait bounds reflect the actual
// dependency requirements.
use crate::cli_commands::taxonomy_result_vo::LintResultList;
use crate::common::contract_parser_port::ISourceParserPort;
use crate::common::contract_system_port::IFileSystemPort;
use crate::common::taxonomy_common_error::ErrorMessage;
use crate::common::taxonomy_common_vo::Count;
use crate::common::taxonomy_common_vo::PatternList;
use crate::common::taxonomy_definition_vo::LayerMapVO;
use crate::common::taxonomy_layer_vo::Identity;
use crate::common::taxonomy_layer_vo::LayerNameVO;
use crate::common::taxonomy_path_vo::FilePath;
use crate::common::taxonomy_paths_vo::FilePathList;

/// IAnalyzer — the central configuration and analysis hub.
///
/// Provides access to:
///   - File system (for reading/writing files)
///   - Source parser (for AST-level analysis)
///   - Layer detection (maps file paths to architectural layers)
///
/// Also implements INamingAnalyzerProtocol, which allows naming-rules
/// to reuse the same layer-detection logic without duplicating it.
pub trait IAnalyzer:
    crate::naming_rules::contract_naming_analyzer_protocol::INamingAnalyzerProtocol + Send + Sync
{
    fn fs(&self) -> &dyn IFileSystemPort;
    fn parser(&self) -> &dyn ISourceParserPort;
    fn detect_module_layer(&self, module_path: &FilePath) -> Option<LayerNameVO>;
}

/// Base trait for all AES rule implementations.
/// Every checker must have a unique identity (e.g., "AES201").
pub trait IArchRuleProtocol {
    fn rule_name(&self) -> Identity;
}

/// Checks that imports within a layer respect internal boundaries
/// (e.g., a capabilities file should not import from infrastructure).
pub trait IInternalCheckerProtocol: Send + Sync {
    fn check_layer_internal_rules(
        &self,
        analyzer: &dyn IAnalyzer,
        files: &FilePathList,
        root_dir: &FilePath,
        results: &mut LintResultList,
    );
}

/// Metric-based checks: file line counts, function lengths, and
/// mandatory class/struct definitions within each file.
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

/// Parameters for validating imports in a single file.
/// Bundles all data needed to check whether a file imports from required layers.
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

/// Processes imports at the per-file level.
/// Validates that files import from the correct layers and not from forbidden ones.
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

/// Parameters for file-naming checks.
/// Passes all configuration needed to check naming conventions across layers.
pub struct CheckFileNamingParams<'a> {
    pub files: &'a FilePathList,
    pub root_dir: &'a FilePath,
    pub layer_map: &'a LayerMapVO,
    pub global_expected: Count,
    pub global_exceptions: &'a PatternList,
    pub results: &'a mut LintResultList,
    pub detect_layer_fn: &'a dyn Fn(&FilePath, &FilePath) -> Option<LayerNameVO>,
}

/// Naming convention rules (AES101-102).
/// Checks file names, class names, and function names against
/// the AES layer-based naming conventions.
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

/// Combined structure + naming + metrics protocol.
/// This is a legacy trait that aggregates multiple responsibilities.
/// New implementations should prefer the more granular trait separations.
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

/// Import compliance protocol (AES201-202, AES204).
/// Checks for mandatory imports (files MUST import certain symbols) and
/// forbidden imports (files MUST NOT import certain symbols).
///
/// Both checks use the same async trait because they share the same
/// file-walking and analysis infrastructure — only the rule config differs.
#[async_trait::async_trait]
pub trait IArchImportProtocol: IArchRuleProtocol + Send + Sync {
    /// Check that files contain required imports based on their layer role.
    async fn check_mandatory_imports(
        &self,
        analyzer: &dyn IAnalyzer,
        files: &FilePathList,
        root_dir: &FilePath,
        results: &mut LintResultList,
    );
    /// Check that files do NOT contain prohibited imports.
    async fn check_forbidden_imports(
        &self,
        analyzer: &dyn IAnalyzer,
        files: &FilePathList,
        root_dir: &FilePath,
        results: &mut LintResultList,
    );
}
