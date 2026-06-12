// PURPOSE: Root container for code-analysis — defines CheckerContainer, RoleOrchestrator, and AnalysisContainer

use std::sync::Arc;

use crate::capabilities_check_bypass_checker::BypassChecker;
use crate::capabilities_class_checker::ArchClassChecker;
use crate::capabilities_cycle_analyzer::DependencyCycleAnalyzer;
use crate::capabilities_dead_inheritance_checker::DeadInheritanceChecker;
use crate::capabilities_inline_unused_checker::InlineUnusedChecker;
use crate::capabilities_line_checker::ArchLineChecker;
use crate::capabilities_mandatory_inheritance_checker::MandatoryInheritanceChecker;
use shared::code_analysis::contract_bypass_checker_protocol::IBypassCheckerProtocol;
use shared::code_analysis::contract_class_protocol::IMandatoryClassProtocol;
use shared::code_analysis::contract_cycle_protocol::ICycleAnalysisProtocol;
use shared::code_analysis::contract_dead_inheritance_protocol::IDeadInheritanceProtocol;
use shared::code_analysis::contract_inline_unused_protocol::IInlineUnusedProtocol;
use shared::code_analysis::contract_line_protocol::ILineCheckerProtocol;
use shared::code_analysis::contract_mandatory_inheritance_protocol::IMandatoryInheritanceProtocol;
use shared::config_system::taxonomy_config_vo::ArchitectureConfig;
use shared::import_rules::contract_rule_protocol::IAnalyzer;
use shared::output_report::taxonomy_result_vo::LintResult;
use shared::output_report::taxonomy_result_vo::LintResultList;
use shared::role_rules::contract_role_aggregate::IRoleAggregate;
use shared::source_parsing::taxonomy_path_vo::FilePath;
use shared::source_parsing::taxonomy_paths_vo::FilePathList;

/// CheckerContainer holds all the protocol implementations for AES checking
pub struct CheckerContainer {
    analyzer: Arc<dyn IAnalyzer>,
    bypass_checker: Arc<dyn IBypassCheckerProtocol>,
    inline_unused_checker: Arc<dyn IInlineUnusedProtocol>,
    dead_inheritance_checker: Arc<dyn IDeadInheritanceProtocol>,
    mandatory_inheritance_checker: Arc<dyn IMandatoryInheritanceProtocol>,
    capabilities_role_checker: Arc<dyn ICapabilitiesRoleProtocol>,
    line_checker: Arc<dyn ILineCheckerProtocol>,
    taxonomy_checker: Arc<dyn ITaxonomyProtocol>,
    contract_checker: Arc<dyn IContractProtocol>,
    class_checker: Arc<dyn IMandatoryClassProtocol>,
    naming_checker: Arc<dyn INamingProtocol>,
    import_mandatory_checker: Arc<dyn IImportMandatoryProtocol>,
    import_intent_checker: Arc<dyn IImportIntentProtocol>,
    import_forbidden_checker: Arc<dyn IImportForbiddenProtocol>,
    cycle_analyzer: Arc<dyn ICycleAnalysisProtocol>,
    surface_checker: Arc<dyn ISurfaceProtocol>,
    orphan_aggregate: Arc<dyn IOrphanAggregate>,
}

pub struct CheckerContainerParts {
    pub capabilities_role_checker: Arc<dyn ICapabilitiesRoleProtocol>,
    pub taxonomy_checker: Arc<dyn ITaxonomyProtocol>,
    pub contract_checker: Arc<dyn IContractProtocol>,
    pub naming_checker: Arc<dyn INamingProtocol>,
    pub import_mandatory_checker: Arc<dyn IImportMandatoryProtocol>,
    pub import_intent_checker: Arc<dyn IImportIntentProtocol>,
    pub import_forbidden_checker: Arc<dyn IImportForbiddenProtocol>,
    pub surface_checker: Arc<dyn ISurfaceProtocol>,
    pub orphan_aggregate: Arc<dyn IOrphanAggregate>,
}

impl CheckerContainer {
    pub fn new(analyzer: Arc<dyn IAnalyzer>) -> Self {
        Self::new_with_parts(
            analyzer,
            CheckerContainerParts {
                capabilities_role_checker: Arc::new(PlaceholderCapabilitiesRoleChecker),
                taxonomy_checker: Arc::new(PlaceholderTaxonomyChecker),
                contract_checker: Arc::new(PlaceholderContractChecker),
                naming_checker: Arc::new(PlaceholderNamingChecker),
                import_mandatory_checker: Arc::new(PlaceholderImportMandatoryChecker),
                import_intent_checker: Arc::new(PlaceholderImportIntentChecker),
                import_forbidden_checker: Arc::new(PlaceholderImportForbiddenChecker),
                surface_checker: Arc::new(PlaceholderSurfaceChecker),
                orphan_aggregate: Arc::new(PlaceholderOrphanAggregate),
            },
        )
    }

    pub fn new_with_parts(analyzer: Arc<dyn IAnalyzer>, parts: CheckerContainerParts) -> Self {
        Self {
            analyzer,
            bypass_checker: Arc::new(BypassChecker {}),
            inline_unused_checker: Arc::new(InlineUnusedChecker {}),
            dead_inheritance_checker: Arc::new(DeadInheritanceChecker {}),
            mandatory_inheritance_checker: Arc::new(MandatoryInheritanceChecker {}),
            capabilities_role_checker: parts.capabilities_role_checker,
            line_checker: Arc::new(ArchLineChecker {}),
            taxonomy_checker: parts.taxonomy_checker,
            contract_checker: parts.contract_checker,
            class_checker: Arc::new(ArchClassChecker {}),
            naming_checker: parts.naming_checker,
            import_mandatory_checker: parts.import_mandatory_checker,
            import_intent_checker: parts.import_intent_checker,
            import_forbidden_checker: parts.import_forbidden_checker,
            cycle_analyzer: Arc::new(DependencyCycleAnalyzer::new(ArchitectureConfig::default())),
            surface_checker: parts.surface_checker,
            orphan_aggregate: parts.orphan_aggregate,
        }
    }

    pub fn bypass_checker(&self) -> &Arc<dyn IBypassCheckerProtocol> {
        &self.bypass_checker
    }

    pub fn inline_unused_checker(&self) -> &Arc<dyn IInlineUnusedProtocol> {
        &self.inline_unused_checker
    }

    pub fn dead_inheritance_checker(&self) -> &Arc<dyn IDeadInheritanceProtocol> {
        &self.dead_inheritance_checker
    }

    pub fn mandatory_inheritance_checker(&self) -> &Arc<dyn IMandatoryInheritanceProtocol> {
        &self.mandatory_inheritance_checker
    }

    pub fn capabilities_role_checker(&self) -> &Arc<dyn ICapabilitiesRoleProtocol> {
        &self.capabilities_role_checker
    }

    pub fn line_checker(&self) -> &Arc<dyn ILineCheckerProtocol> {
        &self.line_checker
    }

    pub fn taxonomy_checker(&self) -> &Arc<dyn ITaxonomyProtocol> {
        &self.taxonomy_checker
    }

    pub fn contract_checker(&self) -> &Arc<dyn IContractProtocol> {
        &self.contract_checker
    }

    pub fn class_checker(&self) -> &Arc<dyn IMandatoryClassProtocol> {
        &self.class_checker
    }

    pub fn naming_checker(&self) -> &Arc<dyn INamingProtocol> {
        &self.naming_checker
    }

    pub fn import_mandatory_checker(&self) -> &Arc<dyn IImportMandatoryProtocol> {
        &self.import_mandatory_checker
    }

    pub fn import_intent_checker(&self) -> &Arc<dyn IImportIntentProtocol> {
        &self.import_intent_checker
    }

    pub fn import_forbidden_checker(&self) -> &Arc<dyn IImportForbiddenProtocol> {
        &self.import_forbidden_checker
    }

    pub fn cycle_analyzer(&self) -> &Arc<dyn ICycleAnalysisProtocol> {
        &self.cycle_analyzer
    }

    pub fn surface_checker(&self) -> &Arc<dyn ISurfaceProtocol> {
        &self.surface_checker
    }

    pub fn orphan_aggregate(&self) -> &Arc<dyn IOrphanAggregate> {
        &self.orphan_aggregate
    }

    pub fn detect_layer(
        &self,
        _file: &str,
        _root_dir: &str,
    ) -> Option<shared::taxonomy_layer_vo::LayerNameVO> {
        None
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

    pub fn as_checker_ref(&self) -> &dyn CheckerContainerRef {
        self
    }
}

/// Trait for dynamic dispatch of CheckerContainer
pub trait CheckerContainerRef {
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

// Local protocols that aren't in shared
pub trait ICapabilitiesRoleProtocol: Send + Sync {
    fn check_capability_routing(
        &self,
        source: &shared::config_system::taxonomy_source_vo::SourceContentVO,
        layer: &shared::taxonomy_layer_vo::LayerNameVO,
        violations: &mut Vec<LintResult>,
    );
}

pub trait ITaxonomyProtocol: Send + Sync {
    fn check_entity(
        &self,
        source: &shared::config_system::taxonomy_source_vo::SourceContentVO,
        violations: &mut Vec<LintResult>,
    );
    fn check_error(
        &self,
        source: &shared::config_system::taxonomy_source_vo::SourceContentVO,
        violations: &mut Vec<LintResult>,
    );
    fn check_event(
        &self,
        source: &shared::config_system::taxonomy_source_vo::SourceContentVO,
        violations: &mut Vec<LintResult>,
    );
    fn check_constant(
        &self,
        source: &shared::config_system::taxonomy_source_vo::SourceContentVO,
        violations: &mut Vec<LintResult>,
    );
}

pub trait IContractProtocol: Send + Sync {
    fn check_aggregate(
        &self,
        source: &shared::config_system::taxonomy_source_vo::SourceContentVO,
        def: &shared::common::taxonomy_definition_vo::LayerDefinition,
        violations: &mut Vec<LintResult>,
    );
}

pub trait INamingProtocol: Send + Sync {
    fn check_file_naming(
        &self,
        analyzer: &Arc<dyn IAnalyzer>,
        files: &FilePathList,
        root: &FilePath,
        violations: &mut LintResultList,
    );
    fn check_domain_suffixes(
        &self,
        analyzer: &Arc<dyn IAnalyzer>,
        files: &FilePathList,
        root: &FilePath,
        violations: &mut LintResultList,
    );
}

pub trait IImportMandatoryProtocol: Send + Sync {
    fn check_mandatory_imports(
        &self,
        analyzer: &Arc<dyn IAnalyzer>,
        files: &FilePathList,
        root: &FilePath,
        violations: &mut LintResultList,
    );
}

pub trait IImportIntentProtocol: Send + Sync {
    fn check_mandatory_imports(
        &self,
        analyzer: &Arc<dyn IAnalyzer>,
        files: &FilePathList,
        root: &FilePath,
        violations: &mut LintResultList,
    );
}

pub trait IImportForbiddenProtocol: Send + Sync {
    fn check_forbidden_imports(
        &self,
        analyzer: &Arc<dyn IAnalyzer>,
        files: &FilePathList,
        root: &FilePath,
        violations: &mut LintResultList,
    );
}

pub trait ISurfaceProtocol: Send + Sync {
    fn check_surface_hierarchy(
        &self,
        files: &[FilePath],
        root: &FilePath,
        violations: &mut LintResultList,
    );
}

pub trait IOrphanAggregate: Send + Sync {
    fn check_orphans(
        &self,
        container: &dyn CheckerContainerRef,
        files: &[String],
        root_dir: &str,
    ) -> Vec<LintResult>;
}

struct PlaceholderCapabilitiesRoleChecker;
impl ICapabilitiesRoleProtocol for PlaceholderCapabilitiesRoleChecker {
    fn check_capability_routing(
        &self,
        _source: &shared::config_system::taxonomy_source_vo::SourceContentVO,
        _layer: &shared::taxonomy_layer_vo::LayerNameVO,
        _violations: &mut Vec<LintResult>,
    ) {
    }
}

struct PlaceholderTaxonomyChecker;
impl ITaxonomyProtocol for PlaceholderTaxonomyChecker {
    fn check_entity(
        &self,
        _source: &shared::config_system::taxonomy_source_vo::SourceContentVO,
        _violations: &mut Vec<LintResult>,
    ) {
    }
    fn check_error(
        &self,
        _source: &shared::config_system::taxonomy_source_vo::SourceContentVO,
        _violations: &mut Vec<LintResult>,
    ) {
    }
    fn check_event(
        &self,
        _source: &shared::config_system::taxonomy_source_vo::SourceContentVO,
        _violations: &mut Vec<LintResult>,
    ) {
    }
    fn check_constant(
        &self,
        _source: &shared::config_system::taxonomy_source_vo::SourceContentVO,
        _violations: &mut Vec<LintResult>,
    ) {
    }
}

struct PlaceholderContractChecker;
impl IContractProtocol for PlaceholderContractChecker {
    fn check_aggregate(
        &self,
        _source: &shared::config_system::taxonomy_source_vo::SourceContentVO,
        _def: &shared::common::taxonomy_definition_vo::LayerDefinition,
        _violations: &mut Vec<LintResult>,
    ) {
    }
}

struct PlaceholderNamingChecker;
impl INamingProtocol for PlaceholderNamingChecker {
    fn check_file_naming(
        &self,
        _analyzer: &Arc<dyn IAnalyzer>,
        _files: &FilePathList,
        _root: &FilePath,
        _violations: &mut LintResultList,
    ) {
    }
    fn check_domain_suffixes(
        &self,
        _analyzer: &Arc<dyn IAnalyzer>,
        _files: &FilePathList,
        _root: &FilePath,
        _violations: &mut LintResultList,
    ) {
    }
}

struct PlaceholderImportMandatoryChecker;
impl IImportMandatoryProtocol for PlaceholderImportMandatoryChecker {
    fn check_mandatory_imports(
        &self,
        _analyzer: &Arc<dyn IAnalyzer>,
        _files: &FilePathList,
        _root: &FilePath,
        _violations: &mut LintResultList,
    ) {
    }
}

struct PlaceholderImportIntentChecker;
impl IImportIntentProtocol for PlaceholderImportIntentChecker {
    fn check_mandatory_imports(
        &self,
        _analyzer: &Arc<dyn IAnalyzer>,
        _files: &FilePathList,
        _root: &FilePath,
        _violations: &mut LintResultList,
    ) {
    }
}

struct PlaceholderImportForbiddenChecker;
impl IImportForbiddenProtocol for PlaceholderImportForbiddenChecker {
    fn check_forbidden_imports(
        &self,
        _analyzer: &Arc<dyn IAnalyzer>,
        _files: &FilePathList,
        _root: &FilePath,
        _violations: &mut LintResultList,
    ) {
    }
}

struct PlaceholderSurfaceChecker;
impl ISurfaceProtocol for PlaceholderSurfaceChecker {
    fn check_surface_hierarchy(
        &self,
        _files: &[FilePath],
        _root: &FilePath,
        _violations: &mut LintResultList,
    ) {
    }
}

struct PlaceholderOrphanAggregate;
impl IOrphanAggregate for PlaceholderOrphanAggregate {
    fn check_orphans(
        &self,
        _container: &dyn CheckerContainerRef,
        _files: &[String],
        _root_dir: &str,
    ) -> Vec<LintResult> {
        Vec::new()
    }
}

/// RoleOrchestrator for agent and surface role checks
pub struct RoleOrchestrator {
    _aggregate: Arc<dyn IRoleAggregate>,
}

impl RoleOrchestrator {
    pub fn new(aggregate: Arc<dyn IRoleAggregate>) -> Self {
        Self {
            _aggregate: aggregate,
        }
    }

    pub fn run_all_role_checks(
        &self,
        files: &[String],
        max_lines: usize,
        violations: &mut Vec<LintResult>,
    ) {
        // Placeholder implementation
        // In a full implementation, this would check AES0305 and AES0306
        for file in files {
            let content = std::fs::read_to_string(file).unwrap_or_default();
            let line_count = content.lines().count();
            if line_count > max_lines {
                violations.push(LintResult::new_arch(
                    file,
                    0,
                    "AES0305",
                    shared::output_report::taxonomy_severity_vo::Severity::CRITICAL,
                    "Role violation: agent file exceeds max lines",
                ));
            }
        }
    }
}

impl CheckerContainerRef for CheckerContainer {
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

impl Default for CheckerContainer {
    fn default() -> Self {
        Self::new(Arc::new(PlaceholderAnalyzer))
    }
}

struct NullFileSystem;

#[async_trait::async_trait]
impl shared::file_system::contract_system_port::IFileSystemPort for NullFileSystem {
    async fn walk(&self, _path: &FilePath, _ignored_patterns: Option<&shared::common::taxonomy_common_vo::PatternList>) -> FilePathList {
        FilePathList::new(vec![])
    }
    async fn is_directory(&self, _path: &FilePath) -> shared::pipeline_jobs::taxonomy_job_vo::SuccessStatus {
        shared::pipeline_jobs::taxonomy_job_vo::SuccessStatus::new(false)
    }
    async fn is_file(&self, _path: &FilePath) -> shared::pipeline_jobs::taxonomy_job_vo::SuccessStatus {
        shared::pipeline_jobs::taxonomy_job_vo::SuccessStatus::new(false)
    }
    async fn get_relative_path(&self, path: &FilePath, _start: &FilePath) -> FilePath {
        path.clone()
    }
    async fn read_text(&self, _path: &FilePath) -> Result<shared::common::taxonomy_source_vo::ContentString, shared::file_system::taxonomy_filesystem_error::FileSystemError> {
        Err(shared::file_system::taxonomy_filesystem_error::FileSystemError::new(
            FilePath::default(),
            shared::common::taxonomy_common_error::ErrorMessage::new("null filesystem: not initialized"),
            shared::pipeline_jobs::taxonomy_action_vo::ActionName::new("read"),
        ))
    }
    async fn get_line_count(&self, _path: &FilePath) -> shared::common::taxonomy_common_vo::Count {
        shared::common::taxonomy_common_vo::Count::default()
    }
    async fn exists(&self, _path: &FilePath) -> shared::pipeline_jobs::taxonomy_job_vo::SuccessStatus {
        shared::pipeline_jobs::taxonomy_job_vo::SuccessStatus::new(false)
    }
    async fn get_parent(&self, _path: &FilePath) -> FilePath {
        FilePath::default()
    }
    async fn write_text(
        &self,
        _path: &FilePath,
        _content: &shared::common::taxonomy_source_vo::ContentString,
        _mode: Option<&shared::common::taxonomy_layer_vo::Identity>,
    ) -> Result<shared::pipeline_jobs::taxonomy_job_vo::SuccessStatus, shared::file_system::taxonomy_filesystem_error::FileSystemError> {
        Err(shared::file_system::taxonomy_filesystem_error::FileSystemError::new(
            FilePath::default(),
            shared::common::taxonomy_common_error::ErrorMessage::new("null filesystem: not initialized"),
            shared::pipeline_jobs::taxonomy_action_vo::ActionName::new("write"),
        ))
    }
    async fn glob(&self, _pattern: &shared::common::taxonomy_layer_vo::Identity) -> FilePathList {
        FilePathList::new(vec![])
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
    async fn read_file(&self, _path: &FilePath) -> Result<shared::common::taxonomy_source_vo::ContentString, shared::file_system::taxonomy_filesystem_error::FileSystemError> {
        Err(shared::file_system::taxonomy_filesystem_error::FileSystemError::new(
            FilePath::default(),
            shared::common::taxonomy_common_error::ErrorMessage::new("null filesystem: not initialized"),
            shared::pipeline_jobs::taxonomy_action_vo::ActionName::new("read"),
        ))
    }
}

struct NullSourceParser;

impl shared::source_parsing::contract_parser_port::ISourceParserPort for NullSourceParser {
    fn extract_imports(&self, _path: &FilePath) -> Result<shared::code_analysis::taxonomy_import_source_vo::ImportInfoList, shared::source_parsing::taxonomy_parser_error::SourceParserError> {
        Ok(shared::code_analysis::taxonomy_import_source_vo::ImportInfoList::default())
    }
    fn get_raw_symbols(&self, _path: &FilePath) -> Result<shared::pipeline_jobs::taxonomy_job_vo::ResponseData, shared::source_parsing::taxonomy_parser_error::SourceParserError> {
        Ok(shared::pipeline_jobs::taxonomy_job_vo::ResponseData::default())
    }
    fn get_class_attributes(&self, _path: &FilePath) -> shared::pipeline_jobs::taxonomy_job_vo::ResponseData {
        shared::pipeline_jobs::taxonomy_job_vo::ResponseData::default()
    }
    fn has_all_export(&self, _path: &FilePath) -> shared::pipeline_jobs::taxonomy_job_vo::SuccessStatus {
        shared::pipeline_jobs::taxonomy_job_vo::SuccessStatus::new(false)
    }
    fn find_primitive_violations(
        &self,
        _path: &FilePath,
        _primitive_types: &shared::language_adapters::taxonomy_naming_list_vo::PrimitiveTypeList,
    ) -> shared::code_analysis::taxonomy_import_source_vo::PrimitiveViolationList {
        shared::code_analysis::taxonomy_import_source_vo::PrimitiveViolationList::default()
    }
    fn find_unused_imports(&self, _path: &FilePath) -> shared::code_analysis::taxonomy_import_source_vo::ImportInfoList {
        shared::code_analysis::taxonomy_import_source_vo::ImportInfoList::default()
    }
    fn get_class_definitions(&self, _path: &FilePath) -> Result<shared::common::taxonomy_suggestion_vo::MetadataVO, shared::source_parsing::taxonomy_parser_error::SourceParserError> {
        Ok(shared::common::taxonomy_suggestion_vo::MetadataVO::new(std::collections::HashMap::new()))
    }
    fn get_function_definitions(&self, _path: &FilePath) -> shared::common::taxonomy_suggestion_vo::MetadataVO {
        shared::common::taxonomy_suggestion_vo::MetadataVO::new(std::collections::HashMap::new())
    }
    fn is_symbol_exported(&self, _path: &FilePath, _symbol: &shared::common::taxonomy_name_vo::SymbolName) -> shared::pipeline_jobs::taxonomy_job_vo::SuccessStatus {
        shared::pipeline_jobs::taxonomy_job_vo::SuccessStatus::new(false)
    }
    fn get_class_methods(&self, _path: &FilePath) -> shared::common::taxonomy_suggestion_vo::MetadataVO {
        shared::common::taxonomy_suggestion_vo::MetadataVO::new(std::collections::HashMap::new())
    }
    fn get_class_bases_map(&self, _path: &FilePath) -> shared::common::taxonomy_suggestion_vo::MetadataVO {
        shared::common::taxonomy_suggestion_vo::MetadataVO::new(std::collections::HashMap::new())
    }
    fn get_assignment_targets(&self, _path: &FilePath) -> shared::common::taxonomy_suggestion_vo::MetadataVO {
        shared::common::taxonomy_suggestion_vo::MetadataVO::new(std::collections::HashMap::new())
    }
    fn get_control_flow_count(&self, _path: &FilePath) -> shared::common::taxonomy_common_vo::Count {
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

// AnalysisContainer — wiring for code-analysis feature
use crate::CodebaseScanOrchestrator;
use shared::code_analysis::contract_lint_protocol::IArchLintProtocol;

pub struct AnalysisContainer {
    arch_linter: Arc<dyn IArchLintProtocol>,
}

impl AnalysisContainer {
    pub fn new() -> Self {
        Self {
            arch_linter: Arc::new(CodebaseScanOrchestrator::new()),
        }
    }

    pub fn architecture_linter(&self) -> Arc<dyn IArchLintProtocol> {
        self.arch_linter.clone()
    }
}

impl Default for AnalysisContainer {
    fn default() -> Self {
        Self::new()
    }
}
